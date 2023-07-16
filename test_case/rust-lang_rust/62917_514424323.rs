plain
2019-07-23T23:30:01.6311737Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-23T23:30:01.6583795Z ##[command]git config gc.auto 0
2019-07-23T23:30:01.6633164Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-23T23:30:01.6703975Z ##[command]git config --get-all http.proxy
2019-07-23T23:30:01.6850876Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62917/merge:refs/remotes/pull/62917/merge
---
2019-07-23T23:30:39.5268378Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-23T23:30:39.5269257Z 
2019-07-23T23:30:39.5270868Z   git checkout -b <new-branch-name>
2019-07-23T23:30:39.5272612Z 
2019-07-23T23:30:39.5273376Z HEAD is now at 4c5bc0c7c Merge 3e9bba4651a7eba729c0683bdd582d1955310d1d into 299ef86e1f8b3e53154f834115752c719b611fa1
2019-07-23T23:30:39.5455488Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-23T23:30:39.5457958Z ==============================================================================
2019-07-23T23:30:39.5458011Z Task         : Bash
2019-07-23T23:30:39.5458080Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-23T23:36:52.6650144Z    Compiling serde_json v1.0.40
2019-07-23T23:36:56.7901927Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-23T23:37:04.9833563Z     Finished release [optimized] target(s) in 1m 25s
2019-07-23T23:37:04.9899012Z tidy check
2019-07-23T23:37:06.1040268Z tidy error: /checkout/src/test/ui/parser/issue-62913.rs: too many trailing newlines (2)
2019-07-23T23:37:06.6771955Z some tidy checks failed
2019-07-23T23:37:06.6773077Z 
2019-07-23T23:37:06.6773077Z 
2019-07-23T23:37:06.6774258Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-23T23:37:06.6774374Z 
2019-07-23T23:37:06.6774415Z 
2019-07-23T23:37:06.6784700Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-23T23:37:06.6784756Z Build completed unsuccessfully in 0:01:28
2019-07-23T23:37:06.6784756Z Build completed unsuccessfully in 0:01:28
2019-07-23T23:37:08.0237698Z ##[error]Bash exited with code '1'.
2019-07-23T23:37:08.0271223Z ##[section]Starting: Checkout
2019-07-23T23:37:08.0272872Z ==============================================================================
2019-07-23T23:37:08.0272947Z Task         : Get sources
2019-07-23T23:37:08.0272996Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
