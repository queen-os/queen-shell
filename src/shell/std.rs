use crate::{
    commands::{cd::CdArgs, ls::LsArgs, mkdir::MkDirArgs, RunnableContext},
    error::ShellError,
    evaluate::Value,
};
use std::{env, env::current_dir, path::PathBuf, sync::atomic::Ordering};

#[derive(Debug, Clone, Default)]
pub struct StdShell {}

impl StdShell {
    #[inline]
    pub const fn new() -> Self {
        Self {}
    }
}

impl super::Shell for StdShell {
    fn name(&self) -> &str {
        "Std file system shell"
    }

    fn ls(
        &self,
        LsArgs { path }: LsArgs,
        context: &RunnableContext,
    ) -> Result<Option<Vec<Value>>, ShellError> {
        let ctrl_c = context.ctrl_c.clone();
        let path = match path {
            None => {
                if is_dir_empty(&self.path().into()) {
                    return Ok(None);
                } else {
                    PathBuf::from("./*")
                }
            }
            Some(p) => {
                let mut p: PathBuf = p.into();
                if p.is_dir() {
                    if is_dir_empty(&p) {
                        return Ok(None);
                    }
                    p.push("*");
                }
                p
            }
        };
        let mut paths = match glob::glob(&path.to_string_lossy()) {
            Ok(g) => Ok(g),
            Err(_) => Err(ShellError::runtime_error("Invalid File or Pattern")),
        }?
        .peekable();
        if paths.peek().is_none() {
            return Err(ShellError::runtime_error("Invalid File or Pattern"));
        }
        let mut results = vec![];
        for path in paths {
            if ctrl_c.load(Ordering::Acquire) {
                break;
            }
            if let Ok(path) = path {
                if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                    let item = format!("{}: {}", get_path_type(&path), name);
                    results.push(Value::String(item));
                }
            }
        }
        if results.is_empty() {
            Ok(None)
        } else {
            Ok(Some(results))
        }
    }

    fn cd(&self, args: CdArgs) -> Result<Option<Vec<Value>>, ShellError> {
        let target = match args.dst {
            None => match dirs::home_dir() {
                Some(o) => o,
                _ => {
                    return Err(ShellError::runtime_error(
                        "Can not change to home directory",
                    ));
                }
            },
            Some(target) => target.into(),
        };
        if target.exists() && !target.is_dir() {
            return Err(ShellError::runtime_error(format!(
                "{} is not a directory",
                target.to_string_lossy().to_string()
            )));
        }
        let path = PathBuf::from(self.path());
        env::set_current_dir(path.join(&target)).expect("cannot to set current directory");
        Ok(None)
    }

    fn mkdir(
        &self,
        MkDirArgs { rest: directories }: MkDirArgs,
    ) -> Result<Option<Vec<Value>>, ShellError> {
        let full_path = PathBuf::from(self.path());
        for dir in directories {
            let create_at = {
                let mut loc = full_path.clone();
                loc.push(&dir);
                loc
            };

            let dir_res = std::fs::create_dir_all(create_at);
            if let Err(reason) = dir_res {
                return Err(ShellError::runtime_error(reason.to_string()));
            }
        }
        Ok(None)
    }

    fn path(&self) -> String {
        current_dir()
            .expect("can't get current directory")
            .to_string_lossy()
            .to_string()
    }
}

#[inline]
fn is_dir_empty(d: &PathBuf) -> bool {
    match d.read_dir() {
        Err(_e) => true,
        Ok(mut s) => s.next().is_none(),
    }
}

#[inline]
fn get_path_type(d: &PathBuf) -> &str {
    if d.is_dir() {
        "d"
    } else if d.is_file() {
        "f"
    } else {
        " "
    }
}
