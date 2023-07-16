plain
2019-10-07T22:58:22.6429779Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-07T22:58:22.6649776Z ##[command]git config gc.auto 0
2019-10-07T22:58:22.6732200Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-07T22:58:22.6790862Z ##[command]git config --get-all http.proxy
2019-10-07T22:58:23.5033115Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65193/merge:refs/remotes/pull/65193/merge
---
2019-10-07T23:58:49.9474852Z .................................................................................................... 1600/9124
2019-10-07T23:58:57.7725685Z .................................................................................................... 1700/9124
2019-10-07T23:59:08.3922552Z ...............i...............i.................................................................... 1800/9124
2019-10-07T23:59:15.5913703Z .................................................................................................... 1900/9124
2019-10-07T23:59:31.8702665Z ......iiiii......................................................................................... 2000/9124
2019-10-07T23:59:41.5373457Z .................................................................................................... 2200/9124
2019-10-07T23:59:44.2060301Z .................................................................................................... 2300/9124
2019-10-07T23:59:50.0163646Z .................................................................................................... 2400/9124
2019-10-07T23:59:56.1467382Z .................................................................................................... 2500/9124
---
2019-10-08T00:02:48.0813314Z ...............................................................................................i.... 4700/9124
2019-10-08T00:02:55.3506354Z ...........i........................................................................................ 4800/9124
2019-10-08T00:03:06.2211279Z .................................................................................................... 4900/9124
2019-10-08T00:03:11.9202715Z .................................................................................................... 5000/9124
2019-10-08T00:03:23.6388051Z .........................................................................................ii.ii...... 5100/9124
2019-10-08T00:03:33.6213529Z .................................................................................................... 5300/9124
2019-10-08T00:03:43.2535532Z .................................................................................................... 5400/9124
2019-10-08T00:03:50.2451315Z .......................................................i............................................ 5500/9124
2019-10-08T00:03:57.2357744Z .................................................................................................... 5600/9124
2019-10-08T00:03:57.2357744Z .................................................................................................... 5600/9124
2019-10-08T00:04:05.6183572Z .................................................................................................... 5700/9124
2019-10-08T00:04:16.6288271Z ....................................................ii...i..ii...........i.......................... 5800/9124
2019-10-08T00:04:42.2294777Z .................................................................................................... 6000/9124
2019-10-08T00:04:50.8907781Z .................................................................................................... 6100/9124
2019-10-08T00:04:50.8907781Z .................................................................................................... 6100/9124
2019-10-08T00:04:58.0887084Z ..........................................................i..ii..................................... 6200/9124
2019-10-08T00:05:25.7137564Z .................................................................................................... 6400/9124
2019-10-08T00:05:27.8821132Z ..................i................................................................................. 6500/9124
2019-10-08T00:05:30.1967569Z ...........................................................................................i........ 6600/9124
2019-10-08T00:05:33.0275997Z .................................................................................................... 6700/9124
---
2019-10-08T00:10:08.3973369Z  finished in 5.489
2019-10-08T00:10:08.4154491Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-08T00:10:08.5745244Z 
2019-10-08T00:10:08.5745622Z running 150 tests
2019-10-08T00:10:11.8538412Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-10-08T00:10:13.8280061Z ..iiii..............i.........iii.i.......ii......
2019-10-08T00:10:13.8281524Z 
2019-10-08T00:10:13.8288038Z  finished in 5.413
2019-10-08T00:10:13.8466810Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-08T00:10:14.0027082Z 
---
2019-10-08T00:10:16.0743815Z  finished in 2.227
2019-10-08T00:10:16.0921694Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-08T00:10:16.2522988Z 
2019-10-08T00:10:16.2523901Z running 9 tests
2019-10-08T00:10:16.2553325Z iiiiiiiii
2019-10-08T00:10:16.2563703Z 
2019-10-08T00:10:16.2569985Z  finished in 0.163
2019-10-08T00:10:16.2744764Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-08T00:10:16.4358084Z 
---
2019-10-08T00:10:35.1519746Z  finished in 17.996
2019-10-08T00:10:35.1520629Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-08T00:10:35.1520681Z 
2019-10-08T00:10:35.1520730Z running 123 tests
2019-10-08T00:10:58.7318737Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-08T00:11:03.3595816Z i.i.i......iii.i.....ii
2019-10-08T00:11:03.3596387Z 
2019-10-08T00:11:03.3598334Z  finished in 29.068
2019-10-08T00:11:03.3608530Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-08T00:11:03.3611559Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-08T00:11:03.3611559Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-08T00:11:03.3826314Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-08T00:11:04.1589623Z 
2019-10-08T00:11:04.1597363Z running 69 tests
2019-10-08T00:12:00.9219880Z ................F......F...F.....F.FFFFFFF.FFFF.....................thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-08T00:12:00.9228886Z .
2019-10-08T00:12:00.9229199Z failures:
2019-10-08T00:12:00.9229365Z 
2019-10-08T00:12:00.9229831Z ---- [ui] ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs stdout ----
2019-10-08T00:12:00.9229831Z ---- [ui] ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs stdout ----
2019-10-08T00:12:00.9230039Z diff of stderr:
2019-10-08T00:12:00.9230205Z 
2019-10-08T00:12:00.9230360Z + error[E0407]: method `get_lints` is not a member of trait `LintPass`
2019-10-08T00:12:00.9230763Z +   --> $DIR/lint_pass_impl_without_macro.rs:20:5
2019-10-08T00:12:00.9231706Z +    |
2019-10-08T00:12:00.9232200Z + LL | /     fn get_lints(&self) -> LintArray {
2019-10-08T00:12:00.9232413Z + LL | |         lint_array!(TEST_LINT)
2019-10-08T00:12:00.9232584Z + LL | |     }
2019-10-08T00:12:00.9232733Z +    | |_____^ not a member of trait `LintPass`
2019-10-08T00:12:00.9233041Z + error[E0407]: method `get_lints` is not a member of trait `LintPass`
2019-10-08T00:12:00.9233437Z +   --> $DIR/lint_pass_impl_without_macro.rs:34:13
2019-10-08T00:12:00.9233625Z +    |
2019-10-08T00:12:00.9233625Z +    |
2019-10-08T00:12:00.9234044Z + LL | /             fn get_lints(&self) -> LintArray {
2019-10-08T00:12:00.9234247Z + LL | |                 lint_array!(TEST_LINT)
2019-10-08T00:12:00.9234412Z + LL | |             }
2019-10-08T00:12:00.9234567Z +    | |_____________^ not a member of trait `LintPass`
2019-10-08T00:12:00.9234710Z + ...
2019-10-08T00:12:00.9234886Z + LL |   custom_lint_pass_macro!();
2019-10-08T00:12:00.9235486Z + 
2019-10-08T00:12:00.9235650Z 1 error: implementing `LintPass` by hand
2019-10-08T00:12:00.9236032Z 2   --> $DIR/lint_pass_impl_without_macro.rs:19:6
2019-10-08T00:12:00.9236223Z 3    |
2019-10-08T00:12:00.9236223Z 3    |
2019-10-08T00:12:00.9236367Z 
2019-10-08T00:12:00.9236508Z 22    |
2019-10-08T00:12:00.9236654Z 23    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2019-10-08T00:12:00.9237558Z - error: aborting due to 2 previous errors
2019-10-08T00:12:00.9237758Z + error: aborting due to 4 previous errors
2019-10-08T00:12:00.9237919Z 26 
2019-10-08T00:12:00.9238519Z + For more information about this error, try `rustc --explain E0407`.
2019-10-08T00:12:00.9238519Z + For more information about this error, try `rustc --explain E0407`.
2019-10-08T00:12:00.9238775Z 27 
2019-10-08T00:12:00.9238924Z 
2019-10-08T00:12:00.9239051Z 
2019-10-08T00:12:00.9239197Z The actual stderr differed from the expected stderr.
2019-10-08T00:12:00.9239759Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/lint_pass_impl_without_macro.stderr
2019-10-08T00:12:00.9240240Z To update references, rerun the tests and pass the `--bless` flag
2019-10-08T00:12:00.9240717Z To only update this specific test, also pass `--test-args internal-lints/lint_pass_impl_without_macro.rs`
2019-10-08T00:12:00.9241716Z error: 1 errors occurred comparing output.
2019-10-08T00:12:00.9242133Z status: exit code: 1
2019-10-08T00:12:00.9242133Z status: exit code: 1
2019-10-08T00:12:00.9243435Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/auxiliary" "-A" "unused"
2019-10-08T00:12:00.9244487Z ------------------------------------------
2019-10-08T00:12:00.9247845Z 
2019-10-08T00:12:00.9248601Z ------------------------------------------
2019-10-08T00:12:00.9248825Z stderr:
2019-10-08T00:12:00.9248825Z stderr:
2019-10-08T00:12:00.9249198Z ------------------------------------------
2019-10-08T00:12:00.9249875Z error[E0407]: method `get_lints` is not a member of trait `LintPass`
2019-10-08T00:12:00.9250605Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:20:5
2019-10-08T00:12:00.9250958Z    |
2019-10-08T00:12:00.9251765Z LL | /     fn get_lints(&self) -> LintArray {
2019-10-08T00:12:00.9251980Z LL | |         lint_array!(TEST_LINT)
2019-10-08T00:12:00.9252290Z    | |_____^ not a member of trait `LintPass`
2019-10-08T00:12:00.9252414Z 
2019-10-08T00:12:00.9252834Z error[E0407]: method `get_lints` is not a member of trait `LintPass`
2019-10-08T00:12:00.9253335Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:34:13
2019-10-08T00:12:00.9253335Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:34:13
2019-10-08T00:12:00.9253549Z    |
2019-10-08T00:12:00.9253957Z LL | /             fn get_lints(&self) -> LintArray {
2019-10-08T00:12:00.9257107Z LL | |                 lint_array!(TEST_LINT)
2019-10-08T00:12:00.9257232Z    | |_____________^ not a member of trait `LintPass`
2019-10-08T00:12:00.9257277Z ...
2019-10-08T00:12:00.9257277Z ...
2019-10-08T00:12:00.9257338Z LL |   custom_lint_pass_macro!();
2019-10-08T00:12:00.9257747Z 
2019-10-08T00:12:00.9257791Z error: implementing `LintPass` by hand
2019-10-08T00:12:00.9258083Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:19:6
2019-10-08T00:12:00.9258134Z    |
2019-10-08T00:12:00.9258134Z    |
2019-10-08T00:12:00.9258184Z LL | impl LintPass for Foo { //~ERROR implementing `LintPass` by hand
2019-10-08T00:12:00.9258299Z    |
2019-10-08T00:12:00.9258343Z note: lint level defined here
2019-10-08T00:12:00.9258627Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:4:9
2019-10-08T00:12:00.9258678Z    |
2019-10-08T00:12:00.9258678Z    |
2019-10-08T00:12:00.9258724Z LL | #![deny(rustc::lint_pass_impl_without_macro)]
2019-10-08T00:12:00.9258773Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-10-08T00:12:00.9258841Z    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2019-10-08T00:12:00.9259075Z error: implementing `LintPass` by hand
2019-10-08T00:12:00.9259435Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:33:14
2019-10-08T00:12:00.9259488Z    |
2019-10-08T00:12:00.9259488Z    |
2019-10-08T00:12:00.9259537Z LL |         impl LintPass for Custom { //~ERROR implementing `LintPass` by hand
2019-10-08T00:12:00.9259652Z ...
2019-10-08T00:12:00.9259652Z ...
2019-10-08T00:12:00.9259697Z LL | custom_lint_pass_macro!();
2019-10-08T00:12:00.9259999Z    |
2019-10-08T00:12:00.9259999Z    |
2019-10-08T00:12:00.9260047Z    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2019-10-08T00:12:00.9260142Z error: aborting due to 4 previous errors
2019-10-08T00:12:00.9260172Z 
2019-10-08T00:12:00.9260414Z For more information about this error, try `rustc --explain E0407`.
2019-10-08T00:12:00.9260450Z 
2019-10-08T00:12:00.9260450Z 
2019-10-08T00:12:00.9260680Z ------------------------------------------
2019-10-08T00:12:00.9260714Z 
2019-10-08T00:12:00.9260749Z 
2019-10-08T00:12:00.9261228Z ---- [ui] ui-fulldeps/issue-15778-fail.rs stdout ----
2019-10-08T00:12:00.9261289Z 
2019-10-08T00:12:00.9261645Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" failed to compile: 
2019-10-08T00:12:00.9261703Z status: exit code: 1
2019-10-08T00:12:00.9262485Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary"
2019-10-08T00:12:00.9262823Z ------------------------------------------
2019-10-08T00:12:00.9262860Z 
2019-10-08T00:12:00.9263086Z ------------------------------------------
2019-10-08T00:12:00.9263134Z stderr:
2019-10-08T00:12:00.9263134Z stderr:
2019-10-08T00:12:00.9263366Z ------------------------------------------
2019-10-08T00:12:00.9263418Z warning: unused import: `LateLintPassObject`
2019-10-08T00:12:00.9263668Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:10:69
2019-10-08T00:12:00.9263891Z    |
2019-10-08T00:12:00.9263948Z LL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LateLintPassObject, LintArray};
2019-10-08T00:12:00.9264072Z    |
2019-10-08T00:12:00.9264118Z    = note: `#[warn(unused_imports)]` on by default
2019-10-08T00:12:00.9264149Z 
2019-10-08T00:12:00.9264149Z 
2019-10-08T00:12:00.9264672Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-08T00:12:00.9264996Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:33:1
2019-10-08T00:12:00.9265114Z LL | #[plugin_registrar]
2019-10-08T00:12:00.9265165Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-08T00:12:00.9265212Z    |
2019-10-08T00:12:00.9265276Z    = note: `#[warn(deprecated)]` on by default
2019-10-08T00:12:00.9265276Z    = note: `#[warn(deprecated)]` on by default
2019-10-08T00:12:00.9265318Z 
2019-10-08T00:12:00.9265363Z error[E0308]: mismatched types
2019-10-08T00:12:00.9265642Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:35:33
2019-10-08T00:12:00.9265709Z    |
2019-10-08T00:12:00.9265758Z LL |     reg.register_late_lint_pass(box Pass);
2019-10-08T00:12:00.9265816Z    |                                 ^^^^^^^^ expected fn pointer, found struct `std::boxed::Box`
2019-10-08T00:12:00.9265882Z    |
2019-10-08T00:12:00.9266448Z    = note: expected type `fn() -> std::boxed::Box<(dyn for<'a, 'tcx> rustc::lint::LateLintPass<'a, 'tcx> + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-08T00:12:00.9266532Z               found type `std::boxed::Box<Pass>`
2019-10-08T00:12:00.9266638Z error: aborting due to previous error
2019-10-08T00:12:00.9266668Z 
2019-10-08T00:12:00.9266988Z For more information about this error, try `rustc --explain E0308`.
2019-10-08T00:12:00.9267041Z 
2019-10-08T00:12:00.9267041Z 
2019-10-08T00:12:00.9267294Z ------------------------------------------
2019-10-08T00:12:00.9267329Z 
2019-10-08T00:12:00.9267357Z 
2019-10-08T00:12:00.9267624Z ---- [ui] ui-fulldeps/issue-15778-pass.rs stdout ----
2019-10-08T00:12:00.9267661Z 
2019-10-08T00:12:00.9267960Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" failed to compile: 
2019-10-08T00:12:00.9268018Z status: exit code: 1
2019-10-08T00:12:00.9268811Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary"
2019-10-08T00:12:00.9269170Z ------------------------------------------
2019-10-08T00:12:00.9269207Z 
2019-10-08T00:12:00.9269443Z ------------------------------------------
2019-10-08T00:12:00.9269508Z stderr:
2019-10-08T00:12:00.9269508Z stderr:
2019-10-08T00:12:00.9269742Z ------------------------------------------
2019-10-08T00:12:00.9269799Z error[E0407]: method `get_lints` is not a member of trait `LintPass`
2019-10-08T00:12:00.9270092Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:25:13
2019-10-08T00:12:00.9270148Z    |
2019-10-08T00:12:00.9270401Z LL | /             fn get_lints(&self) -> LintArray {
2019-10-08T00:12:00.9270471Z LL | |                 $lints
2019-10-08T00:12:00.9270568Z    | |_____________^ not a member of trait `LintPass`
2019-10-08T00:12:00.9270631Z ...
2019-10-08T00:12:00.9270631Z ...
2019-10-08T00:12:00.9270678Z LL | / fake_lint_pass! {
2019-10-08T00:12:00.9270722Z LL | |     PassOkay,
2019-10-08T00:12:00.9270885Z LL | |     lint_array!(CRATE_NOT_OKAY), // Single lint
2019-10-08T00:12:00.9270953Z LL | |     Symbol::intern("rustc_crate_okay")
2019-10-08T00:12:00.9271554Z    | |_- in this macro invocation
2019-10-08T00:12:00.9271609Z 
2019-10-08T00:12:00.9271659Z error[E0407]: method `get_lints` is not a member of trait `LintPass`
2019-10-08T00:12:00.9271920Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:25:13
2019-10-08T00:12:00.9271920Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:25:13
2019-10-08T00:12:00.9271972Z    |
2019-10-08T00:12:00.9272214Z LL | /             fn get_lints(&self) -> LintArray {
2019-10-08T00:12:00.9272267Z LL | |                 $lints
2019-10-08T00:12:00.9272387Z    | |_____________^ not a member of trait `LintPass`
2019-10-08T00:12:00.9272432Z ...
2019-10-08T00:12:00.9272432Z ...
2019-10-08T00:12:00.9272475Z LL | / fake_lint_pass! {
2019-10-08T00:12:00.9272532Z LL | |     PassRedBlue,
2019-10-08T00:12:00.9272583Z LL | |     lint_array!(CRATE_NOT_RED, CRATE_NOT_BLUE), // Multiple lints
2019-10-08T00:12:00.9272646Z LL | |     Symbol::intern("rustc_crate_red"), Symbol::intern("rustc_crate_blue")
2019-10-08T00:12:00.9272929Z    | |_- in this macro invocation
2019-10-08T00:12:00.9272964Z 
2019-10-08T00:12:00.9273012Z error[E0407]: method `get_lints` is not a member of trait `LintPass`
2019-10-08T00:12:00.9273284Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:25:13
2019-10-08T00:12:00.9273284Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:25:13
2019-10-08T00:12:00.9273334Z    |
2019-10-08T00:12:00.9273558Z LL | /             fn get_lints(&self) -> LintArray {
2019-10-08T00:12:00.9273623Z LL | |                 $lints
2019-10-08T00:12:00.9273859Z    | |_____________^ not a member of trait `LintPass`
2019-10-08T00:12:00.9273902Z ...
2019-10-08T00:12:00.9273902Z ...
2019-10-08T00:12:00.9273961Z LL | / fake_lint_pass! {
2019-10-08T00:12:00.9274005Z LL | |     PassGreyGreen,
2019-10-08T00:12:00.9274055Z LL | |     lint_array!(CRATE_NOT_GREY, CRATE_NOT_GREEN, ), // Trailing comma
2019-10-08T00:12:00.9274134Z LL | |     Symbol::intern("rustc_crate_grey"), Symbol::intern("rustc_crate_green")
2019-10-08T00:12:00.9274444Z    | |_- in this macro invocation
2019-10-08T00:12:00.9274494Z 
2019-10-08T00:12:00.9274494Z 
2019-10-08T00:12:00.9274539Z warning: unused import: `LateLintPassObject`
2019-10-08T00:12:00.9274818Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:10:69
2019-10-08T00:12:00.9274871Z    |
2019-10-08T00:12:00.9274941Z LL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LateLintPassObject, LintArray};
2019-10-08T00:12:00.9275084Z    |
2019-10-08T00:12:00.9275133Z    = note: `#[warn(unused_imports)]` on by default
2019-10-08T00:12:00.9275166Z 
2019-10-08T00:12:00.9275166Z 
2019-10-08T00:12:00.9275579Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-08T00:12:00.9275881Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:68:1
2019-10-08T00:12:00.9275990Z LL | #[plugin_registrar]
2019-10-08T00:12:00.9276058Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-08T00:12:00.9276105Z    |
2019-10-08T00:12:00.9276152Z    = note: `#[warn(deprecated)]` on by default
2019-10-08T00:12:00.9276152Z    = note: `#[warn(deprecated)]` on by default
2019-10-08T00:12:00.9276200Z 
2019-10-08T00:12:00.9276246Z error[E0308]: mismatched types
2019-10-08T00:12:00.9276525Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:70:33
2019-10-08T00:12:00.9276579Z    |
2019-10-08T00:12:00.9276642Z LL |     reg.register_late_lint_pass(box PassOkay);
2019-10-08T00:12:00.9276707Z    |                                 ^^^^^^^^^^^^ expected fn pointer, found struct `std::boxed::Box`
2019-10-08T00:12:00.9276759Z    |
2019-10-08T00:12:00.9277581Z    = note: expected type `fn() -> std::boxed::Box<(dyn for<'a, 'tcx> rustc::lint::LateLintPass<'a, 'tcx> + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-08T00:12:00.9277820Z               found type `std::boxed::Box<PassOkay>`
2019-10-08T00:12:00.9277926Z error[E0308]: mismatched types
2019-10-08T00:12:00.9278257Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:71:33
2019-10-08T00:12:00.9278311Z    |
2019-10-08T00:12:00.9278311Z    |
2019-10-08T00:12:00.9278376Z LL |     reg.register_late_lint_pass(box PassRedBlue);
2019-10-08T00:12:00.9278434Z    |                                 ^^^^^^^^^^^^^^^ expected fn pointer, found struct `std::boxed::Box`
2019-10-08T00:12:00.9278484Z    |
2019-10-08T00:12:00.9278883Z    = note: expected type `fn() -> std::boxed::Box<(dyn for<'a, 'tcx> rustc::lint::LateLintPass<'a, 'tcx> + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-08T00:12:00.9278952Z               found type `std::boxed::Box<PassRedBlue>`
2019-10-08T00:12:00.9279045Z error[E0308]: mismatched types
2019-10-08T00:12:00.9279322Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:72:33
2019-10-08T00:12:00.9279384Z    |
2019-10-08T00:12:00.9279384Z    |
2019-10-08T00:12:00.9279432Z LL |     reg.register_late_lint_pass(box PassGreyGreen);
2019-10-08T00:12:00.9279505Z    |                                 ^^^^^^^^^^^^^^^^^ expected fn pointer, found struct `std::boxed::Box`
2019-10-08T00:12:00.9279554Z    |
2019-10-08T00:12:00.9279927Z    = note: expected type `fn() -> std::boxed::Box<(dyn for<'a, 'tcx> rustc::lint::LateLintPass<'a, 'tcx> + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-08T00:12:00.9280008Z               found type `std::boxed::Box<PassGreyGreen>`
2019-10-08T00:12:00.9280201Z error: aborting due to 6 previous errors
2019-10-08T00:12:00.9280247Z 
2019-10-08T00:12:00.9280296Z Some errors have detailed explanations: E0308, E0407.
2019-10-08T00:12:00.9280600Z For more information about an error, try `rustc --explain E0308`.
2019-10-08T00:12:00.9280600Z For more information about an error, try `rustc --explain E0308`.
2019-10-08T00:12:00.9280636Z 
2019-10-08T00:12:00.9280885Z ------------------------------------------
2019-10-08T00:12:00.9280929Z 
2019-10-08T00:12:00.9280957Z 
2019-10-08T00:12:00.9281508Z ---- [ui] ui-fulldeps/issue-40001.rs stdout ----
2019-10-08T00:12:00.9281566Z 
2019-10-08T00:12:00.9281905Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
2019-10-08T00:12:00.9281965Z status: exit code: 1
2019-10-08T00:12:00.9282736Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
2019-10-08T00:12:00.9283077Z ------------------------------------------
2019-10-08T00:12:00.9283112Z 
2019-10-08T00:12:00.9283333Z ------------------------------------------
2019-10-08T00:12:00.9283382Z stderr:
2019-10-08T00:12:00.9283382Z stderr:
2019-10-08T00:12:00.9283609Z ------------------------------------------
2019-10-08T00:12:00.9283661Z warning: unused import: `syntax::ext::base::*`
2019-10-08T00:12:00.9283912Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:11:5
2019-10-08T00:12:00.9284026Z LL | use syntax::ext::base::*;
2019-10-08T00:12:00.9284071Z    |     ^^^^^^^^^^^^^^^^^^^^
2019-10-08T00:12:00.9284137Z    |
2019-10-08T00:12:00.9284185Z    = note: `#[warn(unused_imports)]` on by default
2019-10-08T00:12:00.9284185Z    = note: `#[warn(unused_imports)]` on by default
2019-10-08T00:12:00.9284216Z 
2019-10-08T00:12:00.9284261Z warning: unused import: `rustc::hir::map as hir_map`
2019-10-08T00:12:00.9284538Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:17:5
2019-10-08T00:12:00.9284590Z    |
2019-10-08T00:12:00.9284783Z LL | use rustc::hir::map as hir_map;
2019-10-08T00:12:00.9284878Z 
2019-10-08T00:12:00.9284920Z warning: unused import: `rustc::ty`
2019-10-08T00:12:00.9284920Z warning: unused import: `rustc::ty`
2019-10-08T00:12:00.9285220Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:20:5
2019-10-08T00:12:00.9285327Z LL | use rustc::ty;
2019-10-08T00:12:00.9285371Z    |     ^^^^^^^^^
2019-10-08T00:12:00.9285413Z 
2019-10-08T00:12:00.9285456Z warning: unused import: `ast`
2019-10-08T00:12:00.9285456Z warning: unused import: `ast`
2019-10-08T00:12:00.9285710Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:21:14
2019-10-08T00:12:00.9285759Z    |
2019-10-08T00:12:00.9285826Z LL | use syntax::{ast, source_map};
2019-10-08T00:12:00.9285900Z 
2019-10-08T00:12:00.9285900Z 
2019-10-08T00:12:00.9286282Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-08T00:12:00.9286563Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:23:1
2019-10-08T00:12:00.9286685Z LL | #[plugin_registrar]
2019-10-08T00:12:00.9286732Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-08T00:12:00.9286776Z    |
2019-10-08T00:12:00.9286837Z    = note: `#[warn(deprecated)]` on by default
2019-10-08T00:12:00.9286837Z    = note: `#[warn(deprecated)]` on by default
2019-10-08T00:12:00.9286868Z 
2019-10-08T00:12:00.9286911Z error[E0308]: mismatched types
2019-10-08T00:12:00.9287168Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:25:33
2019-10-08T00:12:00.9287233Z    |
2019-10-08T00:12:00.9287281Z LL |     reg.register_late_lint_pass(box MissingWhitelistedAttrPass);
2019-10-08T00:12:00.9287446Z    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected fn pointer, found struct `std::boxed::Box`
2019-10-08T00:12:00.9287521Z    |
2019-10-08T00:12:00.9287916Z    = note: expected type `fn() -> std::boxed::Box<(dyn for<'a, 'tcx> rustc::lint::LateLintPass<'a, 'tcx> + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-08T00:12:00.9288009Z               found type `std::boxed::Box<MissingWhitelistedAttrPass>`
2019-10-08T00:12:00.9288091Z error: aborting due to previous error
2019-10-08T00:12:00.9288121Z 
2019-10-08T00:12:00.9288385Z For more information about this error, try `rustc --explain E0308`.
2019-10-08T00:12:00.9288422Z 
2019-10-08T00:12:00.9288422Z 
2019-10-08T00:12:00.9288637Z ------------------------------------------
2019-10-08T00:12:00.9288671Z 
2019-10-08T00:12:00.9288697Z 
2019-10-08T00:12:00.9288948Z ---- [ui] ui-fulldeps/lint-group-plugin-deny-cmdline.rs stdout ----
2019-10-08T00:12:00.9288982Z 
2019-10-08T00:12:00.9289267Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
2019-10-08T00:12:00.9289338Z status: exit code: 1
2019-10-08T00:12:00.9290112Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary"
2019-10-08T00:12:00.9290453Z ------------------------------------------
2019-10-08T00:12:00.9290489Z 
2019-10-08T00:12:00.9290728Z ------------------------------------------
2019-10-08T00:12:00.9290777Z stderr:
2019-10-08T00:12:00.9290777Z stderr:
2019-10-08T00:12:00.9291230Z ------------------------------------------
2019-10-08T00:12:00.9291295Z warning: unused import: `LateLintPassObject`
2019-10-08T00:12:00.9291634Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:12:69
2019-10-08T00:12:00.9291690Z    |
2019-10-08T00:12:00.9291902Z LL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LateLintPassObject, LintArray};
2019-10-08T00:12:00.9292026Z    |
2019-10-08T00:12:00.9292086Z    = note: `#[warn(unused_imports)]` on by default
2019-10-08T00:12:00.9292119Z 
2019-10-08T00:12:00.9292119Z 
2019-10-08T00:12:00.9292534Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-08T00:12:00.9292842Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:31:1
2019-10-08T00:12:00.9292945Z LL | #[plugin_registrar]
2019-10-08T00:12:00.9293011Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-08T00:12:00.9293055Z    |
2019-10-08T00:12:00.9293099Z    = note: `#[warn(deprecated)]` on by default
2019-10-08T00:12:00.9293099Z    = note: `#[warn(deprecated)]` on by default
2019-10-08T00:12:00.9293148Z 
2019-10-08T00:12:00.9293190Z error[E0308]: mismatched types
2019-10-08T00:12:00.9293451Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:33:33
2019-10-08T00:12:00.9293511Z    |
2019-10-08T00:12:00.9293573Z LL |     reg.register_late_lint_pass(box Pass);
2019-10-08T00:12:00.9293629Z    |                                 ^^^^^^^^ expected fn pointer, found struct `std::boxed::Box`
2019-10-08T00:12:00.9293676Z    |
2019-10-08T00:12:00.9294051Z    = note: expected type `fn() -> std::boxed::Box<(dyn for<'a, 'tcx> rustc::lint::LateLintPass<'a, 'tcx> + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-08T00:12:00.9294117Z               found type `std::boxed::Box<Pass>`
2019-10-08T00:12:00.9294329Z error: aborting due to previous error
2019-10-08T00:12:00.9294358Z 
2019-10-08T00:12:00.9294647Z For more information about this error, try `rustc --explain E0308`.
2019-10-08T00:12:00.9294683Z 
2019-10-08T00:12:00.9294683Z 
2019-10-08T00:12:00.9294918Z ------------------------------------------
2019-10-08T00:12:00.9294952Z 
2019-10-08T00:12:00.9294988Z 
2019-10-08T00:12:00.9295217Z ---- [ui] ui-fulldeps/lint-group-plugin.rs stdout ----
2019-10-08T00:12:00.9295268Z 
2019-10-08T00:12:00.9295549Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
2019-10-08T00:12:00.9295605Z status: exit code: 1
2019-10-08T00:12:00.9296373Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary"
2019-10-08T00:12:00.9296706Z ------------------------------------------
2019-10-08T00:12:00.9296750Z 
2019-10-08T00:12:00.9296971Z ------------------------------------------
2019-10-08T00:12:00.9297019Z stderr:
2019-10-08T00:12:00.9297019Z stderr:
2019-10-08T00:12:00.9297250Z ------------------------------------------
2019-10-08T00:12:00.9297302Z warning: unused import: `LateLintPassObject`
2019-10-08T00:12:00.9297555Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:12:69
2019-10-08T00:12:00.9297623Z    |
2019-10-08T00:12:00.9297678Z LL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LateLintPassObject, LintArray};
2019-10-08T00:12:00.9297818Z    |
2019-10-08T00:12:00.9297863Z    = note: `#[warn(unused_imports)]` on by default
2019-10-08T00:12:00.9297895Z 
2019-10-08T00:12:00.9297895Z 
2019-10-08T00:12:00.9298263Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-08T00:12:00.9302041Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:31:1
2019-10-08T00:12:00.9302165Z LL | #[plugin_registrar]
2019-10-08T00:12:00.9302213Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-08T00:12:00.9302258Z    |
2019-10-08T00:12:00.9302320Z    = note: `#[warn(deprecated)]` on by default
2019-10-08T00:12:00.9302320Z    = note: `#[warn(deprecated)]` on by default
2019-10-08T00:12:00.9302351Z 
2019-10-08T00:12:00.9302394Z error[E0308]: mismatched types
2019-10-08T00:12:00.9302658Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:33:33
2019-10-08T00:12:00.9302726Z    |
2019-10-08T00:12:00.9302786Z LL |     reg.register_late_lint_pass(box Pass);
2019-10-08T00:12:00.9302841Z    |                                 ^^^^^^^^ expected fn pointer, found struct `std::boxed::Box`
2019-10-08T00:12:00.9302906Z    |
2019-10-08T00:12:00.9303264Z    = note: expected type `fn() -> std::boxed::Box<(dyn for<'a, 'tcx> rustc::lint::LateLintPass<'a, 'tcx> + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-08T00:12:00.9303356Z               found type `std::boxed::Box<Pass>`
2019-10-08T00:12:00.9303438Z error: aborting due to previous error
2019-10-08T00:12:00.9303469Z 
2019-10-08T00:12:00.9306483Z For more information about this error, try `rustc --explain E0308`.
2019-10-08T00:12:00.9306546Z 
2019-10-08T00:12:00.9306546Z 
2019-10-08T00:12:00.9306884Z ------------------------------------------
2019-10-08T00:12:00.9306919Z 
2019-10-08T00:12:00.9306945Z 
2019-10-08T00:12:00.9307203Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-allow.rs stdout ----
2019-10-08T00:12:00.9307238Z 
2019-10-08T00:12:00.9307696Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-10-08T00:12:00.9307790Z status: exit code: 1
2019-10-08T00:12:00.9308610Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary"
2019-10-08T00:12:00.9308962Z ------------------------------------------
2019-10-08T00:12:00.9308998Z 
2019-10-08T00:12:00.9309236Z ------------------------------------------
2019-10-08T00:12:00.9309292Z stderr:
2019-10-08T00:12:00.9309292Z stderr:
2019-10-08T00:12:00.9309509Z ------------------------------------------
2019-10-08T00:12:00.9309902Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-08T00:12:00.9310183Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:29:1
2019-10-08T00:12:00.9310306Z LL | #[plugin_registrar]
2019-10-08T00:12:00.9310354Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-08T00:12:00.9310398Z    |
2019-10-08T00:12:00.9310460Z    = note: `#[warn(deprecated)]` on by default
2019-10-08T00:12:00.9310460Z    = note: `#[warn(deprecated)]` on by default
2019-10-08T00:12:00.9310491Z 
2019-10-08T00:12:00.9310533Z error[E0308]: mismatched types
2019-10-08T00:12:00.9310791Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:31:34
2019-10-08T00:12:00.9310856Z    |
2019-10-08T00:12:00.9310905Z LL |     reg.register_early_lint_pass(box Pass as EarlyLintPassObject);
2019-10-08T00:12:00.9310972Z    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected fn pointer, found struct `std::boxed::Box`
2019-10-08T00:12:00.9311300Z    |
2019-10-08T00:12:00.9311707Z    = note: expected type `fn() -> std::boxed::Box<(dyn rustc::lint::EarlyLintPass + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-08T00:12:00.9312046Z               found type `std::boxed::Box<(dyn rustc::lint::EarlyLintPass + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-08T00:12:00.9312321Z error: aborting due to previous error
2019-10-08T00:12:00.9312351Z 
2019-10-08T00:12:00.9312669Z For more information about this error, try `rustc --explain E0308`.
2019-10-08T00:12:00.9312706Z 
2019-10-08T00:12:00.9312706Z 
2019-10-08T00:12:00.9312923Z ------------------------------------------
2019-10-08T00:12:00.9312956Z 
2019-10-08T00:12:00.9312982Z 
2019-10-08T00:12:00.9313232Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-load.rs stdout ----
2019-10-08T00:12:00.9313268Z 
2019-10-08T00:12:00.9313550Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-10-08T00:12:00.9313622Z status: exit code: 1
2019-10-08T00:12:00.9314380Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary"
2019-10-08T00:12:00.9314717Z ------------------------------------------
2019-10-08T00:12:00.9314751Z 
2019-10-08T00:12:00.9314983Z ------------------------------------------
2019-10-08T00:12:00.9315129Z stderr:
2019-10-08T00:12:00.9315129Z stderr:
2019-10-08T00:12:00.9315396Z ------------------------------------------
2019-10-08T00:12:00.9315782Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-08T00:12:00.9316085Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:29:1
2019-10-08T00:12:00.9316216Z LL | #[plugin_registrar]
2019-10-08T00:12:00.9316267Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-08T00:12:00.9316315Z    |
2019-10-08T00:12:00.9316379Z    = note: `#[warn(deprecated)]` on by default
2019-10-08T00:12:00.9316379Z    = note: `#[warn(deprecated)]` on by default
2019-10-08T00:12:00.9316413Z 
2019-10-08T00:12:00.9316459Z error[E0308]: mismatched types
2019-10-08T00:12:00.9316734Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:31:34
2019-10-08T00:12:00.9317046Z    |
2019-10-08T00:12:00.9317118Z LL |     reg.register_early_lint_pass(box Pass as EarlyLintPassObject);
2019-10-08T00:12:00.9317192Z    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected fn pointer, found struct `std::boxed::Box`
2019-10-08T00:12:00.9317264Z    |
2019-10-08T00:12:00.9317678Z    = note: expected type `fn() -> std::boxed::Box<(dyn rustc::lint::EarlyLintPass + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-08T00:12:00.9318035Z               found type `std::boxed::Box<(dyn rustc::lint::EarlyLintPass + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-08T00:12:00.9318156Z error: aborting due to previous error
2019-10-08T00:12:00.9318187Z 
2019-10-08T00:12:00.9318471Z For more information about this error, try `rustc --explain E0308`.
2019-10-08T00:12:00.9318509Z 
2019-10-08T00:12:00.9318509Z 
2019-10-08T00:12:00.9318744Z ------------------------------------------
2019-10-08T00:12:00.9318779Z 
2019-10-08T00:12:00.9318807Z 
2019-10-08T00:12:00.9319075Z ---- [ui] ui-fulldeps/lint-plugin-deny-attr.rs stdout ----
2019-10-08T00:12:00.9319112Z 
2019-10-08T00:12:00.9319412Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-10-08T00:12:00.9319488Z status: exit code: 1
2019-10-08T00:12:00.9320256Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary"
2019-10-08T00:12:00.9320806Z ------------------------------------------
2019-10-08T00:12:00.9320844Z 
2019-10-08T00:12:00.9321422Z ------------------------------------------
2019-10-08T00:12:00.9321495Z stderr:
2019-10-08T00:12:00.9321495Z stderr:
2019-10-08T00:12:00.9321726Z ------------------------------------------
2019-10-08T00:12:00.9322121Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-08T00:12:00.9322401Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:29:1
2019-10-08T00:12:00.9322528Z LL | #[plugin_registrar]
---
2019-10-08T00:12:00.9377852Z test result: FAILED. 54 passed; 15 failed; 0 ignored; 0 measured; 0 filtered out
2019-10-08T00:12:00.9377890Z 
2019-10-08T00:12:00.9377916Z 
2019-10-08T00:12:00.9377942Z 
2019-10-08T00:12:00.9379511Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-08T00:12:00.9379765Z 
2019-10-08T00:12:00.9379802Z 
2019-10-08T00:12:01.6685302Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-08T00:12:01.6685668Z Build completed unsuccessfully in 1:06:26
2019-10-08T00:12:01.6685668Z Build completed unsuccessfully in 1:06:26
2019-10-08T00:12:01.6685784Z == clock drift check ==
2019-10-08T00:12:01.6685861Z   local time: Tue Oct  8 00:12:00 UTC 2019
2019-10-08T00:12:01.6685949Z   network time: Tue, 08 Oct 2019 00:12:01 GMT
2019-10-08T00:12:01.6686018Z == end clock drift check ==
2019-10-08T00:12:02.1680253Z ##[error]Bash exited with code '1'.
2019-10-08T00:12:02.1724727Z ##[section]Starting: Checkout
2019-10-08T00:12:02.1726624Z ==============================================================================
2019-10-08T00:12:02.1726698Z Task         : Get sources
2019-10-08T00:12:02.1726746Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
