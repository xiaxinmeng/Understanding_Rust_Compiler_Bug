plain
[00:02:21]  Downloading crossbeam-deque v0.2.0
[00:02:22]  Downloading backtrace-sys v0.1.22
[00:02:22]  Downloading regex-syntax v0.5.5
[00:02:22]  Downloading rustfix v0.3.1
[00:02:22]  Downloading ordslice v0.3.0
[00:02:22]  Downloading matches v0.1.6
[00:02:22]  Downloading wincolor v0.1.6
[00:02:22]  Downloading aho-corasick v0.6.4
[00:02:22]  Downloading open v1.2.1
---
[00:13:25]    Compiling serde_derive_internals v0.23.1
[00:13:27]    Compiling serde_derive v1.0.40
[00:13:32]    Compiling bootstrap v0.0.0 (file:///checkout/obj/build/tmp/distcheck/src/bootstrap)
[00:14:03]     Finished dev [unoptimized] target(s) in 54.22s
[00:14:03] error: failed to load source for a dependency on `cc`
[00:14:03] Caused by:
[00:14:03]   Unable to update registry `https://github.com/rust-lang/crates.io-index`
[00:14:03] 
[00:14:03] Caused by:
[00:14:03] Caused by:
[00:14:03]   failed to update replaced source registry `https://github.com/rust-lang/crates.io-index`
[00:14:03] 
[00:14:03] Caused by:
[00:14:03]   failed to parse manifest at `/checkout/obj/build/tmp/distcheck/src/vendor/clippy_lints/Cargo.toml`
[00:14:03] 
[00:14:03] Caused by:
[00:14:03]   the cargo feature `edition` requires a nightly version of Cargo, but this is the `beta` channel
[00:14:03] Build completed unsuccessfully in 0:01:11
[00:14:03] Build completed unsuccessfully in 0:01:11
[00:14:03] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/obj/build/tmp/distcheck/src/libstd/Cargo.toml"
[00:14:03] expected success, got: exit code: 101', build_helper/lib.rs:122:9
[00:14:03] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:14:03] make: *** [check] Error 1
[00:14:03] Makefile:58: recipe for target 'check' failed
[00:14:03] 
[00:14:03] 
[00:14:03] command did not execute successfully: "make" "check"
[00:14:03] 
[00:14:03] 
[00:14:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:14:03] Build completed unsuccessfully in 0:11:22
---
travis_time:end:0fbc6349:start=1530494070835204636,finish=1530494070841979243,duration=6774607
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:008c09d0
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02ba273a
$ dmesg | grep -i kill
