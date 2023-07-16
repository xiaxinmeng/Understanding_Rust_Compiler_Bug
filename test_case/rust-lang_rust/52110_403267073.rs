plain
[01:01:50] test [run-pass] run-pass/default-method-simple.rs ... ok
[01:01:51] test [run-pass] run-pass/defaults-well-formedness.rs ... ok
[01:01:51] test [run-pass] run-pass/default-method-supertrait-vtable.rs ... ok
[01:01:51] test [run-pass] run-pass/deprecation-in-force-unstable.rs ... ok
[01:01:51] test [run-pass] run-pass/default-alloc-error-hook.rs ... ok
[01:01:51] test [run-pass] run-pass/deref-mut-on-ref.rs ... ok
[01:01:51] test [run-pass] run-pass/deref-on-ref.rs ... ok
[01:01:52] test [run-pass] run-pass/deref.rs ... ok
[01:01:52] test [run-pass] run-pass/deref-rc.rs ... ok
---
[01:51:14] warning: spurious network error (2 tries remaining): curl error: Couldn't resolve host 'github.com'
[01:51:14] ; class=Net (12)
[01:52:10] warning: spurious network error (1 tries remaining): curl error: Couldn't resolve host 'github.com'
[01:52:10] ; class=Net (12)
[01:53:06] error: failed to load source for a dependency on `rand`
[01:53:06] Caused by:
[01:53:06]   Unable to update registry `https://github.com/rust-lang/crates.io-index`
[01:53:06] 
[01:53:06] Caused by:
[01:53:06] Caused by:
[01:53:06]   failed to fetch `https://github.com/rust-lang/crates.io-index`
[01:53:06] 
[01:53:06] Caused by:
[01:53:06]   curl error: Couldn't resolve host 'github.com'
[01:53:06] ; class=Net (12)
[01:53:06] 
[01:53:06] 
[01:53:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "generate-lockfile" "--manifest-path" "/checkout/obj/build/tmp/distcheck-src/rust-src/lib/rustlib/src/rust/src/libstd/Cargo.toml"
[01:53:06] 
[01:53:06] 
[01:53:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[01:53:06] Build completed unsuccessfully in 1:49:57
---
travis_time:end:302fa327:start=1531032879579873433,finish=1531032879586380030,duration=6506597
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:036578be
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1197db33
$ dmesg | grep -i kill
