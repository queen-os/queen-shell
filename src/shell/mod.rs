use crate::{commands::RunnableContext, error::ShellError, evaluate::Value};
use alloc::string::String;

pub trait Shell: core::fmt::Debug {
    fn name(&self) -> String;
    // fn homedir(&self) -> Option<PathBuf>;

    // fn ls(&self, args: LsArgs, context: &RunnableContext)
    //     -> Result<Option<Vec<Value>>, ShellError>;
    // fn cd(&self, args: CdArgs) -> Result<Option<Vec<Value>>, ShellError>;
    // fn cp(&self, args: CopyArgs) -> Result<Option<Vec<Value>>, ShellError>;
    // fn mkdir(&self, args: MkdirArgs) -> Result<Option<Vec<Value>>, ShellError>;
    // fn mv(&self, args: MoveArgs, name: Tag, path: &str) -> Result<OutputStream, ShellError>;
    // fn rm(&self, args: RemoveArgs, name: Tag, path: &str) -> Result<OutputStream, ShellError>;
    // fn path(&self) -> String;
    // fn pwd(&self) -> Result<Option<Vec<Value>>, ShellError>;
    // fn set_path(&mut self, path: String);
}
