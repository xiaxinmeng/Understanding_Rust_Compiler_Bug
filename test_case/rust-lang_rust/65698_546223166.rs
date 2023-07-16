plain
2019-10-25T06:21:40.4132676Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-25T06:21:40.4353147Z ##[command]git config gc.auto 0
2019-10-25T06:21:40.4438961Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-25T06:21:40.4500613Z ##[command]git config --get-all http.proxy
2019-10-25T06:21:40.4630692Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65698/merge:refs/remotes/pull/65698/merge
---
2019-10-25T06:39:01.8104364Z   local time: Fri Oct 25 06:39:01 UTC 2019
2019-10-25T06:39:02.0900837Z   network time: Fri, 25 Oct 2019 06:39:02 GMT
2019-10-25T06:39:02.0900958Z == end clock drift check ==
2019-10-25T06:39:05.2421460Z 
2019-10-25T06:39:05.2522640Z ##[error]Bash exited with code '1'.
2019-10-25T06:39:05.2616878Z ##[section]Starting: Checkout
2019-10-25T06:39:05.2618735Z ==============================================================================
2019-10-25T06:39:05.2618781Z Task         : Get sources
2019-10-25T06:39:05.2618822Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
