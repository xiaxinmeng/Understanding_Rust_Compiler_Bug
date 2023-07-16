plain
################################################################          89.7%
######################################################################## 100.0%
[00:02:11] extracting /checkout/obj/build/cache/2018-07-31/cargo-0.29.0-x86_64-unknown-linux-gnu.tar.gz
[00:02:11]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:35] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:35] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:35] Makefile:81: recipe for target 'prepare' failed
[00:02:35] make: *** [prepare] Error 1
[00:02:36] Command failed. Attempt 2/5:
[00:02:36]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:36]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:36] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:36] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:36] Makefile:81: recipe for target 'prepare' failed
[00:02:36] make: *** [prepare] Error 1
[00:02:38] Command failed. Attempt 3/5:
[00:02:38]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:38]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:39] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:39] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:39] Makefile:81: recipe for target 'prepare' failed
[00:02:39] make: *** [prepare] Error 1
[00:02:42] Command failed. Attempt 4/5:
[00:02:42]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:42]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:42] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:42] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:42] Makefile:81: recipe for target 'prepare' failed
[00:02:42] make: *** [prepare] Error 1
[00:02:46] Command failed. Attempt 5/5:
[00:02:46]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:46]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:47] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:47] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:47] make: *** [prepare] Error 1
[00:02:47] Makefile:81: recipe for target 'prepare' failed
[00:02:47] The command has failed after 5 attempts.
travis_time:end:023eadc0:start=1533040076140033062,finish=1533040247507961314,duration=171367928252
---
travis_time:end:15939c9e:start=1533040247799740887,finish=1533040247808225017,duration=8484130
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:14a9a4a3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06de5736
travis_time:start:06de5736
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13387233
$ dmesg | grep -i kill
