plain
travis_time:end:03e96052:start=1554672247955813677,finish=1554672248859197165,duration=903383488
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:01:56] 
######################################################################## 100.0%
[00:01:56] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:56]     Updating crates.io index
[00:02:10] error: failed to select a version for `compiletest_rs`.
[00:02:10]     ... required by package `miri v0.1.0 (/checkout/src/tools/miri)`
[00:02:10] versions that meet the requirements `= 0.3.19` are: 0.3.19
[00:02:10] all possible versions conflict with previously selected packages.
[00:02:10] 
[00:02:10]   previously selected package `compiletest_rs v0.3.21`
[00:02:10]     ... which is depended on by `clippy v0.0.212 (/checkout/src/tools/clippy)`
---
[00:02:10] Makefile:69: recipe for target 'prepare' failed
[00:02:10] make: *** [prepare] Error 1
[00:02:11] Command failed. Attempt 2/5:
[00:02:12]     Updating crates.io index
[00:02:12] error: failed to select a version for `compiletest_rs`.
[00:02:12]     ... required by package `miri v0.1.0 (/checkout/src/tools/miri)`
[00:02:12] versions that meet the requirements `= 0.3.19` are: 0.3.19
[00:02:12] all possible versions conflict with previously selected packages.
[00:02:12] 
[00:02:12]   previously selected package `compiletest_rs v0.3.21`
[00:02:12]     ... which is depended on by `clippy v0.0.212 (/checkout/src/tools/clippy)`
---
[00:02:12] Makefile:69: recipe for target 'prepare' failed
[00:02:12] make: *** [prepare] Error 1
[00:02:14] Command failed. Attempt 3/5:
[00:02:14]     Updating crates.io index
[00:02:15] error: failed to select a version for `compiletest_rs`.
[00:02:15]     ... required by package `miri v0.1.0 (/checkout/src/tools/miri)`
[00:02:15] versions that meet the requirements `= 0.3.19` are: 0.3.19
[00:02:15] all possible versions conflict with previously selected packages.
[00:02:15] 
[00:02:15]   previously selected package `compiletest_rs v0.3.21`
[00:02:15]     ... which is depended on by `clippy v0.0.212 (/checkout/src/tools/clippy)`
---
[00:02:15] Makefile:69: recipe for target 'prepare' failed
[00:02:15] make: *** [prepare] Error 1
[00:02:18] Command failed. Attempt 4/5:
[00:02:18]     Updating crates.io index
[00:02:18] error: failed to select a version for `compiletest_rs`.
[00:02:18]     ... required by package `miri v0.1.0 (/checkout/src/tools/miri)`
[00:02:18] versions that meet the requirements `= 0.3.19` are: 0.3.19
[00:02:18] all possible versions conflict with previously selected packages.
[00:02:18] 
[00:02:18]   previously selected package `compiletest_rs v0.3.21`
[00:02:18]     ... which is depended on by `clippy v0.0.212 (/checkout/src/tools/clippy)`
---
[00:02:18] make: *** [prepare] Error 1
[00:02:18] Makefile:69: recipe for target 'prepare' failed
[00:02:22] Command failed. Attempt 5/5:
[00:02:22]     Updating crates.io index
[00:02:22] error: failed to select a version for `compiletest_rs`.
[00:02:22]     ... required by package `miri v0.1.0 (/checkout/src/tools/miri)`
[00:02:22] versions that meet the requirements `= 0.3.19` are: 0.3.19
[00:02:22] all possible versions conflict with previously selected packages.
[00:02:22] 
[00:02:22]   previously selected package `compiletest_rs v0.3.21`
[00:02:22]     ... which is depended on by `clippy v0.0.212 (/checkout/src/tools/clippy)`
---
travis_time:end:04b43d28:start=1554672405160209137,finish=1554672405165423240,duration=5214103
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b686a31
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06018450
travis_time:start:06018450
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:094fae92
$ dmesg | grep -i kill
