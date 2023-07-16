plain
2020-01-08T09:19:36.7144124Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-08T09:19:36.7229492Z ##[command]git config gc.auto 0
2020-01-08T09:19:36.7286044Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-08T09:19:36.7337708Z ##[command]git config --get-all http.proxy
2020-01-08T09:19:36.7478781Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67953/merge:refs/remotes/pull/67953/merge
---
2020-01-08T09:44:55.6117821Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2020-01-08T09:44:55.6136473Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-08T09:44:55.9805317Z    Compiling cc v1.0.49
2020-01-08T09:44:55.9807278Z    Compiling core v0.0.0 (/checkout/src/libcore)
2020-01-08T09:45:00.8239503Z thread 'rustc' panicked at 'not implemented', src/librustc/traits/mod.rs:779:5
2020-01-08T09:45:00.8241044Z 
2020-01-08T09:45:00.8241602Z error: internal compiler error: unexpected panic
2020-01-08T09:45:00.8241692Z 
2020-01-08T09:45:00.8241807Z note: the compiler unexpectedly panicked. this is a bug.
2020-01-08T09:45:00.8241807Z note: the compiler unexpectedly panicked. this is a bug.
2020-01-08T09:45:00.8241888Z 
2020-01-08T09:45:00.8242569Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-01-08T09:45:00.8242612Z 
2020-01-08T09:45:00.8242951Z note: rustc 1.42.0-nightly (80066bcb4 2020-01-08) running on x86_64-unknown-linux-gnu
2020-01-08T09:45:00.8242988Z 
2020-01-08T09:45:00.8243470Z note: compiler flags: -Z external-macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-01-08T09:45:00.8243590Z note: some of the compiler flags provided by cargo are hidden
2020-01-08T09:45:00.8243622Z 
2020-01-08T09:45:00.8678416Z error: could not compile `core`.
2020-01-08T09:45:00.8679194Z warning: build failed, waiting for other jobs to finish...
---
2020-01-08T09:45:01.8525770Z   local time: Wed Jan  8 09:45:01 UTC 2020
2020-01-08T09:45:02.0117512Z   network time: Wed, 08 Jan 2020 09:45:02 GMT
2020-01-08T09:45:02.0121866Z == end clock drift check ==
2020-01-08T09:45:03.2515208Z 
2020-01-08T09:45:03.2611755Z ##[error]Bash exited with code '1'.
2020-01-08T09:45:03.2643896Z ##[section]Starting: Checkout
2020-01-08T09:45:03.2645545Z ==============================================================================
2020-01-08T09:45:03.2645610Z Task         : Get sources
2020-01-08T09:45:03.2645648Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
