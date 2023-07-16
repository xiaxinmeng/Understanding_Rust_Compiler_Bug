plain
2019-08-12T23:53:16.4280624Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-12T23:53:16.4479176Z ##[command]git config gc.auto 0
2019-08-12T23:53:16.4540035Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-12T23:53:16.4590193Z ##[command]git config --get-all http.proxy
2019-08-12T23:53:16.4723820Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63470/merge:refs/remotes/pull/63470/merge
---
2019-08-12T23:53:53.4139249Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-12T23:53:53.4139296Z 
2019-08-12T23:53:53.4139496Z   git checkout -b <new-branch-name>
2019-08-12T23:53:53.4139524Z 
2019-08-12T23:53:53.4139569Z HEAD is now at 1a9aa9925 Merge beab7fb5aff44c0d2f79407c3efd489fec64a802 into 60960a260f7b5c695fd0717311d72ce62dd4eb43
2019-08-12T23:53:53.4294082Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-12T23:53:53.4296936Z ==============================================================================
2019-08-12T23:53:53.4296995Z Task         : Bash
2019-08-12T23:53:53.4297044Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-12T23:59:24.4589361Z     Checking backtrace v0.3.34
2019-08-12T23:59:26.4508250Z     Checking rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-08-12T23:59:26.4512106Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-08-12T23:59:26.6072567Z     Checking hashbrown v0.4.0
2019-08-12T23:59:28.4944861Z error: use of item 'core::try' that will be deprecated in future version 1.39.0: use the `?` operator instead
2019-08-12T23:59:28.4946216Z     |
2019-08-12T23:59:28.4946216Z     |
2019-08-12T23:59:28.4946854Z 524 |     r#try,
2019-08-12T23:59:28.4947732Z     |
2019-08-12T23:59:28.4948199Z     = note: `-D deprecated-in-future` implied by `-D warnings`
2019-08-12T23:59:28.4948367Z 
2019-08-12T23:59:30.3433869Z error: aborting due to previous error
2019-08-12T23:59:30.3433869Z error: aborting due to previous error
2019-08-12T23:59:30.3434560Z 
2019-08-12T23:59:30.3771646Z error: Could not compile `std`.
2019-08-12T23:59:30.3771725Z 
2019-08-12T23:59:30.3772235Z To learn more, run the command again with --verbose.
2019-08-12T23:59:30.3794089Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-12T23:59:30.3800655Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-12T23:59:30.3800939Z Build completed unsuccessfully in 0:02:52
2019-08-12T23:59:30.3800939Z Build completed unsuccessfully in 0:02:52
2019-08-12T23:59:36.9017102Z ##[error]Bash exited with code '1'.
2019-08-12T23:59:36.9058225Z ##[section]Starting: Checkout
2019-08-12T23:59:36.9060323Z ==============================================================================
2019-08-12T23:59:36.9060402Z Task         : Get sources
2019-08-12T23:59:36.9060452Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
