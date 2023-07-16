plain
2019-10-09T01:53:57.9398836Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-09T01:53:57.9585121Z ##[command]git config gc.auto 0
2019-10-09T01:53:57.9679134Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-09T01:53:57.9752368Z ##[command]git config --get-all http.proxy
2019-10-09T01:53:57.9935070Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65193/merge:refs/remotes/pull/65193/merge
---
2019-10-09T02:59:03.0809994Z .................................................................................................... 1600/9132
2019-10-09T02:59:11.5232479Z .................................................................................................... 1700/9132
2019-10-09T02:59:22.9855070Z ................i...............i................................................................... 1800/9132
2019-10-09T02:59:30.7265700Z .................................................................................................... 1900/9132
2019-10-09T02:59:48.3765853Z .......iiiii........................................................................................ 2000/9132
2019-10-09T02:59:58.8652489Z .................................................................................................... 2200/9132
2019-10-09T03:00:01.6829682Z .................................................................................................... 2300/9132
2019-10-09T03:00:07.9680363Z .................................................................................................... 2400/9132
2019-10-09T03:00:14.5690198Z .................................................................................................... 2500/9132
---
2019-10-09T03:03:18.5525241Z ...................................................................................................i 4700/9132
2019-10-09T03:03:26.2919897Z ...............i.................................................................................... 4800/9132
2019-10-09T03:03:38.4225158Z .................................................................................................... 4900/9132
2019-10-09T03:03:44.3559678Z .................................................................................................... 5000/9132
2019-10-09T03:03:56.5808079Z .............................................................................................ii.ii.. 5100/9132
2019-10-09T03:04:07.5975106Z .................................................................................................... 5300/9132
2019-10-09T03:04:18.0208282Z .................................................................................................... 5400/9132
2019-10-09T03:04:25.4143271Z ...........................................................i........................................ 5500/9132
2019-10-09T03:04:32.9688785Z .................................................................................................... 5600/9132
2019-10-09T03:04:32.9688785Z .................................................................................................... 5600/9132
2019-10-09T03:04:41.4497194Z .................................................................................................... 5700/9132
2019-10-09T03:04:53.4862702Z ........................................................ii...i..ii...........i...................... 5800/9132
2019-10-09T03:05:21.1113781Z .................................................................................................... 6000/9132
2019-10-09T03:05:28.3636888Z .................................................................................................... 6100/9132
2019-10-09T03:05:28.3636888Z .................................................................................................... 6100/9132
2019-10-09T03:05:35.6061870Z ..............................................................i..ii................................. 6200/9132
2019-10-09T03:06:05.9495197Z .................................................................................................... 6400/9132
2019-10-09T03:06:08.2310522Z ......................i............................................................................. 6500/9132
2019-10-09T03:06:10.5993176Z ...............................................................................................i.... 6600/9132
2019-10-09T03:06:13.5310225Z .................................................................................................... 6700/9132
---
2019-10-09T03:11:08.7382607Z  finished in 5.702
2019-10-09T03:11:08.7612831Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-09T03:11:08.9450671Z 
2019-10-09T03:11:08.9450987Z running 150 tests
2019-10-09T03:11:12.4645088Z i....iii.......iii.iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-10-09T03:11:14.5688718Z ..iiii..............i.........iii.i.......ii......
2019-10-09T03:11:14.5690034Z 
2019-10-09T03:11:14.5694053Z  finished in 5.808
2019-10-09T03:11:14.5883082Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-09T03:11:15.2855294Z 
---
2019-10-09T03:11:16.9442492Z  finished in 2.355
2019-10-09T03:11:16.9637260Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-09T03:11:17.1304845Z 
2019-10-09T03:11:17.1306135Z running 9 tests
2019-10-09T03:11:17.1307427Z iiiiiiiii
2019-10-09T03:11:17.1308132Z 
2019-10-09T03:11:17.1308266Z  finished in 0.166
2019-10-09T03:11:17.1502251Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-09T03:11:18.2815498Z 
---
2019-10-09T03:11:36.4367172Z  finished in 19.286
2019-10-09T03:11:36.4599752Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-09T03:11:36.6572568Z 
2019-10-09T03:11:36.6574161Z running 123 tests
2019-10-09T03:12:02.1637114Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-09T03:12:07.1181031Z i.i.i......iii.i.....ii
2019-10-09T03:12:07.1182603Z 
2019-10-09T03:12:07.1186700Z  finished in 30.658
2019-10-09T03:12:07.1197916Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-09T03:12:07.1198573Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-09T03:12:07.1198573Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-10-09T03:12:07.1425714Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-09T03:12:07.3118452Z 
2019-10-09T03:12:07.3118638Z running 69 tests
2019-10-09T03:13:09.9984520Z ................F.......F..F.....F.FFFFFFF.FFFF......................
2019-10-09T03:13:09.9984848Z 
2019-10-09T03:13:10.0016091Z ---- [ui] ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs stdout ----
2019-10-09T03:13:10.0016223Z diff of stderr:
2019-10-09T03:13:10.0016261Z 
2019-10-09T03:13:10.0016261Z 
2019-10-09T03:13:10.0016311Z 23    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2019-10-09T03:13:10.0016695Z 25 error: aborting due to 2 previous errors
2019-10-09T03:13:10.0016758Z + 
2019-10-09T03:13:10.0016815Z 26 
2019-10-09T03:13:10.0016843Z 
2019-10-09T03:13:10.0016843Z 
2019-10-09T03:13:10.0016869Z 
2019-10-09T03:13:10.0016917Z The actual stderr differed from the expected stderr.
2019-10-09T03:13:10.0017372Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/lint_pass_impl_without_macro.stderr
2019-10-09T03:13:10.0017665Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T03:13:10.0018351Z To only update this specific test, also pass `--test-args internal-lints/lint_pass_impl_without_macro.rs`
2019-10-09T03:13:10.0018466Z error: 1 errors occurred comparing output.
2019-10-09T03:13:10.0018512Z status: exit code: 1
2019-10-09T03:13:10.0018512Z status: exit code: 1
2019-10-09T03:13:10.0019423Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/auxiliary" "-A" "unused"
2019-10-09T03:13:10.0019973Z ------------------------------------------
2019-10-09T03:13:10.0020008Z 
2019-10-09T03:13:10.0020233Z ------------------------------------------
2019-10-09T03:13:10.0020301Z stderr:
2019-10-09T03:13:10.0020301Z stderr:
2019-10-09T03:13:10.0020528Z ------------------------------------------
2019-10-09T03:13:10.0020577Z error: implementing `LintPass` by hand
2019-10-09T03:13:10.0020861Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:19:6
2019-10-09T03:13:10.0020936Z    |
2019-10-09T03:13:10.0020988Z LL | impl LintPass for Foo { //~ERROR implementing `LintPass` by hand
2019-10-09T03:13:10.0021101Z    |
2019-10-09T03:13:10.0021146Z note: lint level defined here
2019-10-09T03:13:10.0021428Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:4:9
2019-10-09T03:13:10.0021507Z    |
2019-10-09T03:13:10.0021507Z    |
2019-10-09T03:13:10.0021552Z LL | #![deny(rustc::lint_pass_impl_without_macro)]
2019-10-09T03:13:10.0021601Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-10-09T03:13:10.0021672Z    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2019-10-09T03:13:10.0021748Z error: implementing `LintPass` by hand
2019-10-09T03:13:10.0022030Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:29:14
2019-10-09T03:13:10.0022099Z    |
2019-10-09T03:13:10.0022099Z    |
2019-10-09T03:13:10.0022155Z LL |         impl LintPass for Custom { //~ERROR implementing `LintPass` by hand
2019-10-09T03:13:10.0022267Z ...
2019-10-09T03:13:10.0022267Z ...
2019-10-09T03:13:10.0022310Z LL | custom_lint_pass_macro!();
2019-10-09T03:13:10.0022620Z    |
2019-10-09T03:13:10.0022620Z    |
2019-10-09T03:13:10.0022669Z    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2019-10-09T03:13:10.0022752Z error: aborting due to 2 previous errors
2019-10-09T03:13:10.0022800Z 
2019-10-09T03:13:10.0022826Z 
2019-10-09T03:13:10.0023051Z ------------------------------------------
2019-10-09T03:13:10.0023051Z ------------------------------------------
2019-10-09T03:13:10.0023082Z 
2019-10-09T03:13:10.0023108Z 
2019-10-09T03:13:10.0023366Z ---- [ui] ui-fulldeps/issue-15778-fail.rs stdout ----
2019-10-09T03:13:10.0023399Z 
2019-10-09T03:13:10.0023682Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" failed to compile: 
2019-10-09T03:13:10.0023755Z status: exit code: 1
2019-10-09T03:13:10.0024897Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary"
2019-10-09T03:13:10.0025666Z ------------------------------------------
2019-10-09T03:13:10.0025705Z 
2019-10-09T03:13:10.0025956Z ------------------------------------------
2019-10-09T03:13:10.0026003Z stderr:
2019-10-09T03:13:10.0026003Z stderr:
2019-10-09T03:13:10.0026228Z ------------------------------------------
2019-10-09T03:13:10.0026280Z warning: unused import: `LateLintPassObject`
2019-10-09T03:13:10.0026572Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:10:69
2019-10-09T03:13:10.0026623Z    |
2019-10-09T03:13:10.0026678Z LL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LateLintPassObject, LintArray};
2019-10-09T03:13:10.0026816Z    |
2019-10-09T03:13:10.0026973Z    = note: `#[warn(unused_imports)]` on by default
2019-10-09T03:13:10.0027023Z 
2019-10-09T03:13:10.0027023Z 
2019-10-09T03:13:10.0027529Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-09T03:13:10.0028353Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:33:1
2019-10-09T03:13:10.0028654Z LL | #[plugin_registrar]
2019-10-09T03:13:10.0028701Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-09T03:13:10.0028768Z    |
2019-10-09T03:13:10.0028813Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T03:13:10.0028813Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T03:13:10.0028856Z 
2019-10-09T03:13:10.0028918Z error[E0308]: mismatched types
2019-10-09T03:13:10.0029221Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:35:33
2019-10-09T03:13:10.0029270Z    |
2019-10-09T03:13:10.0029314Z LL |     reg.register_late_lint_pass(box Pass);
2019-10-09T03:13:10.0029387Z    |                                 ^^^^^^^^ expected fn pointer, found struct `std::boxed::Box`
2019-10-09T03:13:10.0029444Z    |
2019-10-09T03:13:10.0029821Z    = note: expected type `fn() -> std::boxed::Box<(dyn for<'a, 'tcx> rustc::lint::LateLintPass<'a, 'tcx> + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-09T03:13:10.0029912Z               found type `std::boxed::Box<Pass>`
2019-10-09T03:13:10.0029989Z error: aborting due to previous error
2019-10-09T03:13:10.0030037Z 
2019-10-09T03:13:10.0030298Z For more information about this error, try `rustc --explain E0308`.
2019-10-09T03:13:10.0030334Z 
2019-10-09T03:13:10.0030334Z 
2019-10-09T03:13:10.0030566Z ------------------------------------------
2019-10-09T03:13:10.0030617Z 
2019-10-09T03:13:10.0030643Z 
2019-10-09T03:13:10.0030883Z ---- [ui] ui-fulldeps/issue-15778-pass.rs stdout ----
2019-10-09T03:13:10.0030916Z 
2019-10-09T03:13:10.0031234Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" failed to compile: 
2019-10-09T03:13:10.0031298Z status: exit code: 1
2019-10-09T03:13:10.0032189Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary"
2019-10-09T03:13:10.0032566Z ------------------------------------------
2019-10-09T03:13:10.0032623Z 
2019-10-09T03:13:10.0032853Z ------------------------------------------
2019-10-09T03:13:10.0032900Z stderr:
2019-10-09T03:13:10.0032900Z stderr:
2019-10-09T03:13:10.0033126Z ------------------------------------------
2019-10-09T03:13:10.0033198Z error[E0407]: method `get_lints` is not a member of trait `LintPass`
2019-10-09T03:13:10.0033477Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:25:13
2019-10-09T03:13:10.0033529Z    |
2019-10-09T03:13:10.0033785Z LL | /             fn get_lints(&self) -> LintArray {
2019-10-09T03:13:10.0033835Z LL | |                 $lints
2019-10-09T03:13:10.0033944Z    | |_____________^ not a member of trait `LintPass`
2019-10-09T03:13:10.0033988Z ...
2019-10-09T03:13:10.0033988Z ...
2019-10-09T03:13:10.0034030Z LL | / fake_lint_pass! {
2019-10-09T03:13:10.0034092Z LL | |     PassOkay,
2019-10-09T03:13:10.0034149Z LL | |     lint_array!(CRATE_NOT_OKAY), // Single lint
2019-10-09T03:13:10.0034199Z LL | |     Symbol::intern("rustc_crate_okay")
2019-10-09T03:13:10.0034855Z    | |_- in this macro invocation
2019-10-09T03:13:10.0034894Z 
2019-10-09T03:13:10.0034944Z error[E0407]: method `get_lints` is not a member of trait `LintPass`
2019-10-09T03:13:10.0035974Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:25:13
2019-10-09T03:13:10.0035974Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:25:13
2019-10-09T03:13:10.0036028Z    |
2019-10-09T03:13:10.0036270Z LL | /             fn get_lints(&self) -> LintArray {
2019-10-09T03:13:10.0036339Z LL | |                 $lints
2019-10-09T03:13:10.0036429Z    | |_____________^ not a member of trait `LintPass`
2019-10-09T03:13:10.0036473Z ...
2019-10-09T03:13:10.0036473Z ...
2019-10-09T03:13:10.0036535Z LL | / fake_lint_pass! {
2019-10-09T03:13:10.0036577Z LL | |     PassRedBlue,
2019-10-09T03:13:10.0036628Z LL | |     lint_array!(CRATE_NOT_RED, CRATE_NOT_BLUE), // Multiple lints
2019-10-09T03:13:10.0036714Z LL | |     Symbol::intern("rustc_crate_red"), Symbol::intern("rustc_crate_blue")
2019-10-09T03:13:10.0036984Z    | |_- in this macro invocation
2019-10-09T03:13:10.0037016Z 
2019-10-09T03:13:10.0037083Z error[E0407]: method `get_lints` is not a member of trait `LintPass`
2019-10-09T03:13:10.0037346Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:25:13
2019-10-09T03:13:10.0037346Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:25:13
2019-10-09T03:13:10.0037405Z    |
2019-10-09T03:13:10.0037655Z LL | /             fn get_lints(&self) -> LintArray {
2019-10-09T03:13:10.0037705Z LL | |                 $lints
2019-10-09T03:13:10.0037815Z    | |_____________^ not a member of trait `LintPass`
2019-10-09T03:13:10.0037858Z ...
2019-10-09T03:13:10.0037858Z ...
2019-10-09T03:13:10.0037901Z LL | / fake_lint_pass! {
2019-10-09T03:13:10.0037963Z LL | |     PassGreyGreen,
2019-10-09T03:13:10.0038220Z LL | |     lint_array!(CRATE_NOT_GREY, CRATE_NOT_GREEN, ), // Trailing comma
2019-10-09T03:13:10.0038296Z LL | |     Symbol::intern("rustc_crate_grey"), Symbol::intern("rustc_crate_green")
2019-10-09T03:13:10.0038687Z    | |_- in this macro invocation
2019-10-09T03:13:10.0038721Z 
2019-10-09T03:13:10.0038721Z 
2019-10-09T03:13:10.0038766Z warning: unused import: `LateLintPassObject`
2019-10-09T03:13:10.0039048Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:10:69
2019-10-09T03:13:10.0039106Z    |
2019-10-09T03:13:10.0039159Z LL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LateLintPassObject, LintArray};
2019-10-09T03:13:10.0039296Z    |
2019-10-09T03:13:10.0039342Z    = note: `#[warn(unused_imports)]` on by default
2019-10-09T03:13:10.0039372Z 
2019-10-09T03:13:10.0039372Z 
2019-10-09T03:13:10.0039760Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-09T03:13:10.0040182Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:68:1
2019-10-09T03:13:10.0040303Z LL | #[plugin_registrar]
2019-10-09T03:13:10.0040350Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-09T03:13:10.0040412Z    |
2019-10-09T03:13:10.0040457Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T03:13:10.0040457Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T03:13:10.0040488Z 
2019-10-09T03:13:10.0040537Z error[E0308]: mismatched types
2019-10-09T03:13:10.0040845Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:70:33
2019-10-09T03:13:10.0040894Z    |
2019-10-09T03:13:10.0040940Z LL |     reg.register_late_lint_pass(box PassOkay);
2019-10-09T03:13:10.0041014Z    |                                 ^^^^^^^^^^^^ expected fn pointer, found struct `std::boxed::Box`
2019-10-09T03:13:10.0041062Z    |
2019-10-09T03:13:10.0041421Z    = note: expected type `fn() -> std::boxed::Box<(dyn for<'a, 'tcx> rustc::lint::LateLintPass<'a, 'tcx> + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-09T03:13:10.0041514Z               found type `std::boxed::Box<PassOkay>`
2019-10-09T03:13:10.0041592Z error[E0308]: mismatched types
2019-10-09T03:13:10.0041871Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:71:33
2019-10-09T03:13:10.0041920Z    |
2019-10-09T03:13:10.0041920Z    |
2019-10-09T03:13:10.0041965Z LL |     reg.register_late_lint_pass(box PassRedBlue);
2019-10-09T03:13:10.0042114Z    |                                 ^^^^^^^^^^^^^^^ expected fn pointer, found struct `std::boxed::Box`
2019-10-09T03:13:10.0042179Z    |
2019-10-09T03:13:10.0042558Z    = note: expected type `fn() -> std::boxed::Box<(dyn for<'a, 'tcx> rustc::lint::LateLintPass<'a, 'tcx> + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-09T03:13:10.0042640Z               found type `std::boxed::Box<PassRedBlue>`
2019-10-09T03:13:10.0042714Z error[E0308]: mismatched types
2019-10-09T03:13:10.0042979Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:72:33
2019-10-09T03:13:10.0043047Z    |
2019-10-09T03:13:10.0043047Z    |
2019-10-09T03:13:10.0043093Z LL |     reg.register_late_lint_pass(box PassGreyGreen);
2019-10-09T03:13:10.0043148Z    |                                 ^^^^^^^^^^^^^^^^^ expected fn pointer, found struct `std::boxed::Box`
2019-10-09T03:13:10.0043215Z    |
2019-10-09T03:13:10.0043574Z    = note: expected type `fn() -> std::boxed::Box<(dyn for<'a, 'tcx> rustc::lint::LateLintPass<'a, 'tcx> + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-09T03:13:10.0043646Z               found type `std::boxed::Box<PassGreyGreen>`
2019-10-09T03:13:10.0048856Z error: aborting due to 6 previous errors
2019-10-09T03:13:10.0048928Z 
2019-10-09T03:13:10.0048984Z Some errors have detailed explanations: E0308, E0407.
2019-10-09T03:13:10.0049491Z For more information about an error, try `rustc --explain E0308`.
2019-10-09T03:13:10.0049491Z For more information about an error, try `rustc --explain E0308`.
2019-10-09T03:13:10.0049528Z 
2019-10-09T03:13:10.0049775Z ------------------------------------------
2019-10-09T03:13:10.0049812Z 
2019-10-09T03:13:10.0049842Z 
2019-10-09T03:13:10.0050101Z ---- [ui] ui-fulldeps/issue-40001.rs stdout ----
2019-10-09T03:13:10.0050135Z 
2019-10-09T03:13:10.0050433Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
2019-10-09T03:13:10.0050507Z status: exit code: 1
2019-10-09T03:13:10.0051293Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
2019-10-09T03:13:10.0051854Z ------------------------------------------
2019-10-09T03:13:10.0051891Z 
2019-10-09T03:13:10.0052140Z ------------------------------------------
2019-10-09T03:13:10.0052188Z stderr:
2019-10-09T03:13:10.0052188Z stderr:
2019-10-09T03:13:10.0052408Z ------------------------------------------
2019-10-09T03:13:10.0052459Z warning: unused import: `syntax::ext::base::*`
2019-10-09T03:13:10.0052752Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:11:5
2019-10-09T03:13:10.0052851Z LL | use syntax::ext::base::*;
2019-10-09T03:13:10.0052916Z    |     ^^^^^^^^^^^^^^^^^^^^
2019-10-09T03:13:10.0052958Z    |
2019-10-09T03:13:10.0053003Z    = note: `#[warn(unused_imports)]` on by default
2019-10-09T03:13:10.0053003Z    = note: `#[warn(unused_imports)]` on by default
2019-10-09T03:13:10.0053054Z 
2019-10-09T03:13:10.0053099Z warning: unused import: `rustc::hir::map as hir_map`
2019-10-09T03:13:10.0053364Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:17:5
2019-10-09T03:13:10.0053413Z    |
2019-10-09T03:13:10.0053495Z LL | use rustc::hir::map as hir_map;
2019-10-09T03:13:10.0053822Z 
2019-10-09T03:13:10.0053884Z warning: unused import: `rustc::ty`
2019-10-09T03:13:10.0053884Z warning: unused import: `rustc::ty`
2019-10-09T03:13:10.0054188Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:20:5
2019-10-09T03:13:10.0054279Z LL | use rustc::ty;
2019-10-09T03:13:10.0054928Z    |     ^^^^^^^^^
2019-10-09T03:13:10.0054960Z 
2019-10-09T03:13:10.0055003Z warning: unused import: `ast`
2019-10-09T03:13:10.0055003Z warning: unused import: `ast`
2019-10-09T03:13:10.0055660Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:21:14
2019-10-09T03:13:10.0055720Z    |
2019-10-09T03:13:10.0055764Z LL | use syntax::{ast, source_map};
2019-10-09T03:13:10.0055857Z 
2019-10-09T03:13:10.0055857Z 
2019-10-09T03:13:10.0056272Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-09T03:13:10.0056590Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:23:1
2019-10-09T03:13:10.0056686Z LL | #[plugin_registrar]
2019-10-09T03:13:10.0056733Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-09T03:13:10.0056797Z    |
2019-10-09T03:13:10.0056842Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T03:13:10.0056842Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T03:13:10.0056873Z 
2019-10-09T03:13:10.0056933Z error[E0308]: mismatched types
2019-10-09T03:13:10.0057208Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:25:33
2019-10-09T03:13:10.0057258Z    |
2019-10-09T03:13:10.0057324Z LL |     reg.register_late_lint_pass(box MissingWhitelistedAttrPass);
2019-10-09T03:13:10.0057384Z    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected fn pointer, found struct `std::boxed::Box`
2019-10-09T03:13:10.0057484Z    |
2019-10-09T03:13:10.0058089Z    = note: expected type `fn() -> std::boxed::Box<(dyn for<'a, 'tcx> rustc::lint::LateLintPass<'a, 'tcx> + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-09T03:13:10.0058177Z               found type `std::boxed::Box<MissingWhitelistedAttrPass>`
2019-10-09T03:13:10.0058277Z error: aborting due to previous error
2019-10-09T03:13:10.0058307Z 
2019-10-09T03:13:10.0058597Z For more information about this error, try `rustc --explain E0308`.
2019-10-09T03:13:10.0058632Z 
2019-10-09T03:13:10.0058632Z 
2019-10-09T03:13:10.0058888Z ------------------------------------------
2019-10-09T03:13:10.0058920Z 
2019-10-09T03:13:10.0058945Z 
2019-10-09T03:13:10.0059194Z ---- [ui] ui-fulldeps/lint-group-plugin-deny-cmdline.rs stdout ----
2019-10-09T03:13:10.0059227Z 
2019-10-09T03:13:10.0059544Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
2019-10-09T03:13:10.0059598Z status: exit code: 1
2019-10-09T03:13:10.0060586Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary"
2019-10-09T03:13:10.0064887Z ------------------------------------------
2019-10-09T03:13:10.0064958Z 
2019-10-09T03:13:10.0065272Z ------------------------------------------
2019-10-09T03:13:10.0065322Z stderr:
2019-10-09T03:13:10.0065322Z stderr:
2019-10-09T03:13:10.0065573Z ------------------------------------------
2019-10-09T03:13:10.0065625Z warning: unused import: `LateLintPassObject`
2019-10-09T03:13:10.0065898Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:12:69
2019-10-09T03:13:10.0065969Z    |
2019-10-09T03:13:10.0066042Z LL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LateLintPassObject, LintArray};
2019-10-09T03:13:10.0066168Z    |
2019-10-09T03:13:10.0066214Z    = note: `#[warn(unused_imports)]` on by default
2019-10-09T03:13:10.0066246Z 
2019-10-09T03:13:10.0066246Z 
2019-10-09T03:13:10.0066874Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-09T03:13:10.0067165Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:31:1
2019-10-09T03:13:10.0067276Z LL | #[plugin_registrar]
2019-10-09T03:13:10.0067324Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-09T03:13:10.0067368Z    |
2019-10-09T03:13:10.0067432Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T03:13:10.0067432Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T03:13:10.0067464Z 
2019-10-09T03:13:10.0067506Z error[E0308]: mismatched types
2019-10-09T03:13:10.0068187Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:33:33
2019-10-09T03:13:10.0068265Z    |
2019-10-09T03:13:10.0068310Z LL |     reg.register_late_lint_pass(box Pass);
2019-10-09T03:13:10.0068365Z    |                                 ^^^^^^^^ expected fn pointer, found struct `std::boxed::Box`
2019-10-09T03:13:10.0068430Z    |
2019-10-09T03:13:10.0068825Z    = note: expected type `fn() -> std::boxed::Box<(dyn for<'a, 'tcx> rustc::lint::LateLintPass<'a, 'tcx> + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-09T03:13:10.0079117Z               found type `std::boxed::Box<Pass>`
2019-10-09T03:13:10.0079243Z error: aborting due to previous error
2019-10-09T03:13:10.0079274Z 
2019-10-09T03:13:10.0079748Z For more information about this error, try `rustc --explain E0308`.
2019-10-09T03:13:10.0079813Z 
2019-10-09T03:13:10.0079813Z 
2019-10-09T03:13:10.0080051Z ------------------------------------------
2019-10-09T03:13:10.0080085Z 
2019-10-09T03:13:10.0080111Z 
2019-10-09T03:13:10.0080619Z ---- [ui] ui-fulldeps/lint-group-plugin.rs stdout ----
2019-10-09T03:13:10.0080661Z 
2019-10-09T03:13:10.0080966Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
2019-10-09T03:13:10.0081023Z status: exit code: 1
2019-10-09T03:13:10.0081831Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary"
2019-10-09T03:13:10.0082372Z ------------------------------------------
2019-10-09T03:13:10.0082421Z 
2019-10-09T03:13:10.0082683Z ------------------------------------------
2019-10-09T03:13:10.0082751Z stderr:
2019-10-09T03:13:10.0082751Z stderr:
2019-10-09T03:13:10.0082978Z ------------------------------------------
2019-10-09T03:13:10.0083030Z warning: unused import: `LateLintPassObject`
2019-10-09T03:13:10.0083315Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:12:69
2019-10-09T03:13:10.0083376Z    |
2019-10-09T03:13:10.0083432Z LL | use rustc::lint::{LateContext, LintContext, LintPass, LateLintPass, LateLintPassObject, LintArray};
2019-10-09T03:13:10.0083561Z    |
2019-10-09T03:13:10.0083606Z    = note: `#[warn(unused_imports)]` on by default
2019-10-09T03:13:10.0083655Z 
2019-10-09T03:13:10.0083655Z 
2019-10-09T03:13:10.0084043Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-09T03:13:10.0084338Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:31:1
2019-10-09T03:13:10.0084838Z LL | #[plugin_registrar]
2019-10-09T03:13:10.0084888Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-09T03:13:10.0084953Z    |
2019-10-09T03:13:10.0084998Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T03:13:10.0084998Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T03:13:10.0085164Z 
2019-10-09T03:13:10.0085207Z error[E0308]: mismatched types
2019-10-09T03:13:10.0085562Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:33:33
2019-10-09T03:13:10.0085612Z    |
2019-10-09T03:13:10.0085657Z LL |     reg.register_late_lint_pass(box Pass);
2019-10-09T03:13:10.0085732Z    |                                 ^^^^^^^^ expected fn pointer, found struct `std::boxed::Box`
2019-10-09T03:13:10.0085780Z    |
2019-10-09T03:13:10.0086160Z    = note: expected type `fn() -> std::boxed::Box<(dyn for<'a, 'tcx> rustc::lint::LateLintPass<'a, 'tcx> + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-09T03:13:10.0086248Z               found type `std::boxed::Box<Pass>`
2019-10-09T03:13:10.0086325Z error: aborting due to previous error
2019-10-09T03:13:10.0086373Z 
2019-10-09T03:13:10.0086630Z For more information about this error, try `rustc --explain E0308`.
2019-10-09T03:13:10.0086666Z 
2019-10-09T03:13:10.0086666Z 
2019-10-09T03:13:10.0086897Z ------------------------------------------
2019-10-09T03:13:10.0086947Z 
2019-10-09T03:13:10.0086973Z 
2019-10-09T03:13:10.0087386Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-allow.rs stdout ----
2019-10-09T03:13:10.0087423Z 
2019-10-09T03:13:10.0087733Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-10-09T03:13:10.0087989Z status: exit code: 1
2019-10-09T03:13:10.0088884Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary"
2019-10-09T03:13:10.0089243Z ------------------------------------------
2019-10-09T03:13:10.0089279Z 
2019-10-09T03:13:10.0089525Z ------------------------------------------
2019-10-09T03:13:10.0089571Z stderr:
2019-10-09T03:13:10.0089571Z stderr:
2019-10-09T03:13:10.0089792Z ------------------------------------------
2019-10-09T03:13:10.0090183Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-09T03:13:10.0090460Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:29:1
2019-10-09T03:13:10.0090693Z LL | #[plugin_registrar]
2019-10-09T03:13:10.0090752Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-09T03:13:10.0090815Z    |
2019-10-09T03:13:10.0090861Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T03:13:10.0090861Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T03:13:10.0090892Z 
2019-10-09T03:13:10.0090934Z error[E0308]: mismatched types
2019-10-09T03:13:10.0091246Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:31:34
2019-10-09T03:13:10.0091304Z    |
2019-10-09T03:13:10.0091352Z LL |     reg.register_early_lint_pass(box Pass as EarlyLintPassObject);
2019-10-09T03:13:10.0091431Z    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected fn pointer, found struct `std::boxed::Box`
2019-10-09T03:13:10.0091479Z    |
2019-10-09T03:13:10.0091833Z    = note: expected type `fn() -> std::boxed::Box<(dyn rustc::lint::EarlyLintPass + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-09T03:13:10.0092346Z               found type `std::boxed::Box<(dyn rustc::lint::EarlyLintPass + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-09T03:13:10.0092436Z error: aborting due to previous error
2019-10-09T03:13:10.0092483Z 
2019-10-09T03:13:10.0092736Z For more information about this error, try `rustc --explain E0308`.
2019-10-09T03:13:10.0092771Z 
2019-10-09T03:13:10.0092771Z 
2019-10-09T03:13:10.0093105Z ------------------------------------------
2019-10-09T03:13:10.0093155Z 
2019-10-09T03:13:10.0093182Z 
2019-10-09T03:13:10.0093426Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-load.rs stdout ----
2019-10-09T03:13:10.0093460Z 
2019-10-09T03:13:10.0095373Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-10-09T03:13:10.0095462Z status: exit code: 1
2019-10-09T03:13:10.0096381Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary"
2019-10-09T03:13:10.0096740Z ------------------------------------------
2019-10-09T03:13:10.0096774Z 
2019-10-09T03:13:10.0097021Z ------------------------------------------
2019-10-09T03:13:10.0097068Z stderr:
2019-10-09T03:13:10.0097068Z stderr:
2019-10-09T03:13:10.0097376Z ------------------------------------------
2019-10-09T03:13:10.0097763Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-09T03:13:10.0098385Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:29:1
2019-10-09T03:13:10.0098632Z LL | #[plugin_registrar]
2019-10-09T03:13:10.0098679Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-09T03:13:10.0098741Z    |
2019-10-09T03:13:10.0098787Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T03:13:10.0098787Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T03:13:10.0098818Z 
2019-10-09T03:13:10.0098860Z error[E0308]: mismatched types
2019-10-09T03:13:10.0099162Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:31:34
2019-10-09T03:13:10.0099221Z    |
2019-10-09T03:13:10.0099273Z LL |     reg.register_early_lint_pass(box Pass as EarlyLintPassObject);
2019-10-09T03:13:10.0099350Z    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected fn pointer, found struct `std::boxed::Box`
2019-10-09T03:13:10.0099400Z    |
2019-10-09T03:13:10.0099746Z    = note: expected type `fn() -> std::boxed::Box<(dyn rustc::lint::EarlyLintPass + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-09T03:13:10.0100303Z               found type `std::boxed::Box<(dyn rustc::lint::EarlyLintPass + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-09T03:13:10.0100406Z error: aborting due to previous error
2019-10-09T03:13:10.0100455Z 
2019-10-09T03:13:10.0100740Z For more information about this error, try `rustc --explain E0308`.
2019-10-09T03:13:10.0100775Z 
2019-10-09T03:13:10.0100775Z 
2019-10-09T03:13:10.0101507Z ------------------------------------------
2019-10-09T03:13:10.0101567Z 
2019-10-09T03:13:10.0101594Z 
2019-10-09T03:13:10.0101932Z ---- [ui] ui-fulldeps/lint-plugin-deny-attr.rs stdout ----
2019-10-09T03:13:10.0101965Z 
2019-10-09T03:13:10.0102248Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-10-09T03:13:10.0102321Z status: exit code: 1
2019-10-09T03:13:10.0103117Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary"
2019-10-09T03:13:10.0103628Z ------------------------------------------
2019-10-09T03:13:10.0103774Z 
2019-10-09T03:13:10.0104020Z ------------------------------------------
2019-10-09T03:13:10.0104066Z stderr:
2019-10-09T03:13:10.0104066Z stderr:
2019-10-09T03:13:10.0104339Z ------------------------------------------
2019-10-09T03:13:10.0105217Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-09T03:13:10.0105500Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:29:1
2019-10-09T03:13:10.0105626Z LL | #[plugin_registrar]
2019-10-09T03:13:10.0105675Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-09T03:13:10.0105738Z    |
2019-10-09T03:13:10.0105783Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T03:13:10.0105783Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T03:13:10.0105813Z 
2019-10-09T03:13:10.0105855Z error[E0308]: mismatched types
2019-10-09T03:13:10.0106134Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:31:34
2019-10-09T03:13:10.0106408Z    |
2019-10-09T03:13:10.0106458Z LL |     reg.register_early_lint_pass(box Pass as EarlyLintPassObject);
2019-10-09T03:13:10.0106537Z    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected fn pointer, found struct `std::boxed::Box`
2019-10-09T03:13:10.0106584Z    |
2019-10-09T03:13:10.0106968Z    = note: expected type `fn() -> std::boxed::Box<(dyn rustc::lint::EarlyLintPass + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-09T03:13:10.0107439Z               found type `std::boxed::Box<(dyn rustc::lint::EarlyLintPass + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-09T03:13:10.0107530Z error: aborting due to previous error
2019-10-09T03:13:10.0107578Z 
2019-10-09T03:13:10.0108248Z For more information about this error, try `rustc --explain E0308`.
2019-10-09T03:13:10.0108297Z 
2019-10-09T03:13:10.0108297Z 
2019-10-09T03:13:10.0108563Z ------------------------------------------
2019-10-09T03:13:10.0108596Z 
2019-10-09T03:13:10.0108641Z 
2019-10-09T03:13:10.0108884Z ---- [ui] ui-fulldeps/lint-plugin-deny-cmdline.rs stdout ----
2019-10-09T03:13:10.0108919Z 
2019-10-09T03:13:10.0109204Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-10-09T03:13:10.0109276Z status: exit code: 1
2019-10-09T03:13:10.0110215Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary"
2019-10-09T03:13:10.0110606Z ------------------------------------------
2019-10-09T03:13:10.0110641Z 
2019-10-09T03:13:10.0110885Z ------------------------------------------
2019-10-09T03:13:10.0110932Z stderr:
2019-10-09T03:13:10.0110932Z stderr:
2019-10-09T03:13:10.0111151Z ------------------------------------------
2019-10-09T03:13:10.0111532Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-09T03:13:10.0111807Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:29:1
2019-10-09T03:13:10.0111930Z LL | #[plugin_registrar]
2019-10-09T03:13:10.0111977Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-09T03:13:10.0112039Z    |
2019-10-09T03:13:10.0112083Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T03:13:10.0112083Z    = note: `#[warn(deprecated)]` on by default
2019-10-09T03:13:10.0112113Z 
2019-10-09T03:13:10.0112156Z error[E0308]: mismatched types
2019-10-09T03:13:10.0112436Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:31:34
2019-10-09T03:13:10.0112588Z    |
2019-10-09T03:13:10.0112635Z LL |     reg.register_early_lint_pass(box Pass as EarlyLintPassObject);
2019-10-09T03:13:10.0112711Z    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected fn pointer, found struct `std::boxed::Box`
2019-10-09T03:13:10.0112764Z    |
2019-10-09T03:13:10.0113131Z    = note: expected type `fn() -> std::boxed::Box<(dyn rustc::lint::EarlyLintPass + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
2019-10-09T03:13:10.0113510Z               found type `std::boxed::Box<(dyn rustc::lint::EarlyLintPass + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
---
2019-10-09T03:13:10.0181388Z test result: FAILED. 54 passed; 15 failed; 0 ignored; 0 measured; 0 filtered out
2019-10-09T03:13:10.0181425Z 
2019-10-09T03:13:10.0181451Z 
2019-10-09T03:13:10.0181494Z 
2019-10-09T03:13:10.0183009Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-09T03:13:10.0183283Z 
2019-10-09T03:13:10.0183311Z 
2019-10-09T03:13:10.0183463Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-09T03:13:10.0183523Z Build completed unsuccessfully in 1:11:39
2019-10-09T03:13:10.0183523Z Build completed unsuccessfully in 1:11:39
2019-10-09T03:13:10.0183844Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-09T03:13:10.0183905Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-09T03:13:10.0187330Z == clock drift check ==
2019-10-09T03:13:10.0188518Z   local time: Wed Oct  9 03:13:10 UTC 2019
2019-10-09T03:13:10.2877852Z   network time: Wed, 09 Oct 2019 03:13:10 GMT
2019-10-09T03:13:10.8125215Z == end clock drift check ==
2019-10-09T03:13:10.8994907Z ##[error]Bash exited with code '1'.
2019-10-09T03:13:10.9057112Z ##[section]Starting: Checkout
2019-10-09T03:13:10.9059162Z ==============================================================================
2019-10-09T03:13:10.9059220Z Task         : Get sources
2019-10-09T03:13:10.9059287Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
