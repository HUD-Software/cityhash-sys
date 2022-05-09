#![cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]

use crate::int128::Composer;
use core::mem::MaybeUninit;

extern "C" {
    fn CityHashCrc128(
        buf: *const i8,
        len: usize,
        hash_low_128_half: *mut u64,
        hash_high_128_half: *mut u64,
    );

    fn CityHashCrc128WithSeed(
        buf: *const i8,
        len: usize,
        seed_low_128_half: u64,
        seed_high_128_half: u64,
        hash_low_128_half: *mut u64,
        hash_high_128_half: *mut u64,
    );

    fn CityHashCrc256(buf: *const i8, len: usize, hash: *mut u64);
}

/// Retrieves a 128-bit hash of a slice of bytes.
///
/// # Example
///
/// ```
/// use cityhash_sys::city_hash_crc_128;
///
/// assert_eq!(city_hash_crc_128(&[0u8,1,2,3,4]), 0xE3CB1F3F3AB9643BEF3668C150012EEC);
///
/// ```
#[inline]
#[must_use]
#[target_feature(enable = "sse4.2")]
pub fn city_hash_crc_128(bytes: &[u8]) -> u128 {
    unsafe {
        let mut low_128_half = MaybeUninit::<u64>::uninit();
        let mut high_128_half = MaybeUninit::<u64>::uninit();
        CityHashCrc128(
            bytes.as_ptr() as *const i8,
            bytes.len(),
            low_128_half.as_mut_ptr(),
            high_128_half.as_mut_ptr(),
        );
        u128::from_halfs(high_128_half.assume_init(), low_128_half.assume_init())
    }
}

/// Retrieves a 128-bit hash of a slice of bytes, a seed is also hashed into the result.
///
/// # Example
///
/// ```
/// use cityhash_sys::city_hash_crc_128_with_seed;
///
/// assert_eq!(city_hash_crc_128_with_seed(&[0u8,1,2,3,4], 123), 0x68DA6334DE1F04C9CE255B9613AD58B7);
///
/// ```
#[inline]
#[must_use]
#[target_feature(enable = "sse4.2")]
pub fn city_hash_crc_128_with_seed(bytes: &[u8], seed: u128) -> u128 {
    unsafe {
        let mut low_128_half = MaybeUninit::<u64>::uninit();
        let mut high_128_half = MaybeUninit::<u64>::uninit();
        CityHashCrc128WithSeed(
            bytes.as_ptr() as *const i8,
            bytes.len(),
            seed.low_half(),
            seed.high_half(),
            low_128_half.as_mut_ptr(),
            high_128_half.as_mut_ptr(),
        );
        u128::from_halfs(high_128_half.assume_init(), low_128_half.assume_init())
    }
}

/// Retrieves a 256-bit hash fo a slice of bytes.
/// The hash is a slice of u64 where [0..4] is [low..high] bits.
///
/// # Example
///
/// ```
/// use cityhash_sys::city_hash_crc_256;
///
/// assert_eq!(city_hash_crc_256(&[0u8,1,2,3,4]), [0xA7FAC4B64C35C8B4,0xDD83C2CDF35398F6,0xEAF64F6BA6A2C9E8,0x4E72CE1685CE9077]);
///
/// ```
#[inline]
#[must_use]
#[target_feature(enable = "sse4.2")]
pub fn city_hash_crc_256(bytes: &[u8]) -> [u64; 4] {
    unsafe {
        let mut hash = MaybeUninit::<[u64; 4]>::uninit();
        CityHashCrc256(
            bytes.as_ptr() as *const i8,
            bytes.len(),
            (*hash.as_mut_ptr()).as_mut_ptr(),
        );
        hash.assume_init()
    }
}