plain
2019-10-08T00:17:02.8186437Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-08T00:17:02.8408647Z ##[command]git config gc.auto 0
2019-10-08T00:17:02.8469118Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-08T00:17:02.8516589Z ##[command]git config --get-all http.proxy
2019-10-08T00:17:02.8659794Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65195/merge:refs/remotes/pull/65195/merge
---
2019-10-08T00:48:36.1045781Z    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
2019-10-08T00:48:36.6162532Z    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
2019-10-08T00:48:37.1313275Z    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
2019-10-08T00:48:37.6555244Z    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
2019-10-08T00:48:43.6852367Z error: internal compiler error: src/librustc/session/mod.rs:812: trying to get session directory from `IncrCompSession`: NotInitialized
2019-10-08T00:48:43.6862727Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:917:9
2019-10-08T00:48:43.6868224Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-08T00:48:43.6871326Z 
2019-10-08T00:48:43.6876412Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-08T00:48:43.6876412Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-08T00:48:43.6881338Z 
2019-10-08T00:48:43.6886631Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-08T00:48:43.6895043Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-08T00:48:43.6898234Z 
2019-10-08T00:48:43.6898234Z 
2019-10-08T00:48:43.6903558Z note: compiler flags: -Z external-macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type lib
2019-10-08T00:48:43.6904352Z note: some of the compiler flags provided by cargo are hidden
2019-10-08T00:48:43.6904482Z 
2019-10-08T00:48:43.9262446Z error: aborting due to previous error
2019-10-08T00:48:43.9268064Z 
---
2019-10-08T00:48:45.0498152Z == clock drift check ==
2019-10-08T00:48:45.0498221Z   local time: Tue Oct  8 00:48:44 UTC 2019
2019-10-08T00:48:45.0498271Z   network time: Tue, 08 Oct 2019 00:48:44 GMT
2019-10-08T00:48:45.0498318Z == end clock drift check ==
2019-10-08T00:48:46.2078540Z ##[error]Bash exited with code '1'.
2019-10-08T00:48:46.2127476Z ##[section]Starting: Checkout
2019-10-08T00:48:46.2129482Z ==============================================================================
2019-10-08T00:48:46.2129537Z Task         : Get sources
2019-10-08T00:48:46.2129600Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
