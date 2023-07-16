plain
2019-11-29T09:13:18.8215285Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-29T09:13:18.8232869Z ##[command]git config gc.auto 0
2019-11-29T09:13:18.8234750Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-29T09:13:18.8239320Z ##[command]git config --get-all http.proxy
2019-11-29T09:13:18.8243225Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66863/merge:refs/remotes/pull/66863/merge
---
2019-11-29T09:44:52.8448649Z    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
2019-11-29T09:44:52.8449188Z    Compiling once_cell v1.1.0
2019-11-29T09:44:53.0714253Z    Compiling semver v0.9.0
2019-11-29T09:44:54.9943779Z    Compiling crossbeam-utils v0.6.5
2019-11-29T09:44:55.1849415Z thread 'rustc' panicked at 'already borrowed: BorrowMutError', src/libcore/macros/mod.rs:23:13
2019-11-29T09:44:55.1850320Z 
2019-11-29T09:44:55.1850369Z error: internal compiler error: unexpected panic
2019-11-29T09:44:55.1850421Z 
2019-11-29T09:44:55.1850469Z note: the compiler unexpectedly panicked. this is a bug.
2019-11-29T09:44:55.1850469Z note: the compiler unexpectedly panicked. this is a bug.
2019-11-29T09:44:55.1850500Z 
2019-11-29T09:44:55.1851081Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-11-29T09:44:55.1851399Z note: rustc 1.41.0-dev running on x86_64-unknown-linux-gnu
2019-11-29T09:44:55.1851435Z 
2019-11-29T09:44:55.1851435Z 
2019-11-29T09:44:55.1851881Z note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type lib
2019-11-29T09:44:55.1851984Z note: some of the compiler flags provided by cargo are hidden
2019-11-29T09:44:55.1852033Z 
2019-11-29T09:44:55.1956428Z error: could not compile `crossbeam-utils`.
2019-11-29T09:44:55.1978604Z warning: build failed, waiting for other jobs to finish...
---
2019-11-29T09:44:58.8754999Z   local time: Fri Nov 29 09:44:58 UTC 2019
2019-11-29T09:44:59.4102659Z   network time: Fri, 29 Nov 2019 09:44:59 GMT
2019-11-29T09:44:59.4105204Z == end clock drift check ==
2019-11-29T09:45:00.6407728Z 
2019-11-29T09:45:00.6508028Z ##[error]Bash exited with code '1'.
2019-11-29T09:45:00.6545031Z ##[section]Starting: Checkout
2019-11-29T09:45:00.6546792Z ==============================================================================
2019-11-29T09:45:00.6546852Z Task         : Get sources
2019-11-29T09:45:00.6546901Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
