
# CityHash-sys [![Crates.io](https://img.shields.io/crates/v/cityhash-sys?style=plastic)](https://crates.io/crates/cityhash-sys) [![Crates.io](https://img.shields.io/crates/l/cityhash-sys?style=plastic)](https://choosealicense.com/licenses/mit/)

Rust bindings to [Google CityHash](https://github.com/google/cityhash)'s C++ API.
CityHash-sys do not load the standard library (a.k.a `no_std`).

[![Build](https://github.com/HUD-Software/cityhash-sys/actions/workflows/CICD.yml/badge.svg)](https://github.com/HUD-Software/cityhash-sys/actions/workflows/CICD.yml)
[![codecov](https://codecov.io/gh/HUD-Software/cityhash-sys/branch/master/graph/badge.svg?token=LTEI8LUT5R)](https://codecov.io/gh/HUD-Software/cityhash-sys) [![docs.rs](https://img.shields.io/docsrs/cityhash-sys?style=plastic)](https://docs.rs/cityhash-sys/latest/cityhash_sys/)

## Introduction

CityHash provides hash functions for strings. Functions mix the input bits thoroughly but are not suitable for cryptography.
CityHash-sys is tested on little-endian but should work on big-endian architecture.

### Usage

**Using Hasher**
```rust
use core::hash::{BuildHasher, Hasher};
use cityhash_sys::CityHashBuildHasher;

let build_hasher = CityHashBuildHasher::default();
let mut hasher = build_hasher.build_hasher();
hasher.write(b"hash me!");
assert_eq!(hasher.finish(), 0xF04A0CC67B63A0B4);
```
**_Note_** *`CityHashBuildHasher` is an alias to the the 64-bits CityHash `CityHash64Hasher`. `CityHash32Hasher` and `CityHash128Hasher` are also available but result are still `u64`. See documentation for more details.*

**Using Portable CityHash functions**

Rust bindings provides a safe interface to all Google's CityHash hash functions that do not make use of x86_64 CRC intrinsic:

***32-bit hash***
```rust ignore
// uint32 CityHash32(const char *, size_t);
fn city_hash_32(buf: &[u8]) -> u32;
```

***64-bit hash***

```rust ignore
// uint64 CityHash64(const char *, size_t);
fn city_hash_64(buf: &[u8]) -> u64;

// uint64 CityHash64WithSeed(const char *, size_t, uint64);
fn city_hash_64_with_seed(buf: &[u8], seed: u64) -> u64; 

// uint64 CityHash64WithSeeds(const char *, size_t, uint64, uint64);
fn city_hash_64_with_seeds(buf: &[u8], seed_0: u64, seed_1: u64) -> u64;
```

***128-bit hash***

```rust ignore
// uint128 CityHash128(const char *, size_t);
fn city_hash_128(buf: &[u8]) -> u128;

// uint128 CityHash128WithSeed(const char *, size_t, uint128);
fn city_hash_128_with_seed(buf: &[u8], seed: u128) -> u128;

// uint64 Hash128to64(const uint128&);
fn city_hash_128_to_64(hash: u128) -> u64;
```

**_Note:_** *Depending on your compiler and hardware, it's likely faster than CityHash64() on sufficiently long strings.  It's slower than necessary on shorter strings.*

**Using CityHash functions with CRC-32 intrinsic**

Some functions are available only if the target is `x86_64` and support at least `sse4.2` target feature because of the usage of CRC-32 intrinsic `_mm_crc32_u64` . If we want to enable those functions use `-C target-feature=+sse4.2` or above (`avx` or `avx2`).
Note that depending of the length of the buffer you want to hash, it can be faster to use the non-intrinsic version.
If the buffer to hash is less than 900 bytes, `CityHashCrc128WithSeed` and `CityHashCrc128` will respectivelly internally call `CityHash128WithSeed` and `CityHash128`, in this case, it is better to call directly `CityHash128WithSeed` or `CityHash128`.

***128-bit hash with CRC-32 intrinsic***

```rust ignore
// uint128 CityHashCrc128(const char *, size_t);
fn city_hash_crc_128(buf: &[u8]) -> u128;

// uint128 CityHashCrc128WithSeed(const char *, size_t, uint128);
fn city_hash_crc_128_with_seed(buf: &[u8], seed: u128) -> u128;
```

***256-bit hash with CRC-32 intrinsic***

```rust ignore
// void CityHashCrc256(const char *, size_t, uint64 *);
fn city_hash_crc_256(buf: &[u8]) -> [u64; 4]; //
```

**Using Rust convenient traits**

CityHash-sys provides convenient traits to hash.

`CityHash` trait provides hash functions that do not used the CRC-32 intrinsics.

```rust
use cityhash_sys::CityHash;

// Hash the slice with CityHash64
let hash_slice: u64 = [5u8, 4, 3, 2, 1].city_hash_64();
assert_eq!(hash_slice, 0x34EC5F7922A51496);

// Hash the str with CityHash64
let hash_str: u64 = "hash me!".city_hash_64();
assert_eq!(hash_str, 0xF04A0CC67B63A0B4);
```

`CityHashCrc` trait provides hash implementation for `[u8]` and `str` types with `x86_64` CRC-32 intrinsic. (Only available with `target-feature=+sse4.2`)

```rust
use cityhash_sys::CityHashCrc;

// Hash the slice with CityHashCrc128
let hash_crc_slice: u128 = [5u8, 4, 3, 2, 1].city_hash_crc_128();

// Hash the str with CityHashCrc128
let hash_crc_slice: u128 = "hash me!".city_hash_crc_128();
```

## Performance

On 64-bits hardware, CityHash is suitable for short string hashing, e.g., most hash table keys, especially `city_hash_64` that is faster than `city_hash_128`.
On 32-bits hardware, CityHash is the nearest competitor of Murmur3 on x86.

## For more information

See the [Google Cityhash README](./src/google/README/)