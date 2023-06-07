use crate::errors::ParseError;
use crate::{IResult, Span};

use super::{HeaderLine, OfftakeFile, OfftakeRow, ProfileClass, WrappedFile};
use super::{ProfileGroups, TrailerLine};
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::number::complete::double;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::Parser;
use nom_supreme::multi::collect_separated_terminated;
use nom_supreme::tag::complete::tag;
use nom_supreme::ParserExt;

/// The main parsing function that collects our other parsers. First get the header line.
/// Then get all of the offtake lines and finally grab the trailer.
pub fn parse_flow_file(file: Span) -> IResult<WrappedFile> {
    let (rest, header) = header(file)?;
    let (rest, offtakes) = offtake_file(rest)?;
    let (_, trailer) = trailer.parse(rest)?;

    Ok((
        rest,
        WrappedFile {
            header,
            file: OfftakeFile {
                profile_groups: offtakes,
            },
            trailer,
        },
    ))
}

/// Gets the header from an electricity flow file. Since we don't care about most of the details
/// we only consume the date from the file.
fn header(line: Span) -> IResult<HeaderLine> {
    let mut parser = delimited(
        tuple((
            alpha1,
            tag("|"),
            alpha1,
            tag("|"),
            alpha1,
            tag("|"),
            alpha1,
            tag("|"),
            alpha1,
            tag("|"),
            alpha1,
            tag("|"),
        )),
        date_time,
        line_ending,
    );

    let (rest, date) = parser(line)?;

    Ok((rest, HeaderLine { date_time: date }))
}

/// parse the trailer line.
fn trailer(line: Span) -> IResult<TrailerLine> {
    let (rest, (_tag, _, _, _, _, _)) =
        tuple((tag("ZPT"), tag("|"), digit1, tag("|"), digit1, line_ending))(line)?;

    Ok((rest, TrailerLine {}))
}

/// A looping parser over our offtake lines with the PFC grouping.
/// These groups are seperated by `PFC` and end when we reach the trailer with `ZPT`
fn offtake_file(input: Span) -> IResult<Vec<ProfileGroups>> {
    collect_separated_terminated(profile_class_group, tag("PFC").peek(), tag("ZPT").peek())
        .parse(input)
}

/// From the profile class line grab the profile class number
/// ie `PFC|1` => `ProfileClass:Class1`
/// This can fail if the line is malformed or if the number isnt a know profile class.
fn profile_class(line: Span) -> IResult<ProfileClass> {
    let (rest, pc) = delimited(tuple((tag("PFC"), (tag("|")))), digit1, line_ending)(line)?;

    let pc = pc.parse().map_err(|err| {
        nom::Err::Error(ParseError::new(
            format!("failed to parse profile class: {}", err),
            None,
            line,
        ))
    })?;

    Ok((rest, pc))
}

/// Parser the profile class with its offtake lines.
/// It expects a group of the form:
/// ```
/// PFC|1
/// DPP|1|20230401|.0000355
/// DPP|2|20230401|.0000354
/// DPP|3|20230401|.0000361
/// DPP|4|20230401|.0000354
/// DPP|5|20230401|.0000354
/// DPP|6|20230401|.0000349
/// DPP|7|20230401|.0000362
/// ```
fn profile_class_group(line: Span) -> IResult<ProfileGroups> {
    let (input, profile_class) = profile_class(line)?;
    let (rest, values) = offtake_group_parser(input)?;

    Ok((
        rest,
        ProfileGroups {
            profile_class,
            values,
        },
    ))
}

/// Looping parser for offtake lines starting with `DPP`, we use the start of each line as the seperator.
/// We know we have finished when we reach a new profile group starting with `PFC`
/// or the end of the file with `ZPT`
fn offtake_group_parser(input: Span) -> IResult<Vec<OfftakeRow>> {
    collect_separated_terminated(offtake_line, tag("DPP").peek(), offtake_group_terminator)
        .parse(input)
}

/// The end of set of offtake lines could either be a new profile group or the trailer
fn offtake_group_terminator(input: Span) -> IResult<Span> {
    tag("PFC").peek().or(tag("ZPT").peek()).parse(input)
}

/// Parse a single offtake line
fn offtake_line(line: Span) -> IResult<OfftakeRow> {
    let mut parser = tuple((
        tag("DPP"),
        tag("|"),
        digit1,
        tag("|"),
        date,
        tag("|"),
        double,
        line_ending,
    ));
    let (rest, (_, _, location, _, day, _, value, _)) = parser.parse(line)?;

    let location = location.parse().map_err(|err| {
        nom::Err::Error(ParseError::new(
            format!("failed to parse location: {}", err),
            Some("location should be a number between 1 and 50".to_string()),
            line,
        ))
    })?;

    let row = OfftakeRow {
        location,
        day,
        value,
    };
    Ok((rest, row))
}

/// Parse the date in the form `YearMonthDay`
fn date(input: Span) -> IResult<chrono::NaiveDate> {
    let mut parser = digit1;

    let (rest, raw_digits) = parser.parse(input)?;

    let day = chrono::NaiveDate::parse_from_str(&raw_digits, "%Y%m%d").map_err(|err| {
        nom::Err::Error(ParseError::new(
            format!("failed to parse date: {}", err),
            Some("date should be in the format YearMonthDay".to_string()),
            input,
        ))
    })?;

    Ok((rest, day))
}

/// Parse the date and time in the form `YearMonthDayHourMinuteSecond`
fn date_time(input: Span) -> IResult<chrono::NaiveDateTime> {
    let mut parser = digit1;

    let (rest, raw_digits) = parser.parse(input)?;

    let date_time =
        chrono::NaiveDateTime::parse_from_str(&raw_digits, "%Y%m%d%H%M%S").map_err(|err| {
            nom::Err::Error(ParseError::new(
                format!("failed to parse date: {}", err),
                Some("date should be in the format YearMonthDayHourMinuteSecond".to_string()),
                input,
            ))
        })?;

    Ok((rest, date_time))
}

#[cfg(test)]
mod test_parsers {

    use crate::electricity_offtake_parser::Location;

    use super::*;

    #[test]
    fn test_date_time_valid() {
        // valid input
        let input = "20230102121520";
        let (_, got) = date_time(input.into()).unwrap();
        let d = chrono::NaiveDate::from_ymd_opt(2023, 1, 2).unwrap();
        let t = chrono::NaiveTime::from_hms_opt(12, 15, 20).unwrap();
        let want = chrono::NaiveDateTime::new(d, t);
        assert_eq!(want, got);
    }

    #[test]
    fn test_date_time_invalid() {
        let input = "20230552121520";
        assert!(date_time(input.into()).is_err())
    }

    #[test]
    fn test_day_valid() {
        // valid input
        let input = "20230102";
        let (_, got) = date(input.into()).unwrap();
        let want = chrono::NaiveDate::from_ymd_opt(2023, 1, 2).unwrap();
        assert_eq!(want, got);
    }

    #[test]
    fn test_day_invalid() {
        let input = "20230552";
        assert!(date_time(input.into()).is_err())
    }

    #[test]
    fn test_offtake_line_valid() {
        let input = "DPP|1|20230401|.0000491\n";
        let (_, got) = offtake_line(input.into()).unwrap();
        let day = chrono::NaiveDate::from_ymd_opt(2023, 4, 1).unwrap();
        let want = OfftakeRow {
            location: Location::Location1,
            day,
            value: 0.0000491,
        };
        assert_eq!(want, got)
    }
    #[test]
    fn test_offtake_line_invalid() {
        let input = "DPP|1/20230401|.0000491\n";
        assert!(offtake_line(input.into()).is_err());
    }

    #[test]
    fn test_offtake_group_middle() {
        let input = "DPP|1|20230401|.0000491
DPP|2|20230401|.0000472
DPP|3|20230401|.000039
PFC|2
";
        let (_, got) = offtake_group_parser(input.into()).unwrap();
        let day = chrono::NaiveDate::from_ymd_opt(2023, 4, 1).unwrap();
        let want = vec![
            OfftakeRow {
                location: Location::Location1,
                day,
                value: 0.0000491,
            },
            OfftakeRow {
                location: Location::Location2,
                day,
                value: 0.0000472,
            },
            OfftakeRow {
                location: Location::Location3,
                day,
                value: 0.000039,
            },
        ];
        assert_eq!(want, got)
    }

    #[test]
    fn test_offtake_group_final() {
        let input = "DPP|1|20230401|.0000491
DPP|2|20230401|0
DPP|3|20230401|.000039
ZPT|
";
        let (_, got) = offtake_group_parser(input.into()).unwrap();
        let day = chrono::NaiveDate::from_ymd_opt(2023, 4, 1).unwrap();
        let want = vec![
            OfftakeRow {
                location: Location::Location1,
                day,
                value: 0.0000491,
            },
            OfftakeRow {
                location: Location::Location2,
                day,
                value: 0.0,
            },
            OfftakeRow {
                location: Location::Location3,
                day,
                value: 0.000039,
            },
        ];
        assert_eq!(want, got)
    }

    #[test]
    fn test_profile_line() {
        let input = "PFC|2\n";
        let (_, got) = profile_class(input.into()).unwrap();
        let want = ProfileClass::Class2;

        assert_eq!(want, got);
    }

    #[test]
    fn test_profile_groups() {
        let input = "PFC|1
DPP|1|20230401|.1
DPP|2|20230401|.2
PFC|2
DPP|1|20230401|.1
DPP|2|20230401|.2
PFC|3
DPP|1|20230401|.1
DPP|2|20230401|.2
ZPT|140554|1564691006
";

        let (_, got) = offtake_file(input.into()).unwrap();

        let day = chrono::NaiveDate::from_ymd_opt(2023, 4, 1).unwrap();
        let offtake_values = [
            OfftakeRow {
                location: Location::Location1,
                day,
                value: 0.1,
            },
            OfftakeRow {
                location: Location::Location2,
                day,
                value: 0.2,
            },
        ];
        let want = vec![
            ProfileGroups {
                profile_class: ProfileClass::Class1,
                values: offtake_values.to_vec(),
            },
            ProfileGroups {
                profile_class: ProfileClass::Class2,
                values: offtake_values.to_vec(),
            },
            ProfileGroups {
                profile_class: ProfileClass::Class3,
                values: offtake_values.to_vec(),
            },
        ];

        assert_eq!(want, got)
    }
}
