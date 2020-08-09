#![cfg_attr(not(feature = "std"), no_std)]

#[allow(unused_imports)]
#[macro_use]
extern crate log;

mod ccs811;
mod client;

pub use crate::ccs811::*;
pub use client::*;

pub type Ccs811Result<T> = Result<T, Ccs811Error>;
