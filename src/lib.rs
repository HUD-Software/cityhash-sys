#![doc=include_str!("../README.md")]
//! CityHash provides hash functions for strings. CityHash mix the input bits thoroughly but are not suitable for cryptography.
//! It is a Rust binding of [Google CityHash](https://github.com/google/cityhash) library
//!
//! CityHash-sys do not load the standard library (a.k.a `#![no_std]`)
//!
//! It provides 32-bits, 64-bits, 128-bits and 256-bits hash.
//! 256-bits hash is available only for `x86_64` target_arch with `sse4.2`. See the README for more informations
//!
//! # Example
//!
//! ```
//! use cityhash_sys::{city_hash_128, city_hash_32, city_hash_64};
//!
//! // Provides free functions
//! assert_eq!(city_hash_32(&[0,1,2,3,4]), 0xFE6E37D4u32);
//! assert_eq!(city_hash_64(&[0,1,2,3,4]), 0xB4BFA9E87732C149u64);
//! assert_eq!(city_hash_128(&[0,1,2,3,4]), 0xE3CB1F3F3AB9643BEF3668C150012EECu128);
//!
//! // Provides trait implementation for [u8] and str
//! use cityhash_sys::CityHash;
//!
//! assert_eq!([5u8, 4, 3, 2, 1].city_hash_64(), 0x34EC5F7922A51496);
//! assert_eq!("hash me!".city_hash_64(), 0xF04A0CC67B63A0B4);
//!
//! // Some 128-bits and the 256-bits hash needs x86_64 architecture with sse 4.2
//! // Free functions that need it start with city_hash_crc_...
//! // Trait that provides the implementation is named with CityHashCrc
//! #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
//! use cityhash_sys::CityHashCrc;
//!
//! #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
//! assert_eq!([0u8,1,2,3,4].city_hash_crc_128(), 0xE3CB1F3F3AB9643BEF3668C150012EECu128);
//!
//! #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
//! assert_eq!([0u8,1,2,3,4].city_hash_crc_256(), [0xA7FAC4B64C35C8B4,0xDD83C2CDF35398F6,0xEAF64F6BA6A2C9E8,0x4E72CE1685CE9077]);
//!
//! ```
#![no_std]
#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
mod cityhash_crc;
mod cityhash_portable;
mod hasher;
mod int128;

#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
pub use crate::cityhash_crc::*;
pub use crate::cityhash_portable::*;
pub use hasher::*;
