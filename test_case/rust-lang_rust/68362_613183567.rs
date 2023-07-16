plain
2020-04-14T01:08:49.4434593Z ========================== Starting Command Output ===========================
2020-04-14T01:08:49.4436934Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0a0a3c9d-2e7b-44ba-9af0-b6fc8f49b737.sh
2020-04-14T01:08:49.4437193Z 
2020-04-14T01:08:49.4441431Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-14T01:08:49.4460500Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-04-14T01:08:49.4463541Z Task         : Get sources
2020-04-14T01:08:49.4463818Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T01:08:49.4464104Z Version      : 1.0.0
2020-04-14T01:08:49.4464291Z Author       : Microsoft
---
2020-04-14T01:08:50.4396929Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-14T01:08:50.4406214Z ##[command]git config gc.auto 0
2020-04-14T01:08:50.4412389Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-14T01:08:50.4418042Z ##[command]git config --get-all http.proxy
2020-04-14T01:08:50.4427545Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68362/merge:refs/remotes/pull/68362/merge
---
2020-04-14T01:10:59.7071106Z  ---> f58a2bb1e753
2020-04-14T01:10:59.7072678Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-14T01:10:59.7073958Z  ---> Using cache
2020-04-14T01:10:59.7074786Z  ---> d079cc6b6db8
2020-04-14T01:10:59.7076679Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-14T01:10:59.7078984Z  ---> 4183ca46ee56
2020-04-14T01:10:59.7079808Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-14T01:10:59.7083765Z  ---> Using cache
2020-04-14T01:10:59.7084358Z  ---> 69e7f8a2a2fb
---
2020-04-14T01:10:59.7493719Z Looks like docker image is the same as before, not uploading
2020-04-14T01:11:06.0286590Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-14T01:11:06.0551385Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-14T01:11:06.0575187Z == clock drift check ==
2020-04-14T01:11:06.0582511Z   local time: Tue Apr 14 01:11:06 UTC 2020
2020-04-14T01:11:06.2102942Z   network time: Tue, 14 Apr 2020 01:11:06 GMT
2020-04-14T01:11:06.2126684Z Starting sccache server...
2020-04-14T01:11:06.2936247Z configure: processing command line
2020-04-14T01:11:06.2936919Z configure: 
2020-04-14T01:11:06.2937972Z configure: rust.dist-src        := False
---
2020-04-14T01:15:58.5953506Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-14T01:15:59.8518087Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-14T01:16:01.2894255Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-14T01:16:02.0386404Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-14T01:16:10.4082297Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T01:16:12.1833480Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-14T01:16:16.1812174Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-14T01:16:19.9441536Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-14T01:16:28.7872718Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-14T01:36:52.2852283Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-14T01:36:53.9321079Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-14T01:36:55.7852891Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-14T01:36:57.0528128Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-14T01:37:07.1343806Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T01:37:09.9145400Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-14T01:37:14.4925726Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-14T01:37:19.5861298Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-14T01:37:30.1159368Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-14T01:56:14.7585355Z error: internal compiler error: unexpected panic
2020-04-14T01:56:14.7586989Z 
2020-04-14T01:56:14.7592037Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-14T01:56:14.7594487Z 
2020-04-14T01:56:14.7599403Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-14T01:56:14.7603257Z note: rustc 1.44.0-nightly (89b73000f 2020-04-14) running on x86_64-unknown-linux-gnu
2020-04-14T01:56:14.7604500Z 
2020-04-14T01:56:14.7604500Z 
2020-04-14T01:56:14.7609603Z note: compiler flags: -Z macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-04-14T01:56:14.7612604Z note: some of the compiler flags provided by cargo are hidden
2020-04-14T01:56:14.7614022Z 
2020-04-14T01:56:14.7784905Z error: could not compile `rustc_interface`.
2020-04-14T01:56:14.7785934Z 
---
2020-04-14T01:56:29.0163574Z expected success, got: exit code: 101
2020-04-14T01:56:29.0170422Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-14T01:56:29.0170968Z Build completed unsuccessfully in 0:43:48
2020-04-14T01:56:29.0220915Z == clock drift check ==
2020-04-14T01:56:29.0242751Z   local time: Tue Apr 14 01:56:29 UTC 2020
2020-04-14T01:56:29.2165864Z   network time: Tue, 14 Apr 2020 01:56:29 GMT
2020-04-14T01:56:30.3915784Z 
2020-04-14T01:56:30.3915784Z 
2020-04-14T01:56:30.4005755Z ##[error]Bash exited with code '1'.
2020-04-14T01:56:30.4020628Z ##[section]Finishing: Run build
2020-04-14T01:56:30.4059552Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-04-14T01:56:30.4064075Z Task         : Get sources
2020-04-14T01:56:30.4064363Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T01:56:30.4064626Z Version      : 1.0.0
2020-04-14T01:56:30.4065173Z Author       : Microsoft
2020-04-14T01:56:30.4065173Z Author       : Microsoft
2020-04-14T01:56:30.4065476Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-14T01:56:30.4065811Z ==============================================================================
2020-04-14T01:56:30.7245049Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-14T01:56:30.7248932Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-04-14T01:56:30.7340135Z Cleaning up task key
2020-04-14T01:56:30.7341354Z Start cleaning up orphan processes.
2020-04-14T01:56:30.7509593Z Terminate orphan process: pid (3755) (python)
2020-04-14T01:56:30.7652883Z ##[section]Finishing: Finalize Job
