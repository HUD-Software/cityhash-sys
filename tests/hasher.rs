use core::hash::{BuildHasher, Hasher};

#[test]
fn hasher_32_default(){
    let hasher = cityhash_sys::CityHash32Hasher::default();
    assert_eq!(hasher.finish(), 0);
}

#[test]
fn hasher_64_default(){
    let hasher = cityhash_sys::CityHash64Hasher::default();
    assert_eq!(hasher.finish(), 0);
}

#[test]
fn hasher_128_default(){
    let hasher = cityhash_sys::CityHash128Hasher::default();
    assert_eq!(hasher.finish(), 0);
}


#[test]
fn hasher_32_with_seed(){
    let hasher = cityhash_sys::CityHash32Hasher::with_seed(0x9B9BEFFB);
    assert_eq!(hasher.finish(), 0x9B9BEFFB);
}

#[test]
fn hasher_64_with_seed(){
    let hasher = cityhash_sys::CityHash64Hasher::with_seed(0x61808A45C88841F8);
    assert_eq!(hasher.finish(), 0x61808A45C88841F8);
}

#[test]
fn hasher_128_with_seed(){
    let hasher = cityhash_sys::CityHash128Hasher::with_seed(0x9C5514CDF7881DDB8326FD07983BD576);
    assert_eq!(hasher.finish(), 0x986AFDB04708DC28);
}

#[test]
fn build_hasher_64() {
    let build_hasher = cityhash_sys::CityHash64BuildHasher::default();
    let mut hasher = build_hasher.build_hasher();
    hasher.write(b"hash me!");
    assert_eq!(hasher.finish(), 0xF04A0CC67B63A0B4);
}

#[test]
fn build_hasher_64_result_are_coherent(){
    let build_hasher = cityhash_sys::CityHash64BuildHasher::default();
    let mut hasher = build_hasher.build_hasher();

    const HASH_ME: &[u8] = b"hash me!";

    // First hash is equivalent to city_hash_64 with no seed
    let hash_free_function = cityhash_sys::city_hash_64(HASH_ME);
    hasher.write(HASH_ME);
    assert_eq!(hasher.finish(), hash_free_function);

    // Second hash is equivalent to city_hash_64_with_seed with seed that is hash key of the first hash
    let hash_free_function = cityhash_sys::city_hash_64_with_seed(&HASH_ME, hash_free_function);
    hasher.write(&HASH_ME);
    assert_eq!(hasher.finish(), hash_free_function);
}

#[test]
fn build_hasher_32() {
    let build_hasher = cityhash_sys::CityHash32BuildHasher::default();
    let mut hasher = build_hasher.build_hasher();
    hasher.write(b"hash me!");
    assert_eq!(hasher.finish(), 0x9B9BEFFB);
}

#[test]
fn build_hasher_128() {
    let build_hasher = cityhash_sys::CityHash128BuildHasher::default();
    let mut hasher = build_hasher.build_hasher();
    hasher.write(b"hash me!");
    assert_eq!(hasher.finish(), 0x61808A45C88841F8);
}

#[test]
fn build_hasher_128_result_are_coherent(){
    let build_hasher = cityhash_sys::CityHash128BuildHasher::default();
    let mut hasher = build_hasher.build_hasher();
    const HASH_ME: &[u8] = b"hash me!";

    // First hash is equivalent to city_hash_64 with no seed
    let hash_free_function = cityhash_sys::city_hash_128(HASH_ME);
    hasher.write(HASH_ME);
    assert_eq!(hasher.finish(), cityhash_sys::city_hash_128_to_64(hash_free_function));

    // Second hash is equivalent to city_hash_64_with_seed with seed that is hash key of the first hash
    let hash_free_function = cityhash_sys::city_hash_128_with_seed(&HASH_ME, hash_free_function);
    hasher.write(&HASH_ME);
    assert_eq!(hasher.finish(), cityhash_sys::city_hash_128_to_64(hash_free_function));
}

#[test]
fn hasher_is_usable_in_std_collections(){
    use cityhash_sys::CityHashBuildHasher;
    use std::collections::HashMap;
    const HASH_ME: &str = "hash me!";
    const VALUE: &str = "Hi";

    let mut map = HashMap::with_hasher(CityHashBuildHasher::default());
    map.insert(HASH_ME, VALUE);
    assert_eq!(map.get(&HASH_ME), Some(&VALUE));
}

#[test]
fn hasher_32_is_usable_in_std_collections(){
    use cityhash_sys::CityHash32BuildHasher;
    use std::collections::HashMap;
    const HASH_ME: &str = "hash me!";
    const VALUE: &str = "Hi";

    let mut map = HashMap::with_hasher(CityHash32BuildHasher::default());
    map.insert(HASH_ME, VALUE);
    assert_eq!(map.get(&HASH_ME), Some(&VALUE));
}

#[test]
fn hasher_64_is_usable_in_std_collections(){
    use cityhash_sys::CityHash64BuildHasher;
    use std::collections::HashMap;
    const HASH_ME: &str = "hash me!";
    const VALUE: &str = "Hi";

    let mut map = HashMap::with_hasher(CityHash64BuildHasher::default());
    map.insert(HASH_ME, VALUE);
    assert_eq!(map.get(&HASH_ME), Some(&VALUE));
}

#[test]
fn hasher_128_is_usable_in_std_collections(){
    use cityhash_sys::CityHash128BuildHasher;
    use std::collections::HashMap;
    const HASH_ME: &str = "hash me!";
    const VALUE: &str = "Hi";

    let mut map = HashMap::with_hasher(CityHash128BuildHasher::default());
    map.insert(HASH_ME, VALUE);
    assert_eq!(map.get(&HASH_ME), Some(&VALUE));
}
