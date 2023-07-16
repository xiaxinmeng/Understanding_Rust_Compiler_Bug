plain
2019-11-20T18:00:06.2030858Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-20T18:00:06.8007333Z ##[command]git config gc.auto 0
2019-11-20T18:00:06.8015273Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-20T18:00:06.8017016Z ##[command]git config --get-all http.proxy
2019-11-20T18:00:06.8020441Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66579/merge:refs/remotes/pull/66579/merge
---
2019-11-20T18:07:30.9287062Z    Compiling compiler_builtins v0.1.18
2019-11-20T18:07:30.9288361Z error[E0723]: can only call a curated list of intrinsics in `min_const_fn`
2019-11-20T18:07:30.9288847Z    --> src/libcore/mem/maybe_uninit.rs:486:9
2019-11-20T18:07:30.9289091Z     |
2019-11-20T18:07:30.9289389Z 486 |         intrinsics::panic_if_uninhabited::<T>();
2019-11-20T18:07:30.9289879Z     |
2019-11-20T18:07:30.9289879Z     |
2019-11-20T18:07:30.9290271Z     = note: for more information, see issue ***/issues/57563
2019-11-20T18:07:30.9290569Z     = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-11-20T18:07:32.1727612Z    Compiling backtrace-sys v0.1.32
2019-11-20T18:07:33.1253742Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-11-20T18:07:33.8961161Z error: aborting due to previous error
2019-11-20T18:07:33.8961300Z 
---
2019-11-20T18:07:34.0670723Z   local time: Wed Nov 20 18:07:34 UTC 2019
2019-11-20T18:07:34.3510655Z   network time: Wed, 20 Nov 2019 18:07:34 GMT
2019-11-20T18:07:34.3510772Z == end clock drift check ==
2019-11-20T18:07:35.7477276Z 
2019-11-20T18:07:35.7539256Z ##[error]Bash exited with code '1'.
2019-11-20T18:07:35.7567545Z ##[section]Starting: Checkout
2019-11-20T18:07:35.7569150Z ==============================================================================
2019-11-20T18:07:35.7569204Z Task         : Get sources
2019-11-20T18:07:35.7569266Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
