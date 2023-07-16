plain
###################################################################       94.2%
######################################################################## 100.0%
[00:02:16] extracting /checkout/obj/build/cache/2018-09-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:16]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:23] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:23] Build completed unsuccessfully in 0:00:36
[00:02:23] make: *** [prepare] Error 1
[00:02:23] Makefile:81: recipe for target 'prepare' failed
[00:02:24] Command failed. Attempt 2/5:
[00:02:24] Command failed. Attempt 2/5:
[00:02:24] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:24] Build completed unsuccessfully in 0:00:00
[00:02:24] make: *** [prepare] Error 1
[00:02:24] Makefile:81: recipe for target 'prepare' failed
[00:02:26] Command failed. Attempt 3/5:
[00:02:26] Command failed. Attempt 3/5:
[00:02:26] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:26] Build completed unsuccessfully in 0:00:00
[00:02:26] Makefile:81: recipe for target 'prepare' failed
[00:02:26] make: *** [prepare] Error 1
[00:02:29] Command failed. Attempt 4/5:
[00:02:29] Command failed. Attempt 4/5:
[00:02:30] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:30] Build completed unsuccessfully in 0:00:00
[00:02:30] Makefile:81: recipe for target 'prepare' failed
[00:02:30] make: *** [prepare] Error 1
[00:02:34] Command failed. Attempt 5/5:
[00:02:34] Command failed. Attempt 5/5:
[00:02:34] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:34] Build completed unsuccessfully in 0:00:00
[00:02:34] make: *** [prepare] Error 1
[00:02:34] Makefile:81: recipe for target 'prepare' failed
[00:02:34] The command has failed after 5 attempts.
---
travis_time:end:0462fc00:start=1537981701396573676,finish=1537981701401624023,duration=5050347
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11b1b073
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0373de1c
travis_time:start:0373de1c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1f1f88b0
$ dmesg | grep -i kill
