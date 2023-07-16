plain
[00:05:54]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:07:30]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:30]    Compiling rustc_const_math v0.0.0 (file:///checkout/src/librustc_const_math)
[00:07:56]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:08:00] error[E0432]: unresolved import `codemap`
[00:08:00]   --> librustc/lint/mod.rs:57:5
[00:08:00]    |
[00:08:00] 57 | use codemap::{ExpnFormat, ExpnInfo, Span };
[00:08:00]    |     ^^^^^^^ Did you mean `syntax::codemap`?
[00:08:00] 
[00:08:00] error[E0433]: failed to resolve. Use of undeclared type or module `lint`
[00:08:00]     |
[00:08:00]     |
[00:08:00] 472 |                 if !lint::in_external_macro(lint, span) {
[00:08:00]     |                     ^^^^ Use of undeclared type or module `lint`
[00:08:03] error: unused import: `Span`
[00:08:03]   --> librustc/lint/mod.rs:57:37
[00:08:03]    |
[00:08:03]    |
[00:08:03] 57 | use codemap::{ExpnFormat, ExpnInfo, Span };
[00:08:03]    |
[00:08:03]    |
[00:08:03]    = note: `-D unused-imports` implied by `-D warnings`
[00:08:03] 
tc/x86_64-unknown-linux-gnu/release/deps/libserialize-c04ded78717d5d67.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-c04ded78717d5d67.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-3e35efb9e5bc7a9a.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-9866e194db82a141.rlib --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-dd80e21b25e89693.so --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-a9575695dd92c62e.rlib --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-895f7ddc4467bb8d.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-1299638d641ea770.rlib --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-3532894d6c1d20e5.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-5bcc8c1ccd509892.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-6e4119b5ec8457a3.so --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-3706e912fdb98df1.so --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-ca61dfec7c40c4d4.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-609e2421d03f9c9a.so --extern rustc_back=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_back-861dba9fe03aa669.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-93cb1ddd29ab61a4.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-d3b6fcf798f7d22a.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-22398a187b4139a2/out/.libs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-4a7fffed6b170d5b/out` (exit code: 101)
[00:08:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:41] expected success, got: exit code: 101
[00:08:41] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:08:41] travis_fold:end:stage0-rustc

[00:08:41] travis_time:end:stage0-rustc:start=1524510260108531942,finish=1524510468939692692,duration=208831160750


[00:08:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:08:41] Build completed unsuccessfully in 0:03:42
[00:08:41] Makefile:28: recipe for target 'all' failed
[00:08:41] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c0a4dd2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
