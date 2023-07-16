plain
2020-04-12T20:44:27.5404702Z ========================== Starting Command Output ===========================
2020-04-12T20:44:27.5408761Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/292ac07a-0287-4f97-aff7-66710cc26e10.sh
2020-04-12T20:44:27.5409213Z 
2020-04-12T20:44:27.5414044Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-12T20:44:27.5433931Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71073/merge to s
2020-04-12T20:44:27.5437038Z Task         : Get sources
2020-04-12T20:44:27.5437283Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-12T20:44:27.5437539Z Version      : 1.0.0
2020-04-12T20:44:27.5437699Z Author       : Microsoft
---
2020-04-12T20:44:28.5548322Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-12T20:44:28.5553372Z ##[command]git config gc.auto 0
2020-04-12T20:44:28.5557126Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-12T20:44:28.5560727Z ##[command]git config --get-all http.proxy
2020-04-12T20:44:28.5567510Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71073/merge:refs/remotes/pull/71073/merge
---
2020-04-12T20:46:26.3304231Z Looks like docker image is the same as before, not uploading
2020-04-12T20:46:33.9569866Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-12T20:46:33.9858138Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-12T20:46:33.9887737Z == clock drift check ==
2020-04-12T20:46:33.9892235Z   local time: Sun Apr 12 20:46:33 UTC 2020
2020-04-12T20:46:34.2984154Z   network time: Sun, 12 Apr 2020 20:46:34 GMT
2020-04-12T20:46:34.3019212Z Starting sccache server...
2020-04-12T20:46:34.3741156Z configure: processing command line
2020-04-12T20:46:34.3741476Z configure: 
2020-04-12T20:46:34.3742749Z configure: rust.dist-src        := False
---
2020-04-12T20:51:09.2738048Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-12T20:51:10.5015510Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-12T20:51:11.7790285Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-12T20:51:12.4005846Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-12T20:51:20.0351765Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-12T20:51:21.5254109Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-12T20:51:25.3053406Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-12T20:51:28.7461028Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-12T20:51:37.1833146Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-12T21:10:34.6814755Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-12T21:10:36.1445215Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-12T21:10:37.8963475Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-12T21:10:39.1822721Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-12T21:10:48.7932591Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-12T21:10:50.8720208Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-12T21:10:55.5417693Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-12T21:11:00.2715522Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-12T21:11:10.9456778Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-12T21:28:47.8187029Z error: internal compiler error: unexpected panic
2020-04-12T21:28:47.8188865Z 
2020-04-12T21:28:47.8194484Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-12T21:28:47.8196757Z 
2020-04-12T21:28:47.8201671Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-12T21:28:47.8205820Z note: rustc 1.44.0-nightly (f3d3c834b 2020-04-12) running on x86_64-unknown-linux-gnu
2020-04-12T21:28:47.8206668Z 
2020-04-12T21:28:47.8206668Z 
2020-04-12T21:28:47.8210839Z note: compiler flags: -Z macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-04-12T21:28:47.8214706Z note: some of the compiler flags provided by cargo are hidden
2020-04-12T21:28:47.8216920Z 
2020-04-12T21:28:47.8392512Z error: could not compile `rustc_interface`.
2020-04-12T21:28:47.8392773Z 
---
2020-04-12T21:28:58.4152840Z expected success, got: exit code: 101
2020-04-12T21:28:58.4165753Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-12T21:28:58.4166145Z Build completed unsuccessfully in 0:40:53
2020-04-12T21:28:58.4213665Z == clock drift check ==
2020-04-12T21:28:58.4235752Z   local time: Sun Apr 12 21:28:58 UTC 2020
2020-04-12T21:28:58.6209513Z   network time: Sun, 12 Apr 2020 21:28:58 GMT
2020-04-12T21:28:59.8453630Z 
2020-04-12T21:28:59.8453630Z 
2020-04-12T21:28:59.8519054Z ##[error]Bash exited with code '1'.
2020-04-12T21:28:59.8530919Z ##[section]Finishing: Run build
2020-04-12T21:28:59.8573375Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71073/merge to s
2020-04-12T21:28:59.8577641Z Task         : Get sources
2020-04-12T21:28:59.8577932Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-12T21:28:59.8578211Z Version      : 1.0.0
2020-04-12T21:28:59.8578397Z Author       : Microsoft
2020-04-12T21:28:59.8578397Z Author       : Microsoft
2020-04-12T21:28:59.8578805Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-12T21:28:59.8579304Z ==============================================================================
2020-04-12T21:29:00.1541840Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-12T21:29:00.1592218Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71073/merge to s
2020-04-12T21:29:00.1679543Z Cleaning up task key
2020-04-12T21:29:00.1680608Z Start cleaning up orphan processes.
2020-04-12T21:29:00.1840435Z Terminate orphan process: pid (3775) (python)
2020-04-12T21:29:00.1974020Z ##[section]Finishing: Finalize Job
