plain
2019-08-12T13:42:19.1361991Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-12T13:42:19.1550687Z ##[command]git config gc.auto 0
2019-08-12T13:42:19.1629529Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-12T13:42:19.1686000Z ##[command]git config --get-all http.proxy
2019-08-12T13:42:19.1828213Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61041/merge:refs/remotes/pull/61041/merge
---
2019-08-12T13:42:56.1062515Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-12T13:42:56.1062592Z 
2019-08-12T13:42:56.1062871Z   git checkout -b <new-branch-name>
2019-08-12T13:42:56.1062907Z 
2019-08-12T13:42:56.1063002Z HEAD is now at eb7d126a6 Merge 3aa550a09f5c83b23443349660de4d0929e85c31 into c01be67ea40266d6a4c3289654a07ddd7ce2a172
2019-08-12T13:42:56.1190803Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-12T13:42:56.1193927Z ==============================================================================
2019-08-12T13:42:56.1194005Z Task         : Bash
2019-08-12T13:42:56.1194072Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-12T14:15:55.5185807Z    Compiling ena v0.13.0
2019-08-12T14:15:55.9224576Z    Compiling rustc_version v0.2.3
2019-08-12T14:15:56.8950280Z    Compiling lock_api v0.1.3
2019-08-12T14:15:57.6365717Z    Compiling polonius-engine v0.9.0
2019-08-12T14:15:58.8647244Z thread 'rustc' panicked at 'no call frames exist', src/libcore/option.rs:1166:5
2019-08-12T14:15:58.8724974Z 
2019-08-12T14:15:58.8731227Z error: internal compiler error: unexpected panic
2019-08-12T14:15:58.8731325Z 
2019-08-12T14:15:58.8731451Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-12T14:15:58.8731451Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-12T14:15:58.8731515Z 
2019-08-12T14:15:58.8732231Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-12T14:15:58.8734364Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-08-12T14:15:58.8734413Z 
2019-08-12T14:15:58.8734413Z 
2019-08-12T14:15:58.8734832Z note: compiler flags: -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C prefer-dynamic -C debug-assertions=n -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
2019-08-12T14:15:58.8734900Z 
2019-08-12T14:15:58.8734952Z note: some of the compiler flags provided by cargo are hidden
2019-08-12T14:15:58.8768300Z error: Could not compile `crc32fast`.
2019-08-12T14:15:58.8768654Z warning: build failed, waiting for other jobs to finish...
2019-08-12T14:16:01.1852673Z error: build failed
2019-08-12T14:16:01.1874148Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-08-12T14:16:01.1874148Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-08-12T14:16:01.1874333Z expected success, got: exit code: 101
2019-08-12T14:16:01.1882027Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-12T14:16:01.1882237Z Build completed unsuccessfully in 0:26:39
2019-08-12T14:16:02.6190749Z ##[error]Bash exited with code '1'.
2019-08-12T14:16:02.6236830Z ##[section]Starting: Checkout
2019-08-12T14:16:02.6238716Z ==============================================================================
2019-08-12T14:16:02.6238796Z Task         : Get sources
2019-08-12T14:16:02.6238849Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
