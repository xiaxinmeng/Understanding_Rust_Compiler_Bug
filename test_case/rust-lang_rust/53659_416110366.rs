plain
#######################################################################   99.7%
######################################################################## 100.0%
[00:01:58] extracting /checkout/obj/build/cache/2018-08-01/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:00]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:24] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:24] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:24] Makefile:81: recipe for target 'prepare' failed
[00:02:24] make: *** [prepare] Error 1
[00:02:25] Command failed. Attempt 2/5:
[00:02:25] Command failed. Attempt 2/5:
[00:02:27] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:27] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:27] Makefile:81: recipe for target 'prepare' failed
[00:02:27] make: *** [prepare] Error 1
[00:02:29] Command failed. Attempt 3/5:
[00:02:29] Command failed. Attempt 3/5:
[00:02:29] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:29] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:29] Makefile:81: recipe for target 'prepare' failed
[00:02:29] make: *** [prepare] Error 1
[00:02:32] Command failed. Attempt 4/5:
[00:02:32] Command failed. Attempt 4/5:
[00:02:32] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:32] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:32] Makefile:81: recipe for target 'prepare' failed
[00:02:32] make: *** [prepare] Error 1
[00:02:36] Command failed. Attempt 5/5:
[00:02:36] Command failed. Attempt 5/5:
[00:02:38] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:38] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:38] Makefile:81: recipe for target 'prepare' failed
[00:02:38] make: *** [prepare] Error 1
[00:02:38] The command has failed after 5 attempts.
travis_time:end:076feb4c:start=1535343529626126104,finish=1535343691838727626,duration=162212601522
---
travis_time:end:21582c00:start=1535343692350487547,finish=1535343692362633227,duration=12145680
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:008ffebc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08e3b4ab
travis_time:start:08e3b4ab
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1105f73f
$ dmesg | grep -i kill
