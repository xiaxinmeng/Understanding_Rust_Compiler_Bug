plain
2020-03-23T17:40:33.1751710Z ========================== Starting Command Output ===========================
2020-03-23T17:40:33.1754355Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d621a36c-6c80-4461-9140-a216c82fdc8a.sh
2020-03-23T17:40:33.1754615Z 
2020-03-23T17:40:33.1757923Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-23T17:40:33.1778226Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70319/merge to s
2020-03-23T17:40:33.1781832Z Task         : Get sources
2020-03-23T17:40:33.1782187Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T17:40:33.1782474Z Version      : 1.0.0
2020-03-23T17:40:33.1782668Z Author       : Microsoft
---
2020-03-23T17:40:34.3692275Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-23T17:40:34.3699938Z ##[command]git config gc.auto 0
2020-03-23T17:40:34.3704756Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-23T17:40:34.3715507Z ##[command]git config --get-all http.proxy
2020-03-23T17:40:34.3725619Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70319/merge:refs/remotes/pull/70319/merge
---
2020-03-23T18:08:56.2044002Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2020-03-23T18:08:57.0849704Z    Compiling backtrace-sys v0.1.34
2020-03-23T18:08:57.9951335Z    Compiling hashbrown v0.6.2
2020-03-23T18:09:07.4118451Z    Compiling rustc-std-workspace-core v1.99.0 (/checkout/src/tools/rustc-std-workspace-core)
2020-03-23T18:09:08.7194763Z error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:603: collection encountered polymorphic constant
2020-03-23T18:09:08.7195917Z   --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.25/src/int/mod.rs:50:5
2020-03-23T18:09:08.7197827Z 50 |     const ONE: Self;
2020-03-23T18:09:08.7198528Z    |     ^^^^^^^^^^^^^^^^
2020-03-23T18:09:08.7231945Z 
2020-03-23T18:09:08.7233617Z thread 'rustc' panicked at 'Box<Any>', <::std::macros::panic macros>:2:4
2020-03-23T18:09:08.7233617Z thread 'rustc' panicked at 'Box<Any>', <::std::macros::panic macros>:2:4
2020-03-23T18:09:08.7234073Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-23T18:09:08.7234349Z 
2020-03-23T18:09:08.7234579Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-23T18:09:08.7234805Z 
2020-03-23T18:09:08.7235566Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-23T18:09:08.7235880Z 
2020-03-23T18:09:08.7236410Z note: rustc 1.44.0-nightly (fca83592d 2020-03-23) running on x86_64-unknown-linux-gnu
2020-03-23T18:09:08.7236673Z 
2020-03-23T18:09:08.7237770Z note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C panic=abort -C debug-assertions=no --crate-type lib
2020-03-23T18:09:08.7238670Z note: some of the compiler flags provided by cargo are hidden
2020-03-23T18:09:08.7238885Z 
2020-03-23T18:09:08.7239362Z error: aborting due to previous error
2020-03-23T18:09:08.7239614Z 
---
2020-03-23T18:09:14.3195206Z   local time: Mon Mar 23 18:09:14 UTC 2020
2020-03-23T18:09:14.3990374Z   network time: Mon, 23 Mar 2020 18:09:14 GMT
2020-03-23T18:09:14.3990903Z == end clock drift check ==
2020-03-23T18:09:16.7215001Z 
2020-03-23T18:09:16.7305542Z ##[error]Bash exited with code '1'.
2020-03-23T18:09:16.7319911Z ##[section]Finishing: Run build
2020-03-23T18:09:16.7373487Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70319/merge to s
2020-03-23T18:09:16.7378860Z Task         : Get sources
2020-03-23T18:09:16.7379236Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T18:09:16.7379566Z Version      : 1.0.0
2020-03-23T18:09:16.7379793Z Author       : Microsoft
2020-03-23T18:09:16.7379793Z Author       : Microsoft
2020-03-23T18:09:16.7380183Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-23T18:09:16.7380608Z ==============================================================================
2020-03-23T18:09:17.0993563Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-23T18:09:17.1041913Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70319/merge to s
2020-03-23T18:09:17.1134155Z Cleaning up task key
2020-03-23T18:09:17.1135473Z Start cleaning up orphan processes.
2020-03-23T18:09:17.1332189Z Terminate orphan process: pid (4494) (python)
2020-03-23T18:09:17.1503178Z ##[section]Finishing: Finalize Job
