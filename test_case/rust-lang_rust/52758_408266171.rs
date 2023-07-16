plain
[00:02:18] Successfully tagged rust-ci:latest
[00:02:18] Built container sha256:3a2c81f9b8a1633da099160c8d1f37e042d4675ae221b86883d1b84f4b5f9e04
[00:02:18] Uploading finished image to s3://rust-lang-ci-sccache2/docker/254673b822b84233b3d8510170c1c331460198676199202c3bf8d1c7c9e0e1b16875995a19ea0d0d82a3789a612f21c840d7e6245ccdc54916e891d31de24244
[00:02:18] 
[00:02:18] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:02:25] xargs: docker: terminated by signal 13

[00:02:26] travis_time:end:153414c9:start=1532647368838744144,finish=1532647503244758706,duration=134406014562
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:02:26] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---
[00:06:41]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:06:44]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:07:55]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:08:04]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:08:21] error[E0616]: field `no_leak_check` of struct `session::config::DebuggingOptions` is private
[00:08:21]    --> librustc/infer/higher_ranked/mod.rs:628:12
[00:08:21]     |
[00:08:21] 628 |         if self.tcx.sess.opts.debugging_opts.no_leak_check {
[00:08:21] 
[00:08:21] 
3cb288550.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-2b513184d9d612c2.rlib --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-67c8aaeb97e8d843.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-09a8465c475acbfe.so --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-d405c21c3677962b.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-a185b6f37f2c1490.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-96863e4e4a7d8742.rlib --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-1436fcd917ac8cb6.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-7f18c5c76091a313.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-e808ab63ca762e81.so --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-a4d022068ecc6660.rlib --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-5068b78226e8859d.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-f1825829e8a26bad.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-6755b57849a2d1c7.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-2792a41a3401596f.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-5aa06a687b1a439d.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-2d0488f0587a9fe8.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5615a04cb21f88e7.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-5615a04cb21f88e7.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-62c7dbd871cbb630.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-04bbe800f5e81b33.so --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-7fdf657794f85f21.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-7205e6adb9d66e6b/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-27571ee36438df44/out` (exit code: 101)
[00:08:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:31] expected success, got: exit code: 101
[00:08:31] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:08:31] travis_fold:end:stage0-rustc

[00:08:31] travis_time:end:stage0-rustc:start=1532647716485006847,finish=1532647868703844371,duration=152218837524


[00:08:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:08:31] Build completed unsuccessfully in 0:03:25
[00:08:31] make: *** [all] Error 1
[00:08:31] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0682a90a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
