plain
2019-07-29T12:33:51.8281952Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-29T12:33:51.8451341Z ##[command]git config gc.auto 0
2019-07-29T12:33:51.8523906Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-29T12:33:51.8568278Z ##[command]git config --get-all http.proxy
2019-07-29T12:33:51.8702758Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62959/merge:refs/remotes/pull/62959/merge
---
2019-07-29T12:34:25.2350067Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-29T12:34:25.2351553Z 
2019-07-29T12:34:25.2352305Z   git checkout -b <new-branch-name>
2019-07-29T12:34:25.2352688Z 
2019-07-29T12:34:25.2353101Z HEAD is now at 5ddf20b22 Merge 706d396b3d8d050cc59dbbd380d4abcd835b71a2 into 8b94e9e9188b65df38a5f1ae723617dc2dfb3155
2019-07-29T12:34:25.2474888Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-29T12:34:25.2477683Z ==============================================================================
2019-07-29T12:34:25.2477741Z Task         : Bash
2019-07-29T12:34:25.2477776Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-29T13:33:40.0303747Z .................................................................................................... 1400/8804
2019-07-29T13:33:45.8098768Z .................................................................................................... 1500/8804
2019-07-29T13:33:58.1357821Z ................................................................i...............i................... 1600/8804
2019-07-29T13:34:05.6766587Z .................................................................................................... 1700/8804
2019-07-29T13:34:20.2092729Z ..................................................iiiii............................................. 1800/8804
2019-07-29T13:34:31.1742634Z .................................................................................................... 2000/8804
2019-07-29T13:34:33.6921274Z .................................................................................................... 2100/8804
2019-07-29T13:34:37.4518196Z .................................................................................................... 2200/8804
2019-07-29T13:34:44.1015076Z .................................................................................................... 2300/8804
---
2019-07-29T13:38:26.5344611Z .................................................................................................... 5200/8804
2019-07-29T13:38:37.2269755Z .................................................................................................... 5300/8804
2019-07-29T13:38:44.8752277Z ..i................................................................................................. 5400/8804
2019-07-29T13:38:49.9167837Z .................................................................................................... 5500/8804
2019-07-29T13:39:01.5632383Z ................................................................................................ii.. 5600/8804
2019-07-29T13:39:16.2258327Z .i..ii...........i.................................................................................. 5700/8804
2019-07-29T13:39:30.4504004Z .................................................................................................... 5900/8804
2019-07-29T13:39:35.4279663Z ................................................................................................i..i 6000/8804
2019-07-29T13:39:50.1437787Z i................................................................................................... 6100/8804
2019-07-29T13:40:05.4605681Z .................................................................................................... 6200/8804
---
2019-07-29T13:43:52.2934387Z failures:
2019-07-29T13:43:52.2962097Z 
2019-07-29T13:43:52.2962550Z ---- [ui] ui/chalkify/builtin-copy-clone.rs stdout ----
2019-07-29T13:43:52.2962584Z 
2019-07-29T13:43:52.2962996Z error: test compilation failed although it shouldn't!
2019-07-29T13:43:52.2963056Z status: exit code: 101
2019-07-29T13:43:52.2964154Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/chalkify/builtin-copy-clone.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/builtin-copy-clone/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "chalk" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/builtin-copy-clone/auxiliary" "-A" "unused"
2019-07-29T13:43:52.2964736Z ------------------------------------------
2019-07-29T13:43:52.2964794Z 
2019-07-29T13:43:52.2965014Z ------------------------------------------
2019-07-29T13:43:52.2965064Z stderr:
2019-07-29T13:43:52.2965064Z stderr:
2019-07-29T13:43:52.2965290Z ------------------------------------------
2019-07-29T13:43:52.2965385Z error: internal compiler error: src/librustc/ty/relate.rs:581: var types encountered in super_relate_consts: Const { ty: std::array::IntoIter<T, N>, val: Infer(Var(_#0c)) } Const { ty: std::array::IntoIter<T, N>, val: Infer(Var(_#0c)) }
2019-07-29T13:43:52.2965441Z 
2019-07-29T13:43:52.2965715Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:644:9
2019-07-29T13:43:52.2965825Z error: aborting due to previous error
2019-07-29T13:43:52.2965856Z 
2019-07-29T13:43:52.2965901Z 
2019-07-29T13:43:52.2965901Z 
2019-07-29T13:43:52.2965948Z note: the compiler unexpectedly panicked. this is a bug.
2019-07-29T13:43:52.2965980Z 
2019-07-29T13:43:52.2966420Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-07-29T13:43:52.2966459Z 
2019-07-29T13:43:52.2966703Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-07-29T13:43:52.2966737Z 
2019-07-29T13:43:52.2967058Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z chalk -C prefer-dynamic -C rpath -C debuginfo=0
2019-07-29T13:43:52.2967125Z 
2019-07-29T13:43:52.2967479Z ------------------------------------------
2019-07-29T13:43:52.2967513Z 
2019-07-29T13:43:52.2967553Z 
---
2019-07-29T13:43:52.2999298Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-07-29T13:43:52.2999395Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-29T13:43:52.3022058Z 
2019-07-29T13:43:52.3022135Z 
2019-07-29T13:43:52.3027464Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-29T13:43:52.3027835Z 
2019-07-29T13:43:52.3027863Z 
2019-07-29T13:43:52.3027956Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-29T13:43:52.3028003Z Build completed unsuccessfully in 1:02:53
2019-07-29T13:43:52.3028003Z Build completed unsuccessfully in 1:02:53
2019-07-29T13:43:53.0901202Z ##[error]Bash exited with code '1'.
2019-07-29T13:43:53.0954109Z ##[section]Starting: Checkout
2019-07-29T13:43:53.0956779Z ==============================================================================
2019-07-29T13:43:53.0956836Z Task         : Get sources
2019-07-29T13:43:53.0956900Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
