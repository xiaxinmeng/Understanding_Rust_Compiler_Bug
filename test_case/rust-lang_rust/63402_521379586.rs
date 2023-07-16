plain
2019-08-14T18:46:38.8915932Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-14T18:46:38.9088465Z ##[command]git config gc.auto 0
2019-08-14T18:46:38.9168523Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-14T18:46:38.9223348Z ##[command]git config --get-all http.proxy
2019-08-14T18:46:38.9361009Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63402/merge:refs/remotes/pull/63402/merge
---
2019-08-14T18:47:13.8191975Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-14T18:47:13.8192017Z 
2019-08-14T18:47:13.8192198Z   git checkout -b <new-branch-name>
2019-08-14T18:47:13.8192223Z 
2019-08-14T18:47:13.8192279Z HEAD is now at 763d00b0e Merge 53546269a437ccbad26998a1aa65f25f3c82eeec into c43d03a19f326f4a323569328cc501e86eb6d22e
2019-08-14T18:47:13.8338886Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-14T18:47:13.8341425Z ==============================================================================
2019-08-14T18:47:13.8341471Z Task         : Bash
2019-08-14T18:47:13.8341508Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-14T18:53:24.9357813Z    Compiling serde_json v1.0.40
2019-08-14T18:53:29.1395700Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-14T18:53:37.5472219Z     Finished release [optimized] target(s) in 1m 27s
2019-08-14T18:53:37.5544276Z tidy check
2019-08-14T18:53:37.9728312Z tidy error: /checkout/src/librustc_errors/emitter.rs:1262: TODO is deprecated; use FIXME
2019-08-14T18:53:39.3798763Z some tidy checks failed
2019-08-14T18:53:39.3805735Z 
2019-08-14T18:53:39.3805735Z 
2019-08-14T18:53:39.3807575Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-14T18:53:39.3808190Z 
2019-08-14T18:53:39.3808280Z 
2019-08-14T18:53:39.3809400Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-14T18:53:39.3809628Z Build completed unsuccessfully in 0:01:30
2019-08-14T18:53:39.3809628Z Build completed unsuccessfully in 0:01:30
2019-08-14T18:53:39.3852232Z == clock drift check ==
2019-08-14T18:53:39.3863337Z   local time: Wed Aug 14 18:53:39 UTC 2019
2019-08-14T18:53:39.4708365Z   network time: Wed, 14 Aug 2019 18:53:39 GMT
2019-08-14T18:53:39.4710878Z == end clock drift check ==
2019-08-14T18:53:40.8283656Z ##[error]Bash exited with code '1'.
2019-08-14T18:53:40.8312378Z ##[section]Starting: Checkout
2019-08-14T18:53:40.8314105Z ==============================================================================
2019-08-14T18:53:40.8314164Z Task         : Get sources
2019-08-14T18:53:40.8314313Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
