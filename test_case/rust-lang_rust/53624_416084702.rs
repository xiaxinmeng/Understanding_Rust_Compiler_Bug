plain
######################################################################    98.0%
######################################################################## 100.0%
[00:02:08] extracting /checkout/obj/build/cache/2018-08-01/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:13]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:36] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:36] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:36] Makefile:81: recipe for target 'prepare' failed
[00:02:36] make: *** [prepare] Error 1
[00:02:37] Command failed. Attempt 2/5:
[00:02:37] Command failed. Attempt 2/5:
[00:02:37] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:37] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:37] make: *** [prepare] Error 1
[00:02:37] Makefile:81: recipe for target 'prepare' failed
[00:02:39] Command failed. Attempt 3/5:
[00:02:39] Command failed. Attempt 3/5:
[00:02:39] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:39] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:39] make: *** [prepare] Error 1
[00:02:39] Makefile:81: recipe for target 'prepare' failed
[00:02:42] Command failed. Attempt 4/5:
[00:02:42] Command failed. Attempt 4/5:
[00:02:43] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:43] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:43] Makefile:81: recipe for target 'prepare' failed
[00:02:43] make: *** [prepare] Error 1
[00:02:47] Command failed. Attempt 5/5:
[00:02:47] Command failed. Attempt 5/5:
[00:02:47] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:47] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:47] make: *** [prepare] Error 1
[00:02:47] Makefile:81: recipe for target 'prepare' failed
[00:02:47] The command has failed after 5 attempts.
travis_time:end:14570ece:start=1535329687676662679,finish=1535329855435349045,duration=167758686366
---
travis_time:end:25ad7200:start=1535329855879001484,finish=1535329855892545901,duration=13544417
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:28f65ef0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01158c53
travis_time:start:01158c53
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:052459a5
$ dmesg | grep -i kill
