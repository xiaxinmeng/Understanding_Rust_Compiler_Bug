plain
2019-11-09T20:29:08.2202154Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-09T20:29:08.2420625Z ##[command]git config gc.auto 0
2019-11-09T20:29:08.2510198Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-09T20:29:08.2587001Z ##[command]git config --get-all http.proxy
2019-11-09T20:29:08.2728814Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66256/merge:refs/remotes/pull/66256/merge
---
2019-11-09T20:36:13.9649116Z    Compiling rustc-demangle v0.1.16
2019-11-09T20:36:14.8679932Z error[E0599]: no method named `unwrap` found for type `core::alloc::Layout` in the current scope
2019-11-09T20:36:14.8680597Z    --> src/liballoc/sync.rs:754:29
2019-11-09T20:36:14.8680919Z     |
2019-11-09T20:36:14.8681206Z 754 |             .pad_to_align().unwrap();
2019-11-09T20:36:14.8681556Z     |                             ^^^^^^ method not found in `core::alloc::Layout`
2019-11-09T20:36:14.9143684Z error[E0599]: no method named `unwrap` found for type `core::alloc::Layout` in the current scope
2019-11-09T20:36:14.9144098Z    --> src/liballoc/rc.rs:900:29
2019-11-09T20:36:14.9144395Z     |
2019-11-09T20:36:14.9144395Z     |
2019-11-09T20:36:14.9144682Z 900 |             .pad_to_align().unwrap();
2019-11-09T20:36:14.9145033Z     |                             ^^^^^^ method not found in `core::alloc::Layout`
2019-11-09T20:36:15.2176072Z error: aborting due to 2 previous errors
2019-11-09T20:36:15.2181085Z 
2019-11-09T20:36:15.2187969Z For more information about this error, try `rustc --explain E0599`.
2019-11-09T20:36:15.2338393Z error: could not compile `alloc`.
---
2019-11-09T20:36:15.4590818Z   local time: Sat Nov  9 20:36:15 UTC 2019
2019-11-09T20:36:15.6100429Z   network time: Sat, 09 Nov 2019 20:36:15 GMT
2019-11-09T20:36:15.6108630Z == end clock drift check ==
2019-11-09T20:36:18.1622464Z 
2019-11-09T20:36:18.1713941Z ##[error]Bash exited with code '1'.
2019-11-09T20:36:18.1743019Z ##[section]Starting: Checkout
2019-11-09T20:36:18.1744833Z ==============================================================================
2019-11-09T20:36:18.1744893Z Task         : Get sources
2019-11-09T20:36:18.1744944Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
