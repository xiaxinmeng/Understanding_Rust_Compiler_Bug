plain
2020-04-09T03:53:46.0406265Z ========================== Starting Command Output ===========================
2020-04-09T03:53:46.0408392Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/41c92b7f-e2a0-4664-bdb1-74cc83a825d0.sh
2020-04-09T03:53:46.0408606Z 
2020-04-09T03:53:46.0412249Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-09T03:53:46.0432352Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-09T03:53:46.0435278Z Task         : Get sources
2020-04-09T03:53:46.0435708Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-09T03:53:46.0436021Z Version      : 1.0.0
2020-04-09T03:53:46.0436190Z Author       : Microsoft
---
2020-04-09T03:53:47.0358810Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-09T03:53:47.0364008Z ##[command]git config gc.auto 0
2020-04-09T03:53:47.0367358Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-09T03:53:47.0370285Z ##[command]git config --get-all http.proxy
2020-04-09T03:53:47.0375787Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70551/merge:refs/remotes/pull/70551/merge
---
2020-04-09T03:55:59.2624995Z Looks like docker image is the same as before, not uploading
2020-04-09T03:56:06.9249737Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-09T03:56:06.9660493Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-09T03:56:06.9695093Z == clock drift check ==
2020-04-09T03:56:06.9703773Z   local time: Thu Apr  9 03:56:06 UTC 2020
2020-04-09T03:56:07.2610511Z   network time: Thu, 09 Apr 2020 03:56:07 GMT
2020-04-09T03:56:07.2640867Z Starting sccache server...
2020-04-09T03:56:07.3547276Z configure: processing command line
2020-04-09T03:56:07.3548173Z configure: 
2020-04-09T03:56:07.3549592Z configure: rust.dist-src        := False
---
2020-04-09T04:01:35.6430061Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-09T04:01:37.2383330Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-09T04:01:38.8804045Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-09T04:01:41.1350134Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-09T04:01:49.4074007Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-09T04:01:53.6224932Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-09T04:01:58.4380804Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-09T04:02:02.8850533Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-09T04:02:11.3759897Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-09T04:22:01.1509780Z    Compiling alloc v0.0.0 (/checkout/src/liballoc)
2020-04-09T04:22:04.5355883Z    Compiling rustc-demangle v0.1.16
2020-04-09T04:22:05.1707882Z error: internal compiler error: TyKind::Error constructed but no error reported. src/librustc_typeck/check/pat.rs:1435:10
2020-04-09T04:22:05.1712994Z 
2020-04-09T04:22:05.1783467Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:360:17
2020-04-09T04:22:05.1793641Z 
2020-04-09T04:22:05.1799208Z error: internal compiler error: unexpected panic
2020-04-09T04:22:05.1803212Z 
2020-04-09T04:22:05.1809725Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-09T04:22:05.1809725Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-09T04:22:05.1813561Z 
2020-04-09T04:22:05.1820712Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-09T04:22:05.1831223Z 
2020-04-09T04:22:05.1832184Z note: rustc 1.44.0-nightly (dab1e7721 2020-04-09) running on x86_64-unknown-linux-gnu
2020-04-09T04:22:05.1832470Z 
2020-04-09T04:22:05.1833502Z note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-04-09T04:22:05.1834327Z note: some of the compiler flags provided by cargo are hidden
2020-04-09T04:22:05.1834538Z 
2020-04-09T04:22:05.1835001Z error: could not compile `alloc`.
2020-04-09T04:22:05.1835230Z 
---
2020-04-09T04:22:06.3185941Z expected success, got: exit code: 101
2020-04-09T04:22:06.3260320Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-09T04:22:06.3260812Z Build completed unsuccessfully in 0:24:21
2020-04-09T04:22:06.3265626Z == clock drift check ==
2020-04-09T04:22:06.3286935Z   local time: Thu Apr  9 04:22:06 UTC 2020
2020-04-09T04:22:06.4906609Z   network time: Thu, 09 Apr 2020 04:22:06 GMT
2020-04-09T04:22:07.9445241Z 
2020-04-09T04:22:07.9445241Z 
2020-04-09T04:22:07.9539256Z ##[error]Bash exited with code '1'.
2020-04-09T04:22:07.9554351Z ##[section]Finishing: Run build
2020-04-09T04:22:07.9606091Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-09T04:22:07.9611475Z Task         : Get sources
2020-04-09T04:22:07.9611818Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-09T04:22:07.9612154Z Version      : 1.0.0
2020-04-09T04:22:07.9612396Z Author       : Microsoft
2020-04-09T04:22:07.9612396Z Author       : Microsoft
2020-04-09T04:22:07.9612752Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-09T04:22:07.9613180Z ==============================================================================
2020-04-09T04:22:08.3074445Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-09T04:22:08.3128248Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-09T04:22:08.3222644Z Cleaning up task key
2020-04-09T04:22:08.3224079Z Start cleaning up orphan processes.
2020-04-09T04:22:08.3400771Z Terminate orphan process: pid (3579) (python)
2020-04-09T04:22:08.3570654Z ##[section]Finishing: Finalize Job
