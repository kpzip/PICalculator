#![cfg_attr(not(test), no_std)]
#![feature(core_intrinsics)]

extern crate alloc;
#[cfg(test)]
extern crate std;

mod lazylock;
pub mod parser;
