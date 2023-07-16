plain
2020-01-09T08:41:43.3485678Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-09T08:41:43.3539936Z ##[command]git config gc.auto 0
2020-01-09T08:41:43.3542921Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-09T08:41:43.3549803Z ##[command]git config --get-all http.proxy
2020-01-09T08:41:43.3552808Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68045/merge:refs/remotes/pull/68045/merge
---
2020-01-09T09:09:06.7426537Z error: internal compiler error: unexpected panic
2020-01-09T09:09:06.7426825Z 
2020-01-09T09:09:06.7427638Z note: the compiler unexpectedly panicked. this is a bug.
2020-01-09T09:09:06.7427884Z 
2020-01-09T09:09:06.7428729Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-01-09T09:09:06.7429038Z 
2020-01-09T09:09:06.7429601Z note: rustc 1.42.0-nightly (abec0828d 2020-01-09) running on x86_64-unknown-linux-gnu
2020-01-09T09:09:06.7429908Z 
2020-01-09T09:09:06.7430631Z note: compiler flags: -Z external-macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-01-09T09:09:06.7431184Z note: some of the compiler flags provided by cargo are hidden
2020-01-09T09:09:06.7431406Z 
2020-01-09T09:09:06.7934646Z error: could not compile `core`.
2020-01-09T09:09:06.7953441Z warning: build failed, waiting for other jobs to finish...
---
2020-01-09T09:09:07.8559282Z   local time: Thu Jan  9 09:09:07 UTC 2020
2020-01-09T09:09:08.4135542Z   network time: Thu, 09 Jan 2020 09:09:08 GMT
2020-01-09T09:09:08.4136752Z == end clock drift check ==
2020-01-09T09:09:09.7345982Z 
2020-01-09T09:09:09.7464417Z ##[error]Bash exited with code '1'.
2020-01-09T09:09:09.7504740Z ##[section]Starting: Checkout
2020-01-09T09:09:09.7506794Z ==============================================================================
2020-01-09T09:09:09.7506873Z Task         : Get sources
2020-01-09T09:09:09.7506926Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
