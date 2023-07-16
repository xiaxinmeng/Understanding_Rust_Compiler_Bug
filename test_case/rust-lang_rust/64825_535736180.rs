plain
2019-09-27T00:28:39.5373079Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-27T00:28:40.5369739Z ##[command]git config gc.auto 0
2019-09-27T00:28:40.5376089Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-27T00:28:40.5380705Z ##[command]git config --get-all http.proxy
2019-09-27T00:28:40.5385780Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64825/merge:refs/remotes/pull/64825/merge
---
2019-09-27T00:36:29.9685066Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-09-27T00:36:31.4229221Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-09-27T00:36:32.6777974Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-09-27T00:36:45.5729662Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-09-27T00:36:48.1092918Z error[E0433]: failed to resolve: use of undeclared type or module `ParentHirIterator`
2019-09-27T00:36:48.1093320Z    --> src/librustc/hir/map/mod.rs:808:26
2019-09-27T00:36:48.1093592Z     |
2019-09-27T00:36:48.1100249Z 808 |         for (_, node) in ParentHirIterator::new(hir_id, &self) {
2019-09-27T00:36:48.1100605Z     |                          ^^^^^^^^^^^^^^^^^ use of undeclared type or module `ParentHirIterator`
2019-09-27T00:37:11.3405807Z error: aborting due to previous error
2019-09-27T00:37:11.3406246Z 
2019-09-27T00:37:11.3406566Z For more information about this error, try `rustc --explain E0433`.
2019-09-27T00:37:11.5160712Z error: could not compile `rustc`.
---
2019-09-27T00:37:11.5249203Z == clock drift check ==
2019-09-27T00:37:11.5268682Z   local time: Fri Sep 27 00:37:11 UTC 2019
2019-09-27T00:37:12.3720532Z   network time: Fri, 27 Sep 2019 00:37:12 GMT
2019-09-27T00:37:12.3725779Z == end clock drift check ==
2019-09-27T00:37:13.1844141Z ##[error]Bash exited with code '1'.
2019-09-27T00:37:13.1908375Z ##[section]Starting: Checkout
2019-09-27T00:37:13.1910943Z ==============================================================================
2019-09-27T00:37:13.1911000Z Task         : Get sources
2019-09-27T00:37:13.1911065Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
