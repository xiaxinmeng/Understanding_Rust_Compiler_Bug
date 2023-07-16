plain
2020-03-31T20:55:34.0965612Z ========================== Starting Command Output ===========================
2020-03-31T20:55:34.0968837Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bca8aea2-5e98-4acb-8368-2b87cd57517a.sh
2020-03-31T20:55:34.0969119Z 
2020-03-31T20:55:34.0973350Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-31T20:55:34.0989127Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70598/merge to s
2020-03-31T20:55:34.0992146Z Task         : Get sources
2020-03-31T20:55:34.0992454Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T20:55:34.0992737Z Version      : 1.0.0
2020-03-31T20:55:34.0992927Z Author       : Microsoft
---
2020-03-31T20:55:35.0869894Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-31T20:55:35.0878957Z ##[command]git config gc.auto 0
2020-03-31T20:55:35.0883966Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-31T20:55:35.0887534Z ##[command]git config --get-all http.proxy
2020-03-31T20:55:35.0893587Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70598/merge:refs/remotes/pull/70598/merge
---
2020-03-31T21:04:17.8294144Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T21:04:19.1757663Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T21:04:20.6586445Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T21:04:21.0800625Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T21:04:29.8633614Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T21:04:31.4602264Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T21:04:35.5625066Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T21:04:39.4253151Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T21:04:48.6389707Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T21:21:24.1265084Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-31T21:21:24.1265319Z 
2020-03-31T21:21:24.1274719Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-31T21:21:24.1274968Z 
2020-03-31T21:21:24.1275879Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-31T21:21:24.1276664Z note: rustc 1.44.0-nightly (9f14dcdf3 2020-03-31) running on x86_64-unknown-linux-gnu
2020-03-31T21:21:24.1276911Z 
2020-03-31T21:21:24.1276911Z 
2020-03-31T21:21:24.1284039Z note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-03-31T21:21:24.1290642Z note: some of the compiler flags provided by cargo are hidden
2020-03-31T21:21:24.1295170Z 
2020-03-31T21:21:24.1605169Z error: aborting due to previous error
2020-03-31T21:21:24.1610265Z 
---
2020-03-31T21:21:24.9402219Z   local time: Tue Mar 31 21:21:24 UTC 2020
2020-03-31T21:21:25.2360052Z   network time: Tue, 31 Mar 2020 21:21:25 GMT
2020-03-31T21:21:25.2361236Z == end clock drift check ==
2020-03-31T21:21:26.8395833Z 
2020-03-31T21:21:26.8470858Z ##[error]Bash exited with code '1'.
2020-03-31T21:21:26.8486556Z ##[section]Finishing: Run build
2020-03-31T21:21:26.8535457Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70598/merge to s
2020-03-31T21:21:26.8540618Z Task         : Get sources
2020-03-31T21:21:26.8540970Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T21:21:26.8541305Z Version      : 1.0.0
2020-03-31T21:21:26.8541532Z Author       : Microsoft
2020-03-31T21:21:26.8541532Z Author       : Microsoft
2020-03-31T21:21:26.8541887Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-31T21:21:26.8542307Z ==============================================================================
2020-03-31T21:21:27.1719004Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-31T21:21:27.1764145Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70598/merge to s
2020-03-31T21:21:27.1849997Z Cleaning up task key
2020-03-31T21:21:27.1851268Z Start cleaning up orphan processes.
2020-03-31T21:21:27.2133656Z Terminate orphan process: pid (3822) (python)
2020-03-31T21:21:27.2178506Z ##[section]Finishing: Finalize Job
