plain
travis_time:end:061e01c9:start=1557590287747837123,finish=1557590288541204134,duration=793367011
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
#################################################################         90.4%
######################################################################## 100.0%
[00:01:58] extracting /checkout/obj/build/cache/2019-04-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:58]     Updating crates.io index
[00:02:16] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:16] Build completed unsuccessfully in 0:00:35
[00:02:16] make: *** [prepare] Error 1
[00:02:16] Makefile:69: recipe for target 'prepare' failed
[00:02:17] Command failed. Attempt 2/5:
[00:02:17] Command failed. Attempt 2/5:
[00:02:17] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:17] Build completed unsuccessfully in 0:00:00
[00:02:17] make: *** [prepare] Error 1
[00:02:17] Makefile:69: recipe for target 'prepare' failed
[00:02:19] Command failed. Attempt 3/5:
[00:02:19] Command failed. Attempt 3/5:
[00:02:20] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:20] Build completed unsuccessfully in 0:00:00
[00:02:20] make: *** [prepare] Error 1
[00:02:20] Makefile:69: recipe for target 'prepare' failed
[00:02:23] Command failed. Attempt 4/5:
[00:02:23] Command failed. Attempt 4/5:
[00:02:23] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:23] Build completed unsuccessfully in 0:00:00
[00:02:23] make: *** [prepare] Error 1
[00:02:23] Makefile:69: recipe for target 'prepare' failed
[00:02:27] Command failed. Attempt 5/5:
[00:02:27] Command failed. Attempt 5/5:
[00:02:28] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:28] Build completed unsuccessfully in 0:00:00
[00:02:28] make: *** [prepare] Error 1
[00:02:28] Makefile:69: recipe for target 'prepare' failed
[00:02:28] The command has failed after 5 attempts.
---
travis_time:end:06ac519e:start=1557590449722222442,finish=1557590449728382782,duration=6160340
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:20346b13
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:356c33ec
travis_time:start:356c33ec
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03ca1954
$ dmesg | grep -i kill
