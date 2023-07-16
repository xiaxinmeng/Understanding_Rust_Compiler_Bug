plain
2020-04-20T22:11:44.9705893Z ========================== Starting Command Output ===========================
2020-04-20T22:11:44.9708653Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1df2e127-9bba-4b9e-b911-e974e3db6bea.sh
2020-04-20T22:11:44.9708931Z 
2020-04-20T22:11:44.9713118Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-20T22:11:44.9732454Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71298/merge to s
2020-04-20T22:11:44.9735612Z Task         : Get sources
2020-04-20T22:11:44.9735896Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-20T22:11:44.9736188Z Version      : 1.0.0
2020-04-20T22:11:44.9736379Z Author       : Microsoft
---
2020-04-20T22:11:45.9650483Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-20T22:11:45.9656803Z ##[command]git config gc.auto 0
2020-04-20T22:11:45.9661688Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-20T22:11:45.9667063Z ##[command]git config --get-all http.proxy
2020-04-20T22:11:45.9675744Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71298/merge:refs/remotes/pull/71298/merge
---
2020-04-20T22:14:14.5717249Z  ---> 78ad2f4d4aca
2020-04-20T22:14:14.5717503Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-20T22:14:14.5717952Z  ---> Using cache
2020-04-20T22:14:14.5718333Z  ---> 4d2dc61c4d00
2020-04-20T22:14:14.5719788Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-20T22:14:14.5752426Z  ---> 776b6266a8b7
2020-04-20T22:14:14.5760322Z Successfully built 776b6266a8b7
2020-04-20T22:14:14.5794399Z Successfully tagged rust-ci:latest
2020-04-20T22:14:14.6612259Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-20T22:14:14.6612259Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-20T22:14:14.6629679Z Looks like docker image is the same as before, not uploading
2020-04-20T22:14:22.7277730Z [CI_JOB_NAME=mingw-check]
2020-04-20T22:14:22.7595321Z [CI_JOB_NAME=mingw-check]
2020-04-20T22:14:22.7632751Z == clock drift check ==
2020-04-20T22:14:22.7642620Z   local time: Mon Apr 20 22:14:22 UTC 2020
2020-04-20T22:14:22.8618026Z   network time: Mon, 20 Apr 2020 22:14:22 GMT
2020-04-20T22:14:22.8644303Z Starting sccache server...
2020-04-20T22:14:22.9815044Z configure: processing command line
2020-04-20T22:14:22.9815926Z configure: 
2020-04-20T22:14:22.9816882Z configure: rust.parallel-compiler := True
---
2020-04-20T22:18:19.9923573Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-20T22:18:20.3618187Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-20T22:18:20.4140584Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-20T22:18:20.5674641Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-20T22:18:21.0443154Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-20T22:18:24.0817734Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-20T22:18:24.0824354Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-20T22:18:25.9554244Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-20T22:18:26.4059908Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-20T22:19:03.4740996Z     | 
2020-04-20T22:19:03.4741574Z    ::: src/librustc_mir/interpret/machine.rs:317:5
2020-04-20T22:19:03.4742084Z     |
2020-04-20T22:19:03.4742692Z 317 | /     fn stack(
2020-04-20T22:19:03.4743517Z 318 | |         ecx: &'a InterpCx<'mir, 'tcx, Self>,
2020-04-20T22:19:03.4744542Z 319 | |     ) -> &'a [Frame<'mir, 'tcx, Self::PointerTag, Self::FrameExtra>];
2020-04-20T22:19:03.4745561Z     | |_____________________________________________________________________- `stack` from trait
2020-04-20T22:19:03.4748310Z 322 | /     fn stack_mut(
2020-04-20T22:19:03.4749383Z 323 | |         ecx: &'a mut InterpCx<'mir, 'tcx, Self>,
2020-04-20T22:19:03.4749383Z 323 | |         ecx: &'a mut InterpCx<'mir, 'tcx, Self>,
2020-04-20T22:19:03.4750439Z 324 | |     ) -> &'a mut Vec<Frame<'mir, 'tcx, Self::PointerTag, Self::FrameExtra>>;
2020-04-20T22:19:03.4751634Z     | |____________________________________________________________________________- `stack_mut` from trait
2020-04-20T22:19:03.9230204Z     Checking rustc_mir_build v0.0.0 (/checkout/src/librustc_mir_build)
2020-04-20T22:19:06.7723463Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2020-04-20T22:19:08.8711378Z error: aborting due to previous error
2020-04-20T22:19:08.8711683Z 
2020-04-20T22:19:08.8711683Z 
2020-04-20T22:19:08.8720709Z For more information about this error, try `rustc --explain E0046`.
2020-04-20T22:19:08.8878585Z error: could not compile `rustc_mir`.
2020-04-20T22:19:08.8878846Z 
2020-04-20T22:19:08.8879314Z To learn more, run the command again with --verbose.
2020-04-20T22:19:08.8879894Z warning: build failed, waiting for other jobs to finish...
2020-04-20T22:19:12.1915060Z error: build failed
2020-04-20T22:19:12.1940695Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-20T22:19:12.1953766Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-20T22:19:12.1954483Z Build completed unsuccessfully in 0:04:49
2020-04-20T22:19:12.2072336Z == clock drift check ==
2020-04-20T22:19:12.2086979Z   local time: Mon Apr 20 22:19:12 UTC 2020
2020-04-20T22:19:12.2086979Z   local time: Mon Apr 20 22:19:12 UTC 2020
2020-04-20T22:19:12.4988329Z   network time: Mon, 20 Apr 2020 22:19:12 GMT
2020-04-20T22:19:13.2205356Z 
2020-04-20T22:19:13.2205356Z 
2020-04-20T22:19:13.2275929Z ##[error]Bash exited with code '1'.
2020-04-20T22:19:13.2290200Z ##[section]Finishing: Run build
2020-04-20T22:19:13.2335842Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71298/merge to s
2020-04-20T22:19:13.2340743Z Task         : Get sources
2020-04-20T22:19:13.2341086Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-20T22:19:13.2341386Z Version      : 1.0.0
2020-04-20T22:19:13.2341601Z Author       : Microsoft
2020-04-20T22:19:13.2341601Z Author       : Microsoft
2020-04-20T22:19:13.2341963Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-20T22:19:13.2342340Z ==============================================================================
2020-04-20T22:19:13.5938896Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-20T22:19:13.5985172Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71298/merge to s
2020-04-20T22:19:13.6081879Z Cleaning up task key
2020-04-20T22:19:13.6083295Z Start cleaning up orphan processes.
2020-04-20T22:19:13.6273402Z Terminate orphan process: pid (4030) (python)
2020-04-20T22:19:13.6464312Z ##[section]Finishing: Finalize Job
