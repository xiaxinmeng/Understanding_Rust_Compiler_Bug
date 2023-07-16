plain
2020-02-05T17:19:29.1284505Z ========================== Starting Command Output ===========================
2020-02-05T17:19:29.1287785Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6c489ef7-bba7-4150-b2bb-739851d964e3.sh
2020-02-05T17:19:29.1287832Z 
2020-02-05T17:19:29.1290443Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-05T17:19:29.1296174Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68725/merge to s
2020-02-05T17:19:29.1297734Z Task         : Get sources
2020-02-05T17:19:29.1297772Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-05T17:19:29.1297808Z Version      : 1.0.0
2020-02-05T17:19:29.1297882Z Author       : Microsoft
---
2020-02-05T17:19:29.9990059Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-05T17:19:30.0082280Z ##[command]git config gc.auto 0
2020-02-05T17:19:30.0150929Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-05T17:19:30.0201521Z ##[command]git config --get-all http.proxy
2020-02-05T17:19:30.0340931Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68725/merge:refs/remotes/pull/68725/merge
---
2020-02-05T18:12:55.6828173Z .................................................................................................... 1700/9585
2020-02-05T18:13:00.3915936Z .................................................................................................... 1800/9585
2020-02-05T18:13:12.2488380Z ............................i....................................................................... 1900/9585
2020-02-05T18:13:19.2869281Z .................................................................................................... 2000/9585
2020-02-05T18:13:32.7325548Z ..................iiiii............................................................................. 2100/9585
2020-02-05T18:13:42.4291239Z .................................................................................................... 2300/9585
2020-02-05T18:13:44.7432135Z .................................................................................................... 2400/9585
2020-02-05T18:13:49.4986803Z .................................................................................................... 2500/9585
2020-02-05T18:14:09.0931035Z .................................................................................................... 2600/9585
---
2020-02-05T18:16:42.5061662Z .............................................................i...............i...................... 4900/9585
2020-02-05T18:16:49.8479644Z .................................................................................................... 5000/9585
2020-02-05T18:16:57.4885513Z .................................................................................................... 5100/9585
2020-02-05T18:17:02.1770051Z ....i............................................................................................... 5200/9585
2020-02-05T18:17:12.8804898Z ..............................................................................ii.ii........i...i.... 5300/9585
2020-02-05T18:17:21.2127322Z ................i................................................................................... 5500/9585
2020-02-05T18:17:30.2188097Z .................................................................................................... 5600/9585
2020-02-05T18:17:36.9169109Z .................................................................i.................................. 5700/9585
2020-02-05T18:17:43.9635753Z .................................................................................................... 5800/9585
2020-02-05T18:17:43.9635753Z .................................................................................................... 5800/9585
2020-02-05T18:17:51.6636377Z .................................................................................................... 5900/9585
2020-02-05T18:18:00.1100060Z ........................................................ii...i..ii...........i...................... 6000/9585
2020-02-05T18:18:20.7196602Z .................................................................................................... 6200/9585
2020-02-05T18:18:24.5005413Z .................................................................................................... 6300/9585
2020-02-05T18:18:24.5005413Z .................................................................................................... 6300/9585
2020-02-05T18:18:28.4659483Z ....................................................................................i..ii........... 6400/9585
2020-02-05T18:18:50.2097025Z .................................................................................................... 6600/9585
2020-02-05T18:18:58.9998532Z ......................................................................i............................. 6700/9585
2020-02-05T18:19:01.0581011Z .................................................................................................... 6800/9585
2020-02-05T18:19:03.1570666Z ........................................................................i........................... 6900/9585
---
2020-02-05T18:20:35.3207302Z .................................................................................................... 7600/9585
2020-02-05T18:20:39.7855450Z .................................................................................................... 7700/9585
2020-02-05T18:20:46.0741364Z .................................................................................................... 7800/9585
2020-02-05T18:20:54.0274463Z .................................................................................................... 7900/9585
2020-02-05T18:21:00.9194158Z ..................................iiiiiii.i......................................................... 8000/9585
2020-02-05T18:21:14.5248578Z .................................................................................................... 8200/9585
2020-02-05T18:21:22.3035068Z .................................................................................................... 8300/9585
2020-02-05T18:21:35.4715893Z .................................................................................................... 8400/9585
2020-02-05T18:21:42.4365499Z .................................................................................................... 8500/9585
---
2020-02-05T18:23:53.5189735Z  finished in 6.814
2020-02-05T18:23:53.5375015Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-05T18:23:53.7322530Z 
2020-02-05T18:23:53.7323797Z running 172 tests
2020-02-05T18:23:56.4794694Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/172
2020-02-05T18:23:58.6486132Z ...i.i.i...iii..iiiiiiiiii.......................iii............ii......
2020-02-05T18:23:58.6491573Z 
2020-02-05T18:23:58.6495459Z  finished in 5.112
2020-02-05T18:23:58.6684627Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-05T18:23:58.8318546Z 
---
2020-02-05T18:24:00.6767090Z  finished in 2.008
2020-02-05T18:24:00.6956735Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-05T18:24:00.8510051Z 
2020-02-05T18:24:00.8510463Z running 9 tests
2020-02-05T18:24:00.8512395Z iiiiiiiii
2020-02-05T18:24:00.8512860Z 
2020-02-05T18:24:00.8517801Z  finished in 0.156
2020-02-05T18:24:00.8712660Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-05T18:24:01.0562534Z 
---
2020-02-05T18:24:19.6809428Z  finished in 18.809
2020-02-05T18:24:20.1376604Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-05T18:24:20.3239121Z 
2020-02-05T18:24:20.3239263Z running 116 tests
2020-02-05T18:24:32.8853430Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-05T18:24:34.6205346Z ....iiii.....ii.
2020-02-05T18:24:34.6205898Z 
2020-02-05T18:24:34.6211043Z  finished in 14.483
2020-02-05T18:24:34.6216529Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-05T18:24:34.6216930Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-02-05T18:24:34.6216930Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-02-05T18:24:34.6435723Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-05T18:24:34.8150862Z 
2020-02-05T18:24:34.8150997Z running 63 tests
2020-02-05T18:25:05.2953529Z .......................F.F......F..FFFFFFFFFFF.................
2020-02-05T18:25:05.2960690Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-05T18:25:05.2960783Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-05T18:25:05.2961806Z 
2020-02-05T18:25:05.2962145Z ---- [ui] ui-fulldeps/issue-15778-fail.rs stdout ----
2020-02-05T18:25:05.2962145Z ---- [ui] ui-fulldeps/issue-15778-fail.rs stdout ----
2020-02-05T18:25:05.2962182Z 
2020-02-05T18:25:05.2962499Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" failed to compile: 
2020-02-05T18:25:05.2962560Z status: exit code: 1
2020-02-05T18:25:05.2963772Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary"
2020-02-05T18:25:05.2964136Z ------------------------------------------
2020-02-05T18:25:05.2964198Z 
2020-02-05T18:25:05.2964425Z ------------------------------------------
2020-02-05T18:25:05.2964475Z stderr:
---
2020-02-05T18:25:05.2965737Z    |
2020-02-05T18:25:05.2965793Z LL | use rustc_lint::{LateContext, LintContext, LintPass, LateLintPass, LintArray};
2020-02-05T18:25:05.2965876Z    |                               ^^^^^^^^^^^  ^^^^^^^^                ^^^^^^^^^
2020-02-05T18:25:05.2965911Z 
2020-02-05T18:25:05.2966331Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-05T18:25:05.2966624Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:35:1
2020-02-05T18:25:05.2966724Z LL | #[plugin_registrar]
2020-02-05T18:25:05.2966797Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-05T18:25:05.2966856Z    |
2020-02-05T18:25:05.2966905Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.2966905Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.2966938Z 
2020-02-05T18:25:05.2967258Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::LateContext<'_, '_>` in the current scope
2020-02-05T18:25:05.2967523Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:29:16
2020-02-05T18:25:05.2967575Z    |
2020-02-05T18:25:05.2967765Z LL |             cx.span_lint(CRATE_NOT_OKAY, krate.span,
2020-02-05T18:25:05.2968092Z 
2020-02-05T18:25:05.2968156Z error: aborting due to previous error
2020-02-05T18:25:05.2968188Z 
2020-02-05T18:25:05.2968438Z For more information about this error, try `rustc --explain E0599`.
2020-02-05T18:25:05.2968438Z For more information about this error, try `rustc --explain E0599`.
2020-02-05T18:25:05.2968474Z 
2020-02-05T18:25:05.2968711Z ------------------------------------------
2020-02-05T18:25:05.2968821Z 
2020-02-05T18:25:05.2968848Z 
2020-02-05T18:25:05.2969094Z ---- [ui] ui-fulldeps/issue-15778-pass.rs stdout ----
2020-02-05T18:25:05.2969129Z 
2020-02-05T18:25:05.2969441Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" failed to compile: 
2020-02-05T18:25:05.2969498Z status: exit code: 1
2020-02-05T18:25:05.2970358Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary"
2020-02-05T18:25:05.2970707Z ------------------------------------------
2020-02-05T18:25:05.2970758Z 
2020-02-05T18:25:05.2970984Z ------------------------------------------
2020-02-05T18:25:05.2971034Z stderr:
---
2020-02-05T18:25:05.2972285Z    |
2020-02-05T18:25:05.2972337Z LL | use rustc_lint::{LateContext, LintContext, LintPass, LateLintPass};
2020-02-05T18:25:05.2972455Z    |                               ^^^^^^^^^^^
2020-02-05T18:25:05.2972487Z 
2020-02-05T18:25:05.2972844Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-05T18:25:05.2973140Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:62:1
2020-02-05T18:25:05.2981090Z LL | #[plugin_registrar]
2020-02-05T18:25:05.2981182Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-05T18:25:05.2981237Z    |
2020-02-05T18:25:05.2981289Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.2981289Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.2981338Z 
2020-02-05T18:25:05.2981868Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::LateContext<'_, '_>` in the current scope
2020-02-05T18:25:05.2982156Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:31:28
2020-02-05T18:25:05.2982228Z    |
2020-02-05T18:25:05.2982292Z LL |                           cx.span_lint(CRATE_NOT_OKAY, krate.span,
2020-02-05T18:25:05.2982661Z ...
2020-02-05T18:25:05.2982661Z ...
2020-02-05T18:25:05.2982708Z LL | / fake_lint_pass! {
2020-02-05T18:25:05.2982755Z LL | |     PassOkay,
2020-02-05T18:25:05.2982820Z LL | |     Symbol::intern("rustc_crate_okay")
2020-02-05T18:25:05.2983208Z    | |_- in this macro invocation
2020-02-05T18:25:05.2983249Z 
2020-02-05T18:25:05.2983584Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::LateContext<'_, '_>` in the current scope
2020-02-05T18:25:05.2983860Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:31:28
2020-02-05T18:25:05.2983860Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:31:28
2020-02-05T18:25:05.2983913Z    |
2020-02-05T18:25:05.2983983Z LL |                           cx.span_lint(CRATE_NOT_OKAY, krate.span,
2020-02-05T18:25:05.2984398Z ...
2020-02-05T18:25:05.2984398Z ...
2020-02-05T18:25:05.2984461Z LL | / fake_lint_pass! {
2020-02-05T18:25:05.2984509Z LL | |     PassRedBlue,
2020-02-05T18:25:05.2984563Z LL | |     Symbol::intern("rustc_crate_red"), Symbol::intern("rustc_crate_blue")
2020-02-05T18:25:05.2984858Z    | |_- in this macro invocation
2020-02-05T18:25:05.2984892Z 
2020-02-05T18:25:05.2985206Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::LateContext<'_, '_>` in the current scope
2020-02-05T18:25:05.2985497Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:31:28
2020-02-05T18:25:05.2985497Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:31:28
2020-02-05T18:25:05.2985549Z    |
2020-02-05T18:25:05.2985603Z LL |                           cx.span_lint(CRATE_NOT_OKAY, krate.span,
2020-02-05T18:25:05.2985964Z ...
2020-02-05T18:25:05.2985964Z ...
2020-02-05T18:25:05.2986019Z LL | / fake_lint_pass! {
2020-02-05T18:25:05.2986084Z LL | |     PassRedBlue,
2020-02-05T18:25:05.2986138Z LL | |     Symbol::intern("rustc_crate_red"), Symbol::intern("rustc_crate_blue")
2020-02-05T18:25:05.2986406Z    | |_- in this macro invocation
2020-02-05T18:25:05.2986456Z 
2020-02-05T18:25:05.2986762Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::LateContext<'_, '_>` in the current scope
2020-02-05T18:25:05.2987044Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:31:28
2020-02-05T18:25:05.2987044Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:31:28
2020-02-05T18:25:05.2987113Z    |
2020-02-05T18:25:05.2987165Z LL |                           cx.span_lint(CRATE_NOT_OKAY, krate.span,
2020-02-05T18:25:05.2987523Z ...
2020-02-05T18:25:05.2987523Z ...
2020-02-05T18:25:05.2987569Z LL | / fake_lint_pass! {
2020-02-05T18:25:05.2987617Z LL | |     PassGreyGreen,
2020-02-05T18:25:05.2987688Z LL | |     Symbol::intern("rustc_crate_grey"), Symbol::intern("rustc_crate_green")
2020-02-05T18:25:05.2987965Z    | |_- in this macro invocation
2020-02-05T18:25:05.2987998Z 
2020-02-05T18:25:05.2988319Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::LateContext<'_, '_>` in the current scope
2020-02-05T18:25:05.2988593Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:31:28
2020-02-05T18:25:05.2988593Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:31:28
2020-02-05T18:25:05.2988645Z    |
2020-02-05T18:25:05.2988721Z LL |                           cx.span_lint(CRATE_NOT_OKAY, krate.span,
2020-02-05T18:25:05.2989066Z ...
2020-02-05T18:25:05.2989066Z ...
2020-02-05T18:25:05.2989127Z LL | / fake_lint_pass! {
2020-02-05T18:25:05.2989175Z LL | |     PassGreyGreen,
2020-02-05T18:25:05.2989231Z LL | |     Symbol::intern("rustc_crate_grey"), Symbol::intern("rustc_crate_green")
2020-02-05T18:25:05.2989522Z    | |_- in this macro invocation
2020-02-05T18:25:05.2989554Z 
2020-02-05T18:25:05.2989603Z error: aborting due to 5 previous errors
2020-02-05T18:25:05.2989634Z 
2020-02-05T18:25:05.2989634Z 
2020-02-05T18:25:05.2989908Z For more information about this error, try `rustc --explain E0599`.
2020-02-05T18:25:05.2989945Z 
2020-02-05T18:25:05.2990173Z ------------------------------------------
2020-02-05T18:25:05.2990206Z 
2020-02-05T18:25:05.2990253Z 
2020-02-05T18:25:05.2990488Z ---- [ui] ui-fulldeps/issue-40001.rs stdout ----
2020-02-05T18:25:05.2990575Z 
2020-02-05T18:25:05.2990887Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
2020-02-05T18:25:05.2990961Z status: exit code: 1
2020-02-05T18:25:05.2991813Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
2020-02-05T18:25:05.2992240Z ------------------------------------------
2020-02-05T18:25:05.2992276Z 
2020-02-05T18:25:05.2992528Z ------------------------------------------
2020-02-05T18:25:05.2992577Z stderr:
---
2020-02-05T18:25:05.2993838Z    |
2020-02-05T18:25:05.2993892Z LL | use rustc_lint::{LateContext, LintPass, LintArray, LateLintPass, LintContext};
2020-02-05T18:25:05.2993960Z    |                               ^^^^^^^^  ^^^^^^^^^                ^^^^^^^^^^^
2020-02-05T18:25:05.2994008Z 
2020-02-05T18:25:05.2994395Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-05T18:25:05.2994673Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:20:1
2020-02-05T18:25:05.2994787Z LL | #[plugin_registrar]
2020-02-05T18:25:05.2994842Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-05T18:25:05.2994913Z    |
2020-02-05T18:25:05.2994961Z    = note: `#[warn(deprecated)]` on by default
---
2020-02-05T18:25:05.3002146Z 
2020-02-05T18:25:05.3002199Z 
2020-02-05T18:25:05.3002451Z ---- [ui] ui-fulldeps/lint-group-plugin-deny-cmdline.rs stdout ----
2020-02-05T18:25:05.3002486Z 
2020-02-05T18:25:05.3002785Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
2020-02-05T18:25:05.3002858Z status: exit code: 1
2020-02-05T18:25:05.3003835Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary"
2020-02-05T18:25:05.3004252Z ------------------------------------------
2020-02-05T18:25:05.3004303Z 
2020-02-05T18:25:05.3004537Z ------------------------------------------
2020-02-05T18:25:05.3004586Z stderr:
---
2020-02-05T18:25:05.3005865Z    |
2020-02-05T18:25:05.3005920Z LL | use rustc_lint::{LateContext, LintContext, LintPass, LateLintPass, LintArray, LintId};
2020-02-05T18:25:05.3005980Z    |                               ^^^^^^^^^^^  ^^^^^^^^                ^^^^^^^^^
2020-02-05T18:25:05.3006028Z 
2020-02-05T18:25:05.3006406Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-05T18:25:05.3006703Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:31:1
2020-02-05T18:25:05.3006814Z LL | #[plugin_registrar]
2020-02-05T18:25:05.3006868Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-05T18:25:05.3006932Z    |
2020-02-05T18:25:05.3006980Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3006980Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3007012Z 
2020-02-05T18:25:05.3007334Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::LateContext<'_, '_>` in the current scope
2020-02-05T18:25:05.3007620Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:24:28
2020-02-05T18:25:05.3007673Z    |
2020-02-05T18:25:05.3007961Z LL |             "lintme" => cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'"),
2020-02-05T18:25:05.3008294Z 
2020-02-05T18:25:05.3008599Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::LateContext<'_, '_>` in the current scope
2020-02-05T18:25:05.3008898Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:25:34
2020-02-05T18:25:05.3008950Z    |
2020-02-05T18:25:05.3008950Z    |
2020-02-05T18:25:05.3009238Z LL |             "pleaselintme" => cx.span_lint(PLEASE_LINT, it.span, "item is named 'pleaselintme'"),
2020-02-05T18:25:05.3009588Z 
2020-02-05T18:25:05.3009636Z error: aborting due to 2 previous errors
2020-02-05T18:25:05.3009674Z 
2020-02-05T18:25:05.3009947Z For more information about this error, try `rustc --explain E0599`.
2020-02-05T18:25:05.3009947Z For more information about this error, try `rustc --explain E0599`.
2020-02-05T18:25:05.3009983Z 
2020-02-05T18:25:05.3010211Z ------------------------------------------
2020-02-05T18:25:05.3010243Z 
2020-02-05T18:25:05.3010286Z 
2020-02-05T18:25:05.3010526Z ---- [ui] ui-fulldeps/lint-group-plugin.rs stdout ----
2020-02-05T18:25:05.3010560Z 
2020-02-05T18:25:05.3010915Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
2020-02-05T18:25:05.3010992Z status: exit code: 1
2020-02-05T18:25:05.3011851Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary"
2020-02-05T18:25:05.3012261Z ------------------------------------------
2020-02-05T18:25:05.3012295Z 
2020-02-05T18:25:05.3012579Z ------------------------------------------
2020-02-05T18:25:05.3012628Z stderr:
---
2020-02-05T18:25:05.3013902Z    |
2020-02-05T18:25:05.3013957Z LL | use rustc_lint::{LateContext, LintContext, LintPass, LateLintPass, LintArray, LintId};
2020-02-05T18:25:05.3014017Z    |                               ^^^^^^^^^^^  ^^^^^^^^                ^^^^^^^^^
2020-02-05T18:25:05.3014065Z 
2020-02-05T18:25:05.3014424Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-05T18:25:05.3014721Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:31:1
2020-02-05T18:25:05.3014820Z LL | #[plugin_registrar]
2020-02-05T18:25:05.3014873Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-05T18:25:05.3014937Z    |
2020-02-05T18:25:05.3014986Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3014986Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3015026Z 
2020-02-05T18:25:05.3015347Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::LateContext<'_, '_>` in the current scope
2020-02-05T18:25:05.3015625Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:24:28
2020-02-05T18:25:05.3015677Z    |
2020-02-05T18:25:05.3015965Z LL |             "lintme" => cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'"),
2020-02-05T18:25:05.3016305Z 
2020-02-05T18:25:05.3016610Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::LateContext<'_, '_>` in the current scope
2020-02-05T18:25:05.3016901Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:25:34
2020-02-05T18:25:05.3016953Z    |
2020-02-05T18:25:05.3016953Z    |
2020-02-05T18:25:05.3017240Z LL |             "pleaselintme" => cx.span_lint(PLEASE_LINT, it.span, "item is named 'pleaselintme'"),
2020-02-05T18:25:05.3017599Z 
2020-02-05T18:25:05.3017648Z error: aborting due to 2 previous errors
2020-02-05T18:25:05.3017679Z 
2020-02-05T18:25:05.3017950Z For more information about this error, try `rustc --explain E0599`.
2020-02-05T18:25:05.3017950Z For more information about this error, try `rustc --explain E0599`.
2020-02-05T18:25:05.3017986Z 
2020-02-05T18:25:05.3018212Z ------------------------------------------
2020-02-05T18:25:05.3018244Z 
2020-02-05T18:25:05.3018346Z 
2020-02-05T18:25:05.3018604Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-allow.rs stdout ----
2020-02-05T18:25:05.3018639Z 
2020-02-05T18:25:05.3018933Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-02-05T18:25:05.3019006Z status: exit code: 1
2020-02-05T18:25:05.3019866Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary"
2020-02-05T18:25:05.3020281Z ------------------------------------------
2020-02-05T18:25:05.3020332Z 
2020-02-05T18:25:05.3020562Z ------------------------------------------
2020-02-05T18:25:05.3020611Z stderr:
---
2020-02-05T18:25:05.3021863Z    |
2020-02-05T18:25:05.3021924Z LL | use rustc_lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-02-05T18:25:05.3021987Z    |                                ^^^^^^^^^^^                           ^^^^^^^^^
2020-02-05T18:25:05.3022036Z 
2020-02-05T18:25:05.3022385Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-05T18:25:05.3022661Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-02-05T18:25:05.3022786Z LL | #[plugin_registrar]
2020-02-05T18:25:05.3022839Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-05T18:25:05.3022904Z    |
2020-02-05T18:25:05.3022954Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3022954Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3022986Z 
2020-02-05T18:25:05.3023305Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-05T18:25:05.3023584Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:16
2020-02-05T18:25:05.3023636Z    |
2020-02-05T18:25:05.3023910Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2020-02-05T18:25:05.3024223Z 
2020-02-05T18:25:05.3024269Z warning: unused import: `LintPass`
2020-02-05T18:25:05.3024548Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:45
2020-02-05T18:25:05.3024608Z    |
---
2020-02-05T18:25:05.3025867Z 
2020-02-05T18:25:05.3025894Z 
2020-02-05T18:25:05.3026166Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-load.rs stdout ----
2020-02-05T18:25:05.3026202Z 
2020-02-05T18:25:05.3026495Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-02-05T18:25:05.3026570Z status: exit code: 1
2020-02-05T18:25:05.3027437Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary"
2020-02-05T18:25:05.3027848Z ------------------------------------------
2020-02-05T18:25:05.3027884Z 
2020-02-05T18:25:05.3028129Z ------------------------------------------
2020-02-05T18:25:05.3028177Z stderr:
---
2020-02-05T18:25:05.3030363Z    |
2020-02-05T18:25:05.3030417Z LL | use rustc_lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-02-05T18:25:05.3030476Z    |                                ^^^^^^^^^^^                           ^^^^^^^^^
2020-02-05T18:25:05.3030511Z 
2020-02-05T18:25:05.3030896Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-05T18:25:05.3031174Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-02-05T18:25:05.3031299Z LL | #[plugin_registrar]
2020-02-05T18:25:05.3031354Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-05T18:25:05.3031419Z    |
2020-02-05T18:25:05.3031467Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3031467Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3031499Z 
2020-02-05T18:25:05.3031804Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-05T18:25:05.3032099Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:16
2020-02-05T18:25:05.3032152Z    |
2020-02-05T18:25:05.3032414Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2020-02-05T18:25:05.3032745Z 
2020-02-05T18:25:05.3032792Z warning: unused import: `LintPass`
2020-02-05T18:25:05.3037088Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:45
2020-02-05T18:25:05.3037207Z    |
---
2020-02-05T18:25:05.3038173Z 
2020-02-05T18:25:05.3038201Z 
2020-02-05T18:25:05.3038471Z ---- [ui] ui-fulldeps/lint-plugin-deny-attr.rs stdout ----
2020-02-05T18:25:05.3038507Z 
2020-02-05T18:25:05.3038803Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-02-05T18:25:05.3038861Z status: exit code: 1
2020-02-05T18:25:05.3039947Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary"
2020-02-05T18:25:05.3040403Z ------------------------------------------
2020-02-05T18:25:05.3040438Z 
2020-02-05T18:25:05.3040670Z ------------------------------------------
2020-02-05T18:25:05.3040735Z stderr:
---
2020-02-05T18:25:05.3041987Z    |
2020-02-05T18:25:05.3042057Z LL | use rustc_lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-02-05T18:25:05.3042117Z    |                                ^^^^^^^^^^^                           ^^^^^^^^^
2020-02-05T18:25:05.3042152Z 
2020-02-05T18:25:05.3042553Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-05T18:25:05.3042833Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-02-05T18:25:05.3042957Z LL | #[plugin_registrar]
2020-02-05T18:25:05.3043012Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-05T18:25:05.3043063Z    |
2020-02-05T18:25:05.3043127Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3043127Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3043159Z 
2020-02-05T18:25:05.3043466Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-05T18:25:05.3043980Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:16
2020-02-05T18:25:05.3044046Z    |
2020-02-05T18:25:05.3044325Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2020-02-05T18:25:05.3045612Z 
2020-02-05T18:25:05.3045663Z warning: unused import: `LintPass`
2020-02-05T18:25:05.3045950Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:45
2020-02-05T18:25:05.3046031Z    |
---
2020-02-05T18:25:05.3046937Z 
2020-02-05T18:25:05.3046963Z 
2020-02-05T18:25:05.3047217Z ---- [ui] ui-fulldeps/lint-plugin-deny-cmdline.rs stdout ----
2020-02-05T18:25:05.3047269Z 
2020-02-05T18:25:05.3047561Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-02-05T18:25:05.3047620Z status: exit code: 1
2020-02-05T18:25:05.3048706Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary"
2020-02-05T18:25:05.3049171Z ------------------------------------------
2020-02-05T18:25:05.3049206Z 
2020-02-05T18:25:05.3049436Z ------------------------------------------
2020-02-05T18:25:05.3049501Z stderr:
---
2020-02-05T18:25:05.3050751Z    |
2020-02-05T18:25:05.3050804Z LL | use rustc_lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-02-05T18:25:05.3050878Z    |                                ^^^^^^^^^^^                           ^^^^^^^^^
2020-02-05T18:25:05.3050913Z 
2020-02-05T18:25:05.3051305Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-05T18:25:05.3051592Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-02-05T18:25:05.3051707Z LL | #[plugin_registrar]
2020-02-05T18:25:05.3051761Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-05T18:25:05.3051809Z    |
2020-02-05T18:25:05.3051857Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3051857Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3051904Z 
2020-02-05T18:25:05.3052215Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-05T18:25:05.3052486Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:16
2020-02-05T18:25:05.3052551Z    |
2020-02-05T18:25:05.3053059Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2020-02-05T18:25:05.3053389Z 
2020-02-05T18:25:05.3053436Z warning: unused import: `LintPass`
2020-02-05T18:25:05.3053710Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:45
2020-02-05T18:25:05.3054101Z    |
---
2020-02-05T18:25:05.3055072Z 
2020-02-05T18:25:05.3055100Z 
2020-02-05T18:25:05.3055346Z ---- [ui] ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----
2020-02-05T18:25:05.3055381Z 
2020-02-05T18:25:05.3055692Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-02-05T18:25:05.3055750Z status: exit code: 1
2020-02-05T18:25:05.3056692Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary"
2020-02-05T18:25:05.3057054Z ------------------------------------------
2020-02-05T18:25:05.3057220Z 
2020-02-05T18:25:05.3057494Z ------------------------------------------
2020-02-05T18:25:05.3057544Z stderr:
---
2020-02-05T18:25:05.3058973Z    |
2020-02-05T18:25:05.3059026Z LL | use rustc_lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-02-05T18:25:05.3059101Z    |                                ^^^^^^^^^^^                           ^^^^^^^^^
2020-02-05T18:25:05.3059135Z 
2020-02-05T18:25:05.3059620Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-05T18:25:05.3059960Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-02-05T18:25:05.3060060Z LL | #[plugin_registrar]
2020-02-05T18:25:05.3060132Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-05T18:25:05.3060182Z    |
2020-02-05T18:25:05.3060231Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3060231Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3060263Z 
2020-02-05T18:25:05.3060592Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-05T18:25:05.3060864Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:16
2020-02-05T18:25:05.3060915Z    |
2020-02-05T18:25:05.3061191Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2020-02-05T18:25:05.3061501Z 
2020-02-05T18:25:05.3061563Z warning: unused import: `LintPass`
2020-02-05T18:25:05.3061835Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:45
2020-02-05T18:25:05.3061885Z    |
---
2020-02-05T18:25:05.3062779Z 
2020-02-05T18:25:05.3062806Z 
2020-02-05T18:25:05.3063054Z ---- [ui] ui-fulldeps/lint-plugin-forbid-cmdline.rs stdout ----
2020-02-05T18:25:05.3063089Z 
2020-02-05T18:25:05.3063399Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-02-05T18:25:05.3063512Z status: exit code: 1
2020-02-05T18:25:05.3064394Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary"
2020-02-05T18:25:05.3064741Z ------------------------------------------
2020-02-05T18:25:05.3064793Z 
2020-02-05T18:25:05.3065023Z ------------------------------------------
2020-02-05T18:25:05.3065071Z stderr:
---
2020-02-05T18:25:05.3066333Z    |
2020-02-05T18:25:05.3066386Z LL | use rustc_lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-02-05T18:25:05.3066445Z    |                                ^^^^^^^^^^^                           ^^^^^^^^^
2020-02-05T18:25:05.3066493Z 
2020-02-05T18:25:05.3066841Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-05T18:25:05.3067141Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-02-05T18:25:05.3067239Z LL | #[plugin_registrar]
2020-02-05T18:25:05.3067307Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-05T18:25:05.3067357Z    |
2020-02-05T18:25:05.3067406Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3067406Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3067625Z 
2020-02-05T18:25:05.3068502Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-05T18:25:05.3068956Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:16
2020-02-05T18:25:05.3069009Z    |
2020-02-05T18:25:05.3069299Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2020-02-05T18:25:05.3069615Z 
2020-02-05T18:25:05.3069672Z warning: unused import: `LintPass`
2020-02-05T18:25:05.3069952Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:45
2020-02-05T18:25:05.3070002Z    |
---
2020-02-05T18:25:05.3070919Z 
2020-02-05T18:25:05.3070963Z 
2020-02-05T18:25:05.3071195Z ---- [ui] ui-fulldeps/lint-plugin.rs stdout ----
2020-02-05T18:25:05.3071229Z 
2020-02-05T18:25:05.3071560Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-02-05T18:25:05.3071694Z status: exit code: 1
2020-02-05T18:25:05.3072544Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary"
2020-02-05T18:25:05.3072888Z ------------------------------------------
2020-02-05T18:25:05.3072924Z 
2020-02-05T18:25:05.3073170Z ------------------------------------------
2020-02-05T18:25:05.3073221Z stderr:
---
2020-02-05T18:25:05.3074972Z    |
2020-02-05T18:25:05.3075029Z LL | use rustc_lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-02-05T18:25:05.3075092Z    |                                ^^^^^^^^^^^                           ^^^^^^^^^
2020-02-05T18:25:05.3075129Z 
2020-02-05T18:25:05.3075801Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-05T18:25:05.3076090Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-02-05T18:25:05.3076206Z LL | #[plugin_registrar]
2020-02-05T18:25:05.3076260Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-05T18:25:05.3076322Z    |
2020-02-05T18:25:05.3076371Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3076371Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3076403Z 
2020-02-05T18:25:05.3076718Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-05T18:25:05.3077005Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:16
2020-02-05T18:25:05.3077056Z    |
2020-02-05T18:25:05.3077318Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2020-02-05T18:25:05.3077643Z 
2020-02-05T18:25:05.3077700Z warning: unused import: `LintPass`
2020-02-05T18:25:05.3077978Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:45
2020-02-05T18:25:05.3078028Z    |
---
2020-02-05T18:25:05.3078940Z 
2020-02-05T18:25:05.3078967Z 
2020-02-05T18:25:05.3079227Z ---- [ui] ui-fulldeps/lint-tool-cmdline-allow.rs stdout ----
2020-02-05T18:25:05.3079263Z 
2020-02-05T18:25:05.3079553Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" failed to compile: 
2020-02-05T18:25:05.3079673Z status: exit code: 1
2020-02-05T18:25:05.3080556Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary"
2020-02-05T18:25:05.3081097Z ------------------------------------------
2020-02-05T18:25:05.3081132Z 
2020-02-05T18:25:05.3081361Z ------------------------------------------
2020-02-05T18:25:05.3081427Z stderr:
---
2020-02-05T18:25:05.3082785Z    |
2020-02-05T18:25:05.3082855Z LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass, LintId};
2020-02-05T18:25:05.3082916Z    |                                               ^^^^^^^^^  ^^^^^^^^^^^
2020-02-05T18:25:05.3082951Z 
2020-02-05T18:25:05.3083326Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-05T18:25:05.3083608Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:41:1
2020-02-05T18:25:05.3083722Z LL | #[plugin_registrar]
2020-02-05T18:25:05.3083777Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-05T18:25:05.3083827Z    |
2020-02-05T18:25:05.3083891Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3083891Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3083923Z 
2020-02-05T18:25:05.3084235Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-05T18:25:05.3085018Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:33:16
2020-02-05T18:25:05.3085081Z    |
2020-02-05T18:25:05.3085386Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2020-02-05T18:25:05.3085729Z 
2020-02-05T18:25:05.3086035Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-05T18:25:05.3086302Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:36:16
2020-02-05T18:25:05.3086368Z    |
2020-02-05T18:25:05.3086368Z    |
2020-02-05T18:25:05.3086637Z LL |             cx.span_lint(TEST_GROUP, it.span, "item is named 'lintmetoo'");
2020-02-05T18:25:05.3087048Z 
2020-02-05T18:25:05.3087100Z warning: unused import: `LintPass`
2020-02-05T18:25:05.3087372Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:11:71
2020-02-05T18:25:05.3087423Z    |
---
2020-02-05T18:25:05.3088313Z 
2020-02-05T18:25:05.3088341Z 
2020-02-05T18:25:05.3088577Z ---- [ui] ui-fulldeps/lint-tool-test.rs stdout ----
2020-02-05T18:25:05.3088611Z 
2020-02-05T18:25:05.3088928Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" failed to compile: 
2020-02-05T18:25:05.3088986Z status: exit code: 1
2020-02-05T18:25:05.3090071Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary"
2020-02-05T18:25:05.3090429Z ------------------------------------------
2020-02-05T18:25:05.3090464Z 
2020-02-05T18:25:05.3090696Z ------------------------------------------
2020-02-05T18:25:05.3090745Z stderr:
---
2020-02-05T18:25:05.3092003Z    |
2020-02-05T18:25:05.3092057Z LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass, LintId};
2020-02-05T18:25:05.3092265Z    |                                               ^^^^^^^^^  ^^^^^^^^^^^
2020-02-05T18:25:05.3092311Z 
2020-02-05T18:25:05.3092701Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-05T18:25:05.3092992Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:41:1
2020-02-05T18:25:05.3093090Z LL | #[plugin_registrar]
2020-02-05T18:25:05.3093158Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-05T18:25:05.3093207Z    |
2020-02-05T18:25:05.3093254Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3093254Z    = note: `#[warn(deprecated)]` on by default
2020-02-05T18:25:05.3093309Z 
2020-02-05T18:25:05.3093615Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-05T18:25:05.3093984Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:33:16
2020-02-05T18:25:05.3094037Z    |
2020-02-05T18:25:05.3094316Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2020-02-05T18:25:05.3094691Z 
2020-02-05T18:25:05.3095023Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-05T18:25:05.3095291Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:36:16
2020-02-05T18:25:05.3095342Z    |
2020-02-05T18:25:05.3095342Z    |
2020-02-05T18:25:05.3095624Z LL |             cx.span_lint(TEST_GROUP, it.span, "item is named 'lintmetoo'");
2020-02-05T18:25:05.3095995Z 
2020-02-05T18:25:05.3096043Z warning: unused import: `LintPass`
2020-02-05T18:25:05.3096384Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:11:71
2020-02-05T18:25:05.3096436Z    |
---
2020-02-05T18:25:05.3101078Z test result: FAILED. 49 passed; 14 failed; 0 ignored; 0 measured; 0 filtered out
2020-02-05T18:25:05.3101116Z 
2020-02-05T18:25:05.3101144Z 
2020-02-05T18:25:05.3101186Z 
2020-02-05T18:25:05.3102917Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-05T18:25:05.3103234Z 
2020-02-05T18:25:05.3103265Z 
2020-02-05T18:25:05.3103317Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-05T18:25:05.3103373Z Build completed unsuccessfully in 0:59:44
2020-02-05T18:25:05.3103373Z Build completed unsuccessfully in 0:59:44
2020-02-05T18:25:05.3103490Z == clock drift check ==
2020-02-05T18:25:05.3103541Z   local time: Wed Feb  5 18:25:05 UTC 2020
2020-02-05T18:25:05.6053290Z   network time: Wed, 05 Feb 2020 18:25:05 GMT
2020-02-05T18:25:05.6058333Z == end clock drift check ==
2020-02-05T18:25:06.3955183Z 
2020-02-05T18:25:06.4013230Z ##[error]Bash exited with code '1'.
2020-02-05T18:25:06.4025090Z ##[section]Finishing: Run build
2020-02-05T18:25:06.4053363Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68725/merge to s
2020-02-05T18:25:06.4055153Z Task         : Get sources
2020-02-05T18:25:06.4055204Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-05T18:25:06.4055256Z Version      : 1.0.0
2020-02-05T18:25:06.4055317Z Author       : Microsoft
2020-02-05T18:25:06.4055317Z Author       : Microsoft
2020-02-05T18:25:06.4055369Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-05T18:25:06.4055424Z ==============================================================================
2020-02-05T18:25:06.8272379Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-05T18:25:06.8316501Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68725/merge to s
2020-02-05T18:25:06.8441242Z Cleaning up task key
2020-02-05T18:25:06.8442034Z Start cleaning up orphan processes.
2020-02-05T18:25:06.8546646Z Terminate orphan process: pid (4706) (python)
2020-02-05T18:25:06.8741151Z ##[section]Finishing: Finalize Job
