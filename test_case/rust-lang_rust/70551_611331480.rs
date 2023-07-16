plain
2020-04-09T04:44:00.3411298Z ========================== Starting Command Output ===========================
2020-04-09T04:44:00.3414846Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/967b6962-f049-41ce-b11e-39ac71cef5f9.sh
2020-04-09T04:44:00.3415160Z 
2020-04-09T04:44:00.3419837Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-09T04:44:00.3440101Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-09T04:44:00.3443729Z Task         : Get sources
2020-04-09T04:44:00.3444036Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-09T04:44:00.3444334Z Version      : 1.0.0
2020-04-09T04:44:00.3444554Z Author       : Microsoft
---
2020-04-09T04:44:01.3354752Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-09T04:44:01.3360064Z ##[command]git config gc.auto 0
2020-04-09T04:44:01.3363593Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-09T04:44:01.3366888Z ##[command]git config --get-all http.proxy
2020-04-09T04:44:01.3372826Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70551/merge:refs/remotes/pull/70551/merge
---
2020-04-09T04:46:11.4819329Z Looks like docker image is the same as before, not uploading
2020-04-09T04:46:16.4325881Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-09T04:46:16.4598275Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-09T04:46:16.4627784Z == clock drift check ==
2020-04-09T04:46:16.4640044Z   local time: Thu Apr  9 04:46:16 UTC 2020
2020-04-09T04:46:16.5583561Z   network time: Thu, 09 Apr 2020 04:46:16 GMT
2020-04-09T04:46:16.5618546Z Starting sccache server...
2020-04-09T04:46:16.6474225Z configure: processing command line
2020-04-09T04:46:16.6475678Z configure: 
2020-04-09T04:46:16.6476911Z configure: rust.dist-src        := False
---
2020-04-09T04:51:33.5313033Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-09T04:51:35.0641847Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-09T04:51:36.6003387Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-09T04:51:36.8624041Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-09T04:51:46.4271237Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-09T04:51:48.1990449Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-09T04:51:52.6417800Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-09T04:51:56.7139379Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-09T04:52:06.5903012Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-09T05:10:12.1056793Z    Compiling rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2020-04-09T05:10:12.9907306Z    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-04-09T05:10:13.2213670Z error: internal compiler error: TyKind::Error constructed but no error reported. src/librustc_typeck/check/closure.rs:180:60
2020-04-09T05:10:13.2214207Z 
2020-04-09T05:10:13.2223411Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:360:17
2020-04-09T05:10:13.2229408Z 
2020-04-09T05:10:13.2235235Z error: internal compiler error: unexpected panic
2020-04-09T05:10:13.2235441Z 
2020-04-09T05:10:13.2241498Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-09T05:10:13.2241498Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-09T05:10:13.2241709Z 
2020-04-09T05:10:13.2248835Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-09T05:10:13.2283346Z note: rustc 1.44.0-nightly (a9b4c171d 2020-04-09) running on x86_64-unknown-linux-gnu
2020-04-09T05:10:13.2283631Z 
2020-04-09T05:10:13.2283631Z 
2020-04-09T05:10:13.2284675Z note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-04-09T05:10:13.2285499Z note: some of the compiler flags provided by cargo are hidden
2020-04-09T05:10:13.2285721Z 
2020-04-09T05:10:13.2315308Z error: could not compile `panic_unwind`.
2020-04-09T05:10:13.2315894Z 
---
2020-04-09T05:10:13.5897242Z expected success, got: exit code: 101
2020-04-09T05:10:13.5902462Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-09T05:10:13.5902894Z Build completed unsuccessfully in 0:22:12
2020-04-09T05:10:13.5956035Z == clock drift check ==
2020-04-09T05:10:13.5970499Z   local time: Thu Apr  9 05:10:13 UTC 2020
2020-04-09T05:10:13.7583055Z   network time: Thu, 09 Apr 2020 05:10:13 GMT
2020-04-09T05:10:16.2865578Z 
2020-04-09T05:10:16.2865578Z 
2020-04-09T05:10:16.2940800Z ##[error]Bash exited with code '1'.
2020-04-09T05:10:16.2954745Z ##[section]Finishing: Run build
2020-04-09T05:10:16.3003989Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-09T05:10:16.3009162Z Task         : Get sources
2020-04-09T05:10:16.3009504Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-09T05:10:16.3009819Z Version      : 1.0.0
2020-04-09T05:10:16.3010058Z Author       : Microsoft
2020-04-09T05:10:16.3010058Z Author       : Microsoft
2020-04-09T05:10:16.3010412Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-09T05:10:16.3010819Z ==============================================================================
2020-04-09T05:10:16.6264967Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-09T05:10:16.6311439Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-09T05:10:16.6402598Z Cleaning up task key
2020-04-09T05:10:16.6404593Z Start cleaning up orphan processes.
2020-04-09T05:10:16.6587167Z Terminate orphan process: pid (3489) (python)
2020-04-09T05:10:16.6797790Z ##[section]Finishing: Finalize Job
