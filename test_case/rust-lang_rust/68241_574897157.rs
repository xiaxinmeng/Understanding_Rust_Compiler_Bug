plain
2020-01-15T22:43:54.8483768Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-15T22:43:54.8588490Z ##[command]git config gc.auto 0
2020-01-15T22:43:54.8644866Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-15T22:43:54.8699021Z ##[command]git config --get-all http.proxy
2020-01-15T22:43:54.8853406Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68241/merge:refs/remotes/pull/68241/merge
---
2020-01-15T22:51:59.0608036Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2020-01-15T22:51:59.9908527Z error[E0433]: failed to resolve: use of undeclared type or module `Engine`
2020-01-15T22:51:59.9909568Z   --> src/librustc_mir/transform/rustc_peek.rs:42:13
2020-01-15T22:51:59.9910016Z    |
2020-01-15T22:51:59.9910494Z 42 |             Engine::new_gen_kill(tcx, body, def_id, MaybeInitializedPlaces::new(tcx, body, &mdpe))
2020-01-15T22:51:59.9910999Z    |             ^^^^^^ use of undeclared type or module `Engine`
2020-01-15T22:51:59.9939652Z error[E0433]: failed to resolve: use of undeclared type or module `Engine`
2020-01-15T22:51:59.9940161Z   --> src/librustc_mir/transform/rustc_peek.rs:44:28
2020-01-15T22:51:59.9940540Z    |
2020-01-15T22:51:59.9940540Z    |
2020-01-15T22:51:59.9941527Z 44 |         let flow_uninits = Engine::new_gen_kill(
2020-01-15T22:51:59.9943182Z    |                            ^^^^^^ use of undeclared type or module `Engine`
2020-01-15T22:51:59.9971695Z error[E0433]: failed to resolve: use of undeclared type or module `Engine`
2020-01-15T22:51:59.9972223Z   --> src/librustc_mir/transform/rustc_peek.rs:51:30
2020-01-15T22:51:59.9972635Z    |
2020-01-15T22:51:59.9972635Z    |
2020-01-15T22:51:59.9973151Z 51 |         let flow_def_inits = Engine::new_gen_kill(
2020-01-15T22:51:59.9973839Z    |                              ^^^^^^ use of undeclared type or module `Engine`
2020-01-15T22:52:00.0201498Z error[E0433]: failed to resolve: use of undeclared type or module `ResultsCursor`
2020-01-15T22:52:00.0203196Z    --> src/librustc_mir/transform/rustc_peek.rs:116:22
2020-01-15T22:52:00.0203546Z     |
2020-01-15T22:52:00.0203878Z 116 |     let mut cursor = ResultsCursor::new(body, results);
2020-01-15T22:52:00.0203878Z 116 |     let mut cursor = ResultsCursor::new(body, results);
2020-01-15T22:52:00.0204267Z     |                      ^^^^^^^^^^^^^ use of undeclared type or module `ResultsCursor`
2020-01-15T22:52:00.0205916Z 
2020-01-15T22:52:00.0701284Z error[E0412]: cannot find type `Results` in this scope
2020-01-15T22:52:00.0702105Z    --> src/librustc_mir/transform/rustc_peek.rs:110:15
2020-01-15T22:52:00.0702841Z     |
2020-01-15T22:52:00.0703170Z 110 |     results: &Results<'tcx, A>,
2020-01-15T22:52:00.0703799Z     |
2020-01-15T22:52:00.0704065Z help: an enum with a similar name exists
2020-01-15T22:52:00.0704284Z     |
2020-01-15T22:52:00.0704284Z     |
2020-01-15T22:52:00.0704709Z 110 |     results: &Result<'tcx, A>,
2020-01-15T22:52:00.0705260Z help: possible candidate is found in another module, you can import it into scope
2020-01-15T22:52:00.0705501Z     |
2020-01-15T22:52:00.0705501Z     |
2020-01-15T22:52:00.0705822Z 1   | use crate::dataflow::generic::Results;
2020-01-15T22:52:00.0706117Z 
2020-01-15T22:52:00.0797888Z error[E0405]: cannot find trait `Analysis` in this scope
2020-01-15T22:52:00.0798393Z    --> src/librustc_mir/transform/rustc_peek.rs:249:30
2020-01-15T22:52:00.0798921Z     |
2020-01-15T22:52:00.0798921Z     |
2020-01-15T22:52:00.0799767Z 249 | pub trait RustcPeekAt<'tcx>: Analysis<'tcx> {
2020-01-15T22:52:00.0801336Z     |
2020-01-15T22:52:00.0802373Z help: possible candidate is found in another module, you can import it into scope
2020-01-15T22:52:00.0803174Z     |
2020-01-15T22:52:00.0803174Z     |
2020-01-15T22:52:00.0803667Z 1   | use crate::dataflow::generic::Analysis;
2020-01-15T22:52:00.0804332Z 
2020-01-15T22:52:00.0890890Z error[E0405]: cannot find trait `Analysis` in this scope
2020-01-15T22:52:00.0892118Z    --> src/librustc_mir/transform/rustc_peek.rs:261:8
2020-01-15T22:52:00.0893276Z     |
2020-01-15T22:52:00.0893276Z     |
2020-01-15T22:52:00.0894220Z 261 |     A: Analysis<'tcx, Idx = MovePathIndex> + HasMoveData<'tcx>,
2020-01-15T22:52:00.0895152Z     |
2020-01-15T22:52:00.0896123Z help: possible candidate is found in another module, you can import it into scope
2020-01-15T22:52:00.0896775Z     |
2020-01-15T22:52:00.0896775Z     |
2020-01-15T22:52:00.0897331Z 1   | use crate::dataflow::generic::Analysis;
2020-01-15T22:52:00.0897985Z 
2020-01-15T22:52:00.8132397Z error: aborting due to 7 previous errors
2020-01-15T22:52:00.8134014Z 
2020-01-15T22:52:00.8144552Z Some errors have detailed explanations: E0405, E0412, E0433.
2020-01-15T22:52:00.8144552Z Some errors have detailed explanations: E0405, E0412, E0433.
2020-01-15T22:52:00.8155116Z For more information about an error, try `rustc --explain E0405`.
2020-01-15T22:52:00.8236427Z error: could not compile `rustc_mir`.
2020-01-15T22:52:00.8255792Z warning: build failed, waiting for other jobs to finish...
2020-01-15T22:52:13.4677339Z error: build failed
2020-01-15T22:52:13.4698583Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-15T22:52:13.4706083Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-15T22:52:13.4706154Z Build completed unsuccessfully in 0:05:32
2020-01-15T22:52:13.4759433Z == clock drift check ==
2020-01-15T22:52:13.4768955Z   local time: Wed Jan 15 22:52:13 UTC 2020
2020-01-15T22:52:13.4768955Z   local time: Wed Jan 15 22:52:13 UTC 2020
2020-01-15T22:52:13.9968553Z   network time: Wed, 15 Jan 2020 22:52:13 GMT
2020-01-15T22:52:13.9977236Z == end clock drift check ==
2020-01-15T22:52:14.3950719Z 
2020-01-15T22:52:14.4041930Z ##[error]Bash exited with code '1'.
2020-01-15T22:52:14.4070849Z ##[section]Starting: Checkout
2020-01-15T22:52:14.4072266Z ==============================================================================
2020-01-15T22:52:14.4072309Z Task         : Get sources
2020-01-15T22:52:14.4072345Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
