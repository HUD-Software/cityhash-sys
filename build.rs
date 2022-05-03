fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/**");

    let mut build = cc::Build::new();
    build
        .include("src/google")
        .cpp(true)
        .file("src/google/city.cc");

    // CRC32C Intrinsic can be used on x86_64 architecture that support the SSE 4.2 instruction set
    if cfg!(target_arch = "x86_64") && cfg!(target_feature = "sse4.2") {
        if build.get_compiler().is_like_msvc() {
            // MSVC cl.exe compiler do not support sse4.2 options,
            // We just have to define the __SSE4_2__ macro and let the compiler use the _mm_crc32_u64 intrinsic
            build.define("__SSE4_2__", None);
        } else if build.get_compiler().is_like_gnu() || build.get_compiler().is_like_clang() {
            // Clang and GCC support the SSE 4.2 instruction set flag
            build.flag("-msse4.2");
        }
    }

    build.compile("google_cityhash");
}
