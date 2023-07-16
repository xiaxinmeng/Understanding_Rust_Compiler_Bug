plain
2019-09-03T15:56:50.9020160Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-03T15:56:50.9215507Z ##[command]git config gc.auto 0
2019-09-03T15:56:50.9298702Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-03T15:56:50.9371936Z ##[command]git config --get-all http.proxy
2019-09-03T15:56:50.9512055Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64108/merge:refs/remotes/pull/64108/merge
---
2019-09-03T16:05:31.0005285Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-09-03T16:06:28.3135695Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-09-03T16:06:30.1078628Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-09-03T16:06:31.8548377Z     Checking rustc_ast_borrowck v0.0.0 (/checkout/src/librustc_ast_borrowck)
2019-09-03T16:06:32.6451502Z error[E0599]: no method named `references_error` found for type `&rustc::ty::TyS<'_>` in the current scope
2019-09-03T16:06:32.6452706Z    --> src/librustc_typeck/impl_wf_check.rs:102:21
2019-09-03T16:06:32.6452975Z     |
2019-09-03T16:06:32.6453244Z 102 |     if impl_self_ty.references_error() {
2019-09-03T16:06:32.6453762Z     |
2019-09-03T16:06:32.6454030Z     = help: items from traits can only be used if the trait is in scope
2019-09-03T16:06:32.6454030Z     = help: items from traits can only be used if the trait is in scope
2019-09-03T16:06:32.6454572Z     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-09-03T16:06:32.6455230Z             `use crate::rustc::ty::TypeFoldable;`
2019-09-03T16:06:32.7925216Z error: aborting due to previous error
2019-09-03T16:06:32.7926407Z 
2019-09-03T16:06:32.7932271Z For more information about this error, try `rustc --explain E0599`.
2019-09-03T16:06:32.8293964Z error: Could not compile `rustc_typeck`.
---
2019-09-03T16:06:33.2324862Z == clock drift check ==
2019-09-03T16:06:33.2342076Z   local time: Tue Sep  3 16:06:33 UTC 2019
2019-09-03T16:06:33.5150289Z   network time: Tue, 03 Sep 2019 16:06:33 GMT
2019-09-03T16:06:33.5150924Z == end clock drift check ==
2019-09-03T16:06:34.5678590Z ##[error]Bash exited with code '1'.
2019-09-03T16:06:34.5720394Z ##[section]Starting: Checkout
2019-09-03T16:06:34.5722497Z ==============================================================================
2019-09-03T16:06:34.5722570Z Task         : Get sources
2019-09-03T16:06:34.5722617Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
