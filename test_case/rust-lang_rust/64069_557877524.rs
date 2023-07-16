plain
2019-11-24T10:50:46.0052387Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-24T10:50:46.0174410Z ##[command]git config gc.auto 0
2019-11-24T10:50:46.0176899Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-24T10:50:46.0178931Z ##[command]git config --get-all http.proxy
2019-11-24T10:50:47.0018209Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64069/merge:refs/remotes/pull/64069/merge
---
2019-11-24T10:57:09.9272732Z    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2019-11-24T10:57:10.1196925Z    Compiling backtrace v0.3.40
2019-11-24T10:57:11.0329609Z    Compiling rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-11-24T10:57:11.1416434Z    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-11-24T10:57:13.7666936Z error[E0658]: use of unstable library feature 'vec_into_raw_parts': new API
2019-11-24T10:57:13.7667635Z     |
2019-11-24T10:57:13.7667635Z     |
2019-11-24T10:57:13.7668012Z 764 |             let (ptr, len, cap): (*mut NonZeroU8, _, _) = Vec::into_raw_parts(v);
2019-11-24T10:57:13.7668689Z     |
2019-11-24T10:57:13.7668689Z     |
2019-11-24T10:57:13.7669149Z     = note: for more information, see ***/issues/65816
2019-11-24T10:57:13.7669541Z     = help: add `#![feature(vec_into_raw_parts)]` to the crate attributes to enable
2019-11-24T10:57:14.8472199Z error: aborting due to previous error
2019-11-24T10:57:14.8472317Z 
2019-11-24T10:57:14.8472712Z For more information about this error, try `rustc --explain E0658`.
2019-11-24T10:57:14.8852457Z error: could not compile `std`.
---
2019-11-24T10:57:14.8943158Z   local time: Sun Nov 24 10:57:14 UTC 2019
2019-11-24T10:57:15.0395919Z   network time: Sun, 24 Nov 2019 10:57:15 GMT
2019-11-24T10:57:15.0397650Z == end clock drift check ==
2019-11-24T10:57:17.9402685Z 
2019-11-24T10:57:17.9505391Z ##[error]Bash exited with code '1'.
2019-11-24T10:57:17.9534314Z ##[section]Starting: Checkout
2019-11-24T10:57:17.9536204Z ==============================================================================
2019-11-24T10:57:17.9536270Z Task         : Get sources
2019-11-24T10:57:17.9536324Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
