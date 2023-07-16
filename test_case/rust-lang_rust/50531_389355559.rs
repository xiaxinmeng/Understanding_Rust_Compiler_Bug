plain
[00:06:31]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:06:36]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:08:20]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:08:39]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:08:44] error: unused import: `middle::const_val::ConstVal`
[00:08:44]   --> librustc/ty/util.rs:18:5
[00:08:44] 18 | use middle::const_val::ConstVal;
[00:08:44]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:08:44]    |
[00:08:44]    = note: `-D unused-imports` implied by `-D warnings`
[00:08:44]    = note: `-D unused-imports` implied by `-D warnings`
[00:08:44] 
_64-unknown-linux-gnu/release/deps/libtempdir-ee923a086d887011.rlib --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-aed9d8ab86b35123.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-946eff7380f27f57.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-946eff7380f27f57.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-ffd9dcc5ce13143f/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-41d448831c9d08f1/out` (exit code: 101)
[00:09:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:09:16] expected success, got: exit code: 101
[00:09:16] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:09:16] travis_fold:end:stage0-rustc

[00:09:16] travis_time:end:stage0-rustc:start=1526429856932487141,finish=1526430080730588367,duration=223798101226

