#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[macro_use]
extern crate alloc;
extern crate core;

pub mod error;
pub mod parser;
pub mod signature;

pub use error::ShellError;
