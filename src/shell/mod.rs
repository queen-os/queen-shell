use crate::{
    commands::{CdArgs, LsArgs, MkDirArgs, RunnableContext},
    error::ShellError,
    evaluate::Value,
};
use alloc::{boxed::Box, string::String, vec::Vec};
use core::{future::Future, pin::Pin};

#[cfg(feature = "std")]
pub mod std;
#[cfg(feature = "std")]
pub use self::std::StdShell;

pub trait Shell: core::fmt::Debug + Send + Sync {
    fn name(&self) -> &str;

    fn homedir(&self) -> Option<String>;

    fn readline(&self) -> Pin<Box<dyn Future<Output = String>>>;

    fn print(&self, s: &str);

    fn ls(&self, args: LsArgs, context: &RunnableContext)
        -> Result<Option<Vec<Value>>, ShellError>;

    fn cd(&self, args: CdArgs) -> Result<Option<Vec<Value>>, ShellError>;

    // fn cp(&self, args: CopyArgs) -> Result<Option<Vec<Value>>, ShellError>;
    fn mkdir(&self, args: MkDirArgs) -> Result<Option<Vec<Value>>, ShellError>;
    // fn mv(&self, args: MoveArgs, name: Tag, path: &str) -> Result<OutputStream, ShellError>;
    // fn rm(&self, args: RemoveArgs, name: Tag, path: &str) -> Result<OutputStream, ShellError>;
    fn path(&self) -> String;
    // fn pwd(&self) -> Result<Option<Vec<Value>>, ShellError>;
    // fn set_path(&mut self, path: String);
}
