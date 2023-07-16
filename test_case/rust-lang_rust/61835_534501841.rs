plain
2019-09-24T10:32:48.0922907Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-24T10:32:48.1129176Z ##[command]git config gc.auto 0
2019-09-24T10:32:48.1204894Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-24T10:32:48.1255562Z ##[command]git config --get-all http.proxy
2019-09-24T10:32:48.1404294Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61835/merge:refs/remotes/pull/61835/merge
---
2019-09-24T10:41:13.6001269Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-09-24T10:41:19.3101477Z error[E0107]: wrong number of lifetime arguments: expected 1, found 2
2019-09-24T10:41:19.3102727Z   --> src/librustc/ty/constness.rs:72:47
2019-09-24T10:41:19.3103480Z    |
2019-09-24T10:41:19.3104290Z 72 |     fn is_const_evaluatable(tcx: TyCtxt<'tcx, 'tcx>, def_id: DefId) -> bool {
2019-09-24T10:41:19.3105077Z 
2019-09-24T10:41:19.3105506Z error[E0107]: wrong number of lifetime arguments: expected 1, found 2
2019-09-24T10:41:19.3105986Z    --> src/librustc/ty/constness.rs:113:48
2019-09-24T10:41:19.3106411Z     |
2019-09-24T10:41:19.3106411Z     |
2019-09-24T10:41:19.3106986Z 113 |     fn is_const_fn_raw<'tcx>(tcx: TyCtxt<'tcx, 'tcx>, def_id: DefId) -> bool {
2019-09-24T10:41:19.3107754Z 
2019-09-24T10:41:19.4214638Z error: aborting due to 2 previous errors
2019-09-24T10:41:19.4215680Z 
2019-09-24T10:41:19.4216297Z For more information about this error, try `rustc --explain E0107`.
---
2019-09-24T10:41:19.5748599Z == clock drift check ==
2019-09-24T10:41:19.5769126Z   local time: Tue Sep 24 10:41:19 UTC 2019
2019-09-24T10:41:19.7257552Z   network time: Tue, 24 Sep 2019 10:41:19 GMT
2019-09-24T10:41:19.7257644Z == end clock drift check ==
2019-09-24T10:41:20.8413795Z ##[error]Bash exited with code '1'.
2019-09-24T10:41:20.8449874Z ##[section]Starting: Checkout
2019-09-24T10:41:20.8452104Z ==============================================================================
2019-09-24T10:41:20.8452180Z Task         : Get sources
2019-09-24T10:41:20.8452230Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
