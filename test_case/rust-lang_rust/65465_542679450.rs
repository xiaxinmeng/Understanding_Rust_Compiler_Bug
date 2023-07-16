plain
2019-10-16T11:24:31.0821504Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-16T11:24:31.1023241Z ##[command]git config gc.auto 0
2019-10-16T11:24:31.1102491Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-16T11:24:31.1151233Z ##[command]git config --get-all http.proxy
2019-10-16T11:24:31.1286429Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65465/merge:refs/remotes/pull/65465/merge
---
2019-10-16T12:25:54.8698118Z .................................................................................................... 1600/9196
2019-10-16T12:26:00.1197790Z .................................................................................................... 1700/9196
2019-10-16T12:26:12.8819073Z .............................i...............i...................................................... 1800/9196
2019-10-16T12:26:20.3486091Z .................................................................................................... 1900/9196
2019-10-16T12:26:34.4550724Z ...................iiiii............................................................................ 2000/9196
2019-10-16T12:26:44.8813762Z .................................................................................................... 2200/9196
2019-10-16T12:26:47.3660042Z .................................................................................................... 2300/9196
2019-10-16T12:26:52.6977513Z .................................................................................................... 2400/9196
2019-10-16T12:27:14.0061796Z .................................................................................................... 2500/9196
---
2019-10-16T12:30:06.7371284Z ......................i...............i............................................................. 4800/9196
2019-10-16T12:30:18.0325402Z .................................................................................................... 4900/9196
2019-10-16T12:30:24.3106850Z .................................................................................................... 5000/9196
2019-10-16T12:30:34.5114883Z .................................................................................................... 5100/9196
2019-10-16T12:30:40.8124833Z ......................ii.ii......................................................................... 5200/9196
2019-10-16T12:30:51.1889771Z .................................................................................................... 5400/9196
2019-10-16T12:31:00.9846620Z ........................................................................................i........... 5500/9196
2019-10-16T12:31:08.9321818Z .................................................................................................... 5600/9196
2019-10-16T12:31:13.9281606Z .................................................................................................... 5700/9196
2019-10-16T12:31:13.9281606Z .................................................................................................... 5700/9196
2019-10-16T12:31:24.6141314Z .....................................................................................ii...i..ii..... 5800/9196
2019-10-16T12:31:50.8737742Z .................................................................................................... 6000/9196
2019-10-16T12:32:00.6801530Z .................................................................................................... 6100/9196
2019-10-16T12:32:05.7571765Z .................................................................................................... 6200/9196
2019-10-16T12:32:05.7571765Z .................................................................................................... 6200/9196
2019-10-16T12:32:19.4602811Z .......i..ii........................................................................................ 6300/9196
2019-10-16T12:32:38.6963755Z ...................................................................i................................ 6500/9196
2019-10-16T12:32:40.7687139Z .................................................................................................... 6600/9196
2019-10-16T12:32:43.1631357Z .........................................i.......................................................... 6700/9196
2019-10-16T12:32:46.9774701Z .................................................................................................... 6800/9196
---
2019-10-16T12:37:13.3839427Z  finished in 5.428
2019-10-16T12:37:13.4013128Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-16T12:37:13.5989558Z 
2019-10-16T12:37:13.5989825Z running 153 tests
2019-10-16T12:37:16.7607914Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-16T12:37:18.7443390Z i..ii.ii.............i.........iii.i.........ii......
2019-10-16T12:37:18.7445431Z 
2019-10-16T12:37:18.7450851Z  finished in 5.344
2019-10-16T12:37:18.7625277Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-16T12:37:18.9336528Z 
---
2019-10-16T12:37:20.9343344Z  finished in 2.171
2019-10-16T12:37:20.9523286Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-16T12:37:21.1186705Z 
2019-10-16T12:37:21.1187402Z running 9 tests
2019-10-16T12:37:21.1188299Z iiiiiiiii
2019-10-16T12:37:21.1188703Z 
2019-10-16T12:37:21.1188751Z  finished in 0.166
2019-10-16T12:37:21.1371994Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-16T12:37:21.3292222Z 
---
2019-10-16T12:37:38.9196800Z  finished in 17.782
2019-10-16T12:37:38.9422494Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-16T12:37:39.1414793Z 
2019-10-16T12:37:39.1415011Z running 123 tests
2019-10-16T12:38:02.8667540Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-16T12:38:07.4415410Z i.i.i......iii.i.....ii
2019-10-16T12:38:07.4415927Z 
2019-10-16T12:38:07.4419497Z  finished in 28.499
2019-10-16T12:38:07.4427019Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-16T12:38:07.4430014Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-10-16T12:39:04.9675601Z ---- [ui] ui-fulldeps/ast_stmt_expr_attr.rs stdout ----
2019-10-16T12:39:04.9675965Z 
2019-10-16T12:39:04.9676739Z error: test compilation failed although it shouldn't!
2019-10-16T12:39:04.9676954Z status: exit code: 1
2019-10-16T12:39:04.9678379Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/auxiliary"
2019-10-16T12:39:04.9686256Z ------------------------------------------
2019-10-16T12:39:04.9686302Z 
2019-10-16T12:39:04.9686618Z ------------------------------------------
2019-10-16T12:39:04.9686835Z stderr:
2019-10-16T12:39:04.9686835Z stderr:
2019-10-16T12:39:04.9687039Z ------------------------------------------
2019-10-16T12:39:04.9687271Z error[E0603]: module `attr` is private
2019-10-16T12:39:04.9688490Z    |
2019-10-16T12:39:04.9688561Z LL | use syntax::parse::parser::attr::*;
2019-10-16T12:39:04.9688608Z    |                            ^^^^
2019-10-16T12:39:04.9688639Z 
---
2019-10-16T12:39:04.9690361Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-16T12:39:04.9690424Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-16T12:39:04.9690457Z 
2019-10-16T12:39:04.9690483Z 
2019-10-16T12:39:04.9692593Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-16T12:39:04.9692887Z 
2019-10-16T12:39:04.9692929Z 
2019-10-16T12:39:04.9700129Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-16T12:39:04.9744812Z Build completed unsuccessfully in 1:07:30
2019-10-16T12:39:04.9744812Z Build completed unsuccessfully in 1:07:30
2019-10-16T12:39:04.9752023Z == clock drift check ==
2019-10-16T12:39:04.9824359Z   local time: Wed Oct 16 12:39:04 UTC 2019
2019-10-16T12:39:05.2618257Z   network time: Wed, 16 Oct 2019 12:39:05 GMT
2019-10-16T12:39:05.2618448Z == end clock drift check ==
2019-10-16T12:39:06.3378405Z ##[error]Bash exited with code '1'.
2019-10-16T12:39:06.3425142Z ##[section]Starting: Checkout
2019-10-16T12:39:06.3428025Z ==============================================================================
2019-10-16T12:39:06.3428102Z Task         : Get sources
2019-10-16T12:39:06.3428150Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
