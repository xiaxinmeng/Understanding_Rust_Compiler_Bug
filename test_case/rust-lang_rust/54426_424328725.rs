plain
#########################################                                 57.9%
######################################################################## 100.0%
[00:02:15] extracting /checkout/obj/build/cache/2018-09-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:15]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:39] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:39] Build completed unsuccessfully in 0:00:56
[00:02:39] Makefile:81: recipe for target 'prepare' failed
[00:02:39] make: *** [prepare] Error 1
[00:02:40] Command failed. Attempt 2/5:
[00:02:40] Command failed. Attempt 2/5:
[00:02:41] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:41] Build completed unsuccessfully in 0:00:00
[00:02:41] Makefile:81: recipe for target 'prepare' failed
[00:02:41] make: *** [prepare] Error 1
[00:02:43] Command failed. Attempt 3/5:
[00:02:43] Command failed. Attempt 3/5:
[00:02:43] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:43] Build completed unsuccessfully in 0:00:00
[00:02:43] Makefile:81: recipe for target 'prepare' failed
[00:02:43] make: *** [prepare] Error 1
[00:02:46] Command failed. Attempt 4/5:
[00:02:46] Command failed. Attempt 4/5:
[00:02:46] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:46] Build completed unsuccessfully in 0:00:00
[00:02:46] Makefile:81: recipe for target 'prepare' failed
[00:02:46] make: *** [prepare] Error 1
[00:02:50] Command failed. Attempt 5/5:
[00:02:50] Command failed. Attempt 5/5:
[00:02:50] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:50] Build completed unsuccessfully in 0:00:00
[00:02:50] Makefile:81: recipe for target 'prepare' failed
[00:02:50] make: *** [prepare] Error 1
[00:02:50] The command has failed after 5 attempts.
---
travis_time:end:02f80305:start=1537879553500602676,finish=1537879553504911071,duration=4308395
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:216350f7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02ddecc4
travis_time:start:02ddecc4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a873f36
$ dmesg | grep -i kill
