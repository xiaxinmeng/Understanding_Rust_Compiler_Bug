plain
2019-11-23T02:16:53.0350007Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-23T02:16:53.0520763Z ##[command]git config gc.auto 0
2019-11-23T02:16:53.0591598Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-23T02:16:53.0645448Z ##[command]git config --get-all http.proxy
2019-11-23T02:16:53.0814200Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66641/merge:refs/remotes/pull/66641/merge
---
2019-11-23T02:20:08.2145246Z extracting /checkout/obj/build/cache/2019-11-06/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-23T02:20:08.2757194Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-23T02:20:08.2758290Z 
2019-11-23T02:20:08.2758560Z Caused by:
2019-11-23T02:20:08.2759182Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-11-23T02:20:08.2765497Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-23T02:20:08.2808004Z make: *** [prepare] Error 1
2019-11-23T02:20:08.2808813Z Makefile:67: recipe for target 'prepare' failed
2019-11-23T02:20:09.2823964Z Command failed. Attempt 2/5:
2019-11-23T02:20:09.3730961Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-23T02:20:09.3730961Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-23T02:20:09.3731032Z 
2019-11-23T02:20:09.3731089Z Caused by:
2019-11-23T02:20:09.3731502Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-11-23T02:20:09.3731753Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-23T02:20:09.3763100Z Makefile:67: recipe for target 'prepare' failed
2019-11-23T02:20:09.3763157Z make: *** [prepare] Error 1
2019-11-23T02:20:11.3773204Z Command failed. Attempt 3/5:
2019-11-23T02:20:11.4691555Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-23T02:20:11.4691555Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-23T02:20:11.4691796Z 
2019-11-23T02:20:11.4691931Z Caused by:
2019-11-23T02:20:11.4692522Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-11-23T02:20:11.4700101Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-23T02:20:11.4729729Z make: *** [prepare] Error 1
2019-11-23T02:20:11.4733791Z Makefile:67: recipe for target 'prepare' failed
2019-11-23T02:20:14.4746816Z Command failed. Attempt 4/5:
2019-11-23T02:20:14.5784430Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-23T02:20:14.5784430Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-23T02:20:14.5785110Z 
2019-11-23T02:20:14.5785379Z Caused by:
2019-11-23T02:20:14.5786131Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-11-23T02:20:14.5793129Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-23T02:20:14.5834768Z Makefile:67: recipe for target 'prepare' failed
2019-11-23T02:20:14.5834856Z make: *** [prepare] Error 1
2019-11-23T02:20:18.5853524Z Command failed. Attempt 5/5:
2019-11-23T02:20:18.6902845Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-23T02:20:18.6902845Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-11-23T02:20:18.6903513Z 
2019-11-23T02:20:18.6903684Z Caused by:
2019-11-23T02:20:18.6904189Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-11-23T02:20:18.6911305Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
2019-11-23T02:20:18.6967474Z Makefile:67: recipe for target 'prepare' failed
2019-11-23T02:20:18.6985054Z make: *** [prepare] Error 1
2019-11-23T02:20:18.6985214Z The command has failed after 5 attempts.
2019-11-23T02:20:18.6985261Z == clock drift check ==
2019-11-23T02:20:18.6985261Z == clock drift check ==
2019-11-23T02:20:18.6985309Z   local time: Sat Nov 23 02:20:18 UTC 2019
2019-11-23T02:20:18.7316252Z   network time: Sat, 23 Nov 2019 02:20:18 GMT
2019-11-23T02:20:18.7320848Z == end clock drift check ==
2019-11-23T02:20:29.4364190Z 
2019-11-23T02:20:29.4456916Z ##[error]Bash exited with code '1'.
2019-11-23T02:20:29.4482237Z ##[section]Starting: Checkout
2019-11-23T02:20:29.4483661Z ==============================================================================
2019-11-23T02:20:29.4483733Z Task         : Get sources
2019-11-23T02:20:29.4483768Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
