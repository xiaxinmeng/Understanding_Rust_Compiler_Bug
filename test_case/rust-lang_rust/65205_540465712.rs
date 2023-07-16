plain
2019-10-10T08:23:08.3588408Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-10T08:23:08.3832993Z ##[command]git config gc.auto 0
2019-10-10T08:23:08.3952531Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-10T08:23:08.4017511Z ##[command]git config --get-all http.proxy
2019-10-10T08:23:08.4192265Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65205/merge:refs/remotes/pull/65205/merge
---
2019-10-10T08:32:14.5829314Z == clock drift check ==
2019-10-10T08:32:14.5850600Z   local time: Thu Oct 10 08:32:14 UTC 2019
2019-10-10T08:32:14.8624820Z   network time: Thu, 10 Oct 2019 08:32:14 GMT
2019-10-10T08:32:14.8632746Z == end clock drift check ==
2019-10-10T08:32:15.9193937Z ##[error]Bash exited with code '1'.
2019-10-10T08:32:15.9236164Z ##[section]Starting: Checkout
2019-10-10T08:32:15.9238651Z ==============================================================================
2019-10-10T08:32:15.9238760Z Task         : Get sources
2019-10-10T08:32:15.9238812Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
