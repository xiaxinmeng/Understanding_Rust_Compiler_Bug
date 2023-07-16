plain
travis_time:end:00eb855d:start=1546928251223379209,finish=1546928252137913966,duration=914534757
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
###################################################################       94.4%
######################################################################## 100.0%
[00:01:45] extracting /checkout/obj/build/cache/2019-01-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:45]     Updating crates.io index
[00:01:54] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:54] Build completed unsuccessfully in 0:00:25
[00:01:54] make: *** [prepare] Error 1
[00:01:54] Makefile:71: recipe for target 'prepare' failed
[00:01:55] Command failed. Attempt 2/5:
[00:01:55] Command failed. Attempt 2/5:
[00:01:55]     Updating crates.io index
[00:01:55] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:55] Build completed unsuccessfully in 0:00:00
[00:01:55] make: *** [prepare] Error 1
[00:01:55] Makefile:71: recipe for target 'prepare' failed
[00:01:57] Command failed. Attempt 3/5:
[00:01:57] Command failed. Attempt 3/5:
[00:01:57]     Updating crates.io index
[00:01:58] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:58] Build completed unsuccessfully in 0:00:00
[00:01:58] make: *** [prepare] Error 1
[00:01:58] Makefile:71: recipe for target 'prepare' failed
[00:02:01] Command failed. Attempt 4/5:
[00:02:01] Command failed. Attempt 4/5:
[00:02:01]     Updating crates.io index
[00:02:01] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:01] Build completed unsuccessfully in 0:00:00
[00:02:01] make: *** [prepare] Error 1
[00:02:01] Makefile:71: recipe for target 'prepare' failed
[00:02:05] Command failed. Attempt 5/5:
[00:02:05] Command failed. Attempt 5/5:
[00:02:05]     Updating crates.io index
[00:02:06] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:06] Build completed unsuccessfully in 0:00:00
[00:02:06] make: *** [prepare] Error 1
[00:02:06] Makefile:71: recipe for target 'prepare' failed
[00:02:06] The command has failed after 5 attempts.
---
travis_time:end:31e1e41d:start=1546928391610466888,finish=1546928391615697502,duration=5230614
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c355767
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0538ea90
travis_time:start:0538ea90
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:20720ebc
$ dmesg | grep -i kill
