plain
2019-09-27T15:51:57.8595253Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-27T15:51:57.8774246Z ##[command]git config gc.auto 0
2019-09-27T15:51:57.8857697Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-27T15:51:57.8933642Z ##[command]git config --get-all http.proxy
2019-09-27T15:51:57.9096665Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64778/merge:refs/remotes/pull/64778/merge
---
2019-09-27T16:01:57.4340272Z     Checking rand_chacha v0.2.1
2019-09-27T16:01:57.9351260Z     Checking rand v0.7.0
2019-09-27T16:01:59.5113043Z     Checking tempfile v3.1.0
2019-09-27T16:02:00.4233870Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2019-09-27T16:02:01.3533167Z error[E0277]: the trait bound `rustc_target::abi::VariantIdx: rustc_index::vec::Idx` is not satisfied
2019-09-27T16:02:01.3534427Z     --> src/librustdoc/clean/mod.rs:3396:5
2019-09-27T16:02:01.3534960Z      |
2019-09-27T16:02:01.3535469Z 3396 |     pub variants: IndexVec<VariantIdx, Item>,
2019-09-27T16:02:01.3536129Z      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_index::vec::Idx` is not implemented for `rustc_target::abi::VariantIdx`
2019-09-27T16:02:01.3536572Z      |
2019-09-27T16:02:01.3537937Z      = note: required by `rustc_index::vec::IndexVec`
2019-09-27T16:02:01.4109152Z error: aborting due to previous error
2019-09-27T16:02:01.4110023Z 
2019-09-27T16:02:01.4110555Z For more information about this error, try `rustc --explain E0277`.
2019-09-27T16:02:01.4442099Z error: could not compile `rustdoc`.
---
2019-09-27T16:02:01.4554257Z == clock drift check ==
2019-09-27T16:02:01.4569553Z   local time: Fri Sep 27 16:02:01 UTC 2019
2019-09-27T16:02:01.7359705Z   network time: Fri, 27 Sep 2019 16:02:01 GMT
2019-09-27T16:02:01.7359834Z == end clock drift check ==
2019-09-27T16:02:03.3514143Z ##[error]Bash exited with code '1'.
2019-09-27T16:02:03.3558176Z ##[section]Starting: Checkout
2019-09-27T16:02:03.3560372Z ==============================================================================
2019-09-27T16:02:03.3560428Z Task         : Get sources
2019-09-27T16:02:03.3560476Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
