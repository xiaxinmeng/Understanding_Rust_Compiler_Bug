plain
################                                                          22.8%
######################################################################## 100.0%
[00:02:00] extracting /checkout/obj/build/cache/2018-09-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:02]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:27] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:27] Build completed unsuccessfully in 0:00:43
[00:02:27] Makefile:81: recipe for target 'prepare' failed
[00:02:27] make: *** [prepare] Error 1
[00:02:28] Command failed. Attempt 2/5:
[00:02:28] Command failed. Attempt 2/5:
[00:02:29] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:29] Build completed unsuccessfully in 0:00:00
[00:02:29] Makefile:81: recipe for target 'prepare' failed
[00:02:29] make: *** [prepare] Error 1
[00:02:31] Command failed. Attempt 3/5:
[00:02:31] Command failed. Attempt 3/5:
[00:02:31] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:31] Build completed unsuccessfully in 0:00:00
[00:02:31] Makefile:81: recipe for target 'prepare' failed
[00:02:31] make: *** [prepare] Error 1
[00:02:34] Command failed. Attempt 4/5:
[00:02:34] Command failed. Attempt 4/5:
[00:02:34] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:34] Build completed unsuccessfully in 0:00:00
[00:02:34] Makefile:81: recipe for target 'prepare' failed
[00:02:34] make: *** [prepare] Error 1
[00:02:38] Command failed. Attempt 5/5:
[00:02:38] Command failed. Attempt 5/5:
[00:02:40] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:40] Build completed unsuccessfully in 0:00:01
[00:02:40] make: *** [prepare] Error 1
[00:02:40] Makefile:81: recipe for target 'prepare' failed
[00:02:40] The command has failed after 5 attempts.
---
travis_time:end:04f68138:start=1537848763506278542,finish=1537848763512828847,duration=6550305
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04744b96
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1804147a
travis_time:start:1804147a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0dd3cfcf
$ dmesg | grep -i kill
