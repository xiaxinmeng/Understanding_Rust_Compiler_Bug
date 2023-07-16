plain
2019-09-11T02:36:37.5045760Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-11T02:36:38.3144250Z ##[command]git config gc.auto 0
2019-09-11T02:36:38.3153639Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-11T02:36:38.3155181Z ##[command]git config --get-all http.proxy
2019-09-11T02:36:38.3158213Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64303/merge:refs/remotes/pull/64303/merge
---
2019-09-11T02:45:17.5242447Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-09-11T02:46:09.7397625Z error[E0433]: failed to resolve: use of undeclared type or module `kw`
2019-09-11T02:46:09.7398203Z    --> src/librustc_metadata/native_libs.rs:135:44
2019-09-11T02:46:09.7399367Z     |
2019-09-11T02:46:09.7399703Z 135 |         if lib.name.as_ref().map(|&s| s == kw::Invalid).unwrap_or(false) {
2019-09-11T02:46:09.7400130Z     |                                            ^^ use of undeclared type or module `kw`
2019-09-11T02:46:10.6652581Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-09-11T02:46:11.0318062Z error: aborting due to previous error
2019-09-11T02:46:11.0323917Z 
2019-09-11T02:46:11.0364264Z For more information about this error, try `rustc --explain E0433`.
---
2019-09-11T02:46:18.5398684Z == clock drift check ==
2019-09-11T02:46:18.5412072Z   local time: Wed Sep 11 02:46:18 UTC 2019
2019-09-11T02:46:18.6907428Z   network time: Wed, 11 Sep 2019 02:46:18 GMT
2019-09-11T02:46:18.6910342Z == end clock drift check ==
2019-09-11T02:46:19.7323969Z ##[error]Bash exited with code '1'.
2019-09-11T02:46:19.7361678Z ##[section]Starting: Checkout
2019-09-11T02:46:19.7363678Z ==============================================================================
2019-09-11T02:46:19.7363725Z Task         : Get sources
2019-09-11T02:46:19.7363764Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
