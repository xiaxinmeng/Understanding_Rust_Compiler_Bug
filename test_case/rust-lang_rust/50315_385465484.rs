plain
[00:17:53]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:19:18]    Compiling rustc_resolve v0.0.0 (file:///checkout/src/librustc_resolve)
[00:19:19]    Compiling rustc_plugin v0.0.0 (file:///checkout/src/librustc_plugin)
[00:19:19]    Compiling rustc_privacy v0.0.0 (file:///checkout/src/librustc_privacy)
[00:19:19] error[E0425]: cannot find value `path_segments` in this scope
[00:19:19]    --> librustc_resolve/macros.rs:469:24
[00:19:19]     |
[00:19:19] 469 |                 .push((path_segments.into_boxed_slice(), span));
[00:19:19] 
[00:19:21] error: aborting due to previous error
[00:19:21] 
[00:19:21] For more information about this error, try `rustc --explain E0425`.
[00:19:21] For more information about this error, try `rustc --explain E0425`.
[00:19:21] error: Could not compile `rustc_resolve`.
[00:19:21] 
[00:19:21] Caused by:
[00:19:21]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_resolve librustc_resolve/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=8ebf33ee495224fa -C extra-filename=-8ebf33ee495224fa --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-af41b93aec4c1e93.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-f44070055fa5d2f8.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-7cdf848b4ea36a34.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-6ed50b2d1f28e8a9.so --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-19bf33dd406ed70a.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-5a636a569660b030.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-24d121bd290f1f21.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-838d7b7b46636e17/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-c12600360c64db1e/out` (exit code: 101)
[00:19:31] error: build failed
[00:19:31] error: build failed
[00:19:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:19:31] expected success, got: exit code: 101
[00:19:31] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:19:31] travis_fold:end:stage0-rustc

[00:19:31] travis_time:end:stage0-rustc:start=1525107635623841994,finish=1525108478828423434,duration=843204581440


[00:19:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:19:31] Build completed unsuccessfully in 0:14:18
[00:19:31] Makefile:28: recipe for target 'all' failed
[00:19:31] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:29ccdd3b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
149120 ./src/llvm-emscripten/test
144660 ./obj/build/bootstrap/debug/incremental
133972 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
123692 ./obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d
123688 ./obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d/s-f0lsonildk-10zkl3f-6gvofocobazn
89828 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
89824 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
89692 ./src/llvm/test/CodeGen
70944 ./obj/build/x86_64-unknown-linux-gnu/native
