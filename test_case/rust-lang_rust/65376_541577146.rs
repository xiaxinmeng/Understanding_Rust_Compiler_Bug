plain
2019-10-14T08:09:15.4886780Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-14T08:09:15.5020689Z ##[command]git config gc.auto 0
2019-10-14T08:09:15.5098018Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-14T08:09:15.5150999Z ##[command]git config --get-all http.proxy
2019-10-14T08:09:15.5313254Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65376/merge:refs/remotes/pull/65376/merge
---
2019-10-14T09:16:08.8970132Z .................................................................................................... 1600/9173
2019-10-14T09:16:16.0707969Z .................................................................................................... 1700/9173
2019-10-14T09:16:29.6364940Z ......................i...............i............................................................. 1800/9173
2019-10-14T09:16:37.6602481Z .................................................................................................... 1900/9173
2019-10-14T09:16:53.8725300Z .............iiiii.................................................................................. 2000/9173
2019-10-14T09:17:05.5579939Z .................................................................................................... 2200/9173
2019-10-14T09:17:08.4763486Z .................................................................................................... 2300/9173
2019-10-14T09:17:14.7167582Z .................................................................................................... 2400/9173
2019-10-14T09:17:38.9534996Z .................................................................................................... 2500/9173
---
2019-10-14T09:20:55.3899969Z ....................i...............i............................................................... 4800/9173
2019-10-14T09:21:08.1496296Z .................................................................................................... 4900/9173
2019-10-14T09:21:15.2015624Z .................................................................................................... 5000/9173
2019-10-14T09:21:26.9505683Z .................................................................................................... 5100/9173
2019-10-14T09:21:33.8860935Z ....................ii.ii........................................................................... 5200/9173
2019-10-14T09:21:45.7927122Z .................................................................................................... 5400/9173
2019-10-14T09:21:57.0650371Z ......................................................................................i............. 5500/9173
2019-10-14T09:22:06.1506757Z .................................................................................................... 5600/9173
2019-10-14T09:22:12.1153929Z .................................................................................................... 5700/9173
2019-10-14T09:22:12.1153929Z .................................................................................................... 5700/9173
2019-10-14T09:22:23.8421850Z ...................................................................................ii...i..ii....... 5800/9173
2019-10-14T09:22:51.7578624Z .................................................................................................... 6000/9173
2019-10-14T09:23:02.6990313Z .................................................................................................... 6100/9173
2019-10-14T09:23:02.6990313Z .................................................................................................... 6100/9173
2019-10-14T09:23:12.3007659Z ..........................................................................................i..ii..... 6200/9173
2019-10-14T09:23:43.8044622Z .................................................................................................... 6400/9173
2019-10-14T09:23:49.5359765Z ...................................................i................................................ 6500/9173
2019-10-14T09:23:52.0431056Z .................................................................................................... 6600/9173
2019-10-14T09:23:54.7070031Z ........................i........................................................................... 6700/9173
---
2019-10-14T09:29:03.5745115Z  finished in 6.172
2019-10-14T09:29:03.6002398Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-14T09:29:03.7761668Z 
2019-10-14T09:29:03.7761976Z running 153 tests
2019-10-14T09:29:07.4768681Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-14T09:29:09.7327008Z i..iiii..............i.........iii.i.........ii......
2019-10-14T09:29:09.7327591Z 
2019-10-14T09:29:09.7330984Z  finished in 6.132
2019-10-14T09:29:09.7517595Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-14T09:29:09.9267436Z 
---
2019-10-14T09:29:12.2478635Z  finished in 2.496
2019-10-14T09:29:12.2685664Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-14T09:29:12.4319845Z 
2019-10-14T09:29:12.4320130Z running 9 tests
2019-10-14T09:29:12.4321023Z iiiiiiiii
2019-10-14T09:29:12.4321384Z 
2019-10-14T09:29:12.4321427Z  finished in 0.163
2019-10-14T09:29:12.4524836Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-14T09:29:12.6344754Z 
---
2019-10-14T09:29:32.8052385Z  finished in 20.353
2019-10-14T09:29:32.8317812Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-14T09:29:33.0244559Z 
2019-10-14T09:29:33.0244916Z running 123 tests
2019-10-14T09:30:00.2405652Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-14T09:30:05.5766172Z i.i.i......iii.i.....ii
2019-10-14T09:30:05.5771349Z 
2019-10-14T09:30:05.5777296Z  finished in 32.746
2019-10-14T09:30:05.5786404Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-14T09:30:05.5787210Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-10-14T09:30:55.2049556Z ---- [ui] ui-fulldeps/ast_stmt_expr_attr.rs stdout ----
2019-10-14T09:30:55.2049799Z 
2019-10-14T09:30:55.2050235Z error: test compilation failed although it shouldn't!
2019-10-14T09:30:55.2050448Z status: exit code: 1
2019-10-14T09:30:55.2051695Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/ast_stmt_expr_attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/ast_stmt_expr_attr/auxiliary"
2019-10-14T09:30:55.2052511Z ------------------------------------------
2019-10-14T09:30:55.2052692Z 
2019-10-14T09:30:55.2053096Z ------------------------------------------
2019-10-14T09:30:55.2053293Z stderr:
2019-10-14T09:30:55.2053293Z stderr:
2019-10-14T09:30:55.2054070Z ------------------------------------------
2019-10-14T09:30:55.2054316Z error[E0603]: struct `ParseSess` is private
2019-10-14T09:30:55.2054921Z    |
2019-10-14T09:30:55.2054921Z    |
2019-10-14T09:30:55.2055102Z LL | use syntax::parse::{ParseSess, PResult};
2019-10-14T09:30:55.2055368Z 
2019-10-14T09:30:55.2055524Z error: aborting due to previous error
2019-10-14T09:30:55.2055647Z 
2019-10-14T09:30:55.2056039Z For more information about this error, try `rustc --explain E0603`.
---
2019-10-14T09:30:55.2057335Z ---- [ui] ui-fulldeps/mod_dir_path_canonicalized.rs stdout ----
2019-10-14T09:30:55.2057504Z 
2019-10-14T09:30:55.2057875Z error: test compilation failed although it shouldn't!
2019-10-14T09:30:55.2058087Z status: exit code: 1
2019-10-14T09:30:55.2058989Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/mod_dir_path_canonicalized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/auxiliary"
2019-10-14T09:30:55.2059942Z ------------------------------------------
2019-10-14T09:30:55.2060147Z 
2019-10-14T09:30:55.2060522Z ------------------------------------------
2019-10-14T09:30:55.2060709Z stderr:
2019-10-14T09:30:55.2060709Z stderr:
2019-10-14T09:30:55.2061096Z ------------------------------------------
2019-10-14T09:30:55.2061291Z error[E0603]: struct `ParseSess` is private
2019-10-14T09:30:55.2061905Z    |
2019-10-14T09:30:55.2061905Z    |
2019-10-14T09:30:55.2062053Z LL | use syntax::parse::{self, ParseSess};
2019-10-14T09:30:55.2062399Z 
2019-10-14T09:30:55.2062592Z error: aborting due to previous error
2019-10-14T09:30:55.2062793Z 
2019-10-14T09:30:55.2063284Z For more information about this error, try `rustc --explain E0603`.
---
2019-10-14T09:30:55.2065362Z ---- [ui] ui-fulldeps/pprust-expr-roundtrip.rs stdout ----
2019-10-14T09:30:55.2065395Z 
2019-10-14T09:30:55.2065644Z error: test compilation failed although it shouldn't!
2019-10-14T09:30:55.2065694Z status: exit code: 1
2019-10-14T09:30:55.2066408Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/auxiliary"
2019-10-14T09:30:55.2066736Z ------------------------------------------
2019-10-14T09:30:55.2066795Z 
2019-10-14T09:30:55.2067167Z ------------------------------------------
2019-10-14T09:30:55.2067229Z stderr:
2019-10-14T09:30:55.2067229Z stderr:
2019-10-14T09:30:55.2067491Z ------------------------------------------
2019-10-14T09:30:55.2067565Z error[E0603]: struct `ParseSess` is private
2019-10-14T09:30:55.2067865Z    |
2019-10-14T09:30:55.2067865Z    |
2019-10-14T09:30:55.2067929Z LL | use syntax::parse::{self, ParseSess};
2019-10-14T09:30:55.2068003Z 
2019-10-14T09:30:55.2068061Z error: aborting due to previous error
2019-10-14T09:30:55.2068090Z 
2019-10-14T09:30:55.2068339Z For more information about this error, try `rustc --explain E0603`.
---
2019-10-14T09:30:55.2069751Z test result: FAILED. 66 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
2019-10-14T09:30:55.2069789Z 
2019-10-14T09:30:55.2069816Z 
2019-10-14T09:30:55.2069840Z 
2019-10-14T09:30:55.2071323Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-14T09:30:55.2071694Z 
2019-10-14T09:30:55.2071722Z 
2019-10-14T09:30:55.2072030Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-14T09:30:55.2072110Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-14T09:30:55.2072110Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-14T09:30:55.2075031Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-14T09:30:55.2075273Z Build completed unsuccessfully in 1:14:24
2019-10-14T09:30:55.2106366Z == clock drift check ==
2019-10-14T09:30:55.2129893Z   local time: Mon Oct 14 09:30:55 UTC 2019
2019-10-14T09:30:55.3617293Z   network time: Mon, 14 Oct 2019 09:30:55 GMT
2019-10-14T09:30:55.3617642Z == end clock drift check ==
2019-10-14T09:30:55.9440403Z ##[error]Bash exited with code '1'.
2019-10-14T09:30:55.9490151Z ##[section]Starting: Checkout
2019-10-14T09:30:55.9491980Z ==============================================================================
2019-10-14T09:30:55.9492053Z Task         : Get sources
2019-10-14T09:30:55.9492100Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
