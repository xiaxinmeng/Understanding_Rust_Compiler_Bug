plain
2019-07-19T16:01:23.0227842Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-19T16:01:23.5853600Z ##[command]git config gc.auto 0
2019-07-19T16:01:23.5859588Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-19T16:01:23.5863417Z ##[command]git config --get-all http.proxy
2019-07-19T16:01:23.5865821Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62655/merge:refs/remotes/pull/62655/merge
---
2019-07-19T16:02:00.8828207Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-19T16:02:00.8829039Z 
2019-07-19T16:02:00.8829572Z   git checkout -b <new-branch-name>
2019-07-19T16:02:00.8829753Z 
2019-07-19T16:02:00.8829908Z HEAD is now at c037fc79f Merge f6fb25d874ba9216982e5e75927b42b577c1531d into 527dce7137f7a3c7bf47d9a503abf25f88ea22de
2019-07-19T16:02:00.8977607Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-19T16:02:00.8980385Z ==============================================================================
2019-07-19T16:02:00.8980437Z Task         : Bash
2019-07-19T16:02:00.8980478Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-19T16:08:02.8011446Z    Compiling serde_json v1.0.40
2019-07-19T16:08:07.0247045Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-19T16:08:15.5356342Z     Finished release [optimized] target(s) in 1m 27s
2019-07-19T16:08:15.5424253Z tidy check
2019-07-19T16:08:15.8167576Z tidy error: /checkout/src/librustc_mir/interpret/intern.rs:100: line longer than 100 chars
2019-07-19T16:08:15.8183442Z tidy error: /checkout/src/librustc_mir/interpret/memory.rs:625: line longer than 100 chars
2019-07-19T16:08:15.8971030Z tidy error: /checkout/src/librustc/mir/interpret/mod.rs:20: line longer than 100 chars
2019-07-19T16:08:15.9208467Z tidy error: /checkout/src/librustc_codegen_llvm/consts.rs:34: line longer than 100 chars
2019-07-19T16:08:17.3170311Z some tidy checks failed
2019-07-19T16:08:17.3174260Z 
2019-07-19T16:08:17.3174260Z 
2019-07-19T16:08:17.3175480Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-19T16:08:17.3179995Z 
2019-07-19T16:08:17.3180232Z 
2019-07-19T16:08:17.3185012Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-19T16:08:17.3185082Z Build completed unsuccessfully in 0:01:30
2019-07-19T16:08:17.3185082Z Build completed unsuccessfully in 0:01:30
2019-07-19T16:08:19.0633375Z ##[error]Bash exited with code '1'.
2019-07-19T16:08:19.0665092Z ##[section]Starting: Checkout
2019-07-19T16:08:19.0666601Z ==============================================================================
2019-07-19T16:08:19.0666653Z Task         : Get sources
2019-07-19T16:08:19.0666712Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
