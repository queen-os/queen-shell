use crate::parser::{
    span::{HasSpan, Span},
    token::{SpannedToken, Token},
};
use alloc::{string::String, vec::Vec};

pub type ExternalArg = String;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExternalArgs {
    pub list: Vec<ExternalArg>,
    pub span: Span,
}

impl ExternalArgs {
    pub fn new(list: Vec<ExternalArg>, span: Span) -> Self {
        Self { list, span }
    }

    pub fn iter(&self) -> impl Iterator<Item = &ExternalArg> {
        self.list.iter()
    }

    pub fn from_tokens(
        tokens: &mut impl Iterator<Item = SpannedToken>,
        source: &str,
        span: Span,
    ) -> Self {
        let list = tokens
            .map(|spanned| match spanned.item {
                Token::String(s) => Some(s.string(source)),
                Token::Bare | Token::ExternalWord | Token::Flag(_) | Token::GlobPattern => {
                    Some(spanned.span.string(source))
                }
                Token::Separator | Token::Whitespace => None,
            })
            .flatten()
            .collect::<Vec<_>>();
        Self { list, span }
    }
}

impl core::ops::Deref for ExternalArgs {
    type Target = [ExternalArg];

    fn deref(&self) -> &[ExternalArg] {
        &self.list
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExternalCommand {
    pub name: String,
    pub name_span: Span,
    pub args: ExternalArgs,
}

impl ExternalCommand {
    pub fn new(name: String, name_span: Span, args: ExternalArgs) -> Self {
        Self {
            name,
            name_span,
            args,
        }
    }
}

impl HasSpan for ExternalCommand {
    fn span(&self) -> Span {
        self.name_span.until(self.args.span)
    }
}
