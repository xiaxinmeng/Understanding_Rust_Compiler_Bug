plain
2019-07-25T18:29:48.2837431Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-25T18:29:48.3056997Z ##[command]git config gc.auto 0
2019-07-25T18:29:48.3148861Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-25T18:29:48.3202865Z ##[command]git config --get-all http.proxy
2019-07-25T18:29:48.3366475Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62963/merge:refs/remotes/pull/62963/merge
---
2019-07-25T18:30:24.2050070Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T18:30:24.2050397Z 
2019-07-25T18:30:24.2050909Z   git checkout -b <new-branch-name>
2019-07-25T18:30:24.2051199Z 
2019-07-25T18:30:24.2051738Z HEAD is now at 53ec64292 Merge cc5f7ab43a6e15ea3b5e89b33b5617a5eab80a8c into eedf6ce4ef54bb03818ab21d714f1b9f13a6b31c
2019-07-25T18:30:24.2209695Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-25T18:30:24.2212854Z ==============================================================================
2019-07-25T18:30:24.2212940Z Task         : Bash
2019-07-25T18:30:24.2212993Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-25T18:36:41.9905775Z    Compiling serde_json v1.0.40
2019-07-25T18:36:46.6904162Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-25T18:36:55.8132328Z     Finished release [optimized] target(s) in 1m 35s
2019-07-25T18:36:55.8213130Z tidy check
2019-07-25T18:36:56.0166798Z tidy error: /checkout/src/libsyntax/parse/lexer/mod.rs:395: line longer than 100 chars
2019-07-25T18:36:57.7874260Z some tidy checks failed
2019-07-25T18:36:57.7875636Z 
2019-07-25T18:36:57.7875636Z 
2019-07-25T18:36:57.7877047Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-25T18:36:57.7877719Z 
2019-07-25T18:36:57.7878017Z 
2019-07-25T18:36:57.7881934Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-25T18:36:57.7882306Z Build completed unsuccessfully in 0:01:39
2019-07-25T18:36:57.7882306Z Build completed unsuccessfully in 0:01:39
2019-07-25T18:36:59.0900763Z ##[error]Bash exited with code '1'.
2019-07-25T18:36:59.0939581Z ##[section]Starting: Checkout
2019-07-25T18:36:59.0942745Z ==============================================================================
2019-07-25T18:36:59.0942806Z Task         : Get sources
2019-07-25T18:36:59.0942852Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
