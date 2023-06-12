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
use std::{error::Error, str::FromStr};

use chrono::NaiveDate;
use strum::VariantNames;
use strum_macros::EnumVariantNames;

mod parser;

pub type SCWVFile = Vec<SeasonalCompositeWeatherVariableRow>;

#[derive(Debug, PartialEq, Clone)]
pub struct SeasonalCompositeWeatherVariableRow {
    date: NaiveDate,
    sc: f64,
    no: f64,
    nw: f64,
    ne: f64,
    em: f64,
    wm: f64,
    wn: f64,
    ws: f64,
    ea: f64,
    nt: f64,
    se: f64,
    so: f64,
    sw: f64,
}

impl SeasonalCompositeWeatherVariableRow {
    fn new_from_array(date: NaiveDate, values: [f64; 13]) -> Self {
        Self {
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
        }
    }
}

pub type ALPDAFFile = Vec<ALPDAFRow>;

#[derive(Debug, PartialEq)]
pub struct ALPDAFRow {
    ldz: LocalDistributionZone,
    end_user_category: EndUserCategory,
    date: NaiveDate,
    annual_load_profile: f64,
    daily_adjustment_factor: f64,
}

#[derive(Debug, PartialEq, EnumVariantNames)]
enum LocalDistributionZone {
    SC,
    NO,
    NE,
    NW,
    WN,
    WM,
    EM,
    EA,
    WS,
    SW,
    SO,
    SE,
    NT,
    LC,
    LO,
    LS,
    LT,
    LW,
}

impl FromStr for LocalDistributionZone {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SC" => Ok(LocalDistributionZone::SC),
            "NO" => Ok(LocalDistributionZone::NO),
            "NE" => Ok(LocalDistributionZone::NE),
            "NW" => Ok(LocalDistributionZone::NW),
            "WN" => Ok(LocalDistributionZone::WN),
            "WM" => Ok(LocalDistributionZone::WM),
            "EM" => Ok(LocalDistributionZone::EM),
            "EA" => Ok(LocalDistributionZone::EA),
            "WS" => Ok(LocalDistributionZone::WS),
            "SW" => Ok(LocalDistributionZone::SW),
            "SO" => Ok(LocalDistributionZone::SO),
            "SE" => Ok(LocalDistributionZone::SE),
            "NT" => Ok(LocalDistributionZone::NT),
            "LC" => Ok(LocalDistributionZone::LC),
            "LO" => Ok(LocalDistributionZone::LO),
            "LS" => Ok(LocalDistributionZone::LS),
            "LT" => Ok(LocalDistributionZone::LT),
            "LW" => Ok(LocalDistributionZone::LW),
            _ => Err(format!(
                "cannot parse {} into a local distribution zone, should be one of {:?}",
                s,
                LocalDistributionZone::VARIANTS
            )
            .into()),
        }
    }
}

/// EndUserCategory is based on the Annual Quantity,
/// the winter consumption and region of a meterpoint
/// # Example
/// E2201BND => domestic, non prepayment with MWh pa between 0 and 73.2
/// E2202BPI => industrial and commercial, prepayment with MWh pa between 73.2 and 293
#[derive(Debug, PartialEq)]
struct EndUserCategory {
    band: Band,
    user_group: UserGroup,
}

impl EndUserCategory {
    fn new(band: Band, user_group: UserGroup) -> Self {
        Self { band, user_group }
    }
}

#[derive(Debug, PartialEq, EnumVariantNames)]
enum Band {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl FromStr for Band {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "01" => Ok(Band::One),
            "02" => Ok(Band::Two),
            "03" => Ok(Band::Three),
            "04" => Ok(Band::Four),
            "05" => Ok(Band::Five),
            "06" => Ok(Band::Six),
            "07" => Ok(Band::Seven),
            "08" => Ok(Band::Eight),
            "09" => Ok(Band::Nine),
            _ => Err(format!("unknown end user group band {}", s).into()),
        }
    }
}

#[derive(Debug, PartialEq)]
struct UserGroup {
    consumer_type: ConsumerType,
    meter_type: MeterType,
}

impl FromStr for UserGroup {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BND" => Ok(UserGroup {
                consumer_type: ConsumerType::Domestic,
                meter_type: MeterType::Credit,
            }),
            "BPD" => Ok(UserGroup {
                consumer_type: ConsumerType::Domestic,
                meter_type: MeterType::Prepayment,
            }),
            "BNI" => Ok(UserGroup {
                consumer_type: ConsumerType::Industry,
                meter_type: MeterType::Credit,
            }),
            "BPI" => Ok(UserGroup {
                consumer_type: ConsumerType::Industry,
                meter_type: MeterType::Prepayment,
            }),

            "B" => Ok(UserGroup {
                consumer_type: ConsumerType::Industry,
                meter_type: MeterType::Credit,
            }),
            "W01" => Ok(UserGroup {
                consumer_type: ConsumerType::Industry,
                meter_type: MeterType::Credit,
            }),
            "W02" => Ok(UserGroup {
                consumer_type: ConsumerType::Industry,
                meter_type: MeterType::Credit,
            }),
            "W03" => Ok(UserGroup {
                consumer_type: ConsumerType::Industry,
                meter_type: MeterType::Credit,
            }),
            "W04" => Ok(UserGroup {
                consumer_type: ConsumerType::Industry,
                meter_type: MeterType::Credit,
            }),
            _ => Err(format!("unknown consumer meter group {} ", s).into()),
        }
    }
}

#[derive(Debug, PartialEq, EnumVariantNames)]
enum ConsumerType {
    Domestic,
    /// Large scale industrial or commercial sites that as of writing we do
    /// not have in our customer base
    Industry,
}

#[derive(Debug, PartialEq, EnumVariantNames)]
enum MeterType {
    Prepayment,
    Credit,
}
