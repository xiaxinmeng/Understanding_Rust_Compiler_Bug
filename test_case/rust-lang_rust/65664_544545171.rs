plain
2019-10-21T14:08:22.7416890Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-21T14:08:22.7593165Z ##[command]git config gc.auto 0
2019-10-21T14:08:22.7666213Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-21T14:08:22.7720251Z ##[command]git config --get-all http.proxy
2019-10-21T14:08:22.7854877Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65664/merge:refs/remotes/pull/65664/merge
---
2019-10-21T14:37:39.9960542Z    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
2019-10-21T14:37:40.4729430Z    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
2019-10-21T14:37:40.9386063Z    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
2019-10-21T14:37:41.4071744Z    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
2019-10-21T14:37:44.9878489Z Incorrect number of arguments passed to called function!
2019-10-21T14:37:45.0043441Z   call void @_ZN4core9panicking5panic17hb4c1ef8e1fda4a71E([0 x i8]* noalias nonnull readonly align 1 bitcast ({ { [0 x i8]*, i64 }, { { [0 x i8]*, i64 }, i32, i32 }* }* @30 to [0 x i8]*))
2019-10-21T14:37:45.0047179Z LLVM ERROR: Broken function found, compilation aborted!
2019-10-21T14:37:45.0320852Z error: could not compile `core`.
2019-10-21T14:37:45.3945766Z error: build failed
2019-10-21T14:37:45.3970219Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-10-21T14:37:45.3970665Z expected success, got: exit code: 101
2019-10-21T14:37:45.3982922Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-21T14:37:45.3982922Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-21T14:37:45.3983290Z Build completed unsuccessfully in 0:22:53
2019-10-21T14:37:45.4042581Z == clock drift check ==
2019-10-21T14:37:45.4065434Z   local time: Mon Oct 21 14:37:45 UTC 2019
2019-10-21T14:37:45.4855661Z   network time: Mon, 21 Oct 2019 14:37:45 GMT
2019-10-21T14:37:45.4859620Z == end clock drift check ==
2019-10-21T14:37:47.5050280Z 
2019-10-21T14:37:47.5151383Z ##[error]Bash exited with code '1'.
2019-10-21T14:37:47.5188600Z ##[section]Starting: Checkout
2019-10-21T14:37:47.5190275Z ==============================================================================
2019-10-21T14:37:47.5190331Z Task         : Get sources
2019-10-21T14:37:47.5190397Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
