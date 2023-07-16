plain
travis_time:end:09c31340:start=1554230093841327556,finish=1554230096317889139,duration=2476561583
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:02:00] 
######################################################################## 100.0%
[00:02:00] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:00]     Updating crates.io index
[00:02:15] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:15] Build completed unsuccessfully in 0:00:31
[00:02:15] make: *** [prepare] Error 1
[00:02:15] Makefile:69: recipe for target 'prepare' failed
[00:02:16] Command failed. Attempt 2/5:
[00:02:16] Command failed. Attempt 2/5:
[00:02:16] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:16] Build completed unsuccessfully in 0:00:00
[00:02:16] Makefile:69: recipe for target 'prepare' failed
[00:02:16] make: *** [prepare] Error 1
[00:02:18] Command failed. Attempt 3/5:
[00:02:18] Command failed. Attempt 3/5:
[00:02:19] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:19] Build completed unsuccessfully in 0:00:00
[00:02:19] Makefile:69: recipe for target 'prepare' failed
[00:02:19] make: *** [prepare] Error 1
[00:02:22] Command failed. Attempt 4/5:
[00:02:22] Command failed. Attempt 4/5:
[00:02:22] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:22] Build completed unsuccessfully in 0:00:00
[00:02:22] make: *** [prepare] Error 1
[00:02:22] Makefile:69: recipe for target 'prepare' failed
[00:02:26] Command failed. Attempt 5/5:
[00:02:26] Command failed. Attempt 5/5:
[00:02:26] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:26] Build completed unsuccessfully in 0:00:00
[00:02:27] make: *** [prepare] Error 1
[00:02:27] Makefile:69: recipe for target 'prepare' failed
[00:02:27] The command has failed after 5 attempts.
---
travis_time:end:0a5a4668:start=1554230256764929768,finish=1554230256771327382,duration=6397614
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1c66074c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:090c841e
travis_time:start:090c841e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11d77aa8
$ dmesg | grep -i kill
