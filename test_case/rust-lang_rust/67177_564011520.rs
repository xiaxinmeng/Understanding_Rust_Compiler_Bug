plain
2019-12-10T12:23:49.9762269Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-10T12:23:49.9775543Z ##[command]git config gc.auto 0
2019-12-10T12:23:49.9791473Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-10T12:23:49.9796219Z ##[command]git config --get-all http.proxy
2019-12-10T12:23:49.9809882Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67177/merge:refs/remotes/pull/67177/merge
---
2019-12-10T12:29:16.6894693Z     Checking term v0.0.0 (/checkout/src/libterm)
2019-12-10T12:29:16.8341870Z error[E0658]: use of unstable library feature 'result_map_or'
2019-12-10T12:29:16.8343108Z   --> src/libterm/terminfo/mod.rs:80:50
2019-12-10T12:29:16.8344361Z    |
2019-12-10T12:29:16.8345130Z 80 |         if term.is_err() && (env::var("MSYSCON").map_or(false, |s| "mintty.exe" == s) ||
2019-12-10T12:29:16.8346396Z    |
2019-12-10T12:29:16.8346396Z    |
2019-12-10T12:29:16.8347121Z    = note: for more information, see ***/issues/66293
2019-12-10T12:29:16.8347862Z    = help: add `#![feature(result_map_or)]` to the crate attributes to enable
2019-12-10T12:29:16.9239554Z error: aborting due to previous error
2019-12-10T12:29:16.9240452Z 
2019-12-10T12:29:16.9241555Z For more information about this error, try `rustc --explain E0658`.
2019-12-10T12:29:16.9317973Z error: could not compile `term`.
---
2019-12-10T12:29:19.6323376Z   local time: Tue Dec 10 12:29:19 UTC 2019
2019-12-10T12:29:19.6926983Z   network time: Tue, 10 Dec 2019 12:29:19 GMT
2019-12-10T12:29:19.6930900Z == end clock drift check ==
2019-12-10T12:29:26.0562927Z 
2019-12-10T12:29:26.0664145Z ##[error]Bash exited with code '1'.
2019-12-10T12:29:26.0694473Z ##[section]Starting: Checkout
2019-12-10T12:29:26.0697040Z ==============================================================================
2019-12-10T12:29:26.0699211Z Task         : Get sources
2019-12-10T12:29:26.0699325Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
