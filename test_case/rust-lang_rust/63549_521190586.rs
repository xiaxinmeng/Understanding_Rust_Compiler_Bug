plain
2019-08-14T10:16:25.4532991Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-14T10:16:25.4734541Z ##[command]git config gc.auto 0
2019-08-14T10:16:25.4819082Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-14T10:16:25.4878229Z ##[command]git config --get-all http.proxy
2019-08-14T10:16:25.5054098Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63549/merge:refs/remotes/pull/63549/merge
---
2019-08-14T10:17:02.0725521Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-14T10:17:02.0727003Z 
2019-08-14T10:17:02.0728666Z   git checkout -b <new-branch-name>
2019-08-14T10:17:02.0731864Z 
2019-08-14T10:17:02.0733164Z HEAD is now at b6d4373f6 Merge 13a9780405ca13055709f1c4b6f3f3b8ea59797e into 60960a260f7b5c695fd0717311d72ce62dd4eb43
2019-08-14T10:17:02.0885201Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-14T10:17:02.0888167Z ==============================================================================
2019-08-14T10:17:02.0888247Z Task         : Bash
2019-08-14T10:17:02.0888315Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-14T10:22:11.5997057Z    |
2019-08-14T10:22:11.5997351Z 76 | #![feature(const_generics)]
2019-08-14T10:22:11.5997636Z    |            ^^^^^^^^^^^^^^
2019-08-14T10:22:11.5997835Z 
2019-08-14T10:22:18.8894387Z error[E0277]: expected a `ops::function::FnMut<(<I as iter::traits::iterator::Iterator>::Item,)>` closure, found `P`
2019-08-14T10:22:18.8894821Z   --> src/libcore/iter/adapters/mod.rs:75:19
2019-08-14T10:22:18.8895075Z    |
2019-08-14T10:22:18.8895426Z 75 |         self.iter.position(predicate).map(|i| self.len() - i - 1)
2019-08-14T10:22:18.8895876Z    |                   ^^^^^^^^ expected an `FnMut<(<I as iter::traits::iterator::Iterator>::Item,)>` closure, found `P`
2019-08-14T10:22:18.8896236Z    |
2019-08-14T10:22:18.8896600Z    = help: the trait `ops::function::FnMut<(<I as iter::traits::iterator::Iterator>::Item,)>` is not implemented for `P`
2019-08-14T10:22:18.8896990Z    = help: consider adding a `where P: ops::function::FnMut<(<I as iter::traits::iterator::Iterator>::Item,)>` bound
2019-08-14T10:22:20.1958521Z    Compiling libc v0.2.60
2019-08-14T10:22:21.1328707Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-08-14T10:22:22.6217713Z error: aborting due to previous error
2019-08-14T10:22:22.6222163Z 
2019-08-14T10:22:22.6222163Z 
2019-08-14T10:22:22.6228698Z For more information about this error, try `rustc --explain E0277`.
2019-08-14T10:22:22.7279512Z error: Could not compile `core`.
2019-08-14T10:22:22.7280017Z warning: build failed, waiting for other jobs to finish...
2019-08-14T10:22:22.7819760Z error: build failed
2019-08-14T10:22:22.7846070Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-08-14T10:22:22.7859154Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-14T10:22:22.7859358Z Build completed unsuccessfully in 0:02:27
2019-08-14T10:22:22.7859358Z Build completed unsuccessfully in 0:02:27
2019-08-14T10:22:35.2755568Z ##[error]Bash exited with code '1'.
2019-08-14T10:22:35.2796622Z ##[section]Starting: Checkout
2019-08-14T10:22:35.2798339Z ==============================================================================
2019-08-14T10:22:35.2798395Z Task         : Get sources
2019-08-14T10:22:35.2798463Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
