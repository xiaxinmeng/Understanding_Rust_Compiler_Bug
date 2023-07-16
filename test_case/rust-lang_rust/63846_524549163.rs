plain
2019-08-24T12:51:57.1221988Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-24T12:51:57.1399453Z ##[command]git config gc.auto 0
2019-08-24T12:51:57.1464076Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-24T12:51:57.1509608Z ##[command]git config --get-all http.proxy
2019-08-24T12:51:57.1641420Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63846/merge:refs/remotes/pull/63846/merge
---
2019-08-24T12:52:31.0446729Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-24T12:52:31.0447472Z 
2019-08-24T12:52:31.0448371Z   git checkout -b <new-branch-name>
2019-08-24T12:52:31.0449091Z 
2019-08-24T12:52:31.0449751Z HEAD is now at 82cb65f20 Merge 33788c252c83aeea913482b283e33ccc41bd2860 into 478464570e60523adc6d303577d1782229ca1f93
2019-08-24T12:52:31.0606853Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-24T12:52:31.0609903Z ==============================================================================
2019-08-24T12:52:31.0609968Z Task         : Bash
2019-08-24T12:52:31.0610006Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-24T12:58:46.6459252Z    Compiling serde_json v1.0.40
2019-08-24T12:58:48.4377930Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-24T12:58:59.0741505Z     Finished release [optimized] target(s) in 1m 28s
2019-08-24T12:58:59.0829511Z tidy check
2019-08-24T12:58:59.5547503Z tidy error: /checkout/src/libstd/time.rs:68: line longer than 100 chars
2019-08-24T12:58:59.5547608Z tidy error: /checkout/src/libstd/time.rs:70: line longer than 100 chars
2019-08-24T12:58:59.5547651Z tidy error: /checkout/src/libstd/time.rs:71: line longer than 100 chars
2019-08-24T12:58:59.5547721Z tidy error: /checkout/src/libstd/time.rs:72: line longer than 100 chars
2019-08-24T12:58:59.5547947Z tidy error: /checkout/src/libstd/time.rs:73: line longer than 100 chars
2019-08-24T12:58:59.5548001Z tidy error: /checkout/src/libstd/time.rs:138: line longer than 100 chars
2019-08-24T12:58:59.5548043Z tidy error: /checkout/src/libstd/time.rs:140: line longer than 100 chars
2019-08-24T12:58:59.5548101Z tidy error: /checkout/src/libstd/time.rs:141: line longer than 100 chars
2019-08-24T12:58:59.5548143Z tidy error: /checkout/src/libstd/time.rs:142: line longer than 100 chars
2019-08-24T12:58:59.5548186Z tidy error: /checkout/src/libstd/time.rs:143: line longer than 100 chars
2019-08-24T12:59:00.8841969Z some tidy checks failed
2019-08-24T12:59:00.8847740Z 
2019-08-24T12:59:00.8847740Z 
2019-08-24T12:59:00.8848727Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-24T12:59:00.8848913Z 
2019-08-24T12:59:00.8848936Z 
2019-08-24T12:59:00.8858986Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-24T12:59:00.8859294Z Build completed unsuccessfully in 0:01:31
2019-08-24T12:59:00.8859294Z Build completed unsuccessfully in 0:01:31
2019-08-24T12:59:00.8909161Z == clock drift check ==
2019-08-24T12:59:00.8925528Z   local time: Sat Aug 24 12:59:00 UTC 2019
2019-08-24T12:59:01.0022819Z   network time: Sat, 24 Aug 2019 12:59:00 GMT
2019-08-24T12:59:01.0026523Z == end clock drift check ==
2019-08-24T12:59:02.4028366Z ##[error]Bash exited with code '1'.
2019-08-24T12:59:02.4083182Z ##[section]Starting: Checkout
2019-08-24T12:59:02.4085223Z ==============================================================================
2019-08-24T12:59:02.4085292Z Task         : Get sources
2019-08-24T12:59:02.4085478Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
