plain
2019-12-26T17:59:50.3934284Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-26T17:59:50.4235386Z ##[command]git config gc.auto 0
2019-12-26T17:59:50.4316034Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-26T17:59:50.4373937Z ##[command]git config --get-all http.proxy
2019-12-26T17:59:50.4519251Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67637/merge:refs/remotes/pull/67637/merge
---
2019-12-26T18:05:29.3618118Z   local time: Thu Dec 26 18:05:29 UTC 2019
2019-12-26T18:05:29.6521844Z   network time: Thu, 26 Dec 2019 18:05:29 GMT
2019-12-26T18:05:29.6525377Z == end clock drift check ==
2019-12-26T18:05:38.4414663Z 
2019-12-26T18:05:38.4483912Z ##[error]Bash exited with code '1'.
2019-12-26T18:05:38.4514819Z ##[section]Starting: Checkout
2019-12-26T18:05:38.4517037Z ==============================================================================
2019-12-26T18:05:38.4517084Z Task         : Get sources
2019-12-26T18:05:38.4517245Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
