plain
2020-01-14T21:48:23.5765249Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-14T21:48:23.5862894Z ##[command]git config gc.auto 0
2020-01-14T21:48:23.6124815Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-14T21:48:23.6198665Z ##[command]git config --get-all http.proxy
2020-01-14T21:48:23.6353326Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68234/merge:refs/remotes/pull/68234/merge
---
2020-01-14T21:53:12.4201121Z   local time: Tue Jan 14 21:53:12 UTC 2020
2020-01-14T21:53:12.6971069Z   network time: Tue, 14 Jan 2020 21:53:12 GMT
2020-01-14T21:53:12.6975506Z == end clock drift check ==
2020-01-14T21:53:13.6312567Z 
2020-01-14T21:53:13.6419152Z ##[error]Bash exited with code '1'.
2020-01-14T21:53:13.6450036Z ##[section]Starting: Checkout
2020-01-14T21:53:13.6452827Z ==============================================================================
2020-01-14T21:53:13.6452883Z Task         : Get sources
2020-01-14T21:53:13.6452928Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
