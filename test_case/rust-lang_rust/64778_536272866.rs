plain
2019-09-29T08:49:56.2165721Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-29T08:49:56.2332005Z ##[command]git config gc.auto 0
2019-09-29T08:49:56.2404237Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-29T08:49:56.2462723Z ##[command]git config --get-all http.proxy
2019-09-29T08:49:56.2586342Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64778/merge:refs/remotes/pull/64778/merge
---
2019-09-29T08:59:07.4151257Z == clock drift check ==
2019-09-29T08:59:07.4170664Z   local time: Sun Sep 29 08:59:07 UTC 2019
2019-09-29T08:59:07.5805883Z   network time: Sun, 29 Sep 2019 08:59:07 GMT
2019-09-29T08:59:07.5808962Z == end clock drift check ==
2019-09-29T08:59:08.4659783Z ##[error]Bash exited with code '1'.
2019-09-29T08:59:08.4706943Z ##[section]Starting: Checkout
2019-09-29T08:59:08.4709466Z ==============================================================================
2019-09-29T08:59:08.4709532Z Task         : Get sources
2019-09-29T08:59:08.4709571Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
