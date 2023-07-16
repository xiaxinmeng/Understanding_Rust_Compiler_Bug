plain
2020-02-08T02:02:35.9853689Z ========================== Starting Command Output ===========================
2020-02-08T02:02:35.9856407Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6a439e14-1a0e-40d5-91fb-bd6cdeeabc06.sh
2020-02-08T02:02:36.0011257Z 
2020-02-08T02:02:36.0128474Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-08T02:02:36.0135561Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68941/merge to s
2020-02-08T02:02:36.0137114Z Task         : Get sources
2020-02-08T02:02:36.0137155Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T02:02:36.0137181Z Version      : 1.0.0
2020-02-08T02:02:36.0137206Z Author       : Microsoft
---
2020-02-08T02:02:36.9923596Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-08T02:02:37.0028763Z ##[command]git config gc.auto 0
2020-02-08T02:02:37.0108614Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-08T02:02:37.0168915Z ##[command]git config --get-all http.proxy
2020-02-08T02:02:37.0319311Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68941/merge:refs/remotes/pull/68941/merge
---
2020-02-08T02:55:13.6340942Z error: internal compiler error: unexpected panic
2020-02-08T02:55:13.6340972Z 
2020-02-08T02:55:13.6347696Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-08T02:55:13.6347737Z 
2020-02-08T02:55:13.6348301Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-08T02:55:13.6354963Z note: rustc 1.43.0-nightly (bf37edcda 2020-02-08) running on x86_64-unknown-linux-gnu
2020-02-08T02:55:13.6354998Z 
2020-02-08T02:55:13.6354998Z 
2020-02-08T02:55:13.6360142Z note: compiler flags: -Z macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-02-08T02:55:13.6360264Z note: some of the compiler flags provided by cargo are hidden
2020-02-08T02:55:13.6360305Z 
2020-02-08T02:55:13.7522777Z error: could not compile `rustc_codegen_llvm`.
2020-02-08T02:55:13.7523096Z warning: build failed, waiting for other jobs to finish...
---
2020-02-08T02:55:24.6502592Z   local time: Sat Feb  8 02:55:24 UTC 2020
2020-02-08T02:55:24.8255543Z   network time: Sat, 08 Feb 2020 02:55:24 GMT
2020-02-08T02:55:24.8255916Z == end clock drift check ==
2020-02-08T02:55:25.5479568Z 
2020-02-08T02:55:25.5572021Z ##[error]Bash exited with code '1'.
2020-02-08T02:55:25.5587196Z ##[section]Finishing: Run build
2020-02-08T02:55:25.5614226Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68941/merge to s
2020-02-08T02:55:25.5616338Z Task         : Get sources
2020-02-08T02:55:25.5616396Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T02:55:25.5616597Z Version      : 1.0.0
2020-02-08T02:55:25.5616631Z Author       : Microsoft
2020-02-08T02:55:25.5616631Z Author       : Microsoft
2020-02-08T02:55:25.5616687Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-08T02:55:25.5616728Z ==============================================================================
2020-02-08T02:55:25.9883732Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-08T02:55:25.9922383Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68941/merge to s
2020-02-08T02:55:26.0031312Z Cleaning up task key
2020-02-08T02:55:26.0032018Z Start cleaning up orphan processes.
2020-02-08T02:55:26.0139599Z Terminate orphan process: pid (3576) (python)
2020-02-08T02:55:26.0362375Z ##[section]Finishing: Finalize Job
