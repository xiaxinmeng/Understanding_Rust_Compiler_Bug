plain
[01:13:46] 
[01:13:46] 
[01:13:46] running 399 tests
[01:14:15] ....................................................................................................
[01:14:38] ...................................................................i................F...............
[01:15:19] ...................................................................................................
[01:15:19] failures:
[01:15:19] 
[01:15:19] ---- prelude.rs - prelude (line 16) stdout ----
[01:15:19] ---- prelude.rs - prelude (line 16) stdout ----
[01:15:19] error[E0433]: failed to resolve. Did you mean `std::alloc`?
[01:15:19]  --> prelude.rs:18:5
[01:15:19] 5 | use alloc::prelude::*;
[01:15:19]   |     ^^^^^ Did you mean `std::alloc`?
[01:15:19] 
[01:15:19] thread 'prelude.rs - prelude (line 16)' panicked at 'couldn't compile the test', librustdoc/test.rs:327:17
---
[01:15:19] 
[01:15:19] error: test failed, to rerun pass '--doc'
[01:15:19] 
[01:15:19] 
[01:15:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:15:19] 
[01:15:19] 
[01:15:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:15:19] Build completed unsuccessfully in 0:29:28
[01:15:19] Build completed unsuccessfully in 0:29:28
[01:15:19] make: *** [check] Error 1
[01:15:19] Makefile:58: recipe for target 'check' failed
2842592 ./obj
2840232 ./obj/build
2245784 ./obj/build/x86_64-unknown-linux-gnu
730396 ./src
---
144184 ./obj/build/bootstrap/debug/incremental
143672 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
143668 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
129672 ./obj/build/bootstrap/debug/incremental/bootstrap-nf9tlms0eza
129668 ./obj/build/bootstrap/debug/incremental/bootstrap-nf9tlms0eza/s-f2nxwxmvrw-820vkd-jf8cfeolaswv
129452 ./obj/build/x86linux-gnu/stage2-tools
60840 ./src/llvm-emscripten/lib
60776 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
57604 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu
