plain
2019-11-20T19:44:40.4053842Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-20T19:44:40.4264116Z ##[command]git config gc.auto 0
2019-11-20T19:44:40.4331259Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-20T19:44:40.4392000Z ##[command]git config --get-all http.proxy
2019-11-20T19:44:40.4539917Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66579/merge:refs/remotes/pull/66579/merge
---
2019-11-20T19:50:57.8110344Z    Compiling compiler_builtins v0.1.18
2019-11-20T19:50:58.1249053Z error[E0723]: can only call a curated list of intrinsics in `min_const_fn`
2019-11-20T19:50:58.1249507Z    --> src/libcore/mem/maybe_uninit.rs:486:9
2019-11-20T19:50:58.1249771Z     |
2019-11-20T19:50:58.1250071Z 486 |         crate::intrinsics::panic_if_uninhabited::<T>();
2019-11-20T19:50:58.1250641Z     |
2019-11-20T19:50:58.1250641Z     |
2019-11-20T19:50:58.1251048Z     = note: for more information, see issue ***/issues/57563
2019-11-20T19:50:58.1251389Z     = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-11-20T19:50:59.7159671Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-11-20T19:51:00.5870125Z    Compiling backtrace-sys v0.1.32
2019-11-20T19:51:01.2403933Z error: aborting due to previous error
2019-11-20T19:51:01.2404086Z 
---
2019-11-20T19:51:01.5259532Z   local time: Wed Nov 20 19:51:01 UTC 2019
2019-11-20T19:51:01.8036378Z   network time: Wed, 20 Nov 2019 19:51:01 GMT
2019-11-20T19:51:01.8036470Z == end clock drift check ==
2019-11-20T19:51:03.1556403Z 
2019-11-20T19:51:03.1655305Z ##[error]Bash exited with code '1'.
2019-11-20T19:51:03.1684043Z ##[section]Starting: Checkout
2019-11-20T19:51:03.1685566Z ==============================================================================
2019-11-20T19:51:03.1685617Z Task         : Get sources
2019-11-20T19:51:03.1685676Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
