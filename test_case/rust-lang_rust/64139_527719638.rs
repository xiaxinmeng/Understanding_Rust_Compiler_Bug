plain
2019-09-04T02:45:13.0198656Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-04T02:45:13.0382651Z ##[command]git config gc.auto 0
2019-09-04T02:45:13.0432348Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-04T02:45:13.0482698Z ##[command]git config --get-all http.proxy
2019-09-04T02:45:13.0610097Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64139/merge:refs/remotes/pull/64139/merge
---
2019-09-04T02:51:49.7805219Z     Finished release [optimized] target(s) in 1m 17s
2019-09-04T02:51:49.7877148Z tidy check
2019-09-04T02:51:50.5603721Z * 578 error codes
2019-09-04T02:51:50.5603893Z * highest error code: E0733
2019-09-04T02:51:50.5897590Z tidy error: /checkout/src/libsyntax/feature_gate/removed.rs:107: feature rustc_diagnostic_macros is not sorted by since
2019-09-04T02:51:51.4225887Z some tidy checks failed
2019-09-04T02:51:51.4226873Z 
2019-09-04T02:51:51.4226873Z 
2019-09-04T02:51:51.4227949Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-04T02:51:51.4232025Z 
2019-09-04T02:51:51.4232936Z 
2019-09-04T02:51:51.4239634Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-04T02:51:51.4239919Z Build completed unsuccessfully in 0:01:19
2019-09-04T02:51:51.4239919Z Build completed unsuccessfully in 0:01:19
2019-09-04T02:51:51.4285577Z == clock drift check ==
2019-09-04T02:51:51.4302644Z   local time: Wed Sep  4 02:51:51 UTC 2019
2019-09-04T02:51:51.5771690Z   network time: Wed, 04 Sep 2019 02:51:51 GMT
2019-09-04T02:51:51.5777353Z == end clock drift check ==
2019-09-04T02:51:53.2157074Z ##[error]Bash exited with code '1'.
2019-09-04T02:51:53.2185831Z ##[section]Starting: Checkout
2019-09-04T02:51:53.2187291Z ==============================================================================
2019-09-04T02:51:53.2187332Z Task         : Get sources
2019-09-04T02:51:53.2187372Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
