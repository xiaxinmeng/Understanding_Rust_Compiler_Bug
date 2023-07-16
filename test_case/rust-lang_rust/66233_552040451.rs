plain
2019-11-09T00:00:52.5942829Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-09T00:00:52.6124378Z ##[command]git config gc.auto 0
2019-11-09T00:00:52.6191707Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-09T00:00:52.6240894Z ##[command]git config --get-all http.proxy
2019-11-09T00:00:52.6393456Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66233/merge:refs/remotes/pull/66233/merge
---
2019-11-09T00:07:16.9798099Z    Compiling serde_json v1.0.40
2019-11-09T00:07:18.8232814Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-09T00:07:30.9403194Z     Finished release [optimized] target(s) in 1m 32s
2019-11-09T00:07:30.9492947Z tidy check
2019-11-09T00:07:31.6550920Z tidy error: /checkout/src/librustc_mir/hair/pattern/_match.rs:330: line longer than 100 chars
2019-11-09T00:07:31.6556545Z tidy error: /checkout/src/librustc_mir/hair/pattern/_match.rs:1206: line longer than 100 chars
2019-11-09T00:07:32.0469741Z tidy error: /checkout/src/librustc_codegen_ssa/mir/place.rs:483: line longer than 100 chars
2019-11-09T00:07:33.7804844Z Found 485 error codes
2019-11-09T00:07:33.7810019Z Found 0 error codes with no tests
2019-11-09T00:07:33.7810461Z Done!
2019-11-09T00:07:33.7810692Z some tidy checks failed
2019-11-09T00:07:33.7810692Z some tidy checks failed
2019-11-09T00:07:33.7810931Z 
2019-11-09T00:07:33.7811061Z 
2019-11-09T00:07:33.7812589Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-09T00:07:33.7812968Z 
2019-11-09T00:07:33.7813077Z 
2019-11-09T00:07:33.7818802Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-09T00:07:33.7818904Z Build completed unsuccessfully in 0:01:36
2019-11-09T00:07:33.7818904Z Build completed unsuccessfully in 0:01:36
2019-11-09T00:07:33.7867495Z == clock drift check ==
2019-11-09T00:07:33.7889838Z   local time: Sat Nov  9 00:07:33 UTC 2019
2019-11-09T00:07:33.8653857Z   network time: Sat, 09 Nov 2019 00:07:33 GMT
2019-11-09T00:07:33.8657082Z == end clock drift check ==
2019-11-09T00:07:35.1801898Z 
2019-11-09T00:07:35.1902060Z ##[error]Bash exited with code '1'.
2019-11-09T00:07:35.1938471Z ##[section]Starting: Checkout
2019-11-09T00:07:35.1940315Z ==============================================================================
2019-11-09T00:07:35.1940391Z Task         : Get sources
2019-11-09T00:07:35.1940441Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
