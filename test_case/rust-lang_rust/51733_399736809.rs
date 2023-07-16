plain
[00:02:27]  Downloading petgraph v0.4.12
[00:02:33]  Downloading num_cpus v1.8.0
[00:02:37]  Downloading itoa v0.4.1
[00:02:37]  Downloading dtoa v0.4.2
[00:02:48] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/dtoa/0.4.2/download`, got 500
[00:02:48]  Downloading cfg-if v0.1.2
[00:02:59]  Downloading syn v0.13.1
[00:02:59]  Downloading quote v0.5.1
[00:03:02]  Downloading serde_derive_internals v0.23.1
---
[00:03:52]     Finished dev [unoptimized] target(s) in 2m 51.63s
[00:03:52]  Downloading rayon-core v1.4.0
[00:04:04] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/rayon-core/1.4.0/download`, got 500
[00:04:04]  Downloading either v1.5.0
[00:04:17] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/either/1.5.0/download`, got 500
[00:04:17]  Downloading curl v0.4.12
[00:04:17]  Downloading log v0.4.1
[00:04:32] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/log/0.4.1/download`, got 500
[00:04:49] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/log/0.4.1/download`, got 500
[00:04:49] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/log/0.4.1/download`, got 500
[00:05:05] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: HttpNot200 { code: 500, url: "https://crates.io/api/v1/crates/log/0.4.1/download" }
[00:05:05] unable to get packages from source', libcore/result.rs:945:5
[00:05:05] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:05:05] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:05:05] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:05:05] expected success, got: exit code: 101', build_helper/lib.rs:122:9
[00:05:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
[00:05:05] Build completed unsuccessfully in 0:04:22
travis_time:end:1181279c:start=1529826034504923079,finish=1529826340508325166,duration=306003402087

---
travis_time:end:1689c5c3:start=1529826340796160409,finish=1529826340804432813,duration=8272404
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a5dc466
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:15dd9676
$ dmesg | grep -i kill
