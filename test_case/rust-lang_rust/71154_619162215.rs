plain
2020-04-24T17:45:07.9666621Z ========================== Starting Command Output ===========================
2020-04-24T17:45:07.9668751Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0ea6c5dc-5440-4d66-9054-6a88627012c6.sh
2020-04-24T17:45:07.9668972Z 
2020-04-24T17:45:07.9672393Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-24T17:45:07.9703827Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-24T17:45:07.9706916Z Task         : Get sources
2020-04-24T17:45:07.9707213Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T17:45:07.9707485Z Version      : 1.0.0
2020-04-24T17:45:07.9707668Z Author       : Microsoft
---
2020-04-24T17:45:08.9738756Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-24T17:45:08.9745100Z ##[command]git config gc.auto 0
2020-04-24T17:45:08.9750087Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-24T17:45:08.9754832Z ##[command]git config --get-all http.proxy
2020-04-24T17:45:08.9761944Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71154/merge:refs/remotes/pull/71154/merge
---
2020-04-24T17:47:07.5909861Z  ---> f7353ccad5b1
2020-04-24T17:47:07.5910101Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-24T17:47:07.5922360Z  ---> Using cache
2020-04-24T17:47:07.5922979Z  ---> ed38efbaa060
2020-04-24T17:47:07.5926007Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-24T17:47:07.5927904Z  ---> c5008ef7ae8e
2020-04-24T17:47:07.5958996Z Successfully built c5008ef7ae8e
2020-04-24T17:47:07.5993315Z Successfully tagged rust-ci:latest
2020-04-24T17:47:07.6653873Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-24T17:47:07.6653873Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-24T17:47:07.6667686Z Looks like docker image is the same as before, not uploading
2020-04-24T17:47:15.2442541Z [CI_JOB_NAME=mingw-check]
2020-04-24T17:47:15.2687527Z [CI_JOB_NAME=mingw-check]
2020-04-24T17:47:15.2717763Z == clock drift check ==
2020-04-24T17:47:15.2724346Z   local time: Fri Apr 24 17:47:15 UTC 2020
2020-04-24T17:47:15.4339814Z   network time: Fri, 24 Apr 2020 17:47:15 GMT
2020-04-24T17:47:15.4371271Z Starting sccache server...
2020-04-24T17:47:15.5426439Z configure: processing command line
2020-04-24T17:47:15.5426690Z configure: 
2020-04-24T17:47:15.5427546Z configure: rust.parallel-compiler := True
---
2020-04-24T17:50:49.5760771Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T17:50:49.7966682Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T17:50:49.9383322Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T17:50:49.9721282Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T17:50:51.0671696Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T17:50:52.7254989Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T17:50:53.1452650Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T17:50:54.9716015Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T17:50:55.3992302Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-24T17:50:55.3992302Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-24T17:50:56.5415710Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-04-24T17:50:57.2082913Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-04-24T17:50:58.6361334Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-04-24T17:51:11.7999417Z     Checking rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-04-24T17:51:12.0693085Z error[E0425]: cannot find value `tcx` in this scope
2020-04-24T17:51:12.0694852Z     --> src/librustc_metadata/rmeta/encoder.rs:1348:57
2020-04-24T17:51:12.0695618Z      |
2020-04-24T17:51:12.0696483Z 1348 |         let qualifs = self.tcx.mir_const_qualif(def_id, tcx.const_param_of(def_id));
2020-04-24T17:51:12.0698425Z      |                                                         ^^^ help: you might have meant to use the available field: `self.tcx`
2020-04-24T17:51:13.0733398Z error[E0308]: mismatched types
2020-04-24T17:51:13.0733398Z error[E0308]: mismatched types
2020-04-24T17:51:13.0734479Z     --> src/librustc_metadata/rmeta/encoder.rs:1317:44
2020-04-24T17:51:13.0735796Z 1317 |         let ty = self.tcx.typeck_tables_of(def_id).node_type(hir_id);
2020-04-24T17:51:13.0737003Z      |                                            ^^^^^^ expected struct `rustc_span::def_id::DefId`, found struct `rustc_span::def_id::LocalDefId`
2020-04-24T17:51:13.0737640Z 
2020-04-24T17:51:13.3226619Z error: aborting due to 2 previous errors
---
2020-04-24T17:51:13.3284659Z 
2020-04-24T17:51:13.3285237Z To learn more, run the command again with --verbose.
2020-04-24T17:51:13.3293277Z warning: build failed, waiting for other jobs to finish...
2020-04-24T17:51:14.5089292Z error: build failed
2020-04-24T17:51:14.5117087Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-24T17:51:14.5131406Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-24T17:51:14.5131878Z Build completed unsuccessfully in 0:03:58
2020-04-24T17:51:14.5240295Z == clock drift check ==
2020-04-24T17:51:14.5240295Z == clock drift check ==
2020-04-24T17:51:14.5252873Z   local time: Fri Apr 24 17:51:14 UTC 2020
2020-04-24T17:51:14.5903887Z   network time: Fri, 24 Apr 2020 17:51:14 GMT
2020-04-24T17:51:15.4967787Z 
2020-04-24T17:51:15.4967787Z 
2020-04-24T17:51:15.5037054Z ##[error]Bash exited with code '1'.
2020-04-24T17:51:15.5050454Z ##[section]Finishing: Run build
2020-04-24T17:51:15.5093475Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-24T17:51:15.5098139Z Task         : Get sources
2020-04-24T17:51:15.5098447Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T17:51:15.5098743Z Version      : 1.0.0
2020-04-24T17:51:15.5098949Z Author       : Microsoft
2020-04-24T17:51:15.5098949Z Author       : Microsoft
2020-04-24T17:51:15.5099268Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-24T17:51:15.5099642Z ==============================================================================
2020-04-24T17:51:15.8333238Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-24T17:51:15.8386476Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-24T17:51:15.8465483Z Cleaning up task key
2020-04-24T17:51:15.8466556Z Start cleaning up orphan processes.
2020-04-24T17:51:15.8629065Z Terminate orphan process: pid (3822) (python)
2020-04-24T17:51:15.8808725Z ##[section]Finishing: Finalize Job
