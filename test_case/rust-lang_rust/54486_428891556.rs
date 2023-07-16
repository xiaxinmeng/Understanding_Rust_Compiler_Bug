plain
######################################################################## 100.0%
[00:02:16] extracting /checkout/obj/build/cache/2018-09-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:16] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:17]     Updating crates.io index
[00:02:21] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:21] Build completed unsuccessfully in 0:00:34
[00:02:21] make: *** [prepare] Error 1
[00:02:21] Makefile:81: recipe for target 'prepare' failed
[00:02:22] Command failed. Attempt 2/5:
[00:02:22] Command failed. Attempt 2/5:
[00:02:22] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:22] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:22] Build completed unsuccessfully in 0:00:00
[00:02:22] make: *** [prepare] Error 1
[00:02:22] Makefile:81: recipe for target 'prepare' failed
[00:02:24] Command failed. Attempt 3/5:
[00:02:24] Command failed. Attempt 3/5:
[00:02:24] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:24] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:24] Build completed unsuccessfully in 0:00:00
[00:02:24] make: *** [prepare] Error 1
[00:02:24] Makefile:81: recipe for target 'prepare' failed
[00:02:27] Command failed. Attempt 4/5:
[00:02:27] Command failed. Attempt 4/5:
[00:02:28] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:28] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:28] Build completed unsuccessfully in 0:00:00
[00:02:28] make: *** [prepare] Error 1
[00:02:28] Makefile:81: recipe for target 'prepare' failed
[00:02:32] Command failed. Attempt 5/5:
[00:02:32] Command failed. Attempt 5/5:
[00:02:32] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:32] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:32] Build completed unsuccessfully in 0:00:00
[00:02:32] make: *** [prepare] Error 1
[00:02:32] Makefile:81: recipe for target 'prepare' failed
[00:02:32] The command has failed after 5 attempts.
---
travis_time:end:076218ba:start=1539251045610849976,finish=1539251045615404233,duration=4554257
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1209290c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0baf9bf4
travis_time:start:0baf9bf4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:113ab0d6
$ dmesg | grep -i kill
