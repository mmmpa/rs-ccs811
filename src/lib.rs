#![allow(warnings)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate nix;

mod ccs_811;
mod client;

pub use crate::ccs_811::*;
pub use client::*;

pub type Css811Result<T> = Result<T, Css811Error>;
