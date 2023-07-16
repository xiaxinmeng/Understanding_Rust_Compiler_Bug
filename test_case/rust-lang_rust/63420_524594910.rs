plain
2019-08-25T02:08:57.1202079Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-25T02:08:57.1401687Z ##[command]git config gc.auto 0
2019-08-25T02:08:57.1802020Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-25T02:08:57.1846523Z ##[command]git config --get-all http.proxy
2019-08-25T02:08:57.1981991Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63420/merge:refs/remotes/pull/63420/merge
---
2019-08-25T02:09:30.6797069Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-25T02:09:30.6797156Z 
2019-08-25T02:09:30.6797815Z   git checkout -b <new-branch-name>
2019-08-25T02:09:30.6798026Z 
2019-08-25T02:09:30.6798965Z HEAD is now at 9c27ad743 Merge c00b63ef7d0f1071fa45125be811bf5eff8dbf31 into eeba189cfb2cfc5c5898513352d4ca8f1df06e05
2019-08-25T02:09:30.6965415Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-25T02:09:30.6969016Z ==============================================================================
2019-08-25T02:09:30.6969096Z Task         : Bash
2019-08-25T02:09:30.6969143Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-25T02:15:58.2892038Z    Compiling serde_json v1.0.40
2019-08-25T02:16:00.0940173Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-25T02:16:10.6460728Z     Finished release [optimized] target(s) in 1m 31s
2019-08-25T02:16:10.6536383Z tidy check
2019-08-25T02:16:11.0589436Z tidy error: /checkout/src/librustc_mir/build/matches/mod.rs:945: line longer than 100 chars
2019-08-25T02:16:12.4558505Z some tidy checks failed
2019-08-25T02:16:12.4562061Z 
2019-08-25T02:16:12.4562061Z 
2019-08-25T02:16:12.4563135Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-25T02:16:12.4563304Z 
2019-08-25T02:16:12.4563347Z 
2019-08-25T02:16:12.4572079Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-25T02:16:12.4572378Z Build completed unsuccessfully in 0:01:34
2019-08-25T02:16:12.4572378Z Build completed unsuccessfully in 0:01:34
2019-08-25T02:16:12.4618704Z == clock drift check ==
2019-08-25T02:16:12.4633467Z   local time: Sun Aug 25 02:16:12 UTC 2019
2019-08-25T02:16:12.6103418Z   network time: Sun, 25 Aug 2019 02:16:12 GMT
2019-08-25T02:16:12.6106297Z == end clock drift check ==
2019-08-25T02:16:13.8967050Z ##[error]Bash exited with code '1'.
2019-08-25T02:16:13.9025836Z ##[section]Starting: Checkout
2019-08-25T02:16:13.9027455Z ==============================================================================
2019-08-25T02:16:13.9027520Z Task         : Get sources
2019-08-25T02:16:13.9027569Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
