plain
2019-12-30T03:19:46.2638991Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-30T03:19:46.2655265Z ##[command]git config gc.auto 0
2019-12-30T03:19:46.2659793Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-30T03:19:46.2662029Z ##[command]git config --get-all http.proxy
2019-12-30T03:19:46.2664460Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67717/merge:refs/remotes/pull/67717/merge
---
2019-12-30T03:26:45.0564433Z    Compiling serde_json v1.0.40
2019-12-30T03:26:46.8558812Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-30T03:26:57.3803083Z     Finished release [optimized] target(s) in 1m 24s
2019-12-30T03:26:57.3919595Z tidy check
2019-12-30T03:26:58.3304837Z tidy error: /checkout/src/librustc_mir/interpret/eval_context.rs:352: TODO is deprecated; use FIXME
2019-12-30T03:27:00.0473577Z Found 486 error codes
2019-12-30T03:27:00.0473724Z Found 0 error codes with no tests
2019-12-30T03:27:00.0473774Z Done!
2019-12-30T03:27:00.0473823Z some tidy checks failed
2019-12-30T03:27:00.0473823Z some tidy checks failed
2019-12-30T03:27:00.0479766Z 
2019-12-30T03:27:00.0479873Z 
2019-12-30T03:27:00.0480885Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-30T03:27:00.0481263Z 
2019-12-30T03:27:00.0481309Z 
2019-12-30T03:27:00.0490693Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-30T03:27:00.0490828Z Build completed unsuccessfully in 0:01:35
2019-12-30T03:27:00.0490828Z Build completed unsuccessfully in 0:01:35
2019-12-30T03:27:00.0541868Z == clock drift check ==
2019-12-30T03:27:00.0552589Z   local time: Mon Dec 30 03:27:00 UTC 2019
2019-12-30T03:27:00.3630465Z   network time: Mon, 30 Dec 2019 03:27:00 GMT
2019-12-30T03:27:00.3633699Z == end clock drift check ==
2019-12-30T03:27:01.6578919Z 
2019-12-30T03:27:01.6695154Z ##[error]Bash exited with code '1'.
2019-12-30T03:27:01.6724890Z ##[section]Starting: Checkout
2019-12-30T03:27:01.6726687Z ==============================================================================
2019-12-30T03:27:01.6726768Z Task         : Get sources
2019-12-30T03:27:01.6726820Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
