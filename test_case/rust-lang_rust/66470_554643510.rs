plain
2019-11-16T14:26:03.6514797Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-16T14:26:03.6721443Z ##[command]git config gc.auto 0
2019-11-16T14:26:03.6779676Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-16T14:26:03.6840262Z ##[command]git config --get-all http.proxy
2019-11-16T14:26:03.7009246Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66470/merge:refs/remotes/pull/66470/merge
---
2019-11-16T14:32:14.0279327Z Found 0 error codes with no tests
2019-11-16T14:32:14.0279602Z Done!
2019-11-16T14:32:14.0279889Z 
2019-11-16T14:32:14.0280104Z 
2019-11-16T14:32:14.0281279Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-16T14:32:14.0284006Z 
2019-11-16T14:32:14.0284343Z 
2019-11-16T14:32:14.0284678Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-16T14:32:14.0284965Z Build completed unsuccessfully in 0:01:31
2019-11-16T14:32:14.0284965Z Build completed unsuccessfully in 0:01:31
2019-11-16T14:32:14.0332483Z == clock drift check ==
2019-11-16T14:32:14.0342054Z   local time: Sat Nov 16 14:32:14 UTC 2019
2019-11-16T14:32:14.3111115Z   network time: Sat, 16 Nov 2019 14:32:14 GMT
2019-11-16T14:32:14.3115041Z == end clock drift check ==
2019-11-16T14:32:15.5873247Z 
2019-11-16T14:32:15.5967918Z ##[error]Bash exited with code '1'.
2019-11-16T14:32:15.5994450Z ##[section]Starting: Checkout
2019-11-16T14:32:15.5996110Z ==============================================================================
2019-11-16T14:32:15.5996165Z Task         : Get sources
2019-11-16T14:32:15.5996212Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
