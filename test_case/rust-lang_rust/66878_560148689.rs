plain
2019-12-01T18:13:06.4250480Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-01T18:13:06.4435608Z ##[command]git config gc.auto 0
2019-12-01T18:13:06.4520307Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-01T18:13:06.4570710Z ##[command]git config --get-all http.proxy
2019-12-01T18:13:06.4730386Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66878/merge:refs/remotes/pull/66878/merge
---
2019-12-01T19:13:41.6119215Z .................................................................................................... 1600/9316
2019-12-01T19:13:46.4289925Z .................................................................................................... 1700/9316
2019-12-01T19:13:59.2599449Z ........................................i........................................................... 1800/9316
2019-12-01T19:14:07.2855570Z .................................................................................................... 1900/9316
2019-12-01T19:14:21.1984479Z .........................iiiii...................................................................... 2000/9316
2019-12-01T19:14:31.6020858Z .................................................................................................... 2200/9316
2019-12-01T19:14:34.2810477Z .................................................................................................... 2300/9316
2019-12-01T19:14:38.9217511Z .................................................................................................... 2400/9316
2019-12-01T19:15:00.7398541Z .................................................................................................... 2500/9316
---
2019-12-01T19:17:44.5181021Z ...........................i...............i........................................................ 4800/9316
2019-12-01T19:17:55.3494855Z .................................................................................................... 4900/9316
2019-12-01T19:18:01.5334314Z .................................................................................................... 5000/9316
2019-12-01T19:18:09.7947424Z .................................................................................................... 5100/9316
2019-12-01T19:18:17.5768876Z .................................ii.ii...........i.................................................. 5200/9316
2019-12-01T19:18:27.3409070Z .................................................................................................... 5400/9316
2019-12-01T19:18:37.3466932Z .................................................................................................... 5500/9316
2019-12-01T19:18:44.8928785Z ...............i.................................................................................... 5600/9316
2019-12-01T19:18:51.1277043Z .................................................................................................... 5700/9316
2019-12-01T19:18:51.1277043Z .................................................................................................... 5700/9316
2019-12-01T19:19:02.9584679Z .................................................................................................... 5800/9316
2019-12-01T19:19:15.3216356Z .ii...i..ii...........i............................................................................. 5900/9316
2019-12-01T19:19:33.9634027Z .................................................................................................... 6100/9316
2019-12-01T19:19:40.1170628Z .................................................................................................... 6200/9316
2019-12-01T19:19:40.1170628Z .................................................................................................... 6200/9316
2019-12-01T19:19:54.0496856Z ........................i..ii....................................................................... 6300/9316
2019-12-01T19:20:14.2404476Z ...............................................................................................i.... 6500/9316
2019-12-01T19:20:16.5610026Z .................................................................................................... 6600/9316
2019-12-01T19:20:18.8875982Z ......................................................................................i............. 6700/9316
2019-12-01T19:20:21.6909664Z .................................................................................................... 6800/9316
---
2019-12-01T19:25:41.9775628Z  finished in 6.267
2019-12-01T19:25:41.9984057Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T19:25:42.5676393Z 
2019-12-01T19:25:42.5676520Z running 164 tests
2019-12-01T19:25:45.1801539Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/164
2019-12-01T19:25:47.1865102Z .i.i..iiii..iiiiiii............i.........iii.i..........ii......
2019-12-01T19:25:47.1867308Z 
2019-12-01T19:25:47.1871070Z  finished in 5.189
2019-12-01T19:25:47.2086989Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T19:25:47.3777907Z 
---
2019-12-01T19:25:49.4189403Z  finished in 2.210
2019-12-01T19:25:49.4402656Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T19:25:50.5612394Z 
2019-12-01T19:25:50.5612634Z running 9 tests
2019-12-01T19:25:50.5613527Z iiiiiiiii
2019-12-01T19:25:50.5613893Z 
2019-12-01T19:25:50.5613937Z  finished in 0.161
2019-12-01T19:25:50.5614266Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T19:25:50.5614306Z 
---
2019-12-01T19:26:09.8591445Z  finished in 20.233
2019-12-01T19:26:10.4618041Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T19:26:10.6493042Z 
2019-12-01T19:26:10.6493308Z running 124 tests
2019-12-01T19:26:35.5635760Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i....ii...i.......ii 100/124
2019-12-01T19:26:41.5817904Z .i.i.i......iii.i.....ii
2019-12-01T19:26:41.5818762Z 
2019-12-01T19:26:41.5818811Z  finished in 30.264
2019-12-01T19:26:41.5819161Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T19:26:41.5819491Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-12-01T19:27:19.9661577Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-01T19:27:19.9663434Z 
2019-12-01T19:27:19.9663957Z ---- [ui] ui-fulldeps/issue-15778-pass.rs stdout ----
2019-12-01T19:27:19.9663995Z 
2019-12-01T19:27:19.9664767Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" failed to compile: 
2019-12-01T19:27:19.9665203Z status: exit code: 1
2019-12-01T19:27:19.9666970Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary"
2019-12-01T19:27:19.9667681Z ------------------------------------------
2019-12-01T19:27:19.9667850Z 
2019-12-01T19:27:19.9668218Z ------------------------------------------
2019-12-01T19:27:19.9668421Z stderr:
2019-12-01T19:27:19.9668421Z stderr:
2019-12-01T19:27:19.9668779Z ------------------------------------------
2019-12-01T19:27:19.9669152Z error: cannot find macro `declare_lint` in this scope
2019-12-01T19:27:19.9669661Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:40:1
2019-12-01T19:27:19.9670032Z    |
2019-12-01T19:27:19.9670568Z LL | declare_lint!(CRATE_NOT_OKAY, Warn, "crate not marked with #![crate_okay]");
2019-12-01T19:27:19.9670862Z 
2019-12-01T19:27:19.9671006Z error: cannot find macro `declare_lint` in this scope
2019-12-01T19:27:19.9671483Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:41:1
2019-12-01T19:27:19.9671682Z    |
2019-12-01T19:27:19.9671682Z    |
2019-12-01T19:27:19.9671851Z LL | declare_lint!(CRATE_NOT_RED, Warn, "crate not marked with #![crate_red]");
2019-12-01T19:27:19.9672118Z 
2019-12-01T19:27:19.9672262Z error: cannot find macro `declare_lint` in this scope
2019-12-01T19:27:19.9672697Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:42:1
2019-12-01T19:27:19.9672904Z    |
2019-12-01T19:27:19.9672904Z    |
2019-12-01T19:27:19.9673083Z LL | declare_lint!(CRATE_NOT_BLUE, Warn, "crate not marked with #![crate_blue]");
2019-12-01T19:27:19.9673353Z 
2019-12-01T19:27:19.9673516Z error: cannot find macro `declare_lint` in this scope
2019-12-01T19:27:19.9674083Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:43:1
2019-12-01T19:27:19.9674265Z    |
2019-12-01T19:27:19.9674265Z    |
2019-12-01T19:27:19.9674426Z LL | declare_lint!(CRATE_NOT_GREY, Warn, "crate not marked with #![crate_grey]");
2019-12-01T19:27:19.9674681Z 
2019-12-01T19:27:19.9675020Z error: cannot find macro `declare_lint` in this scope
2019-12-01T19:27:19.9675408Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:44:1
2019-12-01T19:27:19.9675585Z    |
2019-12-01T19:27:19.9675585Z    |
2019-12-01T19:27:19.9675745Z LL | declare_lint!(CRATE_NOT_GREEN, Warn, "crate not marked with #![crate_green]");
2019-12-01T19:27:19.9676004Z 
2019-12-01T19:27:19.9676164Z error[E0425]: cannot find value `CRATE_NOT_OKAY` in this scope
2019-12-01T19:27:19.9676557Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:30:38
2019-12-01T19:27:19.9676738Z    |
2019-12-01T19:27:19.9676738Z    |
2019-12-01T19:27:19.9676896Z LL |                           cx.span_lint(CRATE_NOT_OKAY, krate.span,
2019-12-01T19:27:19.9677169Z ...
2019-12-01T19:27:19.9677169Z ...
2019-12-01T19:27:19.9677318Z LL | / fake_lint_pass! {
2019-12-01T19:27:19.9677448Z LL | |     PassOkay,
2019-12-01T19:27:19.9677767Z LL | |     Symbol::intern("rustc_crate_okay")
2019-12-01T19:27:19.9678277Z    | |_- in this macro invocation
2019-12-01T19:27:19.9678437Z 
2019-12-01T19:27:19.9678605Z error[E0425]: cannot find value `CRATE_NOT_OKAY` in this scope
2019-12-01T19:27:19.9678999Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:30:38
2019-12-01T19:27:19.9678999Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:30:38
2019-12-01T19:27:19.9679367Z    |
2019-12-01T19:27:19.9679525Z LL |                           cx.span_lint(CRATE_NOT_OKAY, krate.span,
2019-12-01T19:27:19.9680002Z ...
2019-12-01T19:27:19.9680002Z ...
2019-12-01T19:27:19.9680411Z LL | / fake_lint_pass! {
2019-12-01T19:27:19.9680620Z LL | |     PassRedBlue,
2019-12-01T19:27:19.9680862Z LL | |     Symbol::intern("rustc_crate_red"), Symbol::intern("rustc_crate_blue")
2019-12-01T19:27:19.9681603Z    | |_- in this macro invocation
2019-12-01T19:27:19.9685394Z 
2019-12-01T19:27:19.9685472Z error[E0425]: cannot find value `CRATE_NOT_OKAY` in this scope
2019-12-01T19:27:19.9685869Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:30:38
2019-12-01T19:27:19.9685869Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:30:38
2019-12-01T19:27:19.9685919Z    |
2019-12-01T19:27:19.9685968Z LL |                           cx.span_lint(CRATE_NOT_OKAY, krate.span,
2019-12-01T19:27:19.9686101Z ...
2019-12-01T19:27:19.9686101Z ...
2019-12-01T19:27:19.9686290Z LL | / fake_lint_pass! {
2019-12-01T19:27:19.9686366Z LL | |     PassGreyGreen,
2019-12-01T19:27:19.9686416Z LL | |     Symbol::intern("rustc_crate_grey"), Symbol::intern("rustc_crate_green")
2019-12-01T19:27:19.9686725Z    | |_- in this macro invocation
2019-12-01T19:27:19.9686940Z 
2019-12-01T19:27:19.9686985Z error[E0425]: cannot find value `CRATE_NOT_OKAY` in this scope
2019-12-01T19:27:19.9687254Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:64:10
2019-12-01T19:27:19.9687254Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:64:10
2019-12-01T19:27:19.9687316Z    |
2019-12-01T19:27:19.9687358Z LL |         &CRATE_NOT_OKAY,
2019-12-01T19:27:19.9687433Z 
2019-12-01T19:27:19.9687495Z error[E0425]: cannot find value `CRATE_NOT_RED` in this scope
2019-12-01T19:27:19.9687760Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:65:10
2019-12-01T19:27:19.9687817Z    |
2019-12-01T19:27:19.9687817Z    |
2019-12-01T19:27:19.9687881Z LL |         &CRATE_NOT_RED,
2019-12-01T19:27:19.9687929Z    |          ^^^^^^^^^^^^^ not found in this scope
2019-12-01T19:27:19.9687958Z 
2019-12-01T19:27:19.9688001Z error[E0425]: cannot find value `CRATE_NOT_BLUE` in this scope
2019-12-01T19:27:19.9688444Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:66:10
2019-12-01T19:27:19.9688490Z    |
2019-12-01T19:27:19.9688529Z LL |         &CRATE_NOT_BLUE,
2019-12-01T19:27:19.9688616Z 
2019-12-01T19:27:19.9688616Z 
2019-12-01T19:27:19.9688659Z error[E0425]: cannot find value `CRATE_NOT_GREY` in this scope
2019-12-01T19:27:19.9689096Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:67:10
2019-12-01T19:27:19.9689160Z    |
2019-12-01T19:27:19.9689200Z LL |         &CRATE_NOT_GREY,
2019-12-01T19:27:19.9689294Z 
2019-12-01T19:27:19.9689354Z error[E0425]: cannot find value `CRATE_NOT_GREEN` in this scope
2019-12-01T19:27:19.9689622Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:68:10
2019-12-01T19:27:19.9689684Z    |
2019-12-01T19:27:19.9689684Z    |
2019-12-01T19:27:19.9689726Z LL |         &CRATE_NOT_GREEN,
2019-12-01T19:27:19.9689981Z 
2019-12-01T19:27:19.9690075Z warning: unused `#[macro_use]` import
2019-12-01T19:27:19.9690742Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:6:1
2019-12-01T19:27:19.9690798Z    |
2019-12-01T19:27:19.9690798Z    |
2019-12-01T19:27:19.9690858Z LL | #[macro_use] extern crate rustc;
2019-12-01T19:27:19.9690903Z    | ^^^^^^^^^^^^
2019-12-01T19:27:19.9690944Z    |
2019-12-01T19:27:19.9690989Z    = note: `#[warn(unused_imports)]` on by default
2019-12-01T19:27:19.9691038Z 
2019-12-01T19:27:19.9691516Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T19:27:19.9692041Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:61:1
2019-12-01T19:27:19.9692157Z LL | #[plugin_registrar]
2019-12-01T19:27:19.9692210Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T19:27:19.9692275Z    |
2019-12-01T19:27:19.9692322Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T19:27:19.9693089Z 
2019-12-01T19:27:19.9693117Z 
2019-12-01T19:27:19.9693378Z ---- [ui] ui-fulldeps/issue-40001.rs stdout ----
2019-12-01T19:27:19.9693411Z 
2019-12-01T19:27:19.9693751Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
2019-12-01T19:27:19.9693817Z status: exit code: 1
2019-12-01T19:27:19.9694739Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
2019-12-01T19:27:19.9695150Z ------------------------------------------
2019-12-01T19:27:19.9695203Z 
2019-12-01T19:27:19.9695459Z ------------------------------------------
2019-12-01T19:27:19.9695509Z stderr:
2019-12-01T19:27:19.9695509Z stderr:
2019-12-01T19:27:19.9695935Z ------------------------------------------
2019-12-01T19:27:19.9695987Z error: cannot find macro `declare_lint` in this scope
2019-12-01T19:27:19.9696275Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:21:1
2019-12-01T19:27:19.9696385Z LL | declare_lint! {
2019-12-01T19:27:19.9696427Z    | ^^^^^^^^^^^^
2019-12-01T19:27:19.9696453Z 
2019-12-01T19:27:19.9696517Z error[E0425]: cannot find value `MISSING_WHITELISTED_ATTR` in this scope
2019-12-01T19:27:19.9696517Z error[E0425]: cannot find value `MISSING_WHITELISTED_ATTR` in this scope
2019-12-01T19:27:19.9696784Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:17:38
2019-12-01T19:27:19.9696829Z    |
2019-12-01T19:27:19.9696890Z LL |     reg.lint_store.register_lints(&[&MISSING_WHITELISTED_ATTR]);
2019-12-01T19:27:19.9696974Z 
2019-12-01T19:27:19.9697018Z error[E0425]: cannot find value `MISSING_WHITELISTED_ATTR` in this scope
2019-12-01T19:27:19.9697018Z error[E0425]: cannot find value `MISSING_WHITELISTED_ATTR` in this scope
2019-12-01T19:27:19.9697463Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:27:51
2019-12-01T19:27:19.9697518Z    |
2019-12-01T19:27:19.9697570Z LL | declare_lint_pass!(MissingWhitelistedAttrPass => [MISSING_WHITELISTED_ATTR]);
2019-12-01T19:27:19.9697673Z 
2019-12-01T19:27:19.9697718Z error[E0425]: cannot find value `MISSING_WHITELISTED_ATTR` in this scope
2019-12-01T19:27:19.9697718Z error[E0425]: cannot find value `MISSING_WHITELISTED_ATTR` in this scope
2019-12-01T19:27:19.9697995Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:45:26
2019-12-01T19:27:19.9698039Z    |
2019-12-01T19:27:19.9698082Z LL |             cx.span_lint(MISSING_WHITELISTED_ATTR, span,
2019-12-01T19:27:19.9698177Z 
2019-12-01T19:27:19.9698177Z 
2019-12-01T19:27:19.9698520Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T19:27:19.9698806Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:15:1
2019-12-01T19:27:19.9698995Z LL | #[plugin_registrar]
2019-12-01T19:27:19.9699065Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T19:27:19.9699106Z    |
2019-12-01T19:27:19.9699147Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T19:27:19.9700012Z 
2019-12-01T19:27:19.9700036Z 
2019-12-01T19:27:19.9700787Z ---- [ui] ui-fulldeps/lint-group-plugin-deny-cmdline.rs stdout ----
2019-12-01T19:27:19.9700847Z 
2019-12-01T19:27:19.9701156Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
2019-12-01T19:27:19.9701211Z status: exit code: 1
2019-12-01T19:27:19.9702179Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary"
2019-12-01T19:27:19.9702583Z ------------------------------------------
2019-12-01T19:27:19.9702617Z 
2019-12-01T19:27:19.9702848Z ------------------------------------------
2019-12-01T19:27:19.9702911Z stderr:
2019-12-01T19:27:19.9702911Z stderr:
2019-12-01T19:27:19.9703142Z ------------------------------------------
2019-12-01T19:27:19.9703194Z error: cannot find macro `declare_lint` in this scope
2019-12-01T19:27:19.9703548Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:15:1
2019-12-01T19:27:19.9703620Z    |
2019-12-01T19:27:19.9704052Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T19:27:19.9704143Z 
2019-12-01T19:27:19.9704185Z error: cannot find macro `declare_lint` in this scope
2019-12-01T19:27:19.9704448Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:17:1
2019-12-01T19:27:19.9704512Z    |
2019-12-01T19:27:19.9704512Z    |
2019-12-01T19:27:19.9704774Z LL | declare_lint!(PLEASE_LINT, Warn, "Warn about items named 'pleaselintme'");
2019-12-01T19:27:19.9704848Z 
2019-12-01T19:27:19.9704908Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9705174Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:19:29
2019-12-01T19:27:19.9705221Z    |
2019-12-01T19:27:19.9705221Z    |
2019-12-01T19:27:19.9705280Z LL | declare_lint_pass!(Pass => [TEST_LINT, PLEASE_LINT]);
2019-12-01T19:27:19.9705375Z 
2019-12-01T19:27:19.9705418Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T19:27:19.9705891Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:19:40
2019-12-01T19:27:19.9705942Z    |
2019-12-01T19:27:19.9705942Z    |
2019-12-01T19:27:19.9705985Z LL | declare_lint_pass!(Pass => [TEST_LINT, PLEASE_LINT]);
2019-12-01T19:27:19.9706081Z 
2019-12-01T19:27:19.9706124Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9706408Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:24:38
2019-12-01T19:27:19.9706455Z    |
2019-12-01T19:27:19.9706455Z    |
2019-12-01T19:27:19.9706722Z LL |             "lintme" => cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'"),
2019-12-01T19:27:19.9706944Z 
2019-12-01T19:27:19.9706998Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T19:27:19.9707292Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:25:44
2019-12-01T19:27:19.9707357Z    |
2019-12-01T19:27:19.9707357Z    |
2019-12-01T19:27:19.9707802Z LL |             "pleaselintme" => cx.span_lint(PLEASE_LINT, it.span, "item is named 'pleaselintme'"),
2019-12-01T19:27:19.9707910Z 
2019-12-01T19:27:19.9707952Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9708206Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:33:38
2019-12-01T19:27:19.9708267Z    |
2019-12-01T19:27:19.9708267Z    |
2019-12-01T19:27:19.9708310Z LL |     reg.lint_store.register_lints(&[&TEST_LINT, &PLEASE_LINT]);
2019-12-01T19:27:19.9708397Z 
2019-12-01T19:27:19.9708539Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T19:27:19.9708834Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:33:50
2019-12-01T19:27:19.9708879Z    |
2019-12-01T19:27:19.9708879Z    |
2019-12-01T19:27:19.9708939Z LL |     reg.lint_store.register_lints(&[&TEST_LINT, &PLEASE_LINT]);
2019-12-01T19:27:19.9709020Z 
2019-12-01T19:27:19.9709078Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9709333Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:36:26
2019-12-01T19:27:19.9709377Z    |
2019-12-01T19:27:19.9709377Z    |
2019-12-01T19:27:19.9709420Z LL |         vec![LintId::of(&TEST_LINT), LintId::of(&PLEASE_LINT)]);
2019-12-01T19:27:19.9709513Z 
2019-12-01T19:27:19.9709554Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T19:27:19.9710031Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:36:50
2019-12-01T19:27:19.9710080Z    |
2019-12-01T19:27:19.9710080Z    |
2019-12-01T19:27:19.9710619Z LL |         vec![LintId::of(&TEST_LINT), LintId::of(&PLEASE_LINT)]);
2019-12-01T19:27:19.9710737Z 
2019-12-01T19:27:19.9710737Z 
2019-12-01T19:27:19.9711171Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T19:27:19.9711528Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:31:1
2019-12-01T19:27:19.9711619Z LL | #[plugin_registrar]
2019-12-01T19:27:19.9711685Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T19:27:19.9711729Z    |
2019-12-01T19:27:19.9711774Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T19:27:19.9712498Z 
2019-12-01T19:27:19.9712523Z 
2019-12-01T19:27:19.9712765Z ---- [ui] ui-fulldeps/lint-group-plugin.rs stdout ----
2019-12-01T19:27:19.9712814Z 
2019-12-01T19:27:19.9713115Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
2019-12-01T19:27:19.9713169Z status: exit code: 1
2019-12-01T19:27:19.9714160Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary"
2019-12-01T19:27:19.9714648Z ------------------------------------------
2019-12-01T19:27:19.9714682Z 
2019-12-01T19:27:19.9714905Z ------------------------------------------
2019-12-01T19:27:19.9714949Z stderr:
2019-12-01T19:27:19.9714949Z stderr:
2019-12-01T19:27:19.9715187Z ------------------------------------------
2019-12-01T19:27:19.9715236Z error: cannot find macro `declare_lint` in this scope
2019-12-01T19:27:19.9715494Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:15:1
2019-12-01T19:27:19.9715558Z    |
2019-12-01T19:27:19.9715813Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T19:27:19.9715903Z 
2019-12-01T19:27:19.9715946Z error: cannot find macro `declare_lint` in this scope
2019-12-01T19:27:19.9716303Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:17:1
2019-12-01T19:27:19.9716378Z    |
2019-12-01T19:27:19.9716378Z    |
2019-12-01T19:27:19.9716664Z LL | declare_lint!(PLEASE_LINT, Warn, "Warn about items named 'pleaselintme'");
2019-12-01T19:27:19.9716739Z 
2019-12-01T19:27:19.9716799Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9717062Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:19:29
2019-12-01T19:27:19.9717108Z    |
2019-12-01T19:27:19.9717108Z    |
2019-12-01T19:27:19.9717168Z LL | declare_lint_pass!(Pass => [TEST_LINT, PLEASE_LINT]);
2019-12-01T19:27:19.9717247Z 
2019-12-01T19:27:19.9717290Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T19:27:19.9717573Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:19:40
2019-12-01T19:27:19.9717620Z    |
2019-12-01T19:27:19.9717620Z    |
2019-12-01T19:27:19.9717672Z LL | declare_lint_pass!(Pass => [TEST_LINT, PLEASE_LINT]);
2019-12-01T19:27:19.9717775Z 
2019-12-01T19:27:19.9717818Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9718085Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:24:38
2019-12-01T19:27:19.9718153Z    |
2019-12-01T19:27:19.9718153Z    |
2019-12-01T19:27:19.9718578Z LL |             "lintme" => cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'"),
2019-12-01T19:27:19.9718676Z 
2019-12-01T19:27:19.9718717Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T19:27:19.9718970Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:25:44
2019-12-01T19:27:19.9719032Z    |
2019-12-01T19:27:19.9719032Z    |
2019-12-01T19:27:19.9719303Z LL |             "pleaselintme" => cx.span_lint(PLEASE_LINT, it.span, "item is named 'pleaselintme'"),
2019-12-01T19:27:19.9719425Z 
2019-12-01T19:27:19.9719466Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9719723Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:33:38
2019-12-01T19:27:19.9719946Z    |
2019-12-01T19:27:19.9719946Z    |
2019-12-01T19:27:19.9720008Z LL |     reg.lint_store.register_lints(&[&TEST_LINT, &PLEASE_LINT]);
2019-12-01T19:27:19.9720089Z 
2019-12-01T19:27:19.9720542Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T19:27:19.9720867Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:33:50
2019-12-01T19:27:19.9720917Z    |
2019-12-01T19:27:19.9720917Z    |
2019-12-01T19:27:19.9720981Z LL |     reg.lint_store.register_lints(&[&TEST_LINT, &PLEASE_LINT]);
2019-12-01T19:27:19.9721192Z 
2019-12-01T19:27:19.9721237Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9721556Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:36:26
2019-12-01T19:27:19.9721605Z    |
2019-12-01T19:27:19.9721605Z    |
2019-12-01T19:27:19.9721651Z LL |         vec![LintId::of(&TEST_LINT), LintId::of(&PLEASE_LINT)]);
2019-12-01T19:27:19.9721749Z 
2019-12-01T19:27:19.9721793Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T19:27:19.9722085Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:36:50
2019-12-01T19:27:19.9722134Z    |
2019-12-01T19:27:19.9722134Z    |
2019-12-01T19:27:19.9722179Z LL |         vec![LintId::of(&TEST_LINT), LintId::of(&PLEASE_LINT)]);
2019-12-01T19:27:19.9722293Z 
2019-12-01T19:27:19.9722293Z 
2019-12-01T19:27:19.9722749Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T19:27:19.9723105Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:31:1
2019-12-01T19:27:19.9723196Z LL | #[plugin_registrar]
2019-12-01T19:27:19.9723244Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T19:27:19.9723306Z    |
2019-12-01T19:27:19.9723349Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T19:27:19.9724200Z 
2019-12-01T19:27:19.9724225Z 
2019-12-01T19:27:19.9724463Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-allow.rs stdout ----
2019-12-01T19:27:19.9724505Z 
2019-12-01T19:27:19.9724815Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T19:27:19.9724868Z status: exit code: 1
2019-12-01T19:27:19.9725659Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary"
2019-12-01T19:27:19.9726006Z ------------------------------------------
2019-12-01T19:27:19.9726039Z 
2019-12-01T19:27:19.9726271Z ------------------------------------------
2019-12-01T19:27:19.9726323Z stderr:
2019-12-01T19:27:19.9726323Z stderr:
2019-12-01T19:27:19.9726564Z ------------------------------------------
2019-12-01T19:27:19.9726614Z error: cannot find macro `declare_lint` in this scope
2019-12-01T19:27:19.9726865Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T19:27:19.9726927Z    |
2019-12-01T19:27:19.9727178Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T19:27:19.9727251Z 
2019-12-01T19:27:19.9727313Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9727568Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T19:27:19.9727614Z    |
2019-12-01T19:27:19.9727614Z    |
2019-12-01T19:27:19.9727672Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T19:27:19.9727751Z 
2019-12-01T19:27:19.9727907Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9728205Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T19:27:19.9728253Z    |
2019-12-01T19:27:19.9728253Z    |
2019-12-01T19:27:19.9728506Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T19:27:19.9728606Z 
2019-12-01T19:27:19.9728649Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9728924Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T19:27:19.9728970Z    |
2019-12-01T19:27:19.9728970Z    |
2019-12-01T19:27:19.9729013Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T19:27:19.9729273Z 
2019-12-01T19:27:19.9729273Z 
2019-12-01T19:27:19.9729607Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T19:27:19.9730084Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T19:27:19.9730711Z LL | #[plugin_registrar]
2019-12-01T19:27:19.9730779Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T19:27:19.9730824Z    |
2019-12-01T19:27:19.9730868Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T19:27:19.9731638Z 
2019-12-01T19:27:19.9731664Z 
2019-12-01T19:27:19.9731911Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-load.rs stdout ----
2019-12-01T19:27:19.9731961Z 
2019-12-01T19:27:19.9732258Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T19:27:19.9732324Z status: exit code: 1
2019-12-01T19:27:19.9733171Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary"
2019-12-01T19:27:19.9733519Z ------------------------------------------
2019-12-01T19:27:19.9733552Z 
2019-12-01T19:27:19.9733939Z ------------------------------------------
2019-12-01T19:27:19.9733983Z stderr:
2019-12-01T19:27:19.9733983Z stderr:
2019-12-01T19:27:19.9734220Z ------------------------------------------
2019-12-01T19:27:19.9734282Z error: cannot find macro `declare_lint` in this scope
2019-12-01T19:27:19.9734545Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T19:27:19.9734610Z    |
2019-12-01T19:27:19.9734865Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T19:27:19.9734955Z 
2019-12-01T19:27:19.9735000Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9735257Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T19:27:19.9735321Z    |
2019-12-01T19:27:19.9735321Z    |
2019-12-01T19:27:19.9735363Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T19:27:19.9743372Z 
2019-12-01T19:27:19.9743532Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9744573Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T19:27:19.9744821Z    |
2019-12-01T19:27:19.9744821Z    |
2019-12-01T19:27:19.9745163Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T19:27:19.9745255Z 
2019-12-01T19:27:19.9745298Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9745844Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T19:27:19.9745898Z    |
2019-12-01T19:27:19.9745898Z    |
2019-12-01T19:27:19.9745941Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T19:27:19.9746043Z 
2019-12-01T19:27:19.9746043Z 
2019-12-01T19:27:19.9746480Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T19:27:19.9746820Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T19:27:19.9746919Z LL | #[plugin_registrar]
2019-12-01T19:27:19.9747130Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T19:27:19.9747191Z    |
2019-12-01T19:27:19.9747241Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T19:27:19.9748007Z 
2019-12-01T19:27:19.9748032Z 
2019-12-01T19:27:19.9748287Z ---- [ui] ui-fulldeps/lint-plugin-deny-attr.rs stdout ----
2019-12-01T19:27:19.9748338Z 
2019-12-01T19:27:19.9748642Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T19:27:19.9748697Z status: exit code: 1
2019-12-01T19:27:19.9749541Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary"
2019-12-01T19:27:19.9750088Z ------------------------------------------
2019-12-01T19:27:19.9750343Z 
2019-12-01T19:27:19.9750651Z ------------------------------------------
2019-12-01T19:27:19.9750718Z stderr:
2019-12-01T19:27:19.9750718Z stderr:
2019-12-01T19:27:19.9750948Z ------------------------------------------
2019-12-01T19:27:19.9751001Z error: cannot find macro `declare_lint` in this scope
2019-12-01T19:27:19.9751283Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T19:27:19.9751345Z    |
2019-12-01T19:27:19.9751619Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T19:27:19.9751715Z 
2019-12-01T19:27:19.9751762Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9752034Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T19:27:19.9752100Z    |
2019-12-01T19:27:19.9752100Z    |
2019-12-01T19:27:19.9752145Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T19:27:19.9752226Z 
2019-12-01T19:27:19.9752289Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9752557Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T19:27:19.9752605Z    |
2019-12-01T19:27:19.9752605Z    |
2019-12-01T19:27:19.9752887Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T19:27:19.9753107Z 
2019-12-01T19:27:19.9753179Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9753484Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T19:27:19.9753699Z    |
2019-12-01T19:27:19.9753699Z    |
2019-12-01T19:27:19.9753744Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T19:27:19.9754004Z 
2019-12-01T19:27:19.9754004Z 
2019-12-01T19:27:19.9754370Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T19:27:19.9754639Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T19:27:19.9754741Z LL | #[plugin_registrar]
2019-12-01T19:27:19.9754787Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T19:27:19.9754828Z    |
2019-12-01T19:27:19.9754869Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T19:27:19.9755832Z 
2019-12-01T19:27:19.9755859Z 
2019-12-01T19:27:19.9756115Z ---- [ui] ui-fulldeps/lint-plugin-deny-cmdline.rs stdout ----
2019-12-01T19:27:19.9756147Z 
2019-12-01T19:27:19.9756431Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T19:27:19.9756483Z status: exit code: 1
2019-12-01T19:27:19.9757307Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary"
2019-12-01T19:27:19.9757652Z ------------------------------------------
2019-12-01T19:27:19.9757683Z 
2019-12-01T19:27:19.9757910Z ------------------------------------------
2019-12-01T19:27:19.9757971Z stderr:
2019-12-01T19:27:19.9757971Z stderr:
2019-12-01T19:27:19.9758190Z ------------------------------------------
2019-12-01T19:27:19.9758240Z error: cannot find macro `declare_lint` in this scope
2019-12-01T19:27:19.9758511Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T19:27:19.9758559Z    |
2019-12-01T19:27:19.9758809Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T19:27:19.9758910Z 
2019-12-01T19:27:19.9758961Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9759221Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T19:27:19.9759284Z    |
2019-12-01T19:27:19.9759284Z    |
2019-12-01T19:27:19.9759327Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T19:27:19.9759424Z 
2019-12-01T19:27:19.9759468Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9759727Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T19:27:19.9759963Z    |
2019-12-01T19:27:19.9759963Z    |
2019-12-01T19:27:19.9760477Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T19:27:19.9760572Z 
2019-12-01T19:27:19.9760635Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9761684Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T19:27:19.9761752Z    |
2019-12-01T19:27:19.9761752Z    |
2019-12-01T19:27:19.9761816Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T19:27:19.9761906Z 
2019-12-01T19:27:19.9761906Z 
2019-12-01T19:27:19.9762314Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T19:27:19.9762601Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T19:27:19.9762712Z LL | #[plugin_registrar]
2019-12-01T19:27:19.9762762Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T19:27:19.9762804Z    |
2019-12-01T19:27:19.9762868Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T19:27:19.9763709Z 
2019-12-01T19:27:19.9763735Z 
2019-12-01T19:27:19.9764000Z ---- [ui] ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----
2019-12-01T19:27:19.9764033Z 
2019-12-01T19:27:19.9764326Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T19:27:19.9764397Z status: exit code: 1
2019-12-01T19:27:19.9765392Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary"
2019-12-01T19:27:19.9765740Z ------------------------------------------
2019-12-01T19:27:19.9765773Z 
2019-12-01T19:27:19.9766014Z ------------------------------------------
2019-12-01T19:27:19.9766058Z stderr:
2019-12-01T19:27:19.9766058Z stderr:
2019-12-01T19:27:19.9766278Z ------------------------------------------
2019-12-01T19:27:19.9766506Z error: cannot find macro `declare_lint` in this scope
2019-12-01T19:27:19.9766787Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T19:27:19.9766835Z    |
2019-12-01T19:27:19.9767097Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T19:27:19.9767191Z 
2019-12-01T19:27:19.9767235Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9767518Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T19:27:19.9767574Z    |
2019-12-01T19:27:19.9767574Z    |
2019-12-01T19:27:19.9767626Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T19:27:19.9767725Z 
2019-12-01T19:27:19.9767770Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9768039Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T19:27:19.9768104Z    |
2019-12-01T19:27:19.9768104Z    |
2019-12-01T19:27:19.9768675Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T19:27:19.9768786Z 
2019-12-01T19:27:19.9768833Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9769140Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T19:27:19.9769189Z    |
2019-12-01T19:27:19.9769189Z    |
2019-12-01T19:27:19.9769251Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T19:27:19.9769510Z 
2019-12-01T19:27:19.9769510Z 
2019-12-01T19:27:19.9769931Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T19:27:19.9770487Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T19:27:19.9770608Z LL | #[plugin_registrar]
2019-12-01T19:27:19.9770657Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T19:27:19.9770701Z    |
2019-12-01T19:27:19.9770766Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T19:27:19.9771456Z 
2019-12-01T19:27:19.9771497Z 
2019-12-01T19:27:19.9771860Z ---- [ui] ui-fulldeps/lint-plugin-forbid-cmdline.rs stdout ----
2019-12-01T19:27:19.9771905Z 
2019-12-01T19:27:19.9772232Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T19:27:19.9772303Z status: exit code: 1
2019-12-01T19:27:19.9773137Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary"
2019-12-01T19:27:19.9773499Z ------------------------------------------
2019-12-01T19:27:19.9773700Z 
2019-12-01T19:27:19.9775648Z ------------------------------------------
2019-12-01T19:27:19.9775904Z stderr:
2019-12-01T19:27:19.9775904Z stderr:
2019-12-01T19:27:19.9776191Z ------------------------------------------
2019-12-01T19:27:19.9776263Z error: cannot find macro `declare_lint` in this scope
2019-12-01T19:27:19.9776520Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T19:27:19.9776568Z    |
2019-12-01T19:27:19.9776831Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T19:27:19.9776904Z 
2019-12-01T19:27:19.9776947Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9777219Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T19:27:19.9777263Z    |
2019-12-01T19:27:19.9777263Z    |
2019-12-01T19:27:19.9777304Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T19:27:19.9777420Z 
2019-12-01T19:27:19.9777462Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9778075Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T19:27:19.9778135Z    |
2019-12-01T19:27:19.9778135Z    |
2019-12-01T19:27:19.9778614Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T19:27:19.9778721Z 
2019-12-01T19:27:19.9778765Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9779027Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T19:27:19.9779089Z    |
2019-12-01T19:27:19.9779089Z    |
2019-12-01T19:27:19.9779133Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T19:27:19.9779214Z 
2019-12-01T19:27:19.9779214Z 
2019-12-01T19:27:19.9779984Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T19:27:19.9784926Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T19:27:19.9787622Z LL | #[plugin_registrar]
2019-12-01T19:27:19.9787681Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T19:27:19.9787748Z    |
2019-12-01T19:27:19.9787794Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T19:27:19.9788637Z 
2019-12-01T19:27:19.9788662Z 
2019-12-01T19:27:19.9788892Z ---- [ui] ui-fulldeps/lint-plugin.rs stdout ----
2019-12-01T19:27:19.9788943Z 
2019-12-01T19:27:19.9789407Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T19:27:19.9789478Z status: exit code: 1
2019-12-01T19:27:19.9790783Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary"
2019-12-01T19:27:19.9791161Z ------------------------------------------
2019-12-01T19:27:19.9791214Z 
2019-12-01T19:27:19.9791452Z ------------------------------------------
2019-12-01T19:27:19.9791512Z stderr:
2019-12-01T19:27:19.9791512Z stderr:
2019-12-01T19:27:19.9791751Z ------------------------------------------
2019-12-01T19:27:19.9791820Z error: cannot find macro `declare_lint` in this scope
2019-12-01T19:27:19.9792092Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T19:27:19.9792140Z    |
2019-12-01T19:27:19.9792423Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T19:27:19.9792501Z 
2019-12-01T19:27:19.9792564Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9792835Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T19:27:19.9792883Z    |
2019-12-01T19:27:19.9792883Z    |
2019-12-01T19:27:19.9792927Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T19:27:19.9793027Z 
2019-12-01T19:27:19.9793072Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9793381Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T19:27:19.9793431Z    |
2019-12-01T19:27:19.9793431Z    |
2019-12-01T19:27:19.9793696Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T19:27:19.9794087Z 
2019-12-01T19:27:19.9794214Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9794538Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T19:27:19.9794605Z    |
2019-12-01T19:27:19.9794605Z    |
2019-12-01T19:27:19.9794652Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T19:27:19.9794755Z 
2019-12-01T19:27:19.9794755Z 
2019-12-01T19:27:19.9795260Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T19:27:19.9795579Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T19:27:19.9795851Z LL | #[plugin_registrar]
2019-12-01T19:27:19.9795901Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T19:27:19.9795963Z    |
2019-12-01T19:27:19.9796009Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T19:27:19.9796808Z 
2019-12-01T19:27:19.9796834Z 
2019-12-01T19:27:19.9797088Z ---- [ui] ui-fulldeps/lint-tool-cmdline-allow.rs stdout ----
2019-12-01T19:27:19.9797122Z 
2019-12-01T19:27:19.9797433Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" failed to compile: 
2019-12-01T19:27:19.9797487Z status: exit code: 1
2019-12-01T19:27:19.9798552Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary"
2019-12-01T19:27:19.9798929Z ------------------------------------------
2019-12-01T19:27:19.9798979Z 
2019-12-01T19:27:19.9799206Z ------------------------------------------
2019-12-01T19:27:19.9799251Z stderr:
2019-12-01T19:27:19.9799251Z stderr:
2019-12-01T19:27:19.9799485Z ------------------------------------------
2019-12-01T19:27:19.9799535Z error: cannot find macro `declare_tool_lint` in this scope
2019-12-01T19:27:19.9799989Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:14:1
2019-12-01T19:27:19.9800101Z    |
2019-12-01T19:27:19.9800148Z LL | declare_tool_lint!(pub clippy::TEST_LINT, Warn, "Warn about stuff");
2019-12-01T19:27:19.9800742Z 
2019-12-01T19:27:19.9800808Z error: cannot find macro `declare_tool_lint` in this scope
2019-12-01T19:27:19.9801128Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:15:1
2019-12-01T19:27:19.9801178Z    |
---
2019-12-01T19:27:19.9801828Z 
2019-12-01T19:27:19.9801879Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9802149Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:28:29
2019-12-01T19:27:19.9802215Z    |
2019-12-01T19:27:19.9802265Z LL | declare_lint_pass!(Pass => [TEST_LINT, TEST_GROUP, TEST_RUSTC_TOOL_LINT]);
2019-12-01T19:27:19.9802375Z 
2019-12-01T19:27:19.9802420Z error[E0425]: cannot find value `TEST_GROUP` in this scope
2019-12-01T19:27:19.9802689Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:28:40
2019-12-01T19:27:19.9802753Z    |
2019-12-01T19:27:19.9802753Z    |
2019-12-01T19:27:19.9802802Z LL | declare_lint_pass!(Pass => [TEST_LINT, TEST_GROUP, TEST_RUSTC_TOOL_LINT]);
2019-12-01T19:27:19.9802888Z 
2019-12-01T19:27:19.9802952Z error[E0425]: cannot find value `TEST_RUSTC_TOOL_LINT` in this scope
2019-12-01T19:27:19.9803374Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:28:52
2019-12-01T19:27:19.9803423Z    |
2019-12-01T19:27:19.9803423Z    |
2019-12-01T19:27:19.9803487Z LL | declare_lint_pass!(Pass => [TEST_LINT, TEST_GROUP, TEST_RUSTC_TOOL_LINT]);
2019-12-01T19:27:19.9803581Z 
2019-12-01T19:27:19.9803643Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9803916Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:33:26
2019-12-01T19:27:19.9803963Z    |
2019-12-01T19:27:19.9803963Z    |
2019-12-01T19:27:19.9804224Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T19:27:19.9804327Z 
2019-12-01T19:27:19.9804372Z error[E0425]: cannot find value `TEST_GROUP` in this scope
2019-12-01T19:27:19.9804654Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:36:26
2019-12-01T19:27:19.9804712Z    |
2019-12-01T19:27:19.9804712Z    |
2019-12-01T19:27:19.9805083Z LL |             cx.span_lint(TEST_GROUP, it.span, "item is named 'lintmetoo'");
2019-12-01T19:27:19.9805307Z 
2019-12-01T19:27:19.9805355Z error[E0425]: cannot find value `TEST_RUSTC_TOOL_LINT` in this scope
2019-12-01T19:27:19.9805651Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:43:38
2019-12-01T19:27:19.9805720Z    |
2019-12-01T19:27:19.9805720Z    |
2019-12-01T19:27:19.9806112Z LL |     reg.lint_store.register_lints(&[&TEST_RUSTC_TOOL_LINT, &TEST_LINT, &TEST_GROUP]);
2019-12-01T19:27:19.9806227Z 
2019-12-01T19:27:19.9806272Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9806586Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:43:61
2019-12-01T19:27:19.9806651Z    |
2019-12-01T19:27:19.9806651Z    |
2019-12-01T19:27:19.9806722Z LL |     reg.lint_store.register_lints(&[&TEST_RUSTC_TOOL_LINT, &TEST_LINT, &TEST_GROUP]);
2019-12-01T19:27:19.9806813Z 
2019-12-01T19:27:19.9806878Z error[E0425]: cannot find value `TEST_GROUP` in this scope
2019-12-01T19:27:19.9807148Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:43:73
2019-12-01T19:27:19.9807196Z    |
2019-12-01T19:27:19.9807196Z    |
2019-12-01T19:27:19.9807261Z LL |     reg.lint_store.register_lints(&[&TEST_RUSTC_TOOL_LINT, &TEST_LINT, &TEST_GROUP]);
2019-12-01T19:27:19.9807360Z 
2019-12-01T19:27:19.9807427Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9807698Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:46:26
2019-12-01T19:27:19.9807744Z    |
2019-12-01T19:27:19.9807744Z    |
2019-12-01T19:27:19.9807817Z LL |         vec![LintId::of(&TEST_LINT), LintId::of(&TEST_GROUP)]);
2019-12-01T19:27:19.9807907Z 
2019-12-01T19:27:19.9807953Z error[E0425]: cannot find value `TEST_GROUP` in this scope
2019-12-01T19:27:19.9808264Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:46:50
2019-12-01T19:27:19.9808311Z    |
2019-12-01T19:27:19.9808311Z    |
2019-12-01T19:27:19.9808357Z LL |         vec![LintId::of(&TEST_LINT), LintId::of(&TEST_GROUP)]);
2019-12-01T19:27:19.9808461Z 
2019-12-01T19:27:19.9808461Z 
2019-12-01T19:27:19.9808833Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T19:27:19.9809173Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:41:1
2019-12-01T19:27:19.9809272Z LL | #[plugin_registrar]
2019-12-01T19:27:19.9809468Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T19:27:19.9809525Z    |
2019-12-01T19:27:19.9809574Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T19:27:19.9810611Z 
2019-12-01T19:27:19.9810637Z 
2019-12-01T19:27:19.9810878Z ---- [ui] ui-fulldeps/lint-tool-test.rs stdout ----
2019-12-01T19:27:19.9811117Z 
2019-12-01T19:27:19.9811453Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" failed to compile: 
2019-12-01T19:27:19.9811509Z status: exit code: 1
2019-12-01T19:27:19.9812444Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary"
2019-12-01T19:27:19.9812851Z ------------------------------------------
2019-12-01T19:27:19.9812885Z 
2019-12-01T19:27:19.9813118Z ------------------------------------------
2019-12-01T19:27:19.9813164Z stderr:
2019-12-01T19:27:19.9813164Z stderr:
2019-12-01T19:27:19.9813413Z ------------------------------------------
2019-12-01T19:27:19.9813465Z error: cannot find macro `declare_tool_lint` in this scope
2019-12-01T19:27:19.9813727Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:14:1
2019-12-01T19:27:19.9813799Z    |
2019-12-01T19:27:19.9813864Z LL | declare_tool_lint!(pub clippy::TEST_LINT, Warn, "Warn about stuff");
2019-12-01T19:27:19.9813960Z 
2019-12-01T19:27:19.9814007Z error: cannot find macro `declare_tool_lint` in this scope
2019-12-01T19:27:19.9814280Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:15:1
2019-12-01T19:27:19.9814345Z    |
---
2019-12-01T19:27:19.9814965Z 
2019-12-01T19:27:19.9815008Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9815294Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:28:29
2019-12-01T19:27:19.9815359Z    |
2019-12-01T19:27:19.9815408Z LL | declare_lint_pass!(Pass => [TEST_LINT, TEST_GROUP, TEST_RUSTC_TOOL_LINT]);
2019-12-01T19:27:19.9815518Z 
2019-12-01T19:27:19.9815562Z error[E0425]: cannot find value `TEST_GROUP` in this scope
2019-12-01T19:27:19.9815832Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:28:40
2019-12-01T19:27:19.9815899Z    |
2019-12-01T19:27:19.9815899Z    |
2019-12-01T19:27:19.9815948Z LL | declare_lint_pass!(Pass => [TEST_LINT, TEST_GROUP, TEST_RUSTC_TOOL_LINT]);
2019-12-01T19:27:19.9816052Z 
2019-12-01T19:27:19.9816100Z error[E0425]: cannot find value `TEST_RUSTC_TOOL_LINT` in this scope
2019-12-01T19:27:19.9816366Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:28:52
2019-12-01T19:27:19.9816431Z    |
2019-12-01T19:27:19.9816431Z    |
2019-12-01T19:27:19.9816580Z LL | declare_lint_pass!(Pass => [TEST_LINT, TEST_GROUP, TEST_RUSTC_TOOL_LINT]);
2019-12-01T19:27:19.9816671Z 
2019-12-01T19:27:19.9816733Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9817029Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:33:26
2019-12-01T19:27:19.9817078Z    |
2019-12-01T19:27:19.9817078Z    |
2019-12-01T19:27:19.9817361Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T19:27:19.9817448Z 
2019-12-01T19:27:19.9817493Z error[E0425]: cannot find value `TEST_GROUP` in this scope
2019-12-01T19:27:19.9817777Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:36:26
2019-12-01T19:27:19.9817826Z    |
2019-12-01T19:27:19.9817826Z    |
2019-12-01T19:27:19.9818093Z LL |             cx.span_lint(TEST_GROUP, it.span, "item is named 'lintmetoo'");
2019-12-01T19:27:19.9818297Z 
2019-12-01T19:27:19.9818345Z error[E0425]: cannot find value `TEST_RUSTC_TOOL_LINT` in this scope
2019-12-01T19:27:19.9818820Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:43:38
2019-12-01T19:27:19.9818879Z    |
2019-12-01T19:27:19.9818879Z    |
2019-12-01T19:27:19.9818929Z LL |     reg.lint_store.register_lints(&[&TEST_RUSTC_TOOL_LINT, &TEST_LINT, &TEST_GROUP]);
2019-12-01T19:27:19.9819037Z 
2019-12-01T19:27:19.9819081Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9819387Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:43:61
2019-12-01T19:27:19.9819454Z    |
2019-12-01T19:27:19.9819454Z    |
2019-12-01T19:27:19.9819504Z LL |     reg.lint_store.register_lints(&[&TEST_RUSTC_TOOL_LINT, &TEST_LINT, &TEST_GROUP]);
2019-12-01T19:27:19.9819630Z 
2019-12-01T19:27:19.9819675Z error[E0425]: cannot find value `TEST_GROUP` in this scope
2019-12-01T19:27:19.9819945Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:43:73
2019-12-01T19:27:19.9820011Z    |
2019-12-01T19:27:19.9820011Z    |
2019-12-01T19:27:19.9820248Z LL |     reg.lint_store.register_lints(&[&TEST_RUSTC_TOOL_LINT, &TEST_LINT, &TEST_GROUP]);
2019-12-01T19:27:19.9820347Z 
2019-12-01T19:27:19.9820415Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T19:27:19.9820734Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:46:26
2019-12-01T19:27:19.9820783Z    |
2019-12-01T19:27:19.9820783Z    |
2019-12-01T19:27:19.9820848Z LL |         vec![LintId::of(&TEST_LINT), LintId::of(&TEST_GROUP)]);
2019-12-01T19:27:19.9820941Z 
2019-12-01T19:27:19.9821013Z error[E0425]: cannot find value `TEST_GROUP` in this scope
2019-12-01T19:27:19.9821289Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:46:50
2019-12-01T19:27:19.9821336Z    |
2019-12-01T19:27:19.9821336Z    |
2019-12-01T19:27:19.9821381Z LL |         vec![LintId::of(&TEST_LINT), LintId::of(&TEST_GROUP)]);
2019-12-01T19:27:19.9821484Z 
2019-12-01T19:27:19.9821484Z 
2019-12-01T19:27:19.9821877Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T19:27:19.9822159Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:41:1
2019-12-01T19:27:19.9822248Z LL | #[plugin_registrar]
2019-12-01T19:27:19.9822318Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T19:27:19.9822362Z    |
2019-12-01T19:27:19.9822406Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T19:27:19.9827656Z test result: FAILED. 54 passed; 13 failed; 0 ignored; 0 measured; 0 filtered out
2019-12-01T19:27:19.9827695Z 
2019-12-01T19:27:19.9827739Z 
2019-12-01T19:27:19.9827765Z 
2019-12-01T19:27:19.9829358Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-01T19:27:19.9829627Z 
2019-12-01T19:27:19.9829663Z 
2019-12-01T19:27:19.9829710Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-01T19:27:19.9829780Z Build completed unsuccessfully in 1:08:09
2019-12-01T19:27:19.9829780Z Build completed unsuccessfully in 1:08:09
2019-12-01T19:27:19.9834302Z == clock drift check ==
2019-12-01T19:27:19.9834360Z   local time: Sun Dec  1 19:27:19 UTC 2019
2019-12-01T19:27:20.2591431Z   network time: Sun, 01 Dec 2019 19:27:20 GMT
2019-12-01T19:27:20.2591559Z == end clock drift check ==
2019-12-01T19:27:21.8259791Z 
2019-12-01T19:27:21.8334741Z ##[error]Bash exited with code '1'.
2019-12-01T19:27:21.8395118Z ##[section]Starting: Checkout
2019-12-01T19:27:21.8396857Z ==============================================================================
2019-12-01T19:27:21.8396928Z Task         : Get sources
2019-12-01T19:27:21.8396976Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
