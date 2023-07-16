plain
travis_time:end:02d99db8:start=1554061442619112664,finish=1554061443862257290,duration=1243144626
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
#####################################################                     73.8%
######################################################################## 100.0%
[00:02:05] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:05]     Updating crates.io index
[00:02:19]     Updating git repository `https://github.com/tkaitchuck/aHash.git`
[00:02:20] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:20] Build completed unsuccessfully in 0:00:36
[00:02:20] make: *** [prepare] Error 1
[00:02:20] Makefile:69: recipe for target 'prepare' failed
[00:02:21] Command failed. Attempt 2/5:
[00:02:21] Command failed. Attempt 2/5:
[00:02:21] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:21] Build completed unsuccessfully in 0:00:00
[00:02:21] make: *** [prepare] Error 1
[00:02:21] Makefile:69: recipe for target 'prepare' failed
[00:02:23] Command failed. Attempt 3/5:
[00:02:23] Command failed. Attempt 3/5:
[00:02:24] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:24] Build completed unsuccessfully in 0:00:00
[00:02:24] Makefile:69: recipe for target 'prepare' failed
[00:02:24] make: *** [prepare] Error 1
[00:02:27] Command failed. Attempt 4/5:
[00:02:27] Command failed. Attempt 4/5:
[00:02:27] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:27] Build completed unsuccessfully in 0:00:00
[00:02:27] Makefile:69: recipe for target 'prepare' failed
[00:02:27] make: *** [prepare] Error 1
[00:02:31] Command failed. Attempt 5/5:
[00:02:31] Command failed. Attempt 5/5:
[00:02:32] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:32] Build completed unsuccessfully in 0:00:00
[00:02:32] Makefile:69: recipe for target 'prepare' failed
[00:02:32] make: *** [prepare] Error 1
[00:02:32] The command has failed after 5 attempts.
---
travis_time:end:0e68cb39:start=1554061609162078819,finish=1554061609168031511,duration=5952692
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0bc046c5
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0dfc2980
travis_time:start:0dfc2980
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:181838ab
$ dmesg | grep -i kill
