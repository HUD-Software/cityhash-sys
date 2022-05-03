use core::hash::{BuildHasher, Hasher};

#[test]
fn build_hasher_64() {
    let build_hasher = cityhash_sys::CityHash64BuildHasher::default();
    let mut hasher = build_hasher.build_hasher();
    hasher.write(b"hash me!");
    assert_eq!(hasher.finish(), 0xF04A0CC67B63A0B4);
}

#[test]
fn build_hasher_result_are_coherent(){
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