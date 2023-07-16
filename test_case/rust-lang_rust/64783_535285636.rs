plain
2019-09-26T00:49:31.6386958Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-26T00:49:31.6600046Z ##[command]git config gc.auto 0
2019-09-26T00:49:31.6696122Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-26T00:49:31.6750524Z ##[command]git config --get-all http.proxy
2019-09-26T00:49:31.6917684Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64783/merge:refs/remotes/pull/64783/merge
---
2019-09-26T00:56:43.5665815Z    Compiling serde_json v1.0.40
2019-09-26T00:56:45.5053779Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-26T00:56:57.2447643Z     Finished release [optimized] target(s) in 1m 34s
2019-09-26T00:56:57.2555653Z tidy check
2019-09-26T00:56:57.5010705Z tidy error: /checkout/src/test/ui/issues/issue-64732.rs:5: trailing whitespace
2019-09-26T00:56:57.5011104Z tidy error: /checkout/src/test/ui/issues/issue-64732.rs:6: trailing whitespace
2019-09-26T00:56:59.3624785Z some tidy checks failed
2019-09-26T00:56:59.3632067Z 
2019-09-26T00:56:59.3632067Z 
2019-09-26T00:56:59.3633212Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-26T00:56:59.3633748Z 
2019-09-26T00:56:59.3633952Z 
2019-09-26T00:56:59.3638577Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-26T00:56:59.3638857Z Build completed unsuccessfully in 0:01:37
2019-09-26T00:56:59.3638857Z Build completed unsuccessfully in 0:01:37
2019-09-26T00:56:59.3690747Z == clock drift check ==
2019-09-26T00:56:59.3703246Z   local time: Thu Sep 26 00:56:59 UTC 2019
2019-09-26T00:56:59.5207204Z   network time: Thu, 26 Sep 2019 00:56:59 GMT
2019-09-26T00:56:59.5209669Z == end clock drift check ==
2019-09-26T00:57:00.9631833Z ##[error]Bash exited with code '1'.
2019-09-26T00:57:00.9670991Z ##[section]Starting: Checkout
2019-09-26T00:57:00.9672886Z ==============================================================================
2019-09-26T00:57:00.9672942Z Task         : Get sources
2019-09-26T00:57:00.9673004Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
