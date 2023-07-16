plain
2019-08-11T19:05:39.3128354Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-11T19:05:39.3313858Z ##[command]git config gc.auto 0
2019-08-11T19:05:39.3394797Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-11T19:05:39.3447350Z ##[command]git config --get-all http.proxy
2019-08-11T19:05:39.3584964Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63470/merge:refs/remotes/pull/63470/merge
---
2019-08-11T19:06:12.9069125Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-11T19:06:12.9069330Z 
2019-08-11T19:06:12.9069530Z   git checkout -b <new-branch-name>
2019-08-11T19:06:12.9069558Z 
2019-08-11T19:06:12.9069601Z HEAD is now at 8844b12a1 Merge 0afbfe248460b7fccf00849de9ac672862124a67 into 8a068699a24de306334a1f66b9a83552766d853c
2019-08-11T19:06:12.9212756Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-11T19:06:12.9215188Z ==============================================================================
2019-08-11T19:06:12.9215259Z Task         : Bash
2019-08-11T19:06:12.9215300Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-11T19:11:09.1846753Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-11T19:11:09.1868402Z Checking std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-11T19:11:09.5853459Z    Compiling cc v1.0.35
2019-08-11T19:11:09.5853787Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-08-11T19:11:15.0181689Z error[E0093]: unrecognized intrinsic function: `uninit`
2019-08-11T19:11:15.0183952Z     |
2019-08-11T19:11:15.0183952Z     |
2019-08-11T19:11:15.0184432Z 728 |     pub fn uninit<T>() -> T;
2019-08-11T19:11:15.0184755Z     |     ^^^^^^^^^^^^^^^^^^^^^^^^ unrecognized intrinsic
2019-08-11T19:11:18.7086112Z    Compiling libc v0.2.60
2019-08-11T19:11:19.6179404Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-08-11T19:11:20.1175990Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-08-11T19:11:20.3094492Z error: aborting due to previous error
2019-08-11T19:11:20.3094492Z error: aborting due to previous error
2019-08-11T19:11:20.3094599Z 
2019-08-11T19:11:20.3097860Z For more information about this error, try `rustc --explain E0093`.
2019-08-11T19:11:20.4221159Z error: Could not compile `core`.
2019-08-11T19:11:20.4221981Z warning: build failed, waiting for other jobs to finish...
2019-08-11T19:11:21.1342126Z error: build failed
2019-08-11T19:11:21.1375353Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-08-11T19:11:21.1386698Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-11T19:11:21.1387050Z Build completed unsuccessfully in 0:02:28
2019-08-11T19:11:21.1387050Z Build completed unsuccessfully in 0:02:28
2019-08-11T19:11:29.9243239Z ##[error]Bash exited with code '1'.
2019-08-11T19:11:29.9278545Z ##[section]Starting: Checkout
2019-08-11T19:11:29.9280284Z ==============================================================================
2019-08-11T19:11:29.9280353Z Task         : Get sources
2019-08-11T19:11:29.9280396Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
