plain
2019-09-28T12:39:18.8641392Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-28T12:39:18.8823069Z ##[command]git config gc.auto 0
2019-09-28T12:39:18.8905951Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-28T12:39:18.8974644Z ##[command]git config --get-all http.proxy
2019-09-28T12:39:18.9133288Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61835/merge:refs/remotes/pull/61835/merge
---
2019-09-28T13:10:45.5492440Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-28T13:10:46.0418728Z    Compiling cc v1.0.35
2019-09-28T13:10:46.0420375Z    Compiling core v0.0.0 (/checkout/src/libcore)
2019-09-28T13:10:52.5576835Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-09-28T13:10:53.1226335Z thread 'rustc' panicked at 'assertion failed: !self.tcx.is_const_fn(def_id)', src/librustc_mir/transform/qualify_consts.rs:1286:29
2019-09-28T13:10:53.1232351Z 
2019-09-28T13:10:53.1238611Z error: internal compiler error: unexpected panic
2019-09-28T13:10:53.1239009Z 
2019-09-28T13:10:53.1245165Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-28T13:10:53.1245165Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-28T13:10:53.1245216Z 
2019-09-28T13:10:53.1253245Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-09-28T13:10:53.1259400Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-09-28T13:10:53.1259492Z 
2019-09-28T13:10:53.1259492Z 
2019-09-28T13:10:53.1296605Z note: compiler flags: -Z external-macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type lib
2019-09-28T13:10:53.1296761Z note: some of the compiler flags provided by cargo are hidden
2019-09-28T13:10:53.1296796Z 
2019-09-28T13:10:53.7999280Z    Compiling libc v0.2.62
2019-09-28T13:10:53.8032907Z error: could not compile `core`.
---
2019-09-28T13:10:54.4935113Z == clock drift check ==
2019-09-28T13:10:54.4952827Z   local time: Sat Sep 28 13:10:54 UTC 2019
2019-09-28T13:10:54.6454763Z   network time: Sat, 28 Sep 2019 13:10:54 GMT
2019-09-28T13:10:54.6455909Z == end clock drift check ==
2019-09-28T13:10:55.4646243Z ##[error]Bash exited with code '1'.
2019-09-28T13:10:55.4699932Z ##[section]Starting: Checkout
2019-09-28T13:10:55.4702724Z ==============================================================================
2019-09-28T13:10:55.4702790Z Task         : Get sources
2019-09-28T13:10:55.4702845Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
