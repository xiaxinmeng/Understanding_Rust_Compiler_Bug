plain
2019-10-22T04:37:52.8423165Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-22T04:37:52.8643571Z ##[command]git config gc.auto 0
2019-10-22T04:37:52.8710250Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-22T04:37:52.8769945Z ##[command]git config --get-all http.proxy
2019-10-22T04:37:52.8911737Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65664/merge:refs/remotes/pull/65664/merge
---
2019-10-22T05:09:36.2747443Z    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
2019-10-22T05:09:36.7898590Z    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
2019-10-22T05:09:37.3209573Z    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
2019-10-22T05:09:37.8293676Z    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
2019-10-22T05:09:42.6813784Z Incorrect number of arguments passed to called function!
2019-10-22T05:09:42.7093486Z   call void @_ZN4core9panicking5panic17hb4c1ef8e1fda4a71E([0 x i8]* noalias nonnull readonly align 1 bitcast ({ [0 x i8]*, i64 } { [0 x i8]* bitcast ([31 x i8]* @str.3 to [0 x i8]*), i64 31 } to [0 x i8]*), i64 bitcast ({ { [0 x i8]*, i64 }, i32, i32 }* @21 to i64))
2019-10-22T05:09:42.7097489Z LLVM ERROR: Broken function found, compilation aborted!
2019-10-22T05:09:42.7364322Z error: could not compile `core`.
2019-10-22T05:09:43.7322327Z error: build failed
2019-10-22T05:09:43.7338581Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-10-22T05:09:43.7339002Z expected success, got: exit code: 101
2019-10-22T05:09:43.7349207Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-22T05:09:43.7349207Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-22T05:09:43.7349829Z Build completed unsuccessfully in 0:24:54
2019-10-22T05:09:43.7414942Z == clock drift check ==
2019-10-22T05:09:43.7439019Z   local time: Tue Oct 22 05:09:43 UTC 2019
2019-10-22T05:09:43.9078781Z   network time: Tue, 22 Oct 2019 05:09:43 GMT
2019-10-22T05:09:43.9079739Z == end clock drift check ==
2019-10-22T05:09:46.2895760Z 
2019-10-22T05:09:46.3002076Z ##[error]Bash exited with code '1'.
2019-10-22T05:09:46.3041804Z ##[section]Starting: Checkout
2019-10-22T05:09:46.3044379Z ==============================================================================
2019-10-22T05:09:46.3044445Z Task         : Get sources
2019-10-22T05:09:46.3044899Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
