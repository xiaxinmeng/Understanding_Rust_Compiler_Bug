plain
2019-11-22T19:05:06.7739499Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-22T19:05:06.7935490Z ##[command]git config gc.auto 0
2019-11-22T19:05:06.7993095Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-22T19:05:06.8041139Z ##[command]git config --get-all http.proxy
2019-11-22T19:05:06.8162221Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66641/merge:refs/remotes/pull/66641/merge
---
2019-11-22T19:10:22.5212329Z    Compiling serde_json v1.0.40
2019-11-22T19:10:23.9851359Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-22T19:10:33.4520553Z     Finished release [optimized] target(s) in 1m 17s
2019-11-22T19:10:33.4605948Z tidy check
2019-11-22T19:10:34.1145729Z tidy error: /checkout/src/librustc_parse/parser/item.rs:1745: line longer than 100 chars
2019-11-22T19:10:35.5861900Z some tidy checks failed
2019-11-22T19:10:35.5862051Z Found 441 error codes
2019-11-22T19:10:35.5862098Z Found 0 error codes with no tests
2019-11-22T19:10:35.5862176Z Done!
2019-11-22T19:10:35.5862176Z Done!
2019-11-22T19:10:35.5865969Z 
2019-11-22T19:10:35.5866034Z 
2019-11-22T19:10:35.5867032Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-22T19:10:35.5867162Z 
2019-11-22T19:10:35.5867187Z 
2019-11-22T19:10:35.5873505Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-22T19:10:35.5873774Z Build completed unsuccessfully in 0:01:20
2019-11-22T19:10:35.5873774Z Build completed unsuccessfully in 0:01:20
2019-11-22T19:10:35.5918879Z == clock drift check ==
2019-11-22T19:10:35.5927158Z   local time: Fri Nov 22 19:10:35 UTC 2019
2019-11-22T19:10:35.8687866Z   network time: Fri, 22 Nov 2019 19:10:35 GMT
2019-11-22T19:10:35.8693570Z == end clock drift check ==
2019-11-22T19:10:37.2835390Z 
2019-11-22T19:10:37.2926045Z ##[error]Bash exited with code '1'.
2019-11-22T19:10:37.2948422Z ##[section]Starting: Checkout
2019-11-22T19:10:37.2949777Z ==============================================================================
2019-11-22T19:10:37.2949818Z Task         : Get sources
2019-11-22T19:10:37.2949853Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
