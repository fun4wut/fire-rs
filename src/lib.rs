//! # Fire-rs
//!
//! Use the attribute macro to convert a ordinary function into a command line application (aka `cli`)
//!
//! ## Supported feature
//!
//! - normal arguments
//! - named arguments
//!
//! ## Usage
//! ```rust
//! use fire_rs::fire;
//! #[fire]
//! fn foo(a: i32, b: f64, c: String) {
//!     println!("{} is {}", a as f64 + b, c);
//! }
//! // cancel the comment when in use!
//! //fn main() {
//! //    foo_fire();
//! //}
//! ```
//! Run `cargo run -- 1 2.1 cool` or `cargo run -- a 1 --b 2.1 --c cool`,
//!
//! the program will output `3.1 is cool`
//!

pub use fire_rs_core::fire;

/// Clap Re-Export.
///
pub use clap::{App, Arg};
