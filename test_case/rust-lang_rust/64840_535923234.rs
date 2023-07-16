plain
2019-09-27T12:24:48.7281969Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-27T12:24:48.7468361Z ##[command]git config gc.auto 0
2019-09-27T12:24:48.7521931Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-27T12:24:48.7582616Z ##[command]git config --get-all http.proxy
2019-09-27T12:24:48.7716993Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64840/merge:refs/remotes/pull/64840/merge
---
2019-09-27T12:31:46.6472538Z    Compiling serde_json v1.0.40
2019-09-27T12:31:48.4473081Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-27T12:31:59.3912584Z     Finished release [optimized] target(s) in 1m 27s
2019-09-27T12:31:59.3998264Z tidy check
2019-09-27T12:32:00.2149914Z tidy error: /checkout/src/librustc_codegen_llvm/back/write.rs:528: line longer than 100 chars
2019-09-27T12:32:01.3504099Z some tidy checks failed
2019-09-27T12:32:01.3511357Z 
2019-09-27T12:32:01.3511357Z 
2019-09-27T12:32:01.3513305Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-27T12:32:01.3513496Z 
2019-09-27T12:32:01.3513525Z 
2019-09-27T12:32:01.3571530Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-27T12:32:01.3571623Z Build completed unsuccessfully in 0:01:31
2019-09-27T12:32:01.3571623Z Build completed unsuccessfully in 0:01:31
2019-09-27T12:32:01.3575675Z == clock drift check ==
2019-09-27T12:32:01.3592465Z   local time: Fri Sep 27 12:32:01 UTC 2019
2019-09-27T12:32:01.6359082Z   network time: Fri, 27 Sep 2019 12:32:01 GMT
2019-09-27T12:32:01.6359741Z == end clock drift check ==
2019-09-27T12:32:03.0196412Z ##[error]Bash exited with code '1'.
2019-09-27T12:32:03.0235204Z ##[section]Starting: Checkout
2019-09-27T12:32:03.0237563Z ==============================================================================
2019-09-27T12:32:03.0237623Z Task         : Get sources
2019-09-27T12:32:03.0237691Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
