plain
[00:01:20] extracting /checkout/obj/build/cache/2018-05-10/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:20] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:20] 
[00:01:20] Caused by:
[00:01:20]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:20] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:20] Build completed unsuccessfully in 0:00:18
[00:01:20] make: *** [prepare] Error 1
[00:01:20] Makefile:81: recipe for target 'prepare' failed
[00:01:22] error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
[00:01:22] 
[00:01:22] Caused by:
[00:01:22] Caused by:
[00:01:22]   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:22] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:22] Build completed unsuccessfully in 0:00:00
[00:01:22] make: *** [prepare] Error 1
[00:01:22] Makefile:81: recipe for target 'prepare' failed
[00:01:24] Command failed. Attempt 3/5:
[00:01:24] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:24] Caused by:
[00:01:24] Caused by:
[00:01:24]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:24] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:24] Build completed unsuccessfully in 0:00:00
[00:01:24] Makefile:81: recipe for target 'prepare' failed
[00:01:24] make: *** [prepare] Error 1
[00:01:27] Command failed. Attempt 4/5:
[00:01:27] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:27] Caused by:
[00:01:27] Caused by:
[00:01:27]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:27] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:27] Build completed unsuccessfully in 0:00:00
[00:01:27] make: *** [prepare] Error 1
[00:01:27] Makefile:81: recipe for target 'prepare' failed
[00:01:31] Command failed. Attempt 5/5:
[00:01:31] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:31] Caused by:
[00:01:31] Caused by:
[00:01:31]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:31] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:31] Build completed unsuccessfully in 0:00:00
[00:01:31] make: *** [prepare] Error 1
[00:01:31] Makefile:81: recipe for target 'prepare' failed
[00:01:31] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:00b2a453
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
