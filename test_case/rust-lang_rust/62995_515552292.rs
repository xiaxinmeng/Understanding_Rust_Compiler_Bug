plain
2019-07-26T18:00:55.8877045Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-26T18:00:55.9066143Z ##[command]git config gc.auto 0
2019-07-26T18:00:55.9131181Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-26T18:00:55.9183606Z ##[command]git config --get-all http.proxy
2019-07-26T18:00:55.9301734Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62995/merge:refs/remotes/pull/62995/merge
---
2019-07-26T18:01:31.0071628Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T18:01:31.0071651Z 
2019-07-26T18:01:31.0071798Z   git checkout -b <new-branch-name>
2019-07-26T18:01:31.0071821Z 
2019-07-26T18:01:31.0071883Z HEAD is now at be6518834 Merge ae9f3d5f36a308fdc65ab5272271641bda37a6f2 into 1a563362865e6051d4c350544131228e8eff5138
2019-07-26T18:01:31.0194754Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T18:01:31.0196930Z ==============================================================================
2019-07-26T18:01:31.0197087Z Task         : Bash
2019-07-26T18:01:31.0197122Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T18:07:26.3026243Z    Compiling serde_json v1.0.40
2019-07-26T18:07:30.1521992Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-26T18:07:37.8908994Z     Finished release [optimized] target(s) in 1m 20s
2019-07-26T18:07:37.8970618Z tidy check
2019-07-26T18:07:38.2817428Z tidy error: /checkout/src/test/ui/parser/issue-62973.rs: ignoring trailing newlines unnecessarily
2019-07-26T18:07:39.5229324Z some tidy checks failed
2019-07-26T18:07:39.5229801Z 
2019-07-26T18:07:39.5229801Z 
2019-07-26T18:07:39.5230655Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-26T18:07:39.5231037Z 
2019-07-26T18:07:39.5231071Z 
2019-07-26T18:07:39.5237324Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-26T18:07:39.5237690Z Build completed unsuccessfully in 0:01:23
2019-07-26T18:07:39.5237690Z Build completed unsuccessfully in 0:01:23
2019-07-26T18:07:40.9192275Z ##[error]Bash exited with code '1'.
2019-07-26T18:07:40.9222147Z ##[section]Starting: Checkout
2019-07-26T18:07:40.9224578Z ==============================================================================
2019-07-26T18:07:40.9224638Z Task         : Get sources
2019-07-26T18:07:40.9224689Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
