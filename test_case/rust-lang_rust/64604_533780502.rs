plain
2019-09-21T08:38:12.4806183Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-21T08:38:13.4357320Z ##[command]git config gc.auto 0
2019-09-21T08:38:13.4360686Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-21T08:38:13.4377539Z ##[command]git config --get-all http.proxy
2019-09-21T08:38:13.4383874Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64604/merge:refs/remotes/pull/64604/merge
---
2019-09-21T08:42:30.3576594Z == clock drift check ==
2019-09-21T08:42:30.3594986Z   local time: Sat Sep 21 08:42:30 UTC 2019
2019-09-21T08:42:30.6379867Z   network time: Sat, 21 Sep 2019 08:42:30 GMT
2019-09-21T08:42:30.6380778Z == end clock drift check ==
2019-09-21T08:42:31.7512228Z ##[error]Bash exited with code '1'.
2019-09-21T08:42:31.7546068Z ##[section]Starting: Checkout
2019-09-21T08:42:31.7547917Z ==============================================================================
2019-09-21T08:42:31.7548129Z Task         : Get sources
2019-09-21T08:42:31.7548178Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
