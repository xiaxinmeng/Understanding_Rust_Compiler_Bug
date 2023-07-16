plain
travis_time:end:0516e6f0:start=1557176987072050363,finish=1557176989299577100,duration=2227526737
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
######################################################################## 100.0%
[00:01:52] extracting /checkout/obj/build/cache/2019-04-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:52]     Updating git repository `https://github.com/redox-os/liblibc.git`
[00:01:54]     Updating crates.io index
[00:02:10] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:10] Build completed unsuccessfully in 0:00:32
[00:02:10] make: *** [prepare] Error 1
[00:02:10] Makefile:69: recipe for target 'prepare' failed
[00:02:11] Command failed. Attempt 2/5:
[00:02:11] Command failed. Attempt 2/5:
[00:02:12] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:12] Build completed unsuccessfully in 0:00:00
[00:02:12] make: *** [prepare] Error 1
[00:02:12] Makefile:69: recipe for target 'prepare' failed
[00:02:14] Command failed. Attempt 3/5:
[00:02:14] Command failed. Attempt 3/5:
[00:02:14] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:14] Build completed unsuccessfully in 0:00:00
[00:02:14] make: *** [prepare] Error 1
[00:02:14] Makefile:69: recipe for target 'prepare' failed
[00:02:17] Command failed. Attempt 4/5:
[00:02:17] Command failed. Attempt 4/5:
[00:02:18] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:18] Build completed unsuccessfully in 0:00:00
[00:02:18] make: *** [prepare] Error 1
[00:02:18] Makefile:69: recipe for target 'prepare' failed
[00:02:22] Command failed. Attempt 5/5:
[00:02:22] Command failed. Attempt 5/5:
[00:02:22] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:22] Build completed unsuccessfully in 0:00:00
[00:02:22] Makefile:69: recipe for target 'prepare' failed
[00:02:22] make: *** [prepare] Error 1
[00:02:22] The command has failed after 5 attempts.
---
travis_time:end:295e3f06:start=1557177143230315399,finish=1557177143235065182,duration=4749783
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2399a160
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2ac572d6
travis_time:start:2ac572d6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13c1cf40
$ dmesg | grep -i kill
