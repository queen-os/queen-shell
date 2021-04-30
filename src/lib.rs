#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[macro_use]
extern crate alloc;
#[cfg(any(test, feature = "std"))]
extern crate core;

pub mod error;
pub mod parser;
pub mod signature;
pub mod evaluate;
pub mod commands;
pub mod context;
pub mod shell;
pub mod deserializer;

pub use error::ShellError;
