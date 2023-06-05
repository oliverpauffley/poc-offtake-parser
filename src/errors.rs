use crate::Span;
use thiserror::Error;

/// Any error from parsing a flow file should be handled with this error.
/// The span locates the error in the source file.
/// The message should be a helpful message that explains why the parse failed.
/// The help is optional additional context.
#[derive(Debug, PartialEq, Error)]
#[error("Could not parse flow file")]
pub struct ParseError<'a> {
    span: Span<'a>,
    pub message: String,
    pub help: Option<String>,
}

impl<'a> ParseError<'a> {
    pub fn new(message: String, help: Option<String>, span: Span<'a>) -> Self {
        Self {
            span,
            message,
            help,
        }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn line(&self) -> u32 {
        self.span().location_line()
    }

    pub fn offset(&self) -> usize {
        self.span().location_offset()
    }
}

// make our custom error convertable into a nom error
impl<'a> nom::error::ParseError<Span<'a>> for ParseError<'a> {
    fn from_error_kind(input: Span<'a>, kind: nom::error::ErrorKind) -> Self {
        match kind {
            nom::error::ErrorKind::Tag => {
                Self::new("got the incorrect symbol".to_string(), None, input)
            }
            nom::error::ErrorKind::Float => {
                Self::new("expected a decimal number here".to_string(), None, input)
            }
            _ => Self::new(format!("parse error {:?}", kind), None, input),
        }
    }

    fn append(_input: Span<'a>, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }

    fn from_char(input: Span<'a>, c: char) -> Self {
        Self::new(format!("unexpected character '{}'", c), None, input)
    }
}

impl<'a> nom_supreme::tag::TagError<Span<'a>, &str> for ParseError<'a> {
    fn from_tag(input: Span<'a>, tag: &str) -> Self {
        Self::new(
            format!("got the incorrect symbol, expected: {:?}", tag),
            None,
            input,
        )
    }
}

/// The main returned error when a file fails translation. This is used to
/// generate a nice error message when a parser fails
#[derive(thiserror::Error, Debug, miette::Diagnostic)]
#[error("could not parse flow file: {file_name}")]
pub struct FileError {
    #[source_code]
    pub src: &'static str,

    pub file_name: &'static str,

    #[label("{message}")]
    pub bad_bit: miette::SourceSpan,

    pub message: String,

    #[help]
    pub help: Option<String>,
}
