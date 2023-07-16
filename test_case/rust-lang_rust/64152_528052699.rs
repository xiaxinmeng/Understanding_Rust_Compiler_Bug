plain
2019-09-04T19:16:04.3140385Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-04T19:16:04.3356311Z ##[command]git config gc.auto 0
2019-09-04T19:16:05.2379149Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-04T19:16:05.2383760Z ##[command]git config --get-all http.proxy
2019-09-04T19:16:05.2389305Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64152/merge:refs/remotes/pull/64152/merge
---
2019-09-04T19:23:01.8269607Z    Compiling serde_json v1.0.40
2019-09-04T19:23:03.6824768Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-04T19:23:14.9417052Z     Finished release [optimized] target(s) in 1m 31s
2019-09-04T19:23:14.9504758Z tidy check
2019-09-04T19:23:15.8066351Z tidy error: /checkout/src/libstd/sys_common/backtrace.rs:40: TODO is deprecated; use FIXME
2019-09-04T19:23:16.9466348Z some tidy checks failed
2019-09-04T19:23:16.9471447Z 
2019-09-04T19:23:16.9471447Z 
2019-09-04T19:23:16.9477319Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-04T19:23:16.9479168Z 
2019-09-04T19:23:16.9479250Z 
2019-09-04T19:23:16.9479310Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-04T19:23:16.9479365Z Build completed unsuccessfully in 0:01:35
2019-09-04T19:23:16.9479365Z Build completed unsuccessfully in 0:01:35
2019-09-04T19:23:16.9535964Z == clock drift check ==
2019-09-04T19:23:16.9551910Z   local time: Wed Sep  4 19:23:16 UTC 2019
2019-09-04T19:23:17.1116280Z   network time: Wed, 04 Sep 2019 19:23:17 GMT
2019-09-04T19:23:17.1116480Z == end clock drift check ==
2019-09-04T19:23:18.4239487Z ##[error]Bash exited with code '1'.
2019-09-04T19:23:18.4275643Z ##[section]Starting: Checkout
2019-09-04T19:23:18.4277250Z ==============================================================================
2019-09-04T19:23:18.4277295Z Task         : Get sources
2019-09-04T19:23:18.4277334Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
