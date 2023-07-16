plain
2019-09-16T23:44:01.8214835Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-16T23:44:01.8403234Z ##[command]git config gc.auto 0
2019-09-16T23:44:01.8492316Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-16T23:44:01.8550018Z ##[command]git config --get-all http.proxy
2019-09-16T23:44:01.8701692Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64470/merge:refs/remotes/pull/64470/merge
---
2019-09-16T23:51:01.2745171Z    Compiling serde_json v1.0.40
2019-09-16T23:51:03.1496777Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-16T23:51:13.9276232Z     Finished release [optimized] target(s) in 1m 29s
2019-09-16T23:51:13.9375183Z tidy check
2019-09-16T23:51:14.5677693Z tidy error: /checkout/src/test/ui/consts/miri_unleashed/feature-gate-unleash_the_miri_inside_of_you.rs:19: line longer than 100 chars
2019-09-16T23:51:15.8316048Z some tidy checks failed
2019-09-16T23:51:15.8316829Z 
2019-09-16T23:51:15.8316829Z 
2019-09-16T23:51:15.8317963Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-16T23:51:15.8318710Z 
2019-09-16T23:51:15.8318876Z 
2019-09-16T23:51:15.8324228Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-16T23:51:15.8324599Z Build completed unsuccessfully in 0:01:32
2019-09-16T23:51:15.8324599Z Build completed unsuccessfully in 0:01:32
2019-09-16T23:51:15.8373866Z == clock drift check ==
2019-09-16T23:51:15.8387023Z   local time: Mon Sep 16 23:51:15 UTC 2019
2019-09-16T23:51:15.9874031Z   network time: Mon, 16 Sep 2019 23:51:15 GMT
2019-09-16T23:51:15.9876244Z == end clock drift check ==
2019-09-16T23:51:17.2706491Z ##[error]Bash exited with code '1'.
2019-09-16T23:51:17.2740107Z ##[section]Starting: Checkout
2019-09-16T23:51:17.2742775Z ==============================================================================
2019-09-16T23:51:17.2742837Z Task         : Get sources
2019-09-16T23:51:17.2742907Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
