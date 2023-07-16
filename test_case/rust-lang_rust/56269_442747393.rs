plain
travis_time:end:054d4258:start=1543479888773956289,finish=1543479944417783227,duration=55643826938
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:01:02] downloading https://static.rust-lang.org/dist/2018-10-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:02] 
######################################################################## 100.0%
[00:01:02] extracting /checkout/obj/build/cache/2018-10-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:02] error: failed to read `/rust-smallvec/Cargo.toml`
[00:01:02] Caused by:
[00:01:02]   No such file or directory (os error 2)
[00:01:02] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:02] Build completed unsuccessfully in 0:00:11
[00:01:02] Build completed unsuccessfully in 0:00:11
[00:01:02] make: *** [prepare] Error 1
[00:01:02] Makefile:81: recipe for target 'prepare' failed
[00:01:03] Command failed. Attempt 2/5:
[00:01:03] error: failed to read `/rust-smallvec/Cargo.toml`
[00:01:03] Caused by:
[00:01:03]   No such file or directory (os error 2)
[00:01:03] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:03] Build completed unsuccessfully in 0:00:00
[00:01:03] Build completed unsuccessfully in 0:00:00
[00:01:03] make: *** [prepare] Error 1
[00:01:03] Makefile:81: recipe for target 'prepare' failed
[00:01:05] Command failed. Attempt 3/5:
[00:01:05] error: failed to read `/rust-smallvec/Cargo.toml`
[00:01:05] Caused by:
[00:01:05]   No such file or directory (os error 2)
[00:01:05] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:05] Build completed unsuccessfully in 0:00:00
[00:01:05] Build completed unsuccessfully in 0:00:00
[00:01:05] make: *** [prepare] Error 1
[00:01:05] Makefile:81: recipe for target 'prepare' failed
[00:01:08] Command failed. Attempt 4/5:
[00:01:08] error: failed to read `/rust-smallvec/Cargo.toml`
[00:01:08] Caused by:
[00:01:08]   No such file or directory (os error 2)
[00:01:08] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:08] Build completed unsuccessfully in 0:00:00
[00:01:08] Build completed unsuccessfully in 0:00:00
[00:01:08] Makefile:81: recipe for target 'prepare' failed
[00:01:08] make: *** [prepare] Error 1
[00:01:12] Command failed. Attempt 5/5:
[00:01:12] error: failed to read `/rust-smallvec/Cargo.toml`
[00:01:12] Caused by:
[00:01:12]   No such file or directory (os error 2)
[00:01:12] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:12] Build completed unsuccessfully in 0:00:00
---
travis_time:end:1716e520:start=1543480026883858139,finish=1543480026888786949,duration=4928810
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04fa31c6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f78fd98
travis_time:start:0f78fd98
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10797e7f
$ dmesg | grep -i kill
