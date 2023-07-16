plain
[00:07:11]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:35] error[E0308]: mismatched types
[00:07:35]     --> librustc/traits/error_reporting.rs:1064:21
[00:07:35]      |
[00:07:35] 1064 | /                     format!("change the closure to take and ignore the argument{}",
[00:07:35] 1065 | |                             if expected_args.is_empty() {
[00:07:35] 1066 | |                                 ""
[00:07:35] 1068 | |                                 "s"
[00:07:35] 1069 | |                             }
[00:07:35] 1070 | |                     ),
[00:07:35]      | |_____________________^ expected &str, found struct `std::string::String`
[00:07:35]      | |_____________________^ expected &str, found struct `std::string::String`
[00:07:35]      |
[00:07:35]      = note: expected type `&str`
[00:07:35]                 found type `std::string::String`
[00:07:35]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:07:35] help: consider borrowing here
[00:07:35]      |
[00:07:35] 1064 |                     &format!("change the closure to take and ignore the argument{}",
[00:07:35] 1065 |                             if expected_args.is_empty() {
[00:07:35] 1066 |                                 ""
[00:07:35] 1068 |                                 "s"
[00:07:35] 1069 |                             }
[00:07:35]    ...
[00:07:35] 
[00:07:35] 
3cb288550.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-2b513184d9d612c2.rlib --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-67c8aaeb97e8d843.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-09a8465c475acbfe.so --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-d405c21c3677962b.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-a185b6f37f2c1490.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-96863e4e4a7d8742.rlib --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-1436fcd917ac8cb6.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-7f18c5c76091a313.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-e808ab63ca762e81.so --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-a4d022068ecc6660.rlib --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-5068b78226e8859d.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-f1825829e8a26bad.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-6755b57849a2d1c7.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-2792a41a3401596f.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-5aa06a687b1a439d.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-2d0488f0587a9fe8.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5615a04cb21f88e7.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5615a04cb21f88e7.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-62c7dbd871cbb630.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-04bbe800f5e81b33.so --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-7fdf657794f85f21.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-7205e6adb9d66e6b/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-27571ee36438df44/out` (exit code: 101)
[00:07:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:41] expected success, got: exit code: 101
[00:07:41] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:07:41] travis_fold:end:stage0-rustc

[00:07:41] travis_time:end:stage0-rustc:start=1532358685224111533,finish=1532358852502092586,duration=167277981053


[00:07:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:41] Build completed unsuccessfully in 0:03:46
[00:07:41] make: *** [all] Error 1
[00:07:41] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2182ad1d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
156608 ./.git/modules/src
149120 ./src/llvm-emscripten/test
145372 ./obj/build/bootstrap/debug/incremental
130560 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02
130556 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02/s-f36bs9i4eh-17t1bqo-3om13ebi8plfd
97528 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
78976 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
77504 ./.git/modules/src/tools
71508 ./src/llvm/lib
---
travis_time:end:0133518c:start=1532358853006798651,finish=1532358853015682489,duration=8883838
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2df252d6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:038a3b9d
$ cat ./obj/build/x86_64-unknown-l
