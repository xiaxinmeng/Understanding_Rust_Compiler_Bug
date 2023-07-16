plain
2019-09-30T00:41:29.4231557Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-30T00:41:29.4499937Z ##[command]git config gc.auto 0
2019-09-30T00:41:29.4519467Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-30T00:41:29.4589666Z ##[command]git config --get-all http.proxy
2019-09-30T00:41:29.4750692Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64648/merge:refs/remotes/pull/64648/merge
---
2019-09-30T00:49:28.6648043Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-09-30T00:49:28.8054292Z error[E0432]: unresolved import `crate::stable_hasher::StableHashingContext`
2019-09-30T00:49:28.8054814Z  --> src/librustc_data_structures/steal.rs:1:54
2019-09-30T00:49:28.8055143Z   |
2019-09-30T00:49:28.8055538Z 1 | use crate::stable_hasher::{HashStable, StableHasher, StableHashingContext};
2019-09-30T00:49:28.8056038Z   |                                                      ^^^^^^^^^^^^^^^^^^^^ no `StableHashingContext` in `stable_hasher`
2019-09-30T00:49:30.0425665Z error: aborting due to previous error
2019-09-30T00:49:30.0427039Z 
2019-09-30T00:49:30.0436695Z For more information about this error, try `rustc --explain E0432`.
2019-09-30T00:49:30.0629510Z error: could not compile `rustc_data_structures`.
---
2019-09-30T00:49:36.1226437Z == clock drift check ==
2019-09-30T00:49:36.1244554Z   local time: Mon Sep 30 00:49:36 UTC 2019
2019-09-30T00:49:36.2762172Z   network time: Mon, 30 Sep 2019 00:49:36 GMT
2019-09-30T00:49:36.2767605Z == end clock drift check ==
2019-09-30T00:49:37.1627085Z ##[error]Bash exited with code '1'.
2019-09-30T00:49:37.1681864Z ##[section]Starting: Checkout
2019-09-30T00:49:37.1683995Z ==============================================================================
2019-09-30T00:49:37.1684058Z Task         : Get sources
2019-09-30T00:49:37.1684111Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
