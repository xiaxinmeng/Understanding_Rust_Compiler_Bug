plain
2020-04-13T01:22:38.2443951Z ========================== Starting Command Output ===========================
2020-04-13T01:22:38.2446488Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/902d9334-9c2d-4b25-b5d8-b1e7f70b9518.sh
2020-04-13T01:22:38.2446715Z 
2020-04-13T01:22:38.2450337Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-13T01:22:38.2467220Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-13T01:22:38.2470394Z Task         : Get sources
2020-04-13T01:22:38.2470621Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T01:22:38.2470896Z Version      : 1.0.0
2020-04-13T01:22:38.2471042Z Author       : Microsoft
---
2020-04-13T01:22:39.2620105Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-13T01:22:39.2626143Z ##[command]git config gc.auto 0
2020-04-13T01:22:39.2631402Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-13T01:22:39.2635772Z ##[command]git config --get-all http.proxy
2020-04-13T01:22:39.2642381Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70551/merge:refs/remotes/pull/70551/merge
---
2020-04-13T01:26:11.7354368Z  ---> 78ad2f4d4aca
2020-04-13T01:26:11.7354932Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-13T01:26:11.7361801Z  ---> Using cache
2020-04-13T01:26:11.7362437Z  ---> 4d2dc61c4d00
2020-04-13T01:26:11.7364077Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-13T01:26:11.7386703Z  ---> 776b6266a8b7
2020-04-13T01:26:11.7475192Z Successfully built 776b6266a8b7
2020-04-13T01:26:11.7789105Z Successfully tagged rust-ci:latest
2020-04-13T01:26:11.8256928Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-13T01:26:11.8256928Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-13T01:26:11.8274985Z Looks like docker image is the same as before, not uploading
2020-04-13T01:26:15.4799752Z [CI_JOB_NAME=mingw-check]
2020-04-13T01:26:15.5075105Z [CI_JOB_NAME=mingw-check]
2020-04-13T01:26:15.5114961Z == clock drift check ==
2020-04-13T01:26:15.5134410Z   local time: Mon Apr 13 01:26:15 UTC 2020
2020-04-13T01:26:15.6384691Z   network time: Mon, 13 Apr 2020 01:26:15 GMT
2020-04-13T01:26:15.6404268Z Starting sccache server...
2020-04-13T01:26:15.7508764Z configure: processing command line
2020-04-13T01:26:15.7509558Z configure: 
2020-04-13T01:26:15.7510607Z configure: rust.parallel-compiler := True
---
2020-04-13T01:30:12.5661718Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-13T01:30:12.5848187Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-13T01:30:12.7810997Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-13T01:30:12.9605930Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-13T01:30:13.3669680Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-13T01:30:15.6552015Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-13T01:30:16.1322110Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-13T01:30:18.1357750Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-13T01:30:18.5491949Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-13T01:30:43.8907147Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2020-04-13T01:30:46.0006616Z error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
2020-04-13T01:30:46.0007345Z     --> src/librustc_typeck/check/mod.rs:4408:54
2020-04-13T01:30:46.0007860Z      |
2020-04-13T01:30:46.0008533Z 4408 |                 let ty = result.map(|(ty, _, _)| ty).unwrap_or_else(|| self.tcx().err());
2020-04-13T01:30:46.0009581Z      |                                                      ^^^^^^^^^^^^^^ -- takes 0 arguments
2020-04-13T01:30:46.0011243Z      |                                                      expected closure that takes 1 argument
2020-04-13T01:30:46.0011822Z      |
2020-04-13T01:30:46.0012374Z help: consider changing the closure to take and ignore the expected argument
2020-04-13T01:30:46.0012882Z      |
2020-04-13T01:30:46.0012882Z      |
2020-04-13T01:30:46.0013541Z 4408 |                 let ty = result.map(|(ty, _, _)| ty).unwrap_or_else(|_| self.tcx().err());
2020-04-13T01:30:46.0014675Z 
2020-04-13T01:30:47.4574646Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2020-04-13T01:30:48.1072708Z error: aborting due to previous error
2020-04-13T01:30:48.1073610Z 
2020-04-13T01:30:48.1073610Z 
2020-04-13T01:30:48.1077369Z For more information about this error, try `rustc --explain E0593`.
2020-04-13T01:30:48.1142449Z error: could not compile `rustc_typeck`.
2020-04-13T01:30:48.1142667Z 
2020-04-13T01:30:48.1143467Z To learn more, run the command again with --verbose.
2020-04-13T01:30:48.1162295Z warning: build failed, waiting for other jobs to finish...
2020-04-13T01:30:49.7689150Z error: build failed
2020-04-13T01:30:49.7714191Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-13T01:30:49.7733347Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-13T01:30:49.7733711Z Build completed unsuccessfully in 0:04:33
2020-04-13T01:30:49.7841403Z == clock drift check ==
2020-04-13T01:30:49.7859372Z   local time: Mon Apr 13 01:30:49 UTC 2020
2020-04-13T01:30:49.7859372Z   local time: Mon Apr 13 01:30:49 UTC 2020
2020-04-13T01:30:50.1091325Z   network time: Mon, 13 Apr 2020 01:30:50 GMT
2020-04-13T01:30:51.0074086Z 
2020-04-13T01:30:51.0074086Z 
2020-04-13T01:30:51.0134237Z ##[error]Bash exited with code '1'.
2020-04-13T01:30:51.0146408Z ##[section]Finishing: Run build
2020-04-13T01:30:51.0212579Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-13T01:30:51.0219631Z Task         : Get sources
2020-04-13T01:30:51.0220118Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T01:30:51.0220394Z Version      : 1.0.0
2020-04-13T01:30:51.0220585Z Author       : Microsoft
2020-04-13T01:30:51.0220585Z Author       : Microsoft
2020-04-13T01:30:51.0220913Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-13T01:30:51.0221450Z ==============================================================================
2020-04-13T01:30:51.3298873Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-13T01:30:51.3346735Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-13T01:30:51.3420599Z Cleaning up task key
2020-04-13T01:30:51.3421643Z Start cleaning up orphan processes.
2020-04-13T01:30:51.3583258Z Terminate orphan process: pid (3488) (python)
2020-04-13T01:30:51.5069255Z ##[section]Finishing: Finalize Job
