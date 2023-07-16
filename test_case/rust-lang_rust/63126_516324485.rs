plain
2019-07-30T06:16:06.0541722Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-30T06:16:06.0725490Z ##[command]git config gc.auto 0
2019-07-30T06:16:06.0890214Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-30T06:16:06.0920978Z ##[command]git config --get-all http.proxy
2019-07-30T06:16:06.1095298Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63126/merge:refs/remotes/pull/63126/merge
---
2019-07-30T06:16:40.7547757Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-30T06:16:40.7547793Z 
2019-07-30T06:16:40.7548024Z   git checkout -b <new-branch-name>
2019-07-30T06:16:40.7548057Z 
2019-07-30T06:16:40.7548128Z HEAD is now at 97e5c4738 Merge dc9d6b13a741b4ec8f41cb3dbe8656365fe6c465 into 04b88a9eba8abbac87eddcb2998beea09589c2c9
2019-07-30T06:16:40.7718048Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-30T06:16:40.7721828Z ==============================================================================
2019-07-30T06:16:40.7721903Z Task         : Bash
2019-07-30T06:16:40.7721960Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-30T06:23:11.8803561Z tidy check
2019-07-30T06:23:12.8568159Z * 578 error codes
2019-07-30T06:23:12.8568324Z * highest error code: E0733
2019-07-30T06:23:13.1998688Z * 261 features
2019-07-30T06:23:13.8378763Z crate `rustc-ap-syntax` is duplicated in `Cargo.lock`
2019-07-30T06:23:13.8379278Z   * rustc-ap-syntax 491.0.0 (registry+https://github.com/rust-lang/crates.io-index)
2019-07-30T06:23:13.8379584Z   * rustc-ap-syntax 542.0.0 (registry+https://github.com/rust-lang/crates.io-index)
2019-07-30T06:23:13.8889216Z some tidy checks failed
2019-07-30T06:23:13.8895140Z 
2019-07-30T06:23:13.8895140Z 
2019-07-30T06:23:13.8897001Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-30T06:23:13.8897161Z 
2019-07-30T06:23:13.8897206Z 
2019-07-30T06:23:13.8908599Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-30T06:23:13.8908840Z Build completed unsuccessfully in 0:01:37
2019-07-30T06:23:13.8908840Z Build completed unsuccessfully in 0:01:37
2019-07-30T06:23:15.3166290Z ##[error]Bash exited with code '1'.
2019-07-30T06:23:15.3201076Z ##[section]Starting: Checkout
2019-07-30T06:23:15.3203730Z ==============================================================================
2019-07-30T06:23:15.3203810Z Task         : Get sources
2019-07-30T06:23:15.3203863Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
