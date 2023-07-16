plain
[00:18:20]    Compiling rustc_privacy v0.0.0 (file:///checkout/src/librustc_privacy)
[00:18:22]    Compiling rustc_save_analysis v0.0.0 (file:///checkout/src/librustc_save_analysis)
[00:18:32]    Compiling rustc_trans_utils v0.0.0 (file:///checkout/src/librustc_trans_utils)
[00:18:51]    Compiling rustc_lint v0.0.0 (file:///checkout/src/librustc_lint)
[00:18:51] error: this file contains an un-closed delimiter
[00:18:51]     --> librustc_lint/builtin.rs:1566:3
[00:18:51] 1566 | }
[00:18:51]      |   ^
[00:18:51]      |
[00:18:51] help: did you mean to close this delimiter?
[00:18:51] help: did you mean to close this delimiter?
[00:18:51]     --> librustc_lint/builtin.rs:1498:61
[00:18:51]      |
[00:18:51] 1498 | impl<'a, 'tcx> LateLintPass<'a, 'tcx> for UnusedBrokenConst {
[00:18:51] help: did you mean to close this delimiter?
[00:18:51]     --> librustc_lint/builtin.rs:1499:64
[00:18:51]      |
[00:18:51]      |
[00:18:51] 1499 |     fn check_item(&mut self, cx: &LateContext, it: &hir::Item) {
[00:18:51] help: did you mean to close this delimiter?
[00:18:51]     --> librustc_lint/builtin.rs:1500:23
[00:18:51]      |
[00:18:51] 1500 |         match it.node {
[00:18:51] 1500 |         match it.node {
[00:18:51]      |                       ^
[00:18:51] 
[00:18:51] error: expected pattern, found keyword `pub`
[00:18:51]     --> librustc_lint/builtin.rs:1512:1
[00:18:51]      |
[00:18:51] 1512 | pub struct TrivialConstraints;
[00:18:51] 
[00:18:51]    Compiling rustc_passes v0.0.0 (file:///checkout/src/librustc_passes)
[00:18:51]    Compiling rustc_borrowck v0.0.0 (file:///checkout/src/librustc_borrowck)
[00:18:51]    Compiling rustc_borrowck v0.0.0 (file:///checkout/src/librustc_borrowck)
[00:18:51] error[E0425]: cannot find value `TrivialConstraints` in this scope
[00:18:51]     |
[00:18:51]     |
[00:18:51] 141 |                  TrivialConstraints,
[00:18:51] 
[00:18:52] error: aborting due to 3 previous errors
[00:18:52] 
[00:18:52] For more information about this error, try `rustc --explain E0425`.
[00:18:52] For more information about this error, try `rustc --explain E0425`.
[00:18:52] error: Could not compile `rustc_lint`.
[00:18:52] 
[00:18:52] Caused by:
[00:18:52]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_lint librustc_lint/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=a4c254b297301da5 -C extra-filename=-a4c254b297301da5 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-40a8b14ac2445c60.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-3a8c539e767f664f.so --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-807d7fc1e1eacde9.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-af41b93aec4c1e93.rlib --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-6ef9097650022d88.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-902ab296c2174750.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-838d7b7b46636e17/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-c12600360c64db1e/out` (exit code: 101)
[00:19:09] error: build failed
[00:19:09] error: build failed
[00:19:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:19:09] expected success, got: exit code: 101
[00:19:09] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:19:09] travis_fold:end:stage0-rustc

[00:19:09] travis_time:end:stage0-rustc:start=1524859747595605848,finish=1524860591327436540,duration=843731830692


[00:19:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:19:09] Build completed unsuccessfully in 0:14:17
[00:19:09] Makefile:28: recipe for target 'all' failed
[00:19:09] make: *** [all] Error 1
65412 ./src/llvm-emscripten/test/CodeGen
63712 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot
63708 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib
63704 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib
---
11900 ./src/tools/lld
11572 ./.git/objects
10676 ./.git/objects/pack
10076 ./src/test/compile-fail
10012 ./src/llvm/test/MC/AMDGPU
9648 ./src/llvm/test/MC/Disassembler/AMDGPU
9304 ./.git/modules/src/tools/clippy/objects
travis_time:end:19500eb4:start=1524860591645873743,finish=1524860591869926884,duration=224053141
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
