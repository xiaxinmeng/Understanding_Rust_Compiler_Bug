plain
2020-04-08T13:24:51.3991890Z ========================== Starting Command Output ===========================
2020-04-08T13:24:51.4012440Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/aabd3c70-9592-48eb-946f-5fe304e246bb.sh
2020-04-08T13:24:51.4235736Z 
2020-04-08T13:24:51.4307730Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-08T13:24:51.4324152Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-08T13:24:51.4327268Z Task         : Get sources
2020-04-08T13:24:51.4327534Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T13:24:51.4327776Z Version      : 1.0.0
2020-04-08T13:24:51.4327938Z Author       : Microsoft
---
2020-04-08T13:24:52.6091036Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-08T13:24:52.6101968Z ##[command]git config gc.auto 0
2020-04-08T13:24:52.6109444Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-08T13:24:52.6117040Z ##[command]git config --get-all http.proxy
2020-04-08T13:24:52.6129635Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70820/merge:refs/remotes/pull/70820/merge
---
2020-04-08T13:27:21.6453734Z  ---> 3fc1b512c57b
2020-04-08T13:27:21.6454097Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-08T13:27:21.6455140Z  ---> Using cache
2020-04-08T13:27:21.6455731Z  ---> 5ee4295733f4
2020-04-08T13:27:21.6457491Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-08T13:27:21.6459470Z  ---> 3d07a0fa42fe
2020-04-08T13:27:21.6459772Z Successfully built 3d07a0fa42fe
2020-04-08T13:27:21.6460286Z Successfully tagged rust-ci:latest
2020-04-08T13:27:21.6466009Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-08T13:27:21.6466009Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-08T13:27:21.6467167Z Looks like docker image is the same as before, not uploading
2020-04-08T13:27:29.7377498Z [CI_JOB_NAME=mingw-check]
2020-04-08T13:27:29.7650212Z [CI_JOB_NAME=mingw-check]
2020-04-08T13:27:29.7680236Z == clock drift check ==
2020-04-08T13:27:29.7690555Z   local time: Wed Apr  8 13:27:29 UTC 2020
2020-04-08T13:27:30.0596409Z   network time: Wed, 08 Apr 2020 13:27:30 GMT
2020-04-08T13:27:30.0621807Z Starting sccache server...
2020-04-08T13:27:30.1548857Z configure: processing command line
2020-04-08T13:27:30.1550290Z configure: 
2020-04-08T13:27:30.1551644Z configure: rust.parallel-compiler := True
---
2020-04-08T13:31:30.6689106Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-08T13:31:30.8641482Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-08T13:31:31.0668708Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-08T13:31:31.1107465Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-08T13:31:31.7288902Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-08T13:31:34.3006950Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-08T13:31:34.8252567Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-08T13:31:37.0839370Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-08T13:31:37.5494330Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-08T13:32:17.2936313Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2020-04-08T13:32:17.4325055Z error: expected identifier, found `&`
2020-04-08T13:32:17.4325809Z   --> src/librustc_mir/transform/uneval_const_set.rs:12:38
2020-04-08T13:32:17.4326348Z    |
2020-04-08T13:32:17.4327063Z 12 |         UnevalConstSetVisitor { tcx, &mut uneval_consts }.visit_body(body);
2020-04-08T13:32:17.4329137Z    |         |
2020-04-08T13:32:17.4329791Z    |         while parsing this struct
2020-04-08T13:32:17.4330074Z 
2020-04-08T13:32:18.5146654Z error: unused import: `self`
2020-04-08T13:32:18.5146654Z error: unused import: `self`
2020-04-08T13:32:18.5147356Z   --> src/librustc_mir/transform/simplify.rs:35:24
2020-04-08T13:32:18.5147898Z    |
2020-04-08T13:32:18.5148524Z 35 | use rustc_middle::ty::{self, TyCtxt};
2020-04-08T13:32:18.5149754Z    |
2020-04-08T13:32:18.5150651Z    = note: `-D unused-imports` implied by `-D warnings`
2020-04-08T13:32:18.5150992Z 
2020-04-08T13:32:21.9350739Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-08T13:32:21.9350739Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-08T13:32:22.1294567Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-08T13:32:25.1815802Z error[E0063]: missing field `uneval_consts` in initializer of `transform::uneval_const_set::UnevalConstSetVisitor<'_, '_>`
2020-04-08T13:32:25.1817257Z   --> src/librustc_mir/transform/uneval_const_set.rs:12:9
2020-04-08T13:32:25.1818506Z    |
2020-04-08T13:32:25.1820550Z 12 |         UnevalConstSetVisitor { tcx, &mut uneval_consts }.visit_body(body);
2020-04-08T13:32:25.1822623Z 
2020-04-08T13:32:25.6863701Z error: aborting due to 3 previous errors
2020-04-08T13:32:25.6864042Z 
2020-04-08T13:32:25.6864614Z For more information about this error, try `rustc --explain E0063`.
2020-04-08T13:32:25.6864614Z For more information about this error, try `rustc --explain E0063`.
2020-04-08T13:32:25.7096669Z error: could not compile `rustc_mir`.
2020-04-08T13:32:25.7097016Z 
2020-04-08T13:32:25.7097465Z To learn more, run the command again with --verbose.
2020-04-08T13:32:25.7123972Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-08T13:32:25.7136086Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-08T13:32:25.7136458Z Build completed unsuccessfully in 0:04:55
2020-04-08T13:32:25.7196305Z == clock drift check ==
2020-04-08T13:32:25.7210197Z   local time: Wed Apr  8 13:32:25 UTC 2020
2020-04-08T13:32:25.7210197Z   local time: Wed Apr  8 13:32:25 UTC 2020
2020-04-08T13:32:26.0113409Z   network time: Wed, 08 Apr 2020 13:32:26 GMT
2020-04-08T13:32:26.7178960Z 
2020-04-08T13:32:26.7178960Z 
2020-04-08T13:32:26.7264767Z ##[error]Bash exited with code '1'.
2020-04-08T13:32:26.7279254Z ##[section]Finishing: Run build
2020-04-08T13:32:26.7406518Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-08T13:32:26.7412229Z Task         : Get sources
2020-04-08T13:32:26.7413672Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T13:32:26.7414040Z Version      : 1.0.0
2020-04-08T13:32:26.7414288Z Author       : Microsoft
2020-04-08T13:32:26.7414288Z Author       : Microsoft
2020-04-08T13:32:26.7414657Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-08T13:32:26.7415303Z ==============================================================================
2020-04-08T13:32:27.1002437Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-08T13:32:27.1050219Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-08T13:32:27.1152606Z Cleaning up task key
2020-04-08T13:32:27.1154232Z Start cleaning up orphan processes.
2020-04-08T13:32:27.1368860Z Terminate orphan process: pid (3461) (python)
2020-04-08T13:32:27.1650643Z ##[section]Finishing: Finalize Job
