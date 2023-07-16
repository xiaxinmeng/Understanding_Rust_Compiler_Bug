plain
2019-12-06T18:51:46.7039288Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-06T18:51:46.7179595Z ##[command]git config gc.auto 0
2019-12-06T18:51:46.7262575Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-06T18:51:46.7315303Z ##[command]git config --get-all http.proxy
2019-12-06T18:51:46.7456295Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67035/merge:refs/remotes/pull/67035/merge
---
2019-12-06T18:57:35.5858739Z    Compiling serde_json v1.0.40
2019-12-06T18:57:37.2387426Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-06T18:57:47.7351169Z     Finished release [optimized] target(s) in 1m 23s
2019-12-06T18:57:47.7448552Z tidy check
2019-12-06T18:57:48.8490592Z tidy error: /checkout/src/libstd/net/ip.rs:1923: line longer than 100 chars
2019-12-06T18:57:50.4417648Z some tidy checks failed
2019-12-06T18:57:50.4427586Z Found 486 error codes
2019-12-06T18:57:50.4430084Z Found 0 error codes with no tests
2019-12-06T18:57:50.4430413Z Done!
2019-12-06T18:57:50.4430413Z Done!
2019-12-06T18:57:50.4430736Z 
2019-12-06T18:57:50.4430932Z 
2019-12-06T18:57:50.4433307Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-06T18:57:50.4433459Z 
2019-12-06T18:57:50.4433504Z 
2019-12-06T18:57:50.4445286Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-06T18:57:50.4445371Z Build completed unsuccessfully in 0:01:27
2019-12-06T18:57:50.4445371Z Build completed unsuccessfully in 0:01:27
2019-12-06T18:57:50.4493628Z == clock drift check ==
2019-12-06T18:57:50.4504571Z   local time: Fri Dec  6 18:57:50 UTC 2019
2019-12-06T18:57:50.4677549Z   network time: Fri, 06 Dec 2019 18:57:50 GMT
2019-12-06T18:57:50.4681787Z == end clock drift check ==
2019-12-06T18:57:51.8729037Z 
2019-12-06T18:57:51.8857354Z ##[error]Bash exited with code '1'.
2019-12-06T18:57:51.8885090Z ##[section]Starting: Checkout
2019-12-06T18:57:51.8886676Z ==============================================================================
2019-12-06T18:57:51.8886749Z Task         : Get sources
2019-12-06T18:57:51.8886795Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
