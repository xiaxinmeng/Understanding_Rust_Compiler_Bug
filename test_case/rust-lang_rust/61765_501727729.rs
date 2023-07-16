plain
travis_time:end:062dcbb5:start=1560435826489793496,finish=1560435950409238255,duration=123919444759
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
$ export TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate.git
$ export TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
$ export CI_JOB_NAME=$TRAVIS_JOB_NAME
$ export IMAGE=x86_64-gnu-llvm-6.0
$ export RUST_BACKTRACE=1
$ bash -c 'echo $BASH_VERSION'
---
[00:02:41]    Compiling toml v0.4.10
[00:02:41]    Compiling serde_json v1.0.33
[00:02:44]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:03:13]     Finished dev [unoptimized] target(s) in 1m 20s
[00:03:13] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "target `x86_64-unknown-linux-gnu` is not configured as a host, only as a target"', src/libcore/result.rs:999:5
[00:03:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:03:13] Build completed unsuccessfully in 0:01:35
[00:03:13] make: *** [prepare] Error 1
[00:03:13] Makefile:69: recipe for target 'prepare' failed
[00:03:13] Makefile:69: recipe for target 'prepare' failed
[00:03:14] Command failed. Attempt 2/5:
[00:03:14]     Finished dev [unoptimized] target(s) in 0.17s
[00:03:14] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "target `x86_64-unknown-linux-gnu` is not configured as a host, only as a target"', src/libcore/result.rs:999:5
[00:03:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:03:14] Build completed unsuccessfully in 0:00:00
[00:03:14] make: *** [prepare] Error 1
[00:03:14] Makefile:69: recipe for target 'prepare' failed
[00:03:14] Makefile:69: recipe for target 'prepare' failed
[00:03:16] Command failed. Attempt 3/5:
[00:03:16]     Finished dev [unoptimized] target(s) in 0.17s
[00:03:16] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "target `x86_64-unknown-linux-gnu` is not configured as a host, only as a target"', src/libcore/result.rs:999:5
[00:03:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:03:16] Build completed unsuccessfully in 0:00:00
[00:03:16] make: *** [prepare] Error 1
[00:03:16] Makefile:69: recipe for target 'prepare' failed
[00:03:16] Makefile:69: recipe for target 'prepare' failed
[00:03:19] Command failed. Attempt 4/5:
[00:03:19]     Finished dev [unoptimized] target(s) in 0.17s
[00:03:19] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "target `x86_64-unknown-linux-gnu` is not configured as a host, only as a target"', src/libcore/result.rs:999:5
[00:03:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:03:19] Build completed unsuccessfully in 0:00:00
[00:03:19] make: *** [prepare] Error 1
[00:03:19] Makefile:69: recipe for target 'prepare' failed
[00:03:19] Makefile:69: recipe for target 'prepare' failed
[00:03:23] Command failed. Attempt 5/5:
[00:03:24]     Finished dev [unoptimized] target(s) in 0.17s
[00:03:24] thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "target `x86_64-unknown-linux-gnu` is not configured as a host, only as a target"', src/libcore/result.rs:999:5
[00:03:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
[00:03:24] Build completed unsuccessfully in 0:00:00
[00:03:24] Makefile:69: recipe for target 'prepare' failed
[00:03:24] make: *** [prepare] Error 1
---
travis_time:end:02c365cf:start=1560436166191651302,finish=1560436166196689278,duration=5037976
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2da72fe3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1fc47876
travis_time:start:1fc47876
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01d767e0
$ dmesg | grep -i kill
