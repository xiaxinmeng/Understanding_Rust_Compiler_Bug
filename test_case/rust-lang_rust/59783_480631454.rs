plain
###########################################################               83.1%
######################################################################## 100.0%
[00:01:59] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:59]     Updating crates.io index
[00:02:14] error: failed to select a version for `compiletest_rs`.
[00:02:14]     ... required by package `miri v0.1.0 (/checkout/src/tools/miri)`
[00:02:14] versions that meet the requirements `= 0.3.19` are: 0.3.19
[00:02:14] all possible versions conflict with previously selected packages.
[00:02:14] 
[00:02:14]   previously selected package `compiletest_rs v0.3.21`
[00:02:14]     ... which is depended on by `clippy v0.0.212 (/checkout/src/tools/clippy)`
---
[00:02:14] make: *** [prepare] Error 1
[00:02:14] Makefile:69: recipe for target 'prepare' failed
[00:02:15] Command failed. Attempt 2/5:
[00:02:15]     Updating crates.io index
[00:02:15] error: failed to select a version for `compiletest_rs`.
[00:02:15]     ... required by package `miri v0.1.0 (/checkout/src/tools/miri)`
[00:02:15] versions that meet the requirements `= 0.3.19` are: 0.3.19
[00:02:15] all possible versions conflict with previously selected packages.
[00:02:15] 
[00:02:15]   previously selected package `compiletest_rs v0.3.21`
[00:02:15]     ... which is depended on by `clippy v0.0.212 (/checkout/src/tools/clippy)`
---
[00:02:15] make: *** [prepare] Error 1
[00:02:15] Makefile:69: recipe for target 'prepare' failed
[00:02:17] Command failed. Attempt 3/5:
[00:02:17]     Updating crates.io index
[00:02:18] error: failed to select a version for `compiletest_rs`.
[00:02:18]     ... required by package `miri v0.1.0 (/checkout/src/tools/miri)`
[00:02:18] versions that meet the requirements `= 0.3.19` are: 0.3.19
[00:02:18] all possible versions conflict with previously selected packages.
[00:02:18] 
[00:02:18]   previously selected package `compiletest_rs v0.3.21`
[00:02:18]     ... which is depended on by `clippy v0.0.212 (/checkout/src/tools/clippy)`
---
[00:02:18] Makefile:69: recipe for target 'prepare' failed
[00:02:18] make: *** [prepare] Error 1
[00:02:21] Command failed. Attempt 4/5:
[00:02:21]     Updating crates.io index
[00:02:21] error: failed to select a version for `compiletest_rs`.
[00:02:21]     ... required by package `miri v0.1.0 (/checkout/src/tools/miri)`
[00:02:21] versions that meet the requirements `= 0.3.19` are: 0.3.19
[00:02:21] all possible versions conflict with previously selected packages.
[00:02:21] 
[00:02:21]   previously selected package `compiletest_rs v0.3.21`
[00:02:21]     ... which is depended on by `clippy v0.0.212 (/checkout/src/tools/clippy)`
---
[00:02:21] make: *** [prepare] Error 1
[00:02:21] Makefile:69: recipe for target 'prepare' failed
[00:02:25] Command failed. Attempt 5/5:
[00:02:25]     Updating crates.io index
[00:02:26] error: failed to select a version for `compiletest_rs`.
[00:02:26]     ... required by package `miri v0.1.0 (/checkout/src/tools/miri)`
[00:02:26] versions that meet the requirements `= 0.3.19` are: 0.3.19
[00:02:26] all possible versions conflict with previously selected packages.
[00:02:26] 
[00:02:26]   previously selected package `compiletest_rs v0.3.21`
[00:02:26]     ... which is depended on by `clippy v0.0.212 (/checkout/src/tools/clippy)`
---
travis_time:end:0c44ce40:start=1554672596360144788,finish=1554672596368494567,duration=8349779
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02553da4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1330b0ab
travis_time:start:1330b0ab
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:072bbc9e
$ dmesg | grep -i kill
