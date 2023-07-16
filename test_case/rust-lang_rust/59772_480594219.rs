plain
travis_time:end:02934d0c:start=1554646378551612725,finish=1554646379467204067,duration=915591342
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
#####################################################                     74.9%
######################################################################## 100.0%
[00:02:05] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:05]     Updating crates.io index
[00:02:19] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:19] Build completed unsuccessfully in 0:00:36
[00:02:19] Makefile:69: recipe for target 'prepare' failed
[00:02:19] make: *** [prepare] Error 1
[00:02:20] Command failed. Attempt 2/5:
[00:02:20] Command failed. Attempt 2/5:
[00:02:20]     Updating crates.io index
[00:02:21] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:21] Build completed unsuccessfully in 0:00:00
[00:02:21] make: *** [prepare] Error 1
[00:02:21] Makefile:69: recipe for target 'prepare' failed
[00:02:23] Command failed. Attempt 3/5:
[00:02:23] Command failed. Attempt 3/5:
[00:02:23]     Updating crates.io index
[00:02:24] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:24] Build completed unsuccessfully in 0:00:00
[00:02:24] make: *** [prepare] Error 1
[00:02:24] Makefile:69: recipe for target 'prepare' failed
[00:02:27] Command failed. Attempt 4/5:
[00:02:27] Command failed. Attempt 4/5:
[00:02:27]     Updating crates.io index
[00:02:27] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:27] Build completed unsuccessfully in 0:00:00
[00:02:27] make: *** [prepare] Error 1
[00:02:27] Makefile:69: recipe for target 'prepare' failed
[00:02:31] Command failed. Attempt 5/5:
[00:02:31] Command failed. Attempt 5/5:
[00:02:31]     Updating crates.io index
[00:02:32] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:32] Build completed unsuccessfully in 0:00:00
[00:02:32] make: *** [prepare] Error 1
[00:02:32] Makefile:69: recipe for target 'prepare' failed
[00:02:32] The command has failed after 5 attempts.
---
travis_time:end:04b89b24:start=1554646545087089478,finish=1554646545092449162,duration=5359684
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1cc656a2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0055d82c
travis_time:start:0055d82c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1e923d24
$ dmesg | grep -i kill
