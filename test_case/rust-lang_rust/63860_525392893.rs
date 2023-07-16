plain
2019-08-27T16:30:34.0739952Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-27T16:30:34.0996598Z ##[command]git config gc.auto 0
2019-08-27T16:30:34.1079892Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-27T16:30:34.9917336Z ##[command]git config --get-all http.proxy
2019-08-27T16:30:34.9925615Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63860/merge:refs/remotes/pull/63860/merge
---
2019-08-27T16:31:09.8747679Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-27T16:31:09.8747728Z 
2019-08-27T16:31:09.8748027Z   git checkout -b <new-branch-name>
2019-08-27T16:31:09.8748059Z 
2019-08-27T16:31:09.8748109Z HEAD is now at 251ada4bf Merge 5859d41217aeeb4d24a5bfd0d98cfd4a8993650f into 0396aace27eea97c3603e9683e921807dff2a314
2019-08-27T16:31:09.8914374Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-27T16:31:09.8917856Z ==============================================================================
2019-08-27T16:31:09.8917917Z Task         : Bash
2019-08-27T16:31:09.8918203Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-27T17:01:58.1281154Z    Compiling libc v0.2.61
2019-08-27T17:01:59.0465534Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-08-27T17:02:00.7210017Z    Compiling cmake v0.1.38
2019-08-27T17:02:04.1220082Z    Compiling compiler_builtins v0.1.18
2019-08-27T17:02:04.9764180Z thread 'rustc' panicked at 'index out of bounds: the len is 6 but the index is 7', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-27T17:02:05.9610420Z 
2019-08-27T17:02:05.9617615Z error: internal compiler error: unexpected panic
2019-08-27T17:02:05.9617814Z 
2019-08-27T17:02:05.9617982Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-27T17:02:05.9617982Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-27T17:02:05.9618096Z 
2019-08-27T17:02:05.9619627Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-27T17:02:05.9620269Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-27T17:02:05.9620591Z 
2019-08-27T17:02:05.9620591Z 
2019-08-27T17:02:05.9621857Z note: compiler flags: -Z binary-dep-depinfo -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C prefer-dynamic -C debug-assertions=n -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
2019-08-27T17:02:05.9622118Z 
2019-08-27T17:02:05.9622305Z note: some of the compiler flags provided by cargo are hidden
2019-08-27T17:02:05.9622869Z error: Could not compile `core`.
2019-08-27T17:02:05.9623374Z warning: build failed, waiting for other jobs to finish...
2019-08-27T17:02:06.0523850Z error: build failed
2019-08-27T17:02:06.0552970Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-27T17:02:06.0552970Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-27T17:02:06.0553449Z expected success, got: exit code: 101
2019-08-27T17:02:06.0558666Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-27T17:02:06.0558919Z Build completed unsuccessfully in 0:24:26
2019-08-27T17:02:06.0615491Z == clock drift check ==
2019-08-27T17:02:06.0632541Z   local time: Tue Aug 27 17:02:06 UTC 2019
2019-08-27T17:02:06.2212551Z   network time: Tue, 27 Aug 2019 17:02:06 GMT
2019-08-27T17:02:06.2220294Z == end clock drift check ==
2019-08-27T17:02:07.1547946Z ##[error]Bash exited with code '1'.
2019-08-27T17:02:07.1585010Z ##[section]Starting: Checkout
2019-08-27T17:02:07.1586791Z ==============================================================================
2019-08-27T17:02:07.1586840Z Task         : Get sources
2019-08-27T17:02:07.1586905Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
