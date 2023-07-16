plain
travis_time:end:0fc2e1ce:start=1557503874130649525,finish=1557503874881205789,duration=750556264
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
#####################                                                     30.3%
######################################################################## 100.0%
[00:02:02] extracting /checkout/obj/build/cache/2019-04-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:02]     Updating crates.io index
[00:02:24] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:24] Build completed unsuccessfully in 0:00:35
[00:02:24] make: *** [prepare] Error 1
[00:02:24] Makefile:69: recipe for target 'prepare' failed
[00:02:25] Command failed. Attempt 2/5:
[00:02:25] Command failed. Attempt 2/5:
[00:02:26] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:26] Build completed unsuccessfully in 0:00:00
[00:02:26] Makefile:69: recipe for target 'prepare' failed
[00:02:26] make: *** [prepare] Error 1
[00:02:28] Command failed. Attempt 3/5:
[00:02:28] Command failed. Attempt 3/5:
[00:02:28] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:28] Build completed unsuccessfully in 0:00:00
[00:02:28] make: *** [prepare] Error 1
[00:02:28] Makefile:69: recipe for target 'prepare' failed
[00:02:31] Command failed. Attempt 4/5:
[00:02:31] Command failed. Attempt 4/5:
[00:02:31] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:31] Build completed unsuccessfully in 0:00:00
[00:02:31] make: *** [prepare] Error 1
[00:02:31] Makefile:69: recipe for target 'prepare' failed
[00:02:35] Command failed. Attempt 5/5:
[00:02:35] Command failed. Attempt 5/5:
[00:02:36] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:36] Build completed unsuccessfully in 0:00:00
[00:02:36] make: *** [prepare] Error 1
[00:02:36] Makefile:69: recipe for target 'prepare' failed
[00:02:36] The command has failed after 5 attempts.
---
travis_time:end:1e91887c:start=1557504042575094848,finish=1557504042580371177,duration=5276329
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:017e3050
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07c01c56
travis_time:start:07c01c56
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01753afe
$ dmesg | grep -i kill
