plain
2019-07-18T14:29:13.8444095Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-18T14:29:13.8650219Z ##[command]git config gc.auto 0
2019-07-18T14:29:13.8726040Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-18T14:29:13.8778390Z ##[command]git config --get-all http.proxy
2019-07-18T14:29:13.8922199Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/60966/merge:refs/remotes/pull/60966/merge
---
2019-07-18T14:29:49.0511956Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-18T14:29:49.0511989Z 
2019-07-18T14:29:49.0512240Z   git checkout -b <new-branch-name>
2019-07-18T14:29:49.0512277Z 
2019-07-18T14:29:49.0512328Z HEAD is now at 8b3deec1a Merge 7a73454b38722095374d00ed17cc9fa865c929cf into 21d5b8bf0c26e3ee4c270ce5527df66b1af56513
2019-07-18T14:29:49.0647310Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-18T14:29:49.0650759Z ==============================================================================
2019-07-18T14:29:49.0650832Z Task         : Bash
2019-07-18T14:29:49.0650881Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-18T15:03:59.3205523Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-07-18T15:04:04.6898527Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-07-18T15:04:24.6475061Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-07-18T15:05:52.4067080Z    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-07-18T15:05:56.1052743Z thread 'rustc' panicked at 'Feature `rustc_diagnostic_items` is not declared anywhere', src/libsyntax/feature_gate.rs:1728:21
2019-07-18T15:05:56.2932001Z 
2019-07-18T15:05:56.2937906Z error: internal compiler error: unexpected panic
2019-07-18T15:05:56.2938430Z 
2019-07-18T15:05:56.2938430Z 
2019-07-18T15:05:56.2955449Z note: the compiler unexpectedly panicked. this is a bug.
2019-07-18T15:05:56.2955539Z 
2019-07-18T15:05:56.2958857Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-07-18T15:05:56.2958936Z 
2019-07-18T15:05:56.2976982Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-07-18T15:05:56.2977051Z 
2019-07-18T15:05:56.2977761Z note: compiler flags: -Z unstable-options -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C prefer-dynamic -C debug-assertions=n -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
2019-07-18T15:05:56.2977845Z 
2019-07-18T15:05:56.2977901Z note: some of the compiler flags provided by cargo are hidden
2019-07-18T15:05:56.3078125Z error: Could not compile `rustc`.
2019-07-18T15:05:56.3078497Z warning: build failed, waiting for other jobs to finish...
2019-07-18T15:06:27.5367882Z error: build failed
2019-07-18T15:06:27.5400399Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-18T15:06:27.5400399Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-18T15:06:27.5400603Z expected success, got: exit code: 101
2019-07-18T15:06:27.5406425Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-18T15:06:27.5406561Z Build completed unsuccessfully in 0:30:13
2019-07-18T15:06:28.4703516Z ##[error]Bash exited with code '1'.
2019-07-18T15:06:28.4742760Z ##[section]Starting: Checkout
2019-07-18T15:06:28.4744827Z ==============================================================================
2019-07-18T15:06:28.4744891Z Task         : Get sources
2019-07-18T15:06:28.4744961Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
