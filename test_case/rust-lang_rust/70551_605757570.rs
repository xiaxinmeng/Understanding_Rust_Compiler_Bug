plain
2020-03-30T02:38:08.3316227Z ========================== Starting Command Output ===========================
2020-03-30T02:38:08.3319767Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ccb608a7-eed0-485d-8632-a5ddb8bc4f11.sh
2020-03-30T02:38:08.3320193Z 
2020-03-30T02:38:08.3324479Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-30T02:38:08.3343149Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-03-30T02:38:08.3346383Z Task         : Get sources
2020-03-30T02:38:08.3346696Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-30T02:38:08.3347020Z Version      : 1.0.0
2020-03-30T02:38:08.3347225Z Author       : Microsoft
---
2020-03-30T02:38:09.3202632Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-30T02:38:09.3208951Z ##[command]git config gc.auto 0
2020-03-30T02:38:09.3213408Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-30T02:38:09.3217800Z ##[command]git config --get-all http.proxy
2020-03-30T02:38:09.3225566Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70551/merge:refs/remotes/pull/70551/merge
---
2020-03-30T02:46:12.1898639Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-30T02:46:13.5898814Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-30T02:46:15.1289785Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-30T02:46:15.3927470Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-30T02:46:24.5447153Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-30T02:46:26.0022171Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-30T02:46:30.1319682Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-30T02:46:34.0168489Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-30T02:46:43.3338683Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-30T03:03:27.4874029Z    Compiling backtrace-sys v0.1.35
2020-03-30T03:03:28.2499218Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2020-03-30T03:03:29.0145958Z    Compiling hashbrown v0.6.2
2020-03-30T03:03:37.6577625Z    Compiling rustc-std-workspace-core v1.99.0 (/checkout/src/tools/rustc-std-workspace-core)
2020-03-30T03:03:39.6306348Z error: internal compiler error: TyKind::Error constructed but no error reported. src/librustc_typeck/check/wfcheck.rs:710:17
2020-03-30T03:03:39.6307359Z error: internal compiler error: TyKind::Error constructed but no error reported. src/librustc_typeck/lib.rs:384:9
2020-03-30T03:03:39.6307875Z 
2020-03-30T03:03:39.6307875Z 
2020-03-30T03:03:39.6308372Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:360:17
2020-03-30T03:03:39.6308976Z 
2020-03-30T03:03:39.6309152Z error: internal compiler error: unexpected panic
2020-03-30T03:03:39.6309406Z 
2020-03-30T03:03:39.6309599Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-30T03:03:39.6309599Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-30T03:03:39.6309761Z 
2020-03-30T03:03:39.6310333Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-30T03:03:39.6310972Z note: rustc 1.44.0-nightly (9dced68d0 2020-03-30) running on x86_64-unknown-linux-gnu
2020-03-30T03:03:39.6311179Z 
2020-03-30T03:03:39.6311179Z 
2020-03-30T03:03:39.6312017Z note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C panic=abort -C debug-assertions=no --crate-type lib
2020-03-30T03:03:39.6312720Z note: some of the compiler flags provided by cargo are hidden
2020-03-30T03:03:39.6312890Z 
2020-03-30T03:03:39.6350352Z error: could not compile `compiler_builtins`.
2020-03-30T03:03:39.6350601Z 
2020-03-30T03:03:39.6350601Z 
2020-03-30T03:03:39.6350939Z To learn more, run the command again with --verbose.
2020-03-30T03:03:39.6351440Z warning: build failed, waiting for other jobs to finish...
2020-03-30T03:03:43.6836632Z error: internal compiler error: TyKind::Error constructed but no error reported. src/librustc_typeck/check/wfcheck.rs:710:17
2020-03-30T03:03:43.6839880Z error: internal compiler error: TyKind::Error constructed but no error reported. src/librustc_typeck/variance/constraints.rs:318:82
2020-03-30T03:03:43.6840453Z 
2020-03-30T03:03:43.6840453Z 
2020-03-30T03:03:43.6847310Z error: internal compiler error: ty::Const Error constructed but no error reported. src/librustc_typeck/check/wfcheck.rs:715:17
2020-03-30T03:03:43.6848066Z 
2020-03-30T03:03:43.6849201Z error: internal compiler error: TyKind::Error constructed but no error reported. src/librustc_typeck/check/wfcheck.rs:715:17
2020-03-30T03:03:43.6902056Z error: internal compiler error: TyKind::Error constructed but no error reported. src/librustc_trait_selection/traits/select.rs:3077:63
2020-03-30T03:03:43.6902844Z 
2020-03-30T03:03:43.6903874Z error: internal compiler error: TyKind::Error constructed but no error reported. src/librustc_typeck/lib.rs:384:9
2020-03-30T03:03:43.6904520Z 
2020-03-30T03:03:43.6904520Z 
2020-03-30T03:03:43.6966188Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:360:17
2020-03-30T03:03:43.6974480Z 
2020-03-30T03:03:43.6981379Z error: internal compiler error: unexpected panic
2020-03-30T03:03:43.6981904Z 
2020-03-30T03:03:43.6982326Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-30T03:03:43.6982326Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-30T03:03:43.6982709Z 
2020-03-30T03:03:43.6983777Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-30T03:03:43.6984854Z note: rustc 1.44.0-nightly (9dced68d0 2020-03-30) running on x86_64-unknown-linux-gnu
2020-03-30T03:03:43.6985236Z 
2020-03-30T03:03:43.6985236Z 
2020-03-30T03:03:43.6986374Z note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-03-30T03:03:43.6987606Z note: some of the compiler flags provided by cargo are hidden
2020-03-30T03:03:43.6987943Z 
2020-03-30T03:03:43.7261539Z error: could not compile `core`.
2020-03-30T03:03:43.7262256Z 
---
2020-03-30T03:03:43.7367517Z   local time: Mon Mar 30 03:03:43 UTC 2020
2020-03-30T03:03:43.9044844Z   network time: Mon, 30 Mar 2020 03:03:43 GMT
2020-03-30T03:03:43.9049218Z == end clock drift check ==
2020-03-30T03:03:46.2554224Z 
2020-03-30T03:03:46.2625994Z ##[error]Bash exited with code '1'.
2020-03-30T03:03:46.2639392Z ##[section]Finishing: Run build
2020-03-30T03:03:46.2687823Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-03-30T03:03:46.2692914Z Task         : Get sources
2020-03-30T03:03:46.2693267Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-30T03:03:46.2693589Z Version      : 1.0.0
2020-03-30T03:03:46.2693833Z Author       : Microsoft
2020-03-30T03:03:46.2693833Z Author       : Microsoft
2020-03-30T03:03:46.2694190Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-30T03:03:46.2694606Z ==============================================================================
2020-03-30T03:03:46.5972029Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-30T03:03:46.6016372Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-03-30T03:03:46.6105013Z Cleaning up task key
2020-03-30T03:03:46.6106238Z Start cleaning up orphan processes.
2020-03-30T03:03:46.6424410Z Terminate orphan process: pid (4355) (python)
2020-03-30T03:03:46.6462999Z ##[section]Finishing: Finalize Job
