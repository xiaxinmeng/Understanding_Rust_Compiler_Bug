plain
travis_time:end:01f02d68:start=1545235788330528876,finish=1545235843265671827,duration=54935142951
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:16]    |                   ^^^^^^ allocation not allowed in constant functions
[00:05:16]    |
[00:05:16]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:05:16] 
[00:05:16] error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
[00:05:16]   --> src/librustc_data_structures/sorted_map.rs:35:19
[00:05:16] 35 |             data: vec![]
[00:05:16]    |                   ^^^^^^
[00:05:16]    |
[00:05:16]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
---
[00:05:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:05:16] expected success, got: exit code: 101
[00:05:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:05:16] Build completed unsuccessfully in 0:02:00
[00:05:16] Makefile:28: recipe for target 'all' failed
[00:05:16] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01ab4770
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Dec 19 16:16:10 UTC 2018
---
travis_time:end:04163b64:start=1545236170886325549,finish=1545236170892816231,duration=6490682
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:035cd859
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1d614368
travis_time:start:1d614368
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00ecb710
$ dmesg | grep -i kill
