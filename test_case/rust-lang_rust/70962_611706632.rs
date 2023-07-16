plain
2020-04-09T18:51:47.0318107Z ========================== Starting Command Output ===========================
2020-04-09T18:51:47.0321175Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6fd3cee6-5802-4abe-a08b-5829be9219b4.sh
2020-04-09T18:51:47.0321491Z 
2020-04-09T18:51:47.0326300Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-09T18:51:47.0345386Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70962/merge to s
2020-04-09T18:51:47.0348517Z Task         : Get sources
2020-04-09T18:51:47.0349015Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-09T18:51:47.0349648Z Version      : 1.0.0
2020-04-09T18:51:47.0349833Z Author       : Microsoft
---
2020-04-09T18:51:48.2068274Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-09T18:51:48.2143317Z ##[command]git config gc.auto 0
2020-04-09T18:51:48.2147789Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-09T18:51:48.2152495Z ##[command]git config --get-all http.proxy
2020-04-09T18:51:48.2162216Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70962/merge:refs/remotes/pull/70962/merge
---
2020-04-09T18:56:06.8687223Z  ---> 3fc1b512c57b
2020-04-09T18:56:06.8687636Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-09T18:56:06.8688077Z  ---> Using cache
2020-04-09T18:56:06.8688412Z  ---> 5ee4295733f4
2020-04-09T18:56:06.8690676Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-09T18:56:06.8692364Z  ---> 3d07a0fa42fe
2020-04-09T18:56:06.8692571Z Successfully built 3d07a0fa42fe
2020-04-09T18:56:06.8714530Z Successfully tagged rust-ci:latest
2020-04-09T18:56:07.0365781Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-09T18:56:07.0365781Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-09T18:56:07.0381491Z Looks like docker image is the same as before, not uploading
2020-04-09T18:56:13.3335020Z [CI_JOB_NAME=mingw-check]
2020-04-09T18:56:13.3609801Z [CI_JOB_NAME=mingw-check]
2020-04-09T18:56:13.3639990Z == clock drift check ==
2020-04-09T18:56:13.3652515Z   local time: Thu Apr  9 18:56:13 UTC 2020
2020-04-09T18:56:13.5012331Z   network time: Thu, 09 Apr 2020 18:56:13 GMT
2020-04-09T18:56:13.5037257Z Starting sccache server...
2020-04-09T18:56:13.5901442Z configure: processing command line
2020-04-09T18:56:13.5901678Z configure: 
2020-04-09T18:56:13.5902676Z configure: rust.parallel-compiler := True
---
2020-04-09T19:00:02.9331070Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-09T19:00:07.6570804Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-09T19:00:08.9176621Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-09T19:00:08.9222870Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-09T19:00:09.1460597Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-09T19:00:10.0133732Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-09T19:00:10.0427925Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-09T19:00:11.5910706Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-09T19:00:12.1026665Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-09T19:00:53.8318264Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2020-04-09T19:00:54.8057841Z error[E0412]: cannot find type `Tag` in this scope
2020-04-09T19:00:54.8059058Z    --> src/librustc_mir/interpret/machine.rs:260:28
2020-04-09T19:00:54.8060059Z     |
2020-04-09T19:00:54.8061072Z 260 |         alloc: &Allocation<Tag, AllocExtra>,
2020-04-09T19:00:54.8064249Z     |
2020-04-09T19:00:54.8064249Z     |
2020-04-09T19:00:54.8064978Z help: there is an enum variant `rustc_target::abi::DiscriminantKind::Tag`; try using the variant's enum
2020-04-09T19:00:54.8065595Z     |
2020-04-09T19:00:54.8066319Z 260 |         alloc: &Allocation<rustc_target::abi::DiscriminantKind, AllocExtra>,
2020-04-09T19:00:54.8068158Z help: you might be missing a type parameter
2020-04-09T19:00:54.8068656Z     |
2020-04-09T19:00:54.8068656Z     |
2020-04-09T19:00:54.8069343Z 80  | pub trait Machine<'mir, 'tcx, Tag>: Sized {
2020-04-09T19:00:54.8074444Z 
2020-04-09T19:00:54.8084112Z error[E0412]: cannot find type `AllocExtra` in this scope
2020-04-09T19:00:54.8084823Z    --> src/librustc_mir/interpret/machine.rs:260:33
2020-04-09T19:00:54.8085327Z     |
2020-04-09T19:00:54.8085327Z     |
2020-04-09T19:00:54.8085977Z 80  | pub trait Machine<'mir, 'tcx>: Sized {
2020-04-09T19:00:54.8087018Z     |                             - help: you might be missing a type parameter: `, AllocExtra`
2020-04-09T19:00:54.8088247Z ...
2020-04-09T19:00:54.8088904Z 260 |         alloc: &Allocation<Tag, AllocExtra>,
2020-04-09T19:00:54.8094777Z 
2020-04-09T19:00:54.8110181Z error[E0412]: cannot find type `Tag` in this scope
2020-04-09T19:00:54.8110935Z    --> src/librustc_mir/interpret/machine.rs:261:22
2020-04-09T19:00:54.8111466Z     |
2020-04-09T19:00:54.8111466Z     |
2020-04-09T19:00:54.8112075Z 261 |         ptr: Pointer<Tag>,
2020-04-09T19:00:54.8113066Z     |                      ^^^ not found in this scope
2020-04-09T19:00:54.8113670Z     |
2020-04-09T19:00:54.8114539Z help: there is an enum variant `rustc_target::abi::DiscriminantKind::Tag`; try using the variant's enum
2020-04-09T19:00:54.8115233Z     |
2020-04-09T19:00:54.8115911Z 261 |         ptr: Pointer<rustc_target::abi::DiscriminantKind>,
2020-04-09T19:00:54.8117408Z help: you might be missing a type parameter
2020-04-09T19:00:54.8117884Z     |
2020-04-09T19:00:54.8117884Z     |
2020-04-09T19:00:54.8118520Z 80  | pub trait Machine<'mir, 'tcx, Tag>: Sized {
2020-04-09T19:00:54.8123545Z 
2020-04-09T19:00:57.8491663Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-09T19:00:58.0419823Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-09T19:01:01.8843387Z error: aborting due to 3 previous errors
2020-04-09T19:01:01.8843387Z error: aborting due to 3 previous errors
2020-04-09T19:01:01.8844369Z 
2020-04-09T19:01:01.8845232Z For more information about this error, try `rustc --explain E0412`.
2020-04-09T19:01:01.9094188Z error: could not compile `rustc_mir`.
2020-04-09T19:01:01.9094936Z 
2020-04-09T19:01:01.9096001Z To learn more, run the command again with --verbose.
2020-04-09T19:01:01.9125453Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-09T19:01:01.9140640Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-09T19:01:01.9141489Z Build completed unsuccessfully in 0:04:48
2020-04-09T19:01:01.9200760Z == clock drift check ==
2020-04-09T19:01:01.9219508Z   local time: Thu Apr  9 19:01:01 UTC 2020
2020-04-09T19:01:01.9219508Z   local time: Thu Apr  9 19:01:01 UTC 2020
2020-04-09T19:01:01.9632438Z   network time: Thu, 09 Apr 2020 19:01:01 GMT
2020-04-09T19:01:02.7310179Z 
2020-04-09T19:01:02.7310179Z 
2020-04-09T19:01:02.7386617Z ##[error]Bash exited with code '1'.
2020-04-09T19:01:02.7399178Z ##[section]Finishing: Run build
2020-04-09T19:01:02.7449936Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70962/merge to s
2020-04-09T19:01:02.7454682Z Task         : Get sources
2020-04-09T19:01:02.7455002Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-09T19:01:02.7455309Z Version      : 1.0.0
2020-04-09T19:01:02.7455510Z Author       : Microsoft
2020-04-09T19:01:02.7455510Z Author       : Microsoft
2020-04-09T19:01:02.7455834Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-09T19:01:02.7456226Z ==============================================================================
2020-04-09T19:01:03.0924912Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-09T19:01:03.0967742Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70962/merge to s
2020-04-09T19:01:03.1055358Z Cleaning up task key
2020-04-09T19:01:03.1056586Z Start cleaning up orphan processes.
2020-04-09T19:01:03.1301259Z Terminate orphan process: pid (3871) (python)
2020-04-09T19:01:03.1461627Z ##[section]Finishing: Finalize Job
