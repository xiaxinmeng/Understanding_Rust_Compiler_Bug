plain
2019-08-19T22:46:00.4366122Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-19T22:46:00.4541599Z ##[command]git config gc.auto 0
2019-08-19T22:46:00.4619328Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-19T22:46:00.4672517Z ##[command]git config --get-all http.proxy
2019-08-19T22:46:00.4795566Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63721/merge:refs/remotes/pull/63721/merge
---
2019-08-19T22:46:36.0131590Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-19T22:46:36.0131638Z 
2019-08-19T22:46:36.0131855Z   git checkout -b <new-branch-name>
2019-08-19T22:46:36.0131884Z 
2019-08-19T22:46:36.0131932Z HEAD is now at ef10896ea Merge ffec016917a68265fbe7c39ab9a51d280a28c689 into 29a54035c77cb2ba7ea2c24b2437760d0495a2c8
2019-08-19T22:46:36.0260860Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-19T22:46:36.0263781Z ==============================================================================
2019-08-19T22:46:36.0263840Z Task         : Bash
2019-08-19T22:46:36.0263885Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-19T22:54:08.9220951Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-08-19T22:54:17.6938323Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-08-19T22:54:19.1157062Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-08-19T22:54:20.3234086Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-08-19T22:54:22.7409519Z thread 'rustc' panicked at 'env var `RUSTC_ERROR_METADATA_DST` isn't set', src/libcore/option.rs:1166:5
2019-08-19T22:54:22.7677696Z 
2019-08-19T22:54:22.7679113Z error: internal compiler error: unexpected panic
2019-08-19T22:54:22.7681090Z 
2019-08-19T22:54:22.7684080Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-19T22:54:22.7684080Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-19T22:54:22.7684292Z 
2019-08-19T22:54:22.7685407Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-19T22:54:22.7685653Z 
2019-08-19T22:54:22.7686429Z note: rustc 1.38.0-beta.1 (e450539c2 2019-08-13) running on x86_64-unknown-linux-gnu
2019-08-19T22:54:22.7686696Z 
2019-08-19T22:54:22.7687350Z note: compiler flags: -Z binary-dep-depinfo -Z unstable-options -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C prefer-dynamic -C debug-assertions=n -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
2019-08-19T22:54:22.7687548Z 
2019-08-19T22:54:22.7687763Z note: some of the compiler flags provided by cargo are hidden
2019-08-19T22:54:22.7787286Z error: Could not compile `syntax`.
2019-08-19T22:54:22.7787591Z 
2019-08-19T22:54:22.7788058Z To learn more, run the command again with --verbose.
2019-08-19T22:54:22.7816788Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-19T22:54:22.7816788Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-19T22:54:22.7817348Z expected success, got: exit code: 101
2019-08-19T22:54:22.7820062Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-19T22:54:22.7820318Z Build completed unsuccessfully in 0:04:48
2019-08-19T22:54:22.7874426Z == clock drift check ==
2019-08-19T22:54:22.7890107Z   local time: Mon Aug 19 22:54:22 UTC 2019
2019-08-19T22:54:23.0655575Z   network time: Mon, 19 Aug 2019 22:54:23 GMT
2019-08-19T22:54:23.0659635Z == end clock drift check ==
2019-08-19T22:54:24.1997859Z ##[error]Bash exited with code '1'.
2019-08-19T22:54:24.2079482Z ##[section]Starting: Checkout
2019-08-19T22:54:24.2083971Z ==============================================================================
2019-08-19T22:54:24.2084041Z Task         : Get sources
2019-08-19T22:54:24.2084112Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
