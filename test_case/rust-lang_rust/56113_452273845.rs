plain
travis_time:end:0a477740:start=1546948595132135541,finish=1546948597619655068,duration=2487519527
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
######################################################################## 100.0%
[00:01:27] extracting /checkout/obj/build/cache/2019-01-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:27]     Updating crates.io index
[00:01:38]     Updating git repository `https://github.com/rust-lang/rust-clippy`
[00:01:40] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:40] Build completed unsuccessfully in 0:00:25
[00:01:40] make: *** [prepare] Error 1
[00:01:40] Makefile:71: recipe for target 'prepare' failed
[00:01:41] Command failed. Attempt 2/5:
[00:01:41] Command failed. Attempt 2/5:
[00:01:41]     Updating git repository `https://github.com/rust-lang/rust-clippy`
[00:01:41] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:41] Build completed unsuccessfully in 0:00:00
[00:01:41] make: *** [prepare] Error 1
[00:01:41] Makefile:71: recipe for target 'prepare' failed
[00:01:43] Command failed. Attempt 3/5:
[00:01:43] Command failed. Attempt 3/5:
[00:01:44]     Updating git repository `https://github.com/rust-lang/rust-clippy`
[00:01:44] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:44] Build completed unsuccessfully in 0:00:00
[00:01:44] Makefile:71: recipe for target 'prepare' failed
[00:01:44] make: *** [prepare] Error 1
[00:01:47] Command failed. Attempt 4/5:
[00:01:47] Command failed. Attempt 4/5:
[00:01:47]     Updating git repository `https://github.com/rust-lang/rust-clippy`
[00:01:48] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:48] Build completed unsuccessfully in 0:00:00
[00:01:48] Makefile:71: recipe for target 'prepare' failed
[00:01:48] make: *** [prepare] Error 1
[00:01:52] Command failed. Attempt 5/5:
[00:01:52] Command failed. Attempt 5/5:
[00:01:52]     Updating git repository `https://github.com/rust-lang/rust-clippy`
[00:01:52] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:52] Build completed unsuccessfully in 0:00:00
[00:01:52] make: *** [prepare] Error 1
[00:01:52] Makefile:71: recipe for target 'prepare' failed
[00:01:52] The command has failed after 5 attempts.
---
travis_time:end:02e036f8:start=1546948723874704953,finish=1546948723881905944,duration=7200991
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05181190
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:106d4849
travis_time:start:106d4849
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ed3097e
$ dmesg | grep -i kill
