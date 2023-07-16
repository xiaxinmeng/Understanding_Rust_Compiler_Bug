plain
2019-09-12T23:22:37.5426510Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-12T23:22:37.5667876Z ##[command]git config gc.auto 0
2019-09-12T23:22:38.1760408Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-12T23:22:38.1766873Z ##[command]git config --get-all http.proxy
2019-09-12T23:22:38.1771161Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64158/merge:refs/remotes/pull/64158/merge
---
2019-09-12T23:29:34.8745065Z    Compiling serde_json v1.0.40
2019-09-12T23:29:36.4584743Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-12T23:29:46.1863717Z     Finished release [optimized] target(s) in 1m 20s
2019-09-12T23:29:46.1940663Z tidy check
2019-09-12T23:29:46.4781621Z tidy error: /checkout/src/libtest/lib.rs:1507: TODO is deprecated; use FIXME
2019-09-12T23:29:47.8970038Z some tidy checks failed
2019-09-12T23:29:47.8970203Z 
2019-09-12T23:29:47.8970203Z 
2019-09-12T23:29:47.8972115Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-12T23:29:47.8972245Z 
2019-09-12T23:29:47.8972292Z 
2019-09-12T23:29:47.8979805Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-12T23:29:47.8980064Z Build completed unsuccessfully in 0:01:23
2019-09-12T23:29:47.8980064Z Build completed unsuccessfully in 0:01:23
2019-09-12T23:29:47.9027934Z == clock drift check ==
2019-09-12T23:29:47.9045956Z   local time: Thu Sep 12 23:29:47 UTC 2019
2019-09-12T23:29:47.9886575Z   network time: Thu, 12 Sep 2019 23:29:47 GMT
2019-09-12T23:29:47.9886669Z == end clock drift check ==
2019-09-12T23:29:49.3256280Z ##[error]Bash exited with code '1'.
2019-09-12T23:29:49.3298921Z ##[section]Starting: Checkout
2019-09-12T23:29:49.3300462Z ==============================================================================
2019-09-12T23:29:49.3300508Z Task         : Get sources
2019-09-12T23:29:49.3300543Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
