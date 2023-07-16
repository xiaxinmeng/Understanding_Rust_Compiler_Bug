plain
2020-03-24T09:43:43.8179322Z ========================== Starting Command Output ===========================
2020-03-24T09:43:43.8183023Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a185cb33-1ed1-4e0d-9277-abef8a0c4c6b.sh
2020-03-24T09:43:43.8183280Z 
2020-03-24T09:43:43.8187928Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-24T09:43:43.8207884Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-03-24T09:43:43.8211143Z Task         : Get sources
2020-03-24T09:43:43.8211459Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T09:43:43.8211760Z Version      : 1.0.0
2020-03-24T09:43:43.8211967Z Author       : Microsoft
---
2020-03-24T09:43:44.8111688Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-24T09:43:44.8120887Z ##[command]git config gc.auto 0
2020-03-24T09:43:44.8124742Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-24T09:43:44.8129274Z ##[command]git config --get-all http.proxy
2020-03-24T09:43:44.8136486Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68362/merge:refs/remotes/pull/68362/merge
---
2020-03-24T09:50:35.0800988Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-24T09:50:44.4457403Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-24T09:50:50.3223591Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-24T09:51:01.4900510Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-24T09:51:04.4598480Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-24T09:51:05.7422667Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-24T09:51:36.7199821Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-24T09:51:44.5724163Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-24T09:52:30.5936327Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-24T10:11:35.1094411Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-24T10:11:46.8635860Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-24T10:11:54.6815661Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-24T10:12:08.5873262Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-24T10:12:12.1339765Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-24T10:12:13.7847853Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-24T10:12:54.9262087Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-24T10:13:04.4497383Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-24T10:14:04.3774788Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-24T10:27:24.6554021Z error: internal compiler error: unexpected panic
2020-03-24T10:27:24.6556434Z 
2020-03-24T10:27:24.6561007Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-24T10:27:24.6563314Z 
2020-03-24T10:27:24.6567605Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-24T10:27:24.6574047Z note: rustc 1.44.0-nightly (97ebf7940 2020-03-24) running on x86_64-unknown-linux-gnu
2020-03-24T10:27:24.6576480Z 
2020-03-24T10:27:24.6576480Z 
2020-03-24T10:27:24.6580885Z note: compiler flags: -Z macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-03-24T10:27:24.6605522Z note: some of the compiler flags provided by cargo are hidden
2020-03-24T10:27:24.6607979Z 
2020-03-24T10:27:24.6765462Z error: could not compile `rustc_interface`.
2020-03-24T10:27:24.6769059Z 
---
2020-03-24T10:28:10.6898287Z   local time: Tue Mar 24 10:28:10 UTC 2020
2020-03-24T10:28:10.8514824Z   network time: Tue, 24 Mar 2020 10:28:10 GMT
2020-03-24T10:28:10.8520946Z == end clock drift check ==
2020-03-24T10:28:11.4453734Z 
2020-03-24T10:28:11.4502844Z ##[error]Bash exited with code '1'.
2020-03-24T10:28:11.4522075Z ##[section]Finishing: Run build
2020-03-24T10:28:11.4562111Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-03-24T10:28:11.4566091Z Task         : Get sources
2020-03-24T10:28:11.4566547Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T10:28:11.4566771Z Version      : 1.0.0
2020-03-24T10:28:11.4566927Z Author       : Microsoft
2020-03-24T10:28:11.4566927Z Author       : Microsoft
2020-03-24T10:28:11.4567196Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-24T10:28:11.4567481Z ==============================================================================
2020-03-24T10:28:11.7253757Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-24T10:28:11.7305671Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-03-24T10:28:11.7369656Z Cleaning up task key
2020-03-24T10:28:11.7370561Z Start cleaning up orphan processes.
2020-03-24T10:28:11.7535329Z Terminate orphan process: pid (3407) (python)
2020-03-24T10:28:11.7679299Z ##[section]Finishing: Finalize Job
