plain
2019-10-01T11:53:46.4363427Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-01T11:53:46.4558634Z ##[command]git config gc.auto 0
2019-10-01T11:53:46.4640591Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-01T11:53:46.4700890Z ##[command]git config --get-all http.proxy
2019-10-01T11:53:46.4837736Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64951/merge:refs/remotes/pull/64951/merge
---
2019-10-01T12:03:19.5977860Z == clock drift check ==
2019-10-01T12:03:19.5996191Z   local time: Tue Oct  1 12:03:19 UTC 2019
2019-10-01T12:03:19.7672158Z   network time: Tue, 01 Oct 2019 12:03:19 GMT
2019-10-01T12:03:19.7675407Z == end clock drift check ==
2019-10-01T12:03:20.9192880Z ##[error]Bash exited with code '1'.
2019-10-01T12:03:20.9241545Z ##[section]Starting: Checkout
2019-10-01T12:03:20.9243899Z ==============================================================================
2019-10-01T12:03:20.9243963Z Task         : Get sources
2019-10-01T12:03:20.9244018Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
