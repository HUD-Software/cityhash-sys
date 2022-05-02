/// `Composer` extract the lower half and higher half bits of a integral
pub trait Composer {
    /// Type that can hold half bits of the type that implement the trait
    type Half;

    /// Extract the lower half bits of an integral type
    ///
    /// # Example
    /// ```ignore
    /// assert_eq!(0xAAAA_AAAA_AAAA_AAAA_BBBB_BBBB_BBBB_BBBB.low(), 0xBBBB_BBBB_BBBB_BBBB);
    /// ```
    fn low_half(self) -> Self::Half;

    /// Extract the higher half bits of an integral type
    ///
    /// # Example
    /// ```ignore
    /// assert_eq!(0xAAAA_AAAA_AAAA_AAAA_BBBB_BBBB_BBBB_BBBB.low(), 0xAAAA_AAAA_AAAA_AAAA);
    /// ```
    fn high_half(self) -> Self::Half;

    /// Create an integral from higher and lower half bits
    ///
    /// # Example
    /// ```ignore
    /// assert_eq!(
    ///             <u128 as Composer>::from(0xAAAA_AAAA_AAAA_AAAA, 0xBBBB_BBBB_BBBB_BBBB),
    ///             0xAAAA_AAAA_AAAA_AAAA_BBBB_BBBB_BBBB_BBBB
    /// );
    /// ```
    fn from_halfs(high: Self::Half, low: Self::Half) -> Self;
}

/// `Composer` trait implementation for u128
impl Composer for u128 {
    /// u128 is splittable into 2*u64
    type Half = u64;

    /// Extract the lower half bits of a u128
    ///
    /// See [`Composer::low_half()`] for more information
    #[cfg(target_endian = "little")]
    fn low_half(self) -> Self::Half {
        self as Self::Half
    }

    /// Extract the lower half bits of a u128
    ///
    /// See [`Composer::low()`] for more information
    #[cfg(target_endian = "big")]
    fn low_half(self) -> Self::Half {
        (self >> u64::BITS) as Self::Half
    }

    /// Extract the higher half bits of a u128
    ///
    /// See [`Composer::high()`] for more information
    #[cfg(target_endian = "little")]
    fn high_half(self) -> Self::Half {
        (self >> u64::BITS) as Self::Half
    }

    /// Extract the higher half bits of a u128
    ///
    /// See [`Composer::high()`] for more information
    #[cfg(target_endian = "big")]
    fn high_half(self) -> Self::Half {
        self as Self::Half
    }

    /// Create a u128 from higher and lower half bits
    ///
    /// See [`Composer::from()`] for more information
    #[cfg(target_endian = "little")]
    fn from_halfs(high: Self::Half, low: Self::Half) -> Self {
        let mut res = high as Self;
        res <<= u64::BITS;
        res |= low as Self;
        res
    }

    /// Create a u128 from higher and lower half bits
    ///
    /// See [`Composer::from()`] for more information
    #[cfg(target_endian = "big")]
    fn from_halfs(high: Self::Half, low: Self::Half) -> Self {
        let mut res = low as Self;
        res <<= u64::BITS;
        res |= high as Self;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Composer;

    #[test]
    fn low() {
        assert_eq!(
            0xAAAA_AAAA_AAAA_AAAA_BBBB_BBBB_BBBB_BBBB.low_half(),
            0xBBBB_BBBB_BBBB_BBBB
        );
    }

    #[test]
    fn high() {
        assert_eq!(
            0xAAAA_AAAA_AAAA_AAAA_BBBB_BBBB_BBBB_BBBB.high_half(),
            0xAAAA_AAAA_AAAA_AAAA
        );
    }

    #[test]
    fn from() {
        assert_eq!(
            <u128 as Composer>::from_halfs(0xAAAA_AAAA_AAAA_AAAA, 0xBBBB_BBBB_BBBB_BBBB),
            0xAAAA_AAAA_AAAA_AAAA_BBBB_BBBB_BBBB_BBBB
        );
    }
}
