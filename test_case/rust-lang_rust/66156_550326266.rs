plain
2019-11-06T13:22:36.2928244Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-06T13:22:36.3174122Z ##[command]git config gc.auto 0
2019-11-06T13:22:36.3250158Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-06T13:22:36.3317967Z ##[command]git config --get-all http.proxy
2019-11-06T13:22:36.3440178Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66156/merge:refs/remotes/pull/66156/merge
---
2019-11-06T14:11:35.9231068Z   local time: Wed Nov  6 14:11:35 UTC 2019
2019-11-06T14:11:36.4454033Z   network time: Wed, 06 Nov 2019 14:11:36 GMT
2019-11-06T14:11:36.4457850Z == end clock drift check ==
2019-11-06T14:11:38.1007936Z 
2019-11-06T14:11:38.1069946Z ##[error]Bash exited with code '1'.
2019-11-06T14:11:38.1133488Z ##[section]Starting: Checkout
2019-11-06T14:11:38.1135421Z ==============================================================================
2019-11-06T14:11:38.1135490Z Task         : Get sources
2019-11-06T14:11:38.1135552Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
