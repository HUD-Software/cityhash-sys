<div align="center">
  <h1>CityHash

  [![Crates.io](https://img.shields.io/crates/v/cityhash-sys?logo=Docs.rs&style=flat-square)](https://crates.io/crates/cityhash-sys) [![License](https://img.shields.io/crates/l/cityhash-sys?style=flat-square)](https://choosealicense.com/licenses/mit/)
  </h1>
</div>

Rust bindings to [Google CityHash](https://github.com/google/cityhash)'s C++ API.
CityHash-sys do not load the standard library (a.k.a `no_std`).

**_Status_**

[![Build](https://img.shields.io/github/actions/workflow/status/hud-software/cityhash-sys/Build.yml?label=Build&logo=Rust&logoColor=lightgrey&style=flat-square)](https://github.com/HUD-Software/cityhash-sys/actions/workflows/Build.yml)
[![Clippy](https://img.shields.io/github/actions/workflow/status/hud-software/cityhash-sys/Clippy.yml?label=Clippy&logo=Rust&logoColor=lightgrey&style=flat-square)](https://github.com/HUD-Software/cityhash-sys/actions/workflows/Clippy.yml)
[![docs.rs](https://img.shields.io/docsrs/cityhash-sys/latest?label=Docs&logo=Docs.rs&logoColor=lightgrey&style=flat-square)](https://docs.rs/cityhash-sys/latest/cityhash_sys/)
[![Test](https://img.shields.io/github/actions/workflow/status/hud-software/cityhash-sys/Test.yml?label=Tests&logo=Rust&logoColor=lightgrey&style=flat-square)](https://github.com/HUD-Software/cityhash-sys/actions/workflows/Test.yml)
[![codecov](https://img.shields.io/codecov/c/github/hud-software/cityhash-sys?label=Codecov&logo=Codecov&logoColor=lightgrey&style=flat-square&token=LTEI8LUT5R)](https://codecov.io/gh/HUD-Software/cityhash-sys)

**_Table of contents_**

1. [Introduction](#introduction)
2. [Usage](#usage)
    1. [Using Hasher](#using-hasher)
    2. [Using Portable CityHash functions](#using-portable-cityhash-functions)
    3. [Using CityHash functions with CRC-32 intrinsic](#using-cityhash-functions-with-crc-32-intrinsic)
3. [Performance](#performance)
4. [For more information](#for-more-information)

## Introduction

CityHash provides hash functions for strings. Functions mix the input bits thoroughly but are not suitable for cryptography.
CityHash-sys is tested on little-endian but should work on big-endian architecture.

## Usage

### Using Hasher

```rust
use cityhash_sys::CityHashBuildHasher;
use std::collections::HashMap;
const KEY: &str = "hash";
const VALUE: &str = "me!";

// Create a HashMap that use CityHash64 to hash keys
let mut map = HashMap::with_hasher(CityHashBuildHasher::default());
map.insert(KEY, VALUE);

assert_eq!(map.get(&KEY), Some(&VALUE));
```

**_Note_** _`CityHashBuildHasher` is an alias to the the 64-bits CityHash `CityHash64Hasher`. `CityHash32Hasher` and `CityHash128Hasher` are also available but result are still `u64`. See documentation for more details._

### Using portable CityHash functions

Rust bindings provides a safe interface to all Google's CityHash hash functions that do not make use of x86_64 CRC intrinsic:

**_32-bit hash_**

```rust ignore
// uint32 CityHash32(const char *, size_t);
fn city_hash_32(buf: &[u8]) -> u32;
```

**_64-bit hash_**

```rust ignore
// uint64 CityHash64(const char *, size_t);
fn city_hash_64(buf: &[u8]) -> u64;

// uint64 CityHash64WithSeed(const char *, size_t, uint64);
fn city_hash_64_with_seed(buf: &[u8], seed: u64) -> u64; 

// uint64 CityHash64WithSeeds(const char *, size_t, uint64, uint64);
fn city_hash_64_with_seeds(buf: &[u8], seed_0: u64, seed_1: u64) -> u64;
```

**_128-bit hash_**

```rust ignore
// uint128 CityHash128(const char *, size_t);
fn city_hash_128(buf: &[u8]) -> u128;

// uint128 CityHash128WithSeed(const char *, size_t, uint128);
fn city_hash_128_with_seed(buf: &[u8], seed: u128) -> u128;

// uint64 Hash128to64(const uint128&);
fn city_hash_128_to_64(hash: u128) -> u64;
```

**_Note:_** _Depending on your compiler and hardware, it's likely faster than CityHash64() on sufficiently long strings.  It's slower than necessary on shorter strings._

### Using CityHash functions with CRC-32 intrinsic

Some functions are available only if the target is `x86_64` and support at least `sse4.2` target feature because of the usage of CRC-32 intrinsic `_mm_crc32_u64` . If we want to enable those functions use `-C target-feature=+sse4.2` or above (`avx` or `avx2`).
Note that depending of the length of the buffer you want to hash, it can be faster to use the non-intrinsic version.
If the buffer to hash is less than 900 bytes, `CityHashCrc128WithSeed` and `CityHashCrc128` will respectivelly internally call `CityHash128WithSeed` and `CityHash128`, in this case, it is better to call directly `CityHash128WithSeed` or `CityHash128`.

**_128-bit hash with CRC-32 intrinsic_**

```rust ignore
// uint128 CityHashCrc128(const char *, size_t);
unsafe fn city_hash_crc_128(buf: &[u8]) -> u128;

// uint128 CityHashCrc128WithSeed(const char *, size_t, uint128);
unsafe fn city_hash_crc_128_with_seed(buf: &[u8], seed: u128) -> u128;
```

**_256-bit hash with CRC-32 intrinsic_**

```rust ignore
// void CityHashCrc256(const char *, size_t, uint64 *);
unsafe fn city_hash_crc_256(buf: &[u8]) -> [u64; 4];
```

## Performance

On 64-bits hardware, CityHash is suitable for short string hashing, e.g., most hash table keys, especially `city_hash_64` that is faster than `city_hash_128`.
On 32-bits hardware, CityHash is the nearest competitor of Murmur3 on x86.

## For more information

See the [Google Cityhash README](./src/google/README/)
