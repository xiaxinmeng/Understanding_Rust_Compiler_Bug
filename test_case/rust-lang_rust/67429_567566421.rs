plain
2019-12-19T16:35:49.0530929Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-19T16:35:49.0543708Z ##[command]git config gc.auto 0
2019-12-19T16:35:49.0548284Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-19T16:35:49.0552631Z ##[command]git config --get-all http.proxy
2019-12-19T16:35:49.0559037Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67429/merge:refs/remotes/pull/67429/merge
---
2019-12-19T16:39:29.7469980Z   local time: Thu Dec 19 16:39:29 UTC 2019
2019-12-19T16:39:30.2667204Z   network time: Thu, 19 Dec 2019 16:39:30 GMT
2019-12-19T16:39:30.2672651Z == end clock drift check ==
2019-12-19T16:39:43.5235272Z 
2019-12-19T16:39:43.5320460Z ##[error]Bash exited with code '1'.
2019-12-19T16:39:43.5347842Z ##[section]Starting: Checkout
2019-12-19T16:39:43.5350637Z ==============================================================================
2019-12-19T16:39:43.5350708Z Task         : Get sources
2019-12-19T16:39:43.5350746Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
