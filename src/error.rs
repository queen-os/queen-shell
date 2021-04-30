use crate::parser::span::Span;
use alloc::{boxed::Box, string::String};
use core::fmt;

/// A `ShellError` is a proximate error and a possible cause, which could have its own cause,
/// creating a cause chain.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub struct ShellError {
    pub error: ProximateShellError,
    pub cause: Option<Box<ShellError>>,
}

impl ShellError {
    pub fn parse_error(
        error: nom::Err<nom::error::Error<nom_locate::LocatedSpan<&str>>>,
    ) -> ShellError {
        let reason = Some(String::from("parse error"));
        match error {
            nom::Err::Incomplete(_) => {
                ProximateShellError::ParseError(Span::unknown(), reason).start()
            }
            nom::Err::Failure(span) | nom::Err::Error(span) => {
                ProximateShellError::ParseError(Span::from(span.input), reason).start()
            }
        }
    }
    pub fn runtime_error(reason: impl Into<String>) -> ShellError {
        ProximateShellError::RuntimeError(reason.into()).start()
    }
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub enum ProximateShellError {
    ParseError(Span, Option<String>),
    RuntimeError(String),
}

impl ProximateShellError {
    pub fn start(self) -> ShellError {
        ShellError {
            cause: None,
            error: self,
        }
    }
}

impl fmt::Display for ProximateShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProximateShellError::ParseError(span, reason) => {
                    let reason = reason.clone().unwrap_or_default();
                    format!(
                        "{}{}{}",
                        " ".repeat(span.start()),
                        "^".repeat(span.len()),
                        reason
                    )
                }
                ProximateShellError::RuntimeError(reason) => {
                    reason.clone()
                }
            }
        )
    }
}
