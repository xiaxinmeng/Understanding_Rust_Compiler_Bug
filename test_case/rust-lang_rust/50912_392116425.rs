plain
[00:07:18]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:07:23]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:09:33]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:09:58]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:10:02] error: unused imports: `IntegerExt`, `Integer`
[00:10:02]   --> librustc/mir/mod.rs:31:18
[00:10:02]    |
[00:10:02] 31 | use ty::layout::{Integer, IntegerExt};
[00:10:02]    |                  ^^^^^^^  ^^^^^^^^^^
[00:10:02]    = note: `-D unused-imports` implied by `-D warnings`
[00:10:02] 
[00:10:02] error: unused import: `syntax::attr::SignedInt`
[00:10:02]   --> librustc/mir/mod.rs:42:5
---
[00:10:43] 
[00:10:43] error: Could not compile `rustc`.
[00:10:43] 
[00:10:43] Caused by:
[00:10:43]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=3 -C metadata=c7310be923214c6b -C extra-filename=-c7310be923214c6b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-2bee8f0c1e719b79.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a8f5cae0510be7d3.so --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-ae6ba972807bb99f.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-bde077c9625bcd15.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-e2bf728a74e7d80c.rlib --extern tempdir=/checkout/obj/build/x86_64-unknown-li0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-6f643d03661f77af.so --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-95178d8862fa1ba9.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-a7e6c70c1d8dd47c.so --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-7575351e73c880a2.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-851dbea9607dbada.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-f085762345e9053e/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-c0082fee642cc0bf/out` (exit code: 101)
[00:10:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:10:43] expected success, got: exit code: 101
[00:10:43] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:10:43] travis_fold:end:stage0-rustc

[00:10:43] travis_time:end:stage0-rustc:start=1527266574187517695,finish=1527266852795199084,duration=278607681389


[00:10:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:10:43] Build completed unsuccessfully in 0:04:56
[00:10:43] make: *** [all] Error 1
[00:10:43] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:228da1c6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
