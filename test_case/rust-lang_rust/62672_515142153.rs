plain
2019-07-25T17:32:50.3865636Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-25T17:32:50.4055458Z ##[command]git config gc.auto 0
2019-07-25T17:32:50.4119196Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-25T17:32:50.4179305Z ##[command]git config --get-all http.proxy
2019-07-25T17:32:50.4315741Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62672/merge:refs/remotes/pull/62672/merge
---
2019-07-25T17:33:24.8269456Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T17:33:24.8269490Z 
2019-07-25T17:33:24.8269720Z   git checkout -b <new-branch-name>
2019-07-25T17:33:24.8269752Z 
2019-07-25T17:33:24.8269825Z HEAD is now at 206caebc5 Merge 4b4c75ba0672807274039e08bb408dc51c67ef4f into eedf6ce4ef54bb03818ab21d714f1b9f13a6b31c
2019-07-25T17:33:24.8432554Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-25T17:33:24.8435483Z ==============================================================================
2019-07-25T17:33:24.8435548Z Task         : Bash
2019-07-25T17:33:24.8435616Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-25T17:38:44.9062682Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2019-07-25T17:38:48.2423670Z     Checking rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-07-25T17:38:48.2940328Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-07-25T17:38:48.4474242Z     Checking hashbrown v0.4.0
2019-07-25T17:38:50.4058387Z error: use of item 'core::try' that will be deprecated in future version 1.39.0: use the `?` operator instead
2019-07-25T17:38:50.4060378Z     |
2019-07-25T17:38:50.4060378Z     |
2019-07-25T17:38:50.4061040Z 330 | pub use core::r#try;
2019-07-25T17:38:50.4062524Z     |
2019-07-25T17:38:50.4062524Z     |
2019-07-25T17:38:50.4063159Z     = note: `-D deprecated-in-future` implied by `-D warnings`
2019-07-25T17:38:52.2893107Z error: aborting due to previous error
2019-07-25T17:38:52.2893991Z 
2019-07-25T17:38:52.3251127Z error: Could not compile `std`.
2019-07-25T17:38:52.3251798Z 
2019-07-25T17:38:52.3251798Z 
2019-07-25T17:38:52.3252540Z To learn more, run the command again with --verbose.
2019-07-25T17:38:52.3290802Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-25T17:38:52.3301649Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-25T17:38:52.3302777Z Build completed unsuccessfully in 0:02:34
2019-07-25T17:38:52.3302777Z Build completed unsuccessfully in 0:02:34
2019-07-25T17:38:56.9483223Z ##[error]Bash exited with code '1'.
2019-07-25T17:38:56.9520672Z ##[section]Starting: Checkout
2019-07-25T17:38:56.9522696Z ==============================================================================
2019-07-25T17:38:56.9522785Z Task         : Get sources
2019-07-25T17:38:56.9522840Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
