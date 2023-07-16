plain
[01:18:51] ....................................................................................................
[01:19:10] ..........................i.................................................................
[01:19:10] failures:
[01:19:10] 
[01:19:10] ---- option.rs - option::Option<T>::xor (line 715) stdout ----
[01:19:10]  thread 'option.rs - option::Option<T>::xor (line 715)' panicked at 'test executable failed:
[01:19:10] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:19:10] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:19:10]   left: `Some(2)`,
[01:19:10]  right: `None`', option.rs:12:1
[01:19:10] 
[01:19:10] ', librustdoc/test.rs:356:17
[01:19:10] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:19:10] 
---
[01:19:10] 
[01:19:10] error: test failed, to rerun pass '--doc'
[01:19:10] 
[01:19:10] 
[01:19:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:19:10] 
[01:19:10] 
[01:19:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:10] Build completed unsuccessfully in 0:34:32
[01:19:10] Build completed unsuccessfully in 0:34:32
[01:19:10] Makefile:58: recipe for target 'check' failed
[01:19:10] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00a83011
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May  9 06:33:51 UTC 2018
0 ./obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d/s-f0v6lhbkmm-7mne91-3a8ftmql7i7fg
113068 ./obj/build/x86_64-unknown-linux-gnu/test/mir-opt
112640 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
112636 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
108644 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
108644 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
103488 ./obj/build/bootstrap/debug/incremental/bootstrap-2wettvttcntnm
103484 ./obj/build/bootstrap/debug/incremental/bootstrap-2wettvttcntnm/s-f0v7s1ubrs-9bdn3m-2ysq90q36kyjn
98488 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental
93316 ./obj/build/x86_64-unknown-linux-gnu/stage1
93292 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
90808 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
90808 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
90804 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz/s-f0v7ppoiyy-rt72ks-dwv0lu6ptg5j
86672 ./obj/build/x86_64-unknown-linux-gnu/doc/core
81220 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
80928 ./obj/build/x86_64-unknown-linux-gnu/doc/std
79032 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot
