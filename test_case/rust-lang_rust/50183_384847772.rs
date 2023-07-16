plain
[00:01:07] extracting /checkout/obj/build/cache/2018-04-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:07] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:07] 
[00:01:07] Caused by:
[00:01:07]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:07] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:07] Build completed unsuccessfully in 0:00:21
[00:01:07] make: *** [prepare] Error 1
[00:01:07] Makefile:81: recipe for target 'prepare' failed
[00:01:07] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:07] 
[00:01:07] Caused by:
[00:01:07] Caused by:
[00:01:07]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:07] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:07] Build completed unsuccessfully in 0:00:00
[00:01:07] Makefile:81: recipe for target 'prepare' failed
[00:01:07] make: *** [prepare] Error 1
[00:01:07] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:07] 
[00:01:07] Caused by:
[00:01:07] Caused by:
[00:01:07]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:07] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:07] Build completed unsuccessfully in 0:00:00
[00:01:07] Makefile:81: recipe for target 'prepare' failed
[00:01:07] make: *** [prepare] Error 1
[00:01:07] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:07] 
[00:01:07] Caused by:
[00:01:07] Caused by:
[00:01:07]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:07] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:07] Build completed unsuccessfully in 0:00:00
[00:01:07] make: *** [prepare] Error 1
[00:01:07] Makefile:81: recipe for target 'prepare' failed
[00:01:07] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:07] 
[00:01:07] Caused by:
[00:01:07] Caused by:
[00:01:07]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:07] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:07] Build completed unsuccessfully in 0:00:00
[00:01:07] Makefile:81: recipe for target 'prepare' failed
[00:01:07] make: *** [prepare] Error 1
[00:01:07] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:1812de48
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
