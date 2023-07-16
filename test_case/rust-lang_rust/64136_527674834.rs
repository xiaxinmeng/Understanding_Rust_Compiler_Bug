plain
2019-09-03T23:01:50.0551941Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-03T23:01:50.0737230Z ##[command]git config gc.auto 0
2019-09-03T23:01:50.0822149Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-03T23:01:50.0911783Z ##[command]git config --get-all http.proxy
2019-09-03T23:01:50.1057734Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64136/merge:refs/remotes/pull/64136/merge
---
2019-09-03T23:10:06.9562794Z == clock drift check ==
2019-09-03T23:10:06.9581515Z   local time: Tue Sep  3 23:10:06 UTC 2019
2019-09-03T23:10:07.0303508Z   network time: Tue, 03 Sep 2019 23:10:07 GMT
2019-09-03T23:10:07.0309235Z == end clock drift check ==
2019-09-03T23:10:08.3394067Z ##[error]Bash exited with code '1'.
2019-09-03T23:10:08.3429595Z ##[section]Starting: Checkout
2019-09-03T23:10:08.3431595Z ==============================================================================
2019-09-03T23:10:08.3431673Z Task         : Get sources
2019-09-03T23:10:08.3431727Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
