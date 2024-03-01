//! xlog-rs
//!
//! A simple logging library for Rust.
//!
//! # Examples
//!
//! ```
//! use xlog::warn;
//!
//! xlog::log::init(std::io::stdout(), xlog::log::Level::Trace);
//!
//! warn!("warn message");
//! ```
/// main module
pub mod log;
