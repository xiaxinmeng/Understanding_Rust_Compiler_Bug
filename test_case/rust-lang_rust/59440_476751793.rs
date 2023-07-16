plain
travis_time:end:10186ed2:start=1553619613005411484,finish=1553619615711995475,duration=2706583991
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
#################################################################         91.1%
######################################################################## 100.0%
[00:01:50] extracting /checkout/obj/build/cache/2019-02-27/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:51]     Updating crates.io index
[00:02:06]     Updating git repository `https://github.com/gnzlbg/libtest`
[00:02:08]   Downloaded cc v1.0.28
[00:02:08]   Downloaded cmake v0.1.33
[00:02:08]   Downloaded libc v0.2.50
[00:02:08]   Downloaded serde_json v1.0.33
---
[00:03:20]    Compiling syn v0.15.22
[00:03:26]    Compiling serde_json v1.0.33
[00:03:33]    Compiling serde_derive v1.0.81
[00:03:50]    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
[00:03:51] error: unused variable: `bad`
[00:03:51]   --> src/tools/tidy/src/extdeps.rs:12:27
[00:03:51]    |
[00:03:51] 12 | pub fn check(path: &Path, bad: &mut bool) {
[00:03:51]    |                           ^^^ help: consider prefixing with an underscore: `_bad`
[00:03:51]    = note: `-D unused-variables` implied by `-D warnings`
[00:03:51] 
[00:03:51] error: aborting due to previous error
[00:03:51] 
[00:03:51] 
[00:03:51] error: Could not compile `tidy`.
[00:03:51] To learn more, run the command again with --verbose.
[00:03:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tidy/Cargo.toml" "--message-format" "json"
[00:03:51] expected success, got: exit code: 101
[00:03:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:51] Build completed unsuccessfully in 0:00:37
[00:03:51] make: *** [tidy] Error 1
[00:03:51] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:036fb870
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 26 17:04:17 UTC 2019
---
travis_time:end:21ffacef:start=1553619857919977850,finish=1553619857924343649,duration=4365799
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0fdf173c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:374d5a10
travis_time:start:374d5a10
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:160e53ea
$ dmesg | grep -i kill
