plain
2019-08-26T02:14:21.2411493Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-26T02:14:21.2610545Z ##[command]git config gc.auto 0
2019-08-26T02:14:21.2684906Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-26T02:14:21.2738554Z ##[command]git config --get-all http.proxy
2019-08-26T02:14:21.2872112Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63906/merge:refs/remotes/pull/63906/merge
---
2019-08-26T02:14:56.0302018Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-26T02:14:56.0302053Z 
2019-08-26T02:14:56.0302329Z   git checkout -b <new-branch-name>
2019-08-26T02:14:56.0302648Z 
2019-08-26T02:14:56.0302723Z HEAD is now at 608fcfb35 Merge 8bb577ff9e907400314029a87ebf49072ca1338f into 521d78407471cb78e9bbf47160f6aa23047ac499
2019-08-26T02:14:56.0489012Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-26T02:14:56.0492274Z ==============================================================================
2019-08-26T02:14:56.0492650Z Task         : Bash
2019-08-26T02:14:56.0492741Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-26T02:21:34.0035329Z    Compiling serde_json v1.0.40
2019-08-26T02:21:35.8555794Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-26T02:21:46.7754332Z     Finished release [optimized] target(s) in 1m 30s
2019-08-26T02:21:46.7836354Z tidy check
2019-08-26T02:21:46.9976993Z tidy error: /checkout/src/librustc_codegen_ssa/mir/rvalue.rs:589: trailing whitespace
2019-08-26T02:21:48.9713099Z some tidy checks failed
2019-08-26T02:21:48.9714239Z 
2019-08-26T02:21:48.9714239Z 
2019-08-26T02:21:48.9715247Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-26T02:21:48.9715422Z 
2019-08-26T02:21:48.9715449Z 
2019-08-26T02:21:48.9715782Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-26T02:21:48.9715849Z Build completed unsuccessfully in 0:01:33
2019-08-26T02:21:48.9715849Z Build completed unsuccessfully in 0:01:33
2019-08-26T02:21:48.9759982Z == clock drift check ==
2019-08-26T02:21:48.9775282Z   local time: Mon Aug 26 02:21:48 UTC 2019
2019-08-26T02:21:49.1331822Z   network time: Mon, 26 Aug 2019 02:21:49 GMT
2019-08-26T02:21:49.1335991Z == end clock drift check ==
2019-08-26T02:21:50.4824763Z ##[error]Bash exited with code '1'.
2019-08-26T02:21:50.4858164Z ##[section]Starting: Checkout
2019-08-26T02:21:50.4860250Z ==============================================================================
2019-08-26T02:21:50.4860314Z Task         : Get sources
2019-08-26T02:21:50.4860368Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
