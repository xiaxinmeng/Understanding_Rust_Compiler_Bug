plain
travis_time:end:0641ddee:start=1548326123343679549,finish=1548326197820146930,duration=74476467381
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:01:24] 
######################################################################## 100.0%
[00:01:24] extracting /checkout/obj/build/cache/2019-01-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:24]     Updating crates.io index
[00:01:34] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:34] Build completed unsuccessfully in 0:00:23
[00:01:34] Makefile:71: recipe for target 'prepare' failed
[00:01:34] make: *** [prepare] Error 1
[00:01:35] Command failed. Attempt 2/5:
[00:01:35] Command failed. Attempt 2/5:
[00:01:36] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:36] Build completed unsuccessfully in 0:00:00
[00:01:36] Makefile:71: recipe for target 'prepare' failed
[00:01:36] make: *** [prepare] Error 1
[00:01:38] Command failed. Attempt 3/5:
[00:01:38] Command failed. Attempt 3/5:
[00:01:38] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:38] Build completed unsuccessfully in 0:00:00
[00:01:38] Makefile:71: recipe for target 'prepare' failed
[00:01:38] make: *** [prepare] Error 1
[00:01:41] Command failed. Attempt 4/5:
[00:01:41] Command failed. Attempt 4/5:
[00:01:41] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:41] Build completed unsuccessfully in 0:00:00
[00:01:41] Makefile:71: recipe for target 'prepare' failed
[00:01:41] make: *** [prepare] Error 1
[00:01:45] Command failed. Attempt 5/5:
[00:01:45] Command failed. Attempt 5/5:
[00:01:46] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:46] Build completed unsuccessfully in 0:00:00
[00:01:46] make: *** [prepare] Error 1
[00:01:46] Makefile:71: recipe for target 'prepare' failed
[00:01:46] The command has failed after 5 attempts.
---
travis_time:end:103f4119:start=1548326316952983366,finish=1548326316958397253,duration=5413887
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1dae1798
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:073076de
travis_time:start:073076de
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0098d276
$ dmesg | grep -i kill
