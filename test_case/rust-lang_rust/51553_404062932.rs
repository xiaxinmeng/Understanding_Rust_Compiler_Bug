plain
[01:49:37] warning: spurious network error (2 tries remaining): curl error: Couldn't resolve host 'github.com'
[01:49:37] ; class=Net (12)
[01:50:33] warning: spurious network error (1 tries remaining): curl error: Couldn't resolve host 'github.com'
[01:50:33] ; class=Net (12)
[01:51:29] error: failed to load source for a dependency on `rand`
[01:51:29] Caused by:
[01:51:29]   Unable to update registry `https://github.com/rust-lang/crates.io-index`
[01:51:29] 
[01:51:29] Caused by:
[01:51:29] Caused by:
[01:51:29]   failed to fetch `https://github.com/rust-lang/crates.io-index`
[01:51:29] 
[01:51:29] Caused by:
[01:51:29]   curl error: Couldn't resolve host 'github.com'
[01:51:29] ; class=Net (12)
[01:51:29] 
[01:51:29] 
[01:51:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "generate-lockfile" "--manifest-path" "/checkout/obj/build/tmp/distcheck-src/rust-src/lib/rustlib/src/rust/src/libstd/Cargo.toml"
[01:51:29] 
[01:51:29] 
[01:51:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[01:51:29] Build completed unsuccessfully in 1:48:36
---
travis_time:end:107ed913:start=1531291444480539795,finish=1531291444488798003,duration=8258208
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01d74c20
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e635a16
$ dmesg | grep -i kill
