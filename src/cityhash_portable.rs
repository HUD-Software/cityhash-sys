use crate::u128_low_high::LowHigh;
use core::mem::MaybeUninit;
use core::ffi::{c_char, c_size_t};

extern "C" {
    fn CityHash32(buf: *const c_char, len: c_size_t) -> u32;
    fn CityHash64(buf: *const c_char, len: c_size_t) -> u64;
    fn CityHash64WithSeed(buf: *const c_char, len: c_size_t, seed: u64) -> u64;
    fn CityHash64WithSeeds(buf: *const c_char, len: c_size_t, seed0: u64, seed1: u64) -> u64;

    fn CityHash128(
        buf: *const c_char,
        len: c_size_t,
        hash_low_128_half: *mut u64,
        hash_high_128_half: *mut u64,
    );
    fn CityHash128WithSeed(
        buf: *const c_char,
        len: c_size_t,
        seed_low_128_half: u64,
        seed_high_128_half: u64,
        hash_low_128_half: *mut u64,
        hash_high_128_half: *mut u64,
    );
    fn Hash128to64(low_128_half: u64, high_128_half: u64) -> u64;
}

/// Retrieves a 32-bit hash of a slice of bytes.
///
///
/// # Example
///
/// ```
/// use cityhash_sys::city_hash_32;
///
/// assert_eq!(city_hash_32(&[0u8,1,2,3,4]), 0xFE6E37D4);
///
/// ```
#[inline]
#[must_use]
pub fn city_hash_32(bytes: &[u8]) -> u32 {
    unsafe { CityHash32(bytes.as_ptr() as *const i8, bytes.len()) }
}

/// Retrieves a 64-bit hash of a slice of bytes.
///
/// # Example
///
/// ```
/// use cityhash_sys::city_hash_64;
///
/// assert_eq!(city_hash_64(&[0u8,1,2,3,4]), 0xB4BFA9E87732C149);
///
/// ```
#[inline]
#[must_use]
pub fn city_hash_64(bytes: &[u8]) -> u64 {
    unsafe { CityHash64(bytes.as_ptr() as *const i8, bytes.len()) }
}

/// Retrieves a 64-bit hash of a slice of bytes, a seed is also hashed into the result.
///
/// # Example
///
/// ```
/// use cityhash_sys::city_hash_64_with_seed;
///
/// assert_eq!(city_hash_64_with_seed(&[0u8,1,2,3,4], 123), 0xCE1706019C5E61A7);
///
/// ```
#[inline]
#[must_use]
pub fn city_hash_64_with_seed(bytes: &[u8], seed: u64) -> u64 {
    unsafe { CityHash64WithSeed(bytes.as_ptr() as *const i8, bytes.len(), seed) }
}

/// Retrieves a 64-bit hash of a slice of bytes, two seeds is also hashed into the result.
///
/// # Example
///
/// ```
/// use cityhash_sys::city_hash_64_with_seeds;
///
/// assert_eq!(city_hash_64_with_seeds(&[0u8,1,2,3,4], 123, 456), 0xD83C81881E3A35E3);
///
/// ```
#[inline]
#[must_use]
pub fn city_hash_64_with_seeds(bytes: &[u8], seed_0: u64, seed_1: u64) -> u64 {
    unsafe { CityHash64WithSeeds(bytes.as_ptr() as *const i8, bytes.len(), seed_0, seed_1) }
}

/// Retrieves a 128-bit hash of a slice of bytes.
///
/// # Example
///
/// ```
/// use cityhash_sys::city_hash_128;
///
/// assert_eq!(city_hash_128(&[0u8,1,2,3,4]), 0xE3CB1F3F3AB9643BEF3668C150012EEC);
///
/// ```
#[inline]
#[must_use]
pub fn city_hash_128(bytes: &[u8]) -> u128 {
    unsafe {
        let mut low_128_half = MaybeUninit::<u64>::uninit();
        let mut high_128_half = MaybeUninit::<u64>::uninit();
        CityHash128(
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
/// use cityhash_sys::city_hash_128_with_seed;
///
/// assert_eq!(city_hash_128_with_seed(&[0u8,1,2,3,4], 123), 0x68DA6334DE1F04C9CE255B9613AD58B7);
///
/// ```
#[inline]
#[must_use]
pub fn city_hash_128_with_seed(bytes: &[u8], seed: u128) -> u128 {
    unsafe {
        let mut low_128_half = MaybeUninit::<u64>::uninit();
        let mut high_128_half = MaybeUninit::<u64>::uninit();
        CityHash128WithSeed(
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

/// Retrieves the 64 bits of a 128 bits input.
///
/// # Example
///
/// ```
/// use cityhash_sys::city_hash_128_to_64;
///
/// assert_eq!(city_hash_128_to_64(0x68DA6334DE1F04C9CE255B9613AD58B7), 0xA991280057F9AACA);
///
/// ```
#[inline]
#[must_use]
pub fn city_hash_128_to_64(input: u128) -> u64 {
    unsafe { Hash128to64(input.low_half(), input.high_half()) }
}