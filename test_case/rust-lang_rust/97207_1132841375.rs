
# Internal feature, only used when building as part of libstd, not part of the
# stable interface of this crate.
rustc-dep-of-std = ['core', 'alloc', 'compiler_builtins', 'adler/rustc-dep-of-std', 'simd-adler32/std']
