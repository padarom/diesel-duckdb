//! A third-party DuckDB adapter for Diesel
//!
//! This crate implements a DuckDB backend and connection for Diesel.

pub mod duckdb;

#[doc(inline)]
pub use crate::duckdb::*;

#[cfg(test)]
mod test;