plain
2019-08-24T21:53:30.0597471Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-24T21:53:30.0774997Z ##[command]git config gc.auto 0
2019-08-24T21:53:30.0849673Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-24T21:53:30.0895615Z ##[command]git config --get-all http.proxy
2019-08-24T21:53:30.1029946Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63870/merge:refs/remotes/pull/63870/merge
---
2019-08-24T21:54:04.1222102Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-24T21:54:04.1222131Z 
2019-08-24T21:54:04.1222348Z   git checkout -b <new-branch-name>
2019-08-24T21:54:04.1222376Z 
2019-08-24T21:54:04.1222417Z HEAD is now at 1095016d0 Merge 928e528c01e0af6bbd4d93c7ae7ad5393dcb8cc1 into eeba189cfb2cfc5c5898513352d4ca8f1df06e05
2019-08-24T21:54:04.1394487Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-24T21:54:04.1398312Z ==============================================================================
2019-08-24T21:54:04.1398453Z Task         : Bash
2019-08-24T21:54:04.1398499Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-24T22:00:16.8431799Z    Compiling serde_json v1.0.40
2019-08-24T22:00:18.6543902Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-24T22:00:29.4268398Z     Finished release [optimized] target(s) in 1m 29s
2019-08-24T22:00:29.4346842Z tidy check
2019-08-24T22:00:29.9599541Z tidy error: /checkout/src/test/ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs:10: trailing whitespace
2019-08-24T22:00:29.9600961Z tidy error: /checkout/src/test/ui/suggestions/fn-ctor-passed-as-arg-where-it-should-have-been-called.rs:18: trailing whitespace
2019-08-24T22:00:31.2342646Z some tidy checks failed
2019-08-24T22:00:31.2343060Z 
2019-08-24T22:00:31.2343060Z 
2019-08-24T22:00:31.2350158Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-24T22:00:31.2350286Z 
2019-08-24T22:00:31.2350340Z 
2019-08-24T22:00:31.2359005Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-24T22:00:31.2359067Z Build completed unsuccessfully in 0:01:31
2019-08-24T22:00:31.2359067Z Build completed unsuccessfully in 0:01:31
2019-08-24T22:00:31.2409910Z == clock drift check ==
2019-08-24T22:00:31.2423386Z   local time: Sat Aug 24 22:00:31 UTC 2019
2019-08-24T22:00:31.3896474Z   network time: Sat, 24 Aug 2019 22:00:31 GMT
2019-08-24T22:00:31.3900001Z == end clock drift check ==
2019-08-24T22:00:32.7329915Z ##[error]Bash exited with code '1'.
2019-08-24T22:00:32.7361477Z ##[section]Starting: Checkout
2019-08-24T22:00:32.7363579Z ==============================================================================
2019-08-24T22:00:32.7363637Z Task         : Get sources
2019-08-24T22:00:32.7363686Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
