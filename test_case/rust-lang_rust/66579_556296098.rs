plain
2019-11-20T19:33:30.0604629Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-20T19:33:30.0801818Z ##[command]git config gc.auto 0
2019-11-20T19:33:30.0881227Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-20T19:33:30.0944126Z ##[command]git config --get-all http.proxy
2019-11-20T19:33:30.1105196Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66579/merge:refs/remotes/pull/66579/merge
---
2019-11-20T19:39:40.6556214Z    Compiling compiler_builtins v0.1.18
2019-11-20T19:39:41.1987352Z error[E0723]: can only call a curated list of intrinsics in `min_const_fn`
2019-11-20T19:39:41.1988566Z    --> src/libcore/mem/maybe_uninit.rs:486:9
2019-11-20T19:39:41.1989148Z     |
2019-11-20T19:39:41.1990287Z 486 |         crate::intrinsics::panic_if_uninhabited::<T>();
2019-11-20T19:39:41.1991212Z     |
2019-11-20T19:39:41.1991212Z     |
2019-11-20T19:39:41.1991691Z     = note: for more information, see issue ***/issues/57563
2019-11-20T19:39:41.1992039Z     = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-11-20T19:39:42.4719690Z    Compiling backtrace-sys v0.1.32
2019-11-20T19:39:43.3982407Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-11-20T19:39:44.2896352Z error: aborting due to previous error
2019-11-20T19:39:44.2900284Z 
---
2019-11-20T19:39:44.5990524Z   local time: Wed Nov 20 19:39:44 UTC 2019
2019-11-20T19:39:44.8821771Z   network time: Wed, 20 Nov 2019 19:39:44 GMT
2019-11-20T19:39:44.8821907Z == end clock drift check ==
2019-11-20T19:39:46.3444051Z 
2019-11-20T19:39:46.3553655Z ##[error]Bash exited with code '1'.
2019-11-20T19:39:46.3584559Z ##[section]Starting: Checkout
2019-11-20T19:39:46.3586365Z ==============================================================================
2019-11-20T19:39:46.3586448Z Task         : Get sources
2019-11-20T19:39:46.3586503Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
