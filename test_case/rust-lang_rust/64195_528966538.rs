plain
2019-09-06T16:27:51.4029599Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-06T16:27:51.4254145Z ##[command]git config gc.auto 0
2019-09-06T16:27:51.4314817Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-06T16:27:51.4367243Z ##[command]git config --get-all http.proxy
2019-09-06T16:27:51.4516248Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64195/merge:refs/remotes/pull/64195/merge
---
2019-09-06T18:16:02.8618112Z    Compiling mdbook v0.3.1
2019-09-06T18:16:33.9059089Z    Compiling mdbook-linkcheck v0.3.0
2019-09-06T18:16:47.1011897Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2019-09-06T18:16:51.1314943Z     Finished release [optimized] target(s) in 8m 02s
2019-09-06T18:16:51.1573292Z thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error(Msg("Unable to create missing chapters"), State { next_error: Some(Error(Io(Os { code: 30, kind: Other, message: "Read-only file system" }), State { next_error: None, backtrace: InternalBacktrace { backtrace: None } })), backtrace: InternalBacktrace { backtrace: None } })', src/libcore/result.rs:1084:5
2019-09-06T18:16:51.1574704Z 
2019-09-06T18:16:51.1574827Z 
2019-09-06T18:16:51.1575594Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "linkcheck" "/checkout/src/doc/rustc-guide"
2019-09-06T18:16:51.1578018Z expected success, got: exit code: 101
---
2019-09-06T18:34:19.5294421Z This PR updated 'src/doc/rustc-guide', verifying if status is 'test-pass'...
2019-09-06T18:34:19.5304002Z 
2019-09-06T18:34:19.5305015Z ⚠️ We detected that this PR updated 'rustc-guide', but its tests failed.
2019-09-06T18:34:19.5305052Z 
2019-09-06T18:34:19.5305271Z If you do intend to update 'rustc-guide', please check the error messages above and
2019-09-06T18:34:19.5305713Z commit another update.
2019-09-06T18:34:19.5305789Z 
2019-09-06T18:34:19.5307074Z If you do NOT intend to update 'rustc-guide', please ensure you did not accidentally
2019-09-06T18:34:19.5307706Z change the submodule at 'src/doc/rustc-guide'. You may ask your reviewer for the
2019-09-06T18:34:19.5307801Z proper steps.
2019-09-06T18:34:19.5324592Z   local time: Fri Sep  6 18:34:19 UTC 2019
2019-09-06T18:34:19.6822048Z   network time: Fri, 06 Sep 2019 18:34:19 GMT
2019-09-06T18:34:19.6822126Z == end clock drift check ==
2019-09-06T18:34:19.6822126Z == end clock drift check ==
2019-09-06T18:34:20.3460481Z ##[error]Bash exited with code '3'.
2019-09-06T18:34:20.3495327Z ##[section]Starting: Checkout
2019-09-06T18:34:20.3497436Z ==============================================================================
2019-09-06T18:34:20.3497513Z Task         : Get sources
2019-09-06T18:34:20.3497561Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
