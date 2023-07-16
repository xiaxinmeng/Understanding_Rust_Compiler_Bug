plain
travis_time:end:08e6fd58:start=1549559635153498027,finish=1549559636187964442,duration=1034466415
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:18:12]    |
[00:18:12] note: lint level defined here
[00:18:12]   --> src/librustc_resolve/lib.rs:12:9
[00:18:12]    |
[00:18:12] 12 | #![deny(rust_2018_idioms)]
[00:18:12]    |         ^^^^^^^^^^^^^^^^
[00:18:12]    = note: #[deny(elided_lifetimes_in_paths)] implied by #[deny(rust_2018_idioms)]
[00:18:12] error: hidden lifetime parameters in types are deprecated
[00:18:12]    --> src/librustc_resolve/error_reporting.rs:238:17
[00:18:12]     |
[00:18:12] 238 |         source: PathSource,
---
[00:18:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:18:53] expected success, got: exit code: 101
[00:18:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:18:53] Build completed unsuccessfully in 0:15:22
[00:18:53] make: *** [all] Error 1
[00:18:53] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:30c4d084
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb  7 17:33:00 UTC 2019
---
travis_time:end:0f838aec:start=1549560781162139126,finish=1549560781170366755,duration=8227629
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d93a960
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:28270fb9
travis_time:start:28270fb9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:004a3d78
$ dmesg | grep -i kill
