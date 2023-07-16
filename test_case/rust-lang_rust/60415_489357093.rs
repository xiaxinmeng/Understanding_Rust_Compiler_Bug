plain
travis_time:end:0bc7439e:start=1556997288580458852,finish=1556997290710627084,duration=2130168232
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
#########################                                                 35.3%
######################################################################## 100.0%
[00:01:34] extracting /checkout/obj/build/cache/2019-04-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:34]     Updating crates.io index
[00:01:50] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:50] Build completed unsuccessfully in 0:00:38
[00:01:50] Makefile:69: recipe for target 'prepare' failed
[00:01:50] make: *** [prepare] Error 1
[00:01:51] Command failed. Attempt 2/5:
[00:01:51] Command failed. Attempt 2/5:
[00:01:52] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:52] Build completed unsuccessfully in 0:00:00
[00:01:52] make: *** [prepare] Error 1
[00:01:52] Makefile:69: recipe for target 'prepare' failed
[00:01:54] Command failed. Attempt 3/5:
[00:01:54] Command failed. Attempt 3/5:
[00:01:54] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:54] Build completed unsuccessfully in 0:00:00
[00:01:54] make: *** [prepare] Error 1
[00:01:54] Makefile:69: recipe for target 'prepare' failed
[00:01:57] Command failed. Attempt 4/5:
[00:01:57] Command failed. Attempt 4/5:
[00:01:58] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:58] Build completed unsuccessfully in 0:00:00
[00:01:58] Makefile:69: recipe for target 'prepare' failed
[00:01:58] make: *** [prepare] Error 1
[00:02:02] Command failed. Attempt 5/5:
[00:02:02] Command failed. Attempt 5/5:
[00:02:02] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:02] Build completed unsuccessfully in 0:00:00
[00:02:02] Makefile:69: recipe for target 'prepare' failed
[00:02:02] make: *** [prepare] Error 1
[00:02:02] The command has failed after 5 attempts.
---
travis_time:end:03a46d18:start=1556997426076778663,finish=1556997426081957990,duration=5179327
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0cc1e63a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a038bce
travis_time:start:1a038bce
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2437d7d6
$ dmesg | grep -i kill
