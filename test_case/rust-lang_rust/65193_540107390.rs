plain
2019-10-09T16:14:40.0091739Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-09T16:14:40.0303731Z ##[command]git config gc.auto 0
2019-10-09T16:14:40.0378761Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-09T16:14:40.0443357Z ##[command]git config --get-all http.proxy
2019-10-09T16:14:40.0611054Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65193/merge:refs/remotes/pull/65193/merge
---
2019-10-09T17:20:16.6793759Z .................................................................................................... 1600/9142
2019-10-09T17:20:25.0221018Z .................................................................................................... 1700/9142
2019-10-09T17:20:37.2397301Z .................i...............i.................................................................. 1800/9142
2019-10-09T17:20:44.8417303Z .................................................................................................... 1900/9142
2019-10-09T17:21:02.0649558Z ........iiiii....................................................................................... 2000/9142
2019-10-09T17:21:12.9624171Z .................................................................................................... 2200/9142
2019-10-09T17:21:15.9259493Z .................................................................................................... 2300/9142
2019-10-09T17:21:22.2389935Z .................................................................................................... 2400/9142
2019-10-09T17:21:29.3682754Z .................................................................................................... 2500/9142
---
2019-10-09T17:24:36.5562775Z .................................................................................................... 4700/9142
2019-10-09T17:24:44.1745748Z .i...............i.................................................................................. 4800/9142
2019-10-09T17:24:55.7467023Z .................................................................................................... 4900/9142
2019-10-09T17:25:01.6432920Z .................................................................................................... 5000/9142
2019-10-09T17:25:14.0315324Z ...............................................................................................ii.ii 5100/9142
2019-10-09T17:25:25.6093326Z .................................................................................................... 5300/9142
2019-10-09T17:25:36.1809416Z .................................................................................................... 5400/9142
2019-10-09T17:25:43.2538975Z .............................................................i...................................... 5500/9142
2019-10-09T17:25:50.9831219Z .................................................................................................... 5600/9142
2019-10-09T17:25:50.9831219Z .................................................................................................... 5600/9142
2019-10-09T17:25:58.7829213Z .................................................................................................... 5700/9142
2019-10-09T17:26:11.5397427Z ..........................................................ii...i..ii...........i.................... 5800/9142
2019-10-09T17:26:39.5417216Z .................................................................................................... 6000/9142
2019-10-09T17:26:49.1049916Z .................................................................................................... 6100/9142
2019-10-09T17:26:49.1049916Z .................................................................................................... 6100/9142
2019-10-09T17:27:02.0509065Z ................................................................i..ii............................... 6200/9142
2019-10-09T17:27:34.7615721Z .................................................................................................... 6400/9142
2019-10-09T17:27:37.1074829Z ........................i........................................................................... 6500/9142
2019-10-09T17:27:39.4997733Z .................................................................................................i.. 6600/9142
2019-10-09T17:27:42.4406018Z .................................................................................................... 6700/9142
---
2019-10-09T17:32:45.2981315Z  finished in 5.988
2019-10-09T17:32:45.3195814Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-09T17:32:45.8925644Z 
2019-10-09T17:32:45.8930972Z running 150 tests
2019-10-09T17:32:49.1045242Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-10-09T17:32:51.2536951Z ..iiii..............i.........iii.i.......ii......
2019-10-09T17:32:51.2537481Z 
2019-10-09T17:32:51.2543304Z  finished in 5.934
2019-10-09T17:32:51.2739597Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-09T17:32:51.4446861Z 
---
2019-10-09T17:32:54.3900154Z  finished in 2.502
2019-10-09T17:32:54.3900422Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-09T17:32:54.3900478Z 
2019-10-09T17:32:54.3900518Z running 9 tests
2019-10-09T17:32:54.3900935Z iiiiiiiii
2019-10-09T17:32:54.3901640Z 
2019-10-09T17:32:54.3901678Z  finished in 0.176
2019-10-09T17:32:54.3901926Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-09T17:32:54.3901976Z 
---
2019-10-09T17:33:14.2187324Z  finished in 20.222
2019-10-09T17:33:14.2431908Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-09T17:33:14.4447190Z 
2019-10-09T17:33:14.4447343Z running 123 tests
2019-10-09T17:33:42.4254214Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-09T17:33:47.7884637Z i.i.i......iii.i.....ii
2019-10-09T17:33:47.7885181Z 
2019-10-09T17:33:47.7887568Z  finished in 33.545
2019-10-09T17:33:47.7897495Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-09T17:33:47.7899850Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-09T17:33:47.7899850Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-09T17:33:47.8131468Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-09T17:33:47.9850232Z 
2019-10-09T17:33:47.9850753Z running 69 tests
2019-10-09T17:34:50.6628477Z .......................FF........F.FFFFFFFFFF.F......................
2019-10-09T17:34:50.6636281Z thread '
2019-10-09T17:34:50.6636822Z ---- [ui] ui-fulldeps/issue-15778-fail.rs stdout ----
2019-10-09T17:34:50.6637003Z 
2019-10-09T17:34:50.6637003Z 
2019-10-09T17:34:50.6637442Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" failed to compile: 
2019-10-09T17:34:50.6637672Z status: exit code: 1
2019-10-09T17:34:50.6639141Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary"
2019-10-09T17:34:50.6643303Z ------------------------------------------
2019-10-09T17:34:50.6643379Z 
2019-10-09T17:34:50.6643829Z ------------------------------------------
2019-10-09T17:34:50.6643887Z stderr:
2019-10-09T17:34:50.6643887Z stderr:
2019-10-09T17:34:50.6644178Z ------------------------------------------
2019-10-09T17:34:50.6644229Z warning: unused import: `LateLintPassObject`
2019-10-09T17:34:50.6644739Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:10:69
2019-10-09T17:34:50.6644828Z    |
2019-10-09T17:34:50.6644884Z LL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LateLintPassObject, LintArray};
2019-10-09T17:34:50.6645187Z    |
2019-10-09T17:34:50.6645235Z    = note: `#[warn(unused_imports)]` on by default
2019-10-09T17:34:50.6645277Z 
2019-10-09T17:34:50.6645277Z 
2019-10-09T17:34:50.6646086Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-09T17:34:50.6646633Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:33:1
2019-10-09T17:34:50.6646757Z LL | #[plugin_registrar]
2019-10-09T17:34:50.6646816Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-09T17:34:50.6646858Z    |
2019-10-09T17:34:50.6646918Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6646918Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6646948Z 
2019-10-09T17:34:50.6647626Z error[E0599]: no method named `register_lint` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-09T17:34:50.6647920Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:35:9
2019-10-09T17:34:50.6648937Z    |
2019-10-09T17:34:50.6648988Z LL |     reg.register_lint(&[&CRATE_NOT_OKAY]);
2019-10-09T17:34:50.6649365Z    |         ^^^^^^^^^^^^^ method not found in `&mut rustc_driver::rustc_plugin_impl::Registry<'_>`
2019-10-09T17:34:50.6650204Z 
2019-10-09T17:34:50.6650601Z error[E0599]: no method named `register_late_lint_pass` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-09T17:34:50.6651133Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:36:9
2019-10-09T17:34:50.6651225Z    |
2019-10-09T17:34:50.6651507Z LL |     reg.register_late_lint_pass(|| box Pass);
2019-10-09T17:34:50.6651926Z    |         ^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `register_llvm_pass`
2019-10-09T17:34:50.6652029Z error: aborting due to 2 previous errors
2019-10-09T17:34:50.6652056Z 
2019-10-09T17:34:50.6652369Z For more information about this error, try `rustc --explain E0599`.
2019-10-09T17:34:50.6652578Z 
2019-10-09T17:34:50.6652578Z 
2019-10-09T17:34:50.6652860Z ------------------------------------------
2019-10-09T17:34:50.6653003Z 
2019-10-09T17:34:50.6653032Z 
2019-10-09T17:34:50.6653504Z ---- [ui] ui-fulldeps/issue-15778-pass.rs stdout ----
2019-10-09T17:34:50.6653585Z 
2019-10-09T17:34:50.6654064Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" failed to compile: 
2019-10-09T17:34:50.6654128Z status: exit code: 1
2019-10-09T17:34:50.6655370Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary"
2019-10-09T17:34:50.6655946Z ------------------------------------------
2019-10-09T17:34:50.6656463Z 
2019-10-09T17:34:50.6656762Z ------------------------------------------
2019-10-09T17:34:50.6656829Z stderr:
2019-10-09T17:34:50.6656829Z stderr:
2019-10-09T17:34:50.6657287Z ------------------------------------------
2019-10-09T17:34:50.6657539Z error[E0407]: method `get_lints` is not a member of trait `LintPass`
2019-10-09T17:34:50.6658612Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:25:13
2019-10-09T17:34:50.6658720Z    |
2019-10-09T17:34:50.6658996Z LL | /             fn get_lints(&self) -> LintArray {
2019-10-09T17:34:50.6659048Z LL | |                 $lints
2019-10-09T17:34:50.6659164Z    | |_____________^ not a member of trait `LintPass`
2019-10-09T17:34:50.6659209Z ...
2019-10-09T17:34:50.6659209Z ...
2019-10-09T17:34:50.6659272Z LL | / fake_lint_pass! {
2019-10-09T17:34:50.6659316Z LL | |     PassOkay,
2019-10-09T17:34:50.6659364Z LL | |     lint_array!(CRATE_NOT_OKAY), // Single lint
2019-10-09T17:34:50.6659431Z LL | |     Symbol::intern("rustc_crate_okay")
2019-10-09T17:34:50.6659707Z    | |_- in this macro invocation
2019-10-09T17:34:50.6659740Z 
2019-10-09T17:34:50.6659806Z error[E0407]: method `get_lints` is not a member of trait `LintPass`
2019-10-09T17:34:50.6660068Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:25:13
2019-10-09T17:34:50.6660068Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:25:13
2019-10-09T17:34:50.6660118Z    |
2019-10-09T17:34:50.6660368Z LL | /             fn get_lints(&self) -> LintArray {
2019-10-09T17:34:50.6660418Z LL | |                 $lints
2019-10-09T17:34:50.6660527Z    | |_____________^ not a member of trait `LintPass`
2019-10-09T17:34:50.6660571Z ...
2019-10-09T17:34:50.6660571Z ...
2019-10-09T17:34:50.6660618Z LL | / fake_lint_pass! {
2019-10-09T17:34:50.6660661Z LL | |     PassRedBlue,
2019-10-09T17:34:50.6660728Z LL | |     lint_array!(CRATE_NOT_RED, CRATE_NOT_BLUE), // Multiple lints
2019-10-09T17:34:50.6660783Z LL | |     Symbol::intern("rustc_crate_red"), Symbol::intern("rustc_crate_blue")
2019-10-09T17:34:50.6661247Z    | |_- in this macro invocation
2019-10-09T17:34:50.6661281Z 
2019-10-09T17:34:50.6661329Z error[E0407]: method `get_lints` is not a member of trait `LintPass`
2019-10-09T17:34:50.6661583Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:25:13
2019-10-09T17:34:50.6661583Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:25:13
2019-10-09T17:34:50.6661654Z    |
2019-10-09T17:34:50.6662037Z LL | /             fn get_lints(&self) -> LintArray {
2019-10-09T17:34:50.6662161Z LL | |                 $lints
2019-10-09T17:34:50.6662277Z    | |_____________^ not a member of trait `LintPass`
2019-10-09T17:34:50.6662318Z ...
2019-10-09T17:34:50.6662318Z ...
2019-10-09T17:34:50.6662376Z LL | / fake_lint_pass! {
2019-10-09T17:34:50.6662418Z LL | |     PassGreyGreen,
2019-10-09T17:34:50.6662465Z LL | |     lint_array!(CRATE_NOT_GREY, CRATE_NOT_GREEN, ), // Trailing comma
2019-10-09T17:34:50.6662536Z LL | |     Symbol::intern("rustc_crate_grey"), Symbol::intern("rustc_crate_green")
2019-10-09T17:34:50.6662813Z    | |_- in this macro invocation
2019-10-09T17:34:50.6662843Z 
2019-10-09T17:34:50.6662843Z 
2019-10-09T17:34:50.6662903Z warning: unused import: `LateLintPassObject`
2019-10-09T17:34:50.6663142Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:10:69
2019-10-09T17:34:50.6663187Z    |
2019-10-09T17:34:50.6663252Z LL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LateLintPassObject, LintArray};
2019-10-09T17:34:50.6663364Z    |
2019-10-09T17:34:50.6663426Z    = note: `#[warn(unused_imports)]` on by default
2019-10-09T17:34:50.6663455Z 
2019-10-09T17:34:50.6663455Z 
2019-10-09T17:34:50.6664051Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-09T17:34:50.6664395Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:68:1
2019-10-09T17:34:50.6664485Z LL | #[plugin_registrar]
2019-10-09T17:34:50.6664775Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-09T17:34:50.6664817Z    |
2019-10-09T17:34:50.6664860Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6664860Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6664888Z 
2019-10-09T17:34:50.6665274Z error[E0599]: no method named `register_late_lint_pass` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-09T17:34:50.6665530Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:70:9
2019-10-09T17:34:50.6665601Z    |
2019-10-09T17:34:50.6665644Z LL |     reg.register_late_lint_pass(box PassOkay);
2019-10-09T17:34:50.6665697Z    |         ^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `register_llvm_pass`
2019-10-09T17:34:50.6665730Z 
2019-10-09T17:34:50.6666286Z error[E0599]: no method named `register_late_lint_pass` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-09T17:34:50.6666551Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:71:9
2019-10-09T17:34:50.6666608Z    |
2019-10-09T17:34:50.6666849Z LL |     reg.register_late_lint_pass(box PassRedBlue);
2019-10-09T17:34:50.6666907Z    |         ^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `register_llvm_pass`
2019-10-09T17:34:50.6666939Z 
2019-10-09T17:34:50.6667331Z error[E0599]: no method named `register_late_lint_pass` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-09T17:34:50.6668028Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:72:9
2019-10-09T17:34:50.6668089Z    |
2019-10-09T17:34:50.6668154Z LL |     reg.register_late_lint_pass(box PassGreyGreen);
2019-10-09T17:34:50.6668207Z    |         ^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `register_llvm_pass`
2019-10-09T17:34:50.6668849Z error: aborting due to 6 previous errors
2019-10-09T17:34:50.6668909Z 
2019-10-09T17:34:50.6668955Z Some errors have detailed explanations: E0407, E0599.
2019-10-09T17:34:50.6669470Z For more information about an error, try `rustc --explain E0407`.
2019-10-09T17:34:50.6669470Z For more information about an error, try `rustc --explain E0407`.
2019-10-09T17:34:50.6669506Z 
2019-10-09T17:34:50.6669980Z ------------------------------------------
2019-10-09T17:34:50.6670026Z 
2019-10-09T17:34:50.6670052Z 
2019-10-09T17:34:50.6670328Z ---- [ui] ui-fulldeps/issue-40001.rs stdout ----
2019-10-09T17:34:50.6670386Z 
2019-10-09T17:34:50.6671033Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
2019-10-09T17:34:50.6671118Z status: exit code: 1
2019-10-09T17:34:50.6671936Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
2019-10-09T17:34:50.6672584Z ------------------------------------------
2019-10-09T17:34:50.6672616Z 
2019-10-09T17:34:50.6672817Z ------------------------------------------
2019-10-09T17:34:50.6672859Z stderr:
2019-10-09T17:34:50.6672859Z stderr:
2019-10-09T17:34:50.6673077Z ------------------------------------------
2019-10-09T17:34:50.6673124Z warning: unused import: `syntax::ext::base::*`
2019-10-09T17:34:50.6673354Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:11:5
2019-10-09T17:34:50.6673457Z LL | use syntax::ext::base::*;
2019-10-09T17:34:50.6673498Z    |     ^^^^^^^^^^^^^^^^^^^^
2019-10-09T17:34:50.6673554Z    |
2019-10-09T17:34:50.6673595Z    = note: `#[warn(unused_imports)]` on by default
2019-10-09T17:34:50.6673595Z    = note: `#[warn(unused_imports)]` on by default
2019-10-09T17:34:50.6673623Z 
2019-10-09T17:34:50.6673663Z warning: unused import: `rustc::hir::map as hir_map`
2019-10-09T17:34:50.6673923Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:17:5
2019-10-09T17:34:50.6673968Z    |
2019-10-09T17:34:50.6674007Z LL | use rustc::hir::map as hir_map;
2019-10-09T17:34:50.6674096Z 
2019-10-09T17:34:50.6674134Z warning: unused import: `rustc::ty`
2019-10-09T17:34:50.6674134Z warning: unused import: `rustc::ty`
2019-10-09T17:34:50.6674370Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:20:5
2019-10-09T17:34:50.6674471Z LL | use rustc::ty;
2019-10-09T17:34:50.6674510Z    |     ^^^^^^^^^
2019-10-09T17:34:50.6674553Z 
2019-10-09T17:34:50.6674591Z warning: unused import: `ast`
2019-10-09T17:34:50.6674591Z warning: unused import: `ast`
2019-10-09T17:34:50.6674824Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:21:14
2019-10-09T17:34:50.6674870Z    |
2019-10-09T17:34:50.6674925Z LL | use syntax::{ast, source_map};
2019-10-09T17:34:50.6674995Z 
2019-10-09T17:34:50.6674995Z 
2019-10-09T17:34:50.6675357Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-09T17:34:50.6675607Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:23:1
2019-10-09T17:34:50.6675714Z LL | #[plugin_registrar]
2019-10-09T17:34:50.6675758Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-09T17:34:50.6675797Z    |
2019-10-09T17:34:50.6675864Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6675864Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6675892Z 
2019-10-09T17:34:50.6676182Z error[E0599]: no method named `register_late_lint_pass` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-09T17:34:50.6676439Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:25:9
2019-10-09T17:34:50.6676485Z    |
2019-10-09T17:34:50.6676527Z LL |     reg.register_late_lint_pass(box MissingWhitelistedAttrPass);
2019-10-09T17:34:50.6676597Z    |         ^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `register_llvm_pass`
2019-10-09T17:34:50.6676763Z error: aborting due to previous error
2019-10-09T17:34:50.6676793Z 
2019-10-09T17:34:50.6677060Z For more information about this error, try `rustc --explain E0599`.
2019-10-09T17:34:50.6677092Z 
2019-10-09T17:34:50.6677092Z 
2019-10-09T17:34:50.6677290Z ------------------------------------------
2019-10-09T17:34:50.6677318Z 
2019-10-09T17:34:50.6677407Z 
2019-10-09T17:34:50.6677675Z ---- [ui] ui-fulldeps/lint-group-plugin-deny-cmdline.rs stdout ----
2019-10-09T17:34:50.6677706Z 
2019-10-09T17:34:50.6677961Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
2019-10-09T17:34:50.6678027Z status: exit code: 1
2019-10-09T17:34:50.6679395Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary"
2019-10-09T17:34:50.6679765Z ------------------------------------------
2019-10-09T17:34:50.6679799Z 
2019-10-09T17:34:50.6680038Z ------------------------------------------
2019-10-09T17:34:50.6680086Z stderr:
2019-10-09T17:34:50.6680086Z stderr:
2019-10-09T17:34:50.6680302Z ------------------------------------------
2019-10-09T17:34:50.6680371Z warning: unused import: `LateLintPassObject`
2019-10-09T17:34:50.6680631Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:12:69
2019-10-09T17:34:50.6680684Z    |
2019-10-09T17:34:50.6680756Z LL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LateLintPassObject, LintArray};
2019-10-09T17:34:50.6680872Z    |
2019-10-09T17:34:50.6680936Z    = note: `#[warn(unused_imports)]` on by default
2019-10-09T17:34:50.6680967Z 
2019-10-09T17:34:50.6680967Z 
2019-10-09T17:34:50.6681333Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-09T17:34:50.6681627Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:31:1
2019-10-09T17:34:50.6681718Z LL | #[plugin_registrar]
2019-10-09T17:34:50.6681785Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-09T17:34:50.6681829Z    |
2019-10-09T17:34:50.6681874Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6681874Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6681905Z 
2019-10-09T17:34:50.6682397Z error[E0599]: no method named `register_late_lint_pass` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-09T17:34:50.6682928Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:33:9
2019-10-09T17:34:50.6682990Z    |
2019-10-09T17:34:50.6683055Z LL |     reg.register_late_lint_pass(box Pass);
2019-10-09T17:34:50.6683105Z    |         ^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `register_llvm_pass`
2019-10-09T17:34:50.6683138Z 
2019-10-09T17:34:50.6683683Z error[E0599]: no method named `register_lint_group` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-09T17:34:50.6683971Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:34:9
2019-10-09T17:34:50.6684017Z    |
2019-10-09T17:34:50.6684077Z LL |     reg.register_lint_group("lint_me", None, vec![TEST_LINT, PLEASE_LINT]);
2019-10-09T17:34:50.6684331Z    |         ^^^^^^^^^^^^^^^^^^^ method not found in `&mut rustc_driver::rustc_plugin_impl::Registry<'_>`
2019-10-09T17:34:50.6684406Z error: aborting due to 2 previous errors
2019-10-09T17:34:50.6684569Z 
2019-10-09T17:34:50.6684817Z For more information about this error, try `rustc --explain E0599`.
2019-10-09T17:34:50.6684848Z 
2019-10-09T17:34:50.6684848Z 
2019-10-09T17:34:50.6685041Z ------------------------------------------
2019-10-09T17:34:50.6685090Z 
2019-10-09T17:34:50.6685113Z 
2019-10-09T17:34:50.6685320Z ---- [ui] ui-fulldeps/lint-group-plugin.rs stdout ----
2019-10-09T17:34:50.6685350Z 
2019-10-09T17:34:50.6685691Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
2019-10-09T17:34:50.6685751Z status: exit code: 1
2019-10-09T17:34:50.6686466Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary"
2019-10-09T17:34:50.6686839Z ------------------------------------------
2019-10-09T17:34:50.6686888Z 
2019-10-09T17:34:50.6687095Z ------------------------------------------
2019-10-09T17:34:50.6687137Z stderr:
2019-10-09T17:34:50.6687137Z stderr:
2019-10-09T17:34:50.6687349Z ------------------------------------------
2019-10-09T17:34:50.6687396Z warning: unused import: `LateLintPassObject`
2019-10-09T17:34:50.6687629Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:12:69
2019-10-09T17:34:50.6687692Z    |
2019-10-09T17:34:50.6687740Z LL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LateLintPassObject, LintArray};
2019-10-09T17:34:50.6687864Z    |
2019-10-09T17:34:50.6687905Z    = note: `#[warn(unused_imports)]` on by default
2019-10-09T17:34:50.6687933Z 
2019-10-09T17:34:50.6687933Z 
2019-10-09T17:34:50.6688839Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-09T17:34:50.6689171Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:31:1
2019-10-09T17:34:50.6689292Z LL | #[plugin_registrar]
2019-10-09T17:34:50.6689339Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-09T17:34:50.6689383Z    |
2019-10-09T17:34:50.6689446Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6689446Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6689478Z 
2019-10-09T17:34:50.6689798Z error[E0599]: no method named `register_late_lint_pass` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-09T17:34:50.6690060Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:33:9
2019-10-09T17:34:50.6690132Z    |
2019-10-09T17:34:50.6690185Z LL |     reg.register_late_lint_pass(box Pass);
2019-10-09T17:34:50.6690240Z    |         ^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `register_llvm_pass`
2019-10-09T17:34:50.6690294Z 
2019-10-09T17:34:50.6690608Z error[E0599]: no method named `register_lint_group` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-09T17:34:50.6690880Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:34:9
2019-10-09T17:34:50.6690948Z    |
2019-10-09T17:34:50.6690997Z LL |     reg.register_lint_group("lint_me", None, vec![TEST_LINT, PLEASE_LINT]);
2019-10-09T17:34:50.6691281Z    |         ^^^^^^^^^^^^^^^^^^^ method not found in `&mut rustc_driver::rustc_plugin_impl::Registry<'_>`
2019-10-09T17:34:50.6691381Z error: aborting due to 2 previous errors
2019-10-09T17:34:50.6691411Z 
2019-10-09T17:34:50.6691654Z For more information about this error, try `rustc --explain E0599`.
2019-10-09T17:34:50.6691823Z 
2019-10-09T17:34:50.6691823Z 
2019-10-09T17:34:50.6692251Z ------------------------------------------
2019-10-09T17:34:50.6692279Z 
2019-10-09T17:34:50.6692301Z 
2019-10-09T17:34:50.6692506Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-allow.rs stdout ----
2019-10-09T17:34:50.6692554Z 
2019-10-09T17:34:50.6692795Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-10-09T17:34:50.6692923Z status: exit code: 1
2019-10-09T17:34:50.6693644Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary"
2019-10-09T17:34:50.6693951Z ------------------------------------------
2019-10-09T17:34:50.6693981Z 
2019-10-09T17:34:50.6694171Z ------------------------------------------
2019-10-09T17:34:50.6694302Z stderr:
2019-10-09T17:34:50.6694302Z stderr:
2019-10-09T17:34:50.6694503Z ------------------------------------------
2019-10-09T17:34:50.6694994Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-09T17:34:50.6695472Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:29:1
2019-10-09T17:34:50.6695815Z LL | #[plugin_registrar]
2019-10-09T17:34:50.6696067Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-09T17:34:50.6696271Z    |
2019-10-09T17:34:50.6696310Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6696310Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6696381Z 
2019-10-09T17:34:50.6696663Z error[E0599]: no method named `register_early_lint_pass` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-09T17:34:50.6696901Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:31:9
2019-10-09T17:34:50.6696994Z    |
2019-10-09T17:34:50.6697036Z LL |     reg.register_early_lint_pass(box Pass as EarlyLintPassObject);
2019-10-09T17:34:50.6697094Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `register_llvm_pass`
2019-10-09T17:34:50.6697211Z error: aborting due to previous error
2019-10-09T17:34:50.6697237Z 
2019-10-09T17:34:50.6697455Z For more information about this error, try `rustc --explain E0599`.
2019-10-09T17:34:50.6697485Z 
2019-10-09T17:34:50.6697485Z 
2019-10-09T17:34:50.6697725Z ------------------------------------------
2019-10-09T17:34:50.6697754Z 
2019-10-09T17:34:50.6697776Z 
2019-10-09T17:34:50.6697979Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-load.rs stdout ----
2019-10-09T17:34:50.6698015Z 
2019-10-09T17:34:50.6751415Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-10-09T17:34:50.6751509Z status: exit code: 1
2019-10-09T17:34:50.6752875Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary"
2019-10-09T17:34:50.6753201Z ------------------------------------------
2019-10-09T17:34:50.6753235Z 
2019-10-09T17:34:50.6753436Z ------------------------------------------
2019-10-09T17:34:50.6753671Z stderr:
2019-10-09T17:34:50.6753671Z stderr:
2019-10-09T17:34:50.6753912Z ------------------------------------------
2019-10-09T17:34:50.6754445Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-09T17:34:50.6754726Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:29:1
2019-10-09T17:34:50.6754824Z LL | #[plugin_registrar]
2019-10-09T17:34:50.6754999Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-09T17:34:50.6755055Z    |
2019-10-09T17:34:50.6755101Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6755101Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6755131Z 
2019-10-09T17:34:50.6755491Z error[E0599]: no method named `register_early_lint_pass` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-09T17:34:50.6766017Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:31:9
2019-10-09T17:34:50.6766103Z    |
2019-10-09T17:34:50.6766180Z LL |     reg.register_early_lint_pass(box Pass as EarlyLintPassObject);
2019-10-09T17:34:50.6766251Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `register_llvm_pass`
2019-10-09T17:34:50.6766342Z error: aborting due to previous error
2019-10-09T17:34:50.6766370Z 
2019-10-09T17:34:50.6767017Z For more information about this error, try `rustc --explain E0599`.
2019-10-09T17:34:50.6767054Z 
2019-10-09T17:34:50.6767054Z 
2019-10-09T17:34:50.6767293Z ------------------------------------------
2019-10-09T17:34:50.6767324Z 
2019-10-09T17:34:50.6767349Z 
2019-10-09T17:34:50.6767565Z ---- [ui] ui-fulldeps/lint-plugin-deny-attr.rs stdout ----
2019-10-09T17:34:50.6767595Z 
2019-10-09T17:34:50.6767853Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-10-09T17:34:50.6767905Z status: exit code: 1
2019-10-09T17:34:50.6769199Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary"
2019-10-09T17:34:50.6769573Z ------------------------------------------
2019-10-09T17:34:50.6769609Z 
2019-10-09T17:34:50.6769830Z ------------------------------------------
2019-10-09T17:34:50.6769878Z stderr:
2019-10-09T17:34:50.6769878Z stderr:
2019-10-09T17:34:50.6770100Z ------------------------------------------
2019-10-09T17:34:50.6770480Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-09T17:34:50.6770759Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:29:1
2019-10-09T17:34:50.6770868Z LL | #[plugin_registrar]
2019-10-09T17:34:50.6770922Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-09T17:34:50.6770968Z    |
2019-10-09T17:34:50.6771015Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6771015Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6771046Z 
2019-10-09T17:34:50.6771382Z error[E0599]: no method named `register_early_lint_pass` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-09T17:34:50.6771648Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:31:9
2019-10-09T17:34:50.6771699Z    |
2019-10-09T17:34:50.6771755Z LL |     reg.register_early_lint_pass(box Pass as EarlyLintPassObject);
2019-10-09T17:34:50.6771812Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `register_llvm_pass`
2019-10-09T17:34:50.6771895Z error: aborting due to previous error
2019-10-09T17:34:50.6771925Z 
2019-10-09T17:34:50.6772416Z For more information about this error, try `rustc --explain E0599`.
2019-10-09T17:34:50.6772452Z 
2019-10-09T17:34:50.6772452Z 
2019-10-09T17:34:50.6772687Z ------------------------------------------
2019-10-09T17:34:50.6772720Z 
2019-10-09T17:34:50.6772746Z 
2019-10-09T17:34:50.6772982Z ---- [ui] ui-fulldeps/lint-plugin-deny-cmdline.rs stdout ----
2019-10-09T17:34:50.6773016Z 
2019-10-09T17:34:50.6773402Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-10-09T17:34:50.6773469Z status: exit code: 1
2019-10-09T17:34:50.6774610Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary"
2019-10-09T17:34:50.6774906Z ------------------------------------------
2019-10-09T17:34:50.6774937Z 
2019-10-09T17:34:50.6775130Z ------------------------------------------
2019-10-09T17:34:50.6775171Z stderr:
2019-10-09T17:34:50.6775171Z stderr:
2019-10-09T17:34:50.6775371Z ------------------------------------------
2019-10-09T17:34:50.6775676Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-09T17:34:50.6775920Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:29:1
2019-10-09T17:34:50.6776002Z LL | #[plugin_registrar]
2019-10-09T17:34:50.6776061Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-09T17:34:50.6776099Z    |
2019-10-09T17:34:50.6776139Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6776139Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6776172Z 
2019-10-09T17:34:50.6776471Z error[E0599]: no method named `register_early_lint_pass` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-09T17:34:50.6776700Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:31:9
2019-10-09T17:34:50.6776743Z    |
2019-10-09T17:34:50.6776803Z LL |     reg.register_early_lint_pass(box Pass as EarlyLintPassObject);
2019-10-09T17:34:50.6776859Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `register_llvm_pass`
2019-10-09T17:34:50.6776945Z error: aborting due to previous error
2019-10-09T17:34:50.6776971Z 
2019-10-09T17:34:50.6777188Z For more information about this error, try `rustc --explain E0599`.
2019-10-09T17:34:50.6777218Z 
2019-10-09T17:34:50.6777218Z 
2019-10-09T17:34:50.6777420Z ------------------------------------------
2019-10-09T17:34:50.6777447Z 
2019-10-09T17:34:50.6777469Z 
2019-10-09T17:34:50.6777672Z ---- [ui] ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----
2019-10-09T17:34:50.6777708Z 
2019-10-09T17:34:50.6777965Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-10-09T17:34:50.6778012Z status: exit code: 1
2019-10-09T17:34:50.6779470Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary"
2019-10-09T17:34:50.6779834Z ------------------------------------------
2019-10-09T17:34:50.6779870Z 
2019-10-09T17:34:50.6780249Z ------------------------------------------
2019-10-09T17:34:50.6780298Z stderr:
2019-10-09T17:34:50.6780298Z stderr:
2019-10-09T17:34:50.6780534Z ------------------------------------------
2019-10-09T17:34:50.6780888Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-09T17:34:50.6781173Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:29:1
2019-10-09T17:34:50.6781369Z LL | #[plugin_registrar]
2019-10-09T17:34:50.6781436Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-09T17:34:50.6781482Z    |
2019-10-09T17:34:50.6781529Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6781529Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6781560Z 
2019-10-09T17:34:50.6781926Z error[E0599]: no method named `register_early_lint_pass` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-09T17:34:50.6782334Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:31:9
2019-10-09T17:34:50.6782551Z    |
2019-10-09T17:34:50.6782612Z LL |     reg.register_early_lint_pass(box Pass as EarlyLintPassObject);
2019-10-09T17:34:50.6782662Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `register_llvm_pass`
2019-10-09T17:34:50.6782744Z error: aborting due to previous error
2019-10-09T17:34:50.6782770Z 
2019-10-09T17:34:50.6782995Z For more information about this error, try `rustc --explain E0599`.
2019-10-09T17:34:50.6783025Z 
2019-10-09T17:34:50.6783025Z 
2019-10-09T17:34:50.6783228Z ------------------------------------------
2019-10-09T17:34:50.6783256Z 
2019-10-09T17:34:50.6783279Z 
2019-10-09T17:34:50.6783485Z ---- [ui] ui-fulldeps/lint-plugin-forbid-cmdline.rs stdout ----
2019-10-09T17:34:50.6783514Z 
2019-10-09T17:34:50.6783770Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-10-09T17:34:50.6783816Z status: exit code: 1
2019-10-09T17:34:50.6784518Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary"
2019-10-09T17:34:50.6784825Z ------------------------------------------
2019-10-09T17:34:50.6784855Z 
2019-10-09T17:34:50.6785045Z ------------------------------------------
2019-10-09T17:34:50.6785085Z stderr:
2019-10-09T17:34:50.6785085Z stderr:
2019-10-09T17:34:50.6785288Z ------------------------------------------
2019-10-09T17:34:50.6785585Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-09T17:34:50.6785842Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:29:1
2019-10-09T17:34:50.6785925Z LL | #[plugin_registrar]
2019-10-09T17:34:50.6785982Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-09T17:34:50.6786021Z    |
2019-10-09T17:34:50.6786061Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6786061Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T17:34:50.6786087Z 
2019-10-09T17:34:50.6786385Z error[E0599]: no method named `register_early_lint_pass` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-09T17:34:50.6786615Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:31:9
2019-10-09T17:34:50.6786659Z    |
2019-10-09T17:34:50.6786716Z LL |     reg.register_early_lint_pass(box Pass as EarlyLintPassObject);
2019-10-09T17:34:50.6786765Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `register_llvm_pass`
2019-10-09T17:34:50.6786847Z error: aborting due to previous error
2019-10-09T17:34:50.6786989Z 
2019-10-09T17:34:50.6787228Z For more information about this error, try `rustc --explain E0599`.
2019-10-09T17:34:50.6787258Z 
2019-10-09T17:34:50.6787258Z 
---
2019-10-09T17:34:50.6817146Z main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-09T17:34:50.6817288Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-09T17:34:50.6817325Z 
2019-10-09T17:34:50.6817377Z 
2019-10-09T17:34:50.6820465Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-09T17:34:50.6821010Z 
2019-10-09T17:34:50.6821043Z 
2019-10-09T17:34:50.6821180Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-09T17:34:50.6821263Z Build completed unsuccessfully in 1:12:27
2019-10-09T17:34:50.6821263Z Build completed unsuccessfully in 1:12:27
2019-10-09T17:34:50.6821313Z == clock drift check ==
2019-10-09T17:34:50.6821360Z   local time: Wed Oct  9 17:34:50 UTC 2019
2019-10-09T17:34:50.7663062Z   network time: Wed, 09 Oct 2019 17:34:50 GMT
2019-10-09T17:34:50.7667973Z == end clock drift check ==
2019-10-09T17:34:51.4358486Z ##[error]Bash exited with code '1'.
2019-10-09T17:34:51.4412425Z ##[section]Starting: Checkout
2019-10-09T17:34:51.4414362Z ==============================================================================
2019-10-09T17:34:51.4414420Z Task         : Get sources
2019-10-09T17:34:51.4414486Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
