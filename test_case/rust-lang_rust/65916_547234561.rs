plain
2019-10-29T01:50:40.0843795Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-29T01:50:40.1070986Z ##[command]git config gc.auto 0
2019-10-29T01:50:40.1140590Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-29T01:50:40.1188193Z ##[command]git config --get-all http.proxy
2019-10-29T01:50:40.1319621Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65916/merge:refs/remotes/pull/65916/merge
---
2019-10-29T02:47:25.8407437Z .................................................................................................... 1600/9259
2019-10-29T02:47:31.1532227Z .................................................................................................... 1700/9259
2019-10-29T02:47:42.2039156Z ..........................................................i...............i......................... 1800/9259
2019-10-29T02:47:49.1238661Z .................................................................................................... 1900/9259
2019-10-29T02:48:02.2325605Z ................................................iiiii............................................... 2000/9259
2019-10-29T02:48:11.9654192Z .................................................................................................... 2200/9259
2019-10-29T02:48:14.3859806Z .................................................................................................... 2300/9259
2019-10-29T02:48:17.8998184Z .................................................................................................... 2400/9259
2019-10-29T02:48:39.0838280Z .................................................................................................... 2500/9259
---
2019-10-29T02:51:20.6803191Z .................................................i...............i.................................. 4800/9259
2019-10-29T02:51:29.0831709Z .................................................................................................... 4900/9259
2019-10-29T02:51:36.7173137Z .................................................................................................... 5000/9259
2019-10-29T02:51:42.4639399Z .................................................................................................... 5100/9259
2019-10-29T02:51:51.8015120Z ..................................................ii.ii...........i................................. 5200/9259
2019-10-29T02:52:00.8163006Z .................................................................................................... 5400/9259
2019-10-29T02:52:09.2405475Z .................................................................................................... 5500/9259
2019-10-29T02:52:16.4960186Z ....................i............................................................................... 5600/9259
2019-10-29T02:52:21.9851259Z .................................................................................................... 5700/9259
2019-10-29T02:52:21.9851259Z .................................................................................................... 5700/9259
2019-10-29T02:52:32.8007097Z .................................................................................................... 5800/9259
2019-10-29T02:52:44.1199036Z .....ii...i..ii...........i......................................................................... 5900/9259
2019-10-29T02:53:04.0677061Z .................................................................................................... 6100/9259
2019-10-29T02:53:10.1798587Z .................................................................................................... 6200/9259
2019-10-29T02:53:10.1798587Z .................................................................................................... 6200/9259
2019-10-29T02:53:22.9406777Z ........................i..ii....................................................................... 6300/9259
2019-10-29T02:53:41.1109579Z ..........................................................................................i......... 6500/9259
2019-10-29T02:53:43.2680349Z .................................................................................................... 6600/9259
2019-10-29T02:53:45.4668483Z .................................................................i.................................. 6700/9259
2019-10-29T02:53:48.1894047Z .................................................................................................... 6800/9259
---
2019-10-29T02:57:57.7289868Z  finished in 5.166
2019-10-29T02:57:57.7454197Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-29T02:57:57.8967630Z 
2019-10-29T02:57:57.8968224Z running 153 tests
2019-10-29T02:58:00.8367785Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-29T02:58:02.5991320Z i..iiii..............i.........iii.i.........ii......
2019-10-29T02:58:02.5992388Z 
2019-10-29T02:58:02.6001488Z  finished in 4.854
2019-10-29T02:58:02.6159362Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-29T02:58:02.7554617Z 
---
2019-10-29T02:58:04.5262496Z  finished in 1.910
2019-10-29T02:58:04.5407352Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-29T02:58:04.6685763Z 
2019-10-29T02:58:04.6686568Z running 9 tests
2019-10-29T02:58:04.6687535Z iiiiiiiii
2019-10-29T02:58:04.6687920Z 
2019-10-29T02:58:04.6689411Z  finished in 0.127
2019-10-29T02:58:04.6838177Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-29T02:58:04.8256955Z 
---
2019-10-29T02:58:21.6783286Z  finished in 16.994
2019-10-29T02:58:21.6996835Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-29T02:58:21.8492279Z 
2019-10-29T02:58:21.8493379Z running 123 tests
2019-10-29T02:58:44.2388430Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-29T02:58:47.5210184Z i.i.i......iii.i.....ii
2019-10-29T02:58:47.5211296Z 
2019-10-29T02:58:47.5217588Z  finished in 25.822
2019-10-29T02:58:47.5228517Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-29T02:58:47.5252986Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-10-29T02:59:41.2716860Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-29T02:59:41.2717266Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-29T02:59:41.2719138Z ---- [ui] ui-fulldeps/roman-numerals-macro.rs stdout ----
2019-10-29T02:59:41.2719541Z 
2019-10-29T02:59:41.2720040Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" failed to compile: 
2019-10-29T02:59:41.2720458Z status: exit code: 1
2019-10-29T02:59:41.2721797Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary"
2019-10-29T02:59:41.2723363Z ------------------------------------------
2019-10-29T02:59:41.2723532Z 
2019-10-29T02:59:41.2724019Z ------------------------------------------
2019-10-29T02:59:41.2724299Z stderr:
2019-10-29T02:59:41.2724299Z stderr:
2019-10-29T02:59:41.2724635Z ------------------------------------------
2019-10-29T02:59:41.2724813Z error[E0603]: module `token` is private
2019-10-29T02:59:41.2725292Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:18:20
2019-10-29T02:59:41.2725479Z    |
2019-10-29T02:59:41.2725634Z LL | use syntax::parse::token::{self, Token};
2019-10-29T02:59:41.2725846Z 
2019-10-29T02:59:41.2725846Z 
2019-10-29T02:59:41.2726364Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-10-29T02:59:41.2726742Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:66:1
2019-10-29T02:59:41.2727041Z LL | #[plugin_registrar]
2019-10-29T02:59:41.2727177Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-10-29T02:59:41.2727290Z    |
2019-10-29T02:59:41.2727400Z    = note: `#[warn(deprecated)]` on by default
---
2019-10-29T02:59:41.2730014Z test result: FAILED. 68 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-10-29T02:59:41.2730152Z 
2019-10-29T02:59:41.2730306Z 
2019-10-29T02:59:41.2730435Z 
2019-10-29T02:59:41.2732614Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-29T02:59:41.2734024Z 
2019-10-29T02:59:41.2734049Z 
2019-10-29T02:59:41.2737088Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-29T02:59:41.2737260Z Build completed unsuccessfully in 1:02:07
2019-10-29T02:59:41.2737260Z Build completed unsuccessfully in 1:02:07
2019-10-29T02:59:41.2786550Z == clock drift check ==
2019-10-29T02:59:41.2799310Z   local time: Tue Oct 29 02:59:41 UTC 2019
2019-10-29T02:59:41.5451331Z   network time: Tue, 29 Oct 2019 02:59:41 GMT
2019-10-29T02:59:41.5455519Z == end clock drift check ==
2019-10-29T02:59:42.5658914Z 
2019-10-29T02:59:42.5754611Z ##[error]Bash exited with code '1'.
2019-10-29T02:59:42.5816008Z ##[section]Starting: Checkout
2019-10-29T02:59:42.5818581Z ==============================================================================
2019-10-29T02:59:42.5818814Z Task         : Get sources
2019-10-29T02:59:42.5818859Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
