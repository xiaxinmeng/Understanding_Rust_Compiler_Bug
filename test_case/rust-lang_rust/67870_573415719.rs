plain
2020-01-12T12:31:54.0547139Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-12T12:31:54.0555079Z ##[command]git config gc.auto 0
2020-01-12T12:31:54.0556886Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-12T12:31:54.0558369Z ##[command]git config --get-all http.proxy
2020-01-12T12:31:54.0560761Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67870/merge:refs/remotes/pull/67870/merge
---
2020-01-12T13:23:26.6526410Z ........................................i...............i........................................... 4900/9518
2020-01-12T13:23:34.9812158Z .................................................................................................... 5000/9518
2020-01-12T13:23:40.6805499Z ...................................................................................i................ 5100/9518
2020-01-12T13:23:45.5236686Z .................................................................................................... 5200/9518
2020-01-12T13:23:54.6039215Z ......................................................ii.ii...........i............................. 5300/9518
2020-01-12T13:24:02.9473941Z .................................................................................................... 5500/9518
2020-01-12T13:24:11.6956237Z .................................................................................................... 5600/9518
2020-01-12T13:24:17.6905630Z .......................................i............................................................ 5700/9518
2020-01-12T13:24:24.1137325Z .................................................................................................... 5800/9518
2020-01-12T13:24:24.1137325Z .................................................................................................... 5800/9518
2020-01-12T13:24:33.2697150Z .................................................................................................... 5900/9518
2020-01-12T13:24:42.4368605Z ..............................ii...i..ii...........i................................................ 6000/9518
2020-01-12T13:24:59.4498775Z .................................................................................................... 6200/9518
2020-01-12T13:25:06.9809320Z .................................................................................................... 6300/9518
2020-01-12T13:25:06.9809320Z .................................................................................................... 6300/9518
2020-01-12T13:25:17.7533664Z ..........................................................i..ii..................................... 6400/9518
2020-01-12T13:25:42.6597491Z .................................................................................................... 6600/9518
2020-01-12T13:25:44.7395948Z ..................................i................................................................. 6700/9518
2020-01-12T13:25:46.8570197Z .................................................................................................... 6800/9518
2020-01-12T13:25:49.2566874Z ..................................i................................................................. 6900/9518
---
2020-01-12T13:27:16.4739934Z .................................................................................................... 7500/9518
2020-01-12T13:27:20.7361078Z .................................................................................................... 7600/9518
2020-01-12T13:27:26.4501392Z .................................................................................................... 7700/9518
2020-01-12T13:27:33.3948452Z .................................................................................................... 7800/9518
2020-01-12T13:27:42.4355186Z ...................................................................................iiii............. 7900/9518
2020-01-12T13:27:57.4169422Z .................i......i........................................................................... 8100/9518
2020-01-12T13:28:02.1886918Z .................................................................................................... 8200/9518
2020-01-12T13:28:14.1318430Z .................................................................................................... 8300/9518
2020-01-12T13:28:23.1383650Z .................................................................................................... 8400/9518
---
2020-01-12T13:30:35.1399844Z  finished in 6.440
2020-01-12T13:30:35.1583466Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-12T13:30:35.3503664Z 
2020-01-12T13:30:35.3505638Z running 166 tests
2020-01-12T13:30:38.1637537Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-12T13:30:40.2038512Z i.i.i...iii...iiiiiii......................iii............ii......
2020-01-12T13:30:40.2040931Z 
2020-01-12T13:30:40.2045850Z  finished in 5.046
2020-01-12T13:30:40.2201896Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-12T13:30:40.3689226Z 
---
2020-01-12T13:30:42.2557163Z  finished in 2.035
2020-01-12T13:30:42.2719707Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-12T13:30:42.4138622Z 
2020-01-12T13:30:42.4138836Z running 9 tests
2020-01-12T13:30:42.4139622Z iiiiiiiii
2020-01-12T13:30:42.4140117Z 
2020-01-12T13:30:42.4140155Z  finished in 0.142
2020-01-12T13:30:42.4282537Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-12T13:30:42.5946098Z 
---
2020-01-12T13:31:01.0400461Z  finished in 18.611
2020-01-12T13:31:01.0595273Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-12T13:31:01.2330109Z 
2020-01-12T13:31:01.2330555Z running 124 tests
2020-01-12T13:31:21.6844879Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2020-01-12T13:31:25.1955117Z .i.iii.....iiiiii.....ii
2020-01-12T13:31:25.1955540Z 
2020-01-12T13:31:25.1955627Z  finished in 24.136
2020-01-12T13:31:25.1961161Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-12T13:31:25.1961428Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-12T13:32:00.6200086Z 
2020-01-12T13:32:00.6200664Z ---- [ui] ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----
2020-01-12T13:32:00.6201125Z diff of stderr:
2020-01-12T13:32:00.6201469Z 
2020-01-12T13:32:00.6202298Z 45 LL | #[allow(test_lint)]
2020-01-12T13:32:00.6202529Z 46    |         ^^^^^^^^^ overruled by previous forbid
2020-01-12T13:32:00.6203166Z - error: aborting due to 4 previous errors
2020-01-12T13:32:00.6203166Z - error: aborting due to 4 previous errors
2020-01-12T13:32:00.6203436Z + error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-01-12T13:32:00.6203835Z +   --> $DIR/lint-plugin-forbid-attrs.rs:11:9
2020-01-12T13:32:00.6204099Z +    |
2020-01-12T13:32:00.6204279Z + LL | #![forbid(test_lint)]
2020-01-12T13:32:00.6204658Z +    |           --------- `forbid` level set here
2020-01-12T13:32:00.6204909Z + ...
2020-01-12T13:32:00.6205086Z + LL | #[allow(test_lint)]
2020-01-12T13:32:00.6205261Z +    |         ^^^^^^^^^ overruled by previous forbid
2020-01-12T13:32:00.6205630Z + error: aborting due to 5 previous errors
2020-01-12T13:32:00.6205827Z 49 
2020-01-12T13:32:00.6206247Z 50 For more information about this error, try `rustc --explain E0453`.
2020-01-12T13:32:00.6206477Z 51 
2020-01-12T13:32:00.6206477Z 51 
2020-01-12T13:32:00.6206633Z 
2020-01-12T13:32:00.6206827Z 
2020-01-12T13:32:00.6207007Z The actual stderr differed from the expected stderr.
2020-01-12T13:32:00.6207483Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/lint-plugin-forbid-attrs.stderr
2020-01-12T13:32:00.6207976Z To update references, rerun the tests and pass the `--bless` flag
2020-01-12T13:32:00.6208809Z To only update this specific test, also pass `--test-args lint-plugin-forbid-attrs.rs`
2020-01-12T13:32:00.6209248Z error: 1 errors occurred comparing output.
2020-01-12T13:32:00.6209434Z status: exit code: 1
2020-01-12T13:32:00.6209434Z status: exit code: 1
2020-01-12T13:32:00.6210405Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary" "-A" "unused"
2020-01-12T13:32:00.6211146Z ------------------------------------------
2020-01-12T13:32:00.6211686Z 
2020-01-12T13:32:00.6212335Z ------------------------------------------
2020-01-12T13:32:00.6212744Z stderr:
2020-01-12T13:32:00.6212744Z stderr:
2020-01-12T13:32:00.6213119Z ------------------------------------------
2020-01-12T13:32:00.6213370Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-01-12T13:32:00.6214017Z    |
2020-01-12T13:32:00.6214017Z    |
2020-01-12T13:32:00.6214236Z LL | #![forbid(test_lint)]
2020-01-12T13:32:00.6214614Z    |           --------- `forbid` level set here
2020-01-12T13:32:00.6214837Z ...
2020-01-12T13:32:00.6215035Z LL | #[allow(test_lint)]
2020-01-12T13:32:00.6215211Z    |         ^^^^^^^^^ overruled by previous forbid
2020-01-12T13:32:00.6215364Z 
2020-01-12T13:32:00.6215673Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-01-12T13:32:00.6216396Z    |
2020-01-12T13:32:00.6216396Z    |
2020-01-12T13:32:00.6216595Z LL | #![forbid(test_lint)]
2020-01-12T13:32:00.6216970Z    |           --------- `forbid` level set here
2020-01-12T13:32:00.6217216Z ...
2020-01-12T13:32:00.6217397Z LL | #[allow(test_lint)]
2020-01-12T13:32:00.6217572Z    |         ^^^^^^^^^ overruled by previous forbid
2020-01-12T13:32:00.6217728Z 
2020-01-12T13:32:00.6218289Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-01-12T13:32:00.6219210Z    |
2020-01-12T13:32:00.6219390Z LL | #![plugin(lint_plugin_test)]
2020-01-12T13:32:00.6219593Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-12T13:32:00.6219771Z    |
2020-01-12T13:32:00.6219771Z    |
2020-01-12T13:32:00.6219945Z    = note: `#[warn(deprecated)]` on by default
2020-01-12T13:32:00.6220112Z 
2020-01-12T13:32:00.6220505Z error: item is named 'lintme'
2020-01-12T13:32:00.6221194Z    |
2020-01-12T13:32:00.6221194Z    |
2020-01-12T13:32:00.6221575Z LL | fn lintme() { } //~ ERROR item is named 'lintme'
2020-01-12T13:32:00.6222470Z    |
2020-01-12T13:32:00.6222623Z note: lint level defined here
2020-01-12T13:32:00.6223070Z   --> /checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs:7:11
2020-01-12T13:32:00.6223231Z    |
2020-01-12T13:32:00.6223231Z    |
2020-01-12T13:32:00.6223374Z LL | #![forbid(test_lint)]
2020-01-12T13:32:00.6223611Z 
2020-01-12T13:32:00.6223611Z 
2020-01-12T13:32:00.6223752Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-01-12T13:32:00.6224230Z    |
2020-01-12T13:32:00.6224230Z    |
2020-01-12T13:32:00.6224368Z LL | #![forbid(test_lint)]
2020-01-12T13:32:00.6224856Z    |           --------- `forbid` level set here
2020-01-12T13:32:00.6225219Z ...
2020-01-12T13:32:00.6225366Z LL | #[allow(test_lint)]
2020-01-12T13:32:00.6225500Z    |         ^^^^^^^^^ overruled by previous forbid
2020-01-12T13:32:00.6225613Z 
2020-01-12T13:32:00.6225761Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-01-12T13:32:00.6226263Z    |
2020-01-12T13:32:00.6226263Z    |
2020-01-12T13:32:00.6226408Z LL | #![forbid(test_lint)]
2020-01-12T13:32:00.6226717Z    |           --------- `forbid` level set here
2020-01-12T13:32:00.6226890Z ...
2020-01-12T13:32:00.6227037Z LL | #[allow(test_lint)]
2020-01-12T13:32:00.6227164Z    |         ^^^^^^^^^ overruled by previous forbid
2020-01-12T13:32:00.6227423Z error: aborting due to 5 previous errors
2020-01-12T13:32:00.6227536Z 
2020-01-12T13:32:00.6227871Z For more information about this error, try `rustc --explain E0453`.
2020-01-12T13:32:00.6228039Z 
---
2020-01-12T13:32:00.6229711Z 38    |
2020-01-12T13:32:00.6229833Z 39    = note: `forbid` lint level was set on command line
2020-01-12T13:32:00.6229952Z 40 
2020-01-12T13:32:00.6230376Z - error: aborting due to 4 previous errors
2020-01-12T13:32:00.6230540Z + error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-01-12T13:32:00.6230851Z +   --> $DIR/lint-plugin-forbid-cmdline.rs:10:9
2020-01-12T13:32:00.6231018Z +    |
2020-01-12T13:32:00.6231138Z + LL | #[allow(test_lint)]
2020-01-12T13:32:00.6231258Z +    |         ^^^^^^^^^ overruled by previous forbid
2020-01-12T13:32:00.6231516Z +    = note: `forbid` lint level was set on command line
2020-01-12T13:32:00.6231746Z + 
2020-01-12T13:32:00.6231902Z + error: aborting due to 5 previous errors
2020-01-12T13:32:00.6232021Z 42 
2020-01-12T13:32:00.6232021Z 42 
2020-01-12T13:32:00.6232391Z 43 For more information about this error, try `rustc --explain E0453`.
2020-01-12T13:32:00.6232697Z 44 
2020-01-12T13:32:00.6232802Z 
2020-01-12T13:32:00.6233072Z 
2020-01-12T13:32:00.6233206Z The actual stderr differed from the expected stderr.
2020-01-12T13:32:00.6233575Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/lint-plugin-forbid-cmdline.stderr
2020-01-12T13:32:00.6233921Z To update references, rerun the tests and pass the `--bless` flag
2020-01-12T13:32:00.6234401Z To only update this specific test, also pass `--test-args lint-plugin-forbid-cmdline.rs`
2020-01-12T13:32:00.6234673Z error: 1 errors occurred comparing output.
2020-01-12T13:32:00.6234787Z status: exit code: 1
2020-01-12T13:32:00.6234787Z status: exit code: 1
2020-01-12T13:32:00.6235592Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-forbid-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-F" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary" "-A" "unused"
2020-01-12T13:32:00.6236099Z ------------------------------------------
2020-01-12T13:32:00.6236229Z 
2020-01-12T13:32:00.6236510Z ------------------------------------------
2020-01-12T13:32:00.6236667Z stderr:
2020-01-12T13:32:00.6236667Z stderr:
2020-01-12T13:32:00.6236936Z ------------------------------------------
2020-01-12T13:32:00.6237090Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-01-12T13:32:00.6237755Z    |
2020-01-12T13:32:00.6237755Z    |
2020-01-12T13:32:00.6237877Z LL | #[allow(test_lint)] //~ ERROR allow(test_lint) overruled by outer forbid(test_lint)
2020-01-12T13:32:00.6238019Z    |         ^^^^^^^^^ overruled by previous forbid
2020-01-12T13:32:00.6238268Z    = note: `forbid` lint level was set on command line
2020-01-12T13:32:00.6238371Z 
2020-01-12T13:32:00.6238371Z 
2020-01-12T13:32:00.6239031Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-01-12T13:32:00.6241294Z    |
2020-01-12T13:32:00.6241294Z    |
2020-01-12T13:32:00.6241594Z LL | #[allow(test_lint)] //~ ERROR allow(test_lint) overruled by outer forbid(test_lint)
2020-01-12T13:32:00.6241736Z    |         ^^^^^^^^^ overruled by previous forbid
2020-01-12T13:32:00.6242013Z    = note: `forbid` lint level was set on command line
2020-01-12T13:32:00.6242126Z 
2020-01-12T13:32:00.6242126Z 
2020-01-12T13:32:00.6242667Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-01-12T13:32:00.6243453Z    |
2020-01-12T13:32:00.6244855Z LL | #![plugin(lint_plugin_test)]
2020-01-12T13:32:00.6245090Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-12T13:32:00.6245304Z    |
2020-01-12T13:32:00.6245304Z    |
2020-01-12T13:32:00.6245443Z    = note: `#[warn(deprecated)]` on by default
2020-01-12T13:32:00.6245528Z 
2020-01-12T13:32:00.6245885Z error: item is named 'lintme'
2020-01-12T13:32:00.6246139Z    |
2020-01-12T13:32:00.6246139Z    |
2020-01-12T13:32:00.6246345Z LL | fn lintme() { } //~ ERROR item is named 'lintme'
2020-01-12T13:32:00.6246416Z    |
2020-01-12T13:32:00.6246769Z    = note: requested on the command line with `-F test-lint`
2020-01-12T13:32:00.6246809Z 
2020-01-12T13:32:00.6246809Z 
2020-01-12T13:32:00.6246847Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-01-12T13:32:00.6247127Z    |
2020-01-12T13:32:00.6247127Z    |
2020-01-12T13:32:00.6247166Z LL | #[allow(test_lint)] //~ ERROR allow(test_lint) overruled by outer forbid(test_lint)
2020-01-12T13:32:00.6247207Z    |         ^^^^^^^^^ overruled by previous forbid
2020-01-12T13:32:00.6247292Z    = note: `forbid` lint level was set on command line
2020-01-12T13:32:00.6247386Z 
2020-01-12T13:32:00.6247386Z 
2020-01-12T13:32:00.6247440Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-01-12T13:32:00.6247693Z    |
2020-01-12T13:32:00.6247693Z    |
2020-01-12T13:32:00.6247747Z LL | #[allow(test_lint)] //~ ERROR allow(test_lint) overruled by outer forbid(test_lint)
2020-01-12T13:32:00.6247796Z    |         ^^^^^^^^^ overruled by previous forbid
2020-01-12T13:32:00.6247863Z    = note: `forbid` lint level was set on command line
2020-01-12T13:32:00.6247903Z 
2020-01-12T13:32:00.6247938Z error: aborting due to 5 previous errors
2020-01-12T13:32:00.6247960Z 
---
2020-01-12T13:32:00.6248406Z 
2020-01-12T13:32:00.6248593Z ---- [ui] ui-fulldeps/lint-tool-test.rs stdout ----
2020-01-12T13:32:00.6248639Z diff of stderr:
2020-01-12T13:32:00.6248660Z 
2020-01-12T13:32:00.6248693Z 96 LL | #[allow(test_group)]
2020-01-12T13:32:00.6248748Z 97    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2020-01-12T13:32:00.6248784Z 98 
2020-01-12T13:32:00.6249158Z + warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-12T13:32:00.6249337Z +   --> $DIR/lint-tool-test.rs:9:23
2020-01-12T13:32:00.6249396Z +    |
2020-01-12T13:32:00.6249432Z + LL | #![cfg_attr(foo, warn(test_lint))]
2020-01-12T13:32:00.6249471Z +    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2020-01-12T13:32:00.6249524Z + 
2020-01-12T13:32:00.6249776Z + warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-12T13:32:00.6249945Z +   --> $DIR/lint-tool-test.rs:13:9
2020-01-12T13:32:00.6250036Z + LL | #![deny(clippy_group)]
2020-01-12T13:32:00.6250074Z +    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2020-01-12T13:32:00.6250109Z + 
2020-01-12T13:32:00.6250109Z + 
2020-01-12T13:32:00.6250521Z + warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-12T13:32:00.6250688Z +   --> $DIR/lint-tool-test.rs:29:9
2020-01-12T13:32:00.6250720Z +    |
2020-01-12T13:32:00.6250768Z + LL | #[allow(test_group)]
2020-01-12T13:32:00.6250803Z +    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2020-01-12T13:32:00.6250883Z 99 error: aborting due to 2 previous errors
2020-01-12T13:32:00.6250914Z 100 
2020-01-12T13:32:00.6250942Z 101 
2020-01-12T13:32:00.6250961Z 
2020-01-12T13:32:00.6250961Z 
2020-01-12T13:32:00.6250995Z 
2020-01-12T13:32:00.6251028Z The actual stderr differed from the expected stderr.
2020-01-12T13:32:00.6251613Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/lint-tool-test.stderr
2020-01-12T13:32:00.6251864Z To update references, rerun the tests and pass the `--bless` flag
2020-01-12T13:32:00.6252063Z To only update this specific test, also pass `--test-args lint-tool-test.rs`
2020-01-12T13:32:00.6252123Z error: 1 errors occurred comparing output.
2020-01-12T13:32:00.6252253Z status: exit code: 1
2020-01-12T13:32:00.6252253Z status: exit code: 1
2020-01-12T13:32:00.6252916Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "foo" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary" "-A" "unused"
2020-01-12T13:32:00.6253257Z ------------------------------------------
2020-01-12T13:32:00.6253283Z 
2020-01-12T13:32:00.6253473Z ------------------------------------------
2020-01-12T13:32:00.6253507Z stderr:
2020-01-12T13:32:00.6253507Z stderr:
2020-01-12T13:32:00.6253678Z ------------------------------------------
2020-01-12T13:32:00.6253944Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-12T13:32:00.6254177Z    |
2020-01-12T13:32:00.6254177Z    |
2020-01-12T13:32:00.6254227Z LL | #![cfg_attr(foo, warn(test_lint))]
2020-01-12T13:32:00.6254266Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2020-01-12T13:32:00.6254352Z    = note: `#[warn(renamed_and_removed_lints)]` on by default
2020-01-12T13:32:00.6254549Z 
2020-01-12T13:32:00.6254549Z 
2020-01-12T13:32:00.6254785Z warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-12T13:32:00.6255020Z    |
2020-01-12T13:32:00.6255051Z LL | #![deny(clippy_group)]
2020-01-12T13:32:00.6255092Z    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2020-01-12T13:32:00.6255131Z 
2020-01-12T13:32:00.6255131Z 
2020-01-12T13:32:00.6255366Z warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-12T13:32:00.6255595Z    |
2020-01-12T13:32:00.6255595Z    |
2020-01-12T13:32:00.6255625Z LL | #[allow(test_group)]
2020-01-12T13:32:00.6255660Z    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2020-01-12T13:32:00.6255683Z 
2020-01-12T13:32:00.6255924Z warning: unknown lint: `this_lint_does_not_exist`
2020-01-12T13:32:00.6256191Z    |
2020-01-12T13:32:00.6256191Z    |
2020-01-12T13:32:00.6256247Z LL | #[deny(this_lint_does_not_exist)] //~ WARNING unknown lint: `this_lint_does_not_exist`
2020-01-12T13:32:00.6256316Z    |
2020-01-12T13:32:00.6256373Z    = note: `#[warn(unknown_lints)]` on by default
2020-01-12T13:32:00.6256396Z 
2020-01-12T13:32:00.6256396Z 
2020-01-12T13:32:00.6256643Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-12T13:32:00.6256884Z    |
2020-01-12T13:32:00.6256884Z    |
2020-01-12T13:32:00.6256916Z LL | #![cfg_attr(foo, warn(test_lint))]
2020-01-12T13:32:00.6256954Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2020-01-12T13:32:00.6256996Z 
2020-01-12T13:32:00.6257241Z warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-12T13:32:00.6257486Z    |
2020-01-12T13:32:00.6257518Z LL | #![deny(clippy_group)]
2020-01-12T13:32:00.6257554Z    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2020-01-12T13:32:00.6257631Z 
2020-01-12T13:32:00.6257631Z 
2020-01-12T13:32:00.6257915Z warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-12T13:32:00.6258140Z    |
2020-01-12T13:32:00.6258140Z    |
2020-01-12T13:32:00.6258188Z LL | #[allow(test_group)]
2020-01-12T13:32:00.6258224Z    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2020-01-12T13:32:00.6258248Z 
2020-01-12T13:32:00.6258545Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-01-12T13:32:00.6258864Z    |
2020-01-12T13:32:00.6258864Z    |
2020-01-12T13:32:00.6258912Z LL | #![plugin(lint_tool_test)]
2020-01-12T13:32:00.6258986Z    |
2020-01-12T13:32:00.6259201Z    = note: `#[warn(deprecated)]` on by default
2020-01-12T13:32:00.6259224Z 
2020-01-12T13:32:00.6259224Z 
2020-01-12T13:32:00.6259457Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-12T13:32:00.6259685Z    |
2020-01-12T13:32:00.6259685Z    |
2020-01-12T13:32:00.6259716Z LL | #![cfg_attr(foo, warn(test_lint))]
2020-01-12T13:32:00.6259752Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2020-01-12T13:32:00.6259795Z 
2020-01-12T13:32:00.6260028Z warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-12T13:32:00.6260265Z    |
2020-01-12T13:32:00.6260296Z LL | #![deny(clippy_group)]
2020-01-12T13:32:00.6260330Z    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2020-01-12T13:32:00.6260353Z 
2020-01-12T13:32:00.6260353Z 
2020-01-12T13:32:00.6260523Z error: item is named 'lintme'
2020-01-12T13:32:00.6260731Z    |
2020-01-12T13:32:00.6260731Z    |
2020-01-12T13:32:00.6260911Z LL | fn lintme() { } //~ ERROR item is named 'lintme'
2020-01-12T13:32:00.6260976Z    |
2020-01-12T13:32:00.6261023Z note: lint level defined here
2020-01-12T13:32:00.6261195Z   --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:13:9
2020-01-12T13:32:00.6261228Z    |
2020-01-12T13:32:00.6261228Z    |
2020-01-12T13:32:00.6261258Z LL | #![deny(clippy_group)]
2020-01-12T13:32:00.6261307Z    |         ^^^^^^^^^^^^
2020-01-12T13:32:00.6261348Z    = note: `#[deny(clippy::test_lint)]` implied by `#[deny(clippy::group)]`
2020-01-12T13:32:00.6261372Z 
2020-01-12T13:32:00.6261541Z error: item is named 'lintmetoo'
2020-01-12T13:32:00.6261749Z    |
2020-01-12T13:32:00.6261749Z    |
2020-01-12T13:32:00.6261926Z LL |     fn lintmetoo() { } //~ ERROR item is named 'lintmetoo'
2020-01-12T13:32:00.6262007Z    |
2020-01-12T13:32:00.6262037Z note: lint level defined here
2020-01-12T13:32:00.6262227Z   --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:13:9
2020-01-12T13:32:00.6262261Z    |
2020-01-12T13:32:00.6262261Z    |
2020-01-12T13:32:00.6262291Z LL | #![deny(clippy_group)]
2020-01-12T13:32:00.6262337Z    |         ^^^^^^^^^^^^
2020-01-12T13:32:00.6262374Z    = note: `#[deny(clippy::test_group)]` implied by `#[deny(clippy::group)]`
2020-01-12T13:32:00.6262397Z 
2020-01-12T13:32:00.6262628Z warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-12T13:32:00.6262868Z    |
2020-01-12T13:32:00.6262868Z    |
2020-01-12T13:32:00.6262898Z LL | #[allow(test_group)]
2020-01-12T13:32:00.6262948Z    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2020-01-12T13:32:00.6262973Z 
2020-01-12T13:32:00.6263254Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-12T13:32:00.6263684Z    |
2020-01-12T13:32:00.6263684Z    |
2020-01-12T13:32:00.6263717Z LL | #![cfg_attr(foo, warn(test_lint))]
2020-01-12T13:32:00.6263755Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2020-01-12T13:32:00.6265075Z 
2020-01-12T13:32:00.6265481Z warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-12T13:32:00.6265941Z    |
2020-01-12T13:32:00.6265983Z LL | #![deny(clippy_group)]
2020-01-12T13:32:00.6266031Z    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2020-01-12T13:32:00.6266062Z 
2020-01-12T13:32:00.6266062Z 
2020-01-12T13:32:00.6266407Z warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-12T13:32:00.6266698Z    |
2020-01-12T13:32:00.6266698Z    |
2020-01-12T13:32:00.6266757Z LL | #[allow(test_group)]
2020-01-12T13:32:00.6266804Z    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2020-01-12T13:32:00.6266899Z error: aborting due to 2 previous errors
2020-01-12T13:32:00.6266928Z 
2020-01-12T13:32:00.6266953Z 
2020-01-12T13:32:00.6267168Z ------------------------------------------
---
2020-01-12T13:32:00.6269298Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-12T13:32:00.6269341Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-12T13:32:00.6269382Z 
2020-01-12T13:32:00.6269402Z 
2020-01-12T13:32:00.6270969Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-12T13:32:00.6271196Z 
2020-01-12T13:32:00.6271220Z 
2020-01-12T13:32:00.6271330Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-12T13:32:00.6271392Z Build completed unsuccessfully in 0:54:58
2020-01-12T13:32:00.6271392Z Build completed unsuccessfully in 0:54:58
2020-01-12T13:32:00.6279243Z == clock drift check ==
2020-01-12T13:32:00.6301872Z   local time: Sun Jan 12 13:32:00 UTC 2020
2020-01-12T13:32:00.9005872Z   network time: Sun, 12 Jan 2020 13:32:00 GMT
2020-01-12T13:32:00.9006140Z == end clock drift check ==
2020-01-12T13:32:01.9097908Z 
2020-01-12T13:32:01.9175635Z ##[error]Bash exited with code '1'.
2020-01-12T13:32:01.9208719Z ##[section]Starting: Checkout
2020-01-12T13:32:01.9210070Z ==============================================================================
2020-01-12T13:32:01.9210252Z Task         : Get sources
2020-01-12T13:32:01.9210288Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
