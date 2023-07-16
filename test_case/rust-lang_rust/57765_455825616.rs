plain
travis_time:end:076df0b6:start=1547940448615658111,finish=1547940451602466492,duration=2986808381
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:47]    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
[00:03:47] error: use of deprecated item 'core::str::<impl str>::trim_left_matches': superseded by `trim_start_matches`
[00:03:47]    --> src/tools/tidy/src/features.rs:191:61
[00:03:47]     |
[00:03:47] 191 |             let level = match parts.next().map(|l| l.trim().trim_left_matches('(')) {
[00:03:47]     |
[00:03:47]     = note: `-D deprecated` implied by `-D warnings`
[00:03:47] 
[00:03:47] error: aborting due to previous error
[00:03:47] error: aborting due to previous error
[00:03:47] 
[00:03:47] error: Could not compile `tidy`.
[00:03:47] To learn more, run the command again with --verbose.
[00:03:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tidy/Cargo.toml" "--message-format" "json"
[00:03:47] expected success, got: exit code: 101
[00:03:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:47] Build completed unsuccessfully in 0:00:36
[00:03:47] Makefile:69: recipe for target 'tidy' failed
[00:03:47] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03b3e307
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan 19 23:31:30 UTC 2019
---
travis_time:end:0a47d8b5:start=1547940691336788773,finish=1547940691340958475,duration=4169702
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0232cacf
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06d2bfee
travis_time:start:06d2bfee
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02d9e7ae
$ dmesg | grep -i kill
