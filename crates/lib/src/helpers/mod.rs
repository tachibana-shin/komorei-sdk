//! Utilities for commonly used functions when creating Komorei sources.
extern crate alloc;

#[cfg(feature = "imports")]
pub mod cfemail;
#[cfg(feature = "imports")]
pub mod element;

pub mod string;
pub mod uri;
