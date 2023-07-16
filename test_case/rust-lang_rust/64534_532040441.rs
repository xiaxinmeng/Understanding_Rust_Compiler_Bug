plain
2019-09-17T00:40:57.0877400Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-17T00:40:57.1082560Z ##[command]git config gc.auto 0
2019-09-17T00:40:57.1165378Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-17T00:40:57.1198368Z ##[command]git config --get-all http.proxy
2019-09-17T00:40:57.8409896Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64534/merge:refs/remotes/pull/64534/merge
---
2019-09-17T00:47:55.3443163Z    Compiling serde_json v1.0.40
2019-09-17T00:47:57.2211472Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-17T00:48:08.2323988Z     Finished release [optimized] target(s) in 1m 30s
2019-09-17T00:48:08.2401936Z tidy check
2019-09-17T00:48:09.1636013Z tidy error: /checkout/src/test/ui/stdarch/is_x86_feature_detected-gate.rs: leading newline
2019-09-17T00:48:10.4997865Z some tidy checks failed
2019-09-17T00:48:10.5018233Z 
2019-09-17T00:48:10.5018233Z 
2019-09-17T00:48:10.5019163Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-17T00:48:10.5019483Z 
2019-09-17T00:48:10.5019618Z 
2019-09-17T00:48:10.5030403Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-17T00:48:10.5030730Z Build completed unsuccessfully in 0:01:34
2019-09-17T00:48:10.5030730Z Build completed unsuccessfully in 0:01:34
2019-09-17T00:48:10.5081477Z == clock drift check ==
2019-09-17T00:48:10.5099209Z   local time: Tue Sep 17 00:48:10 UTC 2019
2019-09-17T00:48:10.6628983Z   network time: Tue, 17 Sep 2019 00:48:10 GMT
2019-09-17T00:48:10.6630736Z == end clock drift check ==
2019-09-17T00:48:11.9506749Z ##[error]Bash exited with code '1'.
2019-09-17T00:48:11.9546176Z ##[section]Starting: Checkout
2019-09-17T00:48:11.9547937Z ==============================================================================
2019-09-17T00:48:11.9548008Z Task         : Get sources
2019-09-17T00:48:11.9548052Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
