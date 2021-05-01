use crate::{
    commands::CommandRef,
    error::ShellError,
    evaluate::{call_info::CallInfo, evaluate_args, Value},
    parser::hir::Call,
    shell::Shell,
    signature::Signature,
};
use alloc::{string::{String, ToString}, sync::Arc, vec::Vec};
use core::sync::atomic::AtomicBool;
use indexmap::IndexMap;
use spin::RwLock;

#[derive(Clone, Default)]
pub struct CommandRegistry {
    registry: Arc<RwLock<IndexMap<String, CommandRef>>>,
}

impl CommandRegistry {
    #[inline]
    pub fn has(&self, name: &str) -> bool {
        self.registry.read().contains_key(name)
    }

    #[inline]
    pub fn get(&self, name: &str) -> Option<Signature> {
        self.registry.read().get(name).map(|command| command.signature())
    }

    #[inline]
    pub fn empty() -> CommandRegistry {
        CommandRegistry::default()
    }

    #[inline]
    pub fn get_command(&self, name: &str) -> Option<CommandRef> {
        self.registry.read().get(name).cloned()
    }

    #[inline]
    pub fn expect_command(&self, name: &str) -> Result<CommandRef, ShellError> {
        self.get_command(name)
            .ok_or_else(|| ShellError::runtime_error(format!("Could not load command: {}", name)))
    }

    #[inline]
    pub fn insert(&mut self, name: impl Into<String>, command: CommandRef) {
        self.registry.write().insert(name.into(), command);
    }

    #[inline]
    pub fn names(&self) -> Vec<String> {
        self.registry.read().keys().cloned().collect()
    }
}

#[derive(Clone)]
pub struct Context {
    pub registry: CommandRegistry,
    pub current_errors: Arc<RwLock<Vec<ShellError>>>,
    pub ctrl_c: Arc<AtomicBool>,
    pub shell: Arc<dyn Shell>,
}

impl Context {
    // pub fn basic() -> Self {
    //     Self {
    //         registry: CommandRegistry::empty(),
    //         current_errors: Arc::new(RwLock::new(Vec::new())),
    //         ctrl_c: Arc::new(AtomicBool::new(false)),
    //         shell: Arc::new(FilesystemShell::new()),
    //     }
    // }

    pub fn add_commands(&mut self, commands: Vec<CommandRef>) {
        for command in commands {
            self.registry.insert(command.name().to_string(), command);
        }
    }

    pub fn get_command(&self, name: &str) -> Option<CommandRef> {
        self.registry.get_command(name)
    }

    pub fn expect_command(&self, name: &str) -> Result<CommandRef, ShellError> {
        self.registry.expect_command(name)
    }

    pub fn run_command(
        &mut self,
        command: CommandRef,
        args: Call,
        source: &str,
        input: Option<Vec<Value>>,
    ) -> Result<Option<Vec<Value>>, ShellError> {
        let call_info = CallInfo {
            args: evaluate_args(args, command.clone(), &self.registry, source)?,
        };
        command.run(
            call_info,
            input,
            self.ctrl_c.clone(),
            self.shell.clone(),
            &self.registry,
        )
    }
}
