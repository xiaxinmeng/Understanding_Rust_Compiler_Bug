plain
2020-01-15T15:12:20.7519762Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-15T15:12:20.7600551Z ##[command]git config gc.auto 0
2020-01-15T15:12:20.7603055Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-15T15:12:20.7605633Z ##[command]git config --get-all http.proxy
2020-01-15T15:12:20.7608028Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68012/merge:refs/remotes/pull/68012/merge
---
2020-01-15T15:15:04.6205665Z   local time: Wed Jan 15 15:15:04 UTC 2020
2020-01-15T15:15:04.6854129Z   network time: Wed, 15 Jan 2020 15:15:04 GMT
2020-01-15T15:15:04.6859738Z == end clock drift check ==
2020-01-15T15:15:13.7843259Z 
2020-01-15T15:15:13.7910684Z ##[error]Bash exited with code '1'.
2020-01-15T15:15:13.7940614Z ##[section]Starting: Checkout
2020-01-15T15:15:13.7943683Z ==============================================================================
2020-01-15T15:15:13.7943741Z Task         : Get sources
2020-01-15T15:15:13.7943841Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
