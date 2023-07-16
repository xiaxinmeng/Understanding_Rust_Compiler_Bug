plain
travis_time:end:00c1c017:start=1556289763768723716,finish=1556289861012497593,duration=97243773877
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:01:23] 
######################################################################## 100.0%
[00:01:23] extracting /checkout/obj/build/cache/2019-04-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:23]     Updating crates.io index
[00:01:39] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:39] Build completed unsuccessfully in 0:00:29
[00:01:39] make: *** [prepare] Error 1
[00:01:39] Makefile:69: recipe for target 'prepare' failed
[00:01:40] Command failed. Attempt 2/5:
[00:01:40] Command failed. Attempt 2/5:
[00:01:40]     Updating crates.io index
[00:01:41] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:41] Build completed unsuccessfully in 0:00:00
[00:01:41] Makefile:69: recipe for target 'prepare' failed
[00:01:41] make: *** [prepare] Error 1
[00:01:43] Command failed. Attempt 3/5:
[00:01:43] Command failed. Attempt 3/5:
[00:01:43]     Updating crates.io index
[00:01:44] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:44] Build completed unsuccessfully in 0:00:00
[00:01:44] Makefile:69: recipe for target 'prepare' failed
[00:01:44] make: *** [prepare] Error 1
[00:01:47] Command failed. Attempt 4/5:
[00:01:47] Command failed. Attempt 4/5:
[00:01:47]     Updating crates.io index
[00:01:48] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:48] Build completed unsuccessfully in 0:00:00
[00:01:48] Makefile:69: recipe for target 'prepare' failed
[00:01:48] make: *** [prepare] Error 1
[00:01:52] Command failed. Attempt 5/5:
[00:01:52] Command failed. Attempt 5/5:
[00:01:52]     Updating crates.io index
[00:01:53] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:53] Build completed unsuccessfully in 0:00:00
[00:01:53] make: *** [prepare] Error 1
[00:01:53] Makefile:69: recipe for target 'prepare' failed
[00:01:53] The command has failed after 5 attempts.
---
travis_time:end:00c20184:start=1556289984648374468,finish=1556289984655055406,duration=6680938
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:009161f4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:116f1526
travis_time:start:116f1526
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2ddc3978
$ dmesg | grep -i kill
