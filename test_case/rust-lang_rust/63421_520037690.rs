plain
2019-08-09T19:23:58.5275920Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-09T19:23:58.5476337Z ##[command]git config gc.auto 0
2019-08-09T19:23:58.5535271Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-09T19:23:58.5580126Z ##[command]git config --get-all http.proxy
2019-08-09T19:23:58.5717280Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63421/merge:refs/remotes/pull/63421/merge
---
2019-08-09T19:24:33.3848789Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-09T19:24:33.3849043Z 
2019-08-09T19:24:33.3849475Z   git checkout -b <new-branch-name>
2019-08-09T19:24:33.3849744Z 
2019-08-09T19:24:33.3849966Z HEAD is now at ad4c97316 Merge a6b62904d00f5198cf05daec2fcafb947b508432 into 534b42394d743511db1335d5ed08d507ab7c6e73
2019-08-09T19:24:33.4001118Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-09T19:24:33.4004376Z ==============================================================================
2019-08-09T19:24:33.4004435Z Task         : Bash
2019-08-09T19:24:33.4004481Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-09T19:29:42.5162324Z    Compiling libc v0.2.60
2019-08-09T19:29:43.4098640Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-08-09T19:29:45.0138088Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-08-09T19:29:45.5439644Z    Compiling compiler_builtins v0.1.18
2019-08-09T19:29:46.7415110Z error[E0507]: cannot move out of `self.range` which is behind a shared reference
2019-08-09T19:29:46.7429340Z     |
2019-08-09T19:29:46.7429340Z     |
2019-08-09T19:29:46.7434634Z 138 |         f.write_str(from_utf8_unchecked(&self.data[self.range]))
2019-08-09T19:29:46.7440932Z     |                                                    ^^^^^^^^^^ move occurs because `self.range` has type `ops::range::Range<usize>`, which does not implement the `Copy` trait
2019-08-09T19:29:48.0051472Z    Compiling backtrace-sys v0.1.30
2019-08-09T19:29:49.2645349Z    Compiling cmake v0.1.38
2019-08-09T19:29:49.6024621Z error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
2019-08-09T19:29:49.6033460Z    --> src/libcore/ascii.rs:138:21
2019-08-09T19:29:49.6033460Z    --> src/libcore/ascii.rs:138:21
2019-08-09T19:29:49.6040126Z     |
2019-08-09T19:29:49.6045743Z 138 |         f.write_str(from_utf8_unchecked(&self.data[self.range]))
2019-08-09T19:29:49.6052213Z     |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
2019-08-09T19:29:49.6058594Z     |
2019-08-09T19:29:49.6102259Z     = note: consult the function's documentation for information on how to avoid undefined behavior
2019-08-09T19:29:49.6102595Z error: aborting due to 2 previous errors
2019-08-09T19:29:49.6102628Z 
2019-08-09T19:29:49.6102839Z Some errors have detailed explanations: E0133, E0507.
2019-08-09T19:29:49.6103098Z For more information about an error, try `rustc --explain E0133`.
2019-08-09T19:29:49.6103098Z For more information about an error, try `rustc --explain E0133`.
2019-08-09T19:29:49.7429145Z error: Could not compile `core`.
2019-08-09T19:29:49.7429490Z warning: build failed, waiting for other jobs to finish...
2019-08-09T19:29:51.2593741Z error: build failed
2019-08-09T19:29:51.2655264Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-08-09T19:29:51.2660638Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-09T19:29:51.2660731Z Build completed unsuccessfully in 0:02:18
2019-08-09T19:29:51.2660731Z Build completed unsuccessfully in 0:02:18
2019-08-09T19:30:05.1657197Z ##[error]Bash exited with code '1'.
2019-08-09T19:30:05.1693570Z ##[section]Starting: Checkout
2019-08-09T19:30:05.1696071Z ==============================================================================
2019-08-09T19:30:05.1696145Z Task         : Get sources
2019-08-09T19:30:05.1696189Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
