plain
travis_time:end:33f8958a:start=1544577176343681956,finish=1544577238683468649,duration=62339786693
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
############################################################              84.5%
######################################################################## 100.0%
[00:01:06] extracting /checkout/obj/build/cache/2018-10-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:06]     Updating crates.io index
[00:01:14] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:14] Build completed unsuccessfully in 0:00:20
[00:01:14] Makefile:81: recipe for target 'prepare' failed
[00:01:14] make: *** [prepare] Error 1
[00:01:15] Command failed. Attempt 2/5:
[00:01:15] Command failed. Attempt 2/5:
[00:01:15] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:15] Build completed unsuccessfully in 0:00:00
[00:01:15] Makefile:81: recipe for target 'prepare' failed
[00:01:15] make: *** [prepare] Error 1
[00:01:17] Command failed. Attempt 3/5:
[00:01:17] Command failed. Attempt 3/5:
[00:01:17] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:17] Build completed unsuccessfully in 0:00:00
[00:01:17] Makefile:81: recipe for target 'prepare' failed
[00:01:17] make: *** [prepare] Error 1
[00:01:20] Command failed. Attempt 4/5:
[00:01:20] Command failed. Attempt 4/5:
[00:01:21] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:21] Build completed unsuccessfully in 0:00:00
[00:01:21] Makefile:81: recipe for target 'prepare' failed
[00:01:21] make: *** [prepare] Error 1
[00:01:25] Command failed. Attempt 5/5:
[00:01:25] Command failed. Attempt 5/5:
[00:01:25] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:25] Build completed unsuccessfully in 0:00:00
[00:01:25] make: *** [prepare] Error 1
[00:01:25] Makefile:81: recipe for target 'prepare' failed
[00:01:25] The command has failed after 5 attempts.
---
travis_time:end:0c49a889:start=1544577334744179024,finish=1544577334752233677,duration=8054653
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:33e507e2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06d7eeec
travis_time:start:06d7eeec
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0af33ed2
$ dmesg | grep -i kill
