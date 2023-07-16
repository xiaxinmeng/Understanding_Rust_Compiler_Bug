plain
#####################################                                     52.1%
######################################################################## 100.0%
[00:02:02] extracting /checkout/obj/build/cache/2018-09-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:04]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:28] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:28] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:28] Makefile:81: recipe for target 'prepare' failed
[00:02:28] make: *** [prepare] Error 1
[00:02:29] Command failed. Attempt 2/5:
[00:02:29] Command failed. Attempt 2/5:
[00:02:31] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:31] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:31] make: *** [prepare] Error 1
[00:02:31] Makefile:81: recipe for target 'prepare' failed
[00:02:33] Command failed. Attempt 3/5:
[00:02:33] Command failed. Attempt 3/5:
[00:02:33] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:33] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:33] make: *** [prepare] Error 1
[00:02:33] Makefile:81: recipe for target 'prepare' failed
[00:02:36] Command failed. Attempt 4/5:
[00:02:36] Command failed. Attempt 4/5:
[00:02:36] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:36] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:36] Makefile:81: recipe for target 'prepare' failed
[00:02:36] make: *** [prepare] Error 1
[00:02:40] Command failed. Attempt 5/5:
[00:02:40] Command failed. Attempt 5/5:
[00:02:41] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:41] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:41] Makefile:81: recipe for target 'prepare' failed
[00:02:41] make: *** [prepare] Error 1
[00:02:41] The command has failed after 5 attempts.
travis_time:end:2bf35670:start=1537252588752718227,finish=1537252755206919928,duration=166454201701
---
travis_time:end:0463602c:start=1537252755615275698,finish=1537252755622805334,duration=7529636
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b4deb16
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2ed0a696
travis_time:start:2ed0a696
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00f01a33
$ dmesg | grep -i kill
