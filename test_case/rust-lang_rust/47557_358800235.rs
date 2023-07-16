
[00:02:53] Makefile:76: recipe for target 'prepare' failed
[00:02:53] Command failed. Attempt 5/5:
[00:02:53]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:53] error: failed to select a version for `compiletest_rs` (required by `miri`):
[00:02:53] all possible versions conflict with previously selected versions of `compiletest_rs`
[00:02:53]   version 0.3.5 in use by compiletest_rs v0.3.5
[00:02:53]   possible versions to select: 0.3.3
[00:02:53] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
