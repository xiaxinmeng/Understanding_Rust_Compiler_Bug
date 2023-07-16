plain
2019-12-18T16:40:24.7955345Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-18T16:40:24.8197118Z ##[command]git config gc.auto 0
2019-12-18T16:40:24.8269333Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-18T16:40:24.8321660Z ##[command]git config --get-all http.proxy
2019-12-18T16:40:24.8446496Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67397/merge:refs/remotes/pull/67397/merge
---
2019-12-18T16:45:53.3899191Z    Compiling serde_json v1.0.40
2019-12-18T16:45:54.7476804Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-18T16:46:03.7159422Z     Finished release [optimized] target(s) in 1m 10s
2019-12-18T16:46:03.7242610Z tidy check
2019-12-18T16:46:04.4002700Z tidy error: /checkout/src/librustc/ty/query/profiling_support.rs: leading newline
2019-12-18T16:46:05.8358022Z Found 485 error codes
2019-12-18T16:46:05.8358175Z Found 0 error codes with no tests
2019-12-18T16:46:05.8358215Z Done!
2019-12-18T16:46:05.8363181Z some tidy checks failed
2019-12-18T16:46:05.8363181Z some tidy checks failed
2019-12-18T16:46:05.8363424Z 
2019-12-18T16:46:05.8363589Z 
2019-12-18T16:46:05.8364725Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-18T16:46:05.8365116Z 
2019-12-18T16:46:05.8365222Z 
2019-12-18T16:46:05.8365397Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-18T16:46:05.8365554Z Build completed unsuccessfully in 0:01:13
2019-12-18T16:46:05.8365554Z Build completed unsuccessfully in 0:01:13
2019-12-18T16:46:05.8411895Z == clock drift check ==
2019-12-18T16:46:05.8423050Z   local time: Wed Dec 18 16:46:05 UTC 2019
2019-12-18T16:46:06.1297352Z   network time: Wed, 18 Dec 2019 16:46:06 GMT
2019-12-18T16:46:06.1297458Z == end clock drift check ==
2019-12-18T16:46:07.4748868Z 
2019-12-18T16:46:07.4840040Z ##[error]Bash exited with code '1'.
2019-12-18T16:46:07.4865665Z ##[section]Starting: Checkout
2019-12-18T16:46:07.4867129Z ==============================================================================
2019-12-18T16:46:07.4867308Z Task         : Get sources
2019-12-18T16:46:07.4867344Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
