plain
2019-09-05T22:07:33.3122688Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-05T22:07:33.3304297Z ##[command]git config gc.auto 0
2019-09-05T22:07:33.3385433Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-05T22:07:33.3447095Z ##[command]git config --get-all http.proxy
2019-09-05T22:07:33.3637782Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64195/merge:refs/remotes/pull/64195/merge
---
2019-09-05T23:49:38.4470179Z    Compiling mdbook v0.3.1
2019-09-05T23:50:10.7346463Z    Compiling mdbook-linkcheck v0.3.0
2019-09-05T23:50:22.4199500Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2019-09-05T23:50:26.3068559Z     Finished release [optimized] target(s) in 7m 38s
2019-09-05T23:50:26.3324887Z thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error(Msg("Unable to create missing chapters"), State { next_error: Some(Error(Io(Os { code: 30, kind: Other, message: "Read-only file system" }), State { next_error: None, backtrace: InternalBacktrace { backtrace: None } })), backtrace: InternalBacktrace { backtrace: None } })', src/libcore/result.rs:1084:5
2019-09-05T23:50:26.3326644Z 
2019-09-05T23:50:26.3326852Z 
2019-09-05T23:50:26.3327352Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "linkcheck" "/checkout/src/doc/rustc-guide"
2019-09-05T23:50:26.3327719Z expected success, got: exit code: 101
---
2019-09-06T00:01:17.2008799Z This PR updated 'src/doc/rustc-guide', verifying if status is 'test-pass'...
2019-09-06T00:01:17.2019690Z 
2019-09-06T00:01:17.2020385Z ⚠️ We detected that this PR updated 'rustc-guide', but its tests failed.
2019-09-06T00:01:17.2020457Z 
2019-09-06T00:01:17.2020777Z If you do intend to update 'rustc-guide', please check the error messages above and
2019-09-06T00:01:17.2020834Z commit another update.
2019-09-06T00:01:17.2020863Z 
2019-09-06T00:01:17.2021168Z If you do NOT intend to update 'rustc-guide', please ensure you did not accidentally
2019-09-06T00:01:17.2021443Z change the submodule at 'src/doc/rustc-guide'. You may ask your reviewer for the
2019-09-06T00:01:17.2021495Z proper steps.
2019-09-06T00:01:17.2044725Z   local time: Fri Sep  6 00:01:17 UTC 2019
2019-09-06T00:01:17.3692205Z   network time: Fri, 06 Sep 2019 00:01:17 GMT
2019-09-06T00:01:17.3693182Z == end clock drift check ==
2019-09-06T00:01:17.3693182Z == end clock drift check ==
2019-09-06T00:01:19.0905257Z ##[error]Bash exited with code '3'.
2019-09-06T00:01:19.0934588Z ##[section]Starting: Checkout
2019-09-06T00:01:19.0936161Z ==============================================================================
2019-09-06T00:01:19.0936218Z Task         : Get sources
2019-09-06T00:01:19.0936255Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
