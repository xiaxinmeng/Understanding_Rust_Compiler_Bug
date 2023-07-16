plain
#######################################################                   76.7%
######################################################################## 100.0%
[00:02:23] extracting /checkout/obj/build/cache/2018-08-01/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:25]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:47] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:47] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:47] Makefile:81: recipe for target 'prepare' failed
[00:02:47] make: *** [prepare] Error 1
[00:02:48] Command failed. Attempt 2/5:
[00:02:48]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:48]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:49] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:49] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:49] make: *** [prepare] Error 1
[00:02:49] Makefile:81: recipe for target 'prepare' failed
[00:02:51] Command failed. Attempt 3/5:
[00:02:51]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:51]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:51] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:51] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:51] make: *** [prepare] Error 1
[00:02:51] Makefile:81: recipe for target 'prepare' failed
[00:02:54] Command failed. Attempt 4/5:
[00:02:54]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:54]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:55] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:55] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:55] make: *** [prepare] Error 1
[00:02:55] Makefile:81: recipe for target 'prepare' failed
[00:02:59] Command failed. Attempt 5/5:
[00:02:59]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:59]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:59] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:59] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:59] Makefile:81: recipe for target 'prepare' failed
[00:02:59] make: *** [prepare] Error 1
[00:02:59] The command has failed after 5 attempts.
travis_time:end:0b4ab0c9:start=1535758972819127765,finish=1535759160192839620,duration=187373711855
---
travis_time:end:0be20c09:start=1535759160590333652,finish=1535759160596840153,duration=6506501
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:24632646
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0bff49ea
travis_time:start:0bff49ea
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07c7933e
$ dmesg | grep -i kill
