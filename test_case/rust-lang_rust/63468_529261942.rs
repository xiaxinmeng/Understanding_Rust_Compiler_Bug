plain
2019-09-08T23:57:18.8624457Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-08T23:57:18.8848691Z ##[command]git config gc.auto 0
2019-09-08T23:57:19.8505949Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-08T23:57:19.8517416Z ##[command]git config --get-all http.proxy
2019-09-08T23:57:19.8524507Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63468/merge:refs/remotes/pull/63468/merge
---
2019-09-09T00:28:52.1614189Z error: internal compiler error: unexpected panic
2019-09-09T00:28:52.1620152Z 
2019-09-09T00:28:52.1622434Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-09T00:28:52.1622886Z 
2019-09-09T00:28:52.1625571Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-09-09T00:28:52.1628191Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-09-09T00:28:52.1630205Z 
2019-09-09T00:28:52.1630205Z 
2019-09-09T00:28:52.1633510Z note: compiler flags: -Z binary-dep-depinfo -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C prefer-dynamic -C debug-assertions=n -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
2019-09-09T00:28:52.1636394Z note: some of the compiler flags provided by cargo are hidden
2019-09-09T00:28:52.1636581Z 
2019-09-09T00:28:52.1708041Z error: Could not compile `core`.
2019-09-09T00:28:52.1731707Z warning: build failed, waiting for other jobs to finish...
---
2019-09-09T00:28:56.7602657Z == clock drift check ==
2019-09-09T00:28:56.7622668Z   local time: Mon Sep  9 00:28:56 UTC 2019
2019-09-09T00:28:56.9136583Z   network time: Mon, 09 Sep 2019 00:28:56 GMT
2019-09-09T00:28:56.9141718Z == end clock drift check ==
2019-09-09T00:28:57.6260983Z ##[error]Bash exited with code '1'.
2019-09-09T00:28:57.6297573Z ##[section]Starting: Checkout
2019-09-09T00:28:57.6299986Z ==============================================================================
2019-09-09T00:28:57.6300055Z Task         : Get sources
2019-09-09T00:28:57.6300100Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
