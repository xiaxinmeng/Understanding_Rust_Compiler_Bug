
[00:07:21] downloading https://static.rust-lang.org/dist/2017-07-18/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
######################################################################## 100.0%
[00:07:22] extracting /checkout/obj/build/tmp/distcheck/build/cache/2017-07-18/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:07:22] error: failed to load source for a dependency on `serde_derive`
[00:07:22] 
[00:07:22] Caused by:
[00:07:22]   Unable to update registry https://github.com/rust-lang/crates.io-index
[00:07:22] 
[00:07:22] Caused by:
[00:07:22]   failed to update replaced source `registry https://github.com/rust-lang/crates.io-index`
[00:07:22] 
[00:07:22] Caused by:
[00:07:22]   failed to read root of directory source: /checkout/obj/build/tmp/distcheck/src/vendor
[00:07:22] 
[00:07:22] Caused by:
[00:07:22]   No such file or directory (os error 2)
[00:07:22] failed to run: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/obj/build/tmp/distcheck/src/bootstrap/Cargo.toml --locked --frozen
[00:07:22] Build completed unsuccessfully in 0:00:09
[00:07:22] Makefile:54: recipe for target 'check' failed
