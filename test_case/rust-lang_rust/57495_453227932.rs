plain
travis_time:end:13daa8e1:start=1547149245340135286,finish=1547149248225762036,duration=2885626750
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:01:38] downloading https://static.rust-lang.org/dist/2019-01-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:39] 
######################################################################## 100.0%
[00:01:39] extracting /checkout/obj/build/cache/2019-01-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:39] warning: /checkout/src/tools/rustbook/Cargo.toml: the cargo feature `rename-dependency` is now stable and is no longer necessary to be listed in the manifest
[00:01:39]     Updating crates.io index
[00:01:48] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:48] Build completed unsuccessfully in 0:00:22
[00:01:48] Makefile:71: recipe for target 'prepare' failed
[00:01:48] make: *** [prepare] Error 1
[00:01:49] Command failed. Attempt 2/5:
[00:01:49] Command failed. Attempt 2/5:
[00:01:49] warning: /checkout/src/tools/rustbook/Cargo.toml: the cargo feature `rename-dependency` is now stable and is no longer necessary to be listed in the manifest
[00:01:49] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:49] Build completed unsuccessfully in 0:00:00
[00:01:49] Makefile:71: recipe for target 'prepare' failed
[00:01:49] make: *** [prepare] Error 1
[00:01:51] Command failed. Attempt 3/5:
[00:01:51] Command failed. Attempt 3/5:
[00:01:51] warning: /checkout/src/tools/rustbook/Cargo.toml: the cargo feature `rename-dependency` is now stable and is no longer necessary to be listed in the manifest
[00:01:52] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:52] Build completed unsuccessfully in 0:00:00
[00:01:52] make: *** [prepare] Error 1
[00:01:52] Makefile:71: recipe for target 'prepare' failed
[00:01:55] Command failed. Attempt 4/5:
[00:01:55] Command failed. Attempt 4/5:
[00:01:55] warning: /checkout/src/tools/rustbook/Cargo.toml: the cargo feature `rename-dependency` is now stable and is no longer necessary to be listed in the manifest
[00:01:55] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:55] Build completed unsuccessfully in 0:00:00
[00:01:55] make: *** [prepare] Error 1
[00:01:55] Makefile:71: recipe for target 'prepare' failed
[00:01:59] Command failed. Attempt 5/5:
[00:01:59] Command failed. Attempt 5/5:
[00:01:59] warning: /checkout/src/tools/rustbook/Cargo.toml: the cargo feature `rename-dependency` is now stable and is no longer necessary to be listed in the manifest
[00:01:59] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:59] Build completed unsuccessfully in 0:00:00
[00:01:59] make: *** [prepare] Error 1
[00:01:59] Makefile:71: recipe for target 'prepare' failed
[00:01:59] The command has failed after 5 attempts.
---
travis_time:end:091de74f:start=1547149382103166182,finish=1547149382108608360,duration=5442178
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2912f45c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:20e4d594
travis_time:start:20e4d594
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:100d9a48
$ dmesg | grep -i kill
