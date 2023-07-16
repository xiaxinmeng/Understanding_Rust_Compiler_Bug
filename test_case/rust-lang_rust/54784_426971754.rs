plain
######################################################################## 100.0%
[00:01:56] extracting /checkout/obj/build/cache/2018-09-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:56] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:01:56]     Updating crates.io index
[00:02:01] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:01] Build completed unsuccessfully in 0:00:17
[00:02:01] make: *** [prepare] Error 1
[00:02:01] Makefile:81: recipe for target 'prepare' failed
[00:02:02] Command failed. Attempt 2/5:
[00:02:02] Command failed. Attempt 2/5:
[00:02:02] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:02] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:02] Build completed unsuccessfully in 0:00:00
[00:02:02] make: *** [prepare] Error 1
[00:02:02] Makefile:81: recipe for target 'prepare' failed
[00:02:04] Command failed. Attempt 3/5:
[00:02:04] Command failed. Attempt 3/5:
[00:02:04] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:05] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:05] Build completed unsuccessfully in 0:00:00
[00:02:05] make: *** [prepare] Error 1
[00:02:05] Makefile:81: recipe for target 'prepare' failed
[00:02:08] Command failed. Attempt 4/5:
[00:02:08] Command failed. Attempt 4/5:
[00:02:08] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:08] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:08] Build completed unsuccessfully in 0:00:00
[00:02:08] make: *** [prepare] Error 1
[00:02:08] Makefile:81: recipe for target 'prepare' failed
[00:02:12] Command failed. Attempt 5/5:
[00:02:12] Command failed. Attempt 5/5:
[00:02:12] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:12] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:12] Build completed unsuccessfully in 0:00:00
[00:02:12] make: *** [prepare] Error 1
[00:02:12] Makefile:81: recipe for target 'prepare' failed
[00:02:12] The command has failed after 5 attempts.
---
travis_time:end:0db297d8:start=1538649924519494582,finish=1538649924525671831,duration=6177249
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:16d22e89
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03afb6b7
travis_time:start:03afb6b7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00143f70
$ dmesg | grep -i kill
