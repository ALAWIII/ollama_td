//! ollama_td : ```ollama tool download``` crate , which is a crate used exclusively to download the ollama command line tool or the binary itself ,
//!
//! this is Not a crate to download the models, but to download the tool that is used to manage and download the models !!!
//!
//! you can automate the download of the compressed (e.g. zip or tgz) CLI tool and
//!
//!  automatically unpack and place it where ever you want .
//!
//! # Breaking Changes
//!
//! Ollama tool **```v5.8```** and beyond changed ***```ollama-darwin```*** to ***```ollama-darwin.tgz```***.
//!
//! ```download(o_download,f_stream)``` now accepts optional function to give you the ability to customize the process of downloading the tool.
//!
//! # Examples
//! Different platforms have different several options available .
//! ## Windows
//! you have three options :
//! - ollama-windows-amd64.zip
//! - ollama-windows-arm64.zip
//! - OllamaSetup.exe
//!
//!```no_run
#![doc = include_str!("../examples/windows.rs")]
//!```
//! ## Unix
//! you have two options :
//! - ollama-darwin.tgz
//! - Ollama-darwin.zip
//!
//! ```no_run
#![doc = include_str!("../examples/unix.rs")]
//!```
//! ## Linux
//! you have five options :
//! - ollama-linux-amd64.tgz
//! - ollama-linux-amd64-rocm.tgz
//! - ollama-linux-arm64.tgz
//! - ollama-linux-arm64-jetpack5.tgz
//! - ollama-linux-arm64-jetpack6.tgz
//! ```no_run
#![doc = include_str!("../examples/linux.rs")]
//! ```
mod download;
mod platforms;
mod util;
pub use download::*;
pub use platforms::*;
pub use util::*;
