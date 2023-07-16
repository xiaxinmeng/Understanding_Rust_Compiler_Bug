plain
############################################                              61.8%
######################################################################## 100.0%
[00:02:16] extracting /checkout/obj/build/cache/2018-09-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:16]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:40] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:40] Build completed unsuccessfully in 0:00:55
[00:02:40] Makefile:81: recipe for target 'prepare' failed
[00:02:40] make: *** [prepare] Error 1
[00:02:41] Command failed. Attempt 2/5:
[00:02:41] Command failed. Attempt 2/5:
[00:02:41] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:41] Build completed unsuccessfully in 0:00:00
[00:02:41] make: *** [prepare] Error 1
[00:02:41] Makefile:81: recipe for target 'prepare' failed
[00:02:43] Command failed. Attempt 3/5:
[00:02:43] Command failed. Attempt 3/5:
[00:02:44] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:44] Build completed unsuccessfully in 0:00:00
[00:02:44] make: *** [prepare] Error 1
[00:02:44] Makefile:81: recipe for target 'prepare' failed
[00:02:47] Command failed. Attempt 4/5:
[00:02:47] Command failed. Attempt 4/5:
[00:02:47] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:47] Build completed unsuccessfully in 0:00:00
[00:02:47] Makefile:81: recipe for target 'prepare' failed
[00:02:47] make: *** [prepare] Error 1
[00:02:51] Command failed. Attempt 5/5:
[00:02:51] Command failed. Attempt 5/5:
[00:02:51] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:51] Build completed unsuccessfully in 0:00:00
[00:02:51] make: *** [prepare] Error 1
[00:02:51] Makefile:81: recipe for target 'prepare' failed
[00:02:51] The command has failed after 5 attempts.
---
travis_time:end:1b088b20:start=1537878486894660666,finish=1537878486900865480,duration=6204814
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1583cf80
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b83355a
travis_time:start:0b83355a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:029adc00
$ dmesg | grep -i kill
