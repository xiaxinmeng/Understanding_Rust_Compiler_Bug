plain
travis_time:end:129dad93:start=1560780075259020834,finish=1560780077811468223,duration=2552447389
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
####################                                                      28.2%
######################################################################## 100.0%
[00:03:37] extracting /checkout/obj/build/cache/2019-05-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:03:37]     Updating crates.io index
[00:04:34] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:04:34] Build completed unsuccessfully in 0:01:10
[00:04:34] Makefile:69: recipe for target 'prepare' failed
[00:04:34] make: *** [prepare] Error 1
[00:04:35] Command failed. Attempt 2/5:
[00:04:35] Command failed. Attempt 2/5:
[00:04:35]     Updating crates.io index
[00:04:38] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:04:38] Build completed unsuccessfully in 0:00:02
[00:04:38] make: *** [prepare] Error 1
[00:04:38] Makefile:69: recipe for target 'prepare' failed
[00:04:40] Command failed. Attempt 3/5:
[00:04:40] Command failed. Attempt 3/5:
[00:04:40]     Updating crates.io index
[00:04:41] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:04:41] Build completed unsuccessfully in 0:00:00
[00:04:41] make: *** [prepare] Error 1
[00:04:41] Makefile:69: recipe for target 'prepare' failed
[00:04:44] Command failed. Attempt 4/5:
[00:04:44] Command failed. Attempt 4/5:
[00:04:44]     Updating crates.io index
[00:04:44] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:04:44] Build completed unsuccessfully in 0:00:00
[00:04:44] Makefile:69: recipe for target 'prepare' failed
[00:04:44] make: *** [prepare] Error 1
[00:04:48] Command failed. Attempt 5/5:
[00:04:48] Command failed. Attempt 5/5:
[00:04:48]     Updating crates.io index
[00:04:49] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:04:49] Build completed unsuccessfully in 0:00:00
[00:04:49] make: *** [prepare] Error 1
[00:04:49] Makefile:69: recipe for target 'prepare' failed
[00:04:49] The command has failed after 5 attempts.
---
travis_time:end:03f52c23:start=1560780379890391000,finish=1560780379896687946,duration=6296946
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1543e950
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:272db648
travis_time:start:272db648
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:306fe304
$ dmesg | grep -i kill
