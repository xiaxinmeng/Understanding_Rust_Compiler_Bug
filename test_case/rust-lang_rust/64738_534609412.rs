plain
2019-09-24T15:02:54.9108811Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-24T15:02:54.9380464Z ##[command]git config gc.auto 0
2019-09-24T15:02:54.9447037Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-24T15:02:54.9496457Z ##[command]git config --get-all http.proxy
2019-09-24T15:02:54.9651994Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64738/merge:refs/remotes/pull/64738/merge
---
2019-09-24T15:09:45.2012823Z    Compiling serde_json v1.0.40
2019-09-24T15:09:47.0923147Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-24T15:09:58.4645205Z     Finished release [optimized] target(s) in 1m 34s
2019-09-24T15:09:58.4747795Z tidy check
2019-09-24T15:09:58.8923400Z tidy error: /checkout/src/librustc_mir/interpret/intrinsics.rs:251: line longer than 100 chars
2019-09-24T15:10:00.5879242Z some tidy checks failed
2019-09-24T15:10:00.5885809Z 
2019-09-24T15:10:00.5885809Z 
2019-09-24T15:10:00.5887716Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-24T15:10:00.5888359Z 
2019-09-24T15:10:00.5888385Z 
2019-09-24T15:10:00.5894812Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-24T15:10:00.5894923Z Build completed unsuccessfully in 0:01:37
2019-09-24T15:10:00.5894923Z Build completed unsuccessfully in 0:01:37
2019-09-24T15:10:00.5947984Z == clock drift check ==
2019-09-24T15:10:00.5963246Z   local time: Tue Sep 24 15:10:00 UTC 2019
2019-09-24T15:10:00.6700520Z   network time: Tue, 24 Sep 2019 15:10:00 GMT
2019-09-24T15:10:00.6700664Z == end clock drift check ==
2019-09-24T15:10:01.9889240Z ##[error]Bash exited with code '1'.
2019-09-24T15:10:01.9930366Z ##[section]Starting: Checkout
2019-09-24T15:10:01.9932461Z ==============================================================================
2019-09-24T15:10:01.9932545Z Task         : Get sources
2019-09-24T15:10:01.9932602Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
