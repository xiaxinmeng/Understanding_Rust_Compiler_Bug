plain
2019-10-29T23:38:47.9023757Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-29T23:38:47.9220729Z ##[command]git config gc.auto 0
2019-10-29T23:38:47.9292331Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-29T23:38:47.9346380Z ##[command]git config --get-all http.proxy
2019-10-29T23:38:47.9487820Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65949/merge:refs/remotes/pull/65949/merge
---
2019-10-29T23:55:15.0072134Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-10-29T23:55:21.9435489Z error[E0308]: mismatched types
2019-10-29T23:55:21.9435819Z     --> src/librustc_mir/transform/qualify_consts.rs:1777:13
2019-10-29T23:55:21.9436248Z      |
2019-10-29T23:55:21.9437262Z 1723 |       fn run_pass(&self, tcx: TyCtxt<'tcx>, src: MirSource<'tcx>, body: &mut Body<'tcx>) {
2019-10-29T23:55:21.9437816Z      |                                                                                          - expected `()` because of default return type
2019-10-29T23:55:21.9438347Z 1777 | /             match mode {
2019-10-29T23:55:21.9438347Z 1777 | /             match mode {
2019-10-29T23:55:21.9438666Z 1778 | |                 Mode::Const => tcx.mir_const_qualif(def_id),
2019-10-29T23:55:21.9439189Z 1779 | |                 _ => Checker::new(tcx, def_id, body, mode).check_const(),
2019-10-29T23:55:21.9439528Z 1780 | |             }
2019-10-29T23:55:21.9440004Z      | |             ^- help: try adding a semicolon: `;`
2019-10-29T23:55:21.9440484Z      |               expected (), found u8
2019-10-29T23:55:21.9440664Z      |
2019-10-29T23:55:21.9440889Z      = note: expected type `()`
2019-10-29T23:55:21.9441105Z                 found type `u8`
---
2019-10-29T23:56:53.2418895Z   local time: Tue Oct 29 23:56:53 UTC 2019
2019-10-29T23:56:53.3580996Z   network time: Tue, 29 Oct 2019 23:56:53 GMT
2019-10-29T23:56:53.3583115Z == end clock drift check ==
2019-10-29T23:56:56.1373704Z 
2019-10-29T23:56:56.1459978Z ##[error]Bash exited with code '1'.
2019-10-29T23:56:56.1491146Z ##[section]Starting: Checkout
2019-10-29T23:56:56.1492836Z ==============================================================================
2019-10-29T23:56:56.1492899Z Task         : Get sources
2019-10-29T23:56:56.1492937Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
