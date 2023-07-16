plain
2019-09-26T07:13:51.1308685Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-26T07:13:51.1512530Z ##[command]git config gc.auto 0
2019-09-26T07:13:51.1587161Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-26T07:13:51.1653060Z ##[command]git config --get-all http.proxy
2019-09-26T07:13:51.1798459Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64778/merge:refs/remotes/pull/64778/merge
---
2019-09-26T07:23:49.2343320Z     Checking rand_chacha v0.2.1
2019-09-26T07:23:50.1765982Z     Checking rand v0.7.0
2019-09-26T07:23:51.7397456Z     Checking tempfile v3.1.0
2019-09-26T07:23:52.2029501Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2019-09-26T07:23:53.1098887Z error[E0277]: the trait bound `rustc_target::abi::VariantIdx: rustc_index::indexed_vec::Idx` is not satisfied
2019-09-26T07:23:53.1100696Z     --> src/librustdoc/clean/mod.rs:3396:5
2019-09-26T07:23:53.1101425Z      |
2019-09-26T07:23:53.1102025Z 3396 |     pub variants: IndexVec<VariantIdx, Item>,
2019-09-26T07:23:53.1102762Z      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_index::indexed_vec::Idx` is not implemented for `rustc_target::abi::VariantIdx`
2019-09-26T07:23:53.1103296Z      |
2019-09-26T07:23:53.1103981Z      = note: required by `rustc_index::indexed_vec::IndexVec`
2019-09-26T07:23:53.1657532Z error: aborting due to previous error
2019-09-26T07:23:53.1658340Z 
2019-09-26T07:23:53.1659268Z For more information about this error, try `rustc --explain E0277`.
2019-09-26T07:23:53.1994331Z error: could not compile `rustdoc`.
---
2019-09-26T07:23:53.2085413Z == clock drift check ==
2019-09-26T07:23:53.2098497Z   local time: Thu Sep 26 07:23:53 UTC 2019
2019-09-26T07:23:53.3089039Z   network time: Thu, 26 Sep 2019 07:23:53 GMT
2019-09-26T07:23:53.3094152Z == end clock drift check ==
2019-09-26T07:23:54.9702452Z ##[error]Bash exited with code '1'.
2019-09-26T07:23:54.9765076Z ##[section]Starting: Checkout
2019-09-26T07:23:54.9767258Z ==============================================================================
2019-09-26T07:23:54.9767324Z Task         : Get sources
2019-09-26T07:23:54.9767365Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
