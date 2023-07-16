plain
2019-11-17T22:56:22.4551933Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-17T22:56:22.4743978Z ##[command]git config gc.auto 0
2019-11-17T22:56:22.4820251Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-17T22:56:22.4866956Z ##[command]git config --get-all http.proxy
2019-11-17T22:56:22.5022728Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66490/merge:refs/remotes/pull/66490/merge
---
2019-11-17T22:59:36.6157646Z extracting /checkout/obj/build/cache/2019-11-06/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-17T22:59:36.6765097Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-17T22:59:36.6765280Z 
2019-11-17T22:59:36.6765335Z Caused by:
2019-11-17T22:59:36.6766381Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-11-17T22:59:36.6776599Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-17T22:59:36.6823626Z Makefile:67: recipe for target 'prepare' failed
2019-11-17T22:59:36.6823732Z make: *** [prepare] Error 1
2019-11-17T22:59:37.6843329Z Command failed. Attempt 2/5:
2019-11-17T22:59:37.7820463Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-17T22:59:37.7820463Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-17T22:59:37.7821128Z 
2019-11-17T22:59:37.7821357Z Caused by:
2019-11-17T22:59:37.7822001Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-11-17T22:59:37.7832334Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-17T22:59:37.7871376Z Makefile:67: recipe for target 'prepare' failed
2019-11-17T22:59:37.7871898Z make: *** [prepare] Error 1
2019-11-17T22:59:39.7889792Z Command failed. Attempt 3/5:
2019-11-17T22:59:39.8945163Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-17T22:59:39.8945163Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-17T22:59:39.8945873Z 
2019-11-17T22:59:39.8946380Z Caused by:
2019-11-17T22:59:39.8947467Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-11-17T22:59:39.8964575Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-17T22:59:39.9004034Z Makefile:67: recipe for target 'prepare' failed
2019-11-17T22:59:39.9004440Z make: *** [prepare] Error 1
2019-11-17T22:59:42.9027935Z Command failed. Attempt 4/5:
2019-11-17T22:59:43.0002444Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-17T22:59:43.0002444Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-17T22:59:43.0003614Z 
2019-11-17T22:59:43.0003829Z Caused by:
2019-11-17T22:59:43.0004523Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-11-17T22:59:43.0008276Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-17T22:59:43.0045353Z Makefile:67: recipe for target 'prepare' failed
2019-11-17T22:59:43.0049592Z make: *** [prepare] Error 1
2019-11-17T22:59:47.0063807Z Command failed. Attempt 5/5:
2019-11-17T22:59:47.1029958Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-17T22:59:47.1029958Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-17T22:59:47.1030854Z 
2019-11-17T22:59:47.1031216Z Caused by:
2019-11-17T22:59:47.1031852Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-11-17T22:59:47.1038223Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-17T22:59:47.1078706Z Makefile:67: recipe for target 'prepare' failed
2019-11-17T22:59:47.1082367Z make: *** [prepare] Error 1
2019-11-17T22:59:47.1090783Z The command has failed after 5 attempts.
2019-11-17T22:59:47.1091741Z == clock drift check ==
2019-11-17T22:59:47.1091741Z == clock drift check ==
2019-11-17T22:59:47.1099357Z   local time: Sun Nov 17 22:59:47 UTC 2019
2019-11-17T22:59:47.1926507Z   network time: Sun, 17 Nov 2019 22:59:47 GMT
2019-11-17T22:59:47.1929219Z == end clock drift check ==
2019-11-17T22:59:59.4893050Z 
2019-11-17T22:59:59.4983915Z ##[error]Bash exited with code '1'.
2019-11-17T22:59:59.5010411Z ##[section]Starting: Checkout
2019-11-17T22:59:59.5012115Z ==============================================================================
2019-11-17T22:59:59.5012212Z Task         : Get sources
2019-11-17T22:59:59.5012258Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
