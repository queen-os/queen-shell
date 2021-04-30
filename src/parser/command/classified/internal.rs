use crate::parser::{
    hir,
    span::{HasSpan, Span},
};
use alloc::string::String;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InternalCommand {
    pub name: String,
    pub name_span: Span,
    pub args: hir::Call,
}

impl InternalCommand {
    pub fn new(name: String, name_span: Span, args: hir::Call) -> Self {
        Self {
            name,
            name_span,
            args,
        }
    }
}

impl HasSpan for InternalCommand {
    fn span(&self) -> Span {
        let start = self.name_span;

        start.until(self.args.span)
    }
}
