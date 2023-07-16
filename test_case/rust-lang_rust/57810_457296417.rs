plain
travis_time:end:102d1004:start=1548352758219525716,finish=1548352830064230300,duration=71844704584
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
#######################################################                   76.6%
######################################################################## 100.0%
[00:02:38] extracting /checkout/obj/build/cache/2019-01-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:38]     Updating crates.io index
[00:02:48] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:48] Build completed unsuccessfully in 0:00:25
[00:02:48] make: *** [prepare] Error 1
[00:02:48] Makefile:71: recipe for target 'prepare' failed
[00:02:49] Command failed. Attempt 2/5:
[00:02:49] Command failed. Attempt 2/5:
[00:02:50] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:50] Build completed unsuccessfully in 0:00:00
[00:02:50] make: *** [prepare] Error 1
[00:02:50] Makefile:71: recipe for target 'prepare' failed
[00:02:52] Command failed. Attempt 3/5:
[00:02:52] Command failed. Attempt 3/5:
[00:02:52] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:52] Build completed unsuccessfully in 0:00:00
[00:02:52] Makefile:71: recipe for target 'prepare' failed
[00:02:52] make: *** [prepare] Error 1
[00:02:55] Command failed. Attempt 4/5:
[00:02:55] Command failed. Attempt 4/5:
[00:02:55] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:55] Build completed unsuccessfully in 0:00:00
[00:02:55] make: *** [prepare] Error 1
[00:02:55] Makefile:71: recipe for target 'prepare' failed
[00:02:59] Command failed. Attempt 5/5:
[00:02:59] Command failed. Attempt 5/5:
[00:03:00] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:03:00] Build completed unsuccessfully in 0:00:00
[00:03:00] make: *** [prepare] Error 1
[00:03:00] Makefile:71: recipe for target 'prepare' failed
[00:03:00] The command has failed after 5 attempts.
---
travis_time:end:02b59480:start=1548353022460269047,finish=1548353022464989744,duration=4720697
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2806cf52
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1bd6558e
travis_time:start:1bd6558e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:185b0ba8
$ dmesg | grep -i kill
