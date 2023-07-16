plain
travis_time:end:351d602c:start=1551620963756334604,finish=1551620964701404391,duration=945069787
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:01:57] 
######################################################################## 100.0%
[00:01:57] extracting /checkout/obj/build/cache/2019-02-27/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:57]     Updating crates.io index
[00:02:10]     Updating git repository `https://github.com/mati865/compiletest-rs.git`
[00:02:10] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:10] Build completed unsuccessfully in 0:00:25
[00:02:10] Makefile:70: recipe for target 'prepare' failed
[00:02:10] make: *** [prepare] Error 1
[00:02:11] Command failed. Attempt 2/5:
[00:02:11] Command failed. Attempt 2/5:
[00:02:11]     Updating git repository `https://github.com/mati865/compiletest-rs.git`
[00:02:12] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:12] Build completed unsuccessfully in 0:00:00
[00:02:12] Makefile:70: recipe for target 'prepare' failed
[00:02:12] make: *** [prepare] Error 1
[00:02:14] Command failed. Attempt 3/5:
[00:02:14] Command failed. Attempt 3/5:
[00:02:14]     Updating git repository `https://github.com/mati865/compiletest-rs.git`
[00:02:14] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:14] Build completed unsuccessfully in 0:00:00
[00:02:14] Makefile:70: recipe for target 'prepare' failed
[00:02:14] make: *** [prepare] Error 1
[00:02:17] Command failed. Attempt 4/5:
[00:02:17] Command failed. Attempt 4/5:
[00:02:17]     Updating git repository `https://github.com/mati865/compiletest-rs.git`
[00:02:18] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:18] Build completed unsuccessfully in 0:00:00
[00:02:18] Makefile:70: recipe for target 'prepare' failed
[00:02:18] make: *** [prepare] Error 1
[00:02:22] Command failed. Attempt 5/5:
[00:02:22] Command failed. Attempt 5/5:
[00:02:22]     Updating git repository `https://github.com/mati865/compiletest-rs.git`
[00:02:22] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:22] Build completed unsuccessfully in 0:00:00
[00:02:22] Makefile:70: recipe for target 'prepare' failed
[00:02:22] make: *** [prepare] Error 1
[00:02:22] The command has failed after 5 attempts.
---
travis_time:end:0ece04b6:start=1551621121816047113,finish=1551621121820694936,duration=4647823
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:022a5730
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:071f111a
travis_time:start:071f111a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:046dd03f
$ dmesg | grep -i kill
