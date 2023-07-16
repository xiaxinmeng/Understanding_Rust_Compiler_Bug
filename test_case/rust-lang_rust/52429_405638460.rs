plain
###############################################################           88.0%
######################################################################## 100.0%
[00:01:22] extracting /checkout/obj/build/cache/2018-07-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:23]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:00] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:00] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:00] make: *** [prepare] Error 1
[00:02:00] Makefile:81: recipe for target 'prepare' failed
[00:02:01] Command failed. Attempt 2/5:
[00:02:01]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:01]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:01] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:01] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:01] Makefile:81: recipe for target 'prepare' failed
[00:02:01] make: *** [prepare] Error 1
[00:02:03] Command failed. Attempt 3/5:
[00:02:03]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:03]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:04] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:04] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:04] Makefile:81: recipe for target 'prepare' failed
[00:02:04] make: *** [prepare] Error 1
[00:02:07] Command failed. Attempt 4/5:
[00:02:07]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:07]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:08] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:08] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:08] make: *** [prepare] Error 1
[00:02:08] Makefile:81: recipe for target 'prepare' failed
[00:02:12] Command failed. Attempt 5/5:
[00:02:12]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:12]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:12] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:12] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:12] Makefile:81: recipe for target 'prepare' failed
[00:02:12] make: *** [prepare] Error 1
[00:02:12] The command has failed after 5 attempts.
travis_time:end:03da9361:start=1531843656926297600,finish=1531843789942371239,duration=133016073639
---
travis_time:end:00aff01a:start=1531843790297977687,finish=1531843790307242356,duration=9264669
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02c2a1aa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07467a12
travis_time:start:07467a12
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06571668
$ dmesg | grep -i kill
