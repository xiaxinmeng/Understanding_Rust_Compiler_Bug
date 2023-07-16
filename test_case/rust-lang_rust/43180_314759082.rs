
Attempting with retry: make prepare
[00:00:30] downloading https://static.rust-lang.org/dist/2017-06-15/rust-std-beta-x86_64-unknown-linux-gnu.tar.gz
######################################################################## 100.0%
[00:00:38] extracting /checkout/obj/build/cache/2017-06-15/rust-std-beta-x86_64-unknown-linux-gnu.tar.gz
[00:00:38] downloading https://static.rust-lang.org/dist/2017-06-15/rustc-beta-x86_64-unknown-linux-gnu.tar.gz
######################################################################## 100.0%
[00:00:42] extracting /checkout/obj/build/cache/2017-06-15/rustc-beta-x86_64-unknown-linux-gnu.tar.gz
[00:00:42] downloading https://static.rust-lang.org/dist/2017-06-15/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
######################################################################## 100.0%
[00:00:43] extracting /checkout/obj/build/cache/2017-06-15/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:00:43]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:00:49]     Updating git repository `https://github.com/rust-lang/cargo`
[00:00:54]     Updating git repository `https://github.com/rust-lang-nursery/rustfmt`
[00:00:58] error: the lock file needs to be updated but --frozen was passed to prevent this
[00:00:58] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:00:58] Build completed unsuccessfully in 0:00:27
[00:00:58] make: *** [prepare] Error 1
[00:00:58] Makefile:76: recipe for target 'prepare' failed
[00:00:58] Command failed. Attempt 2/5:
[00:00:58] error: the lock file needs to be updated but --frozen was passed to prevent this
[00:00:58] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:00:58] Build completed unsuccessfully in 0:00:00
[00:00:58] Makefile:76: recipe for target 'prepare' failed
[00:00:58] make: *** [prepare] Error 1
[00:00:58] Command failed. Attempt 3/5:
[00:00:58] error: the lock file needs to be updated but --frozen was passed to prevent this
[00:00:58] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:00:58] Build completed unsuccessfully in 0:00:00
[00:00:58] Makefile:76: recipe for target 'prepare' failed
[00:00:58] make: *** [prepare] Error 1
[00:00:58] Command failed. Attempt 4/5:
[00:00:58] error: the lock file needs to be updated but --frozen was passed to prevent this
[00:00:58] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:00:58] Build completed unsuccessfully in 0:00:00
[00:00:58] make: *** [prepare] Error 1
[00:00:58] Makefile:76: recipe for target 'prepare' failed
[00:00:58] Command failed. Attempt 5/5:
[00:00:58] error: the lock file needs to be updated but --frozen was passed to prevent this
[00:00:58] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:00:58] Build completed unsuccessfully in 0:00:00
[00:00:58] Makefile:76: recipe for target 'prepare' failed
[00:00:58] make: *** [prepare] Error 1
[00:00:58] The command has failed after 5 attempts.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
