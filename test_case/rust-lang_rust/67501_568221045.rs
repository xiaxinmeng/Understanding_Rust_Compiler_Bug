plain
2019-12-21T23:39:06.3234542Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-21T23:39:06.3413682Z ##[command]git config gc.auto 0
2019-12-21T23:39:06.3511171Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-21T23:39:06.3536184Z ##[command]git config --get-all http.proxy
2019-12-21T23:39:06.3721780Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67501/merge:refs/remotes/pull/67501/merge
---
2019-12-21T23:44:46.1061649Z    Compiling serde_json v1.0.40
2019-12-21T23:44:47.6543766Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-21T23:44:57.7308178Z     Finished release [optimized] target(s) in 1m 19s
2019-12-21T23:44:57.7400879Z tidy check
2019-12-21T23:44:58.5787293Z tidy error: /checkout/src/librustc_mir/interpret/place.rs:238: line longer than 100 chars
2019-12-21T23:44:58.5788009Z tidy error: /checkout/src/librustc_mir/interpret/place.rs:241: line longer than 100 chars
2019-12-21T23:45:00.1559536Z Found 485 error codes
2019-12-21T23:45:00.1559660Z Found 0 error codes with no tests
2019-12-21T23:45:00.1559727Z Done!
2019-12-21T23:45:00.1559769Z some tidy checks failed
2019-12-21T23:45:00.1559769Z some tidy checks failed
2019-12-21T23:45:00.1559796Z 
2019-12-21T23:45:00.1559823Z 
2019-12-21T23:45:00.1561413Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-21T23:45:00.1561607Z 
2019-12-21T23:45:00.1561664Z 
2019-12-21T23:45:00.1568203Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-21T23:45:00.1568266Z Build completed unsuccessfully in 0:01:23
2019-12-21T23:45:00.1568266Z Build completed unsuccessfully in 0:01:23
2019-12-21T23:45:00.1623415Z == clock drift check ==
2019-12-21T23:45:00.1636536Z   local time: Sat Dec 21 23:45:00 UTC 2019
2019-12-21T23:45:00.3132323Z   network time: Sat, 21 Dec 2019 23:45:00 GMT
2019-12-21T23:45:01.0447398Z == end clock drift check ==
2019-12-21T23:45:01.6958084Z 
2019-12-21T23:45:01.7022412Z ##[error]Bash exited with code '1'.
2019-12-21T23:45:01.7051415Z ##[section]Starting: Checkout
2019-12-21T23:45:01.7053125Z ==============================================================================
2019-12-21T23:45:01.7053202Z Task         : Get sources
2019-12-21T23:45:01.7053253Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
