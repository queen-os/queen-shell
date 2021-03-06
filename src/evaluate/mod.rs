use crate::{
    commands::CommandRef,
    context::CommandRegistry,
    error::ShellError,
    evaluate::call_info::EvaluatedArgs,
    parser::{
        hir,
        token::{SpannedToken, Token},
    },
};
use alloc::{string::String, vec::Vec};
use indexmap::IndexMap;

pub(crate) use call_info::CallInfo;
pub(crate) use value::Value;

pub mod call_info;
pub mod value;

fn evaluate_expr(spanned: &SpannedToken, source: &str) -> Result<Value, ShellError> {
    let token = &spanned.item;
    match token {
        Token::String(s) => Ok(Value::String(s.string(source))),
        Token::Bare | Token::GlobPattern | Token::ExternalWord => {
            Ok(Value::String(spanned.span.string(source)))
        }
        Token::Flag(_) | Token::Whitespace | Token::Separator => Err(ShellError::runtime_error(
            format!("unexpected {}", token.desc()),
        )),
    }
}

pub(crate) fn evaluate_args(
    call: hir::Call,
    _command: CommandRef,
    _registry: &CommandRegistry,
    source: &str,
) -> Result<EvaluatedArgs, ShellError> {
    let positional: Result<Option<Vec<_>>, _> = call
        .positional
        .as_ref()
        .map(|p| p.iter().map(|s| evaluate_expr(s, source)).collect())
        .transpose();
    let positional = positional?;
    let named: Result<Option<IndexMap<String, Value>>, ShellError> = call
        .named
        .as_ref()
        .map(|n| {
            let mut results = IndexMap::new();
            for (name, value) in n.named.iter() {
                match value {
                    hir::NamedValue::PresentSwitch(_) => {
                        results.insert(name.clone(), Value::Boolean(true));
                    }
                    hir::NamedValue::Value(ref expr) => {
                        results.insert(name.clone(), evaluate_expr(expr, source)?);
                    }
                    _ => {}
                };
            }

            Ok(results)
        })
        .transpose();
    let named = named?;

    Ok(EvaluatedArgs::new(positional, named))
}
