plain
2020-03-24T08:08:49.2380313Z ========================== Starting Command Output ===========================
2020-03-24T08:08:49.2384031Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cbe32648-75d3-4320-8bbd-08a4a588f9df.sh
2020-03-24T08:08:49.2384453Z 
2020-03-24T08:08:49.2389636Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-24T08:08:49.2409626Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-03-24T08:08:49.2412917Z Task         : Get sources
2020-03-24T08:08:49.2413225Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T08:08:49.2413528Z Version      : 1.0.0
2020-03-24T08:08:49.2413734Z Author       : Microsoft
---
2020-03-24T08:08:50.2408147Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-24T08:08:50.2415188Z ##[command]git config gc.auto 0
2020-03-24T08:08:50.2420026Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-24T08:08:50.2424647Z ##[command]git config --get-all http.proxy
2020-03-24T08:08:50.2432408Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68362/merge:refs/remotes/pull/68362/merge
---
2020-03-24T08:17:11.9373793Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-24T08:17:21.6528767Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-24T08:17:28.6694376Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-24T08:17:39.4340725Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-24T08:17:43.3403745Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-24T08:17:44.6482502Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-24T08:18:16.9452129Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-24T08:18:25.2215051Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-24T08:19:12.9318341Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-24T08:39:38.1643230Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-24T08:39:38.9083522Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-24T08:39:50.5507785Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-24T08:40:02.2827688Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-24T08:40:07.8741589Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-24T08:40:09.3952656Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-24T08:40:52.7557690Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-24T08:41:02.5229519Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-24T08:42:09.2214323Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-24T09:00:25.4951378Z error: internal compiler error: unexpected panic
2020-03-24T09:00:25.4951621Z 
2020-03-24T09:00:25.4951856Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-24T09:00:25.4952060Z 
2020-03-24T09:00:25.4953471Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-24T09:00:25.4954291Z note: rustc 1.44.0-nightly (9cdf4357a 2020-03-24) running on x86_64-unknown-linux-gnu
2020-03-24T09:00:25.4954548Z 
2020-03-24T09:00:25.4954548Z 
2020-03-24T09:00:25.4955557Z note: compiler flags: -Z macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-03-24T09:00:25.4956382Z note: some of the compiler flags provided by cargo are hidden
2020-03-24T09:00:25.4956608Z 
2020-03-24T09:00:25.5152382Z error: could not compile `rustc_interface`.
2020-03-24T09:00:25.5152711Z 
---
2020-03-24T09:00:34.4929288Z   local time: Tue Mar 24 09:00:34 UTC 2020
2020-03-24T09:00:34.7851639Z   network time: Tue, 24 Mar 2020 09:00:34 GMT
2020-03-24T09:00:34.7856748Z == end clock drift check ==
2020-03-24T09:00:35.9691668Z 
2020-03-24T09:00:35.9769340Z ##[error]Bash exited with code '1'.
2020-03-24T09:00:35.9792899Z ##[section]Finishing: Run build
2020-03-24T09:00:35.9842433Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-03-24T09:00:35.9847675Z Task         : Get sources
2020-03-24T09:00:35.9848026Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T09:00:35.9848351Z Version      : 1.0.0
2020-03-24T09:00:35.9848596Z Author       : Microsoft
2020-03-24T09:00:35.9848596Z Author       : Microsoft
2020-03-24T09:00:35.9848960Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-24T09:00:35.9849374Z ==============================================================================
2020-03-24T09:00:36.3260009Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-24T09:00:36.3310652Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-03-24T09:00:36.3425266Z Cleaning up task key
2020-03-24T09:00:36.3426713Z Start cleaning up orphan processes.
2020-03-24T09:00:36.3609866Z Terminate orphan process: pid (3726) (python)
2020-03-24T09:00:36.3776714Z ##[section]Finishing: Finalize Job
