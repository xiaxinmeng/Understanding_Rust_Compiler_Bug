plain
2019-08-18T21:34:33.7366212Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-18T21:34:33.7540134Z ##[command]git config gc.auto 0
2019-08-18T21:34:33.7608657Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-18T21:34:33.7671089Z ##[command]git config --get-all http.proxy
2019-08-18T21:34:33.7796510Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63549/merge:refs/remotes/pull/63549/merge
---
2019-08-18T21:35:07.6108416Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-18T21:35:07.6109185Z 
2019-08-18T21:35:07.6142439Z   git checkout -b <new-branch-name>
2019-08-18T21:35:07.6143228Z 
2019-08-18T21:35:07.6143921Z HEAD is now at f03a3178f Merge 95af12fbef099a48ce475f21736b9717d4bcdeaf into ea52be482ab4945fda63cb65b6a198309a041e3c
2019-08-18T21:35:07.6249765Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-18T21:35:07.6257525Z ==============================================================================
2019-08-18T21:35:07.6257588Z Task         : Bash
2019-08-18T21:35:07.6257655Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-18T21:40:32.5545805Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-18T21:40:32.5564803Z Checking std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-18T21:40:32.9895223Z    Compiling cc v1.0.35
2019-08-18T21:40:32.9895641Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-08-18T21:40:40.0784821Z error[E0277]: expected a `ops::function::FnMut<(<I as iter::traits::iterator::Iterator>::Item,)>` closure, found `P`
2019-08-18T21:40:40.0785255Z   --> src/libcore/iter/adapters/mod.rs:75:34
2019-08-18T21:40:40.0785526Z    |
2019-08-18T21:40:40.0785826Z 75 |         let position = self.iter.position(predicate);
2019-08-18T21:40:40.0786277Z    |                                  ^^^^^^^^ expected an `FnMut<(<I as iter::traits::iterator::Iterator>::Item,)>` closure, found `P`
2019-08-18T21:40:40.0786516Z    |
2019-08-18T21:40:40.0787127Z    = help: the trait `ops::function::FnMut<(<I as iter::traits::iterator::Iterator>::Item,)>` is not implemented for `P`
2019-08-18T21:40:40.0787553Z    = help: consider adding a `where P: ops::function::FnMut<(<I as iter::traits::iterator::Iterator>::Item,)>` bound
2019-08-18T21:40:41.4248884Z    Compiling libc v0.2.60
2019-08-18T21:40:42.2795519Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-08-18T21:40:43.4118688Z error: aborting due to previous error
2019-08-18T21:40:43.4118804Z 
2019-08-18T21:40:43.4118804Z 
2019-08-18T21:40:43.4124076Z For more information about this error, try `rustc --explain E0277`.
2019-08-18T21:40:43.5127440Z error: Could not compile `core`.
2019-08-18T21:40:43.5141506Z warning: build failed, waiting for other jobs to finish...
2019-08-18T21:40:43.6838749Z error: build failed
2019-08-18T21:40:43.6866435Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-18T21:40:43.6875633Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-18T21:40:43.6875703Z Build completed unsuccessfully in 0:02:40
2019-08-18T21:40:43.6924119Z == clock drift check ==
2019-08-18T21:40:43.6945638Z   local time: Sun Aug 18 21:40:43 UTC 2019
2019-08-18T21:40:43.6945638Z   local time: Sun Aug 18 21:40:43 UTC 2019
2019-08-18T21:40:43.8419126Z   network time: Sun, 18 Aug 2019 21:40:43 GMT
2019-08-18T21:40:43.8422598Z == end clock drift check ==
2019-08-18T21:40:58.1565212Z ##[error]Bash exited with code '1'.
2019-08-18T21:40:58.1597729Z ##[section]Starting: Checkout
2019-08-18T21:40:58.1599406Z ==============================================================================
2019-08-18T21:40:58.1599463Z Task         : Get sources
2019-08-18T21:40:58.1599510Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
