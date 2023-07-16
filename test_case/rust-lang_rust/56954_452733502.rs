plain
travis_time:end:0849832b:start=1547046992661865835,finish=1547046995241109792,duration=2579243957
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:01:59] 
######################################################################## 100.0%
[00:01:59] extracting /checkout/obj/build/cache/2019-01-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:59]     Updating crates.io index
[00:02:08] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:08] Build completed unsuccessfully in 0:00:21
[00:02:08] Makefile:71: recipe for target 'prepare' failed
[00:02:08] make: *** [prepare] Error 1
[00:02:09] Command failed. Attempt 2/5:
[00:02:09] Command failed. Attempt 2/5:
[00:02:09]     Updating crates.io index
[00:02:09] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:09] Build completed unsuccessfully in 0:00:00
[00:02:09] Makefile:71: recipe for target 'prepare' failed
[00:02:09] make: *** [prepare] Error 1
[00:02:11] Command failed. Attempt 3/5:
[00:02:11] Command failed. Attempt 3/5:
[00:02:11]     Updating crates.io index
[00:02:12] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:12] Build completed unsuccessfully in 0:00:00
[00:02:12] Makefile:71: recipe for target 'prepare' failed
[00:02:12] make: *** [prepare] Error 1
[00:02:15] Command failed. Attempt 4/5:
[00:02:15] Command failed. Attempt 4/5:
[00:02:15]     Updating crates.io index
[00:02:16] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:16] Build completed unsuccessfully in 0:00:01
[00:02:16] make: *** [prepare] Error 1
[00:02:16] Makefile:71: recipe for target 'prepare' failed
[00:02:20] Command failed. Attempt 5/5:
[00:02:20] Command failed. Attempt 5/5:
[00:02:20]     Updating crates.io index
[00:02:21] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:21] Build completed unsuccessfully in 0:00:00
[00:02:21] make: *** [prepare] Error 1
[00:02:21] Makefile:71: recipe for target 'prepare' failed
[00:02:21] The command has failed after 5 attempts.
---
travis_time:end:02204f58:start=1547047150540679824,finish=1547047150544866958,duration=4187134
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:29aa9860
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:056f30d6
travis_time:start:056f30d6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b2bf332
$ dmesg | grep -i kill
