plain
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: lifetime may not live long enough
    --> compiler/rustc_middle/src/hir/map/mod.rs:1106:62
     |
1106 |     source_file_hashes.sort_unstable_by_key(|(name_hash, _)| name_hash);
     |                                              --------------- ^^^^^^^^^ returning this value requires that `'1` must outlive `'2`
     |                                              |             |
     |                                              |             return type of closure is &'2 u128
     |                                              has type `&'1 (u128, SourceFileHash)`
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:01:57
