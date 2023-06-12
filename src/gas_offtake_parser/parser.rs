//! # Xoserve Files
//! Xoserve provide two files that must be read to build the seasonality data.
//! ## SNCWV
//! The Seasonal Normal Composite Weather Variables file is a comma separated text file
//! like so
//! ```
//! "01/10/2022", 11.08, 10.65, 11.82, 11.90, 12.14, 12.09, 12.43, 12.82, 14.37, 14.32, 13.78, 14.21, 12.96
//! "02/10/2022", 10.95, 10.52, 11.72, 11.78, 12.03, 11.97, 12.33, 12.74, 14.26, 14.20, 13.68, 14.13, 12.89
//! "03/10/2022", 10.87, 10.41, 11.62, 11.68, 11.93, 11.88, 12.24, 12.66, 14.15, 14.09, 13.60, 14.06, 12.81
//! "04/10/2022", 10.79, 10.31, 11.52, 11.59, 11.85, 11.80, 12.15, 12.58, 14.06, 13.99, 13.51, 13.98, 12.74
//! ```
//! Each Local Distribution Zone (LDZ) has is own value in a column.
//! ## ALDAF
//! The Annual Load Profile and Daily Adjustment Factor file is also comma separated
//! ```
//! "EA:E2201BND",01/10/2022,0.589078,-0.238091
//! "EA:E2201BNI",01/10/2022,0.543703,-0.191565
//! "EA:E2201BPD",01/10/2022,0.595346,-0.222440
//! "EA:E2201BPI",01/10/2022,0.543703,-0.191565
//! "EA:E2202BND",01/10/2022,0.712085,-0.141062
//! ```
//! Here the values are grouped by LDZ and also the End User Catergory (EUC). There are multiple
//! EUCs but we only provide energy currently for domestic and small businesses so
//! only use the rows `E2201BND` and `E2202BND`
use chrono::NaiveDate;
use nom::{
    bytes::complete::take,
    character::{
        complete::{alpha1, line_ending},
        streaming::space0,
    },
    multi::separated_list1,
    number::complete::double,
    sequence::delimited,
    Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
use strum::VariantNames;

use crate::{errors::ParseError, IResult, Span};

use super::{
    ALPDAFFile, ALPDAFRow, Band, EndUserCategory, LocalDistributionZone, SCWVFile,
    SeasonalCompositeWeatherVariableRow,
};

fn scwm_file(input: Span) -> IResult<SCWVFile> {
    separated_list1(line_ending, scwv_row).parse(input)
}

fn scwv_row(input: Span) -> IResult<SeasonalCompositeWeatherVariableRow> {
    let (rest, date) = date.parse(input)?;
    let (rest, _comma) = tag(",").precedes(space0).parse(rest)?;
    let (rest, values) = separated_list1(tag(", "), double).parse(rest)?;

    let length = values.len();
    if length != 13 {
        return Err(nom::Err::Error(ParseError::new(
            format!(
                "incorrect number of seasonal composite weather variables, expected 13 got {}",
                length
            ),
            Some("we should get 13 values, one for each region, possibly the file is formatted incorrectly".to_string()),
            input,
        )));
    }

    Ok((
        rest,
        SeasonalCompositeWeatherVariableRow {
            date,
            sc: values[0],
            no: values[1],
            nw: values[2],
            ne: values[3],
            em: values[4],
            wm: values[5],
            wn: values[6],
            ws: values[7],
            ea: values[8],
            nt: values[9],
            se: values[10],
            so: values[11],
            sw: values[12],
        },
    ))
}

/// parse a date surrounded by quotation marks
fn date(input: Span) -> IResult<NaiveDate> {
    delimited(tag("\""), raw_date, tag("\""))(input)
}

/// Parse a date in the form day/month/year
fn raw_date(input: Span) -> IResult<NaiveDate> {
    let (rest, raw_digits) = take(10usize)(input)?;
    let date =
        chrono::naive::NaiveDate::parse_from_str(&raw_digits, "%d/%m/%Y").map_err(|err| {
            nom::Err::Error(ParseError::new(
                format!("failed to parse date: {}", err),
                Some("date should be in the format Day/Month/Year".to_string()),
                input,
            ))
        })?;

    Ok((rest, date))
}

fn alpdaf_file(input: Span) -> IResult<ALPDAFFile> {
    separated_list1(line_ending, alpdaf_row).parse(input)
}

fn alpdaf_row(input: Span) -> IResult<ALPDAFRow> {
    let (rest, location) = delimited(tag("\""), take(2usize), tag(":")).parse(input)?;
    let ldz = location.parse().map_err(|err| {
        nom::Err::Error(ParseError::new(
            format!("failed to local distribution zone: {}", err),
            None,
            input,
        ))
    })?;

    let (rest, end_user_category) = end_user_category.terminated(tag("\",")).parse(rest)?;
    let (rest, date) = raw_date.terminated(tag(",")).parse(rest)?;
    let (rest, annual_load_profile) = double.terminated(tag(",")).parse(rest)?;
    let (rest, daily_adjustment_factor) = double.parse(rest)?;

    Ok((
        rest,
        ALPDAFRow {
            ldz,
            end_user_category,
            date,
            annual_load_profile,
            daily_adjustment_factor,
        },
    ))
}

fn end_user_category(input: Span) -> IResult<EndUserCategory> {
    let (rest, _year) = tag("E").precedes(take(2usize)).parse(input)?;
    let (rest, band) = take(2usize)
        .context("failed to get 2 digit band code")
        .parse(rest)?;
    let (rest, consumer_meter_code) = alpha1
        .context("failed to get consumer_meter_code")
        .parse(rest)?;

    let band = band.parse().map_err(|err| {
        nom::Err::Error(ParseError::new(
            format!("failed to parse band: {}", err),
            Some(format!("band should be one of {:?}", Band::VARIANTS)),
            input,
        ))
    })?;
    let code = consumer_meter_code.parse().map_err(|err| {
        nom::Err::Error(ParseError::new(
            format!("failed to parse consumer meter code: {}", err),
            None,
            input,
        ))
    })?;

    let category = EndUserCategory::new(band, code);
    Ok((rest, category))
}

#[cfg(test)]
mod test_parser {
    use crate::gas_offtake_parser::{ConsumerType, MeterType, UserGroup};

    use super::*;

    #[test]
    fn test_scwv_row() {
        let input = r#""01/10/2022", 11.08, 10.65, 11.82, 11.90, 12.14, 12.09, 12.43, 12.82, 14.37, 14.32, 13.78, 14.21, 12.96"#;

        let (_, got) = scwv_row(input.into()).unwrap();

        let want = SeasonalCompositeWeatherVariableRow {
            date: NaiveDate::from_ymd_opt(2022, 10, 1).unwrap(),
            sc: 11.08,
            no: 10.65,
            nw: 11.82,
            ne: 11.90,
            em: 12.14,
            wm: 12.09,
            wn: 12.43,
            ws: 12.82,
            ea: 14.37,
            nt: 14.32,
            se: 13.78,
            so: 14.21,
            sw: 12.96,
        };

        assert_eq!(want, got);
    }

    #[test]
    fn test_scwv_file() {
        let input = r#""01/10/2022", 11.08, 10.65, 11.82, 11.90, 12.14, 12.09, 12.43, 12.82, 14.37, 14.32, 13.78, 14.21, 12.96
"02/10/2022", 10.95, 10.52, 11.72, 11.78, 12.03, 11.97, 12.33, 12.74, 14.26, 14.20, 13.68, 14.13, 12.89
"03/10/2022", 10.87, 10.41, 11.62, 11.68, 11.93, 11.88, 12.24, 12.66, 14.15, 14.09, 13.60, 14.06, 12.81
"04/10/2022", 10.79, 10.31, 11.52, 11.59, 11.85, 11.80, 12.15, 12.58, 14.06, 13.99, 13.51, 13.98, 12.74"#;

        let (_, got) = scwm_file(input.into()).unwrap();

        let want = vec![
            SeasonalCompositeWeatherVariableRow::new_from_array(
                NaiveDate::from_ymd_opt(2022, 10, 1).unwrap(),
                [
                    11.08, 10.65, 11.82, 11.90, 12.14, 12.09, 12.43, 12.82, 14.37, 14.32, 13.78,
                    14.21, 12.96,
                ],
            ),
            SeasonalCompositeWeatherVariableRow::new_from_array(
                NaiveDate::from_ymd_opt(2022, 10, 2).unwrap(),
                [
                    10.95, 10.52, 11.72, 11.78, 12.03, 11.97, 12.33, 12.74, 14.26, 14.20, 13.68,
                    14.13, 12.89,
                ],
            ),
            SeasonalCompositeWeatherVariableRow::new_from_array(
                NaiveDate::from_ymd_opt(2022, 10, 3).unwrap(),
                [
                    10.87, 10.41, 11.62, 11.68, 11.93, 11.88, 12.24, 12.66, 14.15, 14.09, 13.60,
                    14.06, 12.81,
                ],
            ),
            SeasonalCompositeWeatherVariableRow::new_from_array(
                NaiveDate::from_ymd_opt(2022, 10, 4).unwrap(),
                [
                    10.79, 10.31, 11.52, 11.59, 11.85, 11.80, 12.15, 12.58, 14.06, 13.99, 13.51,
                    13.98, 12.74,
                ],
            ),
        ];

        assert_eq!(want, got);
    }

    #[test]
    fn test_end_user_category() {
        let input = "E2201BND";

        let (_, got) = end_user_category(input.into()).unwrap();

        let want = EndUserCategory::new(
            Band::One,
            UserGroup {
                consumer_type: ConsumerType::Domestic,
                meter_type: crate::gas_offtake_parser::MeterType::Credit,
            },
        );
        assert_eq!(want, got)
    }

    #[test]
    fn test_alpdaf_file() {
        let input = r#""EA:E2201BND",01/10/2022,0.589078,-0.238091
"EA:E2201BNI",01/10/2022,0.543703,-0.191565
"EA:E2201BPD",01/10/2022,0.595346,-0.222440
"EA:E2202BND",01/10/2022,0.712085,-0.141062
"EA:E2203B",01/10/2022,0.573415,-0.153248"#;

        let (_, got) = alpdaf_file(input.into()).unwrap();

        let date = NaiveDate::from_ymd_opt(2022, 10, 1).unwrap();
        let want = vec![
            ALPDAFRow {
                ldz: LocalDistributionZone::EA,
                end_user_category: EndUserCategory {
                    band: Band::One,
                    user_group: UserGroup {
                        consumer_type: ConsumerType::Domestic,
                        meter_type: MeterType::Credit,
                    },
                },
                date,
                annual_load_profile: 0.589078,
                daily_adjustment_factor: -0.238091,
            },
            ALPDAFRow {
                ldz: LocalDistributionZone::EA,
                end_user_category: EndUserCategory {
                    band: Band::One,
                    user_group: UserGroup {
                        consumer_type: ConsumerType::Industry,
                        meter_type: MeterType::Credit,
                    },
                },
                date,
                annual_load_profile: 0.543703,
                daily_adjustment_factor: -0.191565,
            },
            ALPDAFRow {
                ldz: LocalDistributionZone::EA,
                end_user_category: EndUserCategory {
                    band: Band::One,
                    user_group: UserGroup {
                        consumer_type: ConsumerType::Domestic,
                        meter_type: MeterType::Prepayment,
                    },
                },
                date,
                annual_load_profile: 0.595346,
                daily_adjustment_factor: -0.222440,
            },
            ALPDAFRow {
                ldz: LocalDistributionZone::EA,
                end_user_category: EndUserCategory {
                    band: Band::Two,
                    user_group: UserGroup {
                        consumer_type: ConsumerType::Domestic,
                        meter_type: MeterType::Credit,
                    },
                },
                date,
                annual_load_profile: 0.712085,
                daily_adjustment_factor: -0.141062,
            },
            ALPDAFRow {
                ldz: LocalDistributionZone::EA,
                end_user_category: EndUserCategory {
                    band: Band::Three,
                    user_group: UserGroup {
                        consumer_type: ConsumerType::Industry,
                        meter_type: MeterType::Credit,
                    },
                },
                date,
                annual_load_profile: 0.573415,
                daily_adjustment_factor: -0.153248,
            },
        ];

        assert_eq!(want, got)
    }
}
