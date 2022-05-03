use crate::CityHash;
use core::hash::{BuildHasher, BuildHasherDefault, Hasher};

/// CityHash64 hasher
#[derive(Debug, Default)]
pub struct CityHash64Hasher {
    key: Option<u64>,
}

impl CityHash64Hasher {
    /// Create a new CityHash64Hasher initiated with a hash key
    pub fn new_with_seed(seed: u64) -> CityHash64Hasher {
        CityHash64Hasher { key: Some(seed) }
    }
}

impl Hasher for CityHash64Hasher {

    /// Returns the hash value for the values written so far.
    /// # Example
    ///
    /// ```
    /// use cityhash_sys::CityHash64Hasher;
    /// use core::hash::Hasher;
    ///
    /// let hasher = CityHash64Hasher::new_with_seed(0xB4BFA9E87732C149);
    /// assert_eq!(hasher.finish(), 0xB4BFA9E87732C149);
    /// ```
    fn finish(&self) -> u64 {
        if let Some(hash) = self.key {
            hash
        } else {
            0
        }
    }

    /// Writes some data into the `CityHash64Hasher`.
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
            None => bytes.city_hash_64(),
            Some(seed) => bytes.city_hash_64_with_seed(seed),
        })
    }
}

impl BuildHasher for CityHash64Hasher {
    type Hasher = CityHash64Hasher;

    /// Creates a new CityHash64Hasher.
    fn build_hasher(&self) -> Self::Hasher {
        CityHash64Hasher::default()
    }
}

/// A builder for default CityHash64 hashers.
pub type CityHash64BuildHasher = BuildHasherDefault<CityHash64Hasher>;