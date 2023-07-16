plain
2019-12-27T17:59:28.2907689Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-27T17:59:28.3105633Z ##[command]git config gc.auto 0
2019-12-27T17:59:28.3190420Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-27T17:59:28.3239923Z ##[command]git config --get-all http.proxy
2019-12-27T17:59:28.3384464Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67666/merge:refs/remotes/pull/67666/merge
---
2019-12-27T18:05:10.7779295Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-12-27T18:05:14.0589541Z error[E0614]: type `bool` cannot be dereferenced
2019-12-27T18:05:14.0590884Z    --> src/libstd/sys/unix/os.rs:483:38
2019-12-27T18:05:14.0591511Z     |
2019-12-27T18:05:14.0592172Z 483 |         while !environ.is_null() && !*environ.is_null() {
2019-12-27T18:05:14.0593172Z 
2019-12-27T18:05:14.1918123Z error: aborting due to previous error
2019-12-27T18:05:14.1919042Z 
2019-12-27T18:05:14.1919713Z For more information about this error, try `rustc --explain E0614`.
---
2019-12-27T18:05:14.2169836Z   local time: Fri Dec 27 18:05:14 UTC 2019
2019-12-27T18:05:14.5116008Z   network time: Fri, 27 Dec 2019 18:05:14 GMT
2019-12-27T18:05:14.5121499Z == end clock drift check ==
2019-12-27T18:05:22.6826003Z 
2019-12-27T18:05:22.6950794Z ##[error]Bash exited with code '1'.
2019-12-27T18:05:22.6983995Z ##[section]Starting: Checkout
2019-12-27T18:05:22.6986033Z ==============================================================================
2019-12-27T18:05:22.6986123Z Task         : Get sources
2019-12-27T18:05:22.6986176Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
