plain
2020-01-08T22:27:12.9716594Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-08T22:27:12.9731229Z ##[command]git config gc.auto 0
2020-01-08T22:27:12.9734455Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-08T22:27:12.9781187Z ##[command]git config --get-all http.proxy
2020-01-08T22:27:12.9786975Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68031/merge:refs/remotes/pull/68031/merge
---
2020-01-08T22:53:50.1390775Z    Compiling cc v1.0.49
2020-01-08T22:53:50.1391190Z    Compiling core v0.0.0 (/checkout/src/libcore)
2020-01-08T22:53:55.4406678Z thread 'rustc' panicked at 'assertion failed: `(left == right)`
2020-01-08T22:53:55.4406906Z   left: `0`,
2020-01-08T22:53:55.4408055Z  right: `1`: destination and source slices have different lengths', /checkout/src/libcore/macros/mod.rs:18:13
2020-01-08T22:53:55.4408233Z 
2020-01-08T22:53:55.4408279Z error: internal compiler error: unexpected panic
2020-01-08T22:53:55.4408553Z 
2020-01-08T22:53:55.4424215Z note: the compiler unexpectedly panicked. this is a bug.
2020-01-08T22:53:55.4424215Z note: the compiler unexpectedly panicked. this is a bug.
2020-01-08T22:53:55.4424268Z 
2020-01-08T22:53:55.4425036Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-01-08T22:53:55.4425093Z 
2020-01-08T22:53:55.4425453Z note: rustc 1.42.0-nightly (0ea29ce3e 2020-01-08) running on x86_64-unknown-linux-gnu
2020-01-08T22:53:55.4428839Z 
2020-01-08T22:53:55.4430932Z note: compiler flags: -Z external-macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-01-08T22:53:55.4434880Z note: some of the compiler flags provided by cargo are hidden
2020-01-08T22:53:55.4436013Z 
2020-01-08T22:53:55.4951605Z error: could not compile `core`.
2020-01-08T22:53:55.4970798Z warning: build failed, waiting for other jobs to finish...
---
2020-01-08T22:53:56.4329272Z   local time: Wed Jan  8 22:53:56 UTC 2020
2020-01-08T22:53:56.5279467Z   network time: Wed, 08 Jan 2020 22:53:56 GMT
2020-01-08T22:53:56.5283287Z == end clock drift check ==
2020-01-08T22:53:57.7401012Z 
2020-01-08T22:53:57.7528531Z ##[error]Bash exited with code '1'.
2020-01-08T22:53:57.7572343Z ##[section]Starting: Checkout
2020-01-08T22:53:57.7574427Z ==============================================================================
2020-01-08T22:53:57.7574508Z Task         : Get sources
2020-01-08T22:53:57.7574562Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
