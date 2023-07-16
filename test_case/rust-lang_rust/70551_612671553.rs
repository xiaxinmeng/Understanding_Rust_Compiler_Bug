plain
2020-04-12T20:00:24.4278155Z ========================== Starting Command Output ===========================
2020-04-12T20:00:24.4281005Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4edd7ea8-e0a2-4f85-b561-517a44cb1fd1.sh
2020-04-12T20:00:24.4281243Z 
2020-04-12T20:00:24.4284622Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-12T20:00:24.4303112Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-12T20:00:24.4306106Z Task         : Get sources
2020-04-12T20:00:24.4306354Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-12T20:00:24.4306643Z Version      : 1.0.0
2020-04-12T20:00:24.4306804Z Author       : Microsoft
---
2020-04-12T20:00:25.2263531Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-12T20:00:25.2268515Z ##[command]git config gc.auto 0
2020-04-12T20:00:25.2419633Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-12T20:00:25.2423071Z ##[command]git config --get-all http.proxy
2020-04-12T20:00:25.2428210Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70551/merge:refs/remotes/pull/70551/merge
---
2020-04-12T20:02:37.2001146Z Looks like docker image is the same as before, not uploading
2020-04-12T20:02:44.8215282Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-12T20:02:44.8476100Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-12T20:02:44.8507790Z == clock drift check ==
2020-04-12T20:02:44.8514299Z   local time: Sun Apr 12 20:02:44 UTC 2020
2020-04-12T20:02:44.8786463Z   network time: Sun, 12 Apr 2020 20:02:44 GMT
2020-04-12T20:02:44.8805794Z Starting sccache server...
2020-04-12T20:02:44.9560738Z configure: processing command line
2020-04-12T20:02:44.9561522Z configure: 
2020-04-12T20:02:44.9562575Z configure: rust.dist-src        := False
---
2020-04-12T20:08:06.5031161Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-12T20:08:08.0163357Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-12T20:08:09.6037047Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-12T20:08:11.3978734Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-12T20:08:19.6195002Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-12T20:08:22.7116304Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-12T20:08:27.8382947Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-12T20:08:31.3033516Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-12T20:08:40.2045845Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-12T20:27:18.6459739Z    Compiling rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2020-04-12T20:27:18.8015345Z    Compiling backtrace v0.3.46
2020-04-12T20:27:19.2536467Z error: internal compiler error: TyKind::Error constructed but no error reported. src/librustc_typeck/check/closure.rs:180:60
2020-04-12T20:27:19.2540551Z 
2020-04-12T20:27:19.2549442Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:363:17
2020-04-12T20:27:19.2560341Z 
2020-04-12T20:27:19.2566635Z error: internal compiler error: unexpected panic
2020-04-12T20:27:19.2571104Z 
2020-04-12T20:27:19.2577476Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-12T20:27:19.2577476Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-12T20:27:19.2604397Z 
2020-04-12T20:27:19.2605253Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-12T20:27:19.2605936Z note: rustc 1.44.0-nightly (5e139bf3b 2020-04-12) running on x86_64-unknown-linux-gnu
2020-04-12T20:27:19.2606141Z 
2020-04-12T20:27:19.2606141Z 
2020-04-12T20:27:19.2606948Z note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-04-12T20:27:19.2607630Z note: some of the compiler flags provided by cargo are hidden
2020-04-12T20:27:19.2607799Z 
2020-04-12T20:27:19.2627597Z error: could not compile `backtrace`.
2020-04-12T20:27:19.2627807Z 
---
2020-04-12T20:27:20.1389456Z expected success, got: exit code: 101
2020-04-12T20:27:20.1390085Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-12T20:27:20.1390496Z Build completed unsuccessfully in 0:22:48
2020-04-12T20:27:20.1390754Z == clock drift check ==
2020-04-12T20:27:20.1391007Z   local time: Sun Apr 12 20:27:19 UTC 2020
2020-04-12T20:27:20.1391340Z   network time: Sun, 12 Apr 2020 20:27:19 GMT
2020-04-12T20:27:22.1980742Z 
2020-04-12T20:27:22.1980742Z 
2020-04-12T20:27:22.2024205Z ##[error]Bash exited with code '1'.
2020-04-12T20:27:22.2038129Z ##[section]Finishing: Run build
2020-04-12T20:27:22.2368795Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-12T20:27:22.2373478Z Task         : Get sources
2020-04-12T20:27:22.2373749Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-12T20:27:22.2374015Z Version      : 1.0.0
2020-04-12T20:27:22.2374189Z Author       : Microsoft
2020-04-12T20:27:22.2374189Z Author       : Microsoft
2020-04-12T20:27:22.2374472Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-12T20:27:22.2374809Z ==============================================================================
2020-04-12T20:27:22.5627956Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-12T20:27:22.5675942Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-12T20:27:22.5753238Z Cleaning up task key
2020-04-12T20:27:22.5754270Z Start cleaning up orphan processes.
2020-04-12T20:27:22.5950845Z Terminate orphan process: pid (5908) (python)
2020-04-12T20:27:22.7855710Z ##[section]Finishing: Finalize Job
