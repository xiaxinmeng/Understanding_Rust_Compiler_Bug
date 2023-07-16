plain
[00:01:57] 
######################################################################## 100.0%
[00:01:58] extracting /checkout/obj/build/cache/2019-05-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:58]     Updating crates.io index
[00:02:16] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:16] Build completed unsuccessfully in 0:00:31
[00:02:16] Makefile:69: recipe for target 'prepare' failed
[00:02:16] make: *** [prepare] Error 1
[00:02:17] Command failed. Attempt 2/5:
[00:02:17] Command failed. Attempt 2/5:
[00:02:17]     Updating crates.io index
[00:02:17] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:17] Build completed unsuccessfully in 0:00:00
[00:02:17] Makefile:69: recipe for target 'prepare' failed
[00:02:17] make: *** [prepare] Error 1
[00:02:19] Command failed. Attempt 3/5:
[00:02:19] Command failed. Attempt 3/5:
[00:02:19]     Updating crates.io index
[00:02:20] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:20] Build completed unsuccessfully in 0:00:00
[00:02:20] Makefile:69: recipe for target 'prepare' failed
[00:02:20] make: *** [prepare] Error 1
[00:02:23] Command failed. Attempt 4/5:
[00:02:23] Command failed. Attempt 4/5:
[00:02:23]     Updating crates.io index
[00:02:23] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:23] Build completed unsuccessfully in 0:00:00
[00:02:23] make: *** [prepare] Error 1
[00:02:23] Makefile:69: recipe for target 'prepare' failed
[00:02:27] Command failed. Attempt 5/5:
[00:02:27] Command failed. Attempt 5/5:
[00:02:27]     Updating crates.io index
[00:02:28] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:28] Build completed unsuccessfully in 0:00:00
[00:02:28] Makefile:69: recipe for target 'prepare' failed
[00:02:28] The command has failed after 5 attempts.
[00:02:28] make: *** [prepare] Error 1
---
travis_time:end:35a6dc38:start=1559507309497875640,finish=1559507309505057004,duration=7181364
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0525f7e0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03501ad8
travis_time:start:03501ad8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00923080
$ dmesg | grep -i kill
