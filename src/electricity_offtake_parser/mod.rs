//! # Elexon files
//! The file is a pipe seperated list of offtake values for a given day in a specfic location. A single line looks like so:
//! ```
//! DPP|1|20230401|.0000491
//! ```
//!
//! Which is `identifer|location|date|value`
//!
//! These are grouped into Profile Class Groups with lines:
//! ```
//! PFC|1
//! ```
//! Where the number is the [profile class](https://www.elexon.co.uk/knowledgebase/profile-classes/) identifer
//!
use std::error::Error;
use std::str::FromStr;

mod parser;
pub use parser::parse_flow_file;

#[derive(Debug, PartialEq)]
pub struct WrappedFile {
    pub header: HeaderLine,
    pub file: OfftakeFile,
    pub trailer: TrailerLine,
}

#[derive(Debug, PartialEq)]
pub struct HeaderLine {
    // some stuff
    pub date_time: chrono::NaiveDateTime,
}

#[derive(Debug, PartialEq)]
pub struct TrailerLine {
    // some stuff
}

#[derive(Debug, PartialEq)]
pub struct OfftakeFile {
    pub profile_groups: Vec<ProfileGroups>,
}

#[derive(Debug, PartialEq)]
pub struct ProfileGroups {
    pub profile_class: ProfileClass,
    pub values: Vec<OfftakeRow>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct OfftakeRow {
    pub location: Location,
    pub day: chrono::NaiveDate,
    pub value: f64,
}

#[derive(Debug, PartialEq)]
pub enum ProfileClass {
    Class1,
    Class2,
    Class3,
    Class4,
    Class5,
    Class6,
    Class7,
    Class8,
}

impl FromStr for ProfileClass {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(ProfileClass::Class1),
            "2" => Ok(ProfileClass::Class2),
            "3" => Ok(ProfileClass::Class3),
            "4" => Ok(ProfileClass::Class4),
            "5" => Ok(ProfileClass::Class5),
            "6" => Ok(ProfileClass::Class6),
            "7" => Ok(ProfileClass::Class7),
            "8" => Ok(ProfileClass::Class8),
            _ => Err(format!("cannot parse {} into a profile class", s).into()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Location {
    Location1,
    Location2,
    Location3,
    Location4,
    Location5,
    Location6,
    Location7,
    Location8,
    Location9,
    Location10,
    Location11,
    Location12,
    Location13,
    Location14,
    Location15,
    Location16,
    Location17,
    Location18,
    Location19,
    Location20,
    Location21,
    Location22,
    Location23,
    Location24,
    Location25,
    Location26,
    Location27,
    Location28,
    Location29,
    Location30,
    Location31,
    Location32,
    Location33,
    Location34,
    Location35,
    Location36,
    Location37,
    Location38,
    Location39,
    Location40,
    Location41,
    Location42,
    Location43,
    Location44,
    Location45,
    Location46,
    Location47,
    Location48,
    Location49,
    Location50,
}

impl FromStr for Location {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Location::Location1),
            "2" => Ok(Location::Location2),
            "3" => Ok(Location::Location3),
            "4" => Ok(Location::Location4),
            "5" => Ok(Location::Location5),
            "6" => Ok(Location::Location6),
            "7" => Ok(Location::Location7),
            "8" => Ok(Location::Location8),
            "9" => Ok(Location::Location9),
            "10" => Ok(Location::Location10),
            "11" => Ok(Location::Location11),
            "12" => Ok(Location::Location12),
            "13" => Ok(Location::Location13),
            "14" => Ok(Location::Location14),
            "15" => Ok(Location::Location15),
            "16" => Ok(Location::Location16),
            "17" => Ok(Location::Location17),
            "18" => Ok(Location::Location18),
            "19" => Ok(Location::Location19),
            "20" => Ok(Location::Location20),
            "21" => Ok(Location::Location21),
            "22" => Ok(Location::Location22),
            "23" => Ok(Location::Location23),
            "24" => Ok(Location::Location24),
            "25" => Ok(Location::Location25),
            "26" => Ok(Location::Location26),
            "27" => Ok(Location::Location27),
            "28" => Ok(Location::Location28),
            "29" => Ok(Location::Location29),
            "30" => Ok(Location::Location30),
            "31" => Ok(Location::Location31),
            "32" => Ok(Location::Location32),
            "33" => Ok(Location::Location33),
            "34" => Ok(Location::Location34),
            "35" => Ok(Location::Location35),
            "36" => Ok(Location::Location36),
            "37" => Ok(Location::Location37),
            "38" => Ok(Location::Location38),
            "39" => Ok(Location::Location39),
            "40" => Ok(Location::Location40),
            "41" => Ok(Location::Location41),
            "42" => Ok(Location::Location42),
            "43" => Ok(Location::Location43),
            "44" => Ok(Location::Location44),
            "45" => Ok(Location::Location45),
            "46" => Ok(Location::Location46),
            "47" => Ok(Location::Location47),
            "48" => Ok(Location::Location48),
            "49" => Ok(Location::Location49),
            "50" => Ok(Location::Location50),
            _ => Err(format!("cannot parse {} into a location", s).into()),
        }
    }
}
