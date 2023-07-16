plain
travis_time:end:03a931ac:start=1553635940783833375,finish=1553636015703039459,duration=74919206084
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:01:23] 
######################################################################## 100.0%
[00:01:24] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:24]     Updating crates.io index
[00:01:41] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:41] Build completed unsuccessfully in 0:00:30
[00:01:41] make: *** [prepare] Error 1
[00:01:41] Makefile:69: recipe for target 'prepare' failed
[00:01:42] Command failed. Attempt 2/5:
[00:01:42] Command failed. Attempt 2/5:
[00:01:43] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:43] Build completed unsuccessfully in 0:00:00
[00:01:43] Makefile:69: recipe for target 'prepare' failed
[00:01:43] make: *** [prepare] Error 1
[00:01:45] Command failed. Attempt 3/5:
[00:01:45] Command failed. Attempt 3/5:
[00:01:45] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:45] Build completed unsuccessfully in 0:00:00
[00:01:45] make: *** [prepare] Error 1
[00:01:45] Makefile:69: recipe for target 'prepare' failed
[00:01:48] Command failed. Attempt 4/5:
[00:01:48] Command failed. Attempt 4/5:
[00:01:48] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:48] Build completed unsuccessfully in 0:00:00
[00:01:48] make: *** [prepare] Error 1
[00:01:48] Makefile:69: recipe for target 'prepare' failed
[00:01:52] Command failed. Attempt 5/5:
[00:01:52] Command failed. Attempt 5/5:
[00:01:53] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:53] Build completed unsuccessfully in 0:00:00
[00:01:53] make: *** [prepare] Error 1
[00:01:53] Makefile:69: recipe for target 'prepare' failed
[00:01:53] The command has failed after 5 attempts.
---
travis_time:end:194bf8f0:start=1553636140644617158,finish=1553636140650910067,duration=6292909
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04829846
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:101ccf66
travis_time:start:101ccf66
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02c33262
$ dmesg | grep -i kill
