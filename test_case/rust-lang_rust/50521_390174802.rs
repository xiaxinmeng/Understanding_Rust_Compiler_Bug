plain
[00:01:10] extracting /checkout/obj/build/cache/2018-05-10/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:10] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:10] 
[00:01:10] Caused by:
[00:01:10]   patch for `clippy_lints` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:10] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:10] Build completed unsuccessfully in 0:00:11
[00:01:10] Makefile:81: recipe for target 'prepare' failed
[00:01:10] make: *** [prepare] Error 1
[00:01:11] Command failed. Attempt 2/5:
[00:01:11] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:11] Caused by:
[00:01:11] Caused by:
[00:01:11]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:11] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:11] Build completed unsuccessfully in 0:00:00
[00:01:11] Makefile:81: recipe for target 'prepare' failed
[00:01:11] make: *** [prepare] Error 1
[00:01:13] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:13] 
[00:01:13] Caused by:
[00:01:13] Caused by:
[00:01:13]   patch for `clippy_lints` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:13] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:13] Build completed unsuccessfully in 0:00:00
[00:01:13] make: *** [prepare] Error 1
[00:01:13] Makefile:81: recipe for target 'prepare' failed
[00:01:16] Command failed. Attempt 4/5:
[00:01:16] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:16] Caused by:
[00:01:16] Caused by:
[00:01:16]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:16] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:16] Build completed unsuccessfully in 0:00:00
[00:01:16] Makefile:81: recipe for target 'prepare' failed
[00:01:16] make: *** [prepare] Error 1
[00:01:20] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:20] 
[00:01:20] Caused by:
[00:01:20] Caused by:
[00:01:20]   patch for `clippy_lints` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:20] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:20] Build completed unsuccessfully in 0:00:00
[00:01:20] Makefile:81: recipe for target 'prepare' failed
[00:01:20] make: *** [prepare] Error 1
[00:01:20] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:0acfd712
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_fold:start:after_failure.4
travis_time:start:09968c40
$ dmesg | grep -i kill
[   10.333122] init: failsafe main process (1093) killed by TERM signal
[   41.594719] init: plymouth-upstart-bridge main process (510) killed by TERM signal
travis_fold:end:after_failure.4

Done. Your build exited with 1.
