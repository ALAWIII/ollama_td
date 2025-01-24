//! ollama_td : ```ollama tool download``` crate , which is a crate used exclusively to download the ollama command line tool or the binary itself ,
//!
//! this is Not a crate to download the models, but to download the tool that is used to manage and download the models !!!
//!
//! # Examples
//! Different platforms have different several options available .
//! ## Windows
//!
//!```no_run
#![doc = include_str!("../examples/windows.rs")]
//!```
//! ## Unix
//! ```no_run
#![doc = include_str!("../examples/unix.rs")]
//!```
//! ## Linux
//! ```no_run
#![doc = include_str!("../examples/linux.rs")]
//! ```
mod download;
mod platforms;
mod util;
pub use download::*;
pub use platforms::*;
pub use util::*;
