plain
[00:55:17] ....i...............................................................................................
[00:55:26] ........i..ii.......................................................................................
[00:55:35] ....................................................................................................
[00:55:44] ....................................................................................................
[00:55:53] ..........................................................................i...FF....................
[00:56:13] ....................................................................................................
[00:56:22] ....................................................................................................
[00:56:32] ....................................................................................................
[00:56:33] ..........
[00:56:33] ..........
[00:56:33] failures:
[00:56:33] 
[00:56:33] ---- [compile-fail] compile-fail/rfc-2126-crate-paths/crate-path-non-absolute.rs stdout ----
[00:56:33]  
[00:56:33] error: /checkout/src/test/compile-fail/rfc-2126-crate-paths/crate-path-non-absolute.rs:17: unexpected error: '17:22: 17:27: failed to resolve. Could not find `crate` in `m` [E0433]'
[00:56:33] 
[00:56:33] error: /checkout/src/test/compile-fail/rfc-2126-crate-paths/crate-path-non-absolute.rs:17: expected error not found: `expected unit struct/variant or
[00:56:33] 
[00:56:33] error: 1 unexpected errors found, 1 expected errors not found
[00:56:33] status: exit code: 101
[00:56:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/rfc-2126-crate-paths/crate-path-non-absolute.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/rfc-2126-crate-paths/crate-path-non-absolute.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/rfc-2126-crate-paths/crate-path-non-absolute.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:56:33] unexpected errors (from JSON output): [
[00:56:33]     Error {
[00:56:33]         line_num: 17,
[0nored; 0 measured; 0 filtered out
[00:56:33] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:56:33] 
[00:56:33] 
[00:56:33] 
[00:56:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:33] 
[00:56:33] 
[00:56:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:33] Build completed unsuccessfully in 0:17:16
[00:56:33] Build completed unsuccessfully in 0:17:16
[00:56:33] Makefile:58: recipe for targ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental
110124 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
106224 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
102812 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
101920 ./obj/build/bootstrap/debug/incremental/bootstrap-2s8ik7x786gic
101920 ./obj/build/bootstrap/debug/incremental/bootstrap-2s8ik7x786gic
101916 ./obj/build/bootstrap/debug/incremental/bootstrap-2s8ik7x786gic/s-f0byc8cf6f-1ninjgh-rdc42ohq9o1k
89236 ./obj/build/x86_64-unknown-linux-gnu/stage1
89212 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
87792 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
87792 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
87788 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz/s-f0by9yos2t-mtfkkj-es32axngf1gk
84832 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
80872 ./obj/build/x86_64-unknown-linux-gnu/doc/std
78756 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/release
78580 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot
---
56092 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin
55348 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release
53568 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build
51920 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-33oa6nnkk1g08
51916 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-33oa6nnkk1g08/s-f0bybij43g-w1cmoj-133inn86rvydz
47832 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
46660 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
46656 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release
46652 ./src/test
