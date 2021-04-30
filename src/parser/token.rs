use super::span::{Span, Spanned};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Token {
    String(Span),
    Bare,
    Flag(Span),
    Whitespace,
    Separator,
    GlobPattern,
    ExternalWord,
}

impl Token {
    pub fn desc(&self) -> &'static str {
        match self {
            Token::String(_) => "string",
            Token::Bare => "bare",
            Token::Flag(_) => "flag",
            Token::Whitespace => "whitespace",
            Token::Separator => "separator",
            Token::GlobPattern => "glob pattern",
            Token::ExternalWord => "external word",
        }
    }
}

pub type SpannedToken = Spanned<Token>;

impl From<&SpannedToken> for Span {
    fn from(token: &SpannedToken) -> Span {
        token.span
    }
}
