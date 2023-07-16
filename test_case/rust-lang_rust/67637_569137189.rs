plain
2019-12-26T21:27:29.1879888Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-26T21:27:29.2070781Z ##[command]git config gc.auto 0
2019-12-26T21:27:29.2174957Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-26T21:27:29.2217172Z ##[command]git config --get-all http.proxy
2019-12-26T21:27:29.2360937Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67637/merge:refs/remotes/pull/67637/merge
---
2019-12-26T21:33:00.8670617Z   local time: Thu Dec 26 21:33:00 UTC 2019
2019-12-26T21:33:01.4019416Z   network time: Thu, 26 Dec 2019 21:33:01 GMT
2019-12-26T21:33:01.4019619Z == end clock drift check ==
2019-12-26T21:33:16.3874912Z 
2019-12-26T21:33:16.3970968Z ##[error]Bash exited with code '1'.
2019-12-26T21:33:16.4029875Z ##[section]Starting: Checkout
2019-12-26T21:33:16.4031550Z ==============================================================================
2019-12-26T21:33:16.4031604Z Task         : Get sources
2019-12-26T21:33:16.4031650Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
