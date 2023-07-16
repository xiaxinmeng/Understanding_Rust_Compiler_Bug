plain
[00:21:37]    Compiling rustc_trans v0.0.0 (file:///checkout/src/librustc_trans)
[00:21:37]    Compiling cc v1.0.10
[00:21:37]    Compiling num_cpus v1.8.0
[00:21:40]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:21:48] error: unused imports: `LayoutOf`, `self`
[00:21:48]   --> librustc_trans/declare.rs:26:25
[00:21:48]    |
[00:21:48] 26 | use rustc::ty::layout::{self, LayoutOf};
[00:21:48]    |                         ^^^^  ^^^^^^^^
[00:21:48]    = note: `-D unused-imports` implied by `-D warnings`
[00:21:48] 
[00:21:52] error: aborting due to previous error
[00:21:52] 
[00:21:52] 
[00:21:52] error: Could not compile `rustc_trans`.
[00:21:52] 
[00:21:52] Caused by:
[00:21:52]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_trans librustc_trans/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 --cfg feature="jemalloc" --cfg feature="rustc_target" -C metadata=297a49b6901057a2 -C extra-filename=-297a49b6901057a2 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-49eb0ce85cccebe8.so --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-1a0253e187e6500f.rlib --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-eaf5e781bc45472d.so --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-c634d8fa6a9464ce.rlib --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-58331467d9165b44.rlib --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-38c1456f766597c0.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-ab2cf718bbbe7ba7.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-7cdf848b4ea36a34.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-6870b7044c859a70.rlib --extern rustc_allocator=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_allocator-6d61160066ecb176.so --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-8d5a7d1b252e79e8.rlib --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libenv_logger-edf81ea33b194a72.rlib --extern rustc_incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-55ed83f623673518.so --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-ced35b4b5b9925d9.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-cf2c0cb22fec89b8.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-cf2c0cb22fec89b8.rlib --extern ree112dff218faa5/out -L native=/usr/lib/llvm-3.9/lib` (exit code: 101)
[00:21:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_trans/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
[00:21:52] expected success, got: exit code: 101
[00:21:52] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:21:52] travis_fold:start:stage0-rustc_trans
travis_time:start:stage0-rustc_trans
travis_fold:end:stage0-rustc_trans


[00:21:52] travis_time:end:stage0-rustc_trans:start=1525198894264463615,finish=1525198909587183943,duration=15322720328

[00:21:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:21:52] Build completed unsuccessfully in 0:16:52
[00:21:52] make: *** [all] Error 1
[00:21:52] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0de61710
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
149500 ./.git/modules/src
149112 ./src/llvm-emscripten/test
144660 ./obj/build/bootstrap/debug/incremental
123692 ./obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d
123688 ./obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d/s-f0my5qulwu-1cf8k11-6gvofocobazn
89684 ./src/llvm/test/CodeGen
70944 ./obj/build/x86_64-unknown-linux-gnu/native
70912 ./.git/modules/src/tools
70452 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
