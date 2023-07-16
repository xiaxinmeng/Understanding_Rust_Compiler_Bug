plain
2020-01-15T15:20:03.1299052Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-15T15:20:03.1364613Z ##[command]git config gc.auto 0
2020-01-15T15:20:03.1366495Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-15T15:20:03.1368035Z ##[command]git config --get-all http.proxy
2020-01-15T15:20:03.1370028Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68012/merge:refs/remotes/pull/68012/merge
---
2020-01-15T15:25:50.0129570Z 
2020-01-15T15:25:50.1186372Z error[E0599]: no method named `load_consume` found for type `std::sync::atomic::AtomicUsize` in the current scope
2020-01-15T15:25:50.1187007Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/crossbeam-epoch-0.7.2/src/atomic.rs:234:47
2020-01-15T15:25:50.1187248Z     |
2020-01-15T15:25:50.1187530Z 234 |         unsafe { Shared::from_usize(self.data.load_consume()) }
2020-01-15T15:25:50.1195601Z 
2020-01-15T15:25:50.2014166Z error: aborting due to 2 previous errors
2020-01-15T15:25:50.2019939Z 
2020-01-15T15:25:50.2026643Z Some errors have detailed explanations: E0432, E0599.
2020-01-15T15:25:50.2026643Z Some errors have detailed explanations: E0432, E0599.
2020-01-15T15:25:50.2032903Z For more information about an error, try `rustc --explain E0432`.
2020-01-15T15:25:50.2072719Z error: could not compile `crossbeam-epoch`.
2020-01-15T15:25:50.2094738Z warning: build failed, waiting for other jobs to finish...
2020-01-15T15:26:18.1698735Z error: build failed
2020-01-15T15:26:18.1722361Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-15T15:26:18.1732213Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-15T15:26:18.1732797Z Build completed unsuccessfully in 0:03:18
2020-01-15T15:26:18.1782357Z == clock drift check ==
2020-01-15T15:26:18.1792572Z   local time: Wed Jan 15 15:26:18 UTC 2020
2020-01-15T15:26:18.1792572Z   local time: Wed Jan 15 15:26:18 UTC 2020
2020-01-15T15:26:18.4510446Z   network time: Wed, 15 Jan 2020 15:26:18 GMT
2020-01-15T15:26:18.4516879Z == end clock drift check ==
2020-01-15T15:26:19.2594624Z 
2020-01-15T15:26:19.2699975Z ##[error]Bash exited with code '1'.
2020-01-15T15:26:19.2727131Z ##[section]Starting: Checkout
2020-01-15T15:26:19.2728704Z ==============================================================================
2020-01-15T15:26:19.2728755Z Task         : Get sources
2020-01-15T15:26:19.2728799Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
