plain
2019-12-01T21:07:09.2556946Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-01T21:07:09.2873430Z ##[command]git config gc.auto 0
2019-12-01T21:07:09.2937655Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-01T21:07:09.2998231Z ##[command]git config --get-all http.proxy
2019-12-01T21:07:09.3153989Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66884/merge:refs/remotes/pull/66884/merge
---
2019-12-01T21:36:12.4148554Z 10  | / macro_rules! panic {
2019-12-01T21:36:12.4149349Z 11  | |     () => (
2019-12-01T21:36:12.4149684Z 12  | |         $crate::panic!("explicit panic")
2019-12-01T21:36:12.4150001Z 13  | |     );
2019-12-01T21:36:12.4150572Z 14  | |     ($msg:expr) => (
2019-12-01T21:36:12.4150930Z 15  | |         $crate::panicking::panic($msg, $crate::intrinsics::caller_location())
2019-12-01T21:36:12.4151580Z ...   |
2019-12-01T21:36:12.4151879Z 25  | |     );
2019-12-01T21:36:12.4152367Z 26  | | }
2019-12-01T21:36:12.4152626Z     | |_- in this expansion of `panic!`
2019-12-01T21:36:12.4152626Z     | |_- in this expansion of `panic!`
2019-12-01T21:36:12.4156231Z     | 
2019-12-01T21:36:12.4156528Z    ::: src/libcore/option.rs:475:21
2019-12-01T21:36:12.4156735Z     |
2019-12-01T21:36:12.4157197Z 475 |               None => panic!("called `Option::unwrap()` on a `None` value"),
2019-12-01T21:36:12.4157625Z     |                       ----------------------------------------------------- in this macro invocation
2019-12-01T21:36:12.4157865Z     |
2019-12-01T21:36:12.4158312Z     = note: for more information, see issue ***/issues/57563
2019-12-01T21:36:12.4158655Z     = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:36:12.5222143Z    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
2019-12-01T21:36:12.9957268Z    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
2019-12-01T21:36:14.0292097Z    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
2019-12-01T21:36:14.8437358Z error: aborting due to previous error
---
2019-12-01T21:36:18.9700762Z   local time: Sun Dec  1 21:36:18 UTC 2019
2019-12-01T21:36:19.5071761Z   network time: Sun, 01 Dec 2019 21:36:19 GMT
2019-12-01T21:36:19.5073664Z == end clock drift check ==
2019-12-01T21:36:20.9407958Z 
2019-12-01T21:36:20.9528652Z ##[error]Bash exited with code '1'.
2019-12-01T21:36:20.9569279Z ##[section]Starting: Checkout
2019-12-01T21:36:20.9571507Z ==============================================================================
2019-12-01T21:36:20.9571591Z Task         : Get sources
2019-12-01T21:36:20.9571644Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
