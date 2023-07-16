plain
2019-08-04T02:18:19.6918989Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-04T02:18:19.7144326Z ##[command]git config gc.auto 0
2019-08-04T02:18:19.7208287Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-04T02:18:19.7260559Z ##[command]git config --get-all http.proxy
2019-08-04T02:18:19.7407501Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63175/merge:refs/remotes/pull/63175/merge
---
2019-08-04T02:18:54.1877542Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-04T02:18:54.1877572Z 
2019-08-04T02:18:54.1877798Z   git checkout -b <new-branch-name>
2019-08-04T02:18:54.1877828Z 
2019-08-04T02:18:54.1877895Z HEAD is now at f0cb37811 Merge e61425f7997ee2baae5168f73037777082ec42e3 into 6e0d27d9368e2982bef8e1c4ac14d622c5ad018e
2019-08-04T02:18:54.2040600Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-04T02:18:54.2043713Z ==============================================================================
2019-08-04T02:18:54.2043768Z Task         : Bash
2019-08-04T02:18:54.2043829Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-04T02:24:51.4032119Z    Compiling serde_json v1.0.40
2019-08-04T02:24:55.7816608Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-04T02:25:04.5700963Z     Finished release [optimized] target(s) in 1m 31s
2019-08-04T02:25:04.5785793Z tidy check
2019-08-04T02:25:04.6947025Z tidy error: /checkout/src/librustc_driver/args/mod.rs:114: line longer than 100 chars
2019-08-04T02:25:04.6947213Z tidy error: /checkout/src/librustc_driver/args/mod.rs: missing trailing newline
2019-08-04T02:25:06.6003353Z some tidy checks failed
2019-08-04T02:25:06.6010947Z 
2019-08-04T02:25:06.6010947Z 
2019-08-04T02:25:06.6011848Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-04T02:25:06.6012029Z 
2019-08-04T02:25:06.6012055Z 
2019-08-04T02:25:06.6018574Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-04T02:25:06.6018955Z Build completed unsuccessfully in 0:01:34
2019-08-04T02:25:06.6018955Z Build completed unsuccessfully in 0:01:34
2019-08-04T02:25:07.8807598Z ##[error]Bash exited with code '1'.
2019-08-04T02:25:07.8839786Z ##[section]Starting: Checkout
2019-08-04T02:25:07.8841522Z ==============================================================================
2019-08-04T02:25:07.8841577Z Task         : Get sources
2019-08-04T02:25:07.8841644Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
