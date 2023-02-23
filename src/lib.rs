#![doc=include_str!("../README.md")]
#![no_std]
#![feature(c_size_t)]
#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
mod cityhash_crc;
mod cityhash_portable;
mod hasher;
mod u128_low_high;

#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
pub use crate::cityhash_crc::*;
pub use crate::cityhash_portable::*;
pub use hasher::*;
