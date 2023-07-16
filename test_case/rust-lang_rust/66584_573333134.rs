plain
2020-01-11T16:41:58.1082254Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-11T16:41:58.1092634Z ##[command]git config gc.auto 0
2020-01-11T16:41:58.1095047Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-11T16:41:58.1097051Z ##[command]git config --get-all http.proxy
2020-01-11T16:41:58.1099683Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66584/merge:refs/remotes/pull/66584/merge
---
2020-01-11T16:47:14.0226461Z 
2020-01-11T16:47:14.0231668Z error: missing documentation for a variant
2020-01-11T16:47:14.0233103Z    --> src/libstd/net/ip.rs:128:5
2020-01-11T16:47:14.0233741Z     |
2020-01-11T16:47:14.0234282Z 128 |     RealmLocal,
2020-01-11T16:47:14.0234944Z 
2020-01-11T16:47:14.0242248Z error: missing documentation for a variant
2020-01-11T16:47:14.0242835Z    --> src/libstd/net/ip.rs:130:5
2020-01-11T16:47:14.0243286Z     |
2020-01-11T16:47:14.0243286Z     |
2020-01-11T16:47:14.0243789Z 130 |     AdminLocal,
2020-01-11T16:47:14.0244492Z 
2020-01-11T16:47:14.0250660Z error: missing documentation for a variant
2020-01-11T16:47:14.0251237Z    --> src/libstd/net/ip.rs:132:5
2020-01-11T16:47:14.0251679Z     |
---
2020-01-11T16:47:15.1851148Z   local time: Sat Jan 11 16:47:14 UTC 2020
2020-01-11T16:47:15.1851302Z   network time: Sat, 11 Jan 2020 16:47:14 GMT
2020-01-11T16:47:15.1851453Z == end clock drift check ==
2020-01-11T16:47:15.3288271Z 
2020-01-11T16:47:15.3401967Z ##[error]Bash exited with code '1'.
2020-01-11T16:47:15.3438874Z ##[section]Starting: Checkout
2020-01-11T16:47:15.3441041Z ==============================================================================
2020-01-11T16:47:15.3441127Z Task         : Get sources
2020-01-11T16:47:15.3441177Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
