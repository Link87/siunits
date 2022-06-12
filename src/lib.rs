#![allow(incomplete_features)]
#![deny(clippy::all)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![no_std]

pub mod dim;
pub mod ops;
pub mod si;
pub mod unit;

#[cfg(test)]
mod tests;
