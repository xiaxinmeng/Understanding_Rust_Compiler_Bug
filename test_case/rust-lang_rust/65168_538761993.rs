plain
2019-10-06T16:03:02.5739388Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-06T16:03:02.5933040Z ##[command]git config gc.auto 0
2019-10-06T16:03:02.6020459Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-06T16:03:02.6091187Z ##[command]git config --get-all http.proxy
2019-10-06T16:03:02.6262557Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65168/merge:refs/remotes/pull/65168/merge
---
2019-10-06T16:09:11.8655881Z == clock drift check ==
2019-10-06T16:09:11.8681260Z   local time: Sun Oct  6 16:09:11 UTC 2019
2019-10-06T16:09:12.0170843Z   network time: Sun, 06 Oct 2019 16:09:12 GMT
2019-10-06T16:09:12.0176090Z == end clock drift check ==
2019-10-06T16:09:20.4458206Z ##[error]Bash exited with code '1'.
2019-10-06T16:09:20.4526620Z ##[section]Starting: Checkout
2019-10-06T16:09:20.4528609Z ==============================================================================
2019-10-06T16:09:20.4528667Z Task         : Get sources
2019-10-06T16:09:20.4528735Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
