plain
2019-08-27T18:19:00.7625054Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-27T18:19:00.7852907Z ##[command]git config gc.auto 0
2019-08-27T18:19:00.7963097Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-27T18:19:00.8047271Z ##[command]git config --get-all http.proxy
2019-08-27T18:19:01.5398202Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63860/merge:refs/remotes/pull/63860/merge
---
2019-08-27T18:19:36.6121918Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-27T18:19:36.6121948Z 
2019-08-27T18:19:36.6122289Z   git checkout -b <new-branch-name>
2019-08-27T18:19:36.6122314Z 
2019-08-27T18:19:36.6122362Z HEAD is now at 59bcdca93 Merge 44e4bf391258348fb3b5da44eba52302ac78c9f3 into 0396aace27eea97c3603e9683e921807dff2a314
2019-08-27T18:19:36.6286676Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-27T18:19:36.6290010Z ==============================================================================
2019-08-27T18:19:36.6290081Z Task         : Bash
2019-08-27T18:19:36.6290148Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-27T18:49:41.2462577Z    Compiling libc v0.2.61
2019-08-27T18:49:42.1365648Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-08-27T18:49:43.7321058Z    Compiling cmake v0.1.38
2019-08-27T18:49:46.9442744Z    Compiling compiler_builtins v0.1.18
2019-08-27T18:49:47.7944674Z thread 'rustc' panicked at 'index out of bounds: the len is 6 but the index is 7', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-27T18:49:47.9492463Z 
2019-08-27T18:49:47.9493272Z error: internal compiler error: unexpected panic
2019-08-27T18:49:47.9493464Z 
2019-08-27T18:49:47.9493719Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-27T18:49:47.9493719Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-27T18:49:47.9493883Z 
2019-08-27T18:49:47.9498678Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-27T18:49:47.9499563Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-27T18:49:47.9499846Z 
2019-08-27T18:49:47.9499846Z 
2019-08-27T18:49:47.9500751Z note: compiler flags: -Z binary-dep-depinfo -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C prefer-dynamic -C debug-assertions=n -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
2019-08-27T18:49:47.9500978Z 
2019-08-27T18:49:47.9501180Z note: some of the compiler flags provided by cargo are hidden
2019-08-27T18:49:47.9637939Z error: Could not compile `core`.
2019-08-27T18:49:47.9638841Z warning: build failed, waiting for other jobs to finish...
2019-08-27T18:49:48.7834113Z error: build failed
2019-08-27T18:49:48.7847892Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-27T18:49:48.7847892Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-27T18:49:48.7848071Z expected success, got: exit code: 101
2019-08-27T18:49:48.7861022Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-27T18:49:48.7861137Z Build completed unsuccessfully in 0:23:38
2019-08-27T18:49:48.7924076Z == clock drift check ==
2019-08-27T18:49:48.7942231Z   local time: Tue Aug 27 18:49:48 UTC 2019
2019-08-27T18:49:49.0735989Z   network time: Tue, 27 Aug 2019 18:49:49 GMT
2019-08-27T18:49:49.0740087Z == end clock drift check ==
2019-08-27T18:49:49.9675702Z ##[error]Bash exited with code '1'.
2019-08-27T18:49:49.9721757Z ##[section]Starting: Checkout
2019-08-27T18:49:49.9723342Z ==============================================================================
2019-08-27T18:49:49.9723388Z Task         : Get sources
2019-08-27T18:49:49.9723427Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
