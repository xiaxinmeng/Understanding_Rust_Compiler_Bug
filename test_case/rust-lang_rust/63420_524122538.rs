plain
2019-08-22T23:42:48.2891201Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-22T23:42:48.3084516Z ##[command]git config gc.auto 0
2019-08-22T23:42:48.3155185Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-22T23:42:48.3196808Z ##[command]git config --get-all http.proxy
2019-08-22T23:42:48.3324015Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63420/merge:refs/remotes/pull/63420/merge
---
2019-08-22T23:43:24.6785375Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-22T23:43:24.6786751Z 
2019-08-22T23:43:24.6788468Z   git checkout -b <new-branch-name>
2019-08-22T23:43:24.6789899Z 
2019-08-22T23:43:24.6790781Z HEAD is now at 346dd96ac Merge 2da56cbc5b57429ac4594c6a0906db08d5b17fa3 into 760226733e940cb375f791e894fbb554555eeb01
2019-08-22T23:43:24.7047893Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-22T23:43:24.7050800Z ==============================================================================
2019-08-22T23:43:24.7050858Z Task         : Bash
2019-08-22T23:43:24.7050904Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-22T23:49:37.4165168Z    Compiling serde_json v1.0.40
2019-08-22T23:49:39.2187652Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-22T23:49:49.9211642Z     Finished release [optimized] target(s) in 1m 27s
2019-08-22T23:49:49.9292936Z tidy check
2019-08-22T23:49:50.3731249Z tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:811: line longer than 100 chars
2019-08-22T23:49:50.3752428Z tidy error: /checkout/src/librustc_mir/transform/uniform_array_move_out.rs:114: line longer than 100 chars
2019-08-22T23:49:51.8668968Z some tidy checks failed
2019-08-22T23:49:51.8671640Z 
2019-08-22T23:49:51.8671640Z 
2019-08-22T23:49:51.8672534Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-22T23:49:51.8672645Z 
2019-08-22T23:49:51.8672685Z 
2019-08-22T23:49:51.8688163Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-22T23:49:51.8688239Z Build completed unsuccessfully in 0:01:30
2019-08-22T23:49:51.8688239Z Build completed unsuccessfully in 0:01:30
2019-08-22T23:49:51.8731912Z == clock drift check ==
2019-08-22T23:49:51.8744477Z   local time: Thu Aug 22 23:49:51 UTC 2019
2019-08-22T23:49:52.1597712Z   network time: Thu, 22 Aug 2019 23:49:52 GMT
2019-08-22T23:49:52.1601581Z == end clock drift check ==
2019-08-22T23:49:54.1716898Z ##[error]Bash exited with code '1'.
2019-08-22T23:49:54.1746073Z ##[section]Starting: Checkout
2019-08-22T23:49:54.1748095Z ==============================================================================
2019-08-22T23:49:54.1748158Z Task         : Get sources
2019-08-22T23:49:54.1748200Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
