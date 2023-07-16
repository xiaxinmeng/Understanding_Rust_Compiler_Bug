plain
2019-09-09T05:13:19.4711106Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-09T05:13:19.4899307Z ##[command]git config gc.auto 0
2019-09-09T05:13:19.4966376Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-09T05:13:19.5016401Z ##[command]git config --get-all http.proxy
2019-09-09T05:13:19.5150777Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64278/merge:refs/remotes/pull/64278/merge
---
2019-09-09T05:19:52.6630551Z    Compiling serde_json v1.0.40
2019-09-09T05:19:54.2355076Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-09T05:20:03.8993698Z     Finished release [optimized] target(s) in 1m 19s
2019-09-09T05:20:03.9063668Z tidy check
2019-09-09T05:20:04.6136336Z tidy error: /checkout/src/bootstrap/bootstrap.py:716: line longer than 100 chars
2019-09-09T05:20:05.6167283Z some tidy checks failed
2019-09-09T05:20:05.6167425Z 
2019-09-09T05:20:05.6167425Z 
2019-09-09T05:20:05.6176147Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-09T05:20:05.6181533Z 
2019-09-09T05:20:05.6181556Z 
2019-09-09T05:20:05.6192847Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-09T05:20:05.6192906Z Build completed unsuccessfully in 0:01:22
2019-09-09T05:20:05.6192906Z Build completed unsuccessfully in 0:01:22
2019-09-09T05:20:05.6241141Z == clock drift check ==
2019-09-09T05:20:05.6253368Z   local time: Mon Sep  9 05:20:05 UTC 2019
2019-09-09T05:20:05.7115307Z   network time: Mon, 09 Sep 2019 05:20:05 GMT
2019-09-09T05:20:05.7121441Z == end clock drift check ==
2019-09-09T05:20:07.1732678Z ##[error]Bash exited with code '1'.
2019-09-09T05:20:07.1764560Z ##[section]Starting: Checkout
2019-09-09T05:20:07.1766635Z ==============================================================================
2019-09-09T05:20:07.1766697Z Task         : Get sources
2019-09-09T05:20:07.1766953Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
