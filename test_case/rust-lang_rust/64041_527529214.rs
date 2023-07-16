plain
2019-09-03T14:48:59.1481534Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-03T14:48:59.1682396Z ##[command]git config gc.auto 0
2019-09-03T14:48:59.1781705Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-03T14:48:59.1877016Z ##[command]git config --get-all http.proxy
2019-09-03T14:48:59.2049417Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64041/merge:refs/remotes/pull/64041/merge
---
2019-09-03T15:57:23.3632482Z .................................................................................................... 1500/8987
2019-09-03T15:57:29.5121429Z .................................................................................................... 1600/8987
2019-09-03T15:57:43.4063870Z .................................................i...............i.................................. 1700/8987
2019-09-03T15:57:52.4399049Z .................................................................................................... 1800/8987
2019-09-03T15:58:07.9585630Z ........................................iiiii....................................................... 1900/8987
2019-09-03T15:58:19.8890421Z .................................................................................................... 2100/8987
2019-09-03T15:58:22.7116968Z .................................................................................................... 2200/8987
2019-09-03T15:58:27.1509872Z .................................................................................................... 2300/8987
2019-09-03T15:58:35.4390602Z .................................................................................................... 2400/8987
---
2019-09-03T16:01:49.9027833Z ...........................i...............i........................................................ 4700/8987
2019-09-03T16:02:02.8540455Z .................................................................................................... 4800/8987
2019-09-03T16:02:09.6251326Z .................................................................................................... 4900/8987
2019-09-03T16:02:21.3555299Z .................................................................................................... 5000/8987
2019-09-03T16:02:27.5489696Z ........ii.ii....................................................................................... 5100/8987
2019-09-03T16:02:41.8338201Z .................................................................................................... 5300/8987
2019-09-03T16:02:50.7971346Z .......................................................................i............................ 5400/8987
2019-09-03T16:02:58.8578066Z .................................................................................................... 5500/8987
2019-09-03T16:03:06.0982374Z .................................................................................................... 5600/8987
2019-09-03T16:03:06.0982374Z .................................................................................................... 5600/8987
2019-09-03T16:03:17.4657138Z .................................................................ii...i..ii...........i............. 5700/8987
2019-09-03T16:03:45.0133522Z .................................................................................................... 5900/8987
2019-09-03T16:03:55.6498769Z .................................................................................................... 6000/8987
2019-09-03T16:03:55.6498769Z .................................................................................................... 6000/8987
2019-09-03T16:04:05.8713780Z ...................................................................i..ii............................ 6100/8987
2019-09-03T16:04:37.2074290Z .................................................................................................... 6300/8987
2019-09-03T16:04:39.5646042Z ......................i............................................................................. 6400/8987
2019-09-03T16:04:41.9526245Z ..............................................................................................i..... 6500/8987
2019-09-03T16:04:44.9751968Z .................................................................................................... 6600/8987
---
2019-09-03T16:09:59.1274110Z  finished in 22.791
2019-09-03T16:09:59.1478849Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-03T16:09:59.3765884Z 
2019-09-03T16:09:59.3766891Z running 149 tests
2019-09-03T16:10:02.9973778Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/149
2019-09-03T16:10:05.1421600Z ..iiii..............i.........iii.i......ii......
2019-09-03T16:10:05.1422262Z 
2019-09-03T16:10:05.1503839Z  finished in 5.995
2019-09-03T16:10:05.1622705Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-03T16:10:05.3487752Z 
---
2019-09-03T16:10:07.6073483Z  finished in 2.445
2019-09-03T16:10:07.6294518Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-03T16:10:07.7994812Z 
2019-09-03T16:10:07.7995097Z running 9 tests
2019-09-03T16:10:07.7997473Z iiiiiiiii
2019-09-03T16:10:07.7997893Z 
2019-09-03T16:10:07.7997972Z  finished in 0.170
2019-09-03T16:10:07.8196549Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-03T16:10:08.0170031Z 
---
2019-09-03T16:10:27.9342193Z  finished in 20.114
2019-09-03T16:10:27.9617872Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-03T16:10:28.1865507Z 
2019-09-03T16:10:28.1865792Z running 123 tests
2019-09-03T16:10:55.0874659Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-03T16:11:00.2415393Z i.i.i......iii.i.....ii
2019-09-03T16:11:00.2418806Z 
2019-09-03T16:11:00.2419054Z  finished in 32.280
2019-09-03T16:11:00.2431369Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-03T16:11:00.2432029Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-03T16:12:07.7185793Z failures:
2019-09-03T16:12:07.7185830Z 
2019-09-03T16:12:07.7190989Z ---- [ui] ui-fulldeps/roman-numerals-macro.rs stdout ----
2019-09-03T16:12:07.7191141Z 
2019-09-03T16:12:07.7191574Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" failed to compile: 
2019-09-03T16:12:07.7191640Z status: exit code: 1
2019-09-03T16:12:07.7192515Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary"
2019-09-03T16:12:07.7192917Z ------------------------------------------
2019-09-03T16:12:07.7192957Z 
2019-09-03T16:12:07.7193274Z ------------------------------------------
2019-09-03T16:12:07.7193346Z stderr:
2019-09-03T16:12:07.7193346Z stderr:
2019-09-03T16:12:07.7193596Z ------------------------------------------
2019-09-03T16:12:07.7193652Z error[E0308]: mismatched types
2019-09-03T16:12:07.7193944Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:67:30
2019-09-03T16:12:07.7194003Z    |
2019-09-03T16:12:07.7194067Z LL |     reg.register_macro("rn", expand_rn);
2019-09-03T16:12:07.7194146Z    |                              ^^^^^^^^^ expected struct `syntax::tokenstream::TokenStream`, found reference
2019-09-03T16:12:07.7194202Z    |
2019-09-03T16:12:07.7194615Z    = note: expected type `for<'cx, 'r> fn(&'cx mut syntax::ext::base::ExtCtxt<'r>, syntax_pos::Span, syntax::tokenstream::TokenStream) -> std::boxed::Box<(dyn syntax::ext::base::MacResult + 'cx)>`
2019-09-03T16:12:07.7195299Z               found type `for<'r, 's, 't0> fn(&'r mut syntax::ext::base::ExtCtxt<'s>, syntax_pos::Span, &'t0 [syntax::tokenstream::TokenTree]) -> std::boxed::Box<(dyn syntax::ext::base::MacResult + 'static)> {expand_rn}`
2019-09-03T16:12:07.7195430Z error: aborting due to previous error
2019-09-03T16:12:07.7195485Z 
2019-09-03T16:12:07.7195826Z For more information about this error, try `rustc --explain E0308`.
2019-09-03T16:12:07.7195868Z 
---
2019-09-03T16:12:07.7208385Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-03T16:12:07.7208539Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-03T16:12:07.7212011Z 
2019-09-03T16:12:07.7217011Z 
2019-09-03T16:12:07.7284378Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-03T16:12:07.7285038Z 
2019-09-03T16:12:07.7285070Z 
2019-09-03T16:12:07.7285137Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-03T16:12:07.7285195Z Build completed unsuccessfully in 1:15:31
2019-09-03T16:12:07.7285195Z Build completed unsuccessfully in 1:15:31
2019-09-03T16:12:07.7290626Z == clock drift check ==
2019-09-03T16:12:07.7308519Z   local time: Tue Sep  3 16:12:07 UTC 2019
2019-09-03T16:12:07.8845142Z   network time: Tue, 03 Sep 2019 16:12:07 GMT
2019-09-03T16:12:07.8845253Z == end clock drift check ==
2019-09-03T16:12:08.4900556Z ##[error]Bash exited with code '1'.
2019-09-03T16:12:08.4947301Z ##[section]Starting: Checkout
2019-09-03T16:12:08.4950058Z ==============================================================================
2019-09-03T16:12:08.4950122Z Task         : Get sources
2019-09-03T16:12:08.4950175Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
