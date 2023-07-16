plain
2020-01-10T16:11:38.4822420Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-10T16:11:38.4919419Z ##[command]git config gc.auto 0
2020-01-10T16:11:38.4997966Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-10T16:11:38.5062796Z ##[command]git config --get-all http.proxy
2020-01-10T16:11:38.5225395Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68080/merge:refs/remotes/pull/68080/merge
---
2020-01-10T17:09:49.4972201Z .............................i...............i...................................................... 4900/9503
2020-01-10T17:09:59.3010034Z .................................................................................................... 5000/9503
2020-01-10T17:10:05.6247739Z ..........................................................................i......................... 5100/9503
2020-01-10T17:10:11.7965516Z .................................................................................................... 5200/9503
2020-01-10T17:10:21.2300173Z .........................................ii.ii...........i.......................................... 5300/9503
2020-01-10T17:10:31.9282580Z .................................................................................................... 5500/9503
2020-01-10T17:10:41.6841076Z .................................................................................................... 5600/9503
2020-01-10T17:10:48.8479383Z .........................i.......................................................................... 5700/9503
2020-01-10T17:10:55.1364532Z .................................................................................................... 5800/9503
2020-01-10T17:10:55.1364532Z .................................................................................................... 5800/9503
2020-01-10T17:11:06.6116071Z .................................................................................................... 5900/9503
2020-01-10T17:11:17.5655929Z ................ii...i..ii...........i.............................................................. 6000/9503
2020-01-10T17:11:35.8505529Z .................................................................................................... 6200/9503
2020-01-10T17:11:44.1878617Z .................................................................................................... 6300/9503
2020-01-10T17:11:44.1878617Z .................................................................................................... 6300/9503
2020-01-10T17:11:58.6985338Z ...........................................i..ii.................................................... 6400/9503
2020-01-10T17:12:20.3488432Z .................................................................................................... 6600/9503
2020-01-10T17:12:22.4361733Z ..................i................................................................................. 6700/9503
2020-01-10T17:12:24.7072794Z .................................................................................................... 6800/9503
2020-01-10T17:12:27.2650760Z ..................i................................................................................. 6900/9503
---
2020-01-10T17:14:06.3895606Z .................................................................................................... 7500/9503
2020-01-10T17:14:10.2149880Z .................................................................................................... 7600/9503
2020-01-10T17:14:16.1863490Z .................................................................................................... 7700/9503
2020-01-10T17:14:24.6459922Z .................................................................................................... 7800/9503
2020-01-10T17:14:34.6821617Z .....................................................................iiii........................... 7900/9503
2020-01-10T17:14:50.2975568Z ...i......i......................................................................................... 8100/9503
2020-01-10T17:14:55.6181201Z .................................................................................................... 8200/9503
2020-01-10T17:15:10.9791015Z .................................................................................................... 8300/9503
2020-01-10T17:15:19.5997246Z .................................................................................................... 8400/9503
---
2020-01-10T17:17:46.4017378Z  finished in 7.077
2020-01-10T17:17:46.4220159Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T17:17:46.6328714Z 
2020-01-10T17:17:46.6328945Z running 166 tests
2020-01-10T17:17:49.7444587Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-10T17:17:52.0177564Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-10T17:17:52.0181299Z 
2020-01-10T17:17:52.0189006Z  finished in 5.596
2020-01-10T17:17:52.0407674Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T17:17:52.2177153Z 
---
2020-01-10T17:17:54.2510411Z  finished in 2.210
2020-01-10T17:17:54.2708794Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T17:17:54.4367861Z 
2020-01-10T17:17:54.4368204Z running 9 tests
2020-01-10T17:17:54.4369273Z iiiiiiiii
2020-01-10T17:17:54.4369566Z 
2020-01-10T17:17:54.4369604Z  finished in 0.165
2020-01-10T17:17:54.4572187Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T17:17:54.6598629Z 
---
2020-01-10T17:18:15.1512282Z  finished in 20.693
2020-01-10T17:18:15.1738260Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T17:18:15.3623327Z 
2020-01-10T17:18:15.3626235Z running 124 tests
2020-01-10T17:18:39.5518521Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2020-01-10T17:18:43.6182521Z .i.iii.....iiiiii.....ii
2020-01-10T17:18:43.6184065Z 
2020-01-10T17:18:43.6186071Z  finished in 28.444
2020-01-10T17:18:43.6189770Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-10T17:18:43.6190460Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-10T17:19:24.2731762Z 
2020-01-10T17:19:24.2732663Z ---- [ui] ui-fulldeps/lint-plugin-deny-attr.rs stdout ----
2020-01-10T17:19:24.2736643Z diff of stderr:
2020-01-10T17:19:24.2736703Z 
2020-01-10T17:19:24.2736744Z 12 LL | fn lintme() { }
2020-01-10T17:19:24.2736847Z 14    |
2020-01-10T17:19:24.2741999Z - note: lint level defined here
2020-01-10T17:19:24.2742099Z + note: the lint level is defined here
2020-01-10T17:19:24.2742361Z 16   --> $DIR/lint-plugin-deny-attr.rs:7:9
2020-01-10T17:19:24.2742361Z 16   --> $DIR/lint-plugin-deny-attr.rs:7:9
2020-01-10T17:19:24.2742408Z 17    |
2020-01-10T17:19:24.2742630Z 18 LL | #![deny(test_lint)]
2020-01-10T17:19:24.2742709Z 
2020-01-10T17:19:24.2742755Z 
2020-01-10T17:19:24.2742803Z The actual stderr differed from the expected stderr.
2020-01-10T17:19:24.2743194Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/lint-plugin-deny-attr.stderr
2020-01-10T17:19:24.2743474Z To update references, rerun the tests and pass the `--bless` flag
2020-01-10T17:19:24.2743736Z To only update this specific test, also pass `--test-args lint-plugin-deny-attr.rs`
2020-01-10T17:19:24.2743833Z error: 1 errors occurred comparing output.
2020-01-10T17:19:24.2743879Z status: exit code: 1
2020-01-10T17:19:24.2743879Z status: exit code: 1
2020-01-10T17:19:24.2745266Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-deny-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary" "-A" "unused"
2020-01-10T17:19:24.2745689Z ------------------------------------------
2020-01-10T17:19:24.2745718Z 
2020-01-10T17:19:24.2745902Z ------------------------------------------
2020-01-10T17:19:24.2745940Z stderr:
2020-01-10T17:19:24.2745940Z stderr:
2020-01-10T17:19:24.2746141Z ------------------------------------------
2020-01-10T17:19:24.2746524Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-01-10T17:19:24.2747190Z    |
2020-01-10T17:19:24.2747229Z LL | #![plugin(lint_plugin_test)]
2020-01-10T17:19:24.2747293Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-10T17:19:24.2747449Z    |
2020-01-10T17:19:24.2747449Z    |
2020-01-10T17:19:24.2747505Z    = note: `#[warn(deprecated)]` on by default
2020-01-10T17:19:24.2747532Z 
2020-01-10T17:19:24.2747730Z error: item is named 'lintme'
2020-01-10T17:19:24.2747995Z    |
2020-01-10T17:19:24.2747995Z    |
2020-01-10T17:19:24.2748188Z LL | fn lintme() { } //~ ERROR item is named 'lintme'
2020-01-10T17:19:24.2748285Z    |
2020-01-10T17:19:24.2748323Z note: the lint level is defined here
2020-01-10T17:19:24.2748699Z   --> /checkout/src/test/ui-fulldeps/lint-plugin-deny-attr.rs:7:9
2020-01-10T17:19:24.2748758Z    |
---
2020-01-10T17:19:24.2749687Z 
2020-01-10T17:19:24.2749864Z ---- [ui] ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----
2020-01-10T17:19:24.2749900Z diff of stderr:
2020-01-10T17:19:24.2749941Z 
2020-01-10T17:19:24.2749972Z 30 LL | fn lintme() { }
2020-01-10T17:19:24.2750037Z 32    |
2020-01-10T17:19:24.2750215Z - note: lint level defined here
2020-01-10T17:19:24.2750252Z + note: the lint level is defined here
2020-01-10T17:19:24.2750421Z 34   --> $DIR/lint-plugin-forbid-attrs.rs:7:11
2020-01-10T17:19:24.2750421Z 34   --> $DIR/lint-plugin-forbid-attrs.rs:7:11
2020-01-10T17:19:24.2750480Z 35    |
2020-01-10T17:19:24.2750514Z 36 LL | #![forbid(test_lint)]
2020-01-10T17:19:24.2750556Z 
2020-01-10T17:19:24.2750607Z The actual stderr differed from the expected stderr.
2020-01-10T17:19:24.2751394Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/lint-plugin-forbid-attrs.stderr
2020-01-10T17:19:24.2751394Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/lint-plugin-forbid-attrs.stderr
2020-01-10T17:19:24.2751688Z To update references, rerun the tests and pass the `--bless` flag
2020-01-10T17:19:24.2751976Z To only update this specific test, also pass `--test-args lint-plugin-forbid-attrs.rs`
2020-01-10T17:19:24.2752055Z error: 1 errors occurred comparing output.
2020-01-10T17:19:24.2752117Z status: exit code: 1
2020-01-10T17:19:24.2752117Z status: exit code: 1
2020-01-10T17:19:24.2753144Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary" "-A" "unused"
2020-01-10T17:19:24.2753576Z ------------------------------------------
2020-01-10T17:19:24.2753610Z 
2020-01-10T17:19:24.2753840Z ------------------------------------------
2020-01-10T17:19:24.2753884Z stderr:
2020-01-10T17:19:24.2753884Z stderr:
2020-01-10T17:19:24.2754096Z ------------------------------------------
2020-01-10T17:19:24.2754166Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-01-10T17:19:24.2754640Z    |
2020-01-10T17:19:24.2754640Z    |
2020-01-10T17:19:24.2754701Z LL | #![forbid(test_lint)]
2020-01-10T17:19:24.2754882Z    |           --------- `forbid` level set here
2020-01-10T17:19:24.2754918Z ...
2020-01-10T17:19:24.2754952Z LL | #[allow(test_lint)]
2020-01-10T17:19:24.2755006Z    |         ^^^^^^^^^ overruled by previous forbid
2020-01-10T17:19:24.2755030Z 
2020-01-10T17:19:24.2755067Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-01-10T17:19:24.2755328Z    |
2020-01-10T17:19:24.2755328Z    |
2020-01-10T17:19:24.2755362Z LL | #![forbid(test_lint)]
2020-01-10T17:19:24.2755710Z    |           --------- `forbid` level set here
2020-01-10T17:19:24.2755766Z ...
2020-01-10T17:19:24.2755802Z LL | #[allow(test_lint)]
2020-01-10T17:19:24.2755840Z    |         ^^^^^^^^^ overruled by previous forbid
2020-01-10T17:19:24.2755884Z 
2020-01-10T17:19:24.2756501Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-01-10T17:19:24.2756778Z    |
2020-01-10T17:19:24.2756815Z LL | #![plugin(lint_plugin_test)]
2020-01-10T17:19:24.2756856Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-10T17:19:24.2756915Z    |
2020-01-10T17:19:24.2756915Z    |
2020-01-10T17:19:24.2756951Z    = note: `#[warn(deprecated)]` on by default
2020-01-10T17:19:24.2756982Z 
2020-01-10T17:19:24.2757150Z error: item is named 'lintme'
2020-01-10T17:19:24.2757403Z    |
2020-01-10T17:19:24.2757403Z    |
2020-01-10T17:19:24.2757586Z LL | fn lintme() { } //~ ERROR item is named 'lintme'
2020-01-10T17:19:24.2757676Z    |
2020-01-10T17:19:24.2757711Z note: the lint level is defined here
2020-01-10T17:19:24.2757929Z   --> /checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs:7:11
2020-01-10T17:19:24.2757967Z    |
2020-01-10T17:19:24.2757967Z    |
2020-01-10T17:19:24.2758009Z LL | #![forbid(test_lint)]
2020-01-10T17:19:24.2758086Z 
2020-01-10T17:19:24.2758086Z 
2020-01-10T17:19:24.2758123Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-01-10T17:19:24.2758380Z    |
2020-01-10T17:19:24.2758380Z    |
2020-01-10T17:19:24.2758414Z LL | #![forbid(test_lint)]
2020-01-10T17:19:24.2758600Z    |           --------- `forbid` level set here
2020-01-10T17:19:24.2758653Z ...
2020-01-10T17:19:24.2758688Z LL | #[allow(test_lint)]
2020-01-10T17:19:24.2758725Z    |         ^^^^^^^^^ overruled by previous forbid
2020-01-10T17:19:24.2758803Z error: aborting due to 4 previous errors
2020-01-10T17:19:24.2758826Z 
2020-01-10T17:19:24.2759413Z For more information about this error, try `rustc --explain E0453`.
2020-01-10T17:19:24.2759439Z 
2020-01-10T17:19:24.2759439Z 
2020-01-10T17:19:24.2759625Z ------------------------------------------
2020-01-10T17:19:24.2759651Z 
2020-01-10T17:19:24.2759744Z 
2020-01-10T17:19:24.2759952Z ---- [ui] ui-fulldeps/lint-tool-test.rs stdout ----
2020-01-10T17:19:24.2760007Z diff of stderr:
2020-01-10T17:19:24.2760031Z 
2020-01-10T17:19:24.2760064Z 70 LL | fn lintme() { }
2020-01-10T17:19:24.2760150Z 72    |
2020-01-10T17:19:24.2760315Z - note: lint level defined here
2020-01-10T17:19:24.2760414Z + note: the lint level is defined here
2020-01-10T17:19:24.2760786Z 74   --> $DIR/lint-tool-test.rs:13:9
2020-01-10T17:19:24.2760786Z 74   --> $DIR/lint-tool-test.rs:13:9
2020-01-10T17:19:24.2760823Z 75    |
2020-01-10T17:19:24.2760857Z 76 LL | #![deny(clippy_group)]
2020-01-10T17:19:24.2760880Z 
2020-01-10T17:19:24.2761602Z 83 LL |     fn lintmetoo() { }
2020-01-10T17:19:24.2761693Z 85    |
2020-01-10T17:19:24.2761924Z - note: lint level defined here
2020-01-10T17:19:24.2761991Z + note: the lint level is defined here
2020-01-10T17:19:24.2762202Z 87   --> $DIR/lint-tool-test.rs:13:9
2020-01-10T17:19:24.2762202Z 87   --> $DIR/lint-tool-test.rs:13:9
2020-01-10T17:19:24.2762246Z 88    |
2020-01-10T17:19:24.2762321Z 89 LL | #![deny(clippy_group)]
2020-01-10T17:19:24.2762351Z 
2020-01-10T17:19:24.2762377Z 
2020-01-10T17:19:24.2762421Z The actual stderr differed from the expected stderr.
2020-01-10T17:19:24.2762750Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/lint-tool-test.stderr
2020-01-10T17:19:24.2763003Z To update references, rerun the tests and pass the `--bless` flag
2020-01-10T17:19:24.2763251Z To only update this specific test, also pass `--test-args lint-tool-test.rs`
2020-01-10T17:19:24.2763347Z error: 1 errors occurred comparing output.
2020-01-10T17:19:24.2763390Z status: exit code: 1
2020-01-10T17:19:24.2763390Z status: exit code: 1
2020-01-10T17:19:24.2764309Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "foo" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary" "-A" "unused"
2020-01-10T17:19:24.2764807Z ------------------------------------------
2020-01-10T17:19:24.2764833Z 
2020-01-10T17:19:24.2765166Z ------------------------------------------
2020-01-10T17:19:24.2765217Z stderr:
2020-01-10T17:19:24.2765217Z stderr:
2020-01-10T17:19:24.2765537Z ------------------------------------------
2020-01-10T17:19:24.2765958Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T17:19:24.2766381Z    |
2020-01-10T17:19:24.2766381Z    |
2020-01-10T17:19:24.2766422Z LL | #![cfg_attr(foo, warn(test_lint))]
2020-01-10T17:19:24.2766730Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2020-01-10T17:19:24.2766810Z    = note: `#[warn(renamed_and_removed_lints)]` on by default
2020-01-10T17:19:24.2766836Z 
2020-01-10T17:19:24.2766836Z 
2020-01-10T17:19:24.2767555Z warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T17:19:24.2767812Z    |
2020-01-10T17:19:24.2767865Z LL | #![deny(clippy_group)]
2020-01-10T17:19:24.2767907Z    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2020-01-10T17:19:24.2767933Z 
2020-01-10T17:19:24.2767933Z 
2020-01-10T17:19:24.2768216Z warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T17:19:24.2768552Z    |
2020-01-10T17:19:24.2768552Z    |
2020-01-10T17:19:24.2768605Z LL | #[allow(test_group)]
2020-01-10T17:19:24.2768647Z    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2020-01-10T17:19:24.2768674Z 
2020-01-10T17:19:24.2768710Z warning: unknown lint: `this_lint_does_not_exist`
2020-01-10T17:19:24.2769057Z    |
2020-01-10T17:19:24.2769057Z    |
2020-01-10T17:19:24.2769099Z LL | #[deny(this_lint_does_not_exist)] //~ WARNING unknown lint: `this_lint_does_not_exist`
2020-01-10T17:19:24.2769195Z    |
2020-01-10T17:19:24.2769233Z    = note: `#[warn(unknown_lints)]` on by default
2020-01-10T17:19:24.2769258Z 
2020-01-10T17:19:24.2769258Z 
2020-01-10T17:19:24.2769560Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T17:19:24.2769809Z    |
2020-01-10T17:19:24.2769809Z    |
2020-01-10T17:19:24.2769863Z LL | #![cfg_attr(foo, warn(test_lint))]
2020-01-10T17:19:24.2769906Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2020-01-10T17:19:24.2769936Z 
2020-01-10T17:19:24.2770224Z warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T17:19:24.2770626Z    |
2020-01-10T17:19:24.2770661Z LL | #![deny(clippy_group)]
2020-01-10T17:19:24.2770719Z    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2020-01-10T17:19:24.2770903Z 
2020-01-10T17:19:24.2770903Z 
2020-01-10T17:19:24.2771561Z warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T17:19:24.2771886Z    |
2020-01-10T17:19:24.2771886Z    |
2020-01-10T17:19:24.2771929Z LL | #[allow(test_group)]
2020-01-10T17:19:24.2771993Z    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2020-01-10T17:19:24.2772027Z 
2020-01-10T17:19:24.2772369Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-01-10T17:19:24.2772692Z    |
2020-01-10T17:19:24.2772692Z    |
2020-01-10T17:19:24.2772735Z LL | #![plugin(lint_tool_test)]
2020-01-10T17:19:24.2772851Z    |
2020-01-10T17:19:24.2772895Z    = note: `#[warn(deprecated)]` on by default
2020-01-10T17:19:24.2772925Z 
2020-01-10T17:19:24.2772925Z 
2020-01-10T17:19:24.2773260Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T17:19:24.2773551Z    |
2020-01-10T17:19:24.2773551Z    |
2020-01-10T17:19:24.2773615Z LL | #![cfg_attr(foo, warn(test_lint))]
2020-01-10T17:19:24.2773666Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2020-01-10T17:19:24.2773699Z 
2020-01-10T17:19:24.2774019Z warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T17:19:24.2774333Z    |
2020-01-10T17:19:24.2774375Z LL | #![deny(clippy_group)]
2020-01-10T17:19:24.2774441Z    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2020-01-10T17:19:24.2774472Z 
2020-01-10T17:19:24.2774472Z 
2020-01-10T17:19:24.2774798Z error: item is named 'lintme'
2020-01-10T17:19:24.2775026Z    |
2020-01-10T17:19:24.2775026Z    |
2020-01-10T17:19:24.2775198Z LL | fn lintme() { } //~ ERROR item is named 'lintme'
2020-01-10T17:19:24.2775368Z    |
2020-01-10T17:19:24.2775408Z note: the lint level is defined here
2020-01-10T17:19:24.2775612Z   --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:13:9
2020-01-10T17:19:24.2775664Z    |
2020-01-10T17:19:24.2775664Z    |
2020-01-10T17:19:24.2775696Z LL | #![deny(clippy_group)]
2020-01-10T17:19:24.2775730Z    |         ^^^^^^^^^^^^
2020-01-10T17:19:24.2775785Z    = note: `#[deny(clippy::test_lint)]` implied by `#[deny(clippy::group)]`
2020-01-10T17:19:24.2775872Z 
2020-01-10T17:19:24.2776046Z error: item is named 'lintmetoo'
2020-01-10T17:19:24.2776582Z    |
2020-01-10T17:19:24.2776582Z    |
2020-01-10T17:19:24.2776764Z LL |     fn lintmetoo() { } //~ ERROR item is named 'lintmetoo'
2020-01-10T17:19:24.2776894Z    |
2020-01-10T17:19:24.2776927Z note: the lint level is defined here
2020-01-10T17:19:24.2777112Z   --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:13:9
2020-01-10T17:19:24.2777164Z    |
2020-01-10T17:19:24.2777164Z    |
2020-01-10T17:19:24.2777204Z LL | #![deny(clippy_group)]
2020-01-10T17:19:24.2777238Z    |         ^^^^^^^^^^^^
2020-01-10T17:19:24.2777278Z    = note: `#[deny(clippy::test_group)]` implied by `#[deny(clippy::group)]`
2020-01-10T17:19:24.2777320Z 
2020-01-10T17:19:24.2777572Z warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-01-10T17:19:24.2777818Z    |
2020-01-10T17:19:24.2777818Z    |
2020-01-10T17:19:24.2777852Z LL | #[allow(test_group)]
2020-01-10T17:19:24.2777890Z    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2020-01-10T17:19:24.2777963Z error: aborting due to 2 previous errors
2020-01-10T17:19:24.2777986Z 
2020-01-10T17:19:24.2778006Z 
2020-01-10T17:19:24.2778176Z ------------------------------------------
---
2020-01-10T17:19:24.2779512Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:384:22
2020-01-10T17:19:24.2779559Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-10T17:19:24.2790065Z 
2020-01-10T17:19:24.2790248Z 
2020-01-10T17:19:24.2792591Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-10T17:19:24.2792874Z 
2020-01-10T17:19:24.2792923Z 
2020-01-10T17:19:24.2798713Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-10T17:19:24.2798770Z Build completed unsuccessfully in 1:02:17
2020-01-10T17:19:24.2798770Z Build completed unsuccessfully in 1:02:17
2020-01-10T17:19:24.2857174Z == clock drift check ==
2020-01-10T17:19:24.2875401Z   local time: Fri Jan 10 17:19:24 UTC 2020
2020-01-10T17:19:24.5806077Z   network time: Fri, 10 Jan 2020 17:19:24 GMT
2020-01-10T17:19:24.5810697Z == end clock drift check ==
2020-01-10T17:19:25.3927793Z 
2020-01-10T17:19:25.4039720Z ##[error]Bash exited with code '1'.
2020-01-10T17:19:25.4086982Z ##[section]Starting: Checkout
2020-01-10T17:19:25.4088869Z ==============================================================================
2020-01-10T17:19:25.4088910Z Task         : Get sources
2020-01-10T17:19:25.4088947Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
