plain
2019-08-26T17:38:52.5437377Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-26T17:38:52.5620254Z ##[command]git config gc.auto 0
2019-08-26T17:38:52.5687048Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-26T17:38:52.5741962Z ##[command]git config --get-all http.proxy
2019-08-26T17:38:52.5881947Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63860/merge:refs/remotes/pull/63860/merge
---
2019-08-26T17:39:27.9851309Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-26T17:39:27.9851448Z 
2019-08-26T17:39:27.9851769Z   git checkout -b <new-branch-name>
2019-08-26T17:39:27.9851905Z 
2019-08-26T17:39:27.9852034Z HEAD is now at 7d4aa58e2 Merge 2e8921aeaef8d6de6e9d90f75dd8a3019f642a8a into 9fa8f140233047fb0211dbaee531a290bcfeae7e
2019-08-26T17:39:28.0022668Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-26T17:39:28.0025636Z ==============================================================================
2019-08-26T17:39:28.0025696Z Task         : Bash
2019-08-26T17:39:28.0025760Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-26T18:09:45.1413259Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2019-08-26T18:09:45.1439979Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-26T18:09:45.5642322Z    Compiling cc v1.0.35
2019-08-26T18:09:45.5642862Z    Compiling core v0.0.0 (/checkout/src/libcore)
2019-08-26T18:09:50.7491813Z thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 2', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-26T18:09:50.8765868Z 
2019-08-26T18:09:50.8767823Z error: internal compiler error: unexpected panic
2019-08-26T18:09:50.8768837Z 
2019-08-26T18:09:50.8770564Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-26T18:09:50.8770564Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-26T18:09:50.8772155Z 
2019-08-26T18:09:50.8775696Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-26T18:09:50.8778736Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-26T18:09:50.8779653Z 
2019-08-26T18:09:50.8779653Z 
2019-08-26T18:09:50.8783192Z note: compiler flags: -Z binary-dep-depinfo -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C prefer-dynamic -C debug-assertions=n -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
2019-08-26T18:09:50.8784128Z 
2019-08-26T18:09:50.8785220Z note: some of the compiler flags provided by cargo are hidden
2019-08-26T18:09:50.8915897Z error: Could not compile `core`.
2019-08-26T18:09:50.8917201Z warning: build failed, waiting for other jobs to finish...
2019-08-26T18:09:52.6205012Z error: build failed
2019-08-26T18:09:52.6222412Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-26T18:09:52.6222412Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-26T18:09:52.6222714Z expected success, got: exit code: 101
2019-08-26T18:09:52.6235731Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-26T18:09:52.6235931Z Build completed unsuccessfully in 0:23:59
2019-08-26T18:09:52.6295854Z == clock drift check ==
2019-08-26T18:09:52.6308686Z   local time: Mon Aug 26 18:09:52 UTC 2019
2019-08-26T18:09:52.6973995Z   network time: Mon, 26 Aug 2019 18:09:52 GMT
2019-08-26T18:09:52.6983088Z == end clock drift check ==
2019-08-26T18:09:53.4580934Z ##[error]Bash exited with code '1'.
2019-08-26T18:09:53.4620323Z ##[section]Starting: Checkout
2019-08-26T18:09:53.4622290Z ==============================================================================
2019-08-26T18:09:53.4622338Z Task         : Get sources
2019-08-26T18:09:53.4622400Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
