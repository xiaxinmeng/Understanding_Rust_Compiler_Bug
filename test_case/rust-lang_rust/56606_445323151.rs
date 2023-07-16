plain
travis_time:end:19b2420d:start=1544206635563241032,finish=1544206637803020042,duration=2239779010
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:03:01]    Compiling syn v0.14.9
[00:03:07]    Compiling serde_json v1.0.31
[00:03:17]    Compiling serde_derive v1.0.75
[00:03:40]    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
[00:03:40] error: unused variable: `source`
[00:03:40]   --> src/tools/tidy/src/extdeps.rs:40:13
[00:03:40]    |
[00:03:40] 40 |         let source = line.splitn(2, '=').nth(1).unwrap().trim();
[00:03:40]    |             ^^^^^^ help: consider using `_source` instead
[00:03:40]    = note: `-D unused-variables` implied by `-D warnings`
[00:03:40] 
[00:03:40] error: unused variable: `bad`
[00:03:40] error: unused variable: `bad`
[00:03:40]   --> src/tools/tidy/src/extdeps.rs:23:27
[00:03:40]    |
[00:03:40] 23 | pub fn check(path: &Path, bad: &mut bool) {
[00:03:40]    |                           ^^^ help: consider using `_bad` instead
[00:03:40] error: aborting due to 2 previous errors
[00:03:40] 
[00:03:40] 
[00:03:40] error: Could not compile `tidy`.
[00:03:40] To learn more, run the command again with --verbose.
[00:03:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tidy/Cargo.toml" "--message-format" "json"
[00:03:40] expected success, got: exit code: 101
[00:03:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:40] Build completed unsuccessfully in 0:00:49
[00:03:40] Makefile:79: recipe for target 'tidy' failed
[00:03:40] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1fa3eb1a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec  7 18:21:07 UTC 2018
---
travis_time:end:1f40cfca:start=1544206867604731648,finish=1544206867610102843,duration=5371195
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:029a13f8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0dae151d
travis_time:start:0dae151d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03eb08d6
$ dmesg | grep -i kill
