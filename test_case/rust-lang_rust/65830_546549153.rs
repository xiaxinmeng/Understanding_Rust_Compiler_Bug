plain
2019-10-25T22:58:02.9565966Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-25T22:58:02.9770251Z ##[command]git config gc.auto 0
2019-10-25T22:58:02.9858936Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-25T22:58:02.9916466Z ##[command]git config --get-all http.proxy
2019-10-25T22:58:03.0065234Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65830/merge:refs/remotes/pull/65830/merge
---
2019-10-25T23:30:28.7180566Z    Compiling rustc-rayon v0.3.0
2019-10-25T23:30:33.4236783Z    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-10-25T23:30:36.0875170Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2019-10-25T23:30:38.8115277Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-10-25T23:30:39.3953609Z warning: function is never used: `query`
2019-10-25T23:30:39.3954717Z   --> src/librustc_macros/src/query.rs:16:26
2019-10-25T23:30:39.3955186Z    |
2019-10-25T23:30:39.3955666Z 16 |     syn::custom_keyword!(query);
2019-10-25T23:30:39.3957121Z    |
2019-10-25T23:30:39.3958054Z    = note: `#[warn(dead_code)]` on by default
2019-10-25T23:30:39.3958317Z 
2019-10-25T23:30:39.3958317Z 
2019-10-25T23:30:39.3963879Z warning: function is never used: `Keywords`
2019-10-25T23:30:39.3964491Z   --> src/librustc_macros/src/symbols.rs:13:26
2019-10-25T23:30:39.3964943Z    |
2019-10-25T23:30:39.3965906Z 13 |     syn::custom_keyword!(Keywords);
2019-10-25T23:30:39.3967031Z 
2019-10-25T23:30:39.3973308Z warning: function is never used: `Symbols`
2019-10-25T23:30:39.3974365Z   --> src/librustc_macros/src/symbols.rs:14:26
2019-10-25T23:30:39.3974786Z    |
2019-10-25T23:30:39.3974786Z    |
2019-10-25T23:30:39.3975288Z 14 |     syn::custom_keyword!(Symbols);
2019-10-25T23:30:39.3976475Z 
2019-10-25T23:30:49.9569399Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-25T23:30:51.4850193Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-25T23:31:13.0590349Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
---
2019-10-25T23:57:46.4890223Z .................................................................................................... 1600/9252
2019-10-25T23:57:52.1119643Z .................................................................................................... 1700/9252
2019-10-25T23:58:04.8740657Z ........................................................i........................................... 1800/9252
2019-10-25T23:58:13.0750386Z .................................................................................................... 1900/9252
2019-10-25T23:58:27.2639126Z ........................................iiiii....................................................... 2000/9252
2019-10-25T23:58:37.9997190Z .................................................................................................... 2200/9252
2019-10-25T23:58:40.6407283Z .................................................................................................... 2300/9252
2019-10-25T23:58:45.2257058Z .................................................................................................... 2400/9252
2019-10-25T23:59:08.4784767Z .................................................................................................... 2500/9252
---
2019-10-26T00:02:03.6970599Z .........................................i...............i.......................................... 4800/9252
2019-10-26T00:02:14.4771670Z .................................................................................................... 4900/9252
2019-10-26T00:02:21.5101002Z .................................................................................................... 5000/9252
2019-10-26T00:02:28.9372594Z .................................................................................................... 5100/9252
2019-10-26T00:02:38.5187572Z ..........................................ii.ii...........i......................................... 5200/9252
2019-10-26T00:02:48.4210919Z .................................................................................................... 5400/9252
2019-10-26T00:02:57.7269284Z .................................................................................................... 5500/9252
2019-10-26T00:03:05.7782393Z ....................i............................................................................... 5600/9252
2019-10-26T00:03:11.4707124Z .................................................................................................... 5700/9252
2019-10-26T00:03:11.4707124Z .................................................................................................... 5700/9252
2019-10-26T00:03:23.7958082Z .................................................................................................... 5800/9252
2019-10-26T00:03:35.0768229Z .................ii...i..ii...........i............................................................. 5900/9252
2019-10-26T00:03:57.0246383Z .................................................................................................... 6100/9252
2019-10-26T00:04:04.3489035Z .................................................................................................... 6200/9252
2019-10-26T00:04:04.3489035Z .................................................................................................... 6200/9252
2019-10-26T00:04:17.3882734Z ........................................i..ii....................................................... 6300/9252
2019-10-26T00:04:39.6218584Z .................................................................................................... 6500/9252
2019-10-26T00:04:41.9381097Z ......i............................................................................................. 6600/9252
2019-10-26T00:04:44.1715310Z .................................................................................i.................. 6700/9252
2019-10-26T00:04:46.8788346Z .................................................................................................... 6800/9252
---
2019-10-26T00:09:22.1099883Z  finished in 5.651
2019-10-26T00:09:22.1285867Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-26T00:09:22.3130224Z 
2019-10-26T00:09:22.3130488Z running 153 tests
2019-10-26T00:09:25.5789736Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-26T00:09:27.5719767Z i..iiii..............i.........iii.i.........ii......
2019-10-26T00:09:27.5720782Z 
2019-10-26T00:09:27.5724571Z  finished in 5.443
2019-10-26T00:09:27.5926094Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-26T00:09:27.7601060Z 
---
2019-10-26T00:09:29.8346383Z  finished in 2.242
2019-10-26T00:09:29.8551525Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-26T00:09:30.0263868Z 
2019-10-26T00:09:30.0264563Z running 9 tests
2019-10-26T00:09:30.0269205Z iiiiiiiii
2019-10-26T00:09:30.0269927Z 
2019-10-26T00:09:30.0273618Z  finished in 0.172
2019-10-26T00:09:30.0473455Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-26T00:09:30.2285601Z 
---
2019-10-26T00:09:48.6755467Z  finished in 18.628
2019-10-26T00:09:48.6973824Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-26T00:09:48.8649160Z 
2019-10-26T00:09:48.8649400Z running 123 tests
2019-10-26T00:10:13.9491709Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-26T00:10:18.2442068Z i.i.i......iii.i.....ii
2019-10-26T00:10:18.2442888Z 
2019-10-26T00:10:18.2447864Z  finished in 29.547
2019-10-26T00:10:18.2457656Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-26T00:10:18.2458497Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-10-26T00:11:21.0875776Z diff of stderr:
2019-10-26T00:11:21.0875921Z 
2019-10-26T00:11:21.0876090Z 7    = note: `#[warn(deprecated)]` on by default
2019-10-26T00:11:21.0876229Z 8 
2019-10-26T00:11:21.0876370Z 9 warning: function is never used: `lintme`
2019-10-26T00:11:21.0877219Z +   --> $DIR/lint-plugin-cmdline-allow.rs:10:4
2019-10-26T00:11:21.0878667Z 11    |
2019-10-26T00:11:21.0878667Z 11    |
2019-10-26T00:11:21.0879151Z 12 LL | fn lintme() { }
2019-10-26T00:11:21.0881880Z +    |    ^^^^^^
2019-10-26T00:11:21.0882101Z 14    |
2019-10-26T00:11:21.0882243Z 15 note: lint level defined here
2019-10-26T00:11:21.0882737Z 16   --> $DIR/lint-plugin-cmdline-allow.rs:7:9
2019-10-26T00:11:21.0882737Z 16   --> $DIR/lint-plugin-cmdline-allow.rs:7:9
2019-10-26T00:11:21.0882914Z 
2019-10-26T00:11:21.0883034Z 
2019-10-26T00:11:21.0883196Z The actual stderr differed from the expected stderr.
2019-10-26T00:11:21.0883703Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/lint-plugin-cmdline-allow.stderr
2019-10-26T00:11:21.0884137Z To update references, rerun the tests and pass the `--bless` flag
2019-10-26T00:11:21.0884781Z To only update this specific test, also pass `--test-args lint-plugin-cmdline-allow.rs`
2019-10-26T00:11:21.0885126Z error: 1 errors occurred comparing output.
2019-10-26T00:11:21.0885296Z status: exit code: 0
2019-10-26T00:11:21.0885296Z status: exit code: 0
2019-10-26T00:11:21.0886260Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-cmdline-allow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary"
2019-10-26T00:11:21.0886906Z ------------------------------------------
2019-10-26T00:11:21.0887073Z 
2019-10-26T00:11:21.0887467Z ------------------------------------------
2019-10-26T00:11:21.0959780Z stderr:
2019-10-26T00:11:21.0959780Z stderr:
2019-10-26T00:11:21.0960238Z ------------------------------------------
2019-10-26T00:11:21.0961249Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2019-10-26T00:11:21.0961664Z    |
2019-10-26T00:11:21.0961664Z    |
2019-10-26T00:11:21.0961710Z LL | #![plugin(lint_plugin_test)]
2019-10-26T00:11:21.0961810Z    |
2019-10-26T00:11:21.0961889Z    = note: `#[warn(deprecated)]` on by default
2019-10-26T00:11:21.0961922Z 
2019-10-26T00:11:21.0961922Z 
2019-10-26T00:11:21.0961967Z warning: function is never used: `lintme`
2019-10-26T00:11:21.0962304Z    |
2019-10-26T00:11:21.0962304Z    |
2019-10-26T00:11:21.0962346Z LL | fn lintme() { }
2019-10-26T00:11:21.0962449Z    |
2019-10-26T00:11:21.0962677Z note: lint level defined here
2019-10-26T00:11:21.0962972Z   --> /checkout/src/test/ui-fulldeps/lint-plugin-cmdline-allow.rs:7:9
2019-10-26T00:11:21.0963043Z    |
---
2019-10-26T00:11:21.0964013Z diff of stderr:
2019-10-26T00:11:21.0964040Z 
2019-10-26T00:11:21.0964105Z 19    = note: `#[warn(clippy::test_lint)]` on by default
2019-10-26T00:11:21.0964148Z 20 
2019-10-26T00:11:21.0964193Z 21 warning: function is never used: `lintme`
2019-10-26T00:11:21.0965687Z +   --> $DIR/lint-tool-cmdline-allow.rs:10:4
2019-10-26T00:11:21.0965732Z 23    |
2019-10-26T00:11:21.0965732Z 23    |
2019-10-26T00:11:21.0965771Z 24 LL | fn lintme() {}
2019-10-26T00:11:21.0966694Z +    |    ^^^^^^
2019-10-26T00:11:21.0966735Z 26    |
2019-10-26T00:11:21.0966795Z 27 note: lint level defined here
2019-10-26T00:11:21.0967061Z 28   --> $DIR/lint-tool-cmdline-allow.rs:7:9
2019-10-26T00:11:21.0967061Z 28   --> $DIR/lint-tool-cmdline-allow.rs:7:9
2019-10-26T00:11:21.0967094Z 
2019-10-26T00:11:21.0967119Z 
2019-10-26T00:11:21.0967182Z The actual stderr differed from the expected stderr.
2019-10-26T00:11:21.0967523Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/lint-tool-cmdline-allow.stderr
2019-10-26T00:11:21.0967792Z To update references, rerun the tests and pass the `--bless` flag
2019-10-26T00:11:21.0968123Z To only update this specific test, also pass `--test-args lint-tool-cmdline-allow.rs`
2019-10-26T00:11:21.0968208Z error: 1 errors occurred comparing output.
2019-10-26T00:11:21.0968282Z status: exit code: 0
2019-10-26T00:11:21.0968282Z status: exit code: 0
2019-10-26T00:11:21.0969089Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-tool-cmdline-allow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary"
2019-10-26T00:11:21.0969483Z ------------------------------------------
2019-10-26T00:11:21.0969519Z 
2019-10-26T00:11:21.0969779Z ------------------------------------------
2019-10-26T00:11:21.0969827Z stderr:
2019-10-26T00:11:21.0969827Z stderr:
2019-10-26T00:11:21.0970106Z ------------------------------------------
2019-10-26T00:11:21.0970335Z warning: lint name `test_lint` is deprecated and does not have an effect anymore. Use: clippy::test_lint
2019-10-26T00:11:21.0971031Z    = note: requested on the command line with `-A test_lint`
2019-10-26T00:11:21.0971071Z 
2019-10-26T00:11:21.0971071Z 
2019-10-26T00:11:21.0971482Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2019-10-26T00:11:21.0971830Z    |
2019-10-26T00:11:21.0971830Z    |
2019-10-26T00:11:21.0971894Z LL | #![plugin(lint_tool_test)]
2019-10-26T00:11:21.0972008Z    |
2019-10-26T00:11:21.0972074Z    = note: `#[warn(deprecated)]` on by default
2019-10-26T00:11:21.0972106Z 
2019-10-26T00:11:21.0972106Z 
2019-10-26T00:11:21.0972345Z warning: item is named 'lintme'
2019-10-26T00:11:21.0972683Z    |
2019-10-26T00:11:21.0972683Z    |
2019-10-26T00:11:21.0972867Z LL | fn lintme() {}
2019-10-26T00:11:21.0972973Z    |
2019-10-26T00:11:21.0973021Z    = note: `#[warn(clippy::test_lint)]` on by default
2019-10-26T00:11:21.0973054Z 
2019-10-26T00:11:21.0973054Z 
2019-10-26T00:11:21.0973099Z warning: function is never used: `lintme`
2019-10-26T00:11:21.0973481Z    |
2019-10-26T00:11:21.0973481Z    |
2019-10-26T00:11:21.0973525Z LL | fn lintme() {}
2019-10-26T00:11:21.0973628Z    |
2019-10-26T00:11:21.0973745Z note: lint level defined here
2019-10-26T00:11:21.0974065Z   --> /checkout/src/test/ui-fulldeps/lint-tool-cmdline-allow.rs:7:9
2019-10-26T00:11:21.0974117Z    |
---
2019-10-26T00:11:21.0975613Z 
2019-10-26T00:11:21.0975635Z 
2019-10-26T00:11:21.0975886Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-26T00:11:21.0975956Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-26T00:11:21.0977306Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-26T00:11:21.0977557Z 
2019-10-26T00:11:21.0977583Z 
2019-10-26T00:11:21.0977625Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-26T00:11:21.0977688Z Build completed unsuccessfully in 1:06:36
2019-10-26T00:11:21.0977688Z Build completed unsuccessfully in 1:06:36
2019-10-26T00:11:21.0977730Z == clock drift check ==
2019-10-26T00:11:21.0977771Z   local time: Sat Oct 26 00:11:21 UTC 2019
2019-10-26T00:11:21.9675672Z   network time: Sat, 26 Oct 2019 00:11:21 GMT
2019-10-26T00:11:21.9717005Z == end clock drift check ==
2019-10-26T00:11:22.2614357Z 
2019-10-26T00:11:22.2677659Z ##[error]Bash exited with code '1'.
2019-10-26T00:11:22.2718236Z ##[section]Starting: Checkout
2019-10-26T00:11:22.2719712Z ==============================================================================
2019-10-26T00:11:22.2719760Z Task         : Get sources
2019-10-26T00:11:22.2719943Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
