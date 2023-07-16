plain
2019-11-20T12:17:53.2954490Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-20T12:17:53.3148060Z ##[command]git config gc.auto 0
2019-11-20T12:17:54.0428414Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-20T12:17:54.0435043Z ##[command]git config --get-all http.proxy
2019-11-20T12:17:54.0441571Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66565/merge:refs/remotes/pull/66565/merge
---
2019-11-20T13:14:13.1938036Z .................................................................................................... 1500/9256
2019-11-20T13:14:19.1433821Z .................................................................................................... 1600/9256
2019-11-20T13:14:27.5693353Z .................................................................................................... 1700/9256
2019-11-20T13:14:36.2540802Z ........i........................................................................................... 1800/9256
2019-11-20T13:14:42.6448505Z .............................................................................................iiiii.. 1900/9256
2019-11-20T13:15:03.0167784Z .................................................................................................... 2100/9256
2019-11-20T13:15:05.3609108Z .................................................................................................... 2200/9256
2019-11-20T13:15:07.7999389Z .................................................................................................... 2300/9256
2019-11-20T13:15:13.6318200Z .................................................................................................... 2400/9256
---
2019-11-20T13:18:02.9783128Z ..............................................................................................i..... 4700/9256
2019-11-20T13:18:09.2748054Z ..........i......................................................................................... 4800/9256
2019-11-20T13:18:18.7834904Z .................................................................................................... 4900/9256
2019-11-20T13:18:23.8434125Z .................................................................................................... 5000/9256
2019-11-20T13:18:33.8384062Z ..................................................................................................ii 5100/9256
2019-11-20T13:18:38.7328879Z .ii...........i..................................................................................... 5200/9256
2019-11-20T13:18:48.9908875Z .................................................................................................... 5400/9256
2019-11-20T13:18:59.1098258Z ................................................................................i................... 5500/9256
2019-11-20T13:19:06.6390860Z .................................................................................................... 5600/9256
2019-11-20T13:19:12.7191155Z .................................................................................................... 5700/9256
2019-11-20T13:19:12.7191155Z .................................................................................................... 5700/9256
2019-11-20T13:19:22.6420867Z ..................................................................ii...i..ii...........i............ 5800/9256
2019-11-20T13:19:44.1462592Z .................................................................................................... 6000/9256
2019-11-20T13:19:52.5490244Z .................................................................................................... 6100/9256
2019-11-20T13:19:52.5490244Z .................................................................................................... 6100/9256
2019-11-20T13:20:00.0652122Z .......................................................................................i..ii........ 6200/9256
2019-11-20T13:20:26.3353953Z .................................................................................................... 6400/9256
2019-11-20T13:20:31.3781364Z .......................................................i............................................ 6500/9256
2019-11-20T13:20:33.5903227Z .................................................................................................... 6600/9256
2019-11-20T13:20:35.9865678Z ............................................i....................................................... 6700/9256
---
2019-11-20T13:25:40.6364652Z  finished in 5.666
2019-11-20T13:25:40.6364935Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-20T13:25:40.6364989Z 
2019-11-20T13:25:40.6365026Z running 156 tests
2019-11-20T13:25:42.8478797Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/156
2019-11-20T13:25:44.7352485Z .i.i..iiii..............i.........iii.i.........ii......
2019-11-20T13:25:44.7354178Z 
2019-11-20T13:25:44.7358947Z  finished in 4.901
2019-11-20T13:25:44.7551960Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-20T13:25:44.9225487Z 
---
2019-11-20T13:25:46.8844556Z  finished in 2.129
2019-11-20T13:25:46.9012877Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-20T13:25:47.6453940Z 
2019-11-20T13:25:47.6454055Z running 9 tests
2019-11-20T13:25:47.6454775Z iiiiiiiii
2019-11-20T13:25:47.6455090Z 
2019-11-20T13:25:47.6455147Z  finished in 0.147
2019-11-20T13:25:47.6455564Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-20T13:25:47.6455596Z 
---
2019-11-20T13:26:06.4655397Z  finished in 19.399
2019-11-20T13:26:06.4916019Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-20T13:26:06.6749446Z 
2019-11-20T13:26:06.6749636Z running 123 tests
2019-11-20T13:26:29.7331456Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-11-20T13:26:34.1865818Z i.i.i......iii.i.....ii
2019-11-20T13:26:34.1866236Z 
2019-11-20T13:26:34.1869938Z  finished in 27.695
2019-11-20T13:26:34.1878437Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-20T13:26:34.1878708Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-11-20T13:27:30.6337238Z ---- [ui] ui-fulldeps/ast_stmt_expr_attr.rs stdout ----
2019-11-20T13:27:30.6337385Z 
2019-11-20T13:27:30.6337710Z error: test compilation failed although it shouldn't!
2019-11-20T13:27:30.6337852Z status: exit code: 1
2019-11-20T13:27:30.6338604Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/auxiliary"
2019-11-20T13:27:30.6339099Z ------------------------------------------
2019-11-20T13:27:30.6339226Z 
2019-11-20T13:27:30.6339509Z ------------------------------------------
2019-11-20T13:27:30.6339664Z stderr:
---
2019-11-20T13:27:30.6344587Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-20T13:27:30.6344838Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-20T13:27:30.6344966Z 
2019-11-20T13:27:30.6345081Z 
2019-11-20T13:27:30.6348370Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-20T13:27:30.6348852Z 
2019-11-20T13:27:30.6348989Z 
2019-11-20T13:27:30.6360001Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-20T13:27:30.6360175Z Build completed unsuccessfully in 1:03:52
2019-11-20T13:27:30.6360175Z Build completed unsuccessfully in 1:03:52
2019-11-20T13:27:30.6414486Z == clock drift check ==
2019-11-20T13:27:30.6430242Z   local time: Wed Nov 20 13:27:30 UTC 2019
2019-11-20T13:27:31.6567744Z   network time: Wed, 20 Nov 2019 13:27:30 GMT
2019-11-20T13:27:31.6568525Z == end clock drift check ==
2019-11-20T13:27:31.8829433Z 
2019-11-20T13:27:31.8929153Z ##[error]Bash exited with code '1'.
2019-11-20T13:27:31.8970418Z ##[section]Starting: Checkout
2019-11-20T13:27:31.8974483Z ==============================================================================
2019-11-20T13:27:31.8974543Z Task         : Get sources
2019-11-20T13:27:31.8974608Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
