plain
[00:02:14] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:14] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:14] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:17]     Updating crates.io index
[00:02:22] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:22] Build completed unsuccessfully in 0:00:33
[00:02:22] make: *** [prepare] Error 1
[00:02:22] Makefile:81: recipe for target 'prepare' failed
[00:02:23] Command failed. Attempt 2/5:
[00:02:23] Command failed. Attempt 2/5:
[00:02:23] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:23] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:23] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:23] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:23] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:23] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:23] Build completed unsuccessfully in 0:00:00
[00:02:23] Makefile:81: recipe for target 'prepare' failed
[00:02:23] make: *** [prepare] Error 1
[00:02:25] Command failed. Attempt 3/5:
[00:02:25] Command failed. Attempt 3/5:
[00:02:25] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:25] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:25] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:25] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:25] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:26] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:26] Build completed unsuccessfully in 0:00:00
[00:02:26] Makefile:81: recipe for target 'prepare' failed
[00:02:26] make: *** [prepare] Error 1
[00:02:29] Command failed. Attempt 4/5:
[00:02:29] Command failed. Attempt 4/5:
[00:02:29] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:29] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:29] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:29] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:29] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:29] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:29] Build completed unsuccessfully in 0:00:00
[00:02:29] make: *** [prepare] Error 1
[00:02:29] Makefile:81: recipe for target 'prepare' failed
[00:02:33] Command failed. Attempt 5/5:
[00:02:33] Command failed. Attempt 5/5:
[00:02:33] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:33] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:33] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:33] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:33] warning: the cargo feature `edition` is now stable and is no longer necessary to be listed in the manifest
[00:02:33] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:33] Build completed unsuccessfully in 0:00:00
[00:02:33] make: *** [prepare] Error 1
[00:02:33] Makefile:81: recipe for target 'prepare' failed
[00:02:33] The command has failed after 5 attempts.
---
travis_time:end:1d655f8b:start=1538322640290258344,finish=1538322640297332736,duration=7074392
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:041f351a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d8d0610
travis_time:start:0d8d0610
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2a55cf1a
$ dmesg | grep -i kill
