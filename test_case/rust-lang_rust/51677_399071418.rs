plain
[00:15:33] 
#########                                                                 13.2%
######################################################################## 100.0%
[00:15:33] extracting /checkout/obj/build/tmp/distcheck/build/cache/2018-06-20/cargo-0.28.0-x86_64-unknown-linux-gnu.tar.gz
[00:15:33] error: failed to load source for a dependency on `cc`
[00:15:33] Caused by:
[00:15:33]   Unable to update registry `https://github.com/rust-lang/crates.io-index`
[00:15:33] 
[00:15:33] Caused by:
[00:15:33] Caused by:
[00:15:33]   failed to update replaced source registry `https://github.com/rust-lang/crates.io-index`
[00:15:33] 
[00:15:33] Caused by:
[00:15:33]   failed to parse manifest at `/checkout/obj/build/tmp/distcheck/src/vendor/clippy_lints/Cargo.toml`
[00:15:33] 
[00:15:33] Caused by:
[00:15:33]   the cargo feature `edition` requires a nightly version of Cargo, but this is the `stable` channel
[00:15:33] failed to run: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/obj/build/tmp/distcheck/src/bootstrap/Cargo.toml --locked --frozen
[00:15:33] Build completed unsuccessfully in 0:00:14
[00:15:33] Makefile:58: recipe for target 'check' failed
[00:15:33] make: *** [check] Error 1
[00:15:33] 
[00:15:33] 
[00:15:33] command did not execute successfully: "make" "check"
[00:15:33] 
[00:15:33] 
[00:15:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:15:33] Build completed unsuccessfully in 0:11:56
---
travis_time:end:0f17f440:start=1529580411097826901,finish=1529580411104056288,duration=6229387
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a377784
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a2e6d0f
$ dmesg | grep -i kill
