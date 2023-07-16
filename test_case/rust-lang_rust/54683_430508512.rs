plain
[00:07:11] make: *** [prepare] Error 1
[00:07:12] Command failed. Attempt 2/5:
[00:07:12]     Finished dev [unoptimized] target(s) in 0.26s
[00:07:12]  Downloading quine-mc_cluskey v0.2.4
[00:07:52] warning: spurious network error (2 tries remaining): failed to download from `https://crates.io/api/v1/crates/quine-mc_cluskey/0.2.4/download`
[00:08:33] warning: spurious network error (1 tries remaining): failed to download from `https://crates.io/api/v1/crates/quine-mc_cluskey/0.2.4/download`
[00:09:13] 
[00:09:13] Caused by:
[00:09:13] Caused by:
[00:09:13]   failed to download from `https://crates.io/api/v1/crates/quine-mc_cluskey/0.2.4/download`
[00:09:13] Caused by:
[00:09:13]   [28] Timeout was reached (Resolving timed out after 30000 milliseconds)
[00:09:13] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--features" "panic-unwind jemalloc backtrace profiler" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:09:13] expected success, got: exit code: 101', build_helper/lib.rs:123:9
[00:09:13] expected success, got: exit code: 101', build_helper/lib.rs:123:9
[00:09:13] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:09:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:09:13] Build completed unsuccessfully in 0:02:00
[00:09:13] make: *** [prepare] Error 1
[00:09:15] Command failed. Attempt 3/5:
[00:09:15]     Finished dev [unoptimized] target(s) in 0.26s
[00:09:15]  Downloading phf v0.7.22
[00:09:55] warning: spurious network error (2 tries remaining): failed to download from `https://crates.io/api/v1/crates/phf/0.7.22/download`
[00:10:35] warning: spurious network error (1 tries remaining): failed to download from `https://crates.io/api/v1/crates/phf/0.7.22/download`
[00:11:15] 
[00:11:15] Caused by:
[00:11:15] Caused by:
[00:11:15]   failed to download from `https://crates.io/api/v1/crates/phf/0.7.22/download`
[00:11:15] Caused by:
[00:11:15]   [28] Timeout was reached (Resolving timed out after 30000 milliseconds)
[00:11:15] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--features" "panic-unwind jemalloc backtrace profiler" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:11:15] expected success, got: exit code: 101', build_helper/lib.rs:123:9
---
travis_time:end:046f9570:start=1539758489663193393,finish=1539758489672344123,duration=9150730
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10cf2080
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09fefa5c
travis_time:start:09fefa5c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07458edc
$ dmesg | grep -i kill
