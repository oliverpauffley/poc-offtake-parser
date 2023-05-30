use nom::character::complete::alpha1;
use nom::error::ParseError;
use nom::sequence::tuple;
use nom::{character::complete::digit1, IResult};
use nom::{error_position, Err};
use nom_locate::LocatedSpan;
use nom_supreme::error::BaseErrorKind;

fn main() -> miette::Result<()> {
    let file = include_str!("AUC.txt");

    Ok(())
}

pub type Span<'a> = LocatedSpan<&'a str>;
#[derive(thiserror::Error, Debug, miette::Diagnostic)]
#[error("bad input")]
struct BadInput {
    #[source_code]
    src: &'static str,

    #[label("{kind}")]
    bad_bit: miette::SourceSpan,

    kind: BaseErrorKind<&'static str, Box<dyn std::error::Error + Send + Sync>>,
}

fn header_line<'a, E: ParseError<Span<'a>>>(line: Span<'a>) -> IResult<&str, HeaderLine, E> {
    let mut parser = tuple((
        alpha1,
        nom::character::complete::char('|'),
        alpha1,
        nom::character::complete::char('|'),
        alpha1,
        nom::character::complete::char('|'),
        alpha1,
        nom::character::complete::char('|'),
        alpha1,
        nom::character::complete::char('|'),
        alpha1,
        nom::character::complete::char('|'),
        date,
    ));

    let (rest, (_, _, _, _, _, _, _, _, _, _, _, _, date)) = parser(line)?;

    Ok((&rest, HeaderLine { date }))
}

fn date<'a, E: ParseError<Span<'a>>>(
    input: Span<'a>,
) -> IResult<Span<'a>, chrono::NaiveDateTime, E> {
    let (rest, raw_digits) = digit1(input)?;

    let date_time = chrono::NaiveDateTime::parse_from_str(&raw_digits, "%Y%m%d%H%M%S")
        .map_err(|e| ParseError::from_error_kind(input.into(), nom::error::ErrorKind::Fail))?;

    Ok((rest, date_time))
}

struct WrappedFile {
    header: HeaderLine,
    file: OfftakeFile,
    trailer: TrailerLine,
}

#[derive(Debug)]
struct HeaderLine {
    // some stuff
    date: chrono::NaiveDateTime,
}

struct TrailerLine {
    // some stuff
}

struct OfftakeFile {
    profile_groups: Vec<ProfileGroups>,
}

struct ProfileGroups {
    profile_class: ProfileClass,
    values: Vec<OfftakeRow>,
}

struct OfftakeRow {
    location: Location,
    day: chrono::NaiveDate,
    value: f64,
}

enum ProfileClass {
    Class1,
    Class2,
    Class3,
    Class4,
}

enum Location {
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
}
