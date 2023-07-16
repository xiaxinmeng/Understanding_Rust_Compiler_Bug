plain
2019-07-19T23:17:42.2024623Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-19T23:17:42.2215085Z ##[command]git config gc.auto 0
2019-07-19T23:17:42.7956381Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-19T23:17:42.7964769Z ##[command]git config --get-all http.proxy
2019-07-19T23:17:42.7970852Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62815/merge:refs/remotes/pull/62815/merge
---
2019-07-19T23:18:16.4612444Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-19T23:18:16.4612667Z 
2019-07-19T23:18:16.4613048Z   git checkout -b <new-branch-name>
2019-07-19T23:18:16.4613255Z 
2019-07-19T23:18:16.4613413Z HEAD is now at d329f8e8d Merge 517e4eae88d519363a7f24f8e4412ee771dc5dea into e3cebcb3bd4ffaf86bb0cdfd2af5b7e698717b01
2019-07-19T23:18:16.4679903Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-19T23:18:16.4682703Z ==============================================================================
2019-07-19T23:18:16.4682762Z Task         : Bash
2019-07-19T23:18:16.4682810Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-19T23:24:03.7699259Z    Compiling serde_json v1.0.40
2019-07-19T23:24:07.7236527Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-19T23:24:15.6471360Z     Finished release [optimized] target(s) in 1m 24s
2019-07-19T23:24:15.6547003Z tidy check
2019-07-19T23:24:15.9994773Z tidy error: /checkout/src/librustc/traits/project.rs:1392: line longer than 100 chars
2019-07-19T23:24:15.9994940Z tidy error: /checkout/src/librustc/traits/project.rs:1393: line longer than 100 chars
2019-07-19T23:24:16.0020253Z tidy error: /checkout/src/librustc/traits/project.rs:1395: line longer than 100 chars
2019-07-19T23:24:16.0020326Z tidy error: /checkout/src/librustc/traits/project.rs:1396: line longer than 100 chars
2019-07-19T23:24:16.0020382Z tidy error: /checkout/src/librustc/traits/coherence.rs:195: line longer than 100 chars
2019-07-19T23:24:17.3191050Z some tidy checks failed
2019-07-19T23:24:17.3196581Z 
2019-07-19T23:24:17.3196581Z 
2019-07-19T23:24:17.3197561Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-19T23:24:17.3198067Z 
2019-07-19T23:24:17.3198241Z 
2019-07-19T23:24:17.3202602Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-19T23:24:17.3202972Z Build completed unsuccessfully in 0:01:26
2019-07-19T23:24:17.3202972Z Build completed unsuccessfully in 0:01:26
2019-07-19T23:24:18.5970641Z ##[error]Bash exited with code '1'.
2019-07-19T23:24:18.6001807Z ##[section]Starting: Checkout
2019-07-19T23:24:18.6003634Z ==============================================================================
2019-07-19T23:24:18.6003680Z Task         : Get sources
2019-07-19T23:24:18.6003739Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
