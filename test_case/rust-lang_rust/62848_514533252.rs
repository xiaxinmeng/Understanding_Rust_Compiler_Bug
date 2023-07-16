plain
2019-07-24T07:54:09.5723893Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-24T07:54:09.5932060Z ##[command]git config gc.auto 0
2019-07-24T07:54:09.6014814Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-24T07:54:09.6076214Z ##[command]git config --get-all http.proxy
2019-07-24T07:54:10.5949829Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62848/merge:refs/remotes/pull/62848/merge
---
2019-07-24T07:54:45.0782742Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-24T07:54:45.0782797Z 
2019-07-24T07:54:45.0783059Z   git checkout -b <new-branch-name>
2019-07-24T07:54:45.0783095Z 
2019-07-24T07:54:45.0783155Z HEAD is now at 909fb43a4 Merge 7e73473603f1f16bc9ff9c5cc304aff478ba80b4 into a7f28678bbf4e16893bb6a718e427504167a9494
2019-07-24T07:54:45.0933937Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-24T07:54:45.0936933Z ==============================================================================
2019-07-24T07:54:45.0937000Z Task         : Bash
2019-07-24T07:54:45.0937057Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-24T08:26:09.4274870Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-07-24T08:26:09.9558467Z    Compiling compiler_builtins v0.1.17
2019-07-24T08:26:12.5153722Z    Compiling backtrace-sys v0.1.27
2019-07-24T08:26:13.8197780Z    Compiling cmake v0.1.38
2019-07-24T08:26:15.3717164Z error: item has missing stability attribute
2019-07-24T08:26:15.3725078Z     |
2019-07-24T08:26:15.3725078Z     |
2019-07-24T08:26:15.3728790Z 554 |     pub fn __dummy_method_to_pin_the_rustc_private_feature(self) {}
2019-07-24T08:26:15.3733206Z 
2019-07-24T08:26:15.9780698Z error: aborting due to previous error
2019-07-24T08:26:15.9780865Z 
2019-07-24T08:26:16.2445335Z error: Could not compile `core`.
2019-07-24T08:26:16.2445335Z error: Could not compile `core`.
2019-07-24T08:26:16.2446882Z warning: build failed, waiting for other jobs to finish...
2019-07-24T08:26:16.7686730Z error: build failed
2019-07-24T08:26:16.7704873Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-24T08:26:16.7705014Z expected success, got: exit code: 101
2019-07-24T08:26:16.7712220Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-24T08:26:16.7712501Z Build completed unsuccessfully in 0:24:48
2019-07-24T08:26:17.7638250Z ##[error]Bash exited with code '1'.
2019-07-24T08:26:17.7684081Z ##[section]Starting: Checkout
2019-07-24T08:26:17.7686200Z ==============================================================================
2019-07-24T08:26:17.7686258Z Task         : Get sources
2019-07-24T08:26:17.7686325Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
