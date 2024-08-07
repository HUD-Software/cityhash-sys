use crate::{city_hash_128_to_64, city_hash_128, city_hash_128_with_seed, city_hash_32, city_hash_64, city_hash_64_with_seed};
use core::hash::{BuildHasher, BuildHasherDefault, Hasher};

/// 64-bits CityHash hasher
#[derive(Debug, Default)]
pub struct CityHash64Hasher {
    key: Option<u64>,
}

impl CityHash64Hasher {
    /// Create a new [CityHash64Hasher] initiated with a hash key
    pub fn with_seed(seed: u64) -> CityHash64Hasher {
        CityHash64Hasher { key: Some(seed) }
    }
}

impl Hasher for CityHash64Hasher {
    /// Returns the hash value for the values written so far.
    ///
    /// # Example
    ///
    /// ```
    /// use cityhash_sys::CityHash64Hasher;
    /// use core::hash::Hasher;
    ///
    /// let hasher = CityHash64Hasher::with_seed(0xB4BFA9E87732C149);
    /// assert_eq!(hasher.finish(), 0xB4BFA9E87732C149);
    /// ```
    fn finish(&self) -> u64 {
        self.key.unwrap_or(0)
    }

    /// Writes some data into the [CityHash64Hasher].
    ///
    /// # Example
    ///
    /// ```
    /// use cityhash_sys::CityHash64Hasher;
    /// use core::hash::Hasher;
    ///
    /// let mut hasher = CityHash64Hasher::default();
    /// hasher.write(b"hash me!");
    /// ```
    fn write(&mut self, bytes: &[u8]) {
        self.key = Some(match self.key {
            None => city_hash_64(bytes),
            Some(seed) => city_hash_64_with_seed(bytes, seed),
        })
    }
}

impl BuildHasher for CityHash64Hasher {
    type Hasher = CityHash64Hasher;

    /// Creates a new [CityHash64Hasher].
    fn build_hasher(&self) -> Self::Hasher {
        CityHash64Hasher::default()
    }
}

/// 32-bits CityHash hasher
#[derive(Debug, Default)]
pub struct CityHash32Hasher {
    key: Option<u32>,
}

impl CityHash32Hasher {
    /// Create a new [CityHash32Hasher] initiated with a hash key
    pub fn with_seed(seed: u32) -> CityHash32Hasher {
        CityHash32Hasher { key: Some(seed) }
    }
}

impl Hasher for CityHash32Hasher {
    /// Returns the hash value for the values written so far.
    ///
    /// # Example
    ///
    /// ```
    /// use cityhash_sys::CityHash32Hasher;
    /// use core::hash::Hasher;
    ///
    /// let hasher = CityHash32Hasher::with_seed(0xB4BFA9E);
    /// assert_eq!(hasher.finish(), 0xB4BFA9E);
    /// ```
    fn finish(&self) -> u64 {
        if let Some(hash) = self.key {
            hash as u64
        } else {
            0
        }
    }

    /// Writes some data into the [CityHash32Hasher].
    ///
    /// # Example
    ///
    /// ```
    /// use cityhash_sys::CityHash32Hasher;
    /// use core::hash::Hasher;
    ///
    /// let mut hasher = CityHash32Hasher::default();
    /// hasher.write(b"hash me!");
    /// ```
    fn write(&mut self, bytes: &[u8]) {
        self.key = Some(match self.key {
            None => city_hash_32(bytes),
            Some(mut seed) => {
                let mut key = city_hash_32(bytes);

                // Mix of 2 u32 based on Murmur3.
                // Magic numbers for 32-bit hashing. 
                const C1: u32 = 0xcc9e2d51;
                const C2: u32 = 0x1b873593;

                // Combine 2 32-bits values from Murmur3
                key = key.wrapping_mul(C1);
                key = key.rotate_right(17);
                key = key.wrapping_mul(C2);
                seed ^= key;
                seed = seed.rotate_right(19);
                seed.wrapping_mul(5).wrapping_add(0xe6546b64)
            }
        })
    }
}

impl BuildHasher for CityHash32Hasher {
    type Hasher = CityHash32Hasher;

    /// Creates a new [CityHash32Hasher].
    fn build_hasher(&self) -> Self::Hasher {
        CityHash32Hasher::default()
    }
}

/// 128-bits CityHash hasher
#[derive(Debug, Default)]
pub struct CityHash128Hasher {
    key: Option<u128>,
}

impl CityHash128Hasher {
    /// Create a new [CityHash128Hasher] initiated with a hash key
    pub fn with_seed(seed: u128) -> CityHash128Hasher {
        CityHash128Hasher { key: Some(seed) }
    }
}

impl Hasher for CityHash128Hasher {
    /// Returns the hash value for the values written so far.
    /// The hash is compress to a 64-bits with [city_hash_128_to_64]
    ///
    /// # Example
    ///
    /// ```
    /// use cityhash_sys::CityHash128Hasher;
    /// use core::hash::Hasher;
    ///
    /// let hasher = CityHash128Hasher::with_seed(0xAAAAAAAAAAAAAAAABBBBBBBBBBBBBBBB);
    /// assert_eq!(hasher.finish(), 0x88FC029FFEBB98B4);
    /// ```
    fn finish(&self) -> u64 {
        if let Some(hash) = self.key {
            city_hash_128_to_64(hash)
        } else {
            0
        }
    }

    /// Writes some data into the [CityHash128Hasher].
    ///
    /// # Example
    ///
    /// ```
    /// use cityhash_sys::CityHash128Hasher;
    /// use core::hash::Hasher;
    ///
    /// let mut hasher = CityHash128Hasher::default();
    /// hasher.write(b"hash me!");
    /// ```
    fn write(&mut self, bytes: &[u8]) {
        self.key = Some(match self.key {
            None => city_hash_128(bytes),
            Some(seed) => city_hash_128_with_seed(bytes, seed),
        })
    }
}

impl BuildHasher for CityHash128Hasher {
    type Hasher = CityHash128Hasher;

    /// Creates a new [CityHash128Hasher].
    fn build_hasher(&self) -> Self::Hasher {
        CityHash128Hasher::default()
    }
}

/// A builder for default 32-bits CityHash hashers.
pub type CityHash32BuildHasher = BuildHasherDefault<CityHash32Hasher>;

/// A builder for default 64-bits CityHash hashers.
pub type CityHash64BuildHasher = BuildHasherDefault<CityHash64Hasher>;

/// A builder for default 128-bits CityHash hashers.
pub type CityHash128BuildHasher = BuildHasherDefault<CityHash128Hasher>;

/// A builder for default CityHash hashers.
pub type CityHashBuildHasher = CityHash64BuildHasher;
