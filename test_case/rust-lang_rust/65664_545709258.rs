plain
2019-10-24T01:55:58.7500482Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-24T01:55:58.7711725Z ##[command]git config gc.auto 0
2019-10-24T01:55:58.7771418Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-24T01:55:58.7828557Z ##[command]git config --get-all http.proxy
2019-10-24T01:55:58.7980602Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65664/merge:refs/remotes/pull/65664/merge
---
2019-10-24T02:03:19.9935996Z    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
2019-10-24T02:03:20.5263721Z    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
2019-10-24T02:03:21.0716415Z    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
2019-10-24T02:03:21.6093836Z    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
2019-10-24T02:03:27.2377343Z Incorrect number of arguments passed to called function!
2019-10-24T02:03:27.2385504Z   call void @_ZN4core9panicking5panic17h635a71680f3afa34E([0 x i8]* noalias nonnull readonly align 1 bitcast ({ { [0 x i8]*, i64 }, { [0 x i8]*, i64 }, i32, i32 }* @anon.7cbff8acfa1392488a26f804dd057cf1.20 to [0 x i8]*))
2019-10-24T02:03:27.2391819Z in function _ZN4core3num7flt2dec8strategy5grisu19format_shortest_opt17ha51764645632becbE
2019-10-24T02:03:27.2397520Z LLVM ERROR: Broken function found, compilation aborted!
2019-10-24T02:03:27.2609789Z error: could not compile `core`.
2019-10-24T02:03:27.5811761Z error: build failed
2019-10-24T02:03:27.5833417Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-10-24T02:03:27.5834328Z expected success, got: exit code: 101
2019-10-24T02:03:27.5842652Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-24T02:03:27.5842652Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-24T02:03:27.5843258Z Build completed unsuccessfully in 0:00:29
2019-10-24T02:03:27.5894734Z == clock drift check ==
2019-10-24T02:03:27.5911376Z   local time: Thu Oct 24 02:03:27 UTC 2019
2019-10-24T02:03:27.6893662Z   network time: Thu, 24 Oct 2019 02:03:27 GMT
2019-10-24T02:03:27.6900688Z == end clock drift check ==
2019-10-24T02:03:29.9466622Z 
2019-10-24T02:03:29.9575257Z ##[error]Bash exited with code '1'.
2019-10-24T02:03:29.9632975Z ##[section]Starting: Checkout
2019-10-24T02:03:29.9634580Z ==============================================================================
2019-10-24T02:03:29.9634655Z Task         : Get sources
2019-10-24T02:03:29.9634696Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
