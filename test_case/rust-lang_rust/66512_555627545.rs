plain
2019-11-19T17:44:20.9231282Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-19T17:44:20.9436633Z ##[command]git config gc.auto 0
2019-11-19T17:44:20.9500930Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-19T17:44:20.9550485Z ##[command]git config --get-all http.proxy
2019-11-19T17:44:20.9711268Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66512/merge:refs/remotes/pull/66512/merge
---
2019-11-19T17:51:21.2813698Z   local time: Tue Nov 19 17:51:21 UTC 2019
2019-11-19T17:51:21.5583602Z   network time: Tue, 19 Nov 2019 17:51:21 GMT
2019-11-19T17:51:21.5588291Z == end clock drift check ==
2019-11-19T17:51:24.4697678Z 
2019-11-19T17:51:24.4817011Z ##[error]Bash exited with code '1'.
2019-11-19T17:51:24.4846772Z ##[section]Starting: Checkout
2019-11-19T17:51:24.4860165Z ==============================================================================
2019-11-19T17:51:24.4860243Z Task         : Get sources
2019-11-19T17:51:24.4860297Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
