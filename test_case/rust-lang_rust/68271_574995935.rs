plain
2020-01-16T05:51:29.0477458Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-16T05:51:29.0487221Z ##[command]git config gc.auto 0
2020-01-16T05:51:29.0489970Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-16T05:51:29.0491921Z ##[command]git config --get-all http.proxy
2020-01-16T05:51:29.0494697Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68271/merge:refs/remotes/pull/68271/merge
---
2020-01-16T05:54:33.9735624Z   local time: Thu Jan 16 05:54:33 UTC 2020
2020-01-16T05:54:34.0326621Z   network time: Thu, 16 Jan 2020 05:54:34 GMT
2020-01-16T05:54:34.0336429Z == end clock drift check ==
2020-01-16T05:54:43.0777926Z 
2020-01-16T05:54:43.0842190Z ##[error]Bash exited with code '1'.
2020-01-16T05:54:43.0873149Z ##[section]Starting: Checkout
2020-01-16T05:54:43.0875146Z ==============================================================================
2020-01-16T05:54:43.0875245Z Task         : Get sources
2020-01-16T05:54:43.0875299Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
