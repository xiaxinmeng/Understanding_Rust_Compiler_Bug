plain
[00:01:12] extracting /checkout/obj/build/cache/2018-05-10/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:12] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:12] 
[00:01:12] Caused by:
[00:01:12]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:12] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:12] Build completed unsuccessfully in 0:00:22
[00:01:12] make: *** [prepare] Error 1
[00:01:12] Makefile:81: recipe for target 'prepare' failed
[00:01:13] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:13] 
[00:01:13] Caused by:
[00:01:13] Caused by:
[00:01:13]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:13] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:13] Build completed unsuccessfully in 0:00:00
[00:01:13] Makefile:81: recipe for target 'prepare' failed
[00:01:13] make: *** [prepare] Error 1
[00:01:15] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:15] 
[00:01:15] Caused by:
[00:01:15] Caused by:
[00:01:15]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:15] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:15] Build completed unsuccessfully in 0:00:00
[00:01:15] Makefile:81: recipe for target 'prepare' failed
[00:01:15] make: *** [prepare] Error 1
[00:01:18] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:18] 
[00:01:18] Caused by:
[00:01:18] Caused by:
[00:01:18]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:18] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:18] Build completed unsuccessfully in 0:00:00
[00:01:18] Makefile:81: recipe for target 'prepare' failed
[00:01:18] make: *** [prepare] Error 1
[00:01:22] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:22] 
[00:01:22] Caused by:
[00:01:22] Caused by:
[00:01:22]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:22] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:22] Build completed unsuccessfully in 0:00:00
[00:01:22] make: *** [prepare] Error 1
[00:01:22] Makefile:81: recipe for target 'prepare' failed
[00:01:22] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:02e95d20
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
