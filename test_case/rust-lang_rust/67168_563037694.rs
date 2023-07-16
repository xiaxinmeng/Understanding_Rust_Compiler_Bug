plain
2019-12-09T02:20:21.3051152Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-09T02:20:21.3240025Z ##[command]git config gc.auto 0
2019-12-09T02:20:21.3326478Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-09T02:20:21.3394871Z ##[command]git config --get-all http.proxy
2019-12-09T02:20:21.3556323Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67168/merge:refs/remotes/pull/67168/merge
---
2019-12-09T02:26:19.9334826Z    Compiling serde_json v1.0.40
2019-12-09T02:26:21.6126511Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-09T02:26:32.3803100Z     Finished release [optimized] target(s) in 1m 25s
2019-12-09T02:26:32.3903853Z tidy check
2019-12-09T02:26:33.1316426Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0478.md:24: line longer than 80 chars
2019-12-09T02:26:35.0773596Z some tidy checks failed
2019-12-09T02:26:35.0773676Z Found 485 error codes
2019-12-09T02:26:35.0773899Z Found 0 error codes with no tests
2019-12-09T02:26:35.0773976Z Done!
2019-12-09T02:26:35.0773976Z Done!
2019-12-09T02:26:35.0783597Z 
2019-12-09T02:26:35.0784468Z 
2019-12-09T02:26:35.0785354Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-09T02:26:35.0785487Z 
2019-12-09T02:26:35.0785508Z 
2019-12-09T02:26:35.0837238Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-09T02:26:35.0837382Z Build completed unsuccessfully in 0:01:28
2019-12-09T02:26:35.0837382Z Build completed unsuccessfully in 0:01:28
2019-12-09T02:26:35.0844223Z == clock drift check ==
2019-12-09T02:26:35.0857677Z   local time: Mon Dec  9 02:26:35 UTC 2019
2019-12-09T02:26:35.3663840Z   network time: Mon, 09 Dec 2019 02:26:35 GMT
2019-12-09T02:26:35.3664017Z == end clock drift check ==
2019-12-09T02:26:36.7879734Z 
2019-12-09T02:26:36.7962555Z ##[error]Bash exited with code '1'.
2019-12-09T02:26:36.7990179Z ##[section]Starting: Checkout
2019-12-09T02:26:36.7992325Z ==============================================================================
2019-12-09T02:26:36.7992392Z Task         : Get sources
2019-12-09T02:26:36.7992431Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
