plain
travis_time:end:0ce0eb40:start=1545511078985212849,finish=1545511133064664832,duration=54079451983
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
#########################                                                 35.7%
######################################################################## 100.0%
[00:01:16] extracting /checkout/obj/build/cache/2018-12-09/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:16]     Updating crates.io index
[00:01:24] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:24] Build completed unsuccessfully in 0:00:19
[00:01:24] Makefile:81: recipe for target 'prepare' failed
[00:01:24] make: *** [prepare] Error 1
[00:01:25] Command failed. Attempt 2/5:
[00:01:25] Command failed. Attempt 2/5:
[00:01:25] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:25] Build completed unsuccessfully in 0:00:00
[00:01:25] make: *** [prepare] Error 1
[00:01:25] Makefile:81: recipe for target 'prepare' failed
[00:01:27] Command failed. Attempt 3/5:
[00:01:27] Command failed. Attempt 3/5:
[00:01:28] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:28] Build completed unsuccessfully in 0:00:00
[00:01:28] make: *** [prepare] Error 1
[00:01:28] Makefile:81: recipe for target 'prepare' failed
[00:01:31] Command failed. Attempt 4/5:
[00:01:31] Command failed. Attempt 4/5:
[00:01:31] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:31] Build completed unsuccessfully in 0:00:00
[00:01:31] make: *** [prepare] Error 1
[00:01:31] Makefile:81: recipe for target 'prepare' failed
[00:01:35] Command failed. Attempt 5/5:
[00:01:35] Command failed. Attempt 5/5:
[00:01:35] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:35] Build completed unsuccessfully in 0:00:00
[00:01:35] make: *** [prepare] Error 1
[00:01:35] Makefile:81: recipe for target 'prepare' failed
[00:01:35] The command has failed after 5 attempts.
---
travis_time:end:063952c8:start=1545511237902351042,finish=1545511237907228779,duration=4877737
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d97b95a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:290f2740
travis_time:start:290f2740
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:223099d2
$ dmesg | grep -i kill
