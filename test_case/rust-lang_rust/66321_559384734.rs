plain
2019-11-28T07:49:09.9688865Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-28T07:49:09.9995641Z ##[command]git config gc.auto 0
2019-11-28T07:49:10.0075532Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-28T07:49:10.0116481Z ##[command]git config --get-all http.proxy
2019-11-28T07:49:10.0255819Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66321/merge:refs/remotes/pull/66321/merge
---
2019-11-28T08:06:00.5660467Z   local time: Thu Nov 28 08:06:00 UTC 2019
2019-11-28T08:06:00.8755932Z   network time: Thu, 28 Nov 2019 08:06:00 GMT
2019-11-28T08:06:00.8758430Z == end clock drift check ==
2019-11-28T08:06:03.7277785Z 
2019-11-28T08:06:03.7419466Z ##[error]Bash exited with code '1'.
2019-11-28T08:06:03.7485719Z ##[section]Starting: Checkout
2019-11-28T08:06:03.7489221Z ==============================================================================
2019-11-28T08:06:03.7489274Z Task         : Get sources
2019-11-28T08:06:03.7489328Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
