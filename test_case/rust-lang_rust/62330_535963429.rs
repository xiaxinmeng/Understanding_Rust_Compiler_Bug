plain
2019-09-27T14:08:42.8759422Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-27T14:08:42.8949486Z ##[command]git config gc.auto 0
2019-09-27T14:08:42.9041024Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-27T14:08:42.9058638Z ##[command]git config --get-all http.proxy
2019-09-27T14:08:42.9202197Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62330/merge:refs/remotes/pull/62330/merge
---
2019-09-27T14:17:18.2573966Z == clock drift check ==
2019-09-27T14:17:18.2587925Z   local time: Fri Sep 27 14:17:18 UTC 2019
2019-09-27T14:17:18.4272689Z   network time: Fri, 27 Sep 2019 14:17:18 GMT
2019-09-27T14:17:18.4275506Z == end clock drift check ==
2019-09-27T14:17:19.5681473Z ##[error]Bash exited with code '1'.
2019-09-27T14:17:19.5723376Z ##[section]Starting: Checkout
2019-09-27T14:17:19.5726038Z ==============================================================================
2019-09-27T14:17:19.5726080Z Task         : Get sources
2019-09-27T14:17:19.5726116Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
