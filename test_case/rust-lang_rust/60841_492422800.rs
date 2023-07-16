plain
travis_time:end:2d34fd00:start=1557870375433599379,finish=1557870376303972626,duration=870373247
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
####################                                                      29.1%
######################################################################## 100.0%
[00:01:56] extracting /checkout/obj/build/cache/2019-04-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:56]     Updating crates.io index
[00:02:13] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:13] Build completed unsuccessfully in 0:00:32
[00:02:13] make: *** [prepare] Error 1
[00:02:13] Makefile:69: recipe for target 'prepare' failed
[00:02:14] Command failed. Attempt 2/5:
[00:02:14] Command failed. Attempt 2/5:
[00:02:14]     Updating crates.io index
[00:02:15] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:15] Build completed unsuccessfully in 0:00:00
[00:02:15] Makefile:69: recipe for target 'prepare' failed
[00:02:15] make: *** [prepare] Error 1
[00:02:17] Command failed. Attempt 3/5:
[00:02:17] Command failed. Attempt 3/5:
[00:02:17]     Updating crates.io index
[00:02:17] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:17] Build completed unsuccessfully in 0:00:00
[00:02:17] make: *** [prepare] Error 1
[00:02:17] Makefile:69: recipe for target 'prepare' failed
[00:02:20] Command failed. Attempt 4/5:
[00:02:20] Command failed. Attempt 4/5:
[00:02:21]     Updating crates.io index
[00:02:21] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:21] Build completed unsuccessfully in 0:00:00
[00:02:21] make: *** [prepare] Error 1
[00:02:21] Makefile:69: recipe for target 'prepare' failed
[00:02:25] Command failed. Attempt 5/5:
[00:02:25] Command failed. Attempt 5/5:
[00:02:25]     Updating crates.io index
[00:02:26] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:26] Build completed unsuccessfully in 0:00:00
[00:02:26] make: *** [prepare] Error 1
[00:02:26] Makefile:69: recipe for target 'prepare' failed
[00:02:26] The command has failed after 5 attempts.
---
travis_time:end:35a9f1c8:start=1557870535799193293,finish=1557870535804576672,duration=5383379
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ea1335e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16c0ae80
travis_time:start:16c0ae80
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:062f90f8
$ dmesg | grep -i kill
