plain
2019-12-04T03:05:47.2327003Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-04T03:05:47.2341672Z ##[command]git config gc.auto 0
2019-12-04T03:05:47.2345969Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-04T03:05:47.2349243Z ##[command]git config --get-all http.proxy
2019-12-04T03:05:47.2424491Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67010/merge:refs/remotes/pull/67010/merge
---
2019-12-04T03:32:15.4353478Z    Compiling core v0.0.0 (/checkout/src/libcore)
2019-12-04T03:32:15.5158176Z error[E0583]: file not found for module `r#try`
2019-12-04T03:32:15.5158806Z    --> src/libcore/ops/mod.rs:155:5
2019-12-04T03:32:15.5159172Z     |
2019-12-04T03:32:15.5159543Z 155 | mod r#try;
2019-12-04T03:32:15.5160300Z     |
2019-12-04T03:32:15.5160300Z     |
2019-12-04T03:32:15.5160734Z     = help: name the file either r#try.rs or r#try/mod.rs inside the directory "src/libcore/ops"
2019-12-04T03:32:15.5165315Z error: aborting due to previous error
2019-12-04T03:32:15.5165388Z 
2019-12-04T03:32:15.5171263Z For more information about this error, try `rustc --explain E0583`.
2019-12-04T03:32:15.5207371Z error: could not compile `core`.
---
2019-12-04T03:32:20.8639789Z   local time: Wed Dec  4 03:32:20 UTC 2019
2019-12-04T03:32:21.1401276Z   network time: Wed, 04 Dec 2019 03:32:21 GMT
2019-12-04T03:32:21.1401938Z == end clock drift check ==
2019-12-04T03:32:21.8512910Z 
2019-12-04T03:32:21.8602453Z ##[error]Bash exited with code '1'.
2019-12-04T03:32:21.8631490Z ##[section]Starting: Checkout
2019-12-04T03:32:21.8632964Z ==============================================================================
2019-12-04T03:32:21.8633154Z Task         : Get sources
2019-12-04T03:32:21.8633688Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
