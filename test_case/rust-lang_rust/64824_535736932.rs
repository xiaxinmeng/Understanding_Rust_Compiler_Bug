plain
2019-09-26T23:03:21.6935439Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-26T23:03:21.7129361Z ##[command]git config gc.auto 0
2019-09-26T23:03:21.7211949Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-26T23:03:21.7275686Z ##[command]git config --get-all http.proxy
2019-09-26T23:03:21.7426033Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64824/merge:refs/remotes/pull/64824/merge
---
2019-09-26T23:11:42.7170192Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-09-26T23:11:44.3071548Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-09-26T23:11:45.6786927Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-09-26T23:12:00.0368085Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-09-26T23:12:25.6439458Z error[E0107]: wrong number of type arguments: expected 0, found 1
2019-09-26T23:12:25.6440916Z    --> src/librustc/ty/query/job.rs:337:48
2019-09-26T23:12:25.6441558Z     |
2019-09-26T23:12:25.6442137Z 337 |         let mut stable_hasher = StableHasher::<u64>::new();
2019-09-26T23:12:25.6442844Z     |                                                ^^^ unexpected type argument
2019-09-26T23:12:28.0607678Z error: aborting due to previous error
2019-09-26T23:12:28.0608060Z 
2019-09-26T23:12:28.0608564Z For more information about this error, try `rustc --explain E0107`.
2019-09-26T23:12:28.2436673Z error: could not compile `rustc`.
---
2019-09-26T23:12:28.2527984Z == clock drift check ==
2019-09-26T23:12:28.2542920Z   local time: Thu Sep 26 23:12:28 UTC 2019
2019-09-26T23:12:28.4023303Z   network time: Thu, 26 Sep 2019 23:12:28 GMT
2019-09-26T23:12:28.4023713Z == end clock drift check ==
2019-09-26T23:12:29.0329010Z ##[error]Bash exited with code '1'.
2019-09-26T23:12:29.0374343Z ##[section]Starting: Checkout
2019-09-26T23:12:29.0376334Z ==============================================================================
2019-09-26T23:12:29.0376412Z Task         : Get sources
2019-09-26T23:12:29.0376456Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
