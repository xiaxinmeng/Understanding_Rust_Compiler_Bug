plain
2020-01-01T20:38:39.8194506Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-01T20:38:39.8382425Z ##[command]git config gc.auto 0
2020-01-01T20:38:39.8462128Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-01T20:38:39.8524763Z ##[command]git config --get-all http.proxy
2020-01-01T20:38:40.5954105Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67786/merge:refs/remotes/pull/67786/merge
---
2020-01-01T21:36:34.5387630Z .................................................................................................... 1500/9467
2020-01-01T21:36:40.2305878Z .................................................................................................... 1600/9467
2020-01-01T21:36:44.9928991Z .................................................................................................... 1700/9467
2020-01-01T21:36:54.2265568Z .................................................................................................... 1800/9467
2020-01-01T21:37:02.1452981Z i................................................................................................... 1900/9467
2020-01-01T21:37:08.6389728Z ......................................................................................iiiii......... 2000/9467
2020-01-01T21:37:29.8719264Z .................................................................................................... 2200/9467
2020-01-01T21:37:32.2039205Z .................................................................................................... 2300/9467
2020-01-01T21:37:34.5730154Z .................................................................................................... 2400/9467
2020-01-01T21:37:40.4518904Z .................................................................................................... 2500/9467
---
2020-01-01T21:40:32.1216361Z .................i...............i.................................................................. 4900/9467
2020-01-01T21:40:42.0486479Z .................................................................................................... 5000/9467
2020-01-01T21:40:47.6057956Z ..............................................................i..................................... 5100/9467
2020-01-01T21:40:55.6494210Z .................................................................................................... 5200/9467
2020-01-01T21:41:03.2062285Z .............................ii.ii...........i...................................................... 5300/9467
2020-01-01T21:41:12.2345999Z .................................................................................................... 5500/9467
2020-01-01T21:41:21.9530156Z .................................................................................................... 5600/9467
2020-01-01T21:41:28.6772510Z ............i....................................................................................... 5700/9467
2020-01-01T21:41:34.6987194Z .................................................................................................... 5800/9467
2020-01-01T21:41:34.6987194Z .................................................................................................... 5800/9467
2020-01-01T21:41:45.2399817Z .................................................................................................... 5900/9467
2020-01-01T21:41:56.9018975Z ii...i..ii...........i.............................................................................. 6000/9467
2020-01-01T21:42:13.9518246Z .................................................................................................... 6200/9467
2020-01-01T21:42:20.7106758Z .................................................................................................... 6300/9467
2020-01-01T21:42:20.7106758Z .................................................................................................... 6300/9467
2020-01-01T21:42:36.2503316Z ...........................i..ii.................................................................... 6400/9467
2020-01-01T21:42:55.2437266Z .................................................................................................... 6600/9467
2020-01-01T21:42:57.2451753Z ..i................................................................................................. 6700/9467
2020-01-01T21:42:59.4543476Z .................................................................................................... 6800/9467
2020-01-01T21:43:01.8697626Z ..i................................................................................................. 6900/9467
---
2020-01-01T21:44:34.4816794Z .................................................................................................... 7500/9467
2020-01-01T21:44:39.0473992Z .................................................................................................... 7600/9467
2020-01-01T21:44:44.1961972Z .................................................................................................... 7700/9467
2020-01-01T21:44:53.7042330Z .................................................................................................... 7800/9467
2020-01-01T21:45:00.9238020Z .....................................iiii........................................................... 7900/9467
2020-01-01T21:45:15.0439834Z .................................................................................................... 8100/9467
2020-01-01T21:45:23.1107806Z .................................................................................................... 8200/9467
2020-01-01T21:45:36.6498005Z .................................................................................................... 8300/9467
2020-01-01T21:45:44.2443310Z .................................................................................................... 8400/9467
---
2020-01-01T21:48:00.3154150Z  finished in 6.563
2020-01-01T21:48:00.3347801Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-01T21:48:00.5313409Z 
2020-01-01T21:48:00.5314166Z running 166 tests
2020-01-01T21:48:03.4487716Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-01T21:48:05.4340086Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-01T21:48:05.4340563Z 
2020-01-01T21:48:05.4347485Z  finished in 5.100
2020-01-01T21:48:05.4525504Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-01T21:48:05.6102857Z 
---
2020-01-01T21:48:07.4568122Z  finished in 2.004
2020-01-01T21:48:07.4769003Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-01T21:48:07.6423842Z 
2020-01-01T21:48:07.6424686Z running 9 tests
2020-01-01T21:48:07.6426492Z iiiiiiiii
2020-01-01T21:48:07.6427586Z 
2020-01-01T21:48:07.6429377Z  finished in 0.165
2020-01-01T21:48:07.6622365Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-01T21:48:07.8174283Z 
---
2020-01-01T21:48:26.2390853Z  finished in 18.577
2020-01-01T21:48:27.1930731Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-01T21:48:27.1930802Z 
2020-01-01T21:48:27.1930857Z running 124 tests
2020-01-01T21:48:49.7277554Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2020-01-01T21:48:54.6960914Z .i.iii.....iiiiii.....ii
2020-01-01T21:48:54.6962266Z 
2020-01-01T21:48:54.6963313Z  finished in 27.517
2020-01-01T21:48:54.6971453Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-01T21:48:54.6972083Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-01T21:49:17.2010926Z failures:
2020-01-01T21:49:17.2018619Z 
2020-01-01T21:49:17.2019379Z ---- [ui] ui-fulldeps/issue-15778-fail.rs stdout ----
2020-01-01T21:49:17.2019572Z 
2020-01-01T21:49:17.2020181Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" failed to compile: 
2020-01-01T21:49:17.2020380Z status: exit code: 1
2020-01-01T21:49:17.2021327Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary"
2020-01-01T21:49:17.2022178Z ------------------------------------------
2020-01-01T21:49:17.2022354Z 
2020-01-01T21:49:17.2022739Z ------------------------------------------
2020-01-01T21:49:17.2022919Z stderr:
2020-01-01T21:49:17.2022919Z stderr:
2020-01-01T21:49:17.2023269Z ------------------------------------------
2020-01-01T21:49:17.2023484Z error[E0432]: unresolved import `syntax::symbol`
2020-01-01T21:49:17.2023877Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:15:13
2020-01-01T21:49:17.2024058Z    |
2020-01-01T21:49:17.2024228Z LL | use syntax::symbol::Symbol;
2020-01-01T21:49:17.2024381Z    |             ^^^^^^ could not find `symbol` in `syntax`
2020-01-01T21:49:17.2024507Z 
2020-01-01T21:49:17.2025130Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-01T21:49:17.2025715Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:34:1
2020-01-01T21:49:17.2026030Z LL | #[plugin_registrar]
2020-01-01T21:49:17.2026163Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-01T21:49:17.2026306Z    |
2020-01-01T21:49:17.2026434Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-01T21:49:17.2027759Z 
2020-01-01T21:49:17.2027866Z 
2020-01-01T21:49:17.2028201Z ---- [ui] ui-fulldeps/issue-15778-pass.rs stdout ----
2020-01-01T21:49:17.2028342Z 
2020-01-01T21:49:17.2028709Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" failed to compile: 
2020-01-01T21:49:17.2029068Z status: exit code: 1
2020-01-01T21:49:17.2030058Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary"
2020-01-01T21:49:17.2030765Z ------------------------------------------
2020-01-01T21:49:17.2030931Z 
2020-01-01T21:49:17.2031250Z ------------------------------------------
2020-01-01T21:49:17.2031404Z stderr:
---
2020-01-01T21:49:17.2034416Z    | ^^^^^^^^^^^^
2020-01-01T21:49:17.2034491Z    |
2020-01-01T21:49:17.2034584Z    = note: `#[warn(unused_imports)]` on by default
2020-01-01T21:49:17.2034636Z 
2020-01-01T21:49:17.2035103Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-01T21:49:17.2035722Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:61:1
2020-01-01T21:49:17.2036033Z LL | #[plugin_registrar]
2020-01-01T21:49:17.2036096Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-01T21:49:17.2036169Z    |
2020-01-01T21:49:17.2036236Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-01T21:49:17.2037312Z 
2020-01-01T21:49:17.2037424Z 
2020-01-01T21:49:17.2037698Z ---- [ui] ui-fulldeps/issue-40001.rs stdout ----
2020-01-01T21:49:17.2037870Z 
2020-01-01T21:49:17.2038288Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
2020-01-01T21:49:17.2038471Z status: exit code: 1
2020-01-01T21:49:17.2039498Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
2020-01-01T21:49:17.2040180Z ------------------------------------------
2020-01-01T21:49:17.2040215Z 
2020-01-01T21:49:17.2040607Z ------------------------------------------
2020-01-01T21:49:17.2040777Z stderr:
2020-01-01T21:49:17.2040777Z stderr:
2020-01-01T21:49:17.2041181Z ------------------------------------------
2020-01-01T21:49:17.2041410Z error[E0432]: unresolved import `syntax::source_map`
2020-01-01T21:49:17.2041759Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:13:5
2020-01-01T21:49:17.2042306Z LL | use syntax::source_map;
2020-01-01T21:49:17.2042381Z    |     ^^^^^^^^^^^^^^^^^^ no `source_map` in the root
2020-01-01T21:49:17.2042531Z 
2020-01-01T21:49:17.2042531Z 
2020-01-01T21:49:17.2043397Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-01T21:49:17.2043917Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:15:1
2020-01-01T21:49:17.2044299Z LL | #[plugin_registrar]
2020-01-01T21:49:17.2044377Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-01T21:49:17.2044440Z    |
2020-01-01T21:49:17.2044520Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-01T21:49:17.2046472Z ---- [ui] ui-fulldeps/mod_dir_path_canonicalized.rs stdout ----
2020-01-01T21:49:17.2046514Z 
2020-01-01T21:49:17.2046900Z error: test compilation failed although it shouldn't!
2020-01-01T21:49:17.2047094Z status: exit code: 1
2020-01-01T21:49:17.2047994Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/mod_dir_path_canonicalized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/auxiliary"
2020-01-01T21:49:17.2048479Z ------------------------------------------
2020-01-01T21:49:17.2048632Z 
2020-01-01T21:49:17.2048967Z ------------------------------------------
2020-01-01T21:49:17.2049150Z stderr:
---
2020-01-01T21:49:17.2052330Z ---- [ui] ui-fulldeps/pprust-expr-roundtrip.rs stdout ----
2020-01-01T21:49:17.2052519Z 
2020-01-01T21:49:17.2052889Z error: test compilation failed although it shouldn't!
2020-01-01T21:49:17.2052959Z status: exit code: 1
2020-01-01T21:49:17.2054130Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/auxiliary"
2020-01-01T21:49:17.2054748Z ------------------------------------------
2020-01-01T21:49:17.2054914Z 
2020-01-01T21:49:17.2055296Z ------------------------------------------
2020-01-01T21:49:17.2055875Z stderr:
2020-01-01T21:49:17.2055875Z stderr:
2020-01-01T21:49:17.2056092Z ------------------------------------------
2020-01-01T21:49:17.2056212Z error[E0432]: unresolved import `syntax::source_map`
2020-01-01T21:49:17.2056627Z   --> /checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs:30:13
2020-01-01T21:49:17.2056819Z    |
2020-01-01T21:49:17.2056934Z LL | use syntax::source_map::{Spanned, DUMMY_SP, FileName};
2020-01-01T21:49:17.2056998Z    |             ^^^^^^^^^^ could not find `source_map` in `syntax`
2020-01-01T21:49:17.2057163Z error[E0432]: unresolved import `syntax::source_map`
2020-01-01T21:49:17.2057427Z   --> /checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs:31:13
2020-01-01T21:49:17.2057619Z    |
2020-01-01T21:49:17.2057685Z LL | use syntax::source_map::FilePathMapping;
---
2020-01-01T21:49:17.2061800Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-01T21:49:17.2061902Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-01T21:49:17.2062152Z 
2020-01-01T21:49:17.2062183Z 
2020-01-01T21:49:17.2063952Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-01T21:49:17.2064514Z 
2020-01-01T21:49:17.2064548Z 
2020-01-01T21:49:17.2064615Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-01T21:49:17.2064801Z Build completed unsuccessfully in 1:04:06
2020-01-01T21:49:17.2064801Z Build completed unsuccessfully in 1:04:06
2020-01-01T21:49:17.2064964Z == clock drift check ==
2020-01-01T21:49:17.2065030Z   local time: Wed Jan  1 21:49:16 UTC 2020
2020-01-01T21:49:17.2065105Z   network time: Wed, 01 Jan 2020 21:49:16 GMT
2020-01-01T21:49:17.2065691Z == end clock drift check ==
2020-01-01T21:49:18.6883225Z 
2020-01-01T21:49:18.6980410Z ##[error]Bash exited with code '1'.
2020-01-01T21:49:18.7018380Z ##[section]Starting: Checkout
2020-01-01T21:49:18.7019846Z ==============================================================================
2020-01-01T21:49:18.7019888Z Task         : Get sources
2020-01-01T21:49:18.7019925Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
