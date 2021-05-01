use crate::{
    context::Context, error::ShellError, evaluate::Value,
    parser::command::classified::external::ExternalCommand,
};
use alloc::vec::Vec;

#[allow(unused)]
pub fn run_external_command(
    command: ExternalCommand,
    context: &mut Context,
    input: Option<Vec<Value>>,
    is_last: bool,
) -> Result<Option<Vec<Value>>, ShellError> {
    todo!()
}
