plain
2020-04-06T21:20:18.1624871Z ========================== Starting Command Output ===========================
2020-04-06T21:20:18.1628178Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c8953652-99ca-4902-880e-735fe2e47cd0.sh
2020-04-06T21:20:18.1628435Z 
2020-04-06T21:20:18.1634286Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-06T21:20:18.1654266Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-06T21:20:18.1657843Z Task         : Get sources
2020-04-06T21:20:18.1658143Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T21:20:18.1658571Z Version      : 1.0.0
2020-04-06T21:20:18.1658769Z Author       : Microsoft
---
2020-04-06T21:20:19.1530375Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-06T21:20:19.1542380Z ##[command]git config gc.auto 0
2020-04-06T21:20:19.1550819Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-06T21:20:19.1553981Z ##[command]git config --get-all http.proxy
2020-04-06T21:20:19.1560728Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70820/merge:refs/remotes/pull/70820/merge
---
2020-04-06T21:22:43.6688959Z  ---> 3fc1b512c57b
2020-04-06T21:22:43.6689210Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-06T21:22:43.6693791Z  ---> Using cache
2020-04-06T21:22:43.6694154Z  ---> 5ee4295733f4
2020-04-06T21:22:43.6695457Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-06T21:22:43.6702667Z  ---> 3d07a0fa42fe
2020-04-06T21:22:43.6737068Z Successfully built 3d07a0fa42fe
2020-04-06T21:22:43.6777193Z Successfully tagged rust-ci:latest
2020-04-06T21:22:43.7096042Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-06T21:22:43.7096042Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-06T21:22:43.7116795Z Looks like docker image is the same as before, not uploading
2020-04-06T21:22:51.9360439Z [CI_JOB_NAME=mingw-check]
2020-04-06T21:22:51.9588283Z [CI_JOB_NAME=mingw-check]
2020-04-06T21:22:51.9623495Z == clock drift check ==
2020-04-06T21:22:51.9634797Z   local time: Mon Apr  6 21:22:51 UTC 2020
2020-04-06T21:22:52.0268527Z   network time: Mon, 06 Apr 2020 21:22:52 GMT
2020-04-06T21:22:52.0294031Z Starting sccache server...
2020-04-06T21:22:52.1194714Z configure: processing command line
2020-04-06T21:22:52.1195008Z configure: 
2020-04-06T21:22:52.1195819Z configure: rust.parallel-compiler := True
---
2020-04-06T21:26:48.1273541Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-06T21:26:48.2650280Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-06T21:26:48.4722016Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-06T21:26:48.5905561Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-06T21:26:49.1585631Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-06T21:26:51.7498440Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-06T21:26:52.2768211Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-06T21:26:54.5910972Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-06T21:26:55.0480034Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-06T21:27:32.6762250Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2020-04-06T21:27:33.2500490Z error: unused import: `self`
2020-04-06T21:27:33.2501220Z   --> src/librustc_mir/transform/simplify.rs:35:24
2020-04-06T21:27:33.2501739Z    |
2020-04-06T21:27:33.2502397Z 35 | use rustc_middle::ty::{self, TyCtxt};
2020-04-06T21:27:33.2503774Z    |
2020-04-06T21:27:33.2504396Z    = note: `-D unused-imports` implied by `-D warnings`
2020-04-06T21:27:33.2504732Z 
2020-04-06T21:27:33.3034623Z     Checking rustc_mir_build v0.0.0 (/checkout/src/librustc_mir_build)
---
2020-04-06T21:27:40.2378474Z 
2020-04-06T21:27:40.2379281Z To learn more, run the command again with --verbose.
2020-04-06T21:27:40.2379841Z warning: build failed, waiting for other jobs to finish...
2020-04-06T21:27:40.4189875Z error: build failed
2020-04-06T21:27:40.4230907Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-06T21:27:40.4232342Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-06T21:27:40.4232688Z Build completed unsuccessfully in 0:04:48
2020-04-06T21:27:40.4284456Z == clock drift check ==
2020-04-06T21:27:40.4296102Z   local time: Mon Apr  6 21:27:40 UTC 2020
2020-04-06T21:27:40.4296102Z   local time: Mon Apr  6 21:27:40 UTC 2020
2020-04-06T21:27:40.7198658Z   network time: Mon, 06 Apr 2020 21:27:40 GMT
2020-04-06T21:27:41.4582829Z 
2020-04-06T21:27:41.4582829Z 
2020-04-06T21:27:41.4657841Z ##[error]Bash exited with code '1'.
2020-04-06T21:27:41.4675639Z ##[section]Finishing: Run build
2020-04-06T21:27:41.4734655Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-06T21:27:41.4739919Z Task         : Get sources
2020-04-06T21:27:41.4740310Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T21:27:41.4740635Z Version      : 1.0.0
2020-04-06T21:27:41.4740860Z Author       : Microsoft
2020-04-06T21:27:41.4740860Z Author       : Microsoft
2020-04-06T21:27:41.4741260Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-06T21:27:41.4741682Z ==============================================================================
2020-04-06T21:27:41.8543878Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-06T21:27:41.8590682Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-06T21:27:41.8686515Z Cleaning up task key
2020-04-06T21:27:41.8687797Z Start cleaning up orphan processes.
2020-04-06T21:27:41.8889281Z Terminate orphan process: pid (3386) (python)
2020-04-06T21:27:41.9031992Z ##[section]Finishing: Finalize Job
