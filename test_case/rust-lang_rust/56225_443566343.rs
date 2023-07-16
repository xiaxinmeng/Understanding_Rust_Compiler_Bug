plain
travis_time:end:057c89c0:start=1543802235025400858,finish=1543802237369269054,duration=2343868196
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
######################################################################## 100.0%
[00:01:15] extracting /checkout/obj/build/cache/2018-10-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:15]     Updating crates.io index
[00:01:21]     Updating git repository `https://github.com/rust-lang-nursery/rust-clippy`
[00:01:23] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:23] Build completed unsuccessfully in 0:00:30
[00:01:23] Makefile:81: recipe for target 'prepare' failed
[00:01:23] make: *** [prepare] Error 1
[00:01:24] Command failed. Attempt 2/5:
[00:01:24] Command failed. Attempt 2/5:
[00:01:24]     Updating crates.io index
[00:01:25]     Updating git repository `https://github.com/rust-lang-nursery/rust-clippy`
[00:01:25] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:25] Build completed unsuccessfully in 0:00:00
[00:01:25] Makefile:81: recipe for target 'prepare' failed
[00:01:25] make: *** [prepare] Error 1
[00:01:27] Command failed. Attempt 3/5:
[00:01:27] Command failed. Attempt 3/5:
[00:01:27]     Updating crates.io index
[00:01:28]     Updating git repository `https://github.com/rust-lang-nursery/rust-clippy`
[00:01:28] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:28] Build completed unsuccessfully in 0:00:00
[00:01:28] Makefile:81: recipe for target 'prepare' failed
[00:01:28] make: *** [prepare] Error 1
[00:01:31] Command failed. Attempt 4/5:
[00:01:31] Command failed. Attempt 4/5:
[00:01:31]     Updating crates.io index
[00:01:32]     Updating git repository `https://github.com/rust-lang-nursery/rust-clippy`
[00:01:32] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:32] Build completed unsuccessfully in 0:00:00
[00:01:32] make: *** [prepare] Error 1
[00:01:32] Makefile:81: recipe for target 'prepare' failed
[00:01:36] Command failed. Attempt 5/5:
[00:01:36] Command failed. Attempt 5/5:
[00:01:36]     Updating crates.io index
[00:01:37]     Updating git repository `https://github.com/rust-lang-nursery/rust-clippy`
[00:01:37] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:37] Build completed unsuccessfully in 0:00:01
[00:01:37] Makefile:81: recipe for target 'prepare' failed
[00:01:37] make: *** [prepare] Error 1
[00:01:37] The command has failed after 5 attempts.
---
travis_time:end:080a806e:start=1543802344358068072,finish=1543802344362997033,duration=4928961
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d5b9c20
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:17e3af97
travis_time:start:17e3af97
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1443e506
$ dmesg | grep -i kill
