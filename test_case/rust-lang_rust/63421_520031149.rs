plain
2019-08-09T18:59:42.5412469Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-09T18:59:42.5631488Z ##[command]git config gc.auto 0
2019-08-09T18:59:42.5698262Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-09T18:59:42.5776993Z ##[command]git config --get-all http.proxy
2019-08-09T18:59:42.5918637Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63421/merge:refs/remotes/pull/63421/merge
---
2019-08-09T19:00:18.4811620Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-09T19:00:18.4811691Z 
2019-08-09T19:00:18.4812292Z   git checkout -b <new-branch-name>
2019-08-09T19:00:18.4812477Z 
2019-08-09T19:00:18.4812854Z HEAD is now at 24512b598 Merge 1f0f9ddb282e0ed85c9fdc0d9b430ac23bdde17d into 534b42394d743511db1335d5ed08d507ab7c6e73
2019-08-09T19:00:18.4985119Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-09T19:00:18.4988527Z ==============================================================================
2019-08-09T19:00:18.4988589Z Task         : Bash
2019-08-09T19:00:18.4988635Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-09T19:05:40.4444429Z 
2019-08-09T19:05:47.4228738Z error[E0308]: mismatched types
2019-08-09T19:05:47.4232307Z    --> src/libcore/ascii.rs:137:21
2019-08-09T19:05:47.4233548Z     |
2019-08-09T19:05:47.4234324Z 137 |         f.write_str(&self.data[self.range])
2019-08-09T19:05:47.4235029Z     |                     ^^^^^^^^^^^^^^^^^^^^^^ expected str, found slice
2019-08-09T19:05:47.4236304Z     = note: expected type `&str`
2019-08-09T19:05:47.4236881Z                found type `&[u8]`
2019-08-09T19:05:47.4237180Z 
2019-08-09T19:05:48.8019292Z    Compiling libc v0.2.60
---
2019-08-09T19:05:51.4964419Z For more information about this error, try `rustc --explain E0308`.
2019-08-09T19:05:51.5999250Z error: Could not compile `core`.
2019-08-09T19:05:51.6014465Z warning: build failed, waiting for other jobs to finish...
2019-08-09T19:05:51.8433692Z error: build failed
2019-08-09T19:05:51.8456625Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-08-09T19:05:51.8469446Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-09T19:05:51.8470055Z Build completed unsuccessfully in 0:02:19
2019-08-09T19:05:51.8470055Z Build completed unsuccessfully in 0:02:19
2019-08-09T19:06:06.0597178Z ##[error]Bash exited with code '1'.
2019-08-09T19:06:06.0650055Z ##[section]Starting: Checkout
2019-08-09T19:06:06.0652539Z ==============================================================================
2019-08-09T19:06:06.0652609Z Task         : Get sources
2019-08-09T19:06:06.0652687Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
