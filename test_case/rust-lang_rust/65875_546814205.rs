plain
2019-10-28T06:32:06.9846214Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-28T06:32:07.0233690Z ##[command]git config gc.auto 0
2019-10-28T06:32:07.0315408Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-28T06:32:07.0375398Z ##[command]git config --get-all http.proxy
2019-10-28T06:32:07.0510527Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65875/merge:refs/remotes/pull/65875/merge
---
2019-10-28T06:38:40.8498611Z    Compiling serde_json v1.0.40
2019-10-28T06:38:42.5306194Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-28T06:38:53.1428741Z     Finished release [optimized] target(s) in 1m 23s
2019-10-28T06:38:53.1521584Z tidy check
2019-10-28T06:38:53.7420298Z tidy error: /checkout/src/libstd/tests/block_on_future.rs:255: trailing whitespace
2019-10-28T06:38:53.7594233Z tidy error: /checkout/src/libstd/thread/block_on_future.rs:116: trailing whitespace
2019-10-28T06:38:55.2248009Z Found 484 error codes
2019-10-28T06:38:55.2248789Z Found 0 error codes with no tests
2019-10-28T06:38:55.2249061Z Done!
2019-10-28T06:38:55.2252176Z some tidy checks failed
2019-10-28T06:38:55.2252176Z some tidy checks failed
2019-10-28T06:38:55.2254757Z 
2019-10-28T06:38:55.2261678Z 
2019-10-28T06:38:55.2262765Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-28T06:38:55.2262921Z 
2019-10-28T06:38:55.2262947Z 
2019-10-28T06:38:55.2262994Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-28T06:38:55.2263077Z Build completed unsuccessfully in 0:01:26
2019-10-28T06:38:55.2263077Z Build completed unsuccessfully in 0:01:26
2019-10-28T06:38:55.2311952Z == clock drift check ==
2019-10-28T06:38:55.2325957Z   local time: Mon Oct 28 06:38:55 UTC 2019
2019-10-28T06:38:55.3044892Z   network time: Mon, 28 Oct 2019 06:38:55 GMT
2019-10-28T06:38:55.3051609Z == end clock drift check ==
2019-10-28T06:38:56.7259284Z 
2019-10-28T06:38:56.7396196Z ##[error]Bash exited with code '1'.
2019-10-28T06:38:56.7430328Z ##[section]Starting: Checkout
2019-10-28T06:38:56.7432524Z ==============================================================================
2019-10-28T06:38:56.7432586Z Task         : Get sources
2019-10-28T06:38:56.7432632Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
