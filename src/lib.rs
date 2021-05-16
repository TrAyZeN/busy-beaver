//! Crate for generating busy beaver candidates

#![warn(
    missing_docs,
    rust_2018_idioms,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::inline_always)]

pub mod turing_machine;

/// Tape for binary alphabet Turing machine
pub mod tape;

pub mod transition;

/// Some utils functions
pub mod utils;
