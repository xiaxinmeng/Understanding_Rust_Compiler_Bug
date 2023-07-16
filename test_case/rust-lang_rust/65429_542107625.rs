plain
2019-10-15T08:28:11.5604721Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-15T08:28:11.5724344Z ##[command]git config gc.auto 0
2019-10-15T08:28:11.5810020Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-15T08:28:11.5870043Z ##[command]git config --get-all http.proxy
2019-10-15T08:28:11.6059155Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65429/merge:refs/remotes/pull/65429/merge
---
2019-10-15T08:34:23.2059710Z    Compiling serde_json v1.0.40
2019-10-15T08:34:25.1265778Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-15T08:34:37.2886500Z     Finished release [optimized] target(s) in 1m 36s
2019-10-15T08:34:37.2968154Z tidy check
2019-10-15T08:34:38.1699769Z tidy error: /checkout/src/libstd/fs.rs:91: trailing whitespace
2019-10-15T08:34:39.5902392Z Found 482 error codes
2019-10-15T08:34:39.5903596Z Found 0 error codes with no tests
2019-10-15T08:34:39.5903783Z Done!
2019-10-15T08:34:39.5904096Z some tidy checks failed
2019-10-15T08:34:39.5904096Z some tidy checks failed
2019-10-15T08:34:39.5904613Z 
2019-10-15T08:34:39.5904937Z 
2019-10-15T08:34:39.5906027Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-15T08:34:39.5906270Z 
2019-10-15T08:34:39.5907959Z 
2019-10-15T08:34:39.5919897Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-15T08:34:39.5919991Z Build completed unsuccessfully in 0:01:40
2019-10-15T08:34:39.5919991Z Build completed unsuccessfully in 0:01:40
2019-10-15T08:34:39.5970569Z == clock drift check ==
2019-10-15T08:34:39.5984693Z   local time: Tue Oct 15 08:34:39 UTC 2019
2019-10-15T08:34:39.6830788Z   network time: Tue, 15 Oct 2019 08:34:39 GMT
2019-10-15T08:34:39.6833620Z == end clock drift check ==
2019-10-15T08:34:40.4275201Z ##[error]Bash exited with code '1'.
2019-10-15T08:34:40.4322106Z ##[section]Starting: Checkout
2019-10-15T08:34:40.4324664Z ==============================================================================
2019-10-15T08:34:40.4324738Z Task         : Get sources
2019-10-15T08:34:40.4324782Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
