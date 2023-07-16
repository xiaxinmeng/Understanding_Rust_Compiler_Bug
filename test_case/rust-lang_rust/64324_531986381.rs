plain
2019-09-16T22:32:32.4667522Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-16T22:32:32.4893791Z ##[command]git config gc.auto 0
2019-09-16T22:32:32.4986037Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-16T22:32:32.5051859Z ##[command]git config --get-all http.proxy
2019-09-16T22:32:32.5214602Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64324/merge:refs/remotes/pull/64324/merge
---
2019-09-16T22:39:39.0139018Z    Compiling serde_json v1.0.40
2019-09-16T22:39:40.9211621Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-16T22:39:52.4291484Z     Finished release [optimized] target(s) in 1m 34s
2019-09-16T22:39:52.4386062Z tidy check
2019-09-16T22:39:52.8600087Z tidy error: /checkout/src/librustc/query/mod.rs:643: line longer than 100 chars
2019-09-16T22:39:52.9781087Z tidy error: /checkout/src/librustc_metadata/dependency_format.rs: too many trailing newlines (2)
2019-09-16T22:39:54.5964035Z some tidy checks failed
2019-09-16T22:39:54.5966633Z 
2019-09-16T22:39:54.5966633Z 
2019-09-16T22:39:54.5968298Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-16T22:39:54.5969376Z 
2019-09-16T22:39:54.5969522Z 
2019-09-16T22:39:54.5983957Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-16T22:39:54.5984425Z Build completed unsuccessfully in 0:01:37
2019-09-16T22:39:54.5984425Z Build completed unsuccessfully in 0:01:37
2019-09-16T22:39:54.6047011Z == clock drift check ==
2019-09-16T22:39:54.6072612Z   local time: Mon Sep 16 22:39:54 UTC 2019
2019-09-16T22:39:54.7845988Z   network time: Mon, 16 Sep 2019 22:39:54 GMT
2019-09-16T22:39:54.7850430Z == end clock drift check ==
2019-09-16T22:39:56.1348663Z ##[error]Bash exited with code '1'.
2019-09-16T22:39:56.1409259Z ##[section]Starting: Checkout
2019-09-16T22:39:56.1411407Z ==============================================================================
2019-09-16T22:39:56.1411466Z Task         : Get sources
2019-09-16T22:39:56.1411535Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
