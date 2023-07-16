plain
2019-12-03T03:55:04.5889700Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-03T03:55:04.6051411Z ##[command]git config gc.auto 0
2019-12-03T03:55:04.6130166Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-03T03:55:04.6184649Z ##[command]git config --get-all http.proxy
2019-12-03T03:55:04.6335065Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66884/merge:refs/remotes/pull/66884/merge
---
2019-12-03T04:00:53.7128345Z    Compiling serde_json v1.0.40
2019-12-03T04:00:55.3620598Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-03T04:01:05.7981186Z     Finished release [optimized] target(s) in 1m 24s
2019-12-03T04:01:05.8094886Z tidy check
2019-12-03T04:01:06.7309849Z tidy error: /checkout/src/libcore/num/mod.rs:846: TODO is deprecated; use FIXME
2019-12-03T04:01:08.4656520Z Found 486 error codes
2019-12-03T04:01:08.4658649Z Found 0 error codes with no tests
2019-12-03T04:01:08.4659183Z Done!
2019-12-03T04:01:08.4659456Z some tidy checks failed
2019-12-03T04:01:08.4659456Z some tidy checks failed
2019-12-03T04:01:08.4659676Z 
2019-12-03T04:01:08.4659836Z 
2019-12-03T04:01:08.4660804Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-03T04:01:08.4661401Z 
2019-12-03T04:01:08.4663347Z 
2019-12-03T04:01:08.4663803Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-03T04:01:08.4663998Z Build completed unsuccessfully in 0:01:28
2019-12-03T04:01:08.4663998Z Build completed unsuccessfully in 0:01:28
2019-12-03T04:01:08.4712163Z == clock drift check ==
2019-12-03T04:01:08.4724205Z   local time: Tue Dec  3 04:01:08 UTC 2019
2019-12-03T04:01:08.7482313Z   network time: Tue, 03 Dec 2019 04:01:08 GMT
2019-12-03T04:01:08.7482455Z == end clock drift check ==
2019-12-03T04:01:10.0046862Z 
2019-12-03T04:01:10.0147142Z ##[error]Bash exited with code '1'.
2019-12-03T04:01:10.0174769Z ##[section]Starting: Checkout
2019-12-03T04:01:10.0176530Z ==============================================================================
2019-12-03T04:01:10.0176587Z Task         : Get sources
2019-12-03T04:01:10.0176637Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
