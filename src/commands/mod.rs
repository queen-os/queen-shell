use crate::{
    context::CommandRegistry,
    deserializer::ConfigDeserializer,
    error::ShellError,
    evaluate::{CallInfo, Value},
    shell::Shell,
    signature::Signature,
};
use alloc::{sync::Arc, vec::Vec};
use core::sync::atomic::AtomicBool;
use serde::Deserialize;

mod cd;
mod ls;
mod mkdir;
mod classified;

pub use cd::{Cd, CdArgs};
pub use ls::{Ls, LsArgs};
pub use mkdir::{MkDir, MkDirArgs};
pub use classified::{run_external_command, run_internal_command};

pub trait Command: Send + Sync {
    fn name(&self) -> &str;

    fn signature(&self) -> Signature {
        Signature::new(self.name()).desc(self.usage())
    }

    fn usage(&self) -> &str;

    fn run(
        &self,
        call_info: CallInfo,
        input: Option<Vec<Value>>,
        ctrl_c: Arc<AtomicBool>,
        shell: Arc<dyn Shell>,
        registry: &CommandRegistry,
    ) -> Result<Option<Vec<Value>>, ShellError>;

    fn is_binary(&self) -> bool {
        false
    }
}

pub type CommandRef = Arc<dyn Command>;

pub struct RunnableContext {
    pub input: Option<Vec<Value>>,
    pub shell: Arc<dyn Shell>,
    pub ctrl_c: Arc<AtomicBool>,
}

pub type CommandCallback<T> = fn(T, &RunnableContext) -> Result<Option<Vec<Value>>, ShellError>;

pub struct RunnableArgs<T> {
    args: T,
    context: RunnableContext,
    callback: CommandCallback<T>,
}

impl<T> RunnableArgs<T> {
    #[inline]
    pub fn run(self) -> Result<Option<Vec<Value>>, ShellError> {
        (self.callback)(self.args, &self.context)
    }
}

impl CallInfo {
    pub(crate) fn process<'de, T: Deserialize<'de>>(
        &self,
        shell: &Arc<dyn Shell>,
        ctrl_c: Arc<AtomicBool>,
        callback: CommandCallback<T>,
        input: Option<Vec<Value>>,
    ) -> Result<RunnableArgs<T>, ShellError> {
        let mut deserializer = ConfigDeserializer::from_call_info(self.clone());
        Ok(RunnableArgs {
            args: T::deserialize(&mut deserializer)?,
            context: RunnableContext {
                shell: shell.clone(),
                ctrl_c,
                input,
            },
            callback,
        })
    }
}
