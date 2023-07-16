plain
2019-08-08T15:57:39.9368911Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-08T15:57:39.9577311Z ##[command]git config gc.auto 0
2019-08-08T15:57:39.9648485Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-08T15:57:40.9595075Z ##[command]git config --get-all http.proxy
2019-08-08T15:57:40.9606566Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63382/merge:refs/remotes/pull/63382/merge
---
2019-08-08T15:58:14.9143344Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-08T15:58:14.9143396Z 
2019-08-08T15:58:14.9143628Z   git checkout -b <new-branch-name>
2019-08-08T15:58:14.9143660Z 
2019-08-08T15:58:14.9143713Z HEAD is now at 011ee935b Merge 009fdd727f4b3fd300ba1c8685e18f6dcba2472a into 2628f579f6246df385acf9203bf2ffb6aedf5ccc
2019-08-08T15:58:14.9306117Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-08T15:58:14.9308785Z ==============================================================================
2019-08-08T15:58:14.9308836Z Task         : Bash
2019-08-08T15:58:14.9308878Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-08T16:03:39.9382291Z     Checking rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-08-08T16:03:40.2047969Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-08-08T16:03:40.3609667Z     Checking hashbrown v0.4.0
2019-08-08T16:03:42.2546949Z     Checking backtrace v0.3.34
2019-08-08T16:03:47.2349354Z error[E0711]: feature `rust1` is declared stable since 1.38.0, but was previously declared stable since 1.0.0
2019-08-08T16:03:47.2351781Z     |
2019-08-08T16:03:47.2352436Z 910 | #[stable(feature = "rust1", since = "1.38.0")]
2019-08-08T16:03:47.2353275Z     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-08T16:03:47.2353601Z 
2019-08-08T16:03:47.2353601Z 
2019-08-08T16:03:47.3796378Z error: aborting due to previous error
2019-08-08T16:03:47.3797285Z 
2019-08-08T16:03:47.4292550Z error: Could not compile `std`.
2019-08-08T16:03:47.4293347Z 
2019-08-08T16:03:47.4293905Z To learn more, run the command again with --verbose.
2019-08-08T16:03:47.4318052Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-08-08T16:03:47.4329330Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-08T16:03:47.4329713Z Build completed unsuccessfully in 0:02:35
2019-08-08T16:03:47.4329713Z Build completed unsuccessfully in 0:02:35
2019-08-08T16:03:50.1707095Z ##[error]Bash exited with code '1'.
2019-08-08T16:03:50.1742290Z ##[section]Starting: Checkout
2019-08-08T16:03:50.1744909Z ==============================================================================
2019-08-08T16:03:50.1745009Z Task         : Get sources
2019-08-08T16:03:50.1745059Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
