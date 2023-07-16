plain
[00:01:49] extracting /checkout/obj/build/cache/2018-05-10/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:49] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:49] 
[00:01:49] Caused by:
[00:01:49]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:49] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:49] Build completed unsuccessfully in 0:00:12
[00:01:49] Makefile:81: recipe for target 'prepare' failed
[00:01:49] make: *** [prepare] Error 1
[00:01:50] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:50] 
[00:01:50] Caused by:
[00:01:50] Caused by:
[00:01:50]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:50] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:50] Build completed unsuccessfully in 0:00:00
[00:01:50] make: *** [prepare] Error 1
[00:01:50] Makefile:81: recipe for target 'prepare' failed
[00:01:52] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:52] 
[00:01:52] Caused by:
[00:01:52] Caused by:
[00:01:52]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:52] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:52] Build completed unsuccessfully in 0:00:00
[00:01:52] make: *** [prepare] Error 1
[00:01:52] Makefile:81: recipe for target 'prepare' failed
[00:01:55] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:55] 
[00:01:55] Caused by:
[00:01:55] Caused by:
[00:01:55]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:55] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:55] Build completed unsuccessfully in 0:00:00
[00:01:55] Makefile:81: recipe for target 'prepare' failed
[00:01:55] make: *** [prepare] Error 1
[00:01:59] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:59] 
[00:01:59] Caused by:
[00:01:59] Caused by:
[00:01:59]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:59] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:59] Build completed unsuccessfully in 0:00:00
[00:01:59] Makefile:81: recipe for target 'prepare' failed
[00:01:59] make: *** [prepare] Error 1
[00:01:59] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:0272695c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
