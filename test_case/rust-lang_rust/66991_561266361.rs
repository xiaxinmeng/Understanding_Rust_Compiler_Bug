plain
2019-12-03T16:57:39.4219675Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-03T16:57:40.3051365Z ##[command]git config gc.auto 0
2019-12-03T16:57:40.3058413Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-03T16:57:40.3064201Z ##[command]git config --get-all http.proxy
2019-12-03T16:57:40.3069056Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66991/merge:refs/remotes/pull/66991/merge
---
2019-12-03T17:03:35.2079995Z    Compiling serde_json v1.0.40
2019-12-03T17:03:36.8525170Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-03T17:03:47.5697799Z     Finished release [optimized] target(s) in 1m 25s
2019-12-03T17:03:47.5796719Z tidy check
2019-12-03T17:03:48.2323409Z tidy error: /checkout/src/librustc_mir/util/def_use.rs:3: line longer than 100 chars
2019-12-03T17:03:48.2767910Z tidy error: /checkout/src/librustc_mir/shim.rs:322: line longer than 100 chars
2019-12-03T17:03:50.2831618Z some tidy checks failed
2019-12-03T17:03:50.2833957Z Found 486 error codes
2019-12-03T17:03:50.2834652Z Found 0 error codes with no tests
2019-12-03T17:03:50.2834751Z Done!
2019-12-03T17:03:50.2834751Z Done!
2019-12-03T17:03:50.2834854Z 
2019-12-03T17:03:50.2839001Z 
2019-12-03T17:03:50.2840118Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-03T17:03:50.2840554Z 
2019-12-03T17:03:50.2840685Z 
2019-12-03T17:03:50.2849872Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-03T17:03:50.2850203Z Build completed unsuccessfully in 0:01:29
2019-12-03T17:03:50.2850203Z Build completed unsuccessfully in 0:01:29
2019-12-03T17:03:50.2906645Z == clock drift check ==
2019-12-03T17:03:50.2918001Z   local time: Tue Dec  3 17:03:50 UTC 2019
2019-12-03T17:03:50.4479048Z   network time: Tue, 03 Dec 2019 17:03:50 GMT
2019-12-03T17:03:50.4482466Z == end clock drift check ==
2019-12-03T17:03:51.7940227Z 
2019-12-03T17:03:51.8012439Z ##[error]Bash exited with code '1'.
2019-12-03T17:03:51.8041518Z ##[section]Starting: Checkout
2019-12-03T17:03:51.8043904Z ==============================================================================
2019-12-03T17:03:51.8043987Z Task         : Get sources
2019-12-03T17:03:51.8044041Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
