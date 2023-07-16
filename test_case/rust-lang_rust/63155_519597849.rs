plain
2019-08-08T16:26:42.3482806Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-08T16:26:43.2791345Z ##[command]git config gc.auto 0
2019-08-08T16:26:43.2794566Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-08T16:26:43.2796853Z ##[command]git config --get-all http.proxy
2019-08-08T16:26:43.2800014Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63155/merge:refs/remotes/pull/63155/merge
---
2019-08-08T16:27:18.1040438Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-08T16:27:18.1040469Z 
2019-08-08T16:27:18.1040695Z   git checkout -b <new-branch-name>
2019-08-08T16:27:18.1040741Z 
2019-08-08T16:27:18.1040790Z HEAD is now at 7c65382c0 Merge 89044a908ed602ae3dee74905866cffd63c164b3 into 2628f579f6246df385acf9203bf2ffb6aedf5ccc
2019-08-08T16:27:18.1205828Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-08T16:27:18.1208734Z ==============================================================================
2019-08-08T16:27:18.1208807Z Task         : Bash
2019-08-08T16:27:18.1208854Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-08T16:33:16.0921697Z    Compiling serde_json v1.0.40
2019-08-08T16:33:20.5132276Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-08T16:33:29.1084788Z     Finished release [optimized] target(s) in 1m 29s
2019-08-08T16:33:29.1146799Z tidy check
2019-08-08T16:33:29.7964285Z tidy error: /checkout/src/librustc_codegen_ssa/back/link.rs:1043: trailing whitespace
2019-08-08T16:33:31.1537551Z some tidy checks failed
2019-08-08T16:33:31.1544921Z 
2019-08-08T16:33:31.1544921Z 
2019-08-08T16:33:31.1546332Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-08T16:33:31.1546469Z 
2019-08-08T16:33:31.1546495Z 
2019-08-08T16:33:31.1558006Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-08T16:33:31.1558091Z Build completed unsuccessfully in 0:01:32
2019-08-08T16:33:31.1558091Z Build completed unsuccessfully in 0:01:32
2019-08-08T16:33:32.6128003Z ##[error]Bash exited with code '1'.
2019-08-08T16:33:32.6162939Z ##[section]Starting: Checkout
2019-08-08T16:33:32.6164743Z ==============================================================================
2019-08-08T16:33:32.6164941Z Task         : Get sources
2019-08-08T16:33:32.6164985Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
