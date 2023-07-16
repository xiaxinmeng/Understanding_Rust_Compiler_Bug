plain
travis_time:end:068f711c:start=1540872877271366948,finish=1540872878352787668,duration=1081420720
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
#################################################################         91.5%
######################################################################## 100.0%
[00:01:43] extracting /checkout/obj/build/cache/2018-10-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:43]     Updating crates.io index
[00:01:48] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:48] Build completed unsuccessfully in 0:00:17
[00:01:48] Makefile:81: recipe for target 'prepare' failed
[00:01:48] make: *** [prepare] Error 1
[00:01:49] Command failed. Attempt 2/5:
[00:01:49] Command failed. Attempt 2/5:
[00:01:49]     Updating crates.io index
[00:01:49] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:49] Build completed unsuccessfully in 0:00:00
[00:01:49] Makefile:81: recipe for target 'prepare' failed
[00:01:49] make: *** [prepare] Error 1
[00:01:51] Command failed. Attempt 3/5:
[00:01:51] Command failed. Attempt 3/5:
[00:01:51]     Updating crates.io index
[00:01:52] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:52] Build completed unsuccessfully in 0:00:00
[00:01:52] Makefile:81: recipe for target 'prepare' failed
[00:01:52] make: *** [prepare] Error 1
[00:01:55] Command failed. Attempt 4/5:
[00:01:55] Command failed. Attempt 4/5:
[00:01:55]     Updating crates.io index
[00:01:55] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:55] Build completed unsuccessfully in 0:00:00
[00:01:55] Makefile:81: recipe for target 'prepare' failed
[00:01:55] make: *** [prepare] Error 1
[00:01:59] Command failed. Attempt 5/5:
[00:01:59] Command failed. Attempt 5/5:
[00:01:59]     Updating crates.io index
[00:02:00] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:00] Build completed unsuccessfully in 0:00:00
[00:02:00] make: *** [prepare] Error 1
[00:02:00] Makefile:81: recipe for target 'prepare' failed
[00:02:00] The command has failed after 5 attempts.
---
travis_time:end:00facce2:start=1540873008740075546,finish=1540873008746912874,duration=6837328
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1c70c6c1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03b141f6
travis_time:start:03b141f6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0005e66a
$ dmesg | grep -i kill
