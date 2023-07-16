plain
[00:01:05] extracting /checkout/obj/build/cache/2018-05-10/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:05] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:05] 
[00:01:05] Caused by:
[00:01:05]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:05] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:05] Build completed unsuccessfully in 0:00:13
[00:01:05] Makefile:81: recipe for target 'prepare' failed
[00:01:05] make: *** [prepare] Error 1
[00:01:06] Command failed. Attempt 2/5:
[00:01:06] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:06] Caused by:
[00:01:06] Caused by:
[00:01:06]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:06] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:06] Build completed unsuccessfully in 0:00:00
[00:01:06] Makefile:81: recipe for target 'prepare' failed
[00:01:06] make: *** [prepare] Error 1
[00:01:08] Command failed. Attempt 3/5:
[00:01:08] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:08] Caused by:
[00:01:08] Caused by:
[00:01:08]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:08] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:08] Build completed unsuccessfully in 0:00:00
[00:01:08] Makefile:81: recipe for target 'prepare' failed
[00:01:08] make: *** [prepare] Error 1
[00:01:11] Command failed. Attempt 4/5:
[00:01:11] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:11] Caused by:
[00:01:11] Caused by:
[00:01:11]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:11] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:11] make: *** [prepare] Error 1
[00:01:11] make: *** [prepare] Error 1
[00:01:11] Makefile:81: recipe for target 'prepare' failed
[00:01:15] Command failed. Attempt 5/5:
[00:01:15] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:15] Caused by:
[00:01:15] Caused by:
[00:01:15]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:15] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:15] Build completed unsuccessfully in 0:00:00
[00:01:15] Makefile:81: recipe for target 'prepare' failed
[00:01:15] make: *** [prepare] Error 1
[00:01:15] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:163cee2a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:000f1a4b:start=1528998637461580445,finish=1528998637467663438,duration=6082993
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:21c08550
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1678068c
$ dmesg | grep -i kill
