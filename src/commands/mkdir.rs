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

#[derive(Deserialize)]
pub struct MkDirArgs {
    pub rest: Vec<String>,
}

pub struct MkDir;

impl Command for MkDir {
    fn name(&self) -> &str {
        "mkdir"
    }

    fn usage(&self) -> &str {
        "Make directories, creates intermediary directories as required."
    }

    fn signature(&self) -> Signature {
        Signature::build("mkdir")
            .rest(SyntaxShape::Path, "the name of the path to create")
            .desc(self.usage())
    }

    fn run(
        &self,
        call_info: CallInfo,
        input: Option<Vec<Value>>,
        ctrl_c: Arc<AtomicBool>,
        shell: Arc<dyn Shell>,
        _registry: &CommandRegistry,
    ) -> Result<Option<Vec<Value>>, ShellError> {
        call_info.process(&shell, ctrl_c, mkdir, input)?.run()
    }
}

fn mkdir(args: MkDirArgs, ctx: &RunnableContext) -> Result<Option<Vec<Value>>, ShellError> {
    ctx.shell.mkdir(args)
}
