plain
2019-09-06T20:40:01.5032819Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-06T20:40:01.5204805Z ##[command]git config gc.auto 0
2019-09-06T20:40:01.5282048Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-06T20:40:01.5342559Z ##[command]git config --get-all http.proxy
2019-09-06T20:40:01.5507894Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64227/merge:refs/remotes/pull/64227/merge
---
2019-09-06T20:46:51.7282047Z    Compiling serde_json v1.0.40
2019-09-06T20:46:53.5248470Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-06T20:47:04.3047708Z     Finished release [optimized] target(s) in 1m 30s
2019-09-06T20:47:04.3127141Z tidy check
2019-09-06T20:47:04.7989088Z tidy error: /checkout/src/librustc/ty/context.rs: ignoring file length unnecessarily
2019-09-06T20:47:06.3617552Z some tidy checks failed
2019-09-06T20:47:06.3617689Z 
2019-09-06T20:47:06.3617689Z 
2019-09-06T20:47:06.3618534Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-06T20:47:06.3618807Z 
2019-09-06T20:47:06.3618829Z 
2019-09-06T20:47:06.3668067Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-06T20:47:06.3668303Z Build completed unsuccessfully in 0:01:33
2019-09-06T20:47:06.3668303Z Build completed unsuccessfully in 0:01:33
2019-09-06T20:47:06.3676155Z == clock drift check ==
2019-09-06T20:47:06.3712443Z   local time: Fri Sep  6 20:47:06 UTC 2019
2019-09-06T20:47:06.5193639Z   network time: Fri, 06 Sep 2019 20:47:06 GMT
2019-09-06T20:47:06.5193735Z == end clock drift check ==
2019-09-06T20:47:07.8663814Z ##[error]Bash exited with code '1'.
2019-09-06T20:47:07.8698431Z ##[section]Starting: Checkout
2019-09-06T20:47:07.8700619Z ==============================================================================
2019-09-06T20:47:07.8700667Z Task         : Get sources
2019-09-06T20:47:07.8700709Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
