plain
2019-10-04T20:30:12.6914427Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-04T20:30:12.7150801Z ##[command]git config gc.auto 0
2019-10-04T20:30:12.7250151Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-04T20:30:12.7365531Z ##[command]git config --get-all http.proxy
2019-10-04T20:30:12.7519436Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65112/merge:refs/remotes/pull/65112/merge
---
2019-10-04T21:02:25.9996977Z    Compiling hashbrown v0.5.0
2019-10-04T21:02:27.9570623Z error: unnecessary parentheses around type
2019-10-04T21:02:27.9575933Z     --> src/libstd/collections/hash/map.rs:1821:34
2019-10-04T21:02:27.9576791Z      |
2019-10-04T21:02:27.9577477Z 1821 |     fn next(&mut self) -> Option<(&'a K)> {
2019-10-04T21:02:27.9578361Z      |                                  ^^^^^^^ help: remove these parentheses
2019-10-04T21:02:27.9579967Z      = note: `-D unused-parens` implied by `-D warnings`
2019-10-04T21:02:27.9580186Z 
2019-10-04T21:02:27.9580818Z error: unnecessary parentheses around type
2019-10-04T21:02:27.9581391Z     --> src/libstd/collections/hash/map.rs:1844:34
2019-10-04T21:02:27.9581391Z     --> src/libstd/collections/hash/map.rs:1844:34
2019-10-04T21:02:27.9581799Z      |
2019-10-04T21:02:27.9582417Z 1844 |     fn next(&mut self) -> Option<(&'a V)> {
2019-10-04T21:02:27.9582976Z      |                                  ^^^^^^^ help: remove these parentheses
2019-10-04T21:02:27.9583614Z error: unnecessary parentheses around type
2019-10-04T21:02:27.9584059Z     --> src/libstd/collections/hash/map.rs:1867:34
2019-10-04T21:02:27.9584656Z      |
2019-10-04T21:02:27.9584656Z      |
2019-10-04T21:02:27.9585170Z 1867 |     fn next(&mut self) -> Option<(&'a mut V)> {
2019-10-04T21:02:27.9585749Z      |                                  ^^^^^^^^^^^ help: remove these parentheses
2019-10-04T21:02:30.4229412Z error: aborting due to 3 previous errors
2019-10-04T21:02:30.4230314Z 
2019-10-04T21:02:30.4890829Z error: could not compile `std`.
2019-10-04T21:02:30.4892149Z 
---
2019-10-04T21:02:30.5001794Z == clock drift check ==
2019-10-04T21:02:30.5019168Z   local time: Fri Oct  4 21:02:30 UTC 2019
2019-10-04T21:02:30.6665126Z   network time: Fri, 04 Oct 2019 21:02:30 GMT
2019-10-04T21:02:30.6669901Z == end clock drift check ==
2019-10-04T21:02:33.4173755Z ##[error]Bash exited with code '1'.
2019-10-04T21:02:33.4228261Z ##[section]Starting: Checkout
2019-10-04T21:02:33.4230257Z ==============================================================================
2019-10-04T21:02:33.4230308Z Task         : Get sources
2019-10-04T21:02:33.4230367Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
