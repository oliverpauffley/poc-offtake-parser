//! # Offtake file parsing
//! Using nom parsing to extract data from offtake data files.
//!
//! ## Nom
//! Nom is a combinator parser so it builds the complete parser out of small functions
//! that do increasingly complicated logic. More information on nom can be found [here](https://docs.rs/nom/latest/nom/).
//!
//! We also use [nom_supreme](https://docs.rs/nom-supreme/latest/nom_supreme/) for some utilities to make parsing a little cleaner
//! and [nom_locate](https://docs.rs/nom_locate/latest/nom_locate/) to produce rich error types.
//!
use std::time::Instant;

use errors::FileError;
use errors::ParseError;
use nom_locate::LocatedSpan;

mod electricity_offtake_parser;
mod errors;
mod gas_offtake_parser;

/// example main
fn main() -> miette::Result<()> {
    let file = include_str!("AUC.txt");

    let result = electricity_offtake_parser::parse_flow_file(file.into());

    let now = Instant::now();

    match result {
        Ok((_rest, _file)) => {
            let elasped = now.elapsed();
            println!("{elasped:?}]");
            Ok(())
        }
        Err(e) => match e {
            nom::Err::Error(err) => {
                let offset = err.offset().into();
                Err(FileError {
                    src: file,
                    file_name: "file_name",
                    bad_bit: miette::SourceSpan::new(offset, 0.into()),
                    message: err.message,
                    help: err.help,
                }
                .into())
            }
            _ => unreachable!(),
        },
    }
}

pub type Span<'a> = LocatedSpan<&'a str>;
pub type IResult<'a, O> = nom::IResult<Span<'a>, O, ParseError<'a>>;
