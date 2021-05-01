use crate::{
    context::Context, error::ShellError, evaluate::Value,
    parser::command::classified::internal::InternalCommand,
};
use alloc::vec::Vec;

#[inline]
pub fn run_internal_command(
    command: InternalCommand,
    context: &mut Context,
    input: Option<Vec<Value>>,
    source: &str,
) -> Result<Option<Vec<Value>>, ShellError> {
    let internal_command = context.expect_command(command.name.as_str())?;
    context.run_command(internal_command, command.args, source, input)
}
