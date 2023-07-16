plain
travis_time:end:0007ce5d:start=1551465744845749876,finish=1551465747336525646,duration=2490775770
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:01:52] 
######################################################################## 100.0%
[00:01:52] extracting /checkout/obj/build/cache/2019-02-17/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:52]     Updating crates.io index
[00:02:05]     Updating git repository `https://github.com/mati865/compiletest-rs.git`
[00:02:05] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:05] Build completed unsuccessfully in 0:00:27
[00:02:05] make: *** [prepare] Error 1
[00:02:05] Makefile:70: recipe for target 'prepare' failed
[00:02:06] Command failed. Attempt 2/5:
[00:02:06] Command failed. Attempt 2/5:
[00:02:07]     Updating git repository `https://github.com/mati865/compiletest-rs.git`
[00:02:07] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:07] Build completed unsuccessfully in 0:00:00
[00:02:07] Makefile:70: recipe for target 'prepare' failed
[00:02:07] make: *** [prepare] Error 1
[00:02:09] Command failed. Attempt 3/5:
[00:02:09] Command failed. Attempt 3/5:
[00:02:09]     Updating git repository `https://github.com/mati865/compiletest-rs.git`
[00:02:09] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:09] Build completed unsuccessfully in 0:00:00
[00:02:09] make: *** [prepare] Error 1
[00:02:09] Makefile:70: recipe for target 'prepare' failed
[00:02:12] Command failed. Attempt 4/5:
[00:02:12] Command failed. Attempt 4/5:
[00:02:13]     Updating git repository `https://github.com/mati865/compiletest-rs.git`
[00:02:13] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:13] Build completed unsuccessfully in 0:00:00
[00:02:13] make: *** [prepare] Error 1
[00:02:13] Makefile:70: recipe for target 'prepare' failed
[00:02:17] Command failed. Attempt 5/5:
[00:02:17] Command failed. Attempt 5/5:
[00:02:17]     Updating git repository `https://github.com/mati865/compiletest-rs.git`
[00:02:18] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:18] Build completed unsuccessfully in 0:00:00
[00:02:18] Makefile:70: recipe for target 'prepare' failed
[00:02:18] The command has failed after 5 attempts.
[00:02:18] make: *** [prepare] Error 1
---
travis_time:end:0b4f6446:start=1551465900245205614,finish=1551465900250952207,duration=5746593
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:096fe925
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:018046bf
travis_time:start:018046bf
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10d798c0
$ dmesg | grep -i kill
