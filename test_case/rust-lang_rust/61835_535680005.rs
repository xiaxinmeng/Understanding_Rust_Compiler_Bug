plain
2019-09-26T20:35:37.5728877Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-26T20:35:37.5922914Z ##[command]git config gc.auto 0
2019-09-26T20:35:37.5997554Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-26T20:35:37.6057548Z ##[command]git config --get-all http.proxy
2019-09-26T20:35:37.6203668Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61835/merge:refs/remotes/pull/61835/merge
---
2019-09-26T20:44:01.9012285Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-09-26T20:44:08.4329486Z error[E0107]: wrong number of lifetime arguments: expected 1, found 2
2019-09-26T20:44:08.4330036Z   --> src/librustc/ty/constness.rs:72:47
2019-09-26T20:44:08.4330274Z    |
2019-09-26T20:44:08.4330626Z 72 |     fn is_const_evaluatable(tcx: TyCtxt<'tcx, 'tcx>, def_id: DefId) -> bool {
2019-09-26T20:44:08.4331084Z 
2019-09-26T20:44:08.4331386Z error[E0107]: wrong number of lifetime arguments: expected 1, found 2
2019-09-26T20:44:08.4331649Z    --> src/librustc/ty/constness.rs:113:48
2019-09-26T20:44:08.4331864Z     |
2019-09-26T20:44:08.4331864Z     |
2019-09-26T20:44:08.4332361Z 113 |     fn is_const_fn_raw<'tcx>(tcx: TyCtxt<'tcx, 'tcx>, def_id: DefId) -> bool {
2019-09-26T20:44:08.4332779Z 
2019-09-26T20:44:08.5613200Z error: aborting due to 2 previous errors
2019-09-26T20:44:08.5613774Z 
2019-09-26T20:44:08.5614128Z For more information about this error, try `rustc --explain E0107`.
---
2019-09-26T20:44:08.7346121Z == clock drift check ==
2019-09-26T20:44:08.7362051Z   local time: Thu Sep 26 20:44:08 UTC 2019
2019-09-26T20:44:08.8752459Z   network time: Thu, 26 Sep 2019 20:44:08 GMT
2019-09-26T20:44:08.8757092Z == end clock drift check ==
2019-09-26T20:44:10.1042648Z ##[error]Bash exited with code '1'.
2019-09-26T20:44:10.1083764Z ##[section]Starting: Checkout
2019-09-26T20:44:10.1086049Z ==============================================================================
2019-09-26T20:44:10.1086107Z Task         : Get sources
2019-09-26T20:44:10.1086157Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
