plain
2019-09-19T18:32:03.0771858Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-19T18:32:03.1003579Z ##[command]git config gc.auto 0
2019-09-19T18:32:03.1070637Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-19T18:32:03.1121262Z ##[command]git config --get-all http.proxy
2019-09-19T18:32:03.1271423Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64470/merge:refs/remotes/pull/64470/merge
---
2019-09-19T18:41:20.3760989Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-09-19T18:41:22.0954411Z     Checking rustc_ast_borrowck v0.0.0 (/checkout/src/librustc_ast_borrowck)
2019-09-19T18:41:23.5708287Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-09-19T18:41:24.8735581Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-09-19T18:41:25.8891867Z error[E0407]: method `name` is not a member of trait `dataflow::Analysis`
2019-09-19T18:41:25.8893677Z    --> src/librustc_mir/transform/check_consts/resolver.rs:289:5
2019-09-19T18:41:25.8894758Z 289 | /     fn name() -> &'static str {
2019-09-19T18:41:25.8894758Z 289 | /     fn name() -> &'static str {
2019-09-19T18:41:25.8895288Z 290 | |         "qualifier"
2019-09-19T18:41:25.8896303Z     | |_____^ not a member of trait `dataflow::Analysis`
2019-09-19T18:41:25.8900651Z 
2019-09-19T18:41:27.3162844Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-09-19T18:41:28.0005266Z error[E0046]: not all trait items implemented, missing: `NAME`
2019-09-19T18:41:28.0005266Z error[E0046]: not all trait items implemented, missing: `NAME`
2019-09-19T18:41:28.0005752Z    --> src/librustc_mir/transform/check_consts/resolver.rs:283:1
2019-09-19T18:41:28.0006016Z     |
2019-09-19T18:41:28.0006390Z 283 | / impl<Q> dataflow::Analysis<'tcx> for FlowSensitiveAnalysis<'_, '_, 'tcx, Q>
2019-09-19T18:41:28.0006773Z 284 | | where
2019-09-19T18:41:28.0007084Z 285 | |     Q: Qualif,
2019-09-19T18:41:28.0007399Z 286 | | {
2019-09-19T18:41:28.0007965Z 328 | |     }
2019-09-19T18:41:28.0008274Z 329 | | }
2019-09-19T18:41:28.0008274Z 329 | | }
2019-09-19T18:41:28.0008587Z     | |_^ missing `NAME` in implementation
2019-09-19T18:41:28.0009096Z    ::: src/librustc_mir/dataflow/generic.rs:63:5
2019-09-19T18:41:28.0009314Z     |
2019-09-19T18:41:28.0009599Z 63  |       const NAME: &'static str;
2019-09-19T18:41:28.0009599Z 63  |       const NAME: &'static str;
2019-09-19T18:41:28.0009943Z     |       ------------------------- `NAME` from trait
2019-09-19T18:41:28.0714556Z     Checking rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
2019-09-19T18:41:28.7000505Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2019-09-19T18:41:31.4913096Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin)
2019-09-19T18:41:31.7741610Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
---
2019-09-19T18:41:37.1190480Z == clock drift check ==
2019-09-19T18:41:37.1208884Z   local time: Thu Sep 19 18:41:37 UTC 2019
2019-09-19T18:41:37.2706475Z   network time: Thu, 19 Sep 2019 18:41:37 GMT
2019-09-19T18:41:37.2706665Z == end clock drift check ==
2019-09-19T18:41:38.4986737Z ##[error]Bash exited with code '1'.
2019-09-19T18:41:38.5020328Z ##[section]Starting: Checkout
2019-09-19T18:41:38.5022201Z ==============================================================================
2019-09-19T18:41:38.5022255Z Task         : Get sources
2019-09-19T18:41:38.5022303Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
