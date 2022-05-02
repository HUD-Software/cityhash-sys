#ifndef RUST_LACK_UINT128_COMPATIBLE_TYPE_H
#define RUST_LACK_UINT128_COMPATIBLE_TYPE_H

extern "C" void CityHash128(char const *s, size_t len, uint64* result_low_128, uint64* result_high_128 ) {
  uint128 result_128 = CityHash128(s, len);
  *result_low_128 = Uint128Low64(result_128);
  *result_high_128 = Uint128High64(result_128);
}

extern "C" void CityHash128WithSeed(char const *s, size_t len, uint64 seed_low_128, uint64 seed_high_128, uint64* result_low_128, uint64* result_high_128) {
  const uint128 seed{seed_low_128, seed_high_128};
  uint128 result_128 = CityHash128WithSeed(s, len, seed);
  *result_low_128 = Uint128Low64(result_128);
  *result_high_128 = Uint128High64(result_128);
}

extern "C" uint64 Hash128to64(uint64 low_128, uint64 high_128) {
  const uint128 seed{low_128, high_128};
  return Hash128to64(seed);
}

#ifdef __SSE4_2__

extern "C" void CityHashCrc128(const char *s, size_t len, uint64* result_low_128, uint64* result_high_128 ) {
  uint128 result_128 = CityHashCrc128(s, len);
  *result_low_128 = Uint128Low64(result_128);
  *result_high_128 = Uint128High64(result_128);
}

#endif


#endif // RUST_LACK_UINT128_COMPATIBLE_TYPE_H