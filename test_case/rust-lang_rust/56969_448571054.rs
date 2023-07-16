plain
travis_time:end:002583c8:start=1545220725012526478,finish=1545220726029633601,duration=1017107123
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:23]    |                   ^^^^^^ allocation not allowed in constant functions
[00:05:23]    |
[00:05:23]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:05:23] 
[00:05:23] error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
[00:05:23]   --> src/librustc_data_structures/sorted_map.rs:35:19
[00:05:23] 35 |             data: vec![]
[00:05:23]    |                   ^^^^^^
[00:05:23]    |
[00:05:23]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
---
[00:05:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:05:24] expected success, got: exit code: 101
[00:05:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:05:24] Build completed unsuccessfully in 0:01:59
[00:05:24] make: *** [all] Error 1
[00:05:24] Makefile:28: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c39a6b0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Dec 19 12:04:18 UTC 2018
---
travis_time:end:0d502902:start=1545221059383467347,finish=1545221059388869534,duration=5402187
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0bb124fc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0070f886
travis_time:start:0070f886
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01c3157c
$ dmesg | grep -i kill
