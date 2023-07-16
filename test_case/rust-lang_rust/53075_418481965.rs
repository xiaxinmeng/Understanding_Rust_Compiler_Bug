plain
[00:01:57] extracting /checkout/obj/build/cache/2018-08-01/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:57] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:57] 
[00:01:57] Caused by:
[00:01:57]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:57] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:57] make: *** [prepare] Error 1
[00:01:57] Makefile:81: recipe for target 'prepare' failed
[00:01:58] Command failed. Attempt 2/5:
[00:02:00] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:00] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:00] 
[00:02:00] Caused by:
[00:02:00]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:00] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:00] Makefile:81: recipe for target 'prepare' failed
[00:02:00] make: *** [prepare] Error 1
[00:02:02] Command failed. Attempt 3/5:
[00:02:02] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:02] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:02] 
[00:02:02] Caused by:
[00:02:02]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:02] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:02] make: *** [prepare] Error 1
[00:02:02] Makefile:81: recipe for target 'prepare' failed
[00:02:05] Command failed. Attempt 4/5:
[00:02:05] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:05] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:05] 
[00:02:05] Caused by:
[00:02:05]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:05] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:05] make: *** [prepare] Error 1
[00:02:05] Makefile:81: recipe for target 'prepare' failed
[00:02:09] Command failed. Attempt 5/5:
[00:02:09] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:09] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:09] 
[00:02:09] Caused by:
[00:02:09]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:09] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:09] make: *** [prepare] Error 1
[00:02:09] Makefile:81: recipe for target 'prepare' failed
[00:02:09] The command has failed after 5 attempts.
travis_time:end:35e19604:start=1536087681287676823,finish=1536087814244656807,duration=132956979984
---
travis_time:end:01cf06b6:start=1536087814691838705,finish=1536087814702932455,duration=11093750
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11dc767d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0077cd8a
travis_time:start:0077cd8a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:233c0d48
$ dmesg | grep -i kill
