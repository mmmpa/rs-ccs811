#[macro_use]
extern crate nix;

mod ccs811;
mod client;

pub use crate::ccs811::*;
pub use client::*;

pub type Css811Result<T> = Result<T, Css811Error>;
