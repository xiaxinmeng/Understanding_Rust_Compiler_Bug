plain
2019-11-09T04:20:54.3221893Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-09T04:20:54.3411918Z ##[command]git config gc.auto 0
2019-11-09T04:20:54.3532547Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-09T04:20:54.3570880Z ##[command]git config --get-all http.proxy
2019-11-09T04:20:54.9243264Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66215/merge:refs/remotes/pull/66215/merge
---
2019-11-09T04:27:00.9895001Z    Compiling serde_json v1.0.40
2019-11-09T04:27:02.6654166Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-09T04:27:13.3237758Z     Finished release [optimized] target(s) in 1m 22s
2019-11-09T04:27:13.3323816Z tidy check
2019-11-09T04:27:13.4697986Z tidy error: /checkout/src/liballoc/string.rs:2003: line longer than 100 chars
2019-11-09T04:27:15.8324500Z some tidy checks failed
2019-11-09T04:27:15.8324601Z Found 485 error codes
2019-11-09T04:27:15.8324638Z Found 0 error codes with no tests
2019-11-09T04:27:15.8327725Z Done!
2019-11-09T04:27:15.8327725Z Done!
2019-11-09T04:27:15.8327776Z 
2019-11-09T04:27:15.8327832Z 
2019-11-09T04:27:15.8329021Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-09T04:27:15.8329135Z 
2019-11-09T04:27:15.8329162Z 
2019-11-09T04:27:15.8329655Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-09T04:27:15.8329713Z Build completed unsuccessfully in 0:01:25
2019-11-09T04:27:15.8329713Z Build completed unsuccessfully in 0:01:25
2019-11-09T04:27:15.8381025Z == clock drift check ==
2019-11-09T04:27:15.8405179Z   local time: Sat Nov  9 04:27:15 UTC 2019
2019-11-09T04:27:16.1171815Z   network time: Sat, 09 Nov 2019 04:27:16 GMT
2019-11-09T04:27:16.1171906Z == end clock drift check ==
2019-11-09T04:27:17.5026247Z 
2019-11-09T04:27:17.5130701Z ##[error]Bash exited with code '1'.
2019-11-09T04:27:17.5152746Z ##[section]Starting: Checkout
2019-11-09T04:27:17.5154127Z ==============================================================================
2019-11-09T04:27:17.5154169Z Task         : Get sources
2019-11-09T04:27:17.5154229Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
