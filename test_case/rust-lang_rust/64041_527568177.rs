plain
2019-09-03T16:31:34.1600591Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-03T16:31:34.1808063Z ##[command]git config gc.auto 0
2019-09-03T16:31:34.1898343Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-03T16:31:34.1952933Z ##[command]git config --get-all http.proxy
2019-09-03T16:31:34.2110086Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64041/merge:refs/remotes/pull/64041/merge
---
2019-09-03T17:38:30.2561292Z .................................................................................................... 1500/8987
2019-09-03T17:38:36.4479777Z .................................................................................................... 1600/8987
2019-09-03T17:38:50.3296963Z .................................................i...............i.................................. 1700/8987
2019-09-03T17:38:59.4130256Z .................................................................................................... 1800/8987
2019-09-03T17:39:15.6694136Z ........................................iiiii....................................................... 1900/8987
2019-09-03T17:39:27.6890073Z .................................................................................................... 2100/8987
2019-09-03T17:39:30.5479487Z .................................................................................................... 2200/8987
2019-09-03T17:39:35.0678300Z .................................................................................................... 2300/8987
2019-09-03T17:39:43.4128122Z .................................................................................................... 2400/8987
---
2019-09-03T17:43:00.5865159Z ...........................i...............i........................................................ 4700/8987
2019-09-03T17:43:13.6742869Z .................................................................................................... 4800/8987
2019-09-03T17:43:20.4836500Z .................................................................................................... 4900/8987
2019-09-03T17:43:32.2986621Z .................................................................................................... 5000/8987
2019-09-03T17:43:38.6325736Z ........ii.ii....................................................................................... 5100/8987
2019-09-03T17:43:53.1660593Z .................................................................................................... 5300/8987
2019-09-03T17:44:02.5829701Z .......................................................................i............................ 5400/8987
2019-09-03T17:44:10.7318317Z .................................................................................................... 5500/8987
2019-09-03T17:44:18.0490725Z .................................................................................................... 5600/8987
2019-09-03T17:44:18.0490725Z .................................................................................................... 5600/8987
2019-09-03T17:44:29.8093452Z .................................................................ii...i..ii............i............ 5700/8987
2019-09-03T17:44:58.1210253Z .................................................................................................... 5900/8987
2019-09-03T17:45:08.7536706Z .................................................................................................... 6000/8987
2019-09-03T17:45:08.7536706Z .................................................................................................... 6000/8987
2019-09-03T17:45:19.0906669Z ...................................................................i..ii............................ 6100/8987
2019-09-03T17:45:50.8740422Z .................................................................................................... 6300/8987
2019-09-03T17:45:53.2365948Z ......................i............................................................................. 6400/8987
2019-09-03T17:45:55.6661600Z ..............................................................................................i..... 6500/8987
2019-09-03T17:45:58.6474930Z .................................................................................................... 6600/8987
---
2019-09-03T17:51:11.0383811Z  finished in 20.715
2019-09-03T17:51:11.0579303Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-03T17:51:11.2664931Z 
2019-09-03T17:51:11.2666055Z running 149 tests
2019-09-03T17:51:14.7543203Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/149
2019-09-03T17:51:16.8995991Z ..iiii..............i.........iii.i......ii......
2019-09-03T17:51:16.8997461Z 
2019-09-03T17:51:16.9003197Z  finished in 5.842
2019-09-03T17:51:16.9208840Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-03T17:51:17.1075035Z 
---
2019-09-03T17:51:19.4239184Z  finished in 2.502
2019-09-03T17:51:19.4456413Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-03T17:51:19.6208607Z 
2019-09-03T17:51:19.6209008Z running 9 tests
2019-09-03T17:51:19.6209892Z iiiiiiiii
2019-09-03T17:51:19.6210298Z 
2019-09-03T17:51:19.6210343Z  finished in 0.175
2019-09-03T17:51:19.6428528Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-03T17:51:19.8403026Z 
---
2019-09-03T17:51:39.5451095Z  finished in 19.902
2019-09-03T17:51:39.5685322Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-03T17:51:39.7669521Z 
2019-09-03T17:51:39.7669925Z running 123 tests
2019-09-03T17:52:05.2391071Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-03T17:52:10.2483472Z i.i.i......iii.i.....ii
2019-09-03T17:52:10.2485203Z 
2019-09-03T17:52:10.2486305Z  finished in 30.680
2019-09-03T17:52:10.2497735Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-03T17:52:10.2499881Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-03T17:53:18.4188275Z failures:
2019-09-03T17:53:18.4188308Z 
2019-09-03T17:53:18.4188631Z ---- [ui] ui-fulldeps/roman-numerals-macro.rs stdout ----
2019-09-03T17:53:18.4188671Z 
2019-09-03T17:53:18.4188973Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" failed to compile: 
2019-09-03T17:53:18.4189050Z status: exit code: 1
2019-09-03T17:53:18.4189984Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary"
2019-09-03T17:53:18.4190805Z ------------------------------------------
2019-09-03T17:53:18.4190837Z 
2019-09-03T17:53:18.4191184Z ------------------------------------------
2019-09-03T17:53:18.4191240Z stderr:
2019-09-03T17:53:18.4191240Z stderr:
2019-09-03T17:53:18.4191478Z ------------------------------------------
2019-09-03T17:53:18.4191523Z error[E0308]: mismatched types
2019-09-03T17:53:18.4191776Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:67:30
2019-09-03T17:53:18.4191823Z    |
2019-09-03T17:53:18.4191867Z LL |     reg.register_macro("rn", expand_rn);
2019-09-03T17:53:18.4191936Z    |                              ^^^^^^^^^ expected struct `syntax::tokenstream::TokenStream`, found reference
2019-09-03T17:53:18.4191981Z    |
2019-09-03T17:53:18.4192336Z    = note: expected type `for<'cx, 'r> fn(&'cx mut syntax::ext::base::ExtCtxt<'r>, syntax_pos::Span, syntax::tokenstream::TokenStream) -> std::boxed::Box<(dyn syntax::ext::base::MacResult + 'cx)>`
2019-09-03T17:53:18.4192736Z               found type `for<'r, 's, 't0> fn(&'r mut syntax::ext::base::ExtCtxt<'s>, syntax_pos::Span, &'t0 syntax::tokenstream::TokenStream) -> std::boxed::Box<(dyn syntax::ext::base::MacResult + 'static)> {expand_rn}`
2019-09-03T17:53:18.4192839Z error: aborting due to previous error
2019-09-03T17:53:18.4192866Z 
2019-09-03T17:53:18.4193099Z For more information about this error, try `rustc --explain E0308`.
2019-09-03T17:53:18.4193131Z 
---
2019-09-03T17:53:18.4194264Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-03T17:53:18.4194317Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-03T17:53:18.4201644Z 
2019-09-03T17:53:18.4201891Z 
2019-09-03T17:53:18.4204891Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-03T17:53:18.4205181Z 
2019-09-03T17:53:18.4205213Z 
2019-09-03T17:53:18.4214527Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-03T17:53:18.4214634Z Build completed unsuccessfully in 1:14:17
2019-09-03T17:53:18.4214634Z Build completed unsuccessfully in 1:14:17
2019-09-03T17:53:18.4271531Z == clock drift check ==
2019-09-03T17:53:18.4291462Z   local time: Tue Sep  3 17:53:18 UTC 2019
2019-09-03T17:53:18.7107273Z   network time: Tue, 03 Sep 2019 17:53:18 GMT
2019-09-03T17:53:18.7110630Z == end clock drift check ==
2019-09-03T17:53:19.3193446Z ##[error]Bash exited with code '1'.
2019-09-03T17:53:19.3272495Z ##[section]Starting: Checkout
2019-09-03T17:53:19.3274436Z ==============================================================================
2019-09-03T17:53:19.3274495Z Task         : Get sources
2019-09-03T17:53:19.3274543Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
