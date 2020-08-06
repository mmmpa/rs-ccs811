#[macro_use]
extern crate log;

#[macro_use]
extern crate nix;

mod ccs_811;

pub use crate::ccs_811::*;

pub type Css811Result<T> = Result<T, Css811Error>;
