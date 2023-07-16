plain
2019-11-23T02:42:06.1119436Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-23T02:42:06.1438456Z ##[command]git config gc.auto 0
2019-11-23T02:42:06.1504413Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-23T02:42:06.1569027Z ##[command]git config --get-all http.proxy
2019-11-23T02:42:06.1716511Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66641/merge:refs/remotes/pull/66641/merge
---
2019-11-23T02:48:10.8576342Z    Compiling serde_json v1.0.40
2019-11-23T02:48:12.5710517Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-23T02:48:23.3918291Z     Finished release [optimized] target(s) in 1m 26s
2019-11-23T02:48:23.4022469Z tidy check
2019-11-23T02:48:24.2920349Z tidy error: /checkout/src/librustc_parse/parser/item.rs:1765: line longer than 100 chars
2019-11-23T02:48:24.2921013Z tidy error: /checkout/src/librustc_parse/parser/item.rs:1769: line longer than 100 chars
2019-11-23T02:48:26.1014556Z Found 441 error codes
2019-11-23T02:48:26.1014785Z Found 0 error codes with no tests
2019-11-23T02:48:26.1015147Z Done!
2019-11-23T02:48:26.1015261Z some tidy checks failed
2019-11-23T02:48:26.1015261Z some tidy checks failed
2019-11-23T02:48:26.1015428Z 
2019-11-23T02:48:26.1015762Z 
2019-11-23T02:48:26.1016872Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-23T02:48:26.1017046Z 
2019-11-23T02:48:26.1017113Z 
2019-11-23T02:48:26.1026841Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-23T02:48:26.1027251Z Build completed unsuccessfully in 0:01:30
2019-11-23T02:48:26.1027251Z Build completed unsuccessfully in 0:01:30
2019-11-23T02:48:26.1079372Z == clock drift check ==
2019-11-23T02:48:26.1089789Z   local time: Sat Nov 23 02:48:26 UTC 2019
2019-11-23T02:48:26.1814312Z   network time: Sat, 23 Nov 2019 02:48:26 GMT
2019-11-23T02:48:26.1817922Z == end clock drift check ==
2019-11-23T02:48:27.4667014Z 
2019-11-23T02:48:27.4779112Z ##[error]Bash exited with code '1'.
2019-11-23T02:48:27.4808138Z ##[section]Starting: Checkout
2019-11-23T02:48:27.4810257Z ==============================================================================
2019-11-23T02:48:27.4810343Z Task         : Get sources
2019-11-23T02:48:27.4810396Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
