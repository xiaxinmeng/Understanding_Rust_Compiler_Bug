plain
travis_time:end:05ef9bba:start=1554150426454880679,finish=1554150427488767870,duration=1033887191
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
################################                                          45.0%
######################################################################## 100.0%
[00:01:59] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:59]     Updating crates.io index
[00:02:14] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:14] Build completed unsuccessfully in 0:00:32
[00:02:14] make: *** [prepare] Error 1
[00:02:14] Makefile:69: recipe for target 'prepare' failed
[00:02:15] Command failed. Attempt 2/5:
[00:02:15] Command failed. Attempt 2/5:
[00:02:15] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:15] Build completed unsuccessfully in 0:00:00
[00:02:15] Makefile:69: recipe for target 'prepare' failed
[00:02:15] make: *** [prepare] Error 1
[00:02:17] Command failed. Attempt 3/5:
[00:02:17] Command failed. Attempt 3/5:
[00:02:18] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:18] Build completed unsuccessfully in 0:00:00
[00:02:18] make: *** [prepare] Error 1
[00:02:18] Makefile:69: recipe for target 'prepare' failed
[00:02:21] Command failed. Attempt 4/5:
[00:02:21] Command failed. Attempt 4/5:
[00:02:21] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:21] Build completed unsuccessfully in 0:00:00
[00:02:21] Makefile:69: recipe for target 'prepare' failed
[00:02:21] make: *** [prepare] Error 1
[00:02:25] Command failed. Attempt 5/5:
[00:02:25] Command failed. Attempt 5/5:
[00:02:25] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:25] Build completed unsuccessfully in 0:00:00
[00:02:25] make: *** [prepare] Error 1
[00:02:25] Makefile:69: recipe for target 'prepare' failed
[00:02:25] The command has failed after 5 attempts.
---
travis_time:end:0240a826:start=1554150586977850091,finish=1554150586983564460,duration=5714369
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0cd796a6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08463aea
travis_time:start:08463aea
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:045aecae
$ dmesg | grep -i kill
