plain
2019-12-01T15:38:21.6026545Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-01T15:38:21.6209249Z ##[command]git config gc.auto 0
2019-12-01T15:38:21.6299769Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-01T15:38:21.6352222Z ##[command]git config --get-all http.proxy
2019-12-01T15:38:22.2313591Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66878/merge:refs/remotes/pull/66878/merge
---
2019-12-01T16:39:08.6967840Z .................................................................................................... 1600/9316
2019-12-01T16:39:13.4242401Z .................................................................................................... 1700/9316
2019-12-01T16:39:26.6739762Z ........................................i........................................................... 1800/9316
2019-12-01T16:39:34.6323256Z .................................................................................................... 1900/9316
2019-12-01T16:39:48.4996112Z .........................iiiii...................................................................... 2000/9316
2019-12-01T16:39:58.6649594Z .................................................................................................... 2200/9316
2019-12-01T16:40:01.1884609Z .................................................................................................... 2300/9316
2019-12-01T16:40:05.7081926Z .................................................................................................... 2400/9316
2019-12-01T16:40:27.2524222Z .................................................................................................... 2500/9316
---
2019-12-01T16:43:07.7555960Z ...........................i...............i........................................................ 4800/9316
2019-12-01T16:43:18.3131993Z .................................................................................................... 4900/9316
2019-12-01T16:43:24.3054555Z .................................................................................................... 5000/9316
2019-12-01T16:43:32.4234166Z .................................................................................................... 5100/9316
2019-12-01T16:43:40.0132639Z .................................ii.ii...........i.................................................. 5200/9316
2019-12-01T16:43:49.6586223Z .................................................................................................... 5400/9316
2019-12-01T16:43:59.5116032Z .................................................................................................... 5500/9316
2019-12-01T16:44:06.8359758Z ...............i.................................................................................... 5600/9316
2019-12-01T16:44:12.9518790Z .................................................................................................... 5700/9316
2019-12-01T16:44:12.9518790Z .................................................................................................... 5700/9316
2019-12-01T16:44:24.4289836Z .................................................................................................... 5800/9316
2019-12-01T16:44:36.5877974Z .ii...i..ii...........i............................................................................. 5900/9316
2019-12-01T16:44:55.2408921Z .................................................................................................... 6100/9316
2019-12-01T16:44:59.9691286Z .................................................................................................... 6200/9316
2019-12-01T16:44:59.9691286Z .................................................................................................... 6200/9316
2019-12-01T16:45:13.7526387Z ........................i..ii....................................................................... 6300/9316
2019-12-01T16:45:33.7705486Z ...............................................................................................i.... 6500/9316
2019-12-01T16:45:36.0916504Z .................................................................................................... 6600/9316
2019-12-01T16:45:38.3610701Z ......................................................................................i............. 6700/9316
2019-12-01T16:45:41.1008599Z .................................................................................................... 6800/9316
---
2019-12-01T16:50:55.0274393Z  finished in 6.198
2019-12-01T16:50:55.0460695Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T16:50:55.2959419Z 
2019-12-01T16:50:55.2960900Z running 164 tests
2019-12-01T16:50:58.2544505Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/164
2019-12-01T16:51:00.2780013Z .i.i..iiii..iiiiiii............i.........iii.i..........ii......
2019-12-01T16:51:00.2785714Z 
2019-12-01T16:51:00.2785798Z  finished in 5.232
2019-12-01T16:51:00.2990285Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T16:51:00.4681942Z 
---
2019-12-01T16:51:02.4345734Z  finished in 2.135
2019-12-01T16:51:02.4537231Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T16:51:02.6130398Z 
2019-12-01T16:51:02.6130517Z running 9 tests
2019-12-01T16:51:02.6131586Z iiiiiiiii
2019-12-01T16:51:02.6133497Z 
2019-12-01T16:51:02.6136571Z  finished in 0.159
2019-12-01T16:51:02.6344337Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T16:51:02.8163367Z 
---
2019-12-01T16:51:22.6188907Z  finished in 19.984
2019-12-01T16:51:22.6420911Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T16:51:22.8237917Z 
2019-12-01T16:51:22.8238177Z running 124 tests
2019-12-01T16:51:48.4011108Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i....ii...i.......ii 100/124
2019-12-01T16:51:53.6263324Z .i.i.i......iii.i.....ii
2019-12-01T16:51:53.6267002Z 
2019-12-01T16:51:53.6275575Z  finished in 30.984
2019-12-01T16:51:53.6277187Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T16:51:53.6300787Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-12-01T16:51:53.6300787Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-12-01T16:51:53.6540673Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T16:51:53.8533698Z 
2019-12-01T16:51:53.8535880Z running 67 tests
2019-12-01T16:52:32.5272873Z .................F.....F.F.......F.FFFFFFFFFFF.....................
2019-12-01T16:52:32.5291638Z thread 'main' panicked at 'Some tests failed
2019-12-01T16:52:32.5292555Z ---- [ui] ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs stdout ----
2019-12-01T16:52:32.5292980Z diff of stderr:
2019-12-01T16:52:32.5293239Z 
2019-12-01T16:52:32.5293239Z 
2019-12-01T16:52:32.5293519Z + error[E0432]: unresolved import `rustc::declare_lint`
2019-12-01T16:52:32.5294078Z +   --> $DIR/lint_pass_impl_without_macro.rs:9:13
2019-12-01T16:52:32.5294483Z +    |
2019-12-01T16:52:32.5294772Z + LL | use rustc::{declare_lint, declare_lint_pass, impl_lint_pass};
2019-12-01T16:52:32.5295024Z +    |             ^^^^^^^^^^^^ no `declare_lint` in the root
2019-12-01T16:52:32.5295536Z + error: cannot determine resolution for the macro `declare_lint`
2019-12-01T16:52:32.5296739Z +   --> $DIR/lint_pass_impl_without_macro.rs:11:1
2019-12-01T16:52:32.5297034Z +    |
2019-12-01T16:52:32.5297210Z + LL | declare_lint! {
2019-12-01T16:52:32.5297210Z + LL | declare_lint! {
2019-12-01T16:52:32.5297585Z +    | ^^^^^^^^^^^^
2019-12-01T16:52:32.5297776Z +    |
2019-12-01T16:52:32.5297952Z +    = note: import resolution is stuck, try simplifying macro imports
2019-12-01T16:52:32.5298116Z + 
2019-12-01T16:52:32.5298594Z + error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5299147Z +   --> $DIR/lint_pass_impl_without_macro.rs:41:25
2019-12-01T16:52:32.5299379Z +    |
2019-12-01T16:52:32.5299551Z + LL | impl_lint_pass!(Bar => [TEST_LINT]);
2019-12-01T16:52:32.5300295Z + 
2019-12-01T16:52:32.5300459Z + error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5300952Z +   --> $DIR/lint_pass_impl_without_macro.rs:43:28
2019-12-01T16:52:32.5301190Z +    |
2019-12-01T16:52:32.5301190Z +    |
2019-12-01T16:52:32.5301364Z + LL | declare_lint_pass!(Baz => [TEST_LINT]);
2019-12-01T16:52:32.5301726Z + 
2019-12-01T16:52:32.5301887Z 1 error: implementing `LintPass` by hand
2019-12-01T16:52:32.5302314Z 2   --> $DIR/lint_pass_impl_without_macro.rs:19:6
2019-12-01T16:52:32.5302539Z 3    |
2019-12-01T16:52:32.5302539Z 3    |
2019-12-01T16:52:32.5302685Z 
2019-12-01T16:52:32.5302842Z 22    |
2019-12-01T16:52:32.5303023Z 23    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2019-12-01T16:52:32.5303591Z - error: aborting due to 2 previous errors
2019-12-01T16:52:32.5303820Z + error: aborting due to 6 previous errors
2019-12-01T16:52:32.5303983Z 26 
2019-12-01T16:52:32.5304146Z + Some errors have detailed explanations: E0425, E0432.
2019-12-01T16:52:32.5304146Z + Some errors have detailed explanations: E0425, E0432.
2019-12-01T16:52:32.5304637Z + For more information about an error, try `rustc --explain E0425`.
2019-12-01T16:52:32.5304843Z 27 
2019-12-01T16:52:32.5304984Z 
2019-12-01T16:52:32.5305142Z 
2019-12-01T16:52:32.5305306Z The actual stderr differed from the expected stderr.
2019-12-01T16:52:32.5305855Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/lint_pass_impl_without_macro.stderr
2019-12-01T16:52:32.5306592Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T16:52:32.5335478Z To only update this specific test, also pass `--test-args internal-lints/lint_pass_impl_without_macro.rs`
2019-12-01T16:52:32.5336034Z error: 1 errors occurred comparing output.
2019-12-01T16:52:32.5336210Z status: exit code: 1
2019-12-01T16:52:32.5336210Z status: exit code: 1
2019-12-01T16:52:32.5337573Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/auxiliary" "-A" "unused"
2019-12-01T16:52:32.5338589Z ------------------------------------------
2019-12-01T16:52:32.5338822Z 
2019-12-01T16:52:32.5339256Z ------------------------------------------
2019-12-01T16:52:32.5339467Z stderr:
2019-12-01T16:52:32.5339467Z stderr:
2019-12-01T16:52:32.5340229Z ------------------------------------------
2019-12-01T16:52:32.5340488Z error[E0432]: unresolved import `rustc::declare_lint`
2019-12-01T16:52:32.5340958Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:9:13
2019-12-01T16:52:32.5341168Z    |
2019-12-01T16:52:32.5341364Z LL | use rustc::{declare_lint, declare_lint_pass, impl_lint_pass};
2019-12-01T16:52:32.5341534Z    |             ^^^^^^^^^^^^ no `declare_lint` in the root
2019-12-01T16:52:32.5341858Z error: cannot determine resolution for the macro `declare_lint`
2019-12-01T16:52:32.5342559Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:11:1
2019-12-01T16:52:32.5342768Z    |
2019-12-01T16:52:32.5342950Z LL | declare_lint! {
2019-12-01T16:52:32.5342950Z LL | declare_lint! {
2019-12-01T16:52:32.5343113Z    | ^^^^^^^^^^^^
2019-12-01T16:52:32.5343268Z    |
2019-12-01T16:52:32.5343467Z    = note: import resolution is stuck, try simplifying macro imports
2019-12-01T16:52:32.5343616Z 
2019-12-01T16:52:32.5343779Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5344295Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:41:25
2019-12-01T16:52:32.5344502Z    |
2019-12-01T16:52:32.5344685Z LL | impl_lint_pass!(Bar => [TEST_LINT]);
2019-12-01T16:52:32.5345016Z 
2019-12-01T16:52:32.5345177Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5345651Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:43:28
2019-12-01T16:52:32.5345870Z    |
2019-12-01T16:52:32.5345870Z    |
2019-12-01T16:52:32.5346060Z LL | declare_lint_pass!(Baz => [TEST_LINT]);
2019-12-01T16:52:32.5346378Z 
2019-12-01T16:52:32.5346558Z error: implementing `LintPass` by hand
2019-12-01T16:52:32.5347014Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:19:6
2019-12-01T16:52:32.5347219Z    |
2019-12-01T16:52:32.5347219Z    |
2019-12-01T16:52:32.5347423Z LL | impl LintPass for Foo { //~ERROR implementing `LintPass` by hand
2019-12-01T16:52:32.5347744Z    |
2019-12-01T16:52:32.5347920Z note: lint level defined here
2019-12-01T16:52:32.5348500Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:4:9
2019-12-01T16:52:32.5348716Z    |
2019-12-01T16:52:32.5348716Z    |
2019-12-01T16:52:32.5348901Z LL | #![deny(rustc::lint_pass_impl_without_macro)]
2019-12-01T16:52:32.5349067Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-01T16:52:32.5349260Z    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2019-12-01T16:52:32.5354724Z error: implementing `LintPass` by hand
2019-12-01T16:52:32.5355379Z   --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:29:14
2019-12-01T16:52:32.5355602Z    |
2019-12-01T16:52:32.5355602Z    |
2019-12-01T16:52:32.5359384Z LL |         impl LintPass for Custom { //~ERROR implementing `LintPass` by hand
2019-12-01T16:52:32.5360336Z ...
2019-12-01T16:52:32.5360336Z ...
2019-12-01T16:52:32.5360505Z LL | custom_lint_pass_macro!();
2019-12-01T16:52:32.5361296Z    |
2019-12-01T16:52:32.5361296Z    |
2019-12-01T16:52:32.5361466Z    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
2019-12-01T16:52:32.5361795Z error: aborting due to 6 previous errors
2019-12-01T16:52:32.5361936Z 
2019-12-01T16:52:32.5362104Z Some errors have detailed explanations: E0425, E0432.
2019-12-01T16:52:32.5362846Z For more information about an error, try `rustc --explain E0425`.
2019-12-01T16:52:32.5362846Z For more information about an error, try `rustc --explain E0425`.
2019-12-01T16:52:32.5363091Z 
2019-12-01T16:52:32.5363632Z ------------------------------------------
2019-12-01T16:52:32.5365047Z 
2019-12-01T16:52:32.5365107Z 
2019-12-01T16:52:32.5365512Z ---- [ui] ui-fulldeps/issue-15778-fail.rs stdout ----
2019-12-01T16:52:32.5373700Z 
2019-12-01T16:52:32.5374328Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" failed to compile: 
2019-12-01T16:52:32.5377437Z status: exit code: 1
2019-12-01T16:52:32.5378998Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary"
2019-12-01T16:52:32.5394627Z ------------------------------------------
2019-12-01T16:52:32.5394698Z 
2019-12-01T16:52:32.5395010Z ------------------------------------------
2019-12-01T16:52:32.5395068Z stderr:
---
2019-12-01T16:52:32.5395886Z 
2019-12-01T16:52:32.5395938Z error[E0425]: cannot find value `CRATE_NOT_OKAY` in this scope
2019-12-01T16:52:32.5399094Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:22:29
2019-12-01T16:52:32.5399195Z    |
2019-12-01T16:52:32.5399258Z LL | declare_lint_pass!(Pass => [CRATE_NOT_OKAY]);
2019-12-01T16:52:32.5399736Z 
2019-12-01T16:52:32.5399791Z error[E0425]: cannot find value `CRATE_NOT_OKAY` in this scope
2019-12-01T16:52:32.5400199Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:27:26
2019-12-01T16:52:32.5400277Z    |
2019-12-01T16:52:32.5400277Z    |
2019-12-01T16:52:32.5400329Z LL |             cx.span_lint(CRATE_NOT_OKAY, krate.span,
2019-12-01T16:52:32.5400438Z 
2019-12-01T16:52:32.5400489Z error[E0425]: cannot find value `CRATE_NOT_OKAY` in this scope
2019-12-01T16:52:32.5400781Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:35:38
2019-12-01T16:52:32.5400850Z    |
2019-12-01T16:52:32.5400850Z    |
2019-12-01T16:52:32.5400903Z LL |     reg.lint_store.register_lints(&[&CRATE_NOT_OKAY]);
2019-12-01T16:52:32.5401018Z 
2019-12-01T16:52:32.5401018Z 
2019-12-01T16:52:32.5401537Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T16:52:32.5401847Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:33:1
2019-12-01T16:52:32.5401968Z LL | #[plugin_registrar]
2019-12-01T16:52:32.5402023Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T16:52:32.5402089Z    |
2019-12-01T16:52:32.5402138Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T16:52:32.5402897Z 
2019-12-01T16:52:32.5402925Z 
2019-12-01T16:52:32.5403360Z ---- [ui] ui-fulldeps/issue-15778-pass.rs stdout ----
2019-12-01T16:52:32.5403647Z 
2019-12-01T16:52:32.5404113Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" failed to compile: 
2019-12-01T16:52:32.5404198Z status: exit code: 1
2019-12-01T16:52:32.5414521Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary"
2019-12-01T16:52:32.5415200Z ------------------------------------------
2019-12-01T16:52:32.5415253Z 
2019-12-01T16:52:32.5415536Z ------------------------------------------
2019-12-01T16:52:32.5415589Z stderr:
2019-12-01T16:52:32.5415589Z stderr:
2019-12-01T16:52:32.5416102Z ------------------------------------------
2019-12-01T16:52:32.5416194Z error: cannot find macro `declare_lint` in this scope
2019-12-01T16:52:32.5416540Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:40:1
2019-12-01T16:52:32.5416599Z    |
2019-12-01T16:52:32.5416678Z LL | declare_lint!(CRATE_NOT_OKAY, Warn, "crate not marked with #![crate_okay]");
2019-12-01T16:52:32.5416766Z 
2019-12-01T16:52:32.5416832Z error: cannot find macro `declare_lint` in this scope
2019-12-01T16:52:32.5417126Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:41:1
2019-12-01T16:52:32.5417181Z    |
2019-12-01T16:52:32.5417181Z    |
2019-12-01T16:52:32.5417236Z LL | declare_lint!(CRATE_NOT_RED, Warn, "crate not marked with #![crate_red]");
2019-12-01T16:52:32.5417347Z 
2019-12-01T16:52:32.5417405Z error: cannot find macro `declare_lint` in this scope
2019-12-01T16:52:32.5417711Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:42:1
2019-12-01T16:52:32.5417765Z    |
2019-12-01T16:52:32.5417765Z    |
2019-12-01T16:52:32.5417820Z LL | declare_lint!(CRATE_NOT_BLUE, Warn, "crate not marked with #![crate_blue]");
2019-12-01T16:52:32.5417918Z 
2019-12-01T16:52:32.5417966Z error: cannot find macro `declare_lint` in this scope
2019-12-01T16:52:32.5418499Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:43:1
2019-12-01T16:52:32.5418578Z    |
2019-12-01T16:52:32.5418578Z    |
2019-12-01T16:52:32.5418631Z LL | declare_lint!(CRATE_NOT_GREY, Warn, "crate not marked with #![crate_grey]");
2019-12-01T16:52:32.5418714Z 
2019-12-01T16:52:32.5418779Z error: cannot find macro `declare_lint` in this scope
2019-12-01T16:52:32.5419097Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:44:1
2019-12-01T16:52:32.5419161Z    |
2019-12-01T16:52:32.5419161Z    |
2019-12-01T16:52:32.5419241Z LL | declare_lint!(CRATE_NOT_GREEN, Warn, "crate not marked with #![crate_green]");
2019-12-01T16:52:32.5419324Z 
2019-12-01T16:52:32.5419374Z error[E0425]: cannot find value `CRATE_NOT_OKAY` in this scope
2019-12-01T16:52:32.5420089Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:30:38
2019-12-01T16:52:32.5420154Z    |
2019-12-01T16:52:32.5420154Z    |
2019-12-01T16:52:32.5420206Z LL |                           cx.span_lint(CRATE_NOT_OKAY, krate.span,
2019-12-01T16:52:32.5420336Z ...
2019-12-01T16:52:32.5420336Z ...
2019-12-01T16:52:32.5420384Z LL | / fake_lint_pass! {
2019-12-01T16:52:32.5420447Z LL | |     PassOkay,
2019-12-01T16:52:32.5420497Z LL | |     Symbol::intern("rustc_crate_okay")
2019-12-01T16:52:32.5420823Z    | |_- in this macro invocation
2019-12-01T16:52:32.5420874Z 
2019-12-01T16:52:32.5421274Z error[E0425]: cannot find value `CRATE_NOT_OKAY` in this scope
2019-12-01T16:52:32.5421680Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:30:38
2019-12-01T16:52:32.5421680Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:30:38
2019-12-01T16:52:32.5421759Z    |
2019-12-01T16:52:32.5421811Z LL |                           cx.span_lint(CRATE_NOT_OKAY, krate.span,
2019-12-01T16:52:32.5421934Z ...
2019-12-01T16:52:32.5421934Z ...
2019-12-01T16:52:32.5421982Z LL | / fake_lint_pass! {
2019-12-01T16:52:32.5422029Z LL | |     PassRedBlue,
2019-12-01T16:52:32.5422099Z LL | |     Symbol::intern("rustc_crate_red"), Symbol::intern("rustc_crate_blue")
2019-12-01T16:52:32.5422391Z    | |_- in this macro invocation
2019-12-01T16:52:32.5422428Z 
2019-12-01T16:52:32.5422497Z error[E0425]: cannot find value `CRATE_NOT_OKAY` in this scope
2019-12-01T16:52:32.5422786Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:30:38
2019-12-01T16:52:32.5422786Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:30:38
2019-12-01T16:52:32.5422965Z    |
2019-12-01T16:52:32.5423044Z LL |                           cx.span_lint(CRATE_NOT_OKAY, krate.span,
2019-12-01T16:52:32.5423151Z ...
2019-12-01T16:52:32.5423151Z ...
2019-12-01T16:52:32.5423212Z LL | / fake_lint_pass! {
2019-12-01T16:52:32.5423259Z LL | |     PassGreyGreen,
2019-12-01T16:52:32.5423313Z LL | |     Symbol::intern("rustc_crate_grey"), Symbol::intern("rustc_crate_green")
2019-12-01T16:52:32.5423656Z    | |_- in this macro invocation
2019-12-01T16:52:32.5423694Z 
2019-12-01T16:52:32.5423744Z error[E0425]: cannot find value `CRATE_NOT_OKAY` in this scope
2019-12-01T16:52:32.5424053Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:64:10
2019-12-01T16:52:32.5424053Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:64:10
2019-12-01T16:52:32.5424108Z    |
2019-12-01T16:52:32.5424154Z LL |         &CRATE_NOT_OKAY,
2019-12-01T16:52:32.5424264Z 
2019-12-01T16:52:32.5424321Z error[E0425]: cannot find value `CRATE_NOT_RED` in this scope
2019-12-01T16:52:32.5424617Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:65:10
2019-12-01T16:52:32.5424688Z    |
2019-12-01T16:52:32.5424688Z    |
2019-12-01T16:52:32.5424735Z LL |         &CRATE_NOT_RED,
2019-12-01T16:52:32.5424786Z    |          ^^^^^^^^^^^^^ not found in this scope
2019-12-01T16:52:32.5424870Z 
2019-12-01T16:52:32.5424938Z error[E0425]: cannot find value `CRATE_NOT_BLUE` in this scope
2019-12-01T16:52:32.5425231Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:66:10
2019-12-01T16:52:32.5425283Z    |
2019-12-01T16:52:32.5425348Z LL |         &CRATE_NOT_BLUE,
2019-12-01T16:52:32.5425434Z 
2019-12-01T16:52:32.5425434Z 
2019-12-01T16:52:32.5425483Z error[E0425]: cannot find value `CRATE_NOT_GREY` in this scope
2019-12-01T16:52:32.5425791Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:67:10
2019-12-01T16:52:32.5425853Z    |
2019-12-01T16:52:32.5425907Z LL |         &CRATE_NOT_GREY,
2019-12-01T16:52:32.5426012Z 
2019-12-01T16:52:32.5426062Z error[E0425]: cannot find value `CRATE_NOT_GREEN` in this scope
2019-12-01T16:52:32.5426372Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:68:10
2019-12-01T16:52:32.5426425Z    |
2019-12-01T16:52:32.5426425Z    |
2019-12-01T16:52:32.5426470Z LL |         &CRATE_NOT_GREEN,
2019-12-01T16:52:32.5426571Z 
2019-12-01T16:52:32.5426619Z warning: unused `#[macro_use]` import
2019-12-01T16:52:32.5426901Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:6:1
2019-12-01T16:52:32.5427143Z    |
2019-12-01T16:52:32.5427143Z    |
2019-12-01T16:52:32.5427206Z LL | #[macro_use] extern crate rustc;
2019-12-01T16:52:32.5427254Z    | ^^^^^^^^^^^^
2019-12-01T16:52:32.5427297Z    |
2019-12-01T16:52:32.5427365Z    = note: `#[warn(unused_imports)]` on by default
2019-12-01T16:52:32.5427409Z 
2019-12-01T16:52:32.5428421Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T16:52:32.5428852Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:61:1
2019-12-01T16:52:32.5428983Z LL | #[plugin_registrar]
2019-12-01T16:52:32.5429099Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T16:52:32.5429149Z    |
2019-12-01T16:52:32.5429198Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T16:52:32.5429980Z 
2019-12-01T16:52:32.5430008Z 
2019-12-01T16:52:32.5430408Z ---- [ui] ui-fulldeps/issue-40001.rs stdout ----
2019-12-01T16:52:32.5430456Z 
2019-12-01T16:52:32.5430778Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
2019-12-01T16:52:32.5430838Z status: exit code: 1
2019-12-01T16:52:32.5431729Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
2019-12-01T16:52:32.5432095Z ------------------------------------------
2019-12-01T16:52:32.5432142Z 
2019-12-01T16:52:32.5432395Z ------------------------------------------
2019-12-01T16:52:32.5432463Z stderr:
2019-12-01T16:52:32.5432463Z stderr:
2019-12-01T16:52:32.5432701Z ------------------------------------------
2019-12-01T16:52:32.5432757Z error: cannot find macro `declare_lint` in this scope
2019-12-01T16:52:32.5433055Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:21:1
2019-12-01T16:52:32.5433157Z LL | declare_lint! {
2019-12-01T16:52:32.5433222Z    | ^^^^^^^^^^^^
2019-12-01T16:52:32.5433253Z 
2019-12-01T16:52:32.5433305Z error[E0425]: cannot find value `MISSING_WHITELISTED_ATTR` in this scope
2019-12-01T16:52:32.5433305Z error[E0425]: cannot find value `MISSING_WHITELISTED_ATTR` in this scope
2019-12-01T16:52:32.5433592Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:17:38
2019-12-01T16:52:32.5433662Z    |
2019-12-01T16:52:32.5433713Z LL |     reg.lint_store.register_lints(&[&MISSING_WHITELISTED_ATTR]);
2019-12-01T16:52:32.5433835Z 
2019-12-01T16:52:32.5433895Z error[E0425]: cannot find value `MISSING_WHITELISTED_ATTR` in this scope
2019-12-01T16:52:32.5433895Z error[E0425]: cannot find value `MISSING_WHITELISTED_ATTR` in this scope
2019-12-01T16:52:32.5434181Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:27:51
2019-12-01T16:52:32.5434235Z    |
2019-12-01T16:52:32.5434307Z LL | declare_lint_pass!(MissingWhitelistedAttrPass => [MISSING_WHITELISTED_ATTR]);
2019-12-01T16:52:32.5434411Z 
2019-12-01T16:52:32.5434480Z error[E0425]: cannot find value `MISSING_WHITELISTED_ATTR` in this scope
2019-12-01T16:52:32.5434480Z error[E0425]: cannot find value `MISSING_WHITELISTED_ATTR` in this scope
2019-12-01T16:52:32.5434985Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:45:26
2019-12-01T16:52:32.5435045Z    |
2019-12-01T16:52:32.5435116Z LL |             cx.span_lint(MISSING_WHITELISTED_ATTR, span,
2019-12-01T16:52:32.5435210Z 
2019-12-01T16:52:32.5435210Z 
2019-12-01T16:52:32.5435800Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T16:52:32.5436182Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:15:1
2019-12-01T16:52:32.5436308Z LL | #[plugin_registrar]
2019-12-01T16:52:32.5436365Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T16:52:32.5436417Z    |
2019-12-01T16:52:32.5436486Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T16:52:32.5437273Z 
2019-12-01T16:52:32.5437303Z 
2019-12-01T16:52:32.5437613Z ---- [ui] ui-fulldeps/lint-group-plugin-deny-cmdline.rs stdout ----
2019-12-01T16:52:32.5437746Z 
2019-12-01T16:52:32.5438123Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
2019-12-01T16:52:32.5438204Z status: exit code: 1
2019-12-01T16:52:32.5439145Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary"
2019-12-01T16:52:32.5439538Z ------------------------------------------
2019-12-01T16:52:32.5439617Z 
2019-12-01T16:52:32.5439912Z ------------------------------------------
2019-12-01T16:52:32.5439994Z stderr:
2019-12-01T16:52:32.5439994Z stderr:
2019-12-01T16:52:32.5440237Z ------------------------------------------
2019-12-01T16:52:32.5440312Z error: cannot find macro `declare_lint` in this scope
2019-12-01T16:52:32.5440598Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:15:1
2019-12-01T16:52:32.5440654Z    |
2019-12-01T16:52:32.5440946Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T16:52:32.5441034Z 
2019-12-01T16:52:32.5441082Z error: cannot find macro `declare_lint` in this scope
2019-12-01T16:52:32.5441383Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:17:1
2019-12-01T16:52:32.5441438Z    |
2019-12-01T16:52:32.5441438Z    |
2019-12-01T16:52:32.5441717Z LL | declare_lint!(PLEASE_LINT, Warn, "Warn about items named 'pleaselintme'");
2019-12-01T16:52:32.5441825Z 
2019-12-01T16:52:32.5441875Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5442180Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:19:29
2019-12-01T16:52:32.5442251Z    |
2019-12-01T16:52:32.5442251Z    |
2019-12-01T16:52:32.5442301Z LL | declare_lint_pass!(Pass => [TEST_LINT, PLEASE_LINT]);
2019-12-01T16:52:32.5442408Z 
2019-12-01T16:52:32.5442458Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T16:52:32.5442746Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:19:40
2019-12-01T16:52:32.5442799Z    |
2019-12-01T16:52:32.5442799Z    |
2019-12-01T16:52:32.5442866Z LL | declare_lint_pass!(Pass => [TEST_LINT, PLEASE_LINT]);
2019-12-01T16:52:32.5442957Z 
2019-12-01T16:52:32.5443024Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5443309Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:24:38
2019-12-01T16:52:32.5443369Z    |
2019-12-01T16:52:32.5443369Z    |
2019-12-01T16:52:32.5443769Z LL |             "lintme" => cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'"),
2019-12-01T16:52:32.5443877Z 
2019-12-01T16:52:32.5443926Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T16:52:32.5444916Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:25:44
2019-12-01T16:52:32.5444988Z    |
2019-12-01T16:52:32.5444988Z    |
2019-12-01T16:52:32.5445326Z LL |             "pleaselintme" => cx.span_lint(PLEASE_LINT, it.span, "item is named 'pleaselintme'"),
2019-12-01T16:52:32.5445460Z 
2019-12-01T16:52:32.5445509Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5445820Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:33:38
2019-12-01T16:52:32.5446013Z    |
2019-12-01T16:52:32.5446013Z    |
2019-12-01T16:52:32.5446072Z LL |     reg.lint_store.register_lints(&[&TEST_LINT, &PLEASE_LINT]);
2019-12-01T16:52:32.5446185Z 
2019-12-01T16:52:32.5446233Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T16:52:32.5446554Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:33:50
2019-12-01T16:52:32.5446626Z    |
2019-12-01T16:52:32.5446626Z    |
2019-12-01T16:52:32.5446677Z LL |     reg.lint_store.register_lints(&[&TEST_LINT, &PLEASE_LINT]);
2019-12-01T16:52:32.5446789Z 
2019-12-01T16:52:32.5446838Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5447125Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:36:26
2019-12-01T16:52:32.5447176Z    |
2019-12-01T16:52:32.5447176Z    |
2019-12-01T16:52:32.5447246Z LL |         vec![LintId::of(&TEST_LINT), LintId::of(&PLEASE_LINT)]);
2019-12-01T16:52:32.5447353Z 
2019-12-01T16:52:32.5447419Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T16:52:32.5447708Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:36:50
2019-12-01T16:52:32.5447762Z    |
2019-12-01T16:52:32.5447762Z    |
2019-12-01T16:52:32.5447831Z LL |         vec![LintId::of(&TEST_LINT), LintId::of(&PLEASE_LINT)]);
2019-12-01T16:52:32.5447928Z 
2019-12-01T16:52:32.5447928Z 
2019-12-01T16:52:32.5448358Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T16:52:32.5448695Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:31:1
2019-12-01T16:52:32.5448818Z LL | #[plugin_registrar]
2019-12-01T16:52:32.5448876Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T16:52:32.5448949Z    |
2019-12-01T16:52:32.5449018Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T16:52:32.5449912Z 
2019-12-01T16:52:32.5449940Z 
2019-12-01T16:52:32.5450214Z ---- [ui] ui-fulldeps/lint-group-plugin.rs stdout ----
2019-12-01T16:52:32.5450251Z 
2019-12-01T16:52:32.5450565Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
2019-12-01T16:52:32.5450624Z status: exit code: 1
2019-12-01T16:52:32.5451622Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary"
2019-12-01T16:52:32.5452037Z ------------------------------------------
2019-12-01T16:52:32.5452077Z 
2019-12-01T16:52:32.5452321Z ------------------------------------------
2019-12-01T16:52:32.5452390Z stderr:
2019-12-01T16:52:32.5452390Z stderr:
2019-12-01T16:52:32.5452626Z ------------------------------------------
2019-12-01T16:52:32.5452681Z error: cannot find macro `declare_lint` in this scope
2019-12-01T16:52:32.5453012Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:15:1
2019-12-01T16:52:32.5453066Z    |
2019-12-01T16:52:32.5453467Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T16:52:32.5453576Z 
2019-12-01T16:52:32.5453625Z error: cannot find macro `declare_lint` in this scope
2019-12-01T16:52:32.5453930Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:17:1
2019-12-01T16:52:32.5453983Z    |
2019-12-01T16:52:32.5453983Z    |
2019-12-01T16:52:32.5454262Z LL | declare_lint!(PLEASE_LINT, Warn, "Warn about items named 'pleaselintme'");
2019-12-01T16:52:32.5454366Z 
2019-12-01T16:52:32.5454416Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5454700Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:19:29
2019-12-01T16:52:32.5454770Z    |
2019-12-01T16:52:32.5454770Z    |
2019-12-01T16:52:32.5454820Z LL | declare_lint_pass!(Pass => [TEST_LINT, PLEASE_LINT]);
2019-12-01T16:52:32.5454918Z 
2019-12-01T16:52:32.5454992Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T16:52:32.5455283Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:19:40
2019-12-01T16:52:32.5455335Z    |
2019-12-01T16:52:32.5455335Z    |
2019-12-01T16:52:32.5455402Z LL | declare_lint_pass!(Pass => [TEST_LINT, PLEASE_LINT]);
2019-12-01T16:52:32.5455494Z 
2019-12-01T16:52:32.5455542Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5455845Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:24:38
2019-12-01T16:52:32.5455898Z    |
2019-12-01T16:52:32.5455898Z    |
2019-12-01T16:52:32.5456188Z LL |             "lintme" => cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'"),
2019-12-01T16:52:32.5456303Z 
2019-12-01T16:52:32.5456352Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T16:52:32.5456674Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:25:44
2019-12-01T16:52:32.5456728Z    |
2019-12-01T16:52:32.5456728Z    |
2019-12-01T16:52:32.5457035Z LL |             "pleaselintme" => cx.span_lint(PLEASE_LINT, it.span, "item is named 'pleaselintme'"),
2019-12-01T16:52:32.5457160Z 
2019-12-01T16:52:32.5457209Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5457494Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:33:38
2019-12-01T16:52:32.5457564Z    |
2019-12-01T16:52:32.5457564Z    |
2019-12-01T16:52:32.5457616Z LL |     reg.lint_store.register_lints(&[&TEST_LINT, &PLEASE_LINT]);
2019-12-01T16:52:32.5457725Z 
2019-12-01T16:52:32.5457774Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T16:52:32.5458060Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:33:50
2019-12-01T16:52:32.5458479Z    |
2019-12-01T16:52:32.5458479Z    |
2019-12-01T16:52:32.5458552Z LL |     reg.lint_store.register_lints(&[&TEST_LINT, &PLEASE_LINT]);
2019-12-01T16:52:32.5458647Z 
2019-12-01T16:52:32.5458716Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5459074Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:36:26
2019-12-01T16:52:32.5459127Z    |
2019-12-01T16:52:32.5459127Z    |
2019-12-01T16:52:32.5459196Z LL |         vec![LintId::of(&TEST_LINT), LintId::of(&PLEASE_LINT)]);
2019-12-01T16:52:32.5459287Z 
2019-12-01T16:52:32.5459337Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T16:52:32.5459688Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:36:50
2019-12-01T16:52:32.5459741Z    |
2019-12-01T16:52:32.5459741Z    |
2019-12-01T16:52:32.5459916Z LL |         vec![LintId::of(&TEST_LINT), LintId::of(&PLEASE_LINT)]);
2019-12-01T16:52:32.5460032Z 
2019-12-01T16:52:32.5460032Z 
2019-12-01T16:52:32.5460475Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T16:52:32.5460830Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:31:1
2019-12-01T16:52:32.5460935Z LL | #[plugin_registrar]
2019-12-01T16:52:32.5461010Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T16:52:32.5461062Z    |
2019-12-01T16:52:32.5461113Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T16:52:32.5461923Z 
2019-12-01T16:52:32.5461954Z 
2019-12-01T16:52:32.5462258Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-allow.rs stdout ----
2019-12-01T16:52:32.5462298Z 
2019-12-01T16:52:32.5462628Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T16:52:32.5462690Z status: exit code: 1
2019-12-01T16:52:32.5463636Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary"
2019-12-01T16:52:32.5464087Z ------------------------------------------
2019-12-01T16:52:32.5464125Z 
2019-12-01T16:52:32.5464390Z ------------------------------------------
2019-12-01T16:52:32.5464461Z stderr:
2019-12-01T16:52:32.5464461Z stderr:
2019-12-01T16:52:32.5464719Z ------------------------------------------
2019-12-01T16:52:32.5464779Z error: cannot find macro `declare_lint` in this scope
2019-12-01T16:52:32.5465098Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T16:52:32.5465157Z    |
2019-12-01T16:52:32.5465453Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T16:52:32.5465560Z 
2019-12-01T16:52:32.5465614Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5465916Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T16:52:32.5466000Z    |
2019-12-01T16:52:32.5466000Z    |
2019-12-01T16:52:32.5466053Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T16:52:32.5466268Z 
2019-12-01T16:52:32.5466319Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5466654Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T16:52:32.5466710Z    |
2019-12-01T16:52:32.5466710Z    |
2019-12-01T16:52:32.5467027Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T16:52:32.5467126Z 
2019-12-01T16:52:32.5467197Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5467501Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T16:52:32.5467556Z    |
2019-12-01T16:52:32.5467556Z    |
2019-12-01T16:52:32.5467625Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T16:52:32.5467805Z 
2019-12-01T16:52:32.5467805Z 
2019-12-01T16:52:32.5468274Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T16:52:32.5468599Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T16:52:32.5468725Z LL | #[plugin_registrar]
2019-12-01T16:52:32.5468781Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T16:52:32.5468832Z    |
2019-12-01T16:52:32.5468900Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T16:52:32.5469711Z 
2019-12-01T16:52:32.5469740Z 
2019-12-01T16:52:32.5470038Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-load.rs stdout ----
2019-12-01T16:52:32.5470084Z 
2019-12-01T16:52:32.5470396Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T16:52:32.5470455Z status: exit code: 1
2019-12-01T16:52:32.5471364Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary"
2019-12-01T16:52:32.5471735Z ------------------------------------------
2019-12-01T16:52:32.5471781Z 
2019-12-01T16:52:32.5472036Z ------------------------------------------
2019-12-01T16:52:32.5472105Z stderr:
2019-12-01T16:52:32.5472105Z stderr:
2019-12-01T16:52:32.5472343Z ------------------------------------------
2019-12-01T16:52:32.5472399Z error: cannot find macro `declare_lint` in this scope
2019-12-01T16:52:32.5472694Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T16:52:32.5472749Z    |
2019-12-01T16:52:32.5473021Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T16:52:32.5473126Z 
2019-12-01T16:52:32.5473176Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5473471Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T16:52:32.5473524Z    |
2019-12-01T16:52:32.5473524Z    |
2019-12-01T16:52:32.5473573Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T16:52:32.5473688Z 
2019-12-01T16:52:32.5473829Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5474144Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T16:52:32.5474216Z    |
2019-12-01T16:52:32.5474216Z    |
2019-12-01T16:52:32.5474493Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T16:52:32.5474589Z 
2019-12-01T16:52:32.5474657Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5474934Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T16:52:32.5474985Z    |
2019-12-01T16:52:32.5474985Z    |
2019-12-01T16:52:32.5475052Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T16:52:32.5475144Z 
2019-12-01T16:52:32.5475144Z 
2019-12-01T16:52:32.5475550Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T16:52:32.5475979Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T16:52:32.5476313Z LL | #[plugin_registrar]
2019-12-01T16:52:32.5476502Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T16:52:32.5476550Z    |
2019-12-01T16:52:32.5476624Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T16:52:32.5477517Z 
2019-12-01T16:52:32.5477542Z 
2019-12-01T16:52:32.5477796Z ---- [ui] ui-fulldeps/lint-plugin-deny-attr.rs stdout ----
2019-12-01T16:52:32.5477828Z 
2019-12-01T16:52:32.5478104Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T16:52:32.5478201Z status: exit code: 1
2019-12-01T16:52:32.5479105Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary"
2019-12-01T16:52:32.5480004Z ------------------------------------------
2019-12-01T16:52:32.5480047Z 
2019-12-01T16:52:32.5480291Z ------------------------------------------
2019-12-01T16:52:32.5480337Z stderr:
2019-12-01T16:52:32.5480337Z stderr:
2019-12-01T16:52:32.5480563Z ------------------------------------------
2019-12-01T16:52:32.5480639Z error: cannot find macro `declare_lint` in this scope
2019-12-01T16:52:32.5480895Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T16:52:32.5480944Z    |
2019-12-01T16:52:32.5481208Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T16:52:32.5481288Z 
2019-12-01T16:52:32.5481333Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5481600Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T16:52:32.5481647Z    |
2019-12-01T16:52:32.5481647Z    |
2019-12-01T16:52:32.5481691Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T16:52:32.5481788Z 
2019-12-01T16:52:32.5481832Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5482084Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T16:52:32.5482156Z    |
2019-12-01T16:52:32.5482156Z    |
2019-12-01T16:52:32.5482555Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T16:52:32.5482674Z 
2019-12-01T16:52:32.5482717Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5483005Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T16:52:32.5483054Z    |
2019-12-01T16:52:32.5483054Z    |
2019-12-01T16:52:32.5483222Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T16:52:32.5483419Z 
2019-12-01T16:52:32.5483419Z 
2019-12-01T16:52:32.5483812Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T16:52:32.5484116Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T16:52:32.5484331Z LL | #[plugin_registrar]
2019-12-01T16:52:32.5484390Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T16:52:32.5484437Z    |
2019-12-01T16:52:32.5484500Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T16:52:32.5485223Z 
2019-12-01T16:52:32.5485266Z 
2019-12-01T16:52:32.5485518Z ---- [ui] ui-fulldeps/lint-plugin-deny-cmdline.rs stdout ----
2019-12-01T16:52:32.5485552Z 
2019-12-01T16:52:32.5485852Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T16:52:32.5485925Z status: exit code: 1
2019-12-01T16:52:32.5486754Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary"
2019-12-01T16:52:32.5487115Z ------------------------------------------
2019-12-01T16:52:32.5487148Z 
2019-12-01T16:52:32.5487401Z ------------------------------------------
2019-12-01T16:52:32.5487447Z stderr:
2019-12-01T16:52:32.5487447Z stderr:
2019-12-01T16:52:32.5487676Z ------------------------------------------
2019-12-01T16:52:32.5487744Z error: cannot find macro `declare_lint` in this scope
2019-12-01T16:52:32.5488010Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T16:52:32.5488077Z    |
2019-12-01T16:52:32.5488364Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T16:52:32.5488445Z 
2019-12-01T16:52:32.5488491Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5488777Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T16:52:32.5488828Z    |
2019-12-01T16:52:32.5488828Z    |
2019-12-01T16:52:32.5488873Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T16:52:32.5488971Z 
2019-12-01T16:52:32.5489018Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5489305Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T16:52:32.5489794Z    |
2019-12-01T16:52:32.5489794Z    |
2019-12-01T16:52:32.5490127Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T16:52:32.5490364Z 
2019-12-01T16:52:32.5490408Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5490691Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T16:52:32.5490756Z    |
2019-12-01T16:52:32.5490756Z    |
2019-12-01T16:52:32.5490801Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T16:52:32.5490884Z 
2019-12-01T16:52:32.5490884Z 
2019-12-01T16:52:32.5491263Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T16:52:32.5491533Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T16:52:32.5491641Z LL | #[plugin_registrar]
2019-12-01T16:52:32.5491689Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T16:52:32.5491836Z    |
2019-12-01T16:52:32.5491879Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T16:52:32.5492573Z 
2019-12-01T16:52:32.5492599Z 
2019-12-01T16:52:32.5492941Z ---- [ui] ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----
2019-12-01T16:52:32.5492973Z 
2019-12-01T16:52:32.5493357Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T16:52:32.5493407Z status: exit code: 1
2019-12-01T16:52:32.5494298Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary"
2019-12-01T16:52:32.5494624Z ------------------------------------------
2019-12-01T16:52:32.5494672Z 
2019-12-01T16:52:32.5494999Z ------------------------------------------
2019-12-01T16:52:32.5495045Z stderr:
2019-12-01T16:52:32.5495045Z stderr:
2019-12-01T16:52:32.5495255Z ------------------------------------------
2019-12-01T16:52:32.5495322Z error: cannot find macro `declare_lint` in this scope
2019-12-01T16:52:32.5495570Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T16:52:32.5495619Z    |
2019-12-01T16:52:32.5495883Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T16:52:32.5495974Z 
2019-12-01T16:52:32.5496035Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5496286Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T16:52:32.5496333Z    |
2019-12-01T16:52:32.5496333Z    |
2019-12-01T16:52:32.5496393Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T16:52:32.5496474Z 
2019-12-01T16:52:32.5496517Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5496782Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T16:52:32.5496829Z    |
2019-12-01T16:52:32.5496829Z    |
2019-12-01T16:52:32.5497071Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T16:52:32.5497172Z 
2019-12-01T16:52:32.5497216Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5497561Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T16:52:32.5497632Z    |
2019-12-01T16:52:32.5497632Z    |
2019-12-01T16:52:32.5497675Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T16:52:32.5497774Z 
2019-12-01T16:52:32.5497774Z 
2019-12-01T16:52:32.5498149Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T16:52:32.5498475Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T16:52:32.5498583Z LL | #[plugin_registrar]
2019-12-01T16:52:32.5498631Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T16:52:32.5498690Z    |
2019-12-01T16:52:32.5498733Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T16:52:32.5500999Z 
2019-12-01T16:52:32.5501025Z 
2019-12-01T16:52:32.5501280Z ---- [ui] ui-fulldeps/lint-plugin-forbid-cmdline.rs stdout ----
2019-12-01T16:52:32.5501314Z 
2019-12-01T16:52:32.5501610Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T16:52:32.5501663Z status: exit code: 1
2019-12-01T16:52:32.5502489Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary"
2019-12-01T16:52:32.5502832Z ------------------------------------------
2019-12-01T16:52:32.5502865Z 
2019-12-01T16:52:32.5503082Z ------------------------------------------
2019-12-01T16:52:32.5503127Z stderr:
2019-12-01T16:52:32.5503127Z stderr:
2019-12-01T16:52:32.5503355Z ------------------------------------------
2019-12-01T16:52:32.5503404Z error: cannot find macro `declare_lint` in this scope
2019-12-01T16:52:32.5503650Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T16:52:32.5503715Z    |
2019-12-01T16:52:32.5503957Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T16:52:32.5504032Z 
2019-12-01T16:52:32.5504095Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5504361Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T16:52:32.5504407Z    |
2019-12-01T16:52:32.5504407Z    |
2019-12-01T16:52:32.5504468Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T16:52:32.5504547Z 
2019-12-01T16:52:32.5504607Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5504857Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T16:52:32.5504903Z    |
2019-12-01T16:52:32.5504903Z    |
2019-12-01T16:52:32.5505146Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T16:52:32.5505249Z 
2019-12-01T16:52:32.5505293Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5505555Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T16:52:32.5505610Z    |
2019-12-01T16:52:32.5505610Z    |
2019-12-01T16:52:32.5505984Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T16:52:32.5506116Z 
2019-12-01T16:52:32.5506116Z 
2019-12-01T16:52:32.5506564Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T16:52:32.5506853Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T16:52:32.5506944Z LL | #[plugin_registrar]
2019-12-01T16:52:32.5507009Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T16:52:32.5507052Z    |
2019-12-01T16:52:32.5507097Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T16:52:32.5507897Z 
2019-12-01T16:52:32.5507924Z 
2019-12-01T16:52:32.5508143Z ---- [ui] ui-fulldeps/lint-plugin.rs stdout ----
2019-12-01T16:52:32.5508193Z 
2019-12-01T16:52:32.5508471Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T16:52:32.5508523Z status: exit code: 1
2019-12-01T16:52:32.5509308Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary"
2019-12-01T16:52:32.5509969Z ------------------------------------------
2019-12-01T16:52:32.5510005Z 
2019-12-01T16:52:32.5510227Z ------------------------------------------
2019-12-01T16:52:32.5510272Z stderr:
2019-12-01T16:52:32.5510272Z stderr:
2019-12-01T16:52:32.5510503Z ------------------------------------------
2019-12-01T16:52:32.5510552Z error: cannot find macro `declare_lint` in this scope
2019-12-01T16:52:32.5510799Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T16:52:32.5510864Z    |
2019-12-01T16:52:32.5511230Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T16:52:32.5511325Z 
2019-12-01T16:52:32.5511370Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5511621Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T16:52:32.5511685Z    |
2019-12-01T16:52:32.5511685Z    |
2019-12-01T16:52:32.5511728Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T16:52:32.5511824Z 
2019-12-01T16:52:32.5511887Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5512138Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T16:52:32.5512185Z    |
2019-12-01T16:52:32.5512185Z    |
2019-12-01T16:52:32.5512447Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T16:52:32.5512532Z 
2019-12-01T16:52:32.5512575Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5512941Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T16:52:32.5512985Z    |
2019-12-01T16:52:32.5512985Z    |
2019-12-01T16:52:32.5513029Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T16:52:32.5513132Z 
2019-12-01T16:52:32.5513132Z 
2019-12-01T16:52:32.5513592Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T16:52:32.5513937Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T16:52:32.5514146Z LL | #[plugin_registrar]
2019-12-01T16:52:32.5514213Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T16:52:32.5514256Z    |
2019-12-01T16:52:32.5514299Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T16:52:32.5514957Z 
2019-12-01T16:52:32.5514983Z 
2019-12-01T16:52:32.5515212Z ---- [ui] ui-fulldeps/lint-tool-cmdline-allow.rs stdout ----
2019-12-01T16:52:32.5515366Z 
2019-12-01T16:52:32.5515678Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" failed to compile: 
2019-12-01T16:52:32.5515732Z status: exit code: 1
2019-12-01T16:52:32.5516543Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary"
2019-12-01T16:52:32.5516862Z ------------------------------------------
2019-12-01T16:52:32.5516895Z 
2019-12-01T16:52:32.5517149Z ------------------------------------------
2019-12-01T16:52:32.5517201Z stderr:
2019-12-01T16:52:32.5517201Z stderr:
2019-12-01T16:52:32.5517412Z ------------------------------------------
2019-12-01T16:52:32.5517479Z error: cannot find macro `declare_tool_lint` in this scope
2019-12-01T16:52:32.5517725Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:14:1
2019-12-01T16:52:32.5517774Z    |
2019-12-01T16:52:32.5517837Z LL | declare_tool_lint!(pub clippy::TEST_LINT, Warn, "Warn about stuff");
2019-12-01T16:52:32.5517912Z 
2019-12-01T16:52:32.5517956Z error: cannot find macro `declare_tool_lint` in this scope
2019-12-01T16:52:32.5518219Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:15:1
2019-12-01T16:52:32.5518266Z    |
---
2019-12-01T16:52:32.5518991Z 
2019-12-01T16:52:32.5519033Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5519272Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:28:29
2019-12-01T16:52:32.5519317Z    |
2019-12-01T16:52:32.5519381Z LL | declare_lint_pass!(Pass => [TEST_LINT, TEST_GROUP, TEST_RUSTC_TOOL_LINT]);
2019-12-01T16:52:32.5519864Z 
2019-12-01T16:52:32.5519927Z error[E0425]: cannot find value `TEST_GROUP` in this scope
2019-12-01T16:52:32.5520237Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:28:40
2019-12-01T16:52:32.5520284Z    |
2019-12-01T16:52:32.5520284Z    |
2019-12-01T16:52:32.5520348Z LL | declare_lint_pass!(Pass => [TEST_LINT, TEST_GROUP, TEST_RUSTC_TOOL_LINT]);
2019-12-01T16:52:32.5520556Z 
2019-12-01T16:52:32.5520603Z error[E0425]: cannot find value `TEST_RUSTC_TOOL_LINT` in this scope
2019-12-01T16:52:32.5520900Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:28:52
2019-12-01T16:52:32.5520949Z    |
2019-12-01T16:52:32.5520949Z    |
2019-12-01T16:52:32.5520997Z LL | declare_lint_pass!(Pass => [TEST_LINT, TEST_GROUP, TEST_RUSTC_TOOL_LINT]);
2019-12-01T16:52:32.5521104Z 
2019-12-01T16:52:32.5521148Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5521415Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:33:26
2019-12-01T16:52:32.5521462Z    |
2019-12-01T16:52:32.5521462Z    |
2019-12-01T16:52:32.5521709Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T16:52:32.5521960Z 
2019-12-01T16:52:32.5522013Z error[E0425]: cannot find value `TEST_GROUP` in this scope
2019-12-01T16:52:32.5522285Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:36:26
2019-12-01T16:52:32.5522348Z    |
2019-12-01T16:52:32.5522348Z    |
2019-12-01T16:52:32.5522600Z LL |             cx.span_lint(TEST_GROUP, it.span, "item is named 'lintmetoo'");
2019-12-01T16:52:32.5522702Z 
2019-12-01T16:52:32.5522748Z error[E0425]: cannot find value `TEST_RUSTC_TOOL_LINT` in this scope
2019-12-01T16:52:32.5523102Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:43:38
2019-12-01T16:52:32.5523147Z    |
2019-12-01T16:52:32.5523147Z    |
2019-12-01T16:52:32.5523211Z LL |     reg.lint_store.register_lints(&[&TEST_RUSTC_TOOL_LINT, &TEST_LINT, &TEST_GROUP]);
2019-12-01T16:52:32.5523304Z 
2019-12-01T16:52:32.5523369Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5523611Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:43:61
2019-12-01T16:52:32.5523656Z    |
2019-12-01T16:52:32.5523656Z    |
2019-12-01T16:52:32.5523720Z LL |     reg.lint_store.register_lints(&[&TEST_RUSTC_TOOL_LINT, &TEST_LINT, &TEST_GROUP]);
2019-12-01T16:52:32.5523805Z 
2019-12-01T16:52:32.5523847Z error[E0425]: cannot find value `TEST_GROUP` in this scope
2019-12-01T16:52:32.5524101Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:43:73
2019-12-01T16:52:32.5524145Z    |
2019-12-01T16:52:32.5524145Z    |
2019-12-01T16:52:32.5524192Z LL |     reg.lint_store.register_lints(&[&TEST_RUSTC_TOOL_LINT, &TEST_LINT, &TEST_GROUP]);
2019-12-01T16:52:32.5524354Z 
2019-12-01T16:52:32.5524405Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5524763Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:46:26
2019-12-01T16:52:32.5524807Z    |
2019-12-01T16:52:32.5524807Z    |
2019-12-01T16:52:32.5524850Z LL |         vec![LintId::of(&TEST_LINT), LintId::of(&TEST_GROUP)]);
2019-12-01T16:52:32.5524941Z 
2019-12-01T16:52:32.5524983Z error[E0425]: cannot find value `TEST_GROUP` in this scope
2019-12-01T16:52:32.5525211Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:46:50
2019-12-01T16:52:32.5525270Z    |
2019-12-01T16:52:32.5525270Z    |
2019-12-01T16:52:32.5525311Z LL |         vec![LintId::of(&TEST_LINT), LintId::of(&TEST_GROUP)]);
2019-12-01T16:52:32.5525406Z 
2019-12-01T16:52:32.5525406Z 
2019-12-01T16:52:32.5525740Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T16:52:32.5526069Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:41:1
2019-12-01T16:52:32.5526176Z LL | #[plugin_registrar]
2019-12-01T16:52:32.5526220Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T16:52:32.5526277Z    |
2019-12-01T16:52:32.5526317Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T16:52:32.5526952Z 
2019-12-01T16:52:32.5526976Z 
2019-12-01T16:52:32.5527182Z ---- [ui] ui-fulldeps/lint-tool-test.rs stdout ----
2019-12-01T16:52:32.5527212Z 
2019-12-01T16:52:32.5527483Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" failed to compile: 
2019-12-01T16:52:32.5527613Z status: exit code: 1
2019-12-01T16:52:32.5528355Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary"
2019-12-01T16:52:32.5528661Z ------------------------------------------
2019-12-01T16:52:32.5528707Z 
2019-12-01T16:52:32.5528909Z ------------------------------------------
2019-12-01T16:52:32.5528951Z stderr:
2019-12-01T16:52:32.5528951Z stderr:
2019-12-01T16:52:32.5529164Z ------------------------------------------
2019-12-01T16:52:32.5529225Z error: cannot find macro `declare_tool_lint` in this scope
2019-12-01T16:52:32.5529457Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:14:1
2019-12-01T16:52:32.5529519Z    |
2019-12-01T16:52:32.5529891Z LL | declare_tool_lint!(pub clippy::TEST_LINT, Warn, "Warn about stuff");
2019-12-01T16:52:32.5530029Z 
2019-12-01T16:52:32.5530090Z error: cannot find macro `declare_tool_lint` in this scope
2019-12-01T16:52:32.5530388Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:15:1
2019-12-01T16:52:32.5530436Z    |
---
2019-12-01T16:52:32.5531062Z 
2019-12-01T16:52:32.5531106Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5531357Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:28:29
2019-12-01T16:52:32.5531421Z    |
2019-12-01T16:52:32.5531469Z LL | declare_lint_pass!(Pass => [TEST_LINT, TEST_GROUP, TEST_RUSTC_TOOL_LINT]);
2019-12-01T16:52:32.5531574Z 
2019-12-01T16:52:32.5531619Z error[E0425]: cannot find value `TEST_GROUP` in this scope
2019-12-01T16:52:32.5531869Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:28:40
2019-12-01T16:52:32.5531916Z    |
2019-12-01T16:52:32.5531916Z    |
2019-12-01T16:52:32.5531979Z LL | declare_lint_pass!(Pass => [TEST_LINT, TEST_GROUP, TEST_RUSTC_TOOL_LINT]);
2019-12-01T16:52:32.5532071Z 
2019-12-01T16:52:32.5532238Z error[E0425]: cannot find value `TEST_RUSTC_TOOL_LINT` in this scope
2019-12-01T16:52:32.5532525Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:28:52
2019-12-01T16:52:32.5532571Z    |
2019-12-01T16:52:32.5532571Z    |
2019-12-01T16:52:32.5532637Z LL | declare_lint_pass!(Pass => [TEST_LINT, TEST_GROUP, TEST_RUSTC_TOOL_LINT]);
2019-12-01T16:52:32.5532729Z 
2019-12-01T16:52:32.5532772Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5533036Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:33:26
2019-12-01T16:52:32.5533082Z    |
2019-12-01T16:52:32.5533082Z    |
2019-12-01T16:52:32.5533325Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T16:52:32.5533424Z 
2019-12-01T16:52:32.5533468Z error[E0425]: cannot find value `TEST_GROUP` in this scope
2019-12-01T16:52:32.5533845Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:36:26
2019-12-01T16:52:32.5533893Z    |
2019-12-01T16:52:32.5533893Z    |
2019-12-01T16:52:32.5534146Z LL |             cx.span_lint(TEST_GROUP, it.span, "item is named 'lintmetoo'");
2019-12-01T16:52:32.5534248Z 
2019-12-01T16:52:32.5534294Z error[E0425]: cannot find value `TEST_RUSTC_TOOL_LINT` in this scope
2019-12-01T16:52:32.5534542Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:43:38
2019-12-01T16:52:32.5534605Z    |
2019-12-01T16:52:32.5534605Z    |
2019-12-01T16:52:32.5534654Z LL |     reg.lint_store.register_lints(&[&TEST_RUSTC_TOOL_LINT, &TEST_LINT, &TEST_GROUP]);
2019-12-01T16:52:32.5534758Z 
2019-12-01T16:52:32.5534802Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5535048Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:43:61
2019-12-01T16:52:32.5535101Z    |
2019-12-01T16:52:32.5535101Z    |
2019-12-01T16:52:32.5535173Z LL |     reg.lint_store.register_lints(&[&TEST_RUSTC_TOOL_LINT, &TEST_LINT, &TEST_GROUP]);
2019-12-01T16:52:32.5535261Z 
2019-12-01T16:52:32.5535323Z error[E0425]: cannot find value `TEST_GROUP` in this scope
2019-12-01T16:52:32.5535571Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:43:73
2019-12-01T16:52:32.5535617Z    |
2019-12-01T16:52:32.5535617Z    |
2019-12-01T16:52:32.5535681Z LL |     reg.lint_store.register_lints(&[&TEST_RUSTC_TOOL_LINT, &TEST_LINT, &TEST_GROUP]);
2019-12-01T16:52:32.5535775Z 
2019-12-01T16:52:32.5535838Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T16:52:32.5536088Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:46:26
2019-12-01T16:52:32.5536143Z    |
2019-12-01T16:52:32.5536143Z    |
2019-12-01T16:52:32.5536195Z LL |         vec![LintId::of(&TEST_LINT), LintId::of(&TEST_GROUP)]);
2019-12-01T16:52:32.5536292Z 
2019-12-01T16:52:32.5536336Z error[E0425]: cannot find value `TEST_GROUP` in this scope
2019-12-01T16:52:32.5536603Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:46:50
2019-12-01T16:52:32.5536649Z    |
2019-12-01T16:52:32.5536649Z    |
2019-12-01T16:52:32.5536694Z LL |         vec![LintId::of(&TEST_LINT), LintId::of(&TEST_GROUP)]);
2019-12-01T16:52:32.5536794Z 
2019-12-01T16:52:32.5536794Z 
2019-12-01T16:52:32.5537150Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T16:52:32.5537430Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:41:1
2019-12-01T16:52:32.5537526Z LL | #[plugin_registrar]
2019-12-01T16:52:32.5537668Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T16:52:32.5537718Z    |
2019-12-01T16:52:32.5537761Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T16:52:32.5543372Z test result: FAILED. 52 passed; 15 failed; 0 ignored; 0 measured; 0 filtered out
2019-12-01T16:52:32.5543416Z 
2019-12-01T16:52:32.5543442Z 
2019-12-01T16:52:32.5543467Z 
2019-12-01T16:52:32.5545028Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-01T16:52:32.5545261Z 
2019-12-01T16:52:32.5545290Z 
2019-12-01T16:52:32.5545532Z ', src/tools/compiletest/src/main.rs:537:22
2019-12-01T16:52:32.5545587Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-01T16:52:32.5545587Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-01T16:52:32.5545640Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-01T16:52:32.5545706Z Build completed unsuccessfully in 1:08:01
2019-12-01T16:52:32.5545749Z == clock drift check ==
2019-12-01T16:52:32.5545792Z   local time: Sun Dec  1 16:52:32 UTC 2019
2019-12-01T16:52:32.8197702Z   network time: Sun, 01 Dec 2019 16:52:32 GMT
2019-12-01T16:52:32.8201521Z == end clock drift check ==
2019-12-01T16:52:34.5214354Z 
2019-12-01T16:52:34.5311841Z ##[error]Bash exited with code '1'.
2019-12-01T16:52:34.5347138Z ##[section]Starting: Checkout
2019-12-01T16:52:34.5348977Z ==============================================================================
2019-12-01T16:52:34.5349053Z Task         : Get sources
2019-12-01T16:52:34.5349107Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
