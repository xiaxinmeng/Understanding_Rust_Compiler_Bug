plain
2019-08-16T18:37:14.4274859Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-16T18:37:14.4565647Z ##[command]git config gc.auto 0
2019-08-16T18:37:14.4633711Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-16T18:37:14.4698414Z ##[command]git config --get-all http.proxy
2019-08-16T18:37:14.4849889Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63468/merge:refs/remotes/pull/63468/merge
---
2019-08-16T18:37:51.1304210Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-16T18:37:51.1304242Z 
2019-08-16T18:37:51.1304592Z   git checkout -b <new-branch-name>
2019-08-16T18:37:51.1304638Z 
2019-08-16T18:37:51.1304691Z HEAD is now at 4a5b19b8f Merge 3a17f9d48c605d7e8211354c10b4252445db6961 into 9a32ad0dd51f8451aa6e39d7e9ea89483cb8fcfa
2019-08-16T18:37:51.1465815Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-16T18:37:51.1468064Z ==============================================================================
2019-08-16T18:37:51.1468120Z Task         : Bash
2019-08-16T18:37:51.1468156Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-16T19:08:21.6162059Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2019-08-16T19:08:21.6187164Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-16T19:08:22.0715996Z    Compiling cc v1.0.35
2019-08-16T19:08:22.0753888Z    Compiling core v0.0.0 (/checkout/src/libcore)
2019-08-16T19:08:23.4952724Z thread 'rustc' panicked at 'invocation data is reset for an invocation', src/librustc_resolve/build_reduced_graph.rs:1082:9
2019-08-16T19:08:23.6023817Z 
2019-08-16T19:08:23.6024045Z error: internal compiler error: unexpected panic
2019-08-16T19:08:23.6024076Z 
2019-08-16T19:08:23.6029020Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-16T19:08:23.6029020Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-16T19:08:23.6072943Z 
2019-08-16T19:08:23.6073987Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-16T19:08:23.6074319Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-16T19:08:23.6074346Z 
2019-08-16T19:08:23.6074346Z 
2019-08-16T19:08:23.6074654Z note: compiler flags: -Z binary-dep-depinfo -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C prefer-dynamic -C debug-assertions=n -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
2019-08-16T19:08:23.6074724Z 
2019-08-16T19:08:23.6074762Z note: some of the compiler flags provided by cargo are hidden
2019-08-16T19:08:23.6162723Z error: Could not compile `core`.
2019-08-16T19:08:23.6163181Z warning: build failed, waiting for other jobs to finish...
2019-08-16T19:08:27.5478048Z error: build failed
2019-08-16T19:08:27.5501314Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-16T19:08:27.5501314Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-16T19:08:27.5502056Z expected success, got: exit code: 101
2019-08-16T19:08:27.5513382Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-16T19:08:27.5514350Z Build completed unsuccessfully in 0:23:59
2019-08-16T19:08:27.5573719Z == clock drift check ==
2019-08-16T19:08:27.5586807Z   local time: Fri Aug 16 19:08:27 UTC 2019
2019-08-16T19:08:27.8359314Z   network time: Fri, 16 Aug 2019 19:08:27 GMT
2019-08-16T19:08:27.8360656Z == end clock drift check ==
2019-08-16T19:08:28.6562661Z ##[error]Bash exited with code '1'.
2019-08-16T19:08:28.6604424Z ##[section]Starting: Checkout
2019-08-16T19:08:28.6605942Z ==============================================================================
2019-08-16T19:08:28.6605988Z Task         : Get sources
2019-08-16T19:08:28.6606045Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
