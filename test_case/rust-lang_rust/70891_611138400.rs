plain
2020-04-08T18:46:17.2832629Z ========================== Starting Command Output ===========================
2020-04-08T18:46:17.2835441Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/05128bb8-448d-4441-a5c6-4215da2cacc8.sh
2020-04-08T18:46:17.2835695Z 
2020-04-08T18:46:17.2839884Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-08T18:46:17.2858310Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70891/merge to s
2020-04-08T18:46:17.2861688Z Task         : Get sources
2020-04-08T18:46:17.2861995Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T18:46:17.2862275Z Version      : 1.0.0
2020-04-08T18:46:17.2862461Z Author       : Microsoft
---
2020-04-08T18:46:18.5422940Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-08T18:46:18.5428743Z ##[command]git config gc.auto 0
2020-04-08T18:46:18.5432193Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-08T18:46:18.5435356Z ##[command]git config --get-all http.proxy
2020-04-08T18:46:18.5441694Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70891/merge:refs/remotes/pull/70891/merge
---
2020-04-08T18:49:15.3610984Z  ---> 3fc1b512c57b
2020-04-08T18:49:15.3611223Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-08T18:49:15.3611617Z  ---> Using cache
2020-04-08T18:49:15.3611946Z  ---> 5ee4295733f4
2020-04-08T18:49:15.3613541Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-08T18:49:15.3615106Z  ---> 3d07a0fa42fe
2020-04-08T18:49:15.3615336Z Successfully built 3d07a0fa42fe
2020-04-08T18:49:15.3631704Z Successfully tagged rust-ci:latest
2020-04-08T18:49:15.3887495Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-08T18:49:15.3887495Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-08T18:49:15.3902952Z Looks like docker image is the same as before, not uploading
2020-04-08T18:49:22.4516799Z [CI_JOB_NAME=mingw-check]
2020-04-08T18:49:22.4743183Z [CI_JOB_NAME=mingw-check]
2020-04-08T18:49:22.4766763Z == clock drift check ==
2020-04-08T18:49:22.4774705Z   local time: Wed Apr  8 18:49:22 UTC 2020
2020-04-08T18:49:22.7656065Z   network time: Wed, 08 Apr 2020 18:49:22 GMT
2020-04-08T18:49:22.7694354Z Starting sccache server...
2020-04-08T18:49:22.8492671Z configure: processing command line
2020-04-08T18:49:22.8493368Z configure: 
2020-04-08T18:49:22.8494141Z configure: rust.parallel-compiler := True
---
2020-04-08T18:52:47.5131946Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-08T18:52:47.7121445Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-08T18:52:47.9082972Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-08T18:52:47.9083682Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-08T18:52:48.4849520Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-08T18:52:50.6219119Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-08T18:52:51.0467853Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-08T18:52:52.8692874Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-08T18:52:53.2721436Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-08T18:53:26.5266142Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2020-04-08T18:53:27.9094664Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2020-04-08T18:53:28.4416870Z     Checking rustc_ty v0.0.0 (/checkout/src/librustc_ty)
2020-04-08T18:53:28.7842318Z     Checking rustc_mir_build v0.0.0 (/checkout/src/librustc_mir_build)
2020-04-08T18:53:30.4690939Z error[E0382]: borrow of moved value: `self`
2020-04-08T18:53:30.4692194Z    --> src/librustc_mir_build/build/expr/as_rvalue.rs:231:68
2020-04-08T18:53:30.4693626Z 44  |         &mut self,
2020-04-08T18:53:30.4694880Z     |         --------- move occurs because `self` has type `&mut build::Builder<'_, '_>`, which does not implement the `Copy` trait
2020-04-08T18:53:30.4695818Z ...
2020-04-08T18:53:30.4696537Z 51  |         let this = self;
2020-04-08T18:53:30.4696537Z 51  |         let this = self;
2020-04-08T18:53:30.4697473Z     |                    ---- value moved here
2020-04-08T18:53:30.4698196Z ...
2020-04-08T18:53:30.4699129Z 231 |                     literal: ty::Const::zero_sized(self.hir.tcx(), self.hir.tcx().types.unit),
2020-04-08T18:53:30.4700391Z     |                                                                    ^^^^^^^^ value borrowed here after move
2020-04-08T18:53:31.2151196Z error: aborting due to previous error
2020-04-08T18:53:31.2154925Z 
2020-04-08T18:53:31.2161054Z For more information about this error, try `rustc --explain E0382`.
2020-04-08T18:53:31.2237221Z error: could not compile `rustc_mir_build`.
2020-04-08T18:53:31.2237221Z error: could not compile `rustc_mir_build`.
2020-04-08T18:53:31.2241094Z 
2020-04-08T18:53:31.2241729Z To learn more, run the command again with --verbose.
2020-04-08T18:53:31.2259440Z warning: build failed, waiting for other jobs to finish...
2020-04-08T18:53:32.9353462Z error: build failed
2020-04-08T18:53:32.9382491Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-08T18:53:32.9392614Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-08T18:53:32.9393463Z Build completed unsuccessfully in 0:04:10
2020-04-08T18:53:32.9444412Z == clock drift check ==
2020-04-08T18:53:32.9459848Z   local time: Wed Apr  8 18:53:32 UTC 2020
2020-04-08T18:53:32.9459848Z   local time: Wed Apr  8 18:53:32 UTC 2020
2020-04-08T18:53:33.4920040Z   network time: Wed, 08 Apr 2020 18:53:33 GMT
2020-04-08T18:53:34.2399299Z 
2020-04-08T18:53:34.2399299Z 
2020-04-08T18:53:34.2465184Z ##[error]Bash exited with code '1'.
2020-04-08T18:53:34.2478273Z ##[section]Finishing: Run build
2020-04-08T18:53:34.2529370Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70891/merge to s
2020-04-08T18:53:34.2534442Z Task         : Get sources
2020-04-08T18:53:34.2534805Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T18:53:34.2535121Z Version      : 1.0.0
2020-04-08T18:53:34.2535350Z Author       : Microsoft
2020-04-08T18:53:34.2535350Z Author       : Microsoft
2020-04-08T18:53:34.2535715Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-08T18:53:34.2536121Z ==============================================================================
2020-04-08T18:53:34.5627988Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-08T18:53:34.5670155Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70891/merge to s
2020-04-08T18:53:34.5757110Z Cleaning up task key
2020-04-08T18:53:34.5758413Z Start cleaning up orphan processes.
2020-04-08T18:53:34.5927309Z Terminate orphan process: pid (3386) (python)
2020-04-08T18:53:34.6050068Z ##[section]Finishing: Finalize Job
