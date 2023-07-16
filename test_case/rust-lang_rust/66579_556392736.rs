plain
2019-11-20T20:57:03.0407851Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-20T20:57:03.0619174Z ##[command]git config gc.auto 0
2019-11-20T20:57:03.0691009Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-20T20:57:03.0740827Z ##[command]git config --get-all http.proxy
2019-11-20T20:57:03.0890514Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66579/merge:refs/remotes/pull/66579/merge
---
2019-11-20T21:03:31.1934477Z    Compiling compiler_builtins v0.1.18
2019-11-20T21:03:31.7347702Z error[E0723]: can only call a curated list of intrinsics in `min_const_fn`
2019-11-20T21:03:31.7348123Z    --> src/libcore/mem/maybe_uninit.rs:486:9
2019-11-20T21:03:31.7348362Z     |
2019-11-20T21:03:31.7348844Z 486 |         intrinsics::panic_if_uninhabited::<T>();
2019-11-20T21:03:31.7349526Z     |
2019-11-20T21:03:31.7349526Z     |
2019-11-20T21:03:31.7349927Z     = note: for more information, see issue ***/issues/57563
2019-11-20T21:03:31.7350231Z     = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-11-20T21:03:33.1160414Z    Compiling backtrace-sys v0.1.32
2019-11-20T21:03:34.0397143Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-11-20T21:03:34.8493342Z error: aborting due to previous error
2019-11-20T21:03:34.8493452Z 
---
2019-11-20T21:03:35.2145066Z   local time: Wed Nov 20 21:03:35 UTC 2019
2019-11-20T21:03:35.2992865Z   network time: Wed, 20 Nov 2019 21:03:35 GMT
2019-11-20T21:03:35.2997157Z == end clock drift check ==
2019-11-20T21:03:36.7609417Z 
2019-11-20T21:03:36.7715121Z ##[error]Bash exited with code '1'.
2019-11-20T21:03:36.7745012Z ##[section]Starting: Checkout
2019-11-20T21:03:36.7747055Z ==============================================================================
2019-11-20T21:03:36.7747113Z Task         : Get sources
2019-11-20T21:03:36.7747194Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
