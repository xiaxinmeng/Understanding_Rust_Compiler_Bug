plain
2019-08-09T13:22:51.6409930Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-09T13:22:51.6594177Z ##[command]git config gc.auto 0
2019-08-09T13:22:52.2587929Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-09T13:22:52.2599886Z ##[command]git config --get-all http.proxy
2019-08-09T13:22:52.2607379Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63343/merge:refs/remotes/pull/63343/merge
---
2019-08-09T13:23:27.5410510Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-09T13:23:27.5410560Z 
2019-08-09T13:23:27.5410798Z   git checkout -b <new-branch-name>
2019-08-09T13:23:27.5410831Z 
2019-08-09T13:23:27.5410895Z HEAD is now at d70671342 Merge 77dcdca7be657732dbaabfb4dc2f2db6efa385c5 into 813a3a5d4b2be4e2101faa73a067da02a704a598
2019-08-09T13:23:27.5579902Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-09T13:23:27.5582788Z ==============================================================================
2019-08-09T13:23:27.5582866Z Task         : Bash
2019-08-09T13:23:27.5582917Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-09T13:28:33.2781908Z    |
2019-08-09T13:28:33.2782271Z 76 | #![feature(const_generics)]
2019-08-09T13:28:33.2787536Z    |            ^^^^^^^^^^^^^^
2019-08-09T13:28:33.2787664Z 
2019-08-09T13:28:36.1932478Z error: use of item 'intrinsics::uninit' that will be deprecated in future version 1.38.0: superseded by MaybeUninit, removal planned
2019-08-09T13:28:36.1932959Z    --> src/libcore/mem/mod.rs:483:5
2019-08-09T13:28:36.1933596Z 483 |     intrinsics::uninit()
2019-08-09T13:28:36.1933887Z     |     ^^^^^^^^^^^^^^^^^^
2019-08-09T13:28:36.1940334Z     |
2019-08-09T13:28:36.1940667Z     = note: `-D deprecated-in-future` implied by `-D warnings`
---
2019-08-09T13:28:43.2348084Z 
2019-08-09T13:28:43.3272390Z error: Could not compile `core`.
2019-08-09T13:28:43.3272823Z warning: build failed, waiting for other jobs to finish...
2019-08-09T13:28:43.6172509Z error: build failed
2019-08-09T13:28:43.6204573Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-08-09T13:28:43.6214823Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-09T13:28:43.6215721Z Build completed unsuccessfully in 0:02:21
2019-08-09T13:28:43.6215721Z Build completed unsuccessfully in 0:02:21
2019-08-09T13:28:57.1094454Z ##[error]Bash exited with code '1'.
2019-08-09T13:28:57.1135415Z ##[section]Starting: Checkout
2019-08-09T13:28:57.1137537Z ==============================================================================
2019-08-09T13:28:57.1137603Z Task         : Get sources
2019-08-09T13:28:57.1137657Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
