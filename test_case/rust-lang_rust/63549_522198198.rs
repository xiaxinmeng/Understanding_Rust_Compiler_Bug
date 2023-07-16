plain
2019-08-17T02:42:13.0820721Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-17T02:42:13.1018617Z ##[command]git config gc.auto 0
2019-08-17T02:42:13.1095921Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-17T02:42:13.1147346Z ##[command]git config --get-all http.proxy
2019-08-17T02:42:13.1288128Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63549/merge:refs/remotes/pull/63549/merge
---
2019-08-17T02:42:47.6986991Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-17T02:42:47.6987912Z 
2019-08-17T02:42:47.6989139Z   git checkout -b <new-branch-name>
2019-08-17T02:42:47.6990016Z 
2019-08-17T02:42:47.6990888Z HEAD is now at c4a33fc39 Merge d3fbe6ddc9bc4406a1b193d2757320c0c79fbac5 into bdfd698f37184da42254a03ed466ab1f90e6fb6c
2019-08-17T02:42:47.7126432Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-17T02:42:47.7129489Z ==============================================================================
2019-08-17T02:42:47.7129554Z Task         : Bash
2019-08-17T02:42:47.7129606Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-17T02:48:03.3790769Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-17T02:48:03.3810151Z Checking std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-17T02:48:03.8021679Z    Compiling cc v1.0.35
2019-08-17T02:48:03.8022094Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-08-17T02:48:10.8679175Z error[E0277]: expected a `ops::function::FnMut<(<I as iter::traits::iterator::Iterator>::Item,)>` closure, found `P`
2019-08-17T02:48:10.8680516Z   --> src/libcore/iter/adapters/mod.rs:75:19
2019-08-17T02:48:10.8681187Z    |
2019-08-17T02:48:10.8682263Z 75 |         self.iter.position(predicate).map(|_| {
2019-08-17T02:48:10.8682774Z    |                   ^^^^^^^^ expected an `FnMut<(<I as iter::traits::iterator::Iterator>::Item,)>` closure, found `P`
2019-08-17T02:48:10.8683026Z    |
2019-08-17T02:48:10.8683771Z    = help: the trait `ops::function::FnMut<(<I as iter::traits::iterator::Iterator>::Item,)>` is not implemented for `P`
2019-08-17T02:48:10.8685118Z    = help: consider adding a `where P: ops::function::FnMut<(<I as iter::traits::iterator::Iterator>::Item,)>` bound
2019-08-17T02:48:12.7228616Z    Compiling libc v0.2.60
2019-08-17T02:48:13.6205335Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-08-17T02:48:14.3152407Z error: aborting due to previous error
2019-08-17T02:48:14.3157323Z 
2019-08-17T02:48:14.3157323Z 
2019-08-17T02:48:14.3163276Z For more information about this error, try `rustc --explain E0277`.
2019-08-17T02:48:14.4048176Z error: Could not compile `core`.
2019-08-17T02:48:14.4136963Z warning: build failed, waiting for other jobs to finish...
2019-08-17T02:48:14.8438941Z error: build failed
2019-08-17T02:48:14.8516318Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-17T02:48:14.8519039Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-17T02:48:14.8519330Z Build completed unsuccessfully in 0:02:39
2019-08-17T02:48:14.8586956Z == clock drift check ==
2019-08-17T02:48:14.8612578Z   local time: Sat Aug 17 02:48:14 UTC 2019
2019-08-17T02:48:14.8612578Z   local time: Sat Aug 17 02:48:14 UTC 2019
2019-08-17T02:48:15.0154764Z   network time: Sat, 17 Aug 2019 02:48:15 GMT
2019-08-17T02:48:15.0158994Z == end clock drift check ==
2019-08-17T02:48:28.1361534Z ##[error]Bash exited with code '1'.
2019-08-17T02:48:28.1399079Z ##[section]Starting: Checkout
2019-08-17T02:48:28.1401403Z ==============================================================================
2019-08-17T02:48:28.1401486Z Task         : Get sources
2019-08-17T02:48:28.1401539Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
