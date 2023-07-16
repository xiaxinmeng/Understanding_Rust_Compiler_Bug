plain
2019-12-16T17:33:59.2614354Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-16T17:33:59.2633102Z ##[command]git config gc.auto 0
2019-12-16T17:33:59.2642389Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-16T17:33:59.2646506Z ##[command]git config --get-all http.proxy
2019-12-16T17:33:59.2648793Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67163/merge:refs/remotes/pull/67163/merge
---
2019-12-16T18:04:05.3629797Z   local time: Mon Dec 16 18:04:05 UTC 2019
2019-12-16T18:04:05.6616715Z   network time: Mon, 16 Dec 2019 18:04:05 GMT
2019-12-16T18:04:05.6620030Z == end clock drift check ==
2019-12-16T18:04:08.1561764Z 
2019-12-16T18:04:08.1685963Z ##[error]Bash exited with code '1'.
2019-12-16T18:04:08.1719500Z ##[section]Starting: Checkout
2019-12-16T18:04:08.1721912Z ==============================================================================
2019-12-16T18:04:08.1721971Z Task         : Get sources
2019-12-16T18:04:08.1722037Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
