plain
[00:02:46]     Finished dev [unoptimized] target(s) in 1m 35s
[00:02:46] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:02:46] 
[00:02:46] Caused by:
[00:02:46]   the cargo feature `edition` requires a nightly version of Cargo, but this is the `beta` channel
[00:02:46] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:02:46] expected success, got: exit code: 101', build_helper/lib.rs:122:9
[00:02:46] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:02:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:02:46] Build completed unsuccessfully in 0:01:59
[00:02:46] Makefile:81: recipe for target 'prepare' failed
[00:02:46] make: *** [prepare] Error 1
[00:02:48]     Finished dev [unoptimized] target(s) in 0.26s
[00:02:48] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:02:48] 
[00:02:48] Caused by:
[00:02:48] Caused by:
[00:02:48]   the cargo feature `edition` requires a nightly version of Cargo, but this is the `beta` channel
[00:02:48] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:02:48] expected success, got: exit code: 101', build_helper/lib.rs:122:9
[00:02:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:02:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:02:48] make: *** [prepare] Error 1
[00:02:48] make: *** [prepare] Error 1
[00:02:48] Makefile:81: recipe for target 'prepare' failed
[00:02:50]     Finished dev [unoptimized] target(s) in 0.26s
[00:02:50] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:02:50] 
[00:02:50] Caused by:
[00:02:50] Caused by:
[00:02:50]   the cargo feature `edition` requires a nightly version of Cargo, but this is the `beta` channel
[00:02:50] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:02:50] expected success, got: exit code: 101', build_helper/lib.rs:122:9
[00:02:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:02:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:02:50] make: *** [prepare] Error 1
[00:02:50] make: *** [prepare] Error 1
[00:02:50] Makefile:81: recipe for target 'prepare' failed
[00:02:54]     Finished dev [unoptimized] target(s) in 0.26s
[00:02:54] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:02:54] 
[00:02:54] Caused by:
[00:02:54] Caused by:
[00:02:54]   the cargo feature `edition` requires a nightly version of Cargo, but this is the `beta` channel
[00:02:54] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:02:54] expected success, got: exit code: 101', build_helper/lib.rs:122:9
[00:02:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:02:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:02:54] Build completed unsuccessfully in 0:00:00
[00:02:54] Makefile:81: recipe for target 'prepare' failed
[00:02:54] make: *** [prepare] Error 1
[00:02:58]     Finished dev [unoptimized] target(s) in 0.26s
[00:02:58] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:02:58] 
[00:02:58] Caused by:
[00:02:58] Caused by:
[00:02:58]   the cargo feature `edition` requires a nightly version of Cargo, but this is the `beta` channel
[00:02:58] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:02:58] expected success, got: exit code: 101', build_helper/lib.rs:122:9
[00:02:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:02:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:02:58] make: *** [prepare] Error 1
[00:02:58] make: *** [prepare] Error 1
[00:02:58] Makefile:81: recipe for target 'prepare' failed
[00:02:58] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:00a65510
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:12076930:start=1530402006694525758,finish=1530402006702631648,duration=8105890
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2e8443d2
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a4d7fb0
$ dmesg | grep -i kill
