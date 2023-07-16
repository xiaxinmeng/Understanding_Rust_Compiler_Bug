plain
[00:03:57]    Compiling cc v1.0.18
[00:03:57]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:03:57]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:03:57]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:04:00] warning: unused import: `iter::FusedIterator`
[00:04:00]   --> libcore/char/decode.rs:14:5
[00:04:00] 14 | use iter::FusedIterator;
[00:04:00]    |     ^^^^^^^^^^^^^^^^^^^
[00:04:00]    |
[00:04:00]    = note: #[warn(unused_imports)] on by default
---
[00:04:30]     |     ^^^^^^^^^^^^^^^^^^^^^
[00:04:30]     |
[00:04:30]     = note: #[warn(unused_imports)] on by default
[00:04:30] 
[00:04:30] warning: unused import: `error as std_error`
[00:04:30]    --> libstd/io/mod.rs:274:5
[00:04:30]     |
[00:04:30] 274 | use error as std_error;
[00:04:30] 
[00:04:30] warning: unused import: `result`
[00:04:30]    --> libstd/io/mod.rs:276:5
[00:04:30]     |
---
[00:18:47]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:18:53]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:18:53]    Compiling cmake v0.1.31
[00:18:53]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:18:53] error: unused import: `iter::FusedIterator`
[00:18:53]   --> libcore/char/decode.rs:14:5
[00:18:53] 14 | use iter::FusedIterator;
[00:18:53]    |     ^^^^^^^^^^^^^^^^^^^
[00:18:53]    |
[00:18:53]    = note: `-D unused-imports` implied by `-D warnings`
[00:18:53]    = note: `-D unused-imports` implied by `-D warnings`
[00:18:53] 
[00:18:57]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:18:57]    Compiling rustc_lsan v0.0.0 (file:///checkout/src/librustc_lsan)
[00:18:58]    Compiling rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:18:58]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:18:59]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
00:19:10] Makefile:28: recipe for target 'all' failed
[00:19:10] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0028daf0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jul 26 13:26:11 UTC 2018
t/modules
156240 ./.git/modules/src
149120 ./src/llvm-emscripten/test
145432 ./obj/build/bootstrap/debug/incremental
130564 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02
130560 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02/s-f39jkit88q-wmnq5k-1psc9gui095al
97528 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
88200 ./obj/build/x86_64-unknown-linux-gnu/stage1
88176 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
77028 ./.git/modules/src/tools
