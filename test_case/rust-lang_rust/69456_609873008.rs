plain
2020-04-06T15:28:43.8945793Z ========================== Starting Command Output ===========================
2020-04-06T15:28:43.8949471Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0e5b7143-2af2-473b-9316-dfae6ef9de81.sh
2020-04-06T15:28:43.8949812Z 
2020-04-06T15:28:43.8954285Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-06T15:28:43.8970236Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69456/merge to s
2020-04-06T15:28:43.8972971Z Task         : Get sources
2020-04-06T15:28:43.8973190Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T15:28:43.8973399Z Version      : 1.0.0
2020-04-06T15:28:43.8973542Z Author       : Microsoft
---
2020-04-06T15:28:44.8945019Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-06T15:28:44.8952169Z ##[command]git config gc.auto 0
2020-04-06T15:28:44.8958865Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-06T15:28:44.8964492Z ##[command]git config --get-all http.proxy
2020-04-06T15:28:44.8974465Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69456/merge:refs/remotes/pull/69456/merge
---
2020-04-06T15:32:57.7454706Z  ---> 3fc1b512c57b
2020-04-06T15:32:57.7454905Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-06T15:32:57.7455186Z  ---> Using cache
2020-04-06T15:32:57.7455424Z  ---> 5ee4295733f4
2020-04-06T15:32:57.7456452Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-06T15:32:57.7457469Z  ---> 3d07a0fa42fe
2020-04-06T15:32:57.7479985Z Successfully built 3d07a0fa42fe
2020-04-06T15:32:57.7513403Z Successfully tagged rust-ci:latest
2020-04-06T15:32:57.7995304Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-06T15:32:57.7995304Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-06T15:32:57.8012600Z Looks like docker image is the same as before, not uploading
2020-04-06T15:33:05.1656430Z [CI_JOB_NAME=mingw-check]
2020-04-06T15:33:05.1911559Z [CI_JOB_NAME=mingw-check]
2020-04-06T15:33:05.1936350Z == clock drift check ==
2020-04-06T15:33:05.1942142Z   local time: Mon Apr  6 15:33:05 UTC 2020
2020-04-06T15:33:05.2570773Z   network time: Mon, 06 Apr 2020 15:33:05 GMT
2020-04-06T15:33:05.2594492Z Starting sccache server...
2020-04-06T15:33:05.3315135Z configure: processing command line
2020-04-06T15:33:05.3316772Z configure: 
2020-04-06T15:33:05.3317655Z configure: rust.parallel-compiler := True
---
2020-04-06T15:36:28.6876921Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-06T15:36:28.8302329Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-06T15:36:29.0068449Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-06T15:36:29.0754862Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-06T15:36:29.5843811Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-06T15:36:31.7493558Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-06T15:36:32.2534042Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-06T15:36:34.1700470Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-06T15:36:34.5734575Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-06T15:36:34.5734575Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-06T15:36:35.6968659Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-04-06T15:36:36.3860460Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-04-06T15:36:37.9228032Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-04-06T15:36:52.1696402Z     Checking rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-04-06T15:36:53.2732618Z error[E0308]: mismatched types
2020-04-06T15:36:53.2733264Z    --> src/librustc_infer/infer/error_reporting/need_type_info.rs:245:78
2020-04-06T15:36:53.2733715Z     |
2020-04-06T15:36:53.2734408Z 245 |         let mut local_visitor = FindLocalByTypeVisitor::new(&self, ty, span, &self.tcx.hir());
2020-04-06T15:36:53.2735962Z     |                                                                              |
2020-04-06T15:36:53.2735962Z     |                                                                              |
2020-04-06T15:36:53.2736917Z     |                                                                              expected struct `rustc_middle::hir::map::Map`, found `&rustc_middle::hir::map::Map<'_>`
2020-04-06T15:36:53.2738001Z     |                                                                              help: consider removing the borrow: `self.tcx.hir()`
2020-04-06T15:36:53.2784825Z error[E0061]: this function takes 0 arguments but 2 arguments were supplied
2020-04-06T15:36:53.2785562Z    --> src/librustc_infer/infer/error_reporting/need_type_info.rs:308:54
2020-04-06T15:36:53.2786095Z     |
2020-04-06T15:36:53.2786095Z     |
2020-04-06T15:36:53.2786712Z 308 |                     let fn_sig = substs.as_closure().sig(*def_id, self.tcx);
2020-04-06T15:36:53.2789250Z     |                                                      |
2020-04-06T15:36:53.2790017Z     |                                                      expected 0 arguments
2020-04-06T15:36:53.2790681Z 
2020-04-06T15:36:53.8429567Z error: aborting due to 2 previous errors
---
2020-04-06T15:36:53.8519384Z 
2020-04-06T15:36:53.8520186Z To learn more, run the command again with --verbose.
2020-04-06T15:36:53.8537465Z warning: build failed, waiting for other jobs to finish...
2020-04-06T15:36:54.6967882Z error: build failed
2020-04-06T15:36:54.6990145Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-06T15:36:54.6998929Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-06T15:36:54.6999201Z Build completed unsuccessfully in 0:03:49
2020-04-06T15:36:54.7045296Z == clock drift check ==
2020-04-06T15:36:54.7060180Z   local time: Mon Apr  6 15:36:54 UTC 2020
2020-04-06T15:36:54.7060180Z   local time: Mon Apr  6 15:36:54 UTC 2020
2020-04-06T15:36:54.7689358Z   network time: Mon, 06 Apr 2020 15:36:54 GMT
2020-04-06T15:36:55.3554262Z 
2020-04-06T15:36:55.3554262Z 
2020-04-06T15:36:55.3627976Z ##[error]Bash exited with code '1'.
2020-04-06T15:36:55.3649068Z ##[section]Finishing: Run build
2020-04-06T15:36:55.3705041Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69456/merge to s
2020-04-06T15:36:55.3708954Z Task         : Get sources
2020-04-06T15:36:55.3709216Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T15:36:55.3709470Z Version      : 1.0.0
2020-04-06T15:36:55.3709641Z Author       : Microsoft
2020-04-06T15:36:55.3709641Z Author       : Microsoft
2020-04-06T15:36:55.3709909Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-06T15:36:55.3710232Z ==============================================================================
2020-04-06T15:36:55.6586355Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-06T15:36:55.6625587Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69456/merge to s
2020-04-06T15:36:55.6698822Z Cleaning up task key
2020-04-06T15:36:55.6699948Z Start cleaning up orphan processes.
2020-04-06T15:36:55.6891310Z Terminate orphan process: pid (3615) (python)
2020-04-06T15:36:55.7011364Z ##[section]Finishing: Finalize Job
