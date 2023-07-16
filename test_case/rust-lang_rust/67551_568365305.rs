plain
2019-12-23T05:31:53.1397394Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T05:31:53.1604277Z ##[command]git config gc.auto 0
2019-12-23T05:31:53.1638585Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T05:31:53.1683186Z ##[command]git config --get-all http.proxy
2019-12-23T05:31:53.1825764Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67551/merge:refs/remotes/pull/67551/merge
---
2019-12-23T05:37:56.6443087Z    Compiling serde_json v1.0.40
2019-12-23T05:37:58.2203260Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-23T05:38:08.7356510Z     Finished release [optimized] target(s) in 1m 23s
2019-12-23T05:38:08.7469420Z tidy check
2019-12-23T05:38:09.0224375Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0627.md:1: line longer than 80 chars
2019-12-23T05:38:11.4558200Z some tidy checks failed
2019-12-23T05:38:11.4561966Z Found 485 error codes
2019-12-23T05:38:11.4562631Z Found 0 error codes with no tests
2019-12-23T05:38:11.4562814Z Done!
2019-12-23T05:38:11.4562814Z Done!
2019-12-23T05:38:11.4563049Z 
2019-12-23T05:38:11.4563246Z 
2019-12-23T05:38:11.4564189Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-23T05:38:11.4564699Z 
2019-12-23T05:38:11.4564848Z 
2019-12-23T05:38:11.4568086Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-23T05:38:11.4568374Z Build completed unsuccessfully in 0:01:27
2019-12-23T05:38:11.4568374Z Build completed unsuccessfully in 0:01:27
2019-12-23T05:38:11.4616624Z == clock drift check ==
2019-12-23T05:38:11.4625502Z   local time: Mon Dec 23 05:38:11 UTC 2019
2019-12-23T05:38:11.7428937Z   network time: Mon, 23 Dec 2019 05:38:11 GMT
2019-12-23T05:38:11.7432161Z == end clock drift check ==
2019-12-23T05:38:13.1569981Z 
2019-12-23T05:38:13.1664161Z ##[error]Bash exited with code '1'.
2019-12-23T05:38:13.1718099Z ##[section]Starting: Checkout
2019-12-23T05:38:13.1720174Z ==============================================================================
2019-12-23T05:38:13.1720249Z Task         : Get sources
2019-12-23T05:38:13.1720299Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
