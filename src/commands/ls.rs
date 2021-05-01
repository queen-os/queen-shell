use crate::{
    commands::{Command, RunnableContext},
    context::CommandRegistry,
    error::ShellError,
    evaluate::{CallInfo, Value},
    parser::syntax_shape::SyntaxShape,
    shell::Shell,
    signature::Signature,
};
use alloc::{string::String, sync::Arc, vec::Vec};
use core::sync::atomic::AtomicBool;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LsArgs {
    pub path: Option<String>,
}

pub struct Ls;

impl Command for Ls {
    fn name(&self) -> &str {
        "ls"
    }

    fn signature(&self) -> Signature {
        Signature::build("ls")
            .optional(
                "path",
                SyntaxShape::Pattern,
                "a path to get the directory contents from",
            )
            .desc(self.usage())
    }

    fn usage(&self) -> &str {
        "View the contents of the current or given path."
    }

    fn run(
        &self,
        call_info: CallInfo,
        input: Option<Vec<Value>>,
        ctrl_c: Arc<AtomicBool>,
        shell: Arc<dyn Shell>,
        _registry: &CommandRegistry,
    ) -> Result<Option<Vec<Value>>, ShellError> {
        call_info.process(&shell, ctrl_c, ls, input)?.run()
    }
}

#[inline]
fn ls(args: LsArgs, ctx: &RunnableContext) -> Result<Option<Vec<Value>>, ShellError> {
    ctx.shell.ls(args, ctx)
}
