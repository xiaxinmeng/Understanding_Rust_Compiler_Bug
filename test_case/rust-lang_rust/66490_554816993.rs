plain
2019-11-18T01:44:50.8491971Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-18T01:44:51.8009703Z ##[command]git config gc.auto 0
2019-11-18T01:44:51.8016008Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-18T01:44:51.8021229Z ##[command]git config --get-all http.proxy
2019-11-18T01:44:51.8026583Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66490/merge:refs/remotes/pull/66490/merge
---
2019-11-18T01:48:16.5263683Z extracting /checkout/obj/build/cache/2019-11-06/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-18T01:48:16.5946050Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-18T01:48:16.5946215Z 
2019-11-18T01:48:16.5946374Z Caused by:
2019-11-18T01:48:16.5946827Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-11-18T01:48:16.5958314Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-18T01:48:16.6004769Z Makefile:67: recipe for target 'prepare' failed
2019-11-18T01:48:16.6005120Z make: *** [prepare] Error 1
2019-11-18T01:48:17.6021490Z Command failed. Attempt 2/5:
2019-11-18T01:48:17.7145955Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-18T01:48:17.7145955Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-18T01:48:17.7146140Z 
2019-11-18T01:48:17.7146199Z Caused by:
2019-11-18T01:48:17.7146681Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-11-18T01:48:17.7155886Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-18T01:48:17.7202928Z Makefile:67: recipe for target 'prepare' failed
2019-11-18T01:48:17.7203019Z make: *** [prepare] Error 1
2019-11-18T01:48:19.7217631Z Command failed. Attempt 3/5:
2019-11-18T01:48:19.8334267Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-18T01:48:19.8334267Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-18T01:48:19.8334393Z 
2019-11-18T01:48:19.8334571Z Caused by:
2019-11-18T01:48:19.8335095Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-11-18T01:48:19.8340881Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-18T01:48:19.8385215Z Makefile:67: recipe for target 'prepare' failed
2019-11-18T01:48:19.8389626Z make: *** [prepare] Error 1
2019-11-18T01:48:22.8406198Z Command failed. Attempt 4/5:
2019-11-18T01:48:22.9500354Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-18T01:48:22.9500354Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-18T01:48:22.9500454Z 
2019-11-18T01:48:22.9500579Z Caused by:
2019-11-18T01:48:22.9501072Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-11-18T01:48:22.9508520Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-18T01:48:22.9557421Z Makefile:67: recipe for target 'prepare' failed
2019-11-18T01:48:22.9557587Z make: *** [prepare] Error 1
2019-11-18T01:48:26.9570849Z Command failed. Attempt 5/5:
2019-11-18T01:48:27.0688024Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-18T01:48:27.0688024Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-18T01:48:27.0688130Z 
2019-11-18T01:48:27.0688181Z Caused by:
2019-11-18T01:48:27.0688678Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-11-18T01:48:27.0695673Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-18T01:48:27.0746013Z Makefile:67: recipe for target 'prepare' failed
2019-11-18T01:48:27.0746092Z make: *** [prepare] Error 1
2019-11-18T01:48:27.0746206Z The command has failed after 5 attempts.
2019-11-18T01:48:27.0749360Z == clock drift check ==
2019-11-18T01:48:27.0749360Z == clock drift check ==
2019-11-18T01:48:27.0762471Z   local time: Mon Nov 18 01:48:27 UTC 2019
2019-11-18T01:48:27.3442067Z   network time: Mon, 18 Nov 2019 01:48:27 GMT
2019-11-18T01:48:27.3442192Z == end clock drift check ==
2019-11-18T01:48:37.8716252Z 
2019-11-18T01:48:37.8827949Z ##[error]Bash exited with code '1'.
2019-11-18T01:48:37.8853359Z ##[section]Starting: Checkout
2019-11-18T01:48:37.8855053Z ==============================================================================
2019-11-18T01:48:37.8855146Z Task         : Get sources
2019-11-18T01:48:37.8855186Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
