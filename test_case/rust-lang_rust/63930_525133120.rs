plain
2019-08-27T04:14:58.8198203Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-27T04:14:58.8379654Z ##[command]git config gc.auto 0
2019-08-27T04:14:58.8442599Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-27T04:14:58.8496211Z ##[command]git config --get-all http.proxy
2019-08-27T04:14:58.8644113Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63930/merge:refs/remotes/pull/63930/merge
---
2019-08-27T04:15:32.8438226Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-27T04:15:32.8439521Z 
2019-08-27T04:15:32.8440330Z   git checkout -b <new-branch-name>
2019-08-27T04:15:32.8440952Z 
2019-08-27T04:15:32.8441481Z HEAD is now at f300a2f50 Merge 9f004b995372b17b4a9d0fef51017317664577fa into 0444b9f66acb5da23dc816e0d8eb59623ba9ea50
2019-08-27T04:15:32.8576237Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-27T04:15:32.8578520Z ==============================================================================
2019-08-27T04:15:32.8578580Z Task         : Bash
2019-08-27T04:15:32.8578614Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-27T04:21:18.8717840Z    Compiling serde_json v1.0.40
2019-08-27T04:21:20.3754517Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-27T04:21:29.9205205Z     Finished release [optimized] target(s) in 1m 18s
2019-08-27T04:21:29.9280459Z tidy check
2019-08-27T04:21:30.3781771Z tidy error: /checkout/src/test/rustdoc/through-proc-macro.rs: missing trailing newline
2019-08-27T04:21:31.6010515Z some tidy checks failed
2019-08-27T04:21:31.6010629Z 
2019-08-27T04:21:31.6010629Z 
2019-08-27T04:21:31.6012478Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-27T04:21:31.6012594Z 
2019-08-27T04:21:31.6012614Z 
2019-08-27T04:21:31.6021844Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-27T04:21:31.6021935Z Build completed unsuccessfully in 0:01:21
2019-08-27T04:21:31.6021935Z Build completed unsuccessfully in 0:01:21
2019-08-27T04:21:31.6067168Z == clock drift check ==
2019-08-27T04:21:31.6088372Z   local time: Tue Aug 27 04:21:31 UTC 2019
2019-08-27T04:21:31.7561973Z   network time: Tue, 27 Aug 2019 04:21:31 GMT
2019-08-27T04:21:31.7565252Z == end clock drift check ==
2019-08-27T04:21:33.0663113Z ##[error]Bash exited with code '1'.
2019-08-27T04:21:33.0691845Z ##[section]Starting: Checkout
2019-08-27T04:21:33.0693283Z ==============================================================================
2019-08-27T04:21:33.0693327Z Task         : Get sources
2019-08-27T04:21:33.0693381Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
