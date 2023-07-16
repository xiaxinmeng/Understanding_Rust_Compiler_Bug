plain
2020-04-13T15:09:47.0773842Z ========================== Starting Command Output ===========================
2020-04-13T15:09:47.0791187Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0297628e-85bb-407f-946e-655771cda5e4.sh
2020-04-13T15:09:47.0967551Z 
2020-04-13T15:09:47.1029080Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-13T15:09:47.1049739Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71003/merge to s
2020-04-13T15:09:47.1053937Z Task         : Get sources
2020-04-13T15:09:47.1054244Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T15:09:47.1054549Z Version      : 1.0.0
2020-04-13T15:09:47.1054766Z Author       : Microsoft
---
2020-04-13T15:09:48.0331815Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-13T15:09:48.0377554Z ##[command]git config gc.auto 0
2020-04-13T15:09:48.0409551Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-13T15:09:48.0433109Z ##[command]git config --get-all http.proxy
2020-04-13T15:09:48.0511705Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71003/merge:refs/remotes/pull/71003/merge
---
2020-04-13T15:14:56.2039722Z  ---> 78ad2f4d4aca
2020-04-13T15:14:56.2039938Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-13T15:14:56.2040266Z  ---> Using cache
2020-04-13T15:14:56.2040558Z  ---> 4d2dc61c4d00
2020-04-13T15:14:56.2041719Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-13T15:14:56.2042849Z  ---> 776b6266a8b7
2020-04-13T15:14:56.2103916Z Successfully built 776b6266a8b7
2020-04-13T15:14:56.2128962Z Successfully tagged rust-ci:latest
2020-04-13T15:14:56.2783188Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-13T15:14:56.2783188Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-13T15:14:56.2797196Z Looks like docker image is the same as before, not uploading
2020-04-13T15:15:04.4825102Z [CI_JOB_NAME=mingw-check]
2020-04-13T15:15:04.5044752Z [CI_JOB_NAME=mingw-check]
2020-04-13T15:15:04.5076365Z == clock drift check ==
2020-04-13T15:15:04.5092913Z   local time: Mon Apr 13 15:15:04 UTC 2020
2020-04-13T15:15:04.6285235Z   network time: Mon, 13 Apr 2020 15:15:04 GMT
2020-04-13T15:15:04.6321223Z Starting sccache server...
2020-04-13T15:15:04.7397276Z configure: processing command line
2020-04-13T15:15:04.7398236Z configure: 
2020-04-13T15:15:04.7399559Z configure: rust.parallel-compiler := True
---
2020-04-13T15:18:33.3748297Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-13T15:18:33.4910454Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-13T15:18:33.6701144Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-13T15:18:33.7519533Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-13T15:18:34.2253708Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-13T15:18:36.3187252Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-13T15:18:36.7559677Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-13T15:18:38.5638785Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-13T15:18:38.9790302Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-13T15:19:10.2429525Z     Checking rustc_ty v0.0.0 (/checkout/src/librustc_ty)
2020-04-13T15:19:10.3035210Z     Checking rustc_mir_build v0.0.0 (/checkout/src/librustc_mir_build)
2020-04-13T15:19:10.6021717Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2020-04-13T15:19:12.7623397Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2020-04-13T15:19:12.7908541Z error[E0050]: method `process_projection_elem` has 2 parameters but the declaration in trait `rustc_middle::mir::visit::MutVisitor::process_projection_elem` has 3
2020-04-13T15:19:12.7909511Z    --> src/librustc_mir/transform/dag_nrvo.rs:264:32
2020-04-13T15:19:12.7910041Z     |
2020-04-13T15:19:12.7910785Z 264 |     fn process_projection_elem(&mut self, elem: &PlaceElem<'tcx>) -> Option<PlaceElem<'tcx>> {
2020-04-13T15:19:12.7912410Z     |
2020-04-13T15:19:12.7912410Z     |
2020-04-13T15:19:12.7913783Z     = note: `process_projection_elem` from trait: `fn(&mut Self, &rustc_middle::mir::ProjectionElem<rustc_middle::mir::Local, &'tcx rustc_middle::ty::TyS<'tcx>>, rustc_middle::mir::Location) -> std::option::Option<rustc_middle::mir::ProjectionElem<rustc_middle::mir::Local, &'tcx rustc_middle::ty::TyS<'tcx>>>`
2020-04-13T15:19:16.4226204Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-13T15:19:16.5780100Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-13T15:19:17.4273746Z error: aborting due to previous error
2020-04-13T15:19:17.4281771Z 
2020-04-13T15:19:17.4281771Z 
2020-04-13T15:19:17.4292493Z For more information about this error, try `rustc --explain E0050`.
2020-04-13T15:19:17.4433277Z error: could not compile `rustc_mir`.
2020-04-13T15:19:17.4433496Z 
2020-04-13T15:19:17.4433894Z To learn more, run the command again with --verbose.
2020-04-13T15:19:17.4462891Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-13T15:19:17.4463942Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-13T15:19:17.4464236Z Build completed unsuccessfully in 0:04:12
2020-04-13T15:19:17.4567988Z == clock drift check ==
2020-04-13T15:19:17.4586217Z   local time: Mon Apr 13 15:19:17 UTC 2020
2020-04-13T15:19:17.4586217Z   local time: Mon Apr 13 15:19:17 UTC 2020
2020-04-13T15:19:17.7750540Z   network time: Mon, 13 Apr 2020 15:19:17 GMT
2020-04-13T15:19:18.4801164Z 
2020-04-13T15:19:18.4801164Z 
2020-04-13T15:19:18.4877161Z ##[error]Bash exited with code '1'.
2020-04-13T15:19:18.4890480Z ##[section]Finishing: Run build
2020-04-13T15:19:18.4932975Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71003/merge to s
2020-04-13T15:19:18.4937665Z Task         : Get sources
2020-04-13T15:19:18.4937993Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T15:19:18.4938288Z Version      : 1.0.0
2020-04-13T15:19:18.4938493Z Author       : Microsoft
2020-04-13T15:19:18.4938493Z Author       : Microsoft
2020-04-13T15:19:18.4938828Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-13T15:19:18.4939189Z ==============================================================================
2020-04-13T15:19:18.8122131Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-13T15:19:18.8160923Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71003/merge to s
2020-04-13T15:19:18.8238397Z Cleaning up task key
2020-04-13T15:19:18.8239713Z Start cleaning up orphan processes.
2020-04-13T15:19:18.8401551Z Terminate orphan process: pid (4476) (python)
2020-04-13T15:19:18.8522073Z ##[section]Finishing: Finalize Job
