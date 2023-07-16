plain
2019-11-11T05:34:49.4124711Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-11T05:34:49.4320828Z ##[command]git config gc.auto 0
2019-11-11T05:34:49.4401067Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-11T05:34:49.4445102Z ##[command]git config --get-all http.proxy
2019-11-11T05:34:49.4617656Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66284/merge:refs/remotes/pull/66284/merge
---
2019-11-11T05:41:34.8512758Z    Compiling serde_json v1.0.40
2019-11-11T05:41:36.6799965Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-11T05:41:48.3148275Z     Finished release [optimized] target(s) in 1m 30s
2019-11-11T05:41:48.3226654Z tidy check
2019-11-11T05:41:48.4674462Z tidy error: /checkout/src/liballoc/string.rs:1078: trailing whitespace
2019-11-11T05:41:48.4676250Z tidy error: /checkout/src/liballoc/string.rs:1082: line longer than 100 chars
2019-11-11T05:41:51.1889553Z Found 485 error codes
2019-11-11T05:41:51.1889683Z Found 0 error codes with no tests
2019-11-11T05:41:51.1889784Z Done!
2019-11-11T05:41:51.1889828Z some tidy checks failed
2019-11-11T05:41:51.1889828Z some tidy checks failed
2019-11-11T05:41:51.1889861Z 
2019-11-11T05:41:51.1889889Z 
2019-11-11T05:41:51.1890832Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-11T05:41:51.1890975Z 
2019-11-11T05:41:51.1891003Z 
2019-11-11T05:41:51.1891073Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-11T05:41:51.1891139Z Build completed unsuccessfully in 0:01:34
2019-11-11T05:41:51.1891139Z Build completed unsuccessfully in 0:01:34
2019-11-11T05:41:51.1940451Z == clock drift check ==
2019-11-11T05:41:51.1950274Z   local time: Mon Nov 11 05:41:51 UTC 2019
2019-11-11T05:41:51.2683317Z   network time: Mon, 11 Nov 2019 05:41:51 GMT
2019-11-11T05:41:51.2683439Z == end clock drift check ==
2019-11-11T05:41:52.6258197Z 
2019-11-11T05:41:52.6395008Z ##[error]Bash exited with code '1'.
2019-11-11T05:41:52.6440095Z ##[section]Starting: Checkout
2019-11-11T05:41:52.6441844Z ==============================================================================
2019-11-11T05:41:52.6441916Z Task         : Get sources
2019-11-11T05:41:52.6441967Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
