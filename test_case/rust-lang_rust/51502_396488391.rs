plain
[01:58:22] warning: spurious network error (2 tries remaining): curl error: Couldn't resolve host 'github.com'
[01:58:22] ; class=Net (12)
[01:59:18] warning: spurious network error (1 tries remaining): curl error: Couldn't resolve host 'github.com'
[01:59:18] ; class=Net (12)
[02:00:14] error: failed to load source for a dependency on `rand`
[02:00:14] Caused by:
[02:00:14]   Unable to update registry `https://github.com/rust-lang/crates.io-index`
[02:00:14] 
[02:00:14] Caused by:
[02:00:14] Caused by:
[02:00:14]   failed to fetch `https://github.com/rust-lang/crates.io-index`
[02:00:14] 
[02:00:14] Caused by:
[02:00:14]   curl error: Couldn't resolve host 'github.com'
[02:00:14] ; class=Net (12)
[02:00:14] 
[02:00:14] 
[02:00:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "generate-lockfile" "--manifest-path" "/checkout/obj/build/tmp/distcheck-src/rust-src/lib/rustlib/src/rust/src/libstd/Cargo.toml"
[02:00:14] 
[02:00:14] 
[02:00:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[02:00:14] Build completed unsuccessfully in 1:57:13
---
travis_time:end:0138cab2:start=1528787192782011547,finish=1528787192788334313,duration=6322766
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10df1246
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2144bef0
$ dmesg | grep -i kill
