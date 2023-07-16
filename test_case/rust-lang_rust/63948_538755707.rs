plain
2019-10-06T14:43:42.3145263Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-06T14:43:42.3358429Z ##[command]git config gc.auto 0
2019-10-06T14:43:42.3410711Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-06T14:43:42.3469678Z ##[command]git config --get-all http.proxy
2019-10-06T14:43:42.3623515Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63948/merge:refs/remotes/pull/63948/merge
---
2019-10-06T14:53:14.7507801Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-10-06T14:53:14.8339771Z error[E0026]: struct `syntax::ast::Lit` does not have a field named `node`
2019-10-06T14:53:14.8340983Z     --> src/librustc_typeck/collect.rs:2772:23
2019-10-06T14:53:14.8341552Z      |
2019-10-06T14:53:14.8342047Z 2772 |     if let Some(Lit { node: LitKind::Int(ordinal, LitIntType::Unsuffixed), .. }) = sole_meta_list {
2019-10-06T14:53:14.8342520Z      |                       ^^^^ struct `syntax::ast::Lit` does not have this field
2019-10-06T14:53:14.9970153Z error: aborting due to previous error
2019-10-06T14:53:14.9970566Z 
2019-10-06T14:53:14.9976477Z For more information about this error, try `rustc --explain E0026`.
2019-10-06T14:53:15.0359501Z error: could not compile `rustc_typeck`.
---
2019-10-06T14:53:15.9436156Z == clock drift check ==
2019-10-06T14:53:15.9460678Z   local time: Sun Oct  6 14:53:15 UTC 2019
2019-10-06T14:53:16.0975509Z   network time: Sun, 06 Oct 2019 14:53:16 GMT
2019-10-06T14:53:16.0979485Z == end clock drift check ==
2019-10-06T14:53:17.1720050Z ##[error]Bash exited with code '1'.
2019-10-06T14:53:17.1771537Z ##[section]Starting: Checkout
2019-10-06T14:53:17.1773744Z ==============================================================================
2019-10-06T14:53:17.1773820Z Task         : Get sources
2019-10-06T14:53:17.1773869Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
