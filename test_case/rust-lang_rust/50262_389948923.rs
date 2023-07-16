plain
[00:19:32]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:19:34] error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
[00:19:34]     --> librustc_typeck/check/mod.rs:1085:24
[00:19:34]      |
[00:19:34] 1085 |                 if let hir::Expr_::ExprBlock(ref block) = body.value.node {
[00:19:34] 
[00:19:42] error: aborting due to previous error
[00:19:42] 
[00:19:42] For more information about this error, try `rustc --explain E0023`.
[00:19:42] For more information about this error, try `rustc --explain E0023`.
[00:19:42] error: Could not compile `rustc_typeck`.
[00:19:42] 
[00:19:42] Caused by:
[00:19:42]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_typeck librustc_typeck/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=a8e94a2d6099378c -C extra-filename=-a8e94a2d6099378c --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-1f4a8dc6f96438b3.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-591332c21fa62c5a.so --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-e441b2a0deb55515.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-0563edfb8a25ec52.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-7b60e30e029cf8d2.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-2c4c636497cc7d26.so --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-451f73226863cf84.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-f54c5dc31d03e309.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-c3ce81d43ad9f8bf.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-4d49ca059d8c768d.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-e60e9f8c8a3c23c0/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-76b2f52a872e091f/out` (exit code: 101)
[00:23:50] error: build failed
[00:23:50] error: build failed
[00:23:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:23:50] expected success, got: exit code: 101
[00:23:50] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:23:50] travis_fold:end:stage0-rustc

[00:23:50] travis_time:end:stage0-rustc:start=1526577700915084597,finish=1526578697018288364,duration=996103203767


[00:23:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:23:50] Build completed unsuccessfully in 0:16:54
[00:23:50] make: *** [all] Error 1
[00:23:50] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2cbe1368
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
