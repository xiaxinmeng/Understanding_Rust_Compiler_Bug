plain
[00:13:54]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:16:44]    Compiling rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:17:53]    Compiling rustc_resolve v0.0.0 (file:///checkout/src/librustc_resolve)
[00:17:54]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:17:54] error[E0425]: cannot find value `path_segments` in this scope
[00:17:54]    --> librustc_resolve/macros.rs:469:24
[00:17:54]     |
[00:17:54] 469 |                 .push((path_segments.into_boxed_slice(), span));
[00:17:54] 
[00:17:56] error: aborting due to previous error
[00:17:56] 
[00:17:56] For more information about this error, try `rustc --explain E0425`.
[00:17:56] For more information about this error, try `rustc --explain E0425`.
[00:17:56] error: Could not compile `rustc_resolve`.
[00:17:56] 
[00:17:56] Caused by:
[00:17:56]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_resolve librustc_resolve/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=8ebf33ee495224fa -C extra-filename=-8ebf33ee495224fa --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-7cdf848b4ea36a34.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-6ed50b2d1f28e8a9.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-af41b93aec4c1e93.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-5a636a569660b030.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-24d121bd290f1f21.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-f44070055fa5d2f8.so --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-19bf33dd406ed70a.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-838d7b7b46636e17/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-c12600360c64db1e/out` (exit code: 101)
[00:18:03] error: build failed
[00:18:03] error: build failed
[00:18:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:18:03] expected success, got: exit code: 101
[00:18:03] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:18:03] travis_fold:end:stage0-rustc

[00:18:03] travis_time:end:stage0-rustc:start=1525111617921793813,finish=1525112390095419600,duration=772173625787


[00:18:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:18:03] Build completed unsuccessfully in 0:13:06
[00:18:03] Makefile:28: recipe for target 'all' failed
[00:18:03] make: *** [all] Error 1
60840 ./src/llvm-emscripten/lib
56092 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin
48604 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
47832 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
---
12160 ./src/llvm-emscripten/include/llvm
11900 ./src/tools/lld
11748 ./src/doc
10080 ./src/test/compile-fail
10012 ./src/llvm/test/MC/AMDGPU
9648 ./src/llvm/test/MC/Disassembler/AMDGPU
travis_time:end:001fe429:start=1525112390375488252,finish=1525112390613839691,duration=238351439
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:195abd6e
