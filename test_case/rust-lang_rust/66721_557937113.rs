plain
2019-11-24T22:44:42.5388500Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-24T22:44:42.5577716Z ##[command]git config gc.auto 0
2019-11-24T22:44:42.5657486Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-24T22:44:42.5703578Z ##[command]git config --get-all http.proxy
2019-11-24T22:44:42.5854532Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66721/merge:refs/remotes/pull/66721/merge
---
2019-11-24T22:49:55.8088732Z   = note: `#[warn(unused_imports)]` on by default
2019-11-24T22:49:55.8092986Z 
2019-11-24T22:50:05.6018403Z     Finished release [optimized] target(s) in 1m 18s
2019-11-24T22:50:05.6100531Z tidy check
2019-11-24T22:50:06.4316935Z tidy error: /checkout/src/libcore/fmt/num.rs:264: line longer than 100 chars
2019-11-24T22:50:06.4317701Z tidy error: /checkout/src/libcore/fmt/num.rs:314: line longer than 100 chars
2019-11-24T22:50:07.8744686Z some tidy checks failed
2019-11-24T22:50:07.8744792Z Found 485 error codes
2019-11-24T22:50:07.8745138Z Found 0 error codes with no tests
2019-11-24T22:50:07.8745187Z Done!
2019-11-24T22:50:07.8745187Z Done!
2019-11-24T22:50:07.8748692Z 
2019-11-24T22:50:07.8749013Z 
2019-11-24T22:50:07.8749955Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-24T22:50:07.8750466Z 
2019-11-24T22:50:07.8750619Z 
2019-11-24T22:50:07.8752861Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-24T22:50:07.8753115Z Build completed unsuccessfully in 0:01:21
2019-11-24T22:50:07.8753115Z Build completed unsuccessfully in 0:01:21
2019-11-24T22:50:07.8790615Z == clock drift check ==
2019-11-24T22:50:07.8797574Z   local time: Sun Nov 24 22:50:07 UTC 2019
2019-11-24T22:50:08.1685606Z   network time: Sun, 24 Nov 2019 22:50:08 GMT
2019-11-24T22:50:08.1689515Z == end clock drift check ==
2019-11-24T22:50:09.5912095Z 
2019-11-24T22:50:09.6003494Z ##[error]Bash exited with code '1'.
2019-11-24T22:50:09.6025858Z ##[section]Starting: Checkout
2019-11-24T22:50:09.6027324Z ==============================================================================
2019-11-24T22:50:09.6027388Z Task         : Get sources
2019-11-24T22:50:09.6027427Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
