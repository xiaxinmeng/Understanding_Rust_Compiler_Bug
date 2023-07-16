plain
2020-03-13T01:53:49.6257929Z ========================== Starting Command Output ===========================
2020-03-13T01:53:49.6263174Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/dddcbbca-04f4-4d9a-a7c4-e55b5ec9766e.sh
2020-03-13T01:53:49.6263690Z 
2020-03-13T01:53:49.6268744Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-13T01:53:49.6289683Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69740/merge to s
2020-03-13T01:53:49.6293175Z Task         : Get sources
2020-03-13T01:53:49.6293502Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T01:53:49.6293835Z Version      : 1.0.0
2020-03-13T01:53:49.6294054Z Author       : Microsoft
---
2020-03-13T01:53:50.6219212Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-13T01:53:50.6229670Z ##[command]git config gc.auto 0
2020-03-13T01:53:50.6233709Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-13T01:53:50.6237396Z ##[command]git config --get-all http.proxy
2020-03-13T01:53:50.6244324Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69740/merge:refs/remotes/pull/69740/merge
---
2020-03-13T02:23:39.4855438Z    Compiling compiler_builtins v0.1.25
2020-03-13T02:23:41.2134300Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2020-03-13T02:23:42.0513978Z    Compiling backtrace-sys v0.1.32
2020-03-13T02:23:42.9625508Z    Compiling hashbrown v0.6.2
2020-03-13T02:23:49.0373284Z error: internal compiler error: src/librustc/ty/context.rs:1530: article_and_description called on def_id DefId(0:1 ~ core[50b3]::{{misc}}[0])
2020-03-13T02:23:49.0375322Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:880:9
2020-03-13T02:23:49.0375947Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-13T02:23:49.0376354Z 
2020-03-13T02:23:49.0376708Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-13T02:23:49.0376708Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-13T02:23:49.0377029Z 
2020-03-13T02:23:49.0377968Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-13T02:23:49.0379158Z note: rustc 1.43.0-nightly (0b21918ba 2020-03-12) running on x86_64-unknown-linux-gnu
2020-03-13T02:23:49.0379585Z 
2020-03-13T02:23:49.0379585Z 
2020-03-13T02:23:49.0380819Z note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-03-13T02:23:49.0382014Z note: some of the compiler flags provided by cargo are hidden
2020-03-13T02:23:49.0382340Z 
2020-03-13T02:23:49.3814343Z error: aborting due to previous error
2020-03-13T02:23:49.3819654Z 
---
2020-03-13T02:23:49.4198531Z   local time: Fri Mar 13 02:23:49 UTC 2020
2020-03-13T02:23:49.5226611Z   network time: Fri, 13 Mar 2020 02:23:49 GMT
2020-03-13T02:23:49.5230349Z == end clock drift check ==
2020-03-13T02:23:51.2018674Z 
2020-03-13T02:23:51.2182434Z ##[error]Bash exited with code '1'.
2020-03-13T02:23:51.2196119Z ##[section]Finishing: Run build
2020-03-13T02:23:51.2250882Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69740/merge to s
2020-03-13T02:23:51.2256441Z Task         : Get sources
2020-03-13T02:23:51.2256851Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T02:23:51.2257415Z Version      : 1.0.0
2020-03-13T02:23:51.2257711Z Author       : Microsoft
2020-03-13T02:23:51.2257711Z Author       : Microsoft
2020-03-13T02:23:51.2258130Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-13T02:23:51.2258620Z ==============================================================================
2020-03-13T02:23:51.5899043Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-13T02:23:51.5953140Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69740/merge to s
2020-03-13T02:23:51.6056738Z Cleaning up task key
2020-03-13T02:23:51.6058222Z Start cleaning up orphan processes.
2020-03-13T02:23:51.6257576Z Terminate orphan process: pid (4186) (python)
2020-03-13T02:23:51.6554354Z ##[section]Finishing: Finalize Job
