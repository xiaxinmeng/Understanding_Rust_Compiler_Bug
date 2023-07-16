plain
2020-04-27T00:15:32.3078543Z ========================== Starting Command Output ===========================
2020-04-27T00:15:32.3080923Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/874af0cf-3187-4152-8dde-37d79a9b1b55.sh
2020-04-27T00:15:32.3081187Z 
2020-04-27T00:15:32.3084492Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-27T00:15:32.3101913Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71338/merge to s
2020-04-27T00:15:32.3105004Z Task         : Get sources
2020-04-27T00:15:32.3105280Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-27T00:15:32.3105552Z Version      : 1.0.0
2020-04-27T00:15:32.3105752Z Author       : Microsoft
---
2020-04-27T00:15:33.3078449Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-27T00:15:33.3083955Z ##[command]git config gc.auto 0
2020-04-27T00:15:33.3087492Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-27T00:15:33.3090748Z ##[command]git config --get-all http.proxy
2020-04-27T00:15:33.3096688Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71338/merge:refs/remotes/pull/71338/merge
---
2020-04-27T00:18:47.3154849Z  ---> f7353ccad5b1
2020-04-27T00:18:47.3163681Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-27T00:18:47.3164366Z  ---> Using cache
2020-04-27T00:18:47.3164742Z  ---> ed38efbaa060
2020-04-27T00:18:47.3166090Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-27T00:18:47.3175317Z  ---> c5008ef7ae8e
2020-04-27T00:18:47.3209277Z Successfully built c5008ef7ae8e
2020-04-27T00:18:47.3286236Z Successfully tagged rust-ci:latest
2020-04-27T00:18:47.3858841Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-27T00:18:47.3858841Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-27T00:18:47.3874348Z Looks like docker image is the same as before, not uploading
2020-04-27T00:18:55.7831823Z [CI_JOB_NAME=mingw-check]
2020-04-27T00:18:55.8058785Z [CI_JOB_NAME=mingw-check]
2020-04-27T00:18:55.8091889Z == clock drift check ==
2020-04-27T00:18:55.8096203Z   local time: Mon Apr 27 00:18:55 UTC 2020
2020-04-27T00:18:55.9659294Z   network time: Mon, 27 Apr 2020 00:18:55 GMT
2020-04-27T00:18:55.9690771Z Starting sccache server...
2020-04-27T00:18:56.0845041Z configure: processing command line
2020-04-27T00:18:56.0845359Z configure: 
2020-04-27T00:18:56.0846255Z configure: rust.parallel-compiler := True
---
2020-04-27T00:21:22.6455797Z     Checking unicode-width v0.1.6
2020-04-27T00:21:22.7254595Z     Checking getopts v0.2.21
2020-04-27T00:21:24.9450988Z     Checking test v0.0.0 (/checkout/src/libtest)
2020-04-27T00:21:25.6756755Z     Finished release [optimized] target(s) in 28.10s
2020-04-27T00:21:25.6764151Z {"reason":"build-finished","success":true}
2020-04-27T00:21:26.3118026Z     Checking cfg-if v0.1.10
2020-04-27T00:21:26.3118634Z    Compiling libc v0.2.69
2020-04-27T00:21:26.3491328Z    Compiling semver-parser v0.7.0
2020-04-27T00:21:27.2204399Z     Checking lazy_static v1.4.0
---
2020-04-27T00:23:03.4266233Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-27T00:23:03.5208071Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-27T00:23:03.7236087Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-27T00:23:03.8717887Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-27T00:23:04.3687963Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-27T00:23:06.6863685Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-27T00:23:07.1805897Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-27T00:23:09.3196629Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-27T00:23:09.7506316Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-27T00:23:38.4831511Z     Checking rustc_mir_build v0.0.0 (/checkout/src/librustc_mir_build)
2020-04-27T00:23:39.7574618Z error[E0308]: mismatched types
2020-04-27T00:23:39.7575318Z     --> src/librustc_typeck/check/mod.rs:1628:34
2020-04-27T00:23:39.7576059Z      |
2020-04-27T00:23:39.7581312Z 1628 |     check_opaque_for_cycles(tcx, def_id, substs, span, origin);
2020-04-27T00:23:39.7583021Z 
2020-04-27T00:23:39.8201519Z error[E0308]: mismatched types
2020-04-27T00:23:39.8202188Z     --> src/librustc_typeck/check/mod.rs:1712:26
2020-04-27T00:23:39.8202691Z      |
2020-04-27T00:23:39.8202691Z      |
2020-04-27T00:23:39.8203251Z 1712 |         .as_local_hir_id(def_id)
2020-04-27T00:23:39.8204818Z 
2020-04-27T00:23:39.8255415Z error[E0599]: no method named `and_then` found for struct `rustc_hir::hir_id::HirId` in the current scope
2020-04-27T00:23:39.8256230Z     --> src/librustc_typeck/check/mod.rs:1713:10
2020-04-27T00:23:39.8256738Z      |
2020-04-27T00:23:39.8256738Z      |
2020-04-27T00:23:39.8257271Z 1713 |         .and_then(|hir_id| {
2020-04-27T00:23:39.8258124Z      |          ^^^^^^^^ method not found in `rustc_hir::hir_id::HirId`
2020-04-27T00:23:39.8907533Z error[E0308]: mismatched types
2020-04-27T00:23:39.8908228Z     --> src/librustc_typeck/check/mod.rs:1821:53
2020-04-27T00:23:39.8908730Z      |
2020-04-27T00:23:39.8908730Z      |
2020-04-27T00:23:39.8909352Z 1821 |     if let Some(hir_id) = tcx.hir().as_local_hir_id(def_id) {
2020-04-27T00:23:39.8911069Z 
2020-04-27T00:23:39.8917914Z error[E0308]: mismatched types
2020-04-27T00:23:39.8918544Z     --> src/librustc_typeck/check/mod.rs:1821:12
2020-04-27T00:23:39.8919018Z      |
2020-04-27T00:23:39.8919018Z      |
2020-04-27T00:23:39.8919635Z 1821 |     if let Some(hir_id) = tcx.hir().as_local_hir_id(def_id) {
2020-04-27T00:23:39.8920772Z      |            ^^^^^^^^^^^^   --------------------------------- this expression has type `rustc_hir::hir_id::HirId`
2020-04-27T00:23:39.8922624Z      |            expected struct `rustc_hir::hir_id::HirId`, found enum `std::option::Option`
2020-04-27T00:23:39.8923256Z      |
2020-04-27T00:23:39.8923904Z      = note: expected struct `rustc_hir::hir_id::HirId`
2020-04-27T00:23:39.8924623Z                   found enum `std::option::Option<_>`
2020-04-27T00:23:39.8924623Z                   found enum `std::option::Option<_>`
2020-04-27T00:23:39.8924869Z 
2020-04-27T00:23:39.9492961Z error[E0308]: mismatched types
2020-04-27T00:23:39.9493818Z     --> src/librustc_typeck/check/mod.rs:1846:65
2020-04-27T00:23:39.9494312Z      |
2020-04-27T00:23:39.9495010Z 1846 |                     if let Some(ty) = tcx.hir().as_local_hir_id(def_id).and_then(|hir_id| {
2020-04-27T00:23:39.9496841Z 
2020-04-27T00:23:39.9536458Z error[E0599]: no method named `and_then` found for struct `rustc_hir::hir_id::HirId` in the current scope
2020-04-27T00:23:39.9537290Z     --> src/librustc_typeck/check/mod.rs:1846:73
2020-04-27T00:23:39.9537767Z      |
2020-04-27T00:23:39.9537767Z      |
2020-04-27T00:23:39.9538495Z 1846 |                     if let Some(ty) = tcx.hir().as_local_hir_id(def_id).and_then(|hir_id| {
2020-04-27T00:23:39.9539613Z      |                                                                         ^^^^^^^^ method not found in `rustc_hir::hir_id::HirId`
2020-04-27T00:23:41.2542167Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2020-04-27T00:23:42.2279617Z error: aborting due to 7 previous errors
2020-04-27T00:23:42.2279891Z 
2020-04-27T00:23:42.2280346Z Some errors have detailed explanations: E0308, E0599.
2020-04-27T00:23:42.2280346Z Some errors have detailed explanations: E0308, E0599.
2020-04-27T00:23:42.2280956Z For more information about an error, try `rustc --explain E0308`.
2020-04-27T00:23:42.2345554Z error: could not compile `rustc_typeck`.
2020-04-27T00:23:42.2345821Z 
2020-04-27T00:23:42.2346300Z To learn more, run the command again with --verbose.
2020-04-27T00:23:42.2346856Z warning: build failed, waiting for other jobs to finish...
2020-04-27T00:23:42.7818165Z {"reason":"build-finished","success":false}
2020-04-27T00:23:42.7930365Z error: build failed
2020-04-27T00:23:42.7961052Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-27T00:23:42.7974715Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-27T00:23:42.7975037Z Build completed unsuccessfully in 0:04:46
2020-04-27T00:23:42.8086695Z == clock drift check ==
2020-04-27T00:23:42.8103500Z   local time: Mon Apr 27 00:23:42 UTC 2020
2020-04-27T00:23:42.8103500Z   local time: Mon Apr 27 00:23:42 UTC 2020
2020-04-27T00:23:43.1018974Z   network time: Mon, 27 Apr 2020 00:23:43 GMT
2020-04-27T00:23:43.7510168Z 
2020-04-27T00:23:43.7510168Z 
2020-04-27T00:23:43.7587545Z ##[error]Bash exited with code '1'.
2020-04-27T00:23:43.7601631Z ##[section]Finishing: Run build
2020-04-27T00:23:43.7779125Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71338/merge to s
2020-04-27T00:23:43.7784035Z Task         : Get sources
2020-04-27T00:23:43.7784364Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-27T00:23:43.7784651Z Version      : 1.0.0
2020-04-27T00:23:43.7784863Z Author       : Microsoft
2020-04-27T00:23:43.7784863Z Author       : Microsoft
2020-04-27T00:23:43.7785195Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-27T00:23:43.7785560Z ==============================================================================
2020-04-27T00:23:44.1206769Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-27T00:23:44.1255115Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71338/merge to s
2020-04-27T00:23:44.1352589Z Cleaning up task key
2020-04-27T00:23:44.1354153Z Start cleaning up orphan processes.
2020-04-27T00:23:44.1532959Z Terminate orphan process: pid (4738) (python)
2020-04-27T00:23:44.2174401Z ##[section]Finishing: Finalize Job
