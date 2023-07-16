plain
2019-12-01T23:25:54.9527178Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-01T23:25:54.9745700Z ##[command]git config gc.auto 0
2019-12-01T23:25:54.9839083Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-01T23:25:54.9887295Z ##[command]git config --get-all http.proxy
2019-12-01T23:25:55.6248271Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65572/merge:refs/remotes/pull/65572/merge
---
2019-12-01T23:43:24.2227497Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-12-01T23:43:25.5751085Z error[E0425]: cannot find value `statement` in this scope
2019-12-01T23:43:25.5751502Z    --> src/librustc_mir/transform/qualify_min_const_fn.rs:328:21
2019-12-01T23:43:25.5751794Z     |
2019-12-01T23:43:25.5752093Z 328 |         self.span = statement.source_info.span;
2019-12-01T23:43:25.5758971Z 
2019-12-01T23:43:35.2261332Z error: aborting due to previous error
2019-12-01T23:43:35.2266968Z 
2019-12-01T23:43:35.2278853Z For more information about this error, try `rustc --explain E0425`.
---
2019-12-01T23:45:31.3665560Z   local time: Sun Dec  1 23:45:31 UTC 2019
2019-12-01T23:45:31.6547034Z   network time: Sun, 01 Dec 2019 23:45:31 GMT
2019-12-01T23:45:31.6548201Z == end clock drift check ==
2019-12-01T23:45:34.5992117Z 
2019-12-01T23:45:34.6105702Z ##[error]Bash exited with code '1'.
2019-12-01T23:45:34.6143364Z ##[section]Starting: Checkout
2019-12-01T23:45:34.6145275Z ==============================================================================
2019-12-01T23:45:34.6145352Z Task         : Get sources
2019-12-01T23:45:34.6145404Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
