#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[macro_use]
extern crate alloc;
#[cfg(any(test, feature = "std"))]
extern crate core;

pub mod cli;
pub mod commands;
pub mod context;
pub mod deserializer;
pub mod error;
pub mod evaluate;
pub mod parser;
pub mod shell;
pub mod signature;

pub use cli::cli;
pub use error::ShellError;
pub use shell::Shell;
