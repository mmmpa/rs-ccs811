#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate log;

mod ccs811;
mod client;
mod helpers;

pub use ccs811::*;
pub use client::*;
pub use helpers::*;

pub type Ccs811Result<T> = Result<T, Ccs811Error>;
