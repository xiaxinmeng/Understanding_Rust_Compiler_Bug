plain
2019-11-10T10:10:33.1702813Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-10T10:10:33.1862457Z ##[command]git config gc.auto 0
2019-11-10T10:10:33.1926464Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-10T10:10:33.1964284Z ##[command]git config --get-all http.proxy
2019-11-10T10:10:33.2089682Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66129/merge:refs/remotes/pull/66129/merge
---
2019-11-10T10:16:20.8910064Z    Compiling serde_json v1.0.40
2019-11-10T10:16:22.5869432Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-10T10:16:32.9150755Z     Finished release [optimized] target(s) in 1m 18s
2019-11-10T10:16:32.9223055Z tidy check
2019-11-10T10:16:33.5007608Z tidy error: /checkout/src/librustc_mir/hair/pattern/_match.rs:813: line longer than 100 chars
2019-11-10T10:16:33.5007700Z tidy error: /checkout/src/librustc_mir/hair/pattern/_match.rs:816: line longer than 100 chars
2019-11-10T10:16:35.3787351Z Found 485 error codes
2019-11-10T10:16:35.3789933Z Found 0 error codes with no tests
2019-11-10T10:16:35.3790286Z Done!
2019-11-10T10:16:35.3790494Z some tidy checks failed
2019-11-10T10:16:35.3790494Z some tidy checks failed
2019-11-10T10:16:35.3790756Z 
2019-11-10T10:16:35.3790967Z 
2019-11-10T10:16:35.3792046Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-10T10:16:35.3792567Z 
2019-11-10T10:16:35.3792742Z 
2019-11-10T10:16:35.3795934Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-10T10:16:35.3796278Z Build completed unsuccessfully in 0:01:22
2019-11-10T10:16:35.3796278Z Build completed unsuccessfully in 0:01:22
2019-11-10T10:16:35.3845638Z == clock drift check ==
2019-11-10T10:16:35.3858298Z   local time: Sun Nov 10 10:16:35 UTC 2019
2019-11-10T10:16:35.5342291Z   network time: Sun, 10 Nov 2019 10:16:35 GMT
2019-11-10T10:16:35.5342748Z == end clock drift check ==
2019-11-10T10:16:36.8996255Z 
2019-11-10T10:16:36.9059751Z ##[error]Bash exited with code '1'.
2019-11-10T10:16:36.9090769Z ##[section]Starting: Checkout
2019-11-10T10:16:36.9092341Z ==============================================================================
2019-11-10T10:16:36.9092408Z Task         : Get sources
2019-11-10T10:16:36.9092455Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
