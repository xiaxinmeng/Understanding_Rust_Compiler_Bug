plain
##########################                                                36.9%
######################################################################## 100.0%
[00:01:54] extracting /checkout/obj/build/cache/2018-09-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:54]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:18] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:18] Build completed unsuccessfully in 0:00:40
[00:02:18] make: *** [prepare] Error 1
[00:02:18] Makefile:81: recipe for target 'prepare' failed
[00:02:19] Command failed. Attempt 2/5:
[00:02:19] Command failed. Attempt 2/5:
[00:02:20] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:20] Build completed unsuccessfully in 0:00:00
[00:02:20] make: *** [prepare] Error 1
[00:02:20] Makefile:81: recipe for target 'prepare' failed
[00:02:22] Command failed. Attempt 3/5:
[00:02:22] Command failed. Attempt 3/5:
[00:02:22] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:22] Build completed unsuccessfully in 0:00:00
[00:02:22] make: *** [prepare] Error 1
[00:02:22] Makefile:81: recipe for target 'prepare' failed
[00:02:25] Command failed. Attempt 4/5:
[00:02:25] Command failed. Attempt 4/5:
[00:02:25] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:25] Build completed unsuccessfully in 0:00:00
[00:02:25] make: *** [prepare] Error 1
[00:02:25] Makefile:81: recipe for target 'prepare' failed
[00:02:29] Command failed. Attempt 5/5:
[00:02:29] Command failed. Attempt 5/5:
[00:02:30] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:30] Build completed unsuccessfully in 0:00:00
[00:02:30] make: *** [prepare] Error 1
[00:02:30] Makefile:81: recipe for target 'prepare' failed
[00:02:30] The command has failed after 5 attempts.
---
travis_time:end:22725a66:start=1537882223351525573,finish=1537882223358473768,duration=6948195
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:063d3483
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:031e52c3
travis_time:start:031e52c3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:127cceea
$ dmesg | grep -i kill
