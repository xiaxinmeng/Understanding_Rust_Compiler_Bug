plain
2019-10-03T02:00:19.2247490Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-03T02:00:19.2266800Z ##[command]git config gc.auto 0
2019-10-03T02:00:19.2269932Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-03T02:00:19.2272022Z ##[command]git config --get-all http.proxy
2019-10-03T02:00:19.2279124Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65037/merge:refs/remotes/pull/65037/merge
---
2019-10-03T02:08:00.2250787Z tidy error: /checkout/src/test/ui/feature-gates/feature-gate-track_caller.rs: leading newline
2019-10-03T02:08:01.9870692Z some tidy checks failed
2019-10-03T02:08:01.9871894Z 
2019-10-03T02:08:01.9872727Z 
2019-10-03T02:08:01.9873861Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-03T02:08:01.9874807Z 
2019-10-03T02:08:01.9875225Z 
2019-10-03T02:08:01.9888327Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-03T02:08:01.9888415Z Build completed unsuccessfully in 0:01:38
2019-10-03T02:08:01.9888415Z Build completed unsuccessfully in 0:01:38
2019-10-03T02:08:01.9945533Z == clock drift check ==
2019-10-03T02:08:01.9963241Z   local time: Thu Oct  3 02:08:01 UTC 2019
2019-10-03T02:08:02.1463516Z   network time: Thu, 03 Oct 2019 02:08:02 GMT
2019-10-03T02:08:02.1473961Z == end clock drift check ==
2019-10-03T02:08:03.5050586Z ##[error]Bash exited with code '1'.
2019-10-03T02:08:03.5083123Z ##[section]Starting: Checkout
2019-10-03T02:08:03.5084809Z ==============================================================================
2019-10-03T02:08:03.5084861Z Task         : Get sources
2019-10-03T02:08:03.5084904Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
