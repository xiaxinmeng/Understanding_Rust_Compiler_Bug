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
[00:01:58] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:58] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:58] 
[00:01:58] Caused by:
[00:01:58]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:58] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:58] Makefile:81: recipe for target 'prepare' failed
[00:01:58] make: *** [prepare] Error 1
[00:02:00] Command failed. Attempt 3/5:
[00:02:00] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:00] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:00] 
[00:02:00] Caused by:
[00:02:00]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:00] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:00] Makefile:81: recipe for target 'prepare' failed
[00:02:00] make: *** [prepare] Error 1
[00:02:03] Command failed. Attempt 4/5:
[00:02:03] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:03] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:03] 
[00:02:03] Caused by:
[00:02:03]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:03] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:03] Makefile:81: recipe for target 'prepare' failed
[00:02:03] make: *** [prepare] Error 1
[00:02:07] Command failed. Attempt 5/5:
[00:02:07] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:07] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:07] 
[00:02:07] Caused by:
[00:02:07]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:07] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:07] make: *** [prepare] Error 1
[00:02:07] Makefile:81: recipe for target 'prepare' failed
[00:02:07] The command has failed after 5 attempts.
travis_time:end:16c50f06:start=1536180867770422072,finish=1536181001567828560,duration=133797406488
---
travis_time:end:21f542c9:start=1536181002050515312,finish=1536181002060906144,duration=10390832
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:132639ba
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:050be3a2
travis_time:start:050be3a2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ef40fd2
$ dmesg | grep -i kill
