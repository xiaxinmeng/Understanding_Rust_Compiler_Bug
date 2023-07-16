plain
2019-10-10T16:28:33.4438383Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-10T16:28:33.4517674Z ##[command]git config gc.auto 0
2019-10-10T16:28:33.4601217Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-10T16:28:33.4648115Z ##[command]git config --get-all http.proxy
2019-10-10T16:28:33.4790482Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65277/merge:refs/remotes/pull/65277/merge
---
2019-10-10T16:34:13.3735806Z    Compiling serde_json v1.0.40
2019-10-10T16:34:15.0074872Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-10T16:34:25.6712968Z     Finished release [optimized] target(s) in 1m 21s
2019-10-10T16:34:25.6779758Z tidy check
2019-10-10T16:34:26.0872168Z tidy error: /checkout/src/librustc_mir/borrow_check/error_reporting.rs:815: line longer than 100 chars
2019-10-10T16:34:27.7483235Z Found 482 error codes
2019-10-10T16:34:27.7483931Z Found 0 error codes with no tests
2019-10-10T16:34:27.7484133Z Done!
2019-10-10T16:34:27.7484242Z some tidy checks failed
2019-10-10T16:34:27.7484242Z some tidy checks failed
2019-10-10T16:34:27.7484338Z 
2019-10-10T16:34:27.7484449Z 
2019-10-10T16:34:27.7485272Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-10T16:34:27.7485866Z 
2019-10-10T16:34:27.7485962Z 
2019-10-10T16:34:27.7489388Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-10T16:34:27.7489658Z Build completed unsuccessfully in 0:01:24
2019-10-10T16:34:27.7489658Z Build completed unsuccessfully in 0:01:24
2019-10-10T16:34:27.7531676Z == clock drift check ==
2019-10-10T16:34:27.7548221Z   local time: Thu Oct 10 16:34:27 UTC 2019
2019-10-10T16:34:27.7908252Z   network time: Thu, 10 Oct 2019 16:34:27 GMT
2019-10-10T16:34:27.7912806Z == end clock drift check ==
2019-10-10T16:34:28.6195249Z ##[error]Bash exited with code '1'.
2019-10-10T16:34:28.6232912Z ##[section]Starting: Checkout
2019-10-10T16:34:28.6234341Z ==============================================================================
2019-10-10T16:34:28.6234401Z Task         : Get sources
2019-10-10T16:34:28.6234435Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
