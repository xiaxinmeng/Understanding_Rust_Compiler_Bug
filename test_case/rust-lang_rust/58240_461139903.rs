plain
travis_time:end:0fd86466:start=1549478417503146351,finish=1549478418539391927,duration=1036245576
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:06:05]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:06] error: unused extern crate
[00:06:06]   --> src/librustc_errors/lib.rs:13:1
[00:06:06]    |
[00:06:06] 12 | / #[cfg(unix)]
[00:06:06] 13 | | extern crate libc;
[00:06:06]    | | ^^^^^^^^^^^^^^^^^-
[00:06:06]    | |__________________|
[00:06:06]    |
[00:06:06] note: lint level defined here
[00:06:06]   --> src/librustc_errors/lib.rs:10:9
[00:06:06]    |
[00:06:06]    |
[00:06:06] 10 | #![deny(rust_2018_idioms)]
[00:06:06]    |         ^^^^^^^^^^^^^^^^
[00:06:06]    = note: #[deny(unused_extern_crates)] implied by #[deny(rust_2018_idioms)]
[00:06:06] error: aborting due to previous error
[00:06:06] 
[00:06:06] error: Could not compile `rustc_errors`.
[00:06:06] warning: build failed, waiting for other jobs to finish...
[00:06:06] warning: build failed, waiting for other jobs to finish...
[00:06:13] error: build failed
[00:06:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:13] expected success, got: exit code: 101
[00:06:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:13] Build completed unsuccessfully in 0:02:13
[00:06:13] Makefile:18: recipe for target 'all' failed
[00:06:13] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c235912
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb  6 18:46:43 UTC 2019
---
travis_time:end:2c7367ab:start=1549478804023765604,finish=1549478804028794965,duration=5029361
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:31b26d78
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1e4445e0
travis_time:start:1e4445e0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/as
