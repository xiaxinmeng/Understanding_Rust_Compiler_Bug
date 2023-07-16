plain
2019-08-28T21:40:28.8914560Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-28T21:40:28.9138164Z ##[command]git config gc.auto 0
2019-08-28T21:40:28.9232673Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-28T21:40:28.9301888Z ##[command]git config --get-all http.proxy
2019-08-28T21:40:28.9452061Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63860/merge:refs/remotes/pull/63860/merge
---
2019-08-28T22:09:49.4429499Z    Compiling libc v0.2.61
2019-08-28T22:09:50.3328627Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-08-28T22:09:51.9003019Z    Compiling cmake v0.1.38
2019-08-28T22:09:55.0953969Z    Compiling compiler_builtins v0.1.18
2019-08-28T22:09:56.1171653Z thread 'rustc' panicked at 'index out of bounds: the len is 6 but the index is 7', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-28T22:09:56.2671682Z 
2019-08-28T22:09:56.2675816Z error: internal compiler error: unexpected panic
2019-08-28T22:09:56.2675900Z 
2019-08-28T22:09:56.2679775Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-28T22:09:56.2679775Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-28T22:09:56.2682506Z 
2019-08-28T22:09:56.2688791Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-28T22:09:56.2693205Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-28T22:09:56.2693261Z 
2019-08-28T22:09:56.2693261Z 
2019-08-28T22:09:56.2698271Z note: compiler flags: -Z binary-dep-depinfo -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C prefer-dynamic -C debug-assertions=n -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
2019-08-28T22:09:56.2698359Z 
2019-08-28T22:09:56.2701089Z note: some of the compiler flags provided by cargo are hidden
2019-08-28T22:09:56.2880947Z error: Could not compile `core`.
2019-08-28T22:09:56.2895298Z warning: build failed, waiting for other jobs to finish...
2019-08-28T22:09:56.9890038Z error: build failed
2019-08-28T22:09:56.9921981Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-28T22:09:56.9921981Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-28T22:09:56.9922393Z expected success, got: exit code: 101
2019-08-28T22:09:56.9935540Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-28T22:09:56.9936576Z Build completed unsuccessfully in 0:22:37
2019-08-28T22:09:56.9989520Z == clock drift check ==
2019-08-28T22:09:57.0010108Z   local time: Wed Aug 28 22:09:57 UTC 2019
2019-08-28T22:09:57.1518424Z   network time: Wed, 28 Aug 2019 22:09:57 GMT
2019-08-28T22:09:57.1518880Z == end clock drift check ==
2019-08-28T22:09:58.0834046Z ##[error]Bash exited with code '1'.
2019-08-28T22:09:58.0872223Z ##[section]Starting: Checkout
2019-08-28T22:09:58.0874003Z ==============================================================================
2019-08-28T22:09:58.0874059Z Task         : Get sources
2019-08-28T22:09:58.0874124Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
