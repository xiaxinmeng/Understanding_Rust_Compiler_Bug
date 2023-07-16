plain
2019-07-29T19:18:06.8877353Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-29T19:18:06.9064318Z ##[command]git config gc.auto 0
2019-07-29T19:18:06.9097695Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-29T19:18:06.9163533Z ##[command]git config --get-all http.proxy
2019-07-29T19:18:06.9301344Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63111/merge:refs/remotes/pull/63111/merge
---
2019-07-29T19:18:41.7307290Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-29T19:18:41.7307338Z 
2019-07-29T19:18:41.7307550Z   git checkout -b <new-branch-name>
2019-07-29T19:18:41.7307579Z 
2019-07-29T19:18:41.7307641Z HEAD is now at ab8f7b48f Merge 3bad4fdb4f1e3b868746ee40572fd30c6047d3d7 into 04b88a9eba8abbac87eddcb2998beea09589c2c9
2019-07-29T19:18:41.7467522Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-29T19:18:41.7471719Z ==============================================================================
2019-07-29T19:18:41.7471973Z Task         : Bash
2019-07-29T19:18:41.7472021Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-29T19:24:50.8017028Z    Compiling serde_json v1.0.40
2019-07-29T19:24:55.4045104Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-29T19:25:04.4517908Z     Finished release [optimized] target(s) in 1m 34s
2019-07-29T19:25:04.4586851Z tidy check
2019-07-29T19:25:05.0413251Z tidy error: /checkout/src/test/ui/pattern/rest-pat-semantic-disallowed.rs:2: trailing whitespace
2019-07-29T19:25:06.3778347Z some tidy checks failed
2019-07-29T19:25:06.3787367Z 
2019-07-29T19:25:06.3787367Z 
2019-07-29T19:25:06.3792381Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-29T19:25:06.3792561Z 
2019-07-29T19:25:06.3792587Z 
2019-07-29T19:25:06.3800720Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-29T19:25:06.3801637Z Build completed unsuccessfully in 0:01:37
2019-07-29T19:25:06.3801637Z Build completed unsuccessfully in 0:01:37
2019-07-29T19:25:07.6819292Z ##[error]Bash exited with code '1'.
2019-07-29T19:25:07.6852563Z ##[section]Starting: Checkout
2019-07-29T19:25:07.6854756Z ==============================================================================
2019-07-29T19:25:07.6854811Z Task         : Get sources
2019-07-29T19:25:07.6854859Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
