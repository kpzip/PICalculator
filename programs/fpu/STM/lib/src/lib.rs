#![cfg_attr(not(test), no_std)]

extern crate alloc;
#[cfg(test)]
extern crate std;

mod lazylock;
pub mod parser;
