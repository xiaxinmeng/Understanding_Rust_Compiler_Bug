plain
2020-04-08T04:02:36.6069551Z ========================== Starting Command Output ===========================
2020-04-08T04:02:36.6071847Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fac84656-d38b-4160-89de-a07205d2cb34.sh
2020-04-08T04:02:36.6072311Z 
2020-04-08T04:02:36.6075313Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-08T04:02:36.6095897Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-08T04:02:36.6099150Z Task         : Get sources
2020-04-08T04:02:36.6099412Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T04:02:36.6099659Z Version      : 1.0.0
2020-04-08T04:02:36.6099827Z Author       : Microsoft
---
2020-04-08T04:02:37.6230754Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-08T04:02:37.6237136Z ##[command]git config gc.auto 0
2020-04-08T04:02:37.6240835Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-08T04:02:37.6243943Z ##[command]git config --get-all http.proxy
2020-04-08T04:02:37.6249716Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70551/merge:refs/remotes/pull/70551/merge
---
2020-04-08T04:06:07.1839773Z Looks like docker image is the same as before, not uploading
2020-04-08T04:06:12.9192296Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-08T04:06:12.9521889Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-08T04:06:12.9559655Z == clock drift check ==
2020-04-08T04:06:12.9564467Z   local time: Wed Apr  8 04:06:12 UTC 2020
2020-04-08T04:06:13.0128319Z   network time: Wed, 08 Apr 2020 04:06:13 GMT
2020-04-08T04:06:13.0151500Z Starting sccache server...
2020-04-08T04:06:13.1058092Z configure: processing command line
2020-04-08T04:06:13.1058911Z configure: 
2020-04-08T04:06:13.1061745Z configure: rust.dist-src        := False
---
2020-04-08T04:11:51.4837139Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-08T04:11:53.1392795Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-08T04:11:54.9086768Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-08T04:11:56.2568020Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-08T04:12:06.0686035Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-08T04:12:08.6599510Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-08T04:12:13.6551808Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-08T04:12:18.2150403Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-08T04:12:28.7116590Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-08T04:32:26.2277029Z    Compiling alloc v0.0.0 (/checkout/src/liballoc)
2020-04-08T04:32:29.6545722Z    Compiling rustc-demangle v0.1.16
2020-04-08T04:32:30.5438841Z error: internal compiler error: TyKind::Error constructed but no error reported. src/librustc_typeck/check/pat.rs:1435:10
2020-04-08T04:32:30.5445771Z 
2020-04-08T04:32:30.5446880Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:360:17
2020-04-08T04:32:30.5447852Z 
2020-04-08T04:32:30.5448190Z error: internal compiler error: unexpected panic
2020-04-08T04:32:30.5448478Z 
2020-04-08T04:32:30.5448811Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-08T04:32:30.5448811Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-08T04:32:30.5449118Z 
2020-04-08T04:32:30.5449955Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-08T04:32:30.5450999Z note: rustc 1.44.0-nightly (7422132d0 2020-04-07) running on x86_64-unknown-linux-gnu
2020-04-08T04:32:30.5451371Z 
2020-04-08T04:32:30.5451371Z 
2020-04-08T04:32:30.5452658Z note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-04-08T04:32:30.5454644Z note: some of the compiler flags provided by cargo are hidden
2020-04-08T04:32:30.5454955Z 
2020-04-08T04:32:30.5455919Z error: could not compile `alloc`.
2020-04-08T04:32:30.5456508Z 
---
2020-04-08T04:32:31.4231471Z expected success, got: exit code: 101
2020-04-08T04:32:31.4242937Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-08T04:32:31.4243357Z Build completed unsuccessfully in 0:24:34
2020-04-08T04:32:31.4299406Z == clock drift check ==
2020-04-08T04:32:31.4325344Z   local time: Wed Apr  8 04:32:31 UTC 2020
2020-04-08T04:32:31.7064938Z   network time: Wed, 08 Apr 2020 04:32:31 GMT
2020-04-08T04:32:34.6138166Z 
2020-04-08T04:32:34.6138166Z 
2020-04-08T04:32:34.6212050Z ##[error]Bash exited with code '1'.
2020-04-08T04:32:34.6226773Z ##[section]Finishing: Run build
2020-04-08T04:32:34.6281263Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-08T04:32:34.6286391Z Task         : Get sources
2020-04-08T04:32:34.6286738Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T04:32:34.6287058Z Version      : 1.0.0
2020-04-08T04:32:34.6287296Z Author       : Microsoft
2020-04-08T04:32:34.6287296Z Author       : Microsoft
2020-04-08T04:32:34.6287658Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-08T04:32:34.6288071Z ==============================================================================
2020-04-08T04:32:34.9813446Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-08T04:32:34.9863106Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-08T04:32:34.9951951Z Cleaning up task key
2020-04-08T04:32:34.9953205Z Start cleaning up orphan processes.
2020-04-08T04:32:35.0163951Z Terminate orphan process: pid (3408) (python)
2020-04-08T04:32:35.0382035Z ##[section]Finishing: Finalize Job
