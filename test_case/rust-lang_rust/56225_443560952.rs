plain
travis_time:end:19f60d4a:start=1543799423844280916,finish=1543799426321662598,duration=2477381682
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
[00:00:59] extracting /checkout/obj/build/cache/2018-10-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:02]     Updating crates.io index
[00:01:08]     Updating git repository `https://github.com/rust-lang-nursery/rust-clippy`
[00:01:10] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:10] Build completed unsuccessfully in 0:00:30
[00:01:10] Makefile:81: recipe for target 'prepare' failed
[00:01:10] make: *** [prepare] Error 1
[00:01:11] Command failed. Attempt 2/5:
[00:01:11] Command failed. Attempt 2/5:
[00:01:11]     Updating crates.io index
[00:01:12]     Updating git repository `https://github.com/rust-lang-nursery/rust-clippy`
[00:01:12] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:12] Build completed unsuccessfully in 0:00:00
[00:01:12] Makefile:81: recipe for target 'prepare' failed
[00:01:12] make: *** [prepare] Error 1
[00:01:14] Command failed. Attempt 3/5:
[00:01:14] Command failed. Attempt 3/5:
[00:01:14]     Updating crates.io index
[00:01:14]     Updating git repository `https://github.com/rust-lang-nursery/rust-clippy`
[00:01:15] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:15] Build completed unsuccessfully in 0:00:00
[00:01:15] Makefile:81: recipe for target 'prepare' failed
[00:01:15] make: *** [prepare] Error 1
[00:01:18] Command failed. Attempt 4/5:
[00:01:18] Command failed. Attempt 4/5:
[00:01:18]     Updating crates.io index
[00:01:18]     Updating git repository `https://github.com/rust-lang-nursery/rust-clippy`
[00:01:18] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:18] Build completed unsuccessfully in 0:00:00
[00:01:18] make: *** [prepare] Error 1
[00:01:18] Makefile:81: recipe for target 'prepare' failed
[00:01:22] Command failed. Attempt 5/5:
[00:01:22] Command failed. Attempt 5/5:
[00:01:22]     Updating crates.io index
[00:01:23]     Updating git repository `https://github.com/rust-lang-nursery/rust-clippy`
[00:01:23] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:23] Build completed unsuccessfully in 0:00:00
[00:01:23] make: *** [prepare] Error 1
[00:01:23] Makefile:81: recipe for target 'prepare' failed
[00:01:23] The command has failed after 5 attempts.
---
travis_time:end:15c5a40e:start=1543799519637214440,finish=1543799519644174815,duration=6960375
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1cbd36b0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03836055
travis_time:start:03836055
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2375da4a
$ dmesg | grep -i kill
