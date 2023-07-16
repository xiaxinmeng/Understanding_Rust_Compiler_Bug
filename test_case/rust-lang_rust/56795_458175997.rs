plain
travis_time:end:34100700:start=1548689157290046396,finish=1548689160027264021,duration=2737217625
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:01:36] 
######################################################################## 100.0%
[00:01:36] extracting /checkout/obj/build/cache/2019-01-18/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:37]     Updating crates.io index
[00:01:47] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:47] Build completed unsuccessfully in 0:00:23
[00:01:47] make: *** [prepare] Error 1
[00:01:47] Makefile:70: recipe for target 'prepare' failed
[00:01:48] Command failed. Attempt 2/5:
[00:01:48] Command failed. Attempt 2/5:
[00:01:48] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:48] Build completed unsuccessfully in 0:00:00
[00:01:48] make: *** [prepare] Error 1
[00:01:48] Makefile:70: recipe for target 'prepare' failed
[00:01:50] Command failed. Attempt 3/5:
[00:01:50] Command failed. Attempt 3/5:
[00:01:51] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:51] Build completed unsuccessfully in 0:00:00
[00:01:51] Makefile:70: recipe for target 'prepare' failed
[00:01:51] make: *** [prepare] Error 1
[00:01:54] Command failed. Attempt 4/5:
[00:01:54] Command failed. Attempt 4/5:
[00:01:54] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:54] Build completed unsuccessfully in 0:00:00
[00:01:54] make: *** [prepare] Error 1
[00:01:54] Makefile:70: recipe for target 'prepare' failed
[00:01:58] Command failed. Attempt 5/5:
[00:01:58] Command failed. Attempt 5/5:
[00:01:58] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:58] Build completed unsuccessfully in 0:00:00
[00:01:58] Makefile:70: recipe for target 'prepare' failed
[00:01:58] make: *** [prepare] Error 1
[00:01:58] The command has failed after 5 attempts.
---
travis_time:end:117b5e44:start=1548689292583454401,finish=1548689292588397507,duration=4943106
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ed9e820
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:085d6508
travis_time:start:085d6508
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04464afd
$ dmesg | grep -i kill
