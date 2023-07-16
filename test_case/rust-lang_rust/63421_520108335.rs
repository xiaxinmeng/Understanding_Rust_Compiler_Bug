plain
2019-08-10T01:42:08.8915341Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-10T01:42:08.9109700Z ##[command]git config gc.auto 0
2019-08-10T01:42:08.9171168Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-10T01:42:08.9227519Z ##[command]git config --get-all http.proxy
2019-08-10T01:42:08.9363528Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63421/merge:refs/remotes/pull/63421/merge
---
2019-08-10T01:42:44.1283417Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-10T01:42:44.1283457Z 
2019-08-10T01:42:44.1283614Z   git checkout -b <new-branch-name>
2019-08-10T01:42:44.1283636Z 
2019-08-10T01:42:44.1283691Z HEAD is now at 0a9add3b3 Merge 455c5f7ac4701177c9e76730e4548b5558ac6a18 into 0ff76ad8dd90a6beae0018e773936727e5ad5d2a
2019-08-10T01:42:44.1446696Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-10T01:42:44.1450789Z ==============================================================================
2019-08-10T01:42:44.1450874Z Task         : Bash
2019-08-10T01:42:44.1450923Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-10T01:47:45.2578217Z    Compiling libc v0.2.60
2019-08-10T01:47:46.1983037Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-08-10T01:47:47.8840316Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-08-10T01:47:48.3998297Z    Compiling compiler_builtins v0.1.18
2019-08-10T01:47:50.6312376Z error[E0507]: cannot move out of `self.range` which is behind a shared reference
2019-08-10T01:47:50.6314684Z     |
2019-08-10T01:47:50.6314684Z     |
2019-08-10T01:47:50.6315568Z 138 |         f.write_str(unsafe { from_utf8_unchecked(&self.data[self.range]) })
2019-08-10T01:47:50.6316256Z     |                                                             ^^^^^^^^^^ move occurs because `self.range` has type `ops::range::Range<usize>`, which does not implement the `Copy` trait
2019-08-10T01:47:50.9905724Z    Compiling backtrace-sys v0.1.30
2019-08-10T01:47:52.2726454Z    Compiling cmake v0.1.38
2019-08-10T01:47:53.0329346Z error: aborting due to previous error
2019-08-10T01:47:53.0329522Z 
2019-08-10T01:47:53.0329522Z 
2019-08-10T01:47:53.0329839Z For more information about this error, try `rustc --explain E0507`.
2019-08-10T01:47:53.1880818Z error: Could not compile `core`.
2019-08-10T01:47:53.1881218Z warning: build failed, waiting for other jobs to finish...
2019-08-10T01:47:54.5039442Z error: build failed
2019-08-10T01:47:54.5068497Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-08-10T01:47:54.5071154Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-10T01:47:54.5071215Z Build completed unsuccessfully in 0:02:25
2019-08-10T01:47:54.5071215Z Build completed unsuccessfully in 0:02:25
2019-08-10T01:48:06.3332007Z ##[error]Bash exited with code '1'.
2019-08-10T01:48:06.3367483Z ##[section]Starting: Checkout
2019-08-10T01:48:06.3369089Z ==============================================================================
2019-08-10T01:48:06.3369144Z Task         : Get sources
2019-08-10T01:48:06.3369209Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
