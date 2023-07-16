plain
2019-12-09T01:52:18.7214855Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-09T01:52:18.7231965Z ##[command]git config gc.auto 0
2019-12-09T01:52:18.7242752Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-09T01:52:18.7246945Z ##[command]git config --get-all http.proxy
2019-12-09T01:52:18.7255893Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67116/merge:refs/remotes/pull/67116/merge
---
2019-12-09T01:59:29.6239346Z 
2019-12-09T01:59:29.6239590Z error: variant is never constructed: `Other`
2019-12-09T01:59:29.6239867Z   --> src/librustc/traits/error_reporting.rs:50:5
2019-12-09T01:59:29.6240188Z    |
2019-12-09T01:59:29.6240563Z 50 |     Other(&'a dyn fmt::Display)
2019-12-09T01:59:29.6240882Z 
2019-12-09T01:59:30.5868872Z error: aborting due to 2 previous errors
2019-12-09T01:59:30.5868979Z 
2019-12-09T01:59:30.8270990Z error: could not compile `rustc`.
---
2019-12-09T01:59:30.8387271Z   local time: Mon Dec  9 01:59:30 UTC 2019
2019-12-09T01:59:31.3571903Z   network time: Mon, 09 Dec 2019 01:59:31 GMT
2019-12-09T01:59:31.3572623Z == end clock drift check ==
2019-12-09T01:59:31.9436041Z 
2019-12-09T01:59:31.9537101Z ##[error]Bash exited with code '1'.
2019-12-09T01:59:31.9582920Z ##[section]Starting: Checkout
2019-12-09T01:59:31.9584588Z ==============================================================================
2019-12-09T01:59:31.9584638Z Task         : Get sources
2019-12-09T01:59:31.9584681Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
