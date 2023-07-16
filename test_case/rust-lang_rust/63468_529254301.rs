plain
2019-09-08T22:56:14.1267366Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-08T22:56:14.1435467Z ##[command]git config gc.auto 0
2019-09-08T22:56:14.1531905Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-08T22:56:14.1595698Z ##[command]git config --get-all http.proxy
2019-09-08T22:56:14.1752903Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63468/merge:refs/remotes/pull/63468/merge
---
2019-09-08T23:31:16.4876738Z    Compiling polonius-engine v0.10.0
2019-09-08T23:31:17.1766491Z    Compiling env_logger v0.6.2
2019-09-08T23:31:17.5998203Z    Compiling rustc_apfloat v0.0.0 (/checkout/src/librustc_apfloat)
2019-09-08T23:31:20.1117865Z    Compiling measureme v0.3.0
2019-09-08T23:31:23.3635362Z thread 'rustc' panicked at 'invocation data is reset for an invocation', src/librustc_resolve/build_reduced_graph.rs:1060:9
2019-09-08T23:31:23.3811289Z 
2019-09-08T23:31:23.3820960Z error: internal compiler error: unexpected panic
2019-09-08T23:31:23.3825023Z 
2019-09-08T23:31:23.3830252Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-08T23:31:23.3830252Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-08T23:31:23.3833351Z 
2019-09-08T23:31:23.3841255Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-09-08T23:31:23.3851966Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-09-08T23:31:23.3855196Z 
2019-09-08T23:31:23.3855196Z 
2019-09-08T23:31:23.3895731Z note: compiler flags: -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 --crate-type lib
2019-09-08T23:31:23.3896381Z note: some of the compiler flags provided by cargo are hidden
2019-09-08T23:31:23.3896511Z 
2019-09-08T23:31:23.3920903Z error: Could not compile `syn`.
2019-09-08T23:31:23.3945248Z warning: build failed, waiting for other jobs to finish...
---
2019-09-08T23:31:23.8158377Z == clock drift check ==
2019-09-08T23:31:23.8181022Z   local time: Sun Sep  8 23:31:23 UTC 2019
2019-09-08T23:31:23.9074177Z   network time: Sun, 08 Sep 2019 23:31:23 GMT
2019-09-08T23:31:23.9079295Z == end clock drift check ==
2019-09-08T23:31:25.6074707Z ##[error]Bash exited with code '1'.
2019-09-08T23:31:25.6113893Z ##[section]Starting: Checkout
2019-09-08T23:31:25.6115882Z ==============================================================================
2019-09-08T23:31:25.6115962Z Task         : Get sources
2019-09-08T23:31:25.6116012Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
