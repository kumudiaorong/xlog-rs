//! xlog-rs
//!
//! A simple logging library for Rust.
//!
//! # Examples
//!
//! ```
//! use xlog_rs::warn;
//!
//! xlog_rs::log::init(std::io::stdout(), xlog_rs::log::Level::Trace);
//!
//! warn!("warn message");
//! ```
/// main module
pub mod log;
