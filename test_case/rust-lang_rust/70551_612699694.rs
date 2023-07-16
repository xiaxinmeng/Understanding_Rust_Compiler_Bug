plain
2020-04-13T00:03:14.7250698Z ========================== Starting Command Output ===========================
2020-04-13T00:03:14.7270156Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/91cf56c8-d264-46ad-bcc9-7738b3ccec2d.sh
2020-04-13T00:03:15.0145063Z 
2020-04-13T00:03:15.0328525Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-13T00:03:15.0358733Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-13T00:03:15.0369028Z Task         : Get sources
2020-04-13T00:03:15.0369370Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T00:03:15.0369812Z Version      : 1.0.0
2020-04-13T00:03:15.0370065Z Author       : Microsoft
---
2020-04-13T00:03:17.5236881Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-13T00:03:17.5413472Z ##[command]git config gc.auto 0
2020-04-13T00:03:17.5443441Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-13T00:03:17.5468754Z ##[command]git config --get-all http.proxy
2020-04-13T00:03:17.5568561Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70551/merge:refs/remotes/pull/70551/merge
---
2020-04-13T00:06:50.6250400Z Looks like docker image is the same as before, not uploading
2020-04-13T00:06:59.1263894Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-13T00:06:59.1582700Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-13T00:06:59.1615923Z == clock drift check ==
2020-04-13T00:06:59.1627517Z   local time: Mon Apr 13 00:06:59 UTC 2020
2020-04-13T00:06:59.3016932Z   network time: Mon, 13 Apr 2020 00:06:59 GMT
2020-04-13T00:06:59.3036076Z Starting sccache server...
2020-04-13T00:06:59.3917032Z configure: processing command line
2020-04-13T00:06:59.3917544Z configure: 
2020-04-13T00:06:59.3918446Z configure: rust.dist-src        := False
---
2020-04-13T00:12:46.6946760Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-13T00:12:48.3466582Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-13T00:12:50.0858530Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-13T00:12:51.7400865Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-13T00:13:01.4742443Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-13T00:13:04.4369409Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-13T00:13:09.3911113Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-13T00:13:13.9990911Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-13T00:13:24.3295654Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-13T00:34:05.7432108Z    Compiling unicode-width v0.1.6
2020-04-13T00:34:05.8466988Z    Compiling getopts v0.2.21
2020-04-13T00:34:20.1986451Z error: internal compiler error: TyKind::Error constructed but no error reported. src/librustc_typeck/check/expr.rs:573:57
2020-04-13T00:34:20.1987699Z 
2020-04-13T00:34:20.1988575Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:363:17
2020-04-13T00:34:20.1991363Z 
2020-04-13T00:34:20.1991758Z error: internal compiler error: unexpected panic
2020-04-13T00:34:20.1991977Z 
2020-04-13T00:34:20.1992201Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-13T00:34:20.1992201Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-13T00:34:20.1992403Z 
2020-04-13T00:34:20.1993220Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-13T00:34:20.1994036Z note: rustc 1.44.0-nightly (26cf5d3fc 2020-04-13) running on x86_64-unknown-linux-gnu
2020-04-13T00:34:20.1994297Z 
2020-04-13T00:34:20.1994297Z 
2020-04-13T00:34:20.1995326Z note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-04-13T00:34:20.1996364Z note: some of the compiler flags provided by cargo are hidden
2020-04-13T00:34:20.1996594Z 
2020-04-13T00:34:20.2120994Z error: could not compile `proc_macro`.
2020-04-13T00:34:20.2121294Z 
2020-04-13T00:34:20.2121294Z 
2020-04-13T00:34:20.2121791Z To learn more, run the command again with --verbose.
2020-04-13T00:34:20.2148879Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-13T00:34:20.2151955Z expected success, got: exit code: 101
2020-04-13T00:34:20.2153958Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-13T00:34:20.2154571Z Build completed unsuccessfully in 0:25:34
2020-04-13T00:34:20.2208113Z == clock drift check ==
2020-04-13T00:34:20.2222024Z   local time: Mon Apr 13 00:34:20 UTC 2020
2020-04-13T00:34:20.2513076Z   network time: Mon, 13 Apr 2020 00:34:20 GMT
2020-04-13T00:34:20.8937312Z 
2020-04-13T00:34:20.8937312Z 
2020-04-13T00:34:20.9058641Z ##[error]Bash exited with code '1'.
2020-04-13T00:34:20.9074250Z ##[section]Finishing: Run build
2020-04-13T00:34:20.9130512Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-13T00:34:20.9135910Z Task         : Get sources
2020-04-13T00:34:20.9136328Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T00:34:20.9136739Z Version      : 1.0.0
2020-04-13T00:34:20.9137011Z Author       : Microsoft
2020-04-13T00:34:20.9137011Z Author       : Microsoft
2020-04-13T00:34:20.9137437Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-13T00:34:20.9137979Z ==============================================================================
2020-04-13T00:34:21.2862904Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-13T00:34:21.2912965Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-13T00:34:21.3009067Z Cleaning up task key
2020-04-13T00:34:21.3010492Z Start cleaning up orphan processes.
2020-04-13T00:34:21.3211573Z Terminate orphan process: pid (4289) (python)
2020-04-13T00:34:21.3566955Z ##[section]Finishing: Finalize Job
