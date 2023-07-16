plain
2019-11-08T08:45:25.5110790Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-08T08:45:25.5312045Z ##[command]git config gc.auto 0
2019-11-08T08:45:25.5355684Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-08T08:45:25.5423035Z ##[command]git config --get-all http.proxy
2019-11-08T08:45:25.5563065Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66213/merge:refs/remotes/pull/66213/merge
---
2019-11-08T09:42:15.9902001Z .................................................................................................... 1600/9289
2019-11-08T09:42:21.4913344Z .................................................................................................... 1700/9289
2019-11-08T09:42:33.3615250Z ................................................................i................................... 1800/9289
2019-11-08T09:42:40.8794899Z .................................................................................................... 1900/9289
2019-11-08T09:42:54.7942905Z ................................................iiiii............................................... 2000/9289
2019-11-08T09:43:05.0566186Z .................................................................................................... 2200/9289
2019-11-08T09:43:07.5239447Z .................................................................................................... 2300/9289
2019-11-08T09:43:11.0690351Z .................................................................................................... 2400/9289
2019-11-08T09:43:33.1627814Z .................................................................................................... 2500/9289
---
2019-11-08T09:46:08.3927997Z ............................................i...............i....................................... 4800/9289
2019-11-08T09:46:17.1215062Z .................................................................................................... 4900/9289
2019-11-08T09:46:24.1968260Z .................................................................................................... 5000/9289
2019-11-08T09:46:30.3317149Z .................................................................................................... 5100/9289
2019-11-08T09:46:39.9113850Z ..............................................ii.ii...........i..................................... 5200/9289
2019-11-08T09:46:49.1863135Z .................................................................................................... 5400/9289
2019-11-08T09:46:58.7840513Z .................................................................................................... 5500/9289
2019-11-08T09:47:05.8234680Z ............................i....................................................................... 5600/9289
2019-11-08T09:47:11.9880761Z .................................................................................................... 5700/9289
2019-11-08T09:47:11.9880761Z .................................................................................................... 5700/9289
2019-11-08T09:47:23.4258538Z .................................................................................................... 5800/9289
2019-11-08T09:47:34.4762337Z .............ii...i..ii...........i................................................................. 5900/9289
2019-11-08T09:47:51.4546477Z .................................................................................................... 6100/9289
2019-11-08T09:47:58.7635834Z .................................................................................................... 6200/9289
2019-11-08T09:47:58.7635834Z .................................................................................................... 6200/9289
2019-11-08T09:48:12.0576987Z ................................i..ii............................................................... 6300/9289
2019-11-08T09:48:31.9068454Z .................................................................................................... 6500/9289
2019-11-08T09:48:33.9372625Z i................................................................................................... 6600/9289
2019-11-08T09:48:36.0669828Z ....................................................................................i............... 6700/9289
2019-11-08T09:48:38.5698408Z .................................................................................................... 6800/9289
---
2019-11-08T09:53:41.2044590Z  finished in 5.373
2019-11-08T09:53:41.2208888Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-08T09:53:41.3725628Z 
2019-11-08T09:53:41.3725871Z running 156 tests
2019-11-08T09:53:44.1559487Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/156
2019-11-08T09:53:46.0454405Z .i.i..iiii..............i.........iii.i.........ii......
2019-11-08T09:53:46.0455688Z 
2019-11-08T09:53:46.0457949Z  finished in 4.824
2019-11-08T09:53:46.0637348Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-08T09:53:46.2129011Z 
---
2019-11-08T09:53:48.0735562Z  finished in 2.009
2019-11-08T09:53:48.0905537Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-08T09:53:48.2425395Z 
2019-11-08T09:53:48.2426219Z running 9 tests
2019-11-08T09:53:48.2427055Z iiiiiiiii
2019-11-08T09:53:48.2427891Z 
2019-11-08T09:53:48.2428596Z  finished in 0.151
2019-11-08T09:53:48.2597072Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-08T09:53:48.4129712Z 
---
2019-11-08T09:54:06.9156881Z  finished in 18.656
2019-11-08T09:54:06.9325425Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-08T09:54:07.0744533Z 
2019-11-08T09:54:07.0745307Z running 123 tests
2019-11-08T09:54:29.8050570Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-11-08T09:54:34.2346885Z i.i.i......iii.i.....ii
2019-11-08T09:54:34.2353794Z 
2019-11-08T09:54:34.2381177Z  finished in 27.302
2019-11-08T09:54:34.2385060Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-08T09:54:34.2385534Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-11-08T09:54:34.2385534Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-11-08T09:54:34.2543284Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-08T09:54:34.3995060Z 
2019-11-08T09:54:34.3996083Z running 70 tests
2019-11-08T09:55:34.6982546Z ...........................F.....F...F.......FF...F..F..FF...F..F.....
2019-11-08T09:55:34.6987181Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-08T09:55:34.6999822Z 
2019-11-08T09:55:34.7001139Z ---- [ui] ui-fulldeps/issue-15778-pass.rs stdout ----
2019-11-08T09:55:34.7003271Z 
2019-11-08T09:55:34.7003271Z 
2019-11-08T09:55:34.7004640Z error: /checkout/src/test/ui-fulldeps/issue-15778-pass.rs:8: unexpected warning: '8:1: 8:33: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]'
2019-11-08T09:55:34.7004894Z error: 1 unexpected errors found, 0 expected errors not found
2019-11-08T09:55:34.7005084Z status: exit code: 0
2019-11-08T09:55:34.7005084Z status: exit code: 0
2019-11-08T09:55:34.7005891Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-15778-pass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "crate-not-okay" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary"
2019-11-08T09:55:34.7006140Z unexpected errors (from JSON output): [
2019-11-08T09:55:34.7006239Z         line_num: 8,
2019-11-08T09:55:34.7006285Z         kind: Some(
2019-11-08T09:55:34.7006322Z             Warning,
2019-11-08T09:55:34.7006358Z         ),
2019-11-08T09:55:34.7006358Z         ),
2019-11-08T09:55:34.7006734Z         msg: "8:1: 8:33: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]",
2019-11-08T09:55:34.7006836Z ]
2019-11-08T09:55:34.7006860Z 
2019-11-08T09:55:34.7007146Z thread '[ui] ui-fulldeps/issue-15778-pass.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-08T09:55:34.7007218Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-08T09:55:34.7007218Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-08T09:55:34.7007247Z 
2019-11-08T09:55:34.7007447Z ---- [ui] ui-fulldeps/issue-40001.rs stdout ----
2019-11-08T09:55:34.7007479Z 
2019-11-08T09:55:34.7007862Z error: /checkout/src/test/ui-fulldeps/issue-40001.rs:6: unexpected warning: '6:1: 6:31: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]'
2019-11-08T09:55:34.7008200Z error: 1 unexpected errors found, 0 expected errors not found
2019-11-08T09:55:34.7008241Z status: exit code: 0
2019-11-08T09:55:34.7008241Z status: exit code: 0
2019-11-08T09:55:34.7009199Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-40001.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
2019-11-08T09:55:34.7009308Z unexpected errors (from JSON output): [
2019-11-08T09:55:34.7009401Z         line_num: 6,
2019-11-08T09:55:34.7009439Z         kind: Some(
2019-11-08T09:55:34.7009474Z             Warning,
2019-11-08T09:55:34.7009508Z         ),
2019-11-08T09:55:34.7009508Z         ),
2019-11-08T09:55:34.7009889Z         msg: "6:1: 6:31: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]",
2019-11-08T09:55:34.7009988Z ]
2019-11-08T09:55:34.7010011Z 
2019-11-08T09:55:34.7010749Z thread '[ui] ui-fulldeps/issue-40001.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-08T09:55:34.7010793Z 
2019-11-08T09:55:34.7010793Z 
2019-11-08T09:55:34.7011069Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-allow.rs stdout ----
2019-11-08T09:55:34.7011105Z 
2019-11-08T09:55:34.7011553Z error: /checkout/src/test/ui-fulldeps/lint-plugin-cmdline-allow.rs:8: unexpected warning: '8:1: 8:29: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]'
2019-11-08T09:55:34.7011617Z 
2019-11-08T09:55:34.7011960Z error: /checkout/src/test/ui-fulldeps/lint-plugin-cmdline-allow.rs:10: unexpected warning: '10:4: 10:10: function is never used: `lintme` [dead_code]'
2019-11-08T09:55:34.7012079Z error: 2 unexpected errors found, 0 expected errors not found
2019-11-08T09:55:34.7012125Z status: exit code: 0
2019-11-08T09:55:34.7012125Z status: exit code: 0
2019-11-08T09:55:34.7012925Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-cmdline-allow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary"
2019-11-08T09:55:34.7013038Z unexpected errors (from JSON output): [
2019-11-08T09:55:34.7013143Z         line_num: 8,
2019-11-08T09:55:34.7013185Z         kind: Some(
2019-11-08T09:55:34.7013225Z             Warning,
2019-11-08T09:55:34.7013290Z         ),
2019-11-08T09:55:34.7013290Z         ),
2019-11-08T09:55:34.7013813Z         msg: "8:1: 8:29: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]",
2019-11-08T09:55:34.7013911Z     Error {
2019-11-08T09:55:34.7013947Z         line_num: 10,
2019-11-08T09:55:34.7013999Z         kind: Some(
2019-11-08T09:55:34.7014034Z             Warning,
2019-11-08T09:55:34.7014034Z             Warning,
2019-11-08T09:55:34.7014076Z         ),
2019-11-08T09:55:34.7014116Z         msg: "10:4: 10:10: function is never used: `lintme` [dead_code]",
2019-11-08T09:55:34.7014206Z ]
2019-11-08T09:55:34.7014227Z 
2019-11-08T09:55:34.7014501Z thread '[ui] ui-fulldeps/lint-plugin-cmdline-allow.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-08T09:55:34.7014551Z 
2019-11-08T09:55:34.7014551Z 
2019-11-08T09:55:34.7014749Z ---- [ui] ui-fulldeps/lint-tool-cmdline-allow.rs stdout ----
2019-11-08T09:55:34.7014778Z 
2019-11-08T09:55:34.7015155Z error: /checkout/src/test/ui-fulldeps/lint-tool-cmdline-allow.rs:8: unexpected warning: '8:1: 8:27: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]'
2019-11-08T09:55:34.7015316Z 
2019-11-08T09:55:34.7015652Z error: /checkout/src/test/ui-fulldeps/lint-tool-cmdline-allow.rs:10: unexpected warning: '10:1: 10:15: item is named 'lintme' [clippy::test_lint]'
2019-11-08T09:55:34.7015688Z 
2019-11-08T09:55:34.7016038Z error: /checkout/src/test/ui-fulldeps/lint-tool-cmdline-allow.rs:10: unexpected warning: '10:4: 10:10: function is never used: `lintme` [dead_code]'
2019-11-08T09:55:34.7016140Z error: 3 unexpected errors found, 0 expected errors not found
2019-11-08T09:55:34.7016179Z status: exit code: 0
2019-11-08T09:55:34.7016179Z status: exit code: 0
2019-11-08T09:55:34.7016859Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-tool-cmdline-allow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary"
2019-11-08T09:55:34.7016968Z unexpected errors (from JSON output): [
2019-11-08T09:55:34.7017067Z         line_num: 8,
2019-11-08T09:55:34.7017103Z         kind: Some(
2019-11-08T09:55:34.7017138Z             Warning,
2019-11-08T09:55:34.7017189Z         ),
2019-11-08T09:55:34.7017189Z         ),
2019-11-08T09:55:34.7017504Z         msg: "8:1: 8:27: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]",
2019-11-08T09:55:34.7017601Z     Error {
2019-11-08T09:55:34.7017636Z         line_num: 10,
2019-11-08T09:55:34.7017671Z         kind: Some(
2019-11-08T09:55:34.7017732Z             Warning,
2019-11-08T09:55:34.7017732Z             Warning,
2019-11-08T09:55:34.7017766Z         ),
2019-11-08T09:55:34.7017990Z         msg: "10:1: 10:15: item is named \'lintme\' [clippy::test_lint]",
2019-11-08T09:55:34.7018079Z     Error {
2019-11-08T09:55:34.7018113Z         line_num: 10,
2019-11-08T09:55:34.7018164Z         kind: Some(
2019-11-08T09:55:34.7018198Z             Warning,
2019-11-08T09:55:34.7018198Z             Warning,
2019-11-08T09:55:34.7018231Z         ),
2019-11-08T09:55:34.7018278Z         msg: "10:4: 10:10: function is never used: `lintme` [dead_code]",
2019-11-08T09:55:34.7018364Z ]
2019-11-08T09:55:34.7018386Z 
2019-11-08T09:55:34.7018644Z thread '[ui] ui-fulldeps/lint-tool-cmdline-allow.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-08T09:55:34.7018694Z 
2019-11-08T09:55:34.7018694Z 
2019-11-08T09:55:34.7018886Z ---- [ui] ui-fulldeps/llvm-pass-plugin.rs stdout ----
2019-11-08T09:55:34.7018914Z 
2019-11-08T09:55:34.7019288Z error: /checkout/src/test/ui-fulldeps/llvm-pass-plugin.rs:6: unexpected warning: '6:1: 6:29: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]'
2019-11-08T09:55:34.7019376Z error: 1 unexpected errors found, 0 expected errors not found
2019-11-08T09:55:34.7019431Z status: exit code: 0
2019-11-08T09:55:34.7019431Z status: exit code: 0
2019-11-08T09:55:34.7020559Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/llvm-pass-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/llvm-pass-plugin/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/llvm-pass-plugin/auxiliary"
2019-11-08T09:55:34.7020686Z unexpected errors (from JSON output): [
2019-11-08T09:55:34.7020904Z         line_num: 6,
2019-11-08T09:55:34.7020947Z         kind: Some(
2019-11-08T09:55:34.7020989Z             Warning,
2019-11-08T09:55:34.7021029Z         ),
2019-11-08T09:55:34.7021029Z         ),
2019-11-08T09:55:34.7021477Z         msg: "6:1: 6:29: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]",
2019-11-08T09:55:34.7021590Z ]
2019-11-08T09:55:34.7021617Z 
2019-11-08T09:55:34.7022013Z thread '[ui] ui-fulldeps/llvm-pass-plugin.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-08T09:55:34.7022060Z 
2019-11-08T09:55:34.7022060Z 
2019-11-08T09:55:34.7022337Z ---- [ui] ui-fulldeps/lto-syntax-extension.rs stdout ----
2019-11-08T09:55:34.7022370Z 
2019-11-08T09:55:34.7022806Z error: /checkout/src/test/ui-fulldeps/lto-syntax-extension.rs:9: unexpected warning: '9:1: 9:40: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]'
2019-11-08T09:55:34.7022915Z error: 1 unexpected errors found, 0 expected errors not found
2019-11-08T09:55:34.7022970Z status: exit code: 0
2019-11-08T09:55:34.7022970Z status: exit code: 0
2019-11-08T09:55:34.7023894Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lto-syntax-extension.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lto-syntax-extension/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lto-syntax-extension/auxiliary"
2019-11-08T09:55:34.7023991Z unexpected errors (from JSON output): [
2019-11-08T09:55:34.7024064Z         line_num: 9,
2019-11-08T09:55:34.7024099Z         kind: Some(
2019-11-08T09:55:34.7024150Z             Warning,
2019-11-08T09:55:34.7024183Z         ),
2019-11-08T09:55:34.7024183Z         ),
2019-11-08T09:55:34.7024513Z         msg: "9:1: 9:40: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]",
2019-11-08T09:55:34.7024602Z ]
2019-11-08T09:55:34.7024625Z 
2019-11-08T09:55:34.7025088Z thread '[ui] ui-fulldeps/lto-syntax-extension.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-08T09:55:34.7025122Z 
2019-11-08T09:55:34.7025122Z 
2019-11-08T09:55:34.7025333Z ---- [ui] ui-fulldeps/outlive-expansion-phase.rs stdout ----
2019-11-08T09:55:34.7025382Z 
2019-11-08T09:55:34.7025916Z error: /checkout/src/test/ui-fulldeps/outlive-expansion-phase.rs:6: unexpected warning: '6:1: 6:36: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]'
2019-11-08T09:55:34.7026014Z error: 1 unexpected errors found, 0 expected errors not found
2019-11-08T09:55:34.7026051Z status: exit code: 0
2019-11-08T09:55:34.7026051Z status: exit code: 0
2019-11-08T09:55:34.7027137Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/outlive-expansion-phase.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/outlive-expansion-phase/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/outlive-expansion-phase/auxiliary"
2019-11-08T09:55:34.7027230Z unexpected errors (from JSON output): [
2019-11-08T09:55:34.7027327Z         line_num: 6,
2019-11-08T09:55:34.7027365Z         kind: Some(
2019-11-08T09:55:34.7027403Z             Warning,
2019-11-08T09:55:34.7027456Z         ),
2019-11-08T09:55:34.7027456Z         ),
2019-11-08T09:55:34.7028189Z         msg: "6:1: 6:36: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]",
2019-11-08T09:55:34.7028562Z ]
2019-11-08T09:55:34.7028589Z 
2019-11-08T09:55:34.7028934Z thread '[ui] ui-fulldeps/outlive-expansion-phase.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-08T09:55:34.7028972Z 
2019-11-08T09:55:34.7028972Z 
2019-11-08T09:55:34.7029190Z ---- [ui] ui-fulldeps/plugin-args-1.rs stdout ----
2019-11-08T09:55:34.7029222Z 
2019-11-08T09:55:34.7029726Z error: /checkout/src/test/ui-fulldeps/plugin-args-1.rs:6: unexpected warning: '6:1: 6:24: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]'
2019-11-08T09:55:34.7029842Z error: 1 unexpected errors found, 0 expected errors not found
2019-11-08T09:55:34.7029885Z status: exit code: 0
2019-11-08T09:55:34.7029885Z status: exit code: 0
2019-11-08T09:55:34.7030628Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/plugin-args-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/auxiliary"
2019-11-08T09:55:34.7030743Z unexpected errors (from JSON output): [
2019-11-08T09:55:34.7031013Z         line_num: 6,
2019-11-08T09:55:34.7031058Z         kind: Some(
2019-11-08T09:55:34.7031096Z             Warning,
2019-11-08T09:55:34.7031134Z         ),
2019-11-08T09:55:34.7031134Z         ),
2019-11-08T09:55:34.7031495Z         msg: "6:1: 6:24: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]",
2019-11-08T09:55:34.7031600Z ]
2019-11-08T09:55:34.7031624Z 
2019-11-08T09:55:34.7031905Z thread '[ui] ui-fulldeps/plugin-args-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-08T09:55:34.7031940Z 
2019-11-08T09:55:34.7031940Z 
2019-11-08T09:55:34.7032177Z ---- [ui] ui-fulldeps/plugin-args-2.rs stdout ----
2019-11-08T09:55:34.7032208Z 
2019-11-08T09:55:34.7032600Z error: /checkout/src/test/ui-fulldeps/plugin-args-2.rs:6: unexpected warning: '6:1: 6:26: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]'
2019-11-08T09:55:34.7032703Z error: 1 unexpected errors found, 0 expected errors not found
2019-11-08T09:55:34.7032752Z status: exit code: 0
2019-11-08T09:55:34.7032752Z status: exit code: 0
2019-11-08T09:55:34.7033752Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/plugin-args-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/auxiliary"
2019-11-08T09:55:34.7033859Z unexpected errors (from JSON output): [
2019-11-08T09:55:34.7034542Z         line_num: 6,
2019-11-08T09:55:34.7034588Z         kind: Some(
2019-11-08T09:55:34.7034647Z             Warning,
2019-11-08T09:55:34.7034687Z         ),
2019-11-08T09:55:34.7034687Z         ),
2019-11-08T09:55:34.7035123Z         msg: "6:1: 6:26: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]",
2019-11-08T09:55:34.7035218Z ]
2019-11-08T09:55:34.7035244Z 
2019-11-08T09:55:34.7035570Z thread '[ui] ui-fulldeps/plugin-args-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-08T09:55:34.7035608Z 
2019-11-08T09:55:34.7035608Z 
2019-11-08T09:55:34.7035832Z ---- [ui] ui-fulldeps/plugin-args-3.rs stdout ----
2019-11-08T09:55:34.7035884Z 
2019-11-08T09:55:34.7036307Z error: /checkout/src/test/ui-fulldeps/plugin-args-3.rs:6: unexpected warning: '6:1: 6:54: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]'
2019-11-08T09:55:34.7036530Z error: 1 unexpected errors found, 0 expected errors not found
2019-11-08T09:55:34.7036575Z status: exit code: 0
2019-11-08T09:55:34.7036575Z status: exit code: 0
2019-11-08T09:55:34.7037727Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/plugin-args-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/auxiliary"
2019-11-08T09:55:34.7037864Z unexpected errors (from JSON output): [
2019-11-08T09:55:34.7037975Z         line_num: 6,
2019-11-08T09:55:34.7038017Z         kind: Some(
2019-11-08T09:55:34.7038071Z             Warning,
2019-11-08T09:55:34.7038129Z         ),
2019-11-08T09:55:34.7038129Z         ),
2019-11-08T09:55:34.7038756Z         msg: "6:1: 6:54: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]",
2019-11-08T09:55:34.7038869Z ]
2019-11-08T09:55:34.7038895Z 
2019-11-08T09:55:34.7039203Z thread '[ui] ui-fulldeps/plugin-args-3.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-08T09:55:34.7039258Z 
2019-11-08T09:55:34.7039258Z 
2019-11-08T09:55:34.7039638Z ---- [ui] ui-fulldeps/roman-numerals-macro.rs stdout ----
2019-11-08T09:55:34.7039668Z 
2019-11-08T09:55:34.7040083Z error: /checkout/src/test/ui-fulldeps/roman-numerals-macro.rs:6: unexpected warning: '6:1: 6:27: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]'
2019-11-08T09:55:34.7040171Z error: 1 unexpected errors found, 0 expected errors not found
2019-11-08T09:55:34.7040240Z status: exit code: 0
2019-11-08T09:55:34.7040240Z status: exit code: 0
2019-11-08T09:55:34.7041245Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/roman-numerals-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary"
2019-11-08T09:55:34.7041340Z unexpected errors (from JSON output): [
2019-11-08T09:55:34.7041429Z         line_num: 6,
2019-11-08T09:55:34.7041464Z         kind: Some(
2019-11-08T09:55:34.7041499Z             Warning,
2019-11-08T09:55:34.7041532Z         ),
2019-11-08T09:55:34.7041532Z         ),
2019-11-08T09:55:34.7041859Z         msg: "6:1: 6:27: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675 [deprecated]",
2019-11-08T09:55:34.7041963Z ]
2019-11-08T09:55:34.7041985Z 
2019-11-08T09:55:34.7042249Z thread '[ui] ui-fulldeps/roman-numerals-macro.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-08T09:55:34.7042282Z 
---
2019-11-08T09:55:34.7045579Z test result: FAILED. 59 passed; 11 failed; 0 ignored; 0 measured; 0 filtered out
2019-11-08T09:55:34.7045613Z 
2019-11-08T09:55:34.7045679Z 
2019-11-08T09:55:34.7045704Z 
2019-11-08T09:55:34.7047908Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-08T09:55:34.7048119Z 
2019-11-08T09:55:34.7048160Z 
2019-11-08T09:55:34.7054152Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-08T09:55:34.7054229Z Build completed unsuccessfully in 1:03:46
2019-11-08T09:55:34.7054229Z Build completed unsuccessfully in 1:03:46
2019-11-08T09:55:34.7108019Z == clock drift check ==
2019-11-08T09:55:34.7122045Z   local time: Fri Nov  8 09:55:34 UTC 2019
2019-11-08T09:55:34.9897252Z   network time: Fri, 08 Nov 2019 09:55:34 GMT
2019-11-08T09:55:34.9897365Z == end clock drift check ==
2019-11-08T09:55:35.5518592Z 
2019-11-08T09:55:35.5614886Z ##[error]Bash exited with code '1'.
2019-11-08T09:55:35.5657062Z ##[section]Starting: Checkout
2019-11-08T09:55:35.5658606Z ==============================================================================
2019-11-08T09:55:35.5658651Z Task         : Get sources
2019-11-08T09:55:35.5658703Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
