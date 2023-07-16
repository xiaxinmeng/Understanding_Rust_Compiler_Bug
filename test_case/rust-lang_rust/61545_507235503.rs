plain
[01:44:19] test client_test_simple_workspace ... ok
[01:44:19] test client_workspace_symbol_duplicates ... ok
[01:44:46] thread panicked while panicking. aborting.
[01:44:46] [2019-07-01T11:55:13Z ERROR rls::server] Can't read message
[01:44:46] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/libcore/result.rs:999:5
[01:44:46] note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:44:46] error: process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/client-b5d970883c131c9a` (signal: 4, SIGILL: illegal instruction)
[01:44:46] 
[01:44:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rls/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--"
[01:44:46] expected success, got: exit code: 101
[01:44:46] 
---
travis_time:end:0d8f5a5f:start=1561982478197543450,finish=1561982478393244277,duration=195700827
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14aad7ea
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:021483bb
$ dmesg | grep -i kill
