plain
2020-04-01T02:57:36.4458532Z ========================== Starting Command Output ===========================
2020-04-01T02:57:36.4463259Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/aac3c811-40a9-44dd-acb5-feaa4414b9b4.sh
2020-04-01T02:57:36.4463784Z 
2020-04-01T02:57:36.4468749Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-01T02:57:36.4492400Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70641/merge to s
2020-04-01T02:57:36.4496009Z Task         : Get sources
2020-04-01T02:57:36.4496274Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T02:57:36.4496518Z Version      : 1.0.0
2020-04-01T02:57:36.4496682Z Author       : Microsoft
---
2020-04-01T02:57:38.4563222Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-01T02:57:38.4570100Z ##[command]git config gc.auto 0
2020-04-01T02:57:38.4574613Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-01T02:57:38.4579113Z ##[command]git config --get-all http.proxy
2020-04-01T02:57:38.4588650Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70641/merge:refs/remotes/pull/70641/merge
---
2020-04-01T03:01:34.3796997Z  ---> 3fc1b512c57b
2020-04-01T03:01:34.3798883Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-01T03:01:34.3799283Z  ---> Using cache
2020-04-01T03:01:34.3799610Z  ---> 5ee4295733f4
2020-04-01T03:01:34.3801452Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-01T03:01:34.3805805Z  ---> 3d07a0fa42fe
2020-04-01T03:01:34.3807692Z Successfully built 3d07a0fa42fe
2020-04-01T03:01:34.3837387Z Successfully tagged rust-ci:latest
2020-04-01T03:01:34.5123737Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-01T03:01:34.5123737Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-01T03:01:34.5139059Z Looks like docker image is the same as before, not uploading
2020-04-01T03:01:43.2619832Z [CI_JOB_NAME=mingw-check]
2020-04-01T03:01:43.2856445Z [CI_JOB_NAME=mingw-check]
2020-04-01T03:01:43.2884905Z == clock drift check ==
2020-04-01T03:01:43.2894978Z   local time: Wed Apr  1 03:01:43 UTC 2020
2020-04-01T03:01:43.5783969Z   network time: Wed, 01 Apr 2020 03:01:43 GMT
2020-04-01T03:01:43.5814550Z Starting sccache server...
2020-04-01T03:01:43.6651910Z configure: processing command line
2020-04-01T03:01:43.6652466Z configure: 
2020-04-01T03:01:43.6653382Z configure: rust.parallel-compiler := True
---
2020-04-01T03:05:09.7374462Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-01T03:05:09.8461117Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-01T03:05:10.0236839Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-01T03:05:10.1100161Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-01T03:05:10.6072564Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-01T03:05:12.6636372Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-01T03:05:13.1034207Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-01T03:05:14.9605554Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-01T03:05:15.3627014Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-01T03:05:36.5484229Z     Checking rustc_trait_selection v0.0.0 (/checkout/src/librustc_trait_selection)
2020-04-01T03:05:36.7790151Z error[E0433]: failed to resolve: use of undeclared type or module `rustc`
2020-04-01T03:05:36.7790990Z  --> src/librustc_trait_selection/traits/util.rs:6:5
2020-04-01T03:05:36.7791517Z   |
2020-04-01T03:05:36.7792220Z 6 | use rustc::ty::subst::{GenericArg, Subst, SubstsRef};
2020-04-01T03:05:36.7793469Z 
2020-04-01T03:05:36.7967636Z error[E0433]: failed to resolve: use of undeclared type or module `rustc`
2020-04-01T03:05:36.7968564Z  --> src/librustc_trait_selection/traits/util.rs:7:5
2020-04-01T03:05:36.7969126Z   |
2020-04-01T03:05:36.7969126Z   |
2020-04-01T03:05:36.7969829Z 7 | use rustc::ty::{self, ToPredicate, Ty, TyCtxt, WithConstness};
2020-04-01T03:05:36.7971105Z 
2020-04-01T03:05:36.7971633Z error[E0432]: unresolved import `rustc`
2020-04-01T03:05:36.7972286Z  --> src/librustc_trait_selection/traits/util.rs:7:5
2020-04-01T03:05:36.7972810Z   |
2020-04-01T03:05:36.7972810Z   |
2020-04-01T03:05:36.7973692Z 7 | use rustc::ty::{self, ToPredicate, Ty, TyCtxt, WithConstness};
2020-04-01T03:05:36.7975053Z 
2020-04-01T03:05:36.9163677Z error: unused import: `rustc_middle::ty::outlives::Component`
2020-04-01T03:05:36.9164498Z   --> src/librustc_trait_selection/traits/util.rs:11:5
2020-04-01T03:05:36.9165029Z    |
---
2020-04-01T03:05:36.9167971Z 
2020-04-01T03:05:36.9168459Z error: unused import: `ToPolyTraitRef`
2020-04-01T03:05:36.9169121Z   --> src/librustc_trait_selection/traits/util.rs:13:30
2020-04-01T03:05:36.9169637Z    |
2020-04-01T03:05:36.9170410Z 13 | use rustc_middle::ty::{self, ToPolyTraitRef, ToPredicate, Ty, TyCtxt, WithConstness};
2020-04-01T03:05:36.9171543Z 
2020-04-01T03:05:38.2239041Z error: aborting due to 5 previous errors
2020-04-01T03:05:38.2239343Z 
2020-04-01T03:05:38.2239826Z Some errors have detailed explanations: E0432, E0433.
2020-04-01T03:05:38.2239826Z Some errors have detailed explanations: E0432, E0433.
2020-04-01T03:05:38.2240477Z For more information about an error, try `rustc --explain E0432`.
2020-04-01T03:05:38.2282138Z error: could not compile `rustc_trait_selection`.
2020-04-01T03:05:38.2282408Z 
2020-04-01T03:05:38.2282802Z To learn more, run the command again with --verbose.
2020-04-01T03:05:38.2283399Z warning: build failed, waiting for other jobs to finish...
2020-04-01T03:05:38.6906371Z error: build failed
2020-04-01T03:05:38.6942439Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-01T03:05:38.6956989Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-01T03:05:38.6957659Z Build completed unsuccessfully in 0:03:55
2020-04-01T03:05:38.7009369Z == clock drift check ==
2020-04-01T03:05:38.7030996Z   local time: Wed Apr  1 03:05:38 UTC 2020
2020-04-01T03:05:38.7030996Z   local time: Wed Apr  1 03:05:38 UTC 2020
2020-04-01T03:05:38.8674216Z   network time: Wed, 01 Apr 2020 03:05:38 GMT
2020-04-01T03:05:39.6625646Z 
2020-04-01T03:05:39.6625646Z 
2020-04-01T03:05:39.6686701Z ##[error]Bash exited with code '1'.
2020-04-01T03:05:39.6699851Z ##[section]Finishing: Run build
2020-04-01T03:05:39.6743612Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70641/merge to s
2020-04-01T03:05:39.6747720Z Task         : Get sources
2020-04-01T03:05:39.6747978Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T03:05:39.6748201Z Version      : 1.0.0
2020-04-01T03:05:39.6748366Z Author       : Microsoft
2020-04-01T03:05:39.6748366Z Author       : Microsoft
2020-04-01T03:05:39.6748628Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-01T03:05:39.6748911Z ==============================================================================
2020-04-01T03:05:39.9812535Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-01T03:05:39.9856562Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70641/merge to s
2020-04-01T03:05:39.9948414Z Cleaning up task key
2020-04-01T03:05:39.9949736Z Start cleaning up orphan processes.
2020-04-01T03:05:40.0243892Z Terminate orphan process: pid (3944) (python)
2020-04-01T03:05:40.0275807Z ##[section]Finishing: Finalize Job
