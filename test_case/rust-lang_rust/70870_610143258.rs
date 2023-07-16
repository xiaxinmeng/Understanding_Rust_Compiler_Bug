plain
2020-04-07T02:23:23.4801347Z ========================== Starting Command Output ===========================
2020-04-07T02:23:23.4803598Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/47b4258e-6f91-4ddc-a699-1b09317e5ba5.sh
2020-04-07T02:23:23.4803989Z 
2020-04-07T02:23:23.4808430Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-07T02:23:23.4832252Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70870/merge to s
2020-04-07T02:23:23.4836706Z Task         : Get sources
2020-04-07T02:23:23.4837001Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T02:23:23.4837420Z Version      : 1.0.0
2020-04-07T02:23:23.4837614Z Author       : Microsoft
---
2020-04-07T02:23:24.4746545Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-07T02:23:24.4751120Z ##[command]git config gc.auto 0
2020-04-07T02:23:24.4754562Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-07T02:23:24.4756987Z ##[command]git config --get-all http.proxy
2020-04-07T02:23:24.4763050Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70870/merge:refs/remotes/pull/70870/merge
---
2020-04-07T02:25:16.5946470Z Looks like docker image is the same as before, not uploading
2020-04-07T02:25:17.2993886Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-07T02:25:17.3280825Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-07T02:25:17.3307345Z == clock drift check ==
2020-04-07T02:25:17.3316227Z   local time: Tue Apr  7 02:25:17 UTC 2020
2020-04-07T02:25:17.3943601Z   network time: Tue, 07 Apr 2020 02:25:17 GMT
2020-04-07T02:25:17.3965540Z Starting sccache server...
2020-04-07T02:25:17.4758042Z configure: processing command line
2020-04-07T02:25:17.4758350Z configure: 
2020-04-07T02:25:17.4759209Z configure: rust.dist-src        := False
---
2020-04-07T02:30:39.2317142Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T02:30:40.7822911Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T02:30:42.4723797Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T02:30:43.8816886Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T02:30:52.6889320Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T02:30:55.5469204Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T02:31:00.0882431Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T02:31:04.3660815Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T02:31:13.3774896Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T02:49:11.5825698Z    Compiling compiler_builtins v0.1.25
2020-04-07T02:49:13.1025129Z    Compiling backtrace-sys v0.1.35
2020-04-07T02:49:13.8530623Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2020-04-07T02:49:14.6168805Z    Compiling hashbrown v0.6.2
2020-04-07T02:49:20.3392016Z error: internal compiler error: src/librustc_privacy/lib.rs:226: unexpected type: FreshTy(0)
2020-04-07T02:49:20.3397157Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:880:9
2020-04-07T02:49:20.3397984Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-07T02:49:20.3398515Z 
2020-04-07T02:49:20.3398844Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-07T02:49:20.3398844Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-07T02:49:20.3399157Z 
2020-04-07T02:49:20.3400105Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-07T02:49:20.3401087Z note: rustc 1.44.0-nightly (9c5619d3f 2020-04-06) running on x86_64-unknown-linux-gnu
2020-04-07T02:49:20.3401453Z 
2020-04-07T02:49:20.3401453Z 
2020-04-07T02:49:20.3402369Z note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-04-07T02:49:20.3405932Z note: some of the compiler flags provided by cargo are hidden
2020-04-07T02:49:20.3406254Z 
2020-04-07T02:49:20.4344215Z error: aborting due to previous error
2020-04-07T02:49:20.4344446Z 
---
2020-04-07T02:49:20.4609399Z expected success, got: exit code: 101
2020-04-07T02:49:20.4614234Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-07T02:49:20.4614767Z Build completed unsuccessfully in 0:22:20
2020-04-07T02:49:20.4658836Z == clock drift check ==
2020-04-07T02:49:20.4674134Z   local time: Tue Apr  7 02:49:20 UTC 2020
2020-04-07T02:49:20.7576733Z   network time: Tue, 07 Apr 2020 02:49:20 GMT
2020-04-07T02:49:22.4187358Z 
2020-04-07T02:49:22.4187358Z 
2020-04-07T02:49:22.4257993Z ##[error]Bash exited with code '1'.
2020-04-07T02:49:22.4270209Z ##[section]Finishing: Run build
2020-04-07T02:49:22.4310534Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70870/merge to s
2020-04-07T02:49:22.4314580Z Task         : Get sources
2020-04-07T02:49:22.4314852Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T02:49:22.4315099Z Version      : 1.0.0
2020-04-07T02:49:22.4315261Z Author       : Microsoft
2020-04-07T02:49:22.4315261Z Author       : Microsoft
2020-04-07T02:49:22.4315597Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-07T02:49:22.4315901Z ==============================================================================
2020-04-07T02:49:22.7094504Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-07T02:49:22.7133045Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70870/merge to s
2020-04-07T02:49:22.7202588Z Cleaning up task key
2020-04-07T02:49:22.7203539Z Start cleaning up orphan processes.
2020-04-07T02:49:22.7359128Z Terminate orphan process: pid (3364) (python)
2020-04-07T02:49:22.7490437Z ##[section]Finishing: Finalize Job
