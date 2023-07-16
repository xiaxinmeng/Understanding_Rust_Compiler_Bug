plain
2020-01-15T23:39:52.7274660Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2020-01-15T23:39:53.7840101Z error[E0433]: failed to resolve: use of undeclared type or module `Engine`
2020-01-15T23:39:53.7841230Z   --> src/librustc_mir/transform/rustc_peek.rs:42:13
2020-01-15T23:39:53.7841846Z    |
2020-01-15T23:39:53.7842730Z 42 |             Engine::new_gen_kill(tcx, body, def_id, MaybeInitializedPlaces::new(tcx, body, &mdpe))
2020-01-15T23:39:53.7843237Z    |             ^^^^^^ use of undeclared type or module `Engine`
2020-01-15T23:39:53.7847485Z error[E0433]: failed to resolve: use of undeclared type or module `Engine`
2020-01-15T23:39:53.7852126Z   --> src/librustc_mir/transform/rustc_peek.rs:44:28
2020-01-15T23:39:53.7852413Z    |
2020-01-15T23:39:53.7852413Z    |
2020-01-15T23:39:53.7852729Z 44 |         let flow_uninits = Engine::new_gen_kill(
2020-01-15T23:39:53.7853155Z    |                            ^^^^^^ use of undeclared type or module `Engine`
2020-01-15T23:39:53.7898723Z error[E0433]: failed to resolve: use of undeclared type or module `Engine`
2020-01-15T23:39:53.7899276Z   --> src/librustc_mir/transform/rustc_peek.rs:51:30
2020-01-15T23:39:53.7899522Z    |
2020-01-15T23:39:53.7899522Z    |
2020-01-15T23:39:53.7899832Z 51 |         let flow_def_inits = Engine::new_gen_kill(
2020-01-15T23:39:53.7900195Z    |                              ^^^^^^ use of undeclared type or module `Engine`
2020-01-15T23:39:53.8056625Z error[E0433]: failed to resolve: use of undeclared type or module `ResultsCursor`
2020-01-15T23:39:53.8056994Z    --> src/librustc_mir/transform/rustc_peek.rs:116:22
2020-01-15T23:39:53.8057602Z     |
2020-01-15T23:39:53.8058101Z 116 |     let mut cursor = ResultsCursor::new(body, results);
2020-01-15T23:39:53.8058101Z 116 |     let mut cursor = ResultsCursor::new(body, results);
2020-01-15T23:39:53.8058645Z     |                      ^^^^^^^^^^^^^ use of undeclared type or module `ResultsCursor`
2020-01-15T23:39:53.8064714Z 
2020-01-15T23:39:53.8477759Z error[E0412]: cannot find type `Results` in this scope
2020-01-15T23:39:53.8478208Z    --> src/librustc_mir/transform/rustc_peek.rs:110:15
2020-01-15T23:39:53.8478496Z     |
2020-01-15T23:39:53.8478830Z 110 |     results: &Results<'tcx, A>,
2020-01-15T23:39:53.8479731Z     |
2020-01-15T23:39:53.8480177Z help: an enum with a similar name exists
2020-01-15T23:39:53.8480443Z     |
2020-01-15T23:39:53.8480443Z     |
2020-01-15T23:39:53.8480900Z 110 |     results: &Result<'tcx, A>,
2020-01-15T23:39:53.8481566Z help: possible candidate is found in another module, you can import it into scope
2020-01-15T23:39:53.8482215Z     |
2020-01-15T23:39:53.8482215Z     |
2020-01-15T23:39:53.8482825Z 1   | use crate::dataflow::generic::Results;
2020-01-15T23:39:53.8491089Z 
2020-01-15T23:39:53.8517703Z error[E0405]: cannot find trait `Analysis` in this scope
2020-01-15T23:39:53.8518109Z    --> src/librustc_mir/transform/rustc_peek.rs:249:30
2020-01-15T23:39:53.8518367Z     |
2020-01-15T23:39:53.8518367Z     |
2020-01-15T23:39:53.8518718Z 249 | pub trait RustcPeekAt<'tcx>: Analysis<'tcx> {
2020-01-15T23:39:53.8519396Z     |
2020-01-15T23:39:53.8519735Z help: possible candidate is found in another module, you can import it into scope
2020-01-15T23:39:53.8520015Z     |
2020-01-15T23:39:53.8520015Z     |
2020-01-15T23:39:53.8520357Z 1   | use crate::dataflow::generic::Analysis;
2020-01-15T23:39:53.8526173Z 
2020-01-15T23:39:53.8556181Z error[E0405]: cannot find trait `Analysis` in this scope
2020-01-15T23:39:53.8556724Z    --> src/librustc_mir/transform/rustc_peek.rs:261:8
2020-01-15T23:39:53.8557387Z     |
2020-01-15T23:39:53.8557387Z     |
2020-01-15T23:39:53.8557774Z 261 |     A: Analysis<'tcx, Idx = MovePathIndex> + HasMoveData<'tcx>,
2020-01-15T23:39:53.8558503Z     |
2020-01-15T23:39:53.8558880Z help: possible candidate is found in another module, you can import it into scope
2020-01-15T23:39:53.8559156Z     |
2020-01-15T23:39:53.8559156Z     |
2020-01-15T23:39:53.8559515Z 1   | use crate::dataflow::generic::Analysis;
2020-01-15T23:39:53.8565608Z 
2020-01-15T23:39:54.9298109Z error: aborting due to 7 previous errors
2020-01-15T23:39:54.9303442Z 
2020-01-15T23:39:54.9316528Z Some errors have detailed explanations: E0405, E0412, E0433.
---
2020-01-15T23:41:00.4854854Z   local time: Wed Jan 15 23:41:00 UTC 2020
2020-01-15T23:41:01.1991490Z   network time: Wed, 15 Jan 2020 23:41:01 GMT
2020-01-15T23:41:01.1991622Z == end clock drift check ==
2020-01-15T23:41:01.6743803Z 
2020-01-15T23:41:01.6843651Z ##[error]Bash exited with code '1'.
2020-01-15T23:41:01.6899835Z ##[section]Starting: Checkout
2020-01-15T23:41:01.6901739Z ==============================================================================
2020-01-15T23:41:01.6901836Z Task         : Get sources
2020-01-15T23:41:01.6901927Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
