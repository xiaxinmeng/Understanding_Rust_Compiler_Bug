plain
##############################################                            64.8%
######################################################################## 100.0%
[00:01:15] extracting /checkout/obj/build/cache/2018-08-01/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:15]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:51] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:51] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:51] Makefile:81: recipe for target 'prepare' failed
[00:01:51] make: *** [prepare] Error 1
[00:01:52] Command failed. Attempt 2/5:
[00:01:52] Command failed. Attempt 2/5:
[00:01:52] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:52] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:52] Makefile:81: recipe for target 'prepare' failed
[00:01:52] make: *** [prepare] Error 1
[00:01:54] Command failed. Attempt 3/5:
[00:01:54] Command failed. Attempt 3/5:
[00:01:54] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:54] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:54] make: *** [prepare] Error 1
[00:01:54] Makefile:81: recipe for target 'prepare' failed
[00:01:57] Command failed. Attempt 4/5:
[00:01:57] Command failed. Attempt 4/5:
[00:01:58] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:58] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:58] Makefile:81: recipe for target 'prepare' failed
[00:01:58] make: *** [prepare] Error 1
[00:02:02] Command failed. Attempt 5/5:
[00:02:02] Command failed. Attempt 5/5:
[00:02:02] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:02] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:02] Makefile:81: recipe for target 'prepare' failed
[00:02:02] make: *** [prepare] Error 1
[00:02:02] The command has failed after 5 attempts.
travis_time:end:12653f44:start=1533474648609902903,finish=1533474771393270737,duration=122783367834
---
travis_time:end:0d779f9c:start=1533474771715714082,finish=1533474771727554783,duration=11840701
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2ce10ce0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00779268
travis_time:start:00779268
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07c6c7e5
$ dmesg | grep -i kill
