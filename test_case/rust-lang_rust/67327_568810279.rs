plain
2019-12-24T22:56:52.5495484Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-24T22:56:53.1676792Z ##[command]git config gc.auto 0
2019-12-24T22:56:53.1680665Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-24T22:56:53.1684274Z ##[command]git config --get-all http.proxy
2019-12-24T22:56:53.1689953Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67327/merge:refs/remotes/pull/67327/merge
---
2019-12-24T23:05:12.4708056Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-12-24T23:05:17.8174131Z error[E0061]: this function takes 3 parameters but 4 parameters were supplied
2019-12-24T23:05:17.8174521Z   --> src/librustc_mir/const_eval.rs:33:15
2019-12-24T23:05:17.8175306Z    |
2019-12-24T23:05:17.8176178Z 33 |       let ecx = mk_eval_cx(tcx, DUMMY_SP, param_env, false);
2019-12-24T23:05:17.8177363Z    | 
2019-12-24T23:05:17.8177903Z   ::: src/librustc_mir/const_eval/eval_queries.rs:72:1
2019-12-24T23:05:17.8178422Z    |
2019-12-24T23:05:17.8178422Z    |
2019-12-24T23:05:17.8179000Z 72 | / pub(super) fn mk_eval_cx<'mir, 'tcx>(
2019-12-24T23:05:17.8179825Z 73 | |     tcx: TyCtxt<'tcx>,
2019-12-24T23:05:17.8180504Z 74 | |     span: Span,
2019-12-24T23:05:17.8181093Z 75 | |     param_env: ty::ParamEnv<'tcx>,
2019-12-24T23:05:17.8181620Z ...  |
2019-12-24T23:05:17.8182255Z 78 | |     InterpCx::new(tcx.at(span), param_env, CompileTimeInterpreter::new(), Default::default())
2019-12-24T23:05:17.8183595Z    | |_- defined here
2019-12-24T23:05:17.8183899Z 
2019-12-24T23:05:17.8326589Z error[E0061]: this function takes 3 parameters but 4 parameters were supplied
2019-12-24T23:05:17.8327364Z   --> src/librustc_mir/const_eval.rs:53:19
2019-12-24T23:05:17.8327364Z   --> src/librustc_mir/const_eval.rs:53:19
2019-12-24T23:05:17.8327823Z    |
2019-12-24T23:05:17.8328418Z 53 |       let mut ecx = mk_eval_cx(tcx, DUMMY_SP, ty::ParamEnv::reveal_all(), false);
2019-12-24T23:05:17.8329520Z    | 
2019-12-24T23:05:17.8330019Z   ::: src/librustc_mir/const_eval/eval_queries.rs:72:1
2019-12-24T23:05:17.8330461Z    |
2019-12-24T23:05:17.8330461Z    |
2019-12-24T23:05:17.8331533Z 72 | / pub(super) fn mk_eval_cx<'mir, 'tcx>(
2019-12-24T23:05:17.8332341Z 73 | |     tcx: TyCtxt<'tcx>,
2019-12-24T23:05:17.8332793Z 74 | |     span: Span,
2019-12-24T23:05:17.8333225Z 75 | |     param_env: ty::ParamEnv<'tcx>,
2019-12-24T23:05:17.8333790Z ...  |
2019-12-24T23:05:17.8334281Z 78 | |     InterpCx::new(tcx.at(span), param_env, CompileTimeInterpreter::new(), Default::default())
2019-12-24T23:05:17.8335584Z    | |_- defined here
2019-12-24T23:05:17.8335813Z 
2019-12-24T23:05:17.8336278Z error[E0061]: this function takes 3 parameters but 4 parameters were supplied
2019-12-24T23:05:17.8337005Z   --> src/librustc_mir/const_eval.rs:74:15
2019-12-24T23:05:17.8337005Z   --> src/librustc_mir/const_eval.rs:74:15
2019-12-24T23:05:17.8337523Z    |
2019-12-24T23:05:17.8337937Z 74 |       let ecx = mk_eval_cx(tcx, DUMMY_SP, param_env, false);
2019-12-24T23:05:17.8338795Z    | 
2019-12-24T23:05:17.8339177Z   ::: src/librustc_mir/const_eval/eval_queries.rs:72:1
2019-12-24T23:05:17.8339672Z    |
2019-12-24T23:05:17.8339672Z    |
2019-12-24T23:05:17.8340171Z 72 | / pub(super) fn mk_eval_cx<'mir, 'tcx>(
2019-12-24T23:05:17.8340878Z 73 | |     tcx: TyCtxt<'tcx>,
2019-12-24T23:05:17.8341298Z 74 | |     span: Span,
2019-12-24T23:05:17.8341931Z 75 | |     param_env: ty::ParamEnv<'tcx>,
2019-12-24T23:05:17.8342312Z ...  |
2019-12-24T23:05:17.8342975Z 78 | |     InterpCx::new(tcx.at(span), param_env, CompileTimeInterpreter::new(), Default::default())
2019-12-24T23:05:17.8344355Z    | |_- defined here
2019-12-24T23:05:17.8344524Z 
2019-12-24T23:05:24.7437393Z error: aborting due to 3 previous errors
2019-12-24T23:05:24.7438285Z 
2019-12-24T23:05:24.7438285Z 
2019-12-24T23:05:24.7446899Z For more information about this error, try `rustc --explain E0061`.
2019-12-24T23:05:24.7558977Z error: could not compile `rustc_mir`.
2019-12-24T23:05:24.7578741Z warning: build failed, waiting for other jobs to finish...
2019-12-24T23:05:26.9739803Z error: build failed
2019-12-24T23:05:26.9768022Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-24T23:05:26.9788144Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-24T23:05:26.9788564Z Build completed unsuccessfully in 0:05:15
2019-12-24T23:05:26.9843129Z == clock drift check ==
2019-12-24T23:05:26.9859056Z   local time: Tue Dec 24 23:05:26 UTC 2019
2019-12-24T23:05:26.9859056Z   local time: Tue Dec 24 23:05:26 UTC 2019
2019-12-24T23:05:27.2655630Z   network time: Tue, 24 Dec 2019 23:05:27 GMT
2019-12-24T23:05:27.2659348Z == end clock drift check ==
2019-12-24T23:05:28.4397926Z 
2019-12-24T23:05:28.4507464Z ##[error]Bash exited with code '1'.
2019-12-24T23:05:28.4542622Z ##[section]Starting: Checkout
2019-12-24T23:05:28.4544194Z ==============================================================================
2019-12-24T23:05:28.4544247Z Task         : Get sources
2019-12-24T23:05:28.4544291Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
