plain
2019-12-18T19:22:19.0234136Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-18T19:22:19.0404621Z ##[command]git config gc.auto 0
2019-12-18T19:22:19.0482416Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-18T19:22:19.0543549Z ##[command]git config --get-all http.proxy
2019-12-18T19:22:19.0701016Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67241/merge:refs/remotes/pull/67241/merge
---
2019-12-18T19:27:53.5297214Z    Compiling serde_json v1.0.40
2019-12-18T19:27:55.1582003Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-18T19:28:05.8331403Z     Finished release [optimized] target(s) in 1m 23s
2019-12-18T19:28:05.8447133Z tidy check
2019-12-18T19:28:06.8083496Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1570: TODO is deprecated; use FIXME
2019-12-18T19:28:06.8084162Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1572: line longer than 100 chars
2019-12-18T19:28:08.6254886Z some tidy checks failed
2019-12-18T19:28:08.6256381Z Found 485 error codes
2019-12-18T19:28:08.6256432Z Found 0 error codes with no tests
2019-12-18T19:28:08.6256723Z Done!
2019-12-18T19:28:08.6256723Z Done!
2019-12-18T19:28:08.6260284Z 
2019-12-18T19:28:08.6260446Z 
2019-12-18T19:28:08.6261467Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-18T19:28:08.6263379Z 
2019-12-18T19:28:08.6263694Z 
2019-12-18T19:28:08.6263883Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-18T19:28:08.6264071Z Build completed unsuccessfully in 0:01:27
2019-12-18T19:28:08.6264071Z Build completed unsuccessfully in 0:01:27
2019-12-18T19:28:08.6315740Z == clock drift check ==
2019-12-18T19:28:08.6327594Z   local time: Wed Dec 18 19:28:08 UTC 2019
2019-12-18T19:28:08.9018203Z   network time: Wed, 18 Dec 2019 19:28:08 GMT
2019-12-18T19:28:08.9018318Z == end clock drift check ==
2019-12-18T19:28:10.1978214Z 
2019-12-18T19:28:10.2046428Z ##[error]Bash exited with code '1'.
2019-12-18T19:28:10.2087790Z ##[section]Starting: Checkout
2019-12-18T19:28:10.2089588Z ==============================================================================
2019-12-18T19:28:10.2089648Z Task         : Get sources
2019-12-18T19:28:10.2089702Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
