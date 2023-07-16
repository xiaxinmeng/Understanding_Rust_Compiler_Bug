rust
    #[cfg_attr(not(any(target_os = "emscripten", target_os = "redox",
                       target_endian = "big")),
               repr(simd))]
    struct Block(u64, u64, u64, u64);
