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
mod int128;
use core::mem::MaybeUninit;
use int128::Composer;

extern "C" {
    fn CityHash32(buf: *const i8, len: usize) -> u32;
    fn CityHash64(buf: *const i8, len: usize) -> u64;
    fn CityHash64WithSeed(buf: *const i8, len: usize, seed: u64) -> u64;
    fn CityHash64WithSeeds(buf: *const i8, len: usize, seed0: u64, seed1: u64) -> u64;

    fn CityHash128(
        buf: *const i8,
        len: usize,
        hash_low_128_half: *mut u64,
        hash_high_128_half: *mut u64,
    );
    fn CityHash128WithSeed(
        buf: *const i8,
        len: usize,
        seed_low_128_half: u64,
        seed_high_128_half: u64,
        hash_low_128_half: *mut u64,
        hash_high_128_half: *mut u64,
    );
    fn Hash128to64(low_128_half: u64, high_128_half: u64) -> u64;

    #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
    fn CityHashCrc128(
        buf: *const i8,
        len: usize,
        hash_low_128_half: *mut u64,
        hash_high_128_half: *mut u64,
    );

    #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
    fn CityHashCrc128WithSeed(
        buf: *const i8,
        len: usize,
        seed_low_128_half: u64,
        seed_high_128_half: u64,
        hash_low_128_half: *mut u64,
        hash_high_128_half: *mut u64,
    );

    #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
    fn CityHashCrc256(buf: *const i8, len: usize, hash: *mut u64);
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
#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
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
#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
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
#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
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

/// `CityHash` trait provides CityHash functions to type that implement this trait.
/// This trait is a syntax sugar for `city_hash_...` functions.
pub trait CityHash {
    /// Retrieves a 32-bit hash of a value.
    ///
    /// # Example
    ///
    /// ```
    /// use cityhash_sys::CityHash;
    ///
    /// assert_eq!([0u8,1,2,3,4].city_hash_32(), 0xFE6E37D4);
    ///
    /// ```
    fn city_hash_32(&self) -> u32;

    /// Retrieves a 64-bit hash of a value.
    ///
    /// # Example
    ///
    /// ```
    /// use cityhash_sys::CityHash;
    ///
    /// assert_eq!([0u8,1,2,3,4].city_hash_64(), 0xB4BFA9E87732C149);
    ///
    /// ```
    fn city_hash_64(&self) -> u64;

    /// Retrieves a 64-bit hash of a value, a seed is also hashed into the result.
    ///
    /// # Example
    ///
    /// ```
    /// use cityhash_sys::CityHash;
    ///
    /// assert_eq!([0u8,1,2,3,4].city_hash_64_with_seed(123), 0xCE1706019C5E61A7);
    ///
    /// ```
    fn city_hash_64_with_seed(&self, seed: u64) -> u64;

    /// Retrieves a 64-bit hash of a value, two seeds is also hashed into the result.
    ///
    /// # Example
    ///
    /// ```
    /// use cityhash_sys::CityHash;
    ///
    /// assert_eq!([0u8,1,2,3,4].city_hash_64_with_seeds(123, 456), 0xD83C81881E3A35E3);
    ///
    /// ```
    fn city_hash_64_with_seeds(&self, seed_0: u64, seed_1: u64) -> u64;

    /// Retrieves a 128-bit hash of a slice of bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use cityhash_sys::CityHash;
    ///
    /// assert_eq!([0u8,1,2,3,4].city_hash_128(), 0xE3CB1F3F3AB9643BEF3668C150012EEC);
    ///
    /// ```
    fn city_hash_128(&self) -> u128;

    /// Retrieves a 128-bit hash of a value, a seed is also hashed into the result.
    ///
    /// # Example
    ///
    /// ```
    /// use cityhash_sys::CityHash;
    ///
    /// assert_eq!([0u8,1,2,3,4].city_hash_128_with_seed(123), 0x68DA6334DE1F04C9CE255B9613AD58B7);
    ///
    /// ```
    fn city_hash_128_with_seed(&self, seed: u128) -> u128;
}


impl CityHash for [i8] {
    fn city_hash_32(&self) -> u32 {
        city_hash_32(unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len()) })
    }

    fn city_hash_64(&self) -> u64 {
        city_hash_64(unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len()) })
    }

    fn city_hash_64_with_seed(&self, seed: u64) -> u64 {
        city_hash_64_with_seed(
            unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len()) },
            seed,
        )
    }

    fn city_hash_64_with_seeds(&self, seed_0: u64, seed_1: u64) -> u64 {
        city_hash_64_with_seeds(
            unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len()) },
            seed_0,
            seed_1,
        )
    }

    fn city_hash_128(&self) -> u128 {
        city_hash_128(unsafe {
            core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len())
        })
    }

    fn city_hash_128_with_seed(&self, seed: u128) -> u128 {
        city_hash_128_with_seed(
            unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len()) },
            seed,
        )
    }
}

impl CityHash for [u8] {
    fn city_hash_32(&self) -> u32 {
        city_hash_32(self)
    }

    fn city_hash_64(&self) -> u64 {
        city_hash_64(self)
    }

    fn city_hash_64_with_seed(&self, seed: u64) -> u64 {
        city_hash_64_with_seed(self, seed)
    }

    fn city_hash_64_with_seeds(&self, seed_0: u64, seed_1: u64) -> u64 {
        city_hash_64_with_seeds(self, seed_0, seed_1)
    }

    fn city_hash_128(&self) -> u128 {
        city_hash_128(self)
    }

    fn city_hash_128_with_seed(&self, seed: u128) -> u128 {
        city_hash_128_with_seed(self, seed)
    }
}

impl CityHash for str {
    fn city_hash_32(&self) -> u32 {
        city_hash_32(self.as_bytes())
    }

    fn city_hash_64(&self) -> u64 {
        city_hash_64(self.as_bytes())
    }

    fn city_hash_64_with_seed(&self, seed: u64) -> u64 {
        city_hash_64_with_seed(self.as_bytes(), seed)
    }

    fn city_hash_64_with_seeds(&self, seed_0: u64, seed_1: u64) -> u64 {
        city_hash_64_with_seeds(self.as_bytes(), seed_0, seed_1)
    }

    fn city_hash_128(&self) -> u128 {
        city_hash_128(self.as_bytes())
    }

    fn city_hash_128_with_seed(&self, seed: u128) -> u128 {
        city_hash_128_with_seed(self.as_bytes(), seed)
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
/// `CityHashCrc` trait provides CityHash functions to type that implement this trait.
/// `CityHashCrc` need the `CRC32` intrinsics  that is only available for `x86_64` target_arch with `sse4.2`.
/// This trait is a syntax sugar for `city_hash_crc_...` functions.
pub trait CityHashCrc {
    /// Retrieves a 128-bit hash of a value.
    ///
    /// # Example
    ///
    /// ```
    /// use cityhash_sys::CityHashCrc;
    ///
    /// assert_eq!([0u8,1,2,3,4].city_hash_crc_128(), 0xE3CB1F3F3AB9643BEF3668C150012EEC);
    ///
    /// ```
    fn city_hash_crc_128(&self) -> u128;

    /// Retrieves a 128-bit hash of a value, a seed is also hashed into the result.
    ///
    /// # Example
    ///
    /// ```
    /// use cityhash_sys::CityHashCrc;
    ///
    /// assert_eq!([0u8,1,2,3,4].city_hash_crc_128_with_seed(123), 0x68DA6334DE1F04C9CE255B9613AD58B7);
    ///
    /// ```
    fn city_hash_crc_128_with_seed(&self, seed: u128) -> u128;

    /// Retrieves a 256-bit hash fo a value.
    /// The hash is a slice of u64 where [0..4] is [low..high] bits.
    ///
    /// # Example
    ///
    /// ```
    /// use cityhash_sys::CityHashCrc;
    ///
    /// assert_eq!([0u8,1,2,3,4].city_hash_crc_256(), [0xA7FAC4B64C35C8B4,0xDD83C2CDF35398F6,0xEAF64F6BA6A2C9E8,0x4E72CE1685CE9077]);
    ///
    /// ```
    fn city_hash_crc_256(&self) -> [u64; 4];
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
impl CityHashCrc for [i8] {
    fn city_hash_crc_128(&self) -> u128 {
        city_hash_crc_128(unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len()) })
    }

    fn city_hash_crc_128_with_seed(&self, seed: u128) -> u128 {
        city_hash_crc_128_with_seed(unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len()) }, seed)
    }

    fn city_hash_crc_256(&self) -> [u64; 4] {
        city_hash_crc_256(unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len()) })
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
impl CityHashCrc for [u8] {
    fn city_hash_crc_128(&self) -> u128 {
        city_hash_crc_128(self)
    }

    fn city_hash_crc_128_with_seed(&self, seed: u128) -> u128 {
        city_hash_crc_128_with_seed(self, seed)
    }

    fn city_hash_crc_256(&self) -> [u64; 4] {
        city_hash_crc_256(self)
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
impl CityHashCrc for str {
    fn city_hash_crc_128(&self) -> u128 {
        self.as_bytes().city_hash_crc_128()
    }

    fn city_hash_crc_128_with_seed(&self, seed: u128) -> u128 {
        self.as_bytes().city_hash_crc_128_with_seed(seed)
    }

    fn city_hash_crc_256(&self) -> [u64; 4] {
        self.as_bytes().city_hash_crc_256()
    }
}
