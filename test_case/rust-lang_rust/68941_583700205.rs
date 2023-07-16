plain
2020-02-08T03:08:02.4350286Z ========================== Starting Command Output ===========================
2020-02-08T03:08:02.4352858Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1fdbb236-42cb-4acc-aeb4-b04fbf10c174.sh
2020-02-08T03:08:02.4353026Z 
2020-02-08T03:08:02.4356156Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-08T03:08:02.4363296Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68941/merge to s
2020-02-08T03:08:02.4364945Z Task         : Get sources
2020-02-08T03:08:02.4364996Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T03:08:02.4365032Z Version      : 1.0.0
2020-02-08T03:08:02.4365065Z Author       : Microsoft
---
2020-02-08T03:08:06.4080315Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-08T03:08:06.4472162Z ##[command]git config gc.auto 0
2020-02-08T03:08:06.4523154Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-08T03:08:06.4578816Z ##[command]git config --get-all http.proxy
2020-02-08T03:08:06.4727918Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68941/merge:refs/remotes/pull/68941/merge
---
2020-02-08T04:04:44.3197094Z error: internal compiler error: unexpected panic
2020-02-08T04:04:44.3200463Z 
2020-02-08T04:04:44.3207582Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-08T04:04:44.3210974Z 
2020-02-08T04:04:44.3217718Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-08T04:04:44.3227186Z note: rustc 1.43.0-nightly (f886cd3b5 2020-02-08) running on x86_64-unknown-linux-gnu
2020-02-08T04:04:44.3233549Z 
2020-02-08T04:04:44.3233549Z 
2020-02-08T04:04:44.3241015Z note: compiler flags: -Z macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-02-08T04:04:44.3253412Z note: some of the compiler flags provided by cargo are hidden
2020-02-08T04:04:44.3256301Z 
2020-02-08T04:04:44.4700760Z error: could not compile `rustc_codegen_llvm`.
2020-02-08T04:04:44.4775479Z warning: build failed, waiting for other jobs to finish...
---
2020-02-08T04:04:52.0338364Z   local time: Sat Feb  8 04:04:52 UTC 2020
2020-02-08T04:04:52.3306387Z   network time: Sat, 08 Feb 2020 04:04:52 GMT
2020-02-08T04:04:52.3306502Z == end clock drift check ==
2020-02-08T04:04:52.9276559Z 
2020-02-08T04:04:52.9377075Z ##[error]Bash exited with code '1'.
2020-02-08T04:04:52.9393208Z ##[section]Finishing: Run build
2020-02-08T04:04:52.9418692Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68941/merge to s
2020-02-08T04:04:52.9422324Z Task         : Get sources
2020-02-08T04:04:52.9422392Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T04:04:52.9422441Z Version      : 1.0.0
2020-02-08T04:04:52.9422484Z Author       : Microsoft
2020-02-08T04:04:52.9422484Z Author       : Microsoft
2020-02-08T04:04:52.9422530Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-08T04:04:52.9422614Z ==============================================================================
2020-02-08T04:04:53.4120346Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-08T04:04:53.4149745Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68941/merge to s
2020-02-08T04:04:53.4282560Z Cleaning up task key
2020-02-08T04:04:53.4283353Z Start cleaning up orphan processes.
2020-02-08T04:04:53.4402614Z Terminate orphan process: pid (4176) (python)
2020-02-08T04:04:53.4662865Z ##[section]Finishing: Finalize Job
