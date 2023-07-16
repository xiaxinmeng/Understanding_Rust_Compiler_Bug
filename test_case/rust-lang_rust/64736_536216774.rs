plain
2019-09-28T18:24:26.8468548Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-28T18:24:26.8657367Z ##[command]git config gc.auto 0
2019-09-28T18:24:26.8757995Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-28T18:24:27.8247639Z ##[command]git config --get-all http.proxy
2019-09-28T18:24:27.8253013Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64736/merge:refs/remotes/pull/64736/merge
---
2019-09-28T18:57:10.8209576Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2019-09-28T18:57:10.8243128Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-28T18:57:11.2846123Z    Compiling cc v1.0.35
2019-09-28T18:57:11.2847256Z    Compiling core v0.0.0 (/checkout/src/libcore)
2019-09-28T18:57:17.1244827Z thread 'rustc' panicked at 'assertion failed: self.predecessors_cache.is_some()', src/librustc/mir/mod.rs:239:9
2019-09-28T18:57:17.1245441Z 
2019-09-28T18:57:17.1245491Z error: internal compiler error: unexpected panic
2019-09-28T18:57:17.1245546Z 
2019-09-28T18:57:17.1251403Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-28T18:57:17.1251403Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-28T18:57:17.1251484Z 
2019-09-28T18:57:17.1252670Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-09-28T18:57:17.1253034Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-09-28T18:57:17.1253068Z 
2019-09-28T18:57:17.1253068Z 
2019-09-28T18:57:17.1253544Z note: compiler flags: -Z external-macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type lib
2019-09-28T18:57:17.1253665Z note: some of the compiler flags provided by cargo are hidden
2019-09-28T18:57:17.1253953Z 
2019-09-28T18:57:17.2790891Z error: could not compile `core`.
2019-09-28T18:57:17.2818668Z warning: build failed, waiting for other jobs to finish...
---
2019-09-28T18:57:17.7994863Z == clock drift check ==
2019-09-28T18:57:17.8012614Z   local time: Sat Sep 28 18:57:17 UTC 2019
2019-09-28T18:57:17.8901086Z   network time: Sat, 28 Sep 2019 18:57:17 GMT
2019-09-28T18:57:17.8901205Z == end clock drift check ==
2019-09-28T18:57:18.5931596Z ##[error]Bash exited with code '1'.
2019-09-28T18:57:18.5997665Z ##[section]Starting: Checkout
2019-09-28T18:57:18.5999879Z ==============================================================================
2019-09-28T18:57:18.5999960Z Task         : Get sources
2019-09-28T18:57:18.6000013Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
