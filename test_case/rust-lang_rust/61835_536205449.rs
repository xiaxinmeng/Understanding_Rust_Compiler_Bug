plain
2019-09-28T16:08:23.8435314Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-28T16:08:23.8645424Z ##[command]git config gc.auto 0
2019-09-28T16:08:23.8723763Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-28T16:08:23.8789393Z ##[command]git config --get-all http.proxy
2019-09-28T16:08:23.8925930Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61835/merge:refs/remotes/pull/61835/merge
---
2019-09-28T16:40:03.9684447Z    Compiling compiler_builtins v0.1.18
2019-09-28T16:40:05.9098102Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-09-28T16:40:06.8011037Z    Compiling backtrace-sys v0.1.30
2019-09-28T16:40:07.6804452Z    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
2019-09-28T16:40:07.9661412Z thread 'rustc' panicked at 'intrinsic maxnumf32 is not in const eval whitelist', src/librustc_mir/transform/qualify_consts.rs:1286:29
2019-09-28T16:40:07.9672561Z 
2019-09-28T16:40:07.9678128Z error: internal compiler error: unexpected panic
2019-09-28T16:40:07.9681596Z 
2019-09-28T16:40:07.9686370Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-28T16:40:07.9686370Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-28T16:40:07.9690234Z 
2019-09-28T16:40:07.9755642Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-09-28T16:40:07.9756211Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-09-28T16:40:07.9756244Z 
2019-09-28T16:40:07.9756244Z 
2019-09-28T16:40:07.9757292Z note: compiler flags: -Z external-macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type lib
2019-09-28T16:40:07.9757406Z note: some of the compiler flags provided by cargo are hidden
2019-09-28T16:40:07.9757459Z 
2019-09-28T16:40:08.1492974Z error: could not compile `core`.
2019-09-28T16:40:08.1514349Z warning: build failed, waiting for other jobs to finish...
---
2019-09-28T16:40:08.2168314Z == clock drift check ==
2019-09-28T16:40:08.2182957Z   local time: Sat Sep 28 16:40:08 UTC 2019
2019-09-28T16:40:08.3821394Z   network time: Sat, 28 Sep 2019 16:40:08 GMT
2019-09-28T16:40:08.3822115Z == end clock drift check ==
2019-09-28T16:40:09.5528492Z ##[error]Bash exited with code '1'.
2019-09-28T16:40:09.5572850Z ##[section]Starting: Checkout
2019-09-28T16:40:09.5574628Z ==============================================================================
2019-09-28T16:40:09.5574700Z Task         : Get sources
2019-09-28T16:40:09.5574766Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
