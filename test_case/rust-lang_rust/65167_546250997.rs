plain
2019-10-25T08:05:01.3958918Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-25T08:05:01.4187609Z ##[command]git config gc.auto 0
2019-10-25T08:05:01.4288355Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-25T08:05:01.4375625Z ##[command]git config --get-all http.proxy
2019-10-25T08:05:01.4530746Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65167/merge:refs/remotes/pull/65167/merge
---
2019-10-25T08:10:01.4927877Z   local time: Fri Oct 25 08:10:01 UTC 2019
2019-10-25T08:10:01.5791570Z   network time: Fri, 25 Oct 2019 08:10:01 GMT
2019-10-25T08:10:01.5795422Z == end clock drift check ==
2019-10-25T08:10:10.7916282Z 
2019-10-25T08:10:10.8024740Z ##[error]Bash exited with code '1'.
2019-10-25T08:10:10.8062258Z ##[section]Starting: Checkout
2019-10-25T08:10:10.8063981Z ==============================================================================
2019-10-25T08:10:10.8064041Z Task         : Get sources
2019-10-25T08:10:10.8064111Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
