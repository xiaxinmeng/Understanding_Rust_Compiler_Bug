plain
[00:00:47] configure: rust.quiet-tests     := True
---
[00:01:23] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:23] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:23] Build completed unsuccessfully in 0:00:36
[00:01:23] make: *** [prepare] Error 1
[00:01:23] Makefile:81: recipe for target 'prepare' failed
[00:01:23] Command failed. Attempt 2/5:
[00:01:24]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:24] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:24] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:24] Build completed unsuccessfully in 0:00:00
[00:01:24] Makefile:81: recipe for target 'prepare' failed
[00:01:24] make: *** [prepare] Error 1
[00:01:24] Command failed. Attempt 3/5:
[00:01:24]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:24] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:24] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:24] Build completed unsuccessfully in 0:00:00
[00:01:24] Makefile:81: recipe for target 'prepare' failed
[00:01:24] make: *** [prepare] Error 1
[00:01:24] Command failed. Attempt 4/5:
[00:01:25]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:25] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:25] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:25] Build completed unsuccessfully in 0:00:00
[00:01:25] make: *** [prepare] Error 1
[00:01:25] Makefile:81: recipe for target 'prepare' failed
[00:01:25] Command failed. Attempt 5/5:
[00:01:25]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:25] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:25] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:25] Build completed unsuccessfully in 0:00:00
[00:01:25] make: *** [prepare] Error 1
[00:01:25] Makefile:81: recipe for target 'prepare' failed
[00:01:25] The command has failed after 5 attempts.
