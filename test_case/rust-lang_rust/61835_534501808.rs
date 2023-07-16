plain
2019-09-24T10:32:51.3970906Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-24T10:32:51.4194271Z ##[command]git config gc.auto 0
2019-09-24T10:32:51.4270346Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-24T10:32:51.4345552Z ##[command]git config --get-all http.proxy
2019-09-24T10:32:51.4506165Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61835/merge:refs/remotes/pull/61835/merge
---
2019-09-24T10:41:37.8078455Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-09-24T10:41:44.1164156Z error[E0107]: wrong number of lifetime arguments: expected 1, found 2
2019-09-24T10:41:44.1165572Z   --> src/librustc/ty/constness.rs:72:47
2019-09-24T10:41:44.1166214Z    |
2019-09-24T10:41:44.1166981Z 72 |     fn is_const_evaluatable(tcx: TyCtxt<'tcx, 'tcx>, def_id: DefId) -> bool {
2019-09-24T10:41:44.1168106Z 
2019-09-24T10:41:44.1168593Z error[E0107]: wrong number of lifetime arguments: expected 1, found 2
2019-09-24T10:41:44.1168918Z    --> src/librustc/ty/constness.rs:113:48
2019-09-24T10:41:44.1169238Z     |
2019-09-24T10:41:44.1169238Z     |
2019-09-24T10:41:44.1169927Z 113 |     fn is_const_fn_raw<'tcx>(tcx: TyCtxt<'tcx, 'tcx>, def_id: DefId) -> bool {
2019-09-24T10:41:44.1170948Z 
2019-09-24T10:41:44.2380566Z error: aborting due to 2 previous errors
2019-09-24T10:41:44.2381543Z 
2019-09-24T10:41:44.2382207Z For more information about this error, try `rustc --explain E0107`.
---
2019-09-24T10:41:44.4254320Z == clock drift check ==
2019-09-24T10:41:44.4274988Z   local time: Tue Sep 24 10:41:44 UTC 2019
2019-09-24T10:41:44.5027687Z   network time: Tue, 24 Sep 2019 10:41:44 GMT
2019-09-24T10:41:44.5027805Z == end clock drift check ==
2019-09-24T10:41:45.2435532Z ##[error]Bash exited with code '1'.
2019-09-24T10:41:45.2482632Z ##[section]Starting: Checkout
2019-09-24T10:41:45.2484931Z ==============================================================================
2019-09-24T10:41:45.2484989Z Task         : Get sources
2019-09-24T10:41:45.2485035Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
