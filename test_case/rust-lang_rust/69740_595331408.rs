plain
2020-03-05T16:21:43.3890430Z ========================== Starting Command Output ===========================
2020-03-05T16:21:43.3894409Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c684cc31-65b4-4012-be54-22d90c67cd47.sh
2020-03-05T16:21:43.3894826Z 
2020-03-05T16:21:43.3899566Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-05T16:21:43.3949678Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69740/merge to s
2020-03-05T16:21:43.3954426Z Task         : Get sources
2020-03-05T16:21:43.3954886Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-05T16:21:43.3955345Z Version      : 1.0.0
2020-03-05T16:21:43.3955640Z Author       : Microsoft
---
2020-03-05T16:21:44.3880906Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-05T16:21:44.3888126Z ##[command]git config gc.auto 0
2020-03-05T16:21:44.3893364Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-05T16:21:44.3897965Z ##[command]git config --get-all http.proxy
2020-03-05T16:21:44.3904043Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69740/merge:refs/remotes/pull/69740/merge
---
2020-03-05T16:49:25.4511021Z    Compiling compiler_builtins v0.1.25
2020-03-05T16:49:27.0511484Z    Compiling backtrace-sys v0.1.32
2020-03-05T16:49:27.8686978Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2020-03-05T16:49:28.6757884Z    Compiling hashbrown v0.6.2
2020-03-05T16:49:33.5793839Z error: internal compiler error: src/librustc/ty/context.rs:1530: article_and_description called on def_id DefId(0:1 ~ core[a88a]::{{misc}}[0])
2020-03-05T16:49:33.5802956Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:875:9
2020-03-05T16:49:33.5807374Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-05T16:49:33.5809942Z 
2020-03-05T16:49:33.5814479Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-05T16:49:33.5814479Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-05T16:49:33.5817350Z 
2020-03-05T16:49:33.5822819Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-05T16:49:33.5830472Z note: rustc 1.43.0-nightly (9f478f6c5 2020-03-05) running on x86_64-unknown-linux-gnu
2020-03-05T16:49:33.5834745Z 
2020-03-05T16:49:33.5834745Z 
2020-03-05T16:49:33.5836431Z note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-03-05T16:49:33.5838472Z note: some of the compiler flags provided by cargo are hidden
2020-03-05T16:49:33.5838822Z 
2020-03-05T16:49:33.9390856Z error: aborting due to previous error
2020-03-05T16:49:33.9395484Z 
---
2020-03-05T16:49:34.5514682Z   local time: Thu Mar  5 16:49:34 UTC 2020
2020-03-05T16:49:34.6175003Z   network time: Thu, 05 Mar 2020 16:49:34 GMT
2020-03-05T16:49:34.6175715Z == end clock drift check ==
2020-03-05T16:49:36.2696616Z 
2020-03-05T16:49:36.2769179Z ##[error]Bash exited with code '1'.
2020-03-05T16:49:36.2783707Z ##[section]Finishing: Run build
2020-03-05T16:49:36.2836763Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69740/merge to s
2020-03-05T16:49:36.2841909Z Task         : Get sources
2020-03-05T16:49:36.2842276Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-05T16:49:36.2842603Z Version      : 1.0.0
2020-03-05T16:49:36.2842833Z Author       : Microsoft
2020-03-05T16:49:36.2842833Z Author       : Microsoft
2020-03-05T16:49:36.2843206Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-05T16:49:36.2843620Z ==============================================================================
2020-03-05T16:49:36.6150382Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-05T16:49:36.6193632Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69740/merge to s
2020-03-05T16:49:36.6282843Z Cleaning up task key
2020-03-05T16:49:36.6284232Z Start cleaning up orphan processes.
2020-03-05T16:49:36.6456475Z Terminate orphan process: pid (3323) (python)
2020-03-05T16:49:36.6623964Z ##[section]Finishing: Finalize Job
