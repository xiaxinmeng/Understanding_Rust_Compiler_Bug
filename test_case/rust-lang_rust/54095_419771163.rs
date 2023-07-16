plain
[00:02:04] extracting /checkout/obj/build/cache/2018-08-01/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:04] error: failed to resolve patches for `https://github.com/rust-lang-nursery/rust-clippy`
[00:02:04] 
[00:02:04] Caused by:
[00:02:04]   patch for `clippy_lints` in `https://github.com/rust-lang-nursery/rust-clippy` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:04] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:04] make: *** [prepare] Error 1
[00:02:04] Makefile:81: recipe for target 'prepare' failed
[00:02:05] Command failed. Attempt 2/5:
[00:02:05] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:02:05] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:02:05] 
[00:02:05] Caused by:
[00:02:05]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:05] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:05] Makefile:81: recipe for target 'prepare' failed
[00:02:05] make: *** [prepare] Error 1
[00:02:07] Command failed. Attempt 3/5:
[00:02:07] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:02:07] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:02:07] 
[00:02:07] Caused by:
[00:02:07]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:07] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:07] make: *** [prepare] Error 1
[00:02:07] Makefile:81: recipe for target 'prepare' failed
[00:02:10] Command failed. Attempt 4/5:
[00:02:10] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:10] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:02:10] 
[00:02:10] Caused by:
[00:02:10]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:10] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:10] make: *** [prepare] Error 1
[00:02:10] Makefile:81: recipe for target 'prepare' failed
[00:02:14] Command failed. Attempt 5/5:
[00:02:14] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:02:14] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:02:14] 
[00:02:14] Caused by:
[00:02:14]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:14] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:14] make: *** [prepare] Error 1
[00:02:14] Makefile:81: recipe for target 'prepare' failed
[00:02:14] The command has failed after 5 attempts.
travis_time:end:04ac2630:start=1536547046979439518,finish=1536547181764334898,duration=134784895380
---
travis_time:end:18ce3a66:start=1536547182265977098,finish=1536547182276325278,duration=10348180
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:221efcd1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:117736f0
travis_time:start:117736f0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:15902808
$ dmesg | grep -i kill
