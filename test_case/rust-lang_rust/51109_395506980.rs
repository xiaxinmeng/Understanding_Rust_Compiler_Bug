plain
[00:01:32] extracting /checkout/obj/build/cache/2018-05-10/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:33] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:33] 
[00:01:33] Caused by:
[00:01:33]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:33] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:33] Build completed unsuccessfully in 0:00:12
[00:01:33] Makefile:81: recipe for target 'prepare' failed
[00:01:33] make: *** [prepare] Error 1
[00:01:34] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:34] 
[00:01:34] Caused by:
[00:01:34] Caused by:
[00:01:34]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:34] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:34] Build completed unsuccessfully in 0:00:00
[00:01:34] Makefile:81: recipe for target 'prepare' failed
[00:01:34] make: *** [prepare] Error 1
[00:01:36] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:36] 
[00:01:36] Caused by:
[00:01:36] Caused by:
[00:01:36]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:36] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:36] Build completed unsuccessfully in 0:00:00
[00:01:36] make: *** [prepare] Error 1
[00:01:36] Makefile:81: recipe for target 'prepare' failed
[00:01:39] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:39] 
[00:01:39] Caused by:
[00:01:39] Caused by:
[00:01:39]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:39] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:39] Build completed unsuccessfully in 0:00:00
[00:01:39] Makefile:81: recipe for target 'prepare' failed
[00:01:39] make: *** [prepare] Error 1
[00:01:43] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:43] 
[00:01:43] Caused by:
[00:01:43] Caused by:
[00:01:43]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:43] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:43] Build completed unsuccessfully in 0:00:00
[00:01:43] make: *** [prepare] Error 1
[00:01:43] Makefile:81: recipe for target 'prepare' failed
[00:01:43] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:1643c3e7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
