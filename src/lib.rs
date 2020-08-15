#![cfg_attr(not(feature = "std"), no_std)]

mod ccs811;
mod client;

pub use crate::ccs811::*;
pub use client::*;

pub type Ccs811Result<T> = Result<T, Ccs811Error>;
