plain
[00:04:25]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:04:39]    Compiling libc v0.0.0 (/checkout/src/rustc/libc_shim)
[00:04:39]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:04:40]    Compiling alloc_system v0.0.0 (/checkout/src/liballoc_system)
[00:04:40] error: expected `)`, found `,`
[00:04:40]   --> liballoc_system/lib.rs:25:38
[00:04:40]    |
[00:04:40] 25 |     feature(integer_atomics, stdsimd),
[00:04:40]    |                                      ^ expected `)`
[00:04:40] error: aborting due to previous error
[00:04:40] 
[00:04:40] error: Could not compile `alloc_system`.
[00:04:40] warning: build failed, waiting for other jobs to finish...
---
151492 ./obj/build/bootstrap/debug/incremental
151412 ./src/tools/clang
149116 ./src/llvm-emscripten/test
136028 ./obj/build/bootstrap/debug/incremental/bootstrap-3ivyub3ic2113
136024 ./obj/build/bootstrap/debug/incremental/bootstrap-3ivyub3ic2113/s-f5l0vkwrpx-xl25bp-374dpar6w80ak
107652 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
104700 ./src/tools/lldb
93748 ./src/tools/clang/test
73656 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
