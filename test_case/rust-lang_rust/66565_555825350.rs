plain
2019-11-20T02:47:15.1177377Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-20T02:47:15.9640812Z ##[command]git config gc.auto 0
2019-11-20T02:47:15.9644723Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-20T02:47:15.9647586Z ##[command]git config --get-all http.proxy
2019-11-20T02:47:15.9649889Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66565/merge:refs/remotes/pull/66565/merge
---
2019-11-20T03:39:21.2312672Z .................................................................................................... 1500/9256
2019-11-20T03:39:26.2014258Z .................................................................................................... 1600/9256
2019-11-20T03:39:33.6815821Z .................................................................................................... 1700/9256
2019-11-20T03:39:41.4208944Z ........i........................................................................................... 1800/9256
2019-11-20T03:39:46.9482136Z .............................................................................................iiiii.. 1900/9256
2019-11-20T03:40:05.7468505Z .................................................................................................... 2100/9256
2019-11-20T03:40:07.7513471Z .................................................................................................... 2200/9256
2019-11-20T03:40:09.7831936Z .................................................................................................... 2300/9256
2019-11-20T03:40:14.7829142Z .................................................................................................... 2400/9256
---
2019-11-20T03:42:45.6483239Z ..............................................................................................i..... 4700/9256
2019-11-20T03:42:51.1971216Z ..........i......................................................................................... 4800/9256
2019-11-20T03:42:59.9026986Z .................................................................................................... 4900/9256
2019-11-20T03:43:04.3943148Z .................................................................................................... 5000/9256
2019-11-20T03:43:13.1386781Z ..................................................................................................ii 5100/9256
2019-11-20T03:43:17.3962276Z .ii...........i..................................................................................... 5200/9256
2019-11-20T03:43:26.7016318Z .................................................................................................... 5400/9256
2019-11-20T03:43:36.1048195Z ................................................................................i................... 5500/9256
2019-11-20T03:43:42.7433480Z .................................................................................................... 5600/9256
2019-11-20T03:43:48.2340341Z .................................................................................................... 5700/9256
2019-11-20T03:43:48.2340341Z .................................................................................................... 5700/9256
2019-11-20T03:43:56.7999302Z ..................................................................ii...i...ii..........i............ 5800/9256
2019-11-20T03:44:16.4330130Z .................................................................................................... 6000/9256
2019-11-20T03:44:21.4473256Z .................................................................................................... 6100/9256
2019-11-20T03:44:21.4473256Z .................................................................................................... 6100/9256
2019-11-20T03:44:24.9644055Z .......................................................................................i..ii........ 6200/9256
2019-11-20T03:44:48.0210321Z .................................................................................................... 6400/9256
2019-11-20T03:44:52.1797973Z .......................................................i............................................ 6500/9256
2019-11-20T03:44:53.9718911Z .................................................................................................... 6600/9256
2019-11-20T03:44:56.1960056Z ............................................i....................................................... 6700/9256
---
2019-11-20T03:49:33.4035945Z  finished in 4.997
2019-11-20T03:49:33.4207025Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-20T03:49:33.6310623Z 
2019-11-20T03:49:33.6315688Z running 156 tests
2019-11-20T03:49:36.0682971Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/156
2019-11-20T03:49:37.7286565Z .i.i..iiii..............i.........iii.i.........ii......
2019-11-20T03:49:37.7287880Z 
2019-11-20T03:49:37.7291220Z  finished in 4.308
2019-11-20T03:49:37.7459332Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-20T03:49:37.8954126Z 
---
2019-11-20T03:49:39.6628246Z  finished in 1.917
2019-11-20T03:49:39.6792341Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-20T03:49:39.8281949Z 
2019-11-20T03:49:39.8283129Z running 9 tests
2019-11-20T03:49:39.8284295Z iiiiiiiii
2019-11-20T03:49:39.8284901Z 
2019-11-20T03:49:39.8285076Z  finished in 0.148
2019-11-20T03:49:39.8444817Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-20T03:49:40.0090418Z 
---
2019-11-20T03:49:56.5010857Z  finished in 16.656
2019-11-20T03:49:56.5175501Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-20T03:49:56.6657536Z 
2019-11-20T03:49:56.6657954Z running 123 tests
2019-11-20T03:50:16.3819299Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-11-20T03:50:20.3451953Z i.i.i......iii.i.....ii
2019-11-20T03:50:20.3453320Z 
2019-11-20T03:50:20.3457746Z  finished in 23.828
2019-11-20T03:50:20.3468000Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-20T03:50:20.3469409Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-11-20T03:50:59.7615505Z ---- [ui] ui-fulldeps/ast_stmt_expr_attr.rs stdout ----
2019-11-20T03:50:59.7615536Z 
2019-11-20T03:50:59.7615737Z error: test compilation failed although it shouldn't!
2019-11-20T03:50:59.7615799Z status: exit code: 1
2019-11-20T03:50:59.7616571Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/auxiliary"
2019-11-20T03:50:59.7617005Z ------------------------------------------
2019-11-20T03:50:59.7617032Z 
2019-11-20T03:50:59.7617217Z ------------------------------------------
2019-11-20T03:50:59.7617253Z stderr:
---
2019-11-20T03:50:59.7618553Z ---- [ui] ui-fulldeps/mod_dir_path_canonicalized.rs stdout ----
2019-11-20T03:50:59.7618578Z 
2019-11-20T03:50:59.7618749Z error: test compilation failed although it shouldn't!
2019-11-20T03:50:59.7618804Z status: exit code: 1
2019-11-20T03:50:59.7619520Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/mod_dir_path_canonicalized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/auxiliary"
2019-11-20T03:50:59.7619836Z ------------------------------------------
2019-11-20T03:50:59.7619889Z 
2019-11-20T03:50:59.7620062Z ------------------------------------------
2019-11-20T03:50:59.7620096Z stderr:
2019-11-20T03:50:59.7620096Z stderr:
2019-11-20T03:50:59.7620259Z ------------------------------------------
2019-11-20T03:50:59.7620317Z error[E0061]: this function takes 1 parameter but 2 parameters were supplied
2019-11-20T03:50:59.7620519Z   --> /checkout/src/test/ui-fulldeps/mod_dir_path_canonicalized.rs:27:25
2019-11-20T03:50:59.7620559Z    |
2019-11-20T03:50:59.7620617Z LL |     let parse_session = ParseSess::new(FilePathMapping::empty(), process_configure_mod);
2019-11-20T03:50:59.7620698Z 
2019-11-20T03:50:59.7620751Z error: aborting due to previous error
2019-11-20T03:50:59.7620774Z 
2019-11-20T03:50:59.7620971Z For more information about this error, try `rustc --explain E0061`.
---
2019-11-20T03:50:59.7621879Z ---- [ui] ui-fulldeps/pprust-expr-roundtrip.rs stdout ----
2019-11-20T03:50:59.7621925Z 
2019-11-20T03:50:59.7622109Z error: test compilation failed although it shouldn't!
2019-11-20T03:50:59.7622149Z status: exit code: 1
2019-11-20T03:50:59.7622789Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/auxiliary"
2019-11-20T03:50:59.7623071Z ------------------------------------------
2019-11-20T03:50:59.7623099Z 
2019-11-20T03:50:59.7623278Z ------------------------------------------
2019-11-20T03:50:59.7623314Z stderr:
2019-11-20T03:50:59.7623314Z stderr:
2019-11-20T03:50:59.7623507Z ------------------------------------------
2019-11-20T03:50:59.7623550Z error[E0061]: this function takes 1 parameter but 2 parameters were supplied
2019-11-20T03:50:59.7623755Z   --> /checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs:208:14
2019-11-20T03:50:59.7623814Z    |
2019-11-20T03:50:59.7623855Z LL |     let ps = ParseSess::new(FilePathMapping::empty(), process_configure_mod);
2019-11-20T03:50:59.7623949Z 
2019-11-20T03:50:59.7623994Z error: aborting due to previous error
2019-11-20T03:50:59.7624017Z 
2019-11-20T03:50:59.7624242Z For more information about this error, try `rustc --explain E0061`.
---
2019-11-20T03:50:59.7625630Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-20T03:50:59.7625758Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-20T03:50:59.7625813Z 
2019-11-20T03:50:59.7625857Z 
2019-11-20T03:50:59.7627638Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-20T03:50:59.7631030Z 
2019-11-20T03:50:59.7631870Z 
2019-11-20T03:50:59.7644336Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-20T03:50:59.7644429Z Build completed unsuccessfully in 0:57:56
2019-11-20T03:50:59.7644429Z Build completed unsuccessfully in 0:57:56
2019-11-20T03:50:59.7684505Z == clock drift check ==
2019-11-20T03:50:59.7700076Z   local time: Wed Nov 20 03:50:59 UTC 2019
2019-11-20T03:51:00.0495216Z   network time: Wed, 20 Nov 2019 03:51:00 GMT
2019-11-20T03:51:00.0496749Z == end clock drift check ==
2019-11-20T03:51:01.5954425Z 
2019-11-20T03:51:01.6042695Z ##[error]Bash exited with code '1'.
2019-11-20T03:51:01.6093129Z ##[section]Starting: Checkout
2019-11-20T03:51:01.6094741Z ==============================================================================
2019-11-20T03:51:01.6094816Z Task         : Get sources
2019-11-20T03:51:01.6094853Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
