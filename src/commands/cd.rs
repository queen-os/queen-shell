use alloc::{string::String, sync::Arc, vec::Vec};
use core::sync::atomic::AtomicBool;

use serde::Deserialize;

use crate::{
    commands::{Command, RunnableContext},
    context::CommandRegistry,
    error::ShellError,
    evaluate::{CallInfo, Value},
    parser::syntax_shape::SyntaxShape,
    shell::Shell,
    signature::Signature,
};

#[derive(Deserialize)]
pub struct CdArgs {
    pub dst: Option<String>,
}

pub struct Cd;

impl Command for Cd {
    fn name(&self) -> &str {
        "cd"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .optional(
                "destination",
                SyntaxShape::Path,
                "the directory to change to",
            )
            .desc(self.usage())
    }

    fn usage(&self) -> &str {
        "Change to a new path."
    }

    fn run(
        &self,
        call_info: CallInfo,
        input: Option<Vec<Value>>,
        ctrl_c: Arc<AtomicBool>,
        shell: Arc<dyn Shell>,
        _registry: &CommandRegistry,
    ) -> Result<Option<Vec<Value>>, ShellError> {
        call_info.process(&shell, ctrl_c, cd, input)?.run()
    }
}

fn cd(args: CdArgs, ctx: &RunnableContext) -> Result<Option<Vec<Value>>, ShellError> {
    ctx.shell.cd(args)
}
