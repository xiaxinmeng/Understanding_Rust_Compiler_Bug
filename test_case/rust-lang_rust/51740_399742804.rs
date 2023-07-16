plain
[00:02:56] Caused by:
[00:02:56]   failed to get 200 response from `https://crates.io/api/v1/crates/time/0.1.39/download`, got 500
[00:02:56] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:56] Build completed unsuccessfully in 0:02:08
[00:02:56] Makefile:81: recipe for target 'prepare' failed
[00:02:56] make: *** [prepare] Error 1
[00:02:57]  Downloading lazy_static v0.2.11
[00:03:13] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/lazy_static/0.2.11/download`, got 500
[00:03:13]  Downloading serde_json v1.0.15
[00:03:13]  Downloading serde v1.0.40
---
[00:04:33] Caused by:
[00:04:33]   [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:04:33] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:04:33] Build completed unsuccessfully in 0:01:36
[00:04:33] make: *** [prepare] Error 1
[00:04:33] Makefile:81: recipe for target 'prepare' failed
[00:04:36]  Downloading toml v0.4.6
[00:04:36]  Downloading libc v0.2.40
[00:05:06] warning: spurious network error (2 tries remaining): [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:05:36] warning: spurious network error (1 tries remaining): [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:05:36] warning: spurious network error (1 tries remaining): [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:06:06] error: unable to get packages from source
[00:06:06] 
[00:06:06] Caused by:
[00:06:06]   [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:06:06] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:06:06] Build completed unsuccessfully in 0:01:30
[00:06:06] make: *** [prepare] Error 1
[00:06:06] Makefile:81: recipe for target 'prepare' failed
[00:06:10]  Downloading time v0.1.39
[00:06:40] warning: spurious network error (2 tries remaining): [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:06:40]  Downloading filetime v0.1.15
[00:06:40]  Downloading cmake v0.1.30
---
[00:08:11] Caused by:
[00:08:11]   [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:08:11] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:08:11] Build completed unsuccessfully in 0:02:01
[00:08:11] make: *** [prepare] Error 1
[00:08:11] Makefile:81: recipe for target 'prepare' failed
[00:08:15]  Downloading cmake v0.1.30
[00:08:45] warning: spurious network error (2 tries remaining): [28] Timeout was reached (Operation too slow. Less than 10 bytes/sec transferred the last 30 seconds)
[00:09:08]  Downloading getopts v0.2.17
[00:09:08]  Downloading libc v0.2.40
---
[00:11:05]  Downloading rls-data v0.16.0
[00:11:05]  Downloading rustc-serialize v0.3.24
[00:11:05]  Downloading radix_trie v0.1.3
[00:11:30]  Downloading rustc-ap-serialize v164.0.0
[00:11:48] warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/rustc-ap-serialize/164.0.0/download`, got 500
[00:12:02] warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/rustc-ap-serialize/164.0.0/download`, got 500
[00:12:18] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: HttpNot200 { code: 500, url: "https://crates.io/api/v1/crates/rustc-ap-serialize/164.0.0/download" }
[00:12:18] unable to get packages from source', libcore/result.rs:945:5
[00:12:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:12:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:12:18] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:12:18] expected success, got: exit code: 101', build_helper/lib.rs:122:9
[00:12:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:12:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:12:18] Build completed unsuccessfully in 0:04:03
[00:12:18] Makefile:81: recipe for target 'prepare' failed
[00:12:18] make: *** [prepare] Error 1
[00:12:18] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:2df4afc4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:04707bb6:start=1529832681741583919,finish=1529832681747144346,duration=5560427
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:187e2d60
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00b6cf38
$ dmesg | grep -i kill
