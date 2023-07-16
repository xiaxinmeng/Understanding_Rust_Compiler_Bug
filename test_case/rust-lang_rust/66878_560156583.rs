plain
2019-12-01T19:29:32.5545564Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-01T19:29:33.1713833Z ##[command]git config gc.auto 0
2019-12-01T19:29:33.1718076Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-01T19:29:33.1724256Z ##[command]git config --get-all http.proxy
2019-12-01T19:29:33.1730339Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66878/merge:refs/remotes/pull/66878/merge
---
2019-12-01T20:28:30.9381134Z .................................................................................................... 1600/9316
2019-12-01T20:28:35.5400256Z .................................................................................................... 1700/9316
2019-12-01T20:28:47.8065804Z ........................................i........................................................... 1800/9316
2019-12-01T20:28:55.4938402Z .................................................................................................... 1900/9316
2019-12-01T20:29:08.9524897Z .........................iiiii...................................................................... 2000/9316
2019-12-01T20:29:18.8639296Z .................................................................................................... 2200/9316
2019-12-01T20:29:21.3899722Z .................................................................................................... 2300/9316
2019-12-01T20:29:25.8395731Z .................................................................................................... 2400/9316
2019-12-01T20:29:46.8392646Z .................................................................................................... 2500/9316
---
2019-12-01T20:32:22.9484031Z ...........................i...............i........................................................ 4800/9316
2019-12-01T20:32:33.2606809Z .................................................................................................... 4900/9316
2019-12-01T20:32:39.2741900Z .................................................................................................... 5000/9316
2019-12-01T20:32:47.2093006Z .................................................................................................... 5100/9316
2019-12-01T20:32:54.5690413Z .................................ii.ii...........i.................................................. 5200/9316
2019-12-01T20:33:03.9876572Z .................................................................................................... 5400/9316
2019-12-01T20:33:13.4769976Z .................................................................................................... 5500/9316
2019-12-01T20:33:20.5929470Z ...............i.................................................................................... 5600/9316
2019-12-01T20:33:26.6348763Z .................................................................................................... 5700/9316
2019-12-01T20:33:26.6348763Z .................................................................................................... 5700/9316
2019-12-01T20:33:37.7625543Z .................................................................................................... 5800/9316
2019-12-01T20:33:49.6221837Z .ii...i..ii...........i............................................................................. 5900/9316
2019-12-01T20:34:07.6799804Z .................................................................................................... 6100/9316
2019-12-01T20:34:14.9961564Z .................................................................................................... 6200/9316
2019-12-01T20:34:14.9961564Z .................................................................................................... 6200/9316
2019-12-01T20:34:28.3678589Z ........................i..ii....................................................................... 6300/9316
2019-12-01T20:34:47.6185339Z ...............................................................................................i.... 6500/9316
2019-12-01T20:34:49.8454200Z .................................................................................................... 6600/9316
2019-12-01T20:34:52.0793733Z ......................................................................................i............. 6700/9316
2019-12-01T20:34:54.7481366Z .................................................................................................... 6800/9316
---
2019-12-01T20:40:00.6000125Z  finished in 6.065
2019-12-01T20:40:00.6191732Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T20:40:00.8066946Z 
2019-12-01T20:40:00.8067475Z running 164 tests
2019-12-01T20:40:03.7438905Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/164
2019-12-01T20:40:05.6745691Z .i.i..i.iii.iiiiiii............i.........iii.i..........ii......
2019-12-01T20:40:05.6747861Z 
2019-12-01T20:40:05.6749445Z  finished in 5.055
2019-12-01T20:40:05.6936548Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T20:40:05.8695773Z 
---
2019-12-01T20:40:07.8017459Z  finished in 2.108
2019-12-01T20:40:07.8204550Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T20:40:07.9829175Z 
2019-12-01T20:40:07.9829948Z running 9 tests
2019-12-01T20:40:07.9830774Z iiiiiiiii
2019-12-01T20:40:07.9831585Z 
2019-12-01T20:40:07.9831873Z  finished in 0.162
2019-12-01T20:40:08.0019887Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T20:40:08.1846698Z 
---
2019-12-01T20:40:26.9282380Z  finished in 18.926
2019-12-01T20:40:26.9499871Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T20:40:27.1314455Z 
2019-12-01T20:40:27.1314735Z running 124 tests
2019-12-01T20:40:50.8605942Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i....ii...i.......ii 100/124
2019-12-01T20:40:55.7734951Z .i.i.i......iii.i.....ii
2019-12-01T20:40:55.7736067Z 
2019-12-01T20:40:55.7738009Z  finished in 28.824
2019-12-01T20:40:55.7746821Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T20:40:55.7748069Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-12-01T20:41:34.8189147Z failures:
2019-12-01T20:41:34.8189507Z 
2019-12-01T20:41:34.8189919Z ---- [ui] ui-fulldeps/issue-15778-pass.rs stdout ----
2019-12-01T20:41:34.8189958Z 
2019-12-01T20:41:34.8190534Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" failed to compile: 
2019-12-01T20:41:34.8190777Z status: exit code: 1
2019-12-01T20:41:34.8191789Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary"
2019-12-01T20:41:34.8192457Z ------------------------------------------
2019-12-01T20:41:34.8192494Z 
2019-12-01T20:41:34.8192998Z ------------------------------------------
2019-12-01T20:41:34.8193051Z stderr:
2019-12-01T20:41:34.8193051Z stderr:
2019-12-01T20:41:34.8193511Z ------------------------------------------
2019-12-01T20:41:34.8193714Z error: cannot find macro `declare_lint` in this scope
2019-12-01T20:41:34.8194114Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:40:1
2019-12-01T20:41:34.8194202Z    |
2019-12-01T20:41:34.8194389Z LL | declare_lint!(CRATE_NOT_OKAY, Warn, "crate not marked with #![crate_okay]");
2019-12-01T20:41:34.8194743Z 
2019-12-01T20:41:34.8194791Z error: cannot find macro `declare_lint` in this scope
2019-12-01T20:41:34.8195646Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:41:1
2019-12-01T20:41:34.8195944Z    |
2019-12-01T20:41:34.8195944Z    |
2019-12-01T20:41:34.8196354Z LL | declare_lint!(CRATE_NOT_RED, Warn, "crate not marked with #![crate_red]");
2019-12-01T20:41:34.8196573Z 
2019-12-01T20:41:34.8196638Z error: cannot find macro `declare_lint` in this scope
2019-12-01T20:41:34.8197042Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:42:1
2019-12-01T20:41:34.8197094Z    |
2019-12-01T20:41:34.8197094Z    |
2019-12-01T20:41:34.8197142Z LL | declare_lint!(CRATE_NOT_BLUE, Warn, "crate not marked with #![crate_blue]");
2019-12-01T20:41:34.8197405Z 
2019-12-01T20:41:34.8197578Z error: cannot find macro `declare_lint` in this scope
2019-12-01T20:41:34.8197915Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:43:1
2019-12-01T20:41:34.8198145Z    |
2019-12-01T20:41:34.8198145Z    |
2019-12-01T20:41:34.8198292Z LL | declare_lint!(CRATE_NOT_GREY, Warn, "crate not marked with #![crate_grey]");
2019-12-01T20:41:34.8198422Z 
2019-12-01T20:41:34.8198489Z error: cannot find macro `declare_lint` in this scope
2019-12-01T20:41:34.8198819Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:44:1
2019-12-01T20:41:34.8199039Z    |
2019-12-01T20:41:34.8199039Z    |
2019-12-01T20:41:34.8199184Z LL | declare_lint!(CRATE_NOT_GREEN, Warn, "crate not marked with #![crate_green]");
2019-12-01T20:41:34.8199316Z 
2019-12-01T20:41:34.8199382Z error[E0425]: cannot find value `CRATE_NOT_OKAY` in this scope
2019-12-01T20:41:34.8199730Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:30:38
2019-12-01T20:41:34.8199959Z    |
2019-12-01T20:41:34.8199959Z    |
2019-12-01T20:41:34.8200120Z LL |                           cx.span_lint(CRATE_NOT_OKAY, krate.span,
2019-12-01T20:41:34.8200355Z ...
2019-12-01T20:41:34.8200355Z ...
2019-12-01T20:41:34.8200513Z LL | / fake_lint_pass! {
2019-12-01T20:41:34.8200558Z LL | |     PassOkay,
2019-12-01T20:41:34.8200646Z LL | |     Symbol::intern("rustc_crate_okay")
2019-12-01T20:41:34.8201030Z    | |_- in this macro invocation
2019-12-01T20:41:34.8201204Z 
2019-12-01T20:41:34.8201378Z error[E0425]: cannot find value `CRATE_NOT_OKAY` in this scope
2019-12-01T20:41:34.8201725Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:30:38
2019-12-01T20:41:34.8201725Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:30:38
2019-12-01T20:41:34.8201942Z    |
2019-12-01T20:41:34.8202085Z LL |                           cx.span_lint(CRATE_NOT_OKAY, krate.span,
2019-12-01T20:41:34.8202513Z ...
2019-12-01T20:41:34.8202513Z ...
2019-12-01T20:41:34.8202620Z LL | / fake_lint_pass! {
2019-12-01T20:41:34.8202680Z LL | |     PassRedBlue,
2019-12-01T20:41:34.8202763Z LL | |     Symbol::intern("rustc_crate_red"), Symbol::intern("rustc_crate_blue")
2019-12-01T20:41:34.8203125Z    | |_- in this macro invocation
2019-12-01T20:41:34.8203311Z 
2019-12-01T20:41:34.8203467Z error[E0425]: cannot find value `CRATE_NOT_OKAY` in this scope
2019-12-01T20:41:34.8203834Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:30:38
2019-12-01T20:41:34.8203834Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:30:38
2019-12-01T20:41:34.8204033Z    |
2019-12-01T20:41:34.8204175Z LL |                           cx.span_lint(CRATE_NOT_OKAY, krate.span,
2019-12-01T20:41:34.8204327Z ...
2019-12-01T20:41:34.8204327Z ...
2019-12-01T20:41:34.8204368Z LL | / fake_lint_pass! {
2019-12-01T20:41:34.8204425Z LL | |     PassGreyGreen,
2019-12-01T20:41:34.8204596Z LL | |     Symbol::intern("rustc_crate_grey"), Symbol::intern("rustc_crate_green")
2019-12-01T20:41:34.8204961Z    | |_- in this macro invocation
2019-12-01T20:41:34.8204997Z 
2019-12-01T20:41:34.8205043Z error[E0425]: cannot find value `CRATE_NOT_OKAY` in this scope
2019-12-01T20:41:34.8205641Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:64:10
2019-12-01T20:41:34.8205641Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:64:10
2019-12-01T20:41:34.8205720Z    |
2019-12-01T20:41:34.8206008Z LL |         &CRATE_NOT_OKAY,
2019-12-01T20:41:34.8206140Z 
2019-12-01T20:41:34.8206188Z error[E0425]: cannot find value `CRATE_NOT_RED` in this scope
2019-12-01T20:41:34.8206539Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:65:10
2019-12-01T20:41:34.8206609Z    |
2019-12-01T20:41:34.8206609Z    |
2019-12-01T20:41:34.8206808Z LL |         &CRATE_NOT_RED,
2019-12-01T20:41:34.8206938Z    |          ^^^^^^^^^^^^^ not found in this scope
2019-12-01T20:41:34.8207022Z 
2019-12-01T20:41:34.8207098Z error[E0425]: cannot find value `CRATE_NOT_BLUE` in this scope
2019-12-01T20:41:34.8207475Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:66:10
2019-12-01T20:41:34.8207543Z    |
2019-12-01T20:41:34.8207745Z LL |         &CRATE_NOT_BLUE,
2019-12-01T20:41:34.8207923Z 
2019-12-01T20:41:34.8207923Z 
2019-12-01T20:41:34.8208011Z error[E0425]: cannot find value `CRATE_NOT_GREY` in this scope
2019-12-01T20:41:34.8208357Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:67:10
2019-12-01T20:41:34.8208408Z    |
2019-12-01T20:41:34.8208620Z LL |         &CRATE_NOT_GREY,
2019-12-01T20:41:34.8208793Z 
2019-12-01T20:41:34.8208883Z error[E0425]: cannot find value `CRATE_NOT_GREEN` in this scope
2019-12-01T20:41:34.8209225Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:68:10
2019-12-01T20:41:34.8209287Z    |
2019-12-01T20:41:34.8209287Z    |
2019-12-01T20:41:34.8209510Z LL |         &CRATE_NOT_GREEN,
2019-12-01T20:41:34.8209661Z 
2019-12-01T20:41:34.8209722Z warning: unused `#[macro_use]` import
2019-12-01T20:41:34.8210209Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:6:1
2019-12-01T20:41:34.8210422Z    |
2019-12-01T20:41:34.8210422Z    |
2019-12-01T20:41:34.8210545Z LL | #[macro_use] extern crate rustc;
2019-12-01T20:41:34.8210619Z    | ^^^^^^^^^^^^
2019-12-01T20:41:34.8210661Z    |
2019-12-01T20:41:34.8210723Z    = note: `#[warn(unused_imports)]` on by default
2019-12-01T20:41:34.8210869Z 
2019-12-01T20:41:34.8211497Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T20:41:34.8212046Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:61:1
2019-12-01T20:41:34.8212395Z LL | #[plugin_registrar]
2019-12-01T20:41:34.8212449Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T20:41:34.8212648Z    |
2019-12-01T20:41:34.8212870Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T20:41:34.8214087Z 
2019-12-01T20:41:34.8214264Z 
2019-12-01T20:41:34.8214615Z ---- [ui] ui-fulldeps/issue-40001.rs stdout ----
2019-12-01T20:41:34.8214651Z 
2019-12-01T20:41:34.8215240Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
2019-12-01T20:41:34.8215306Z status: exit code: 1
2019-12-01T20:41:34.8216532Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
2019-12-01T20:41:34.8217264Z ------------------------------------------
2019-12-01T20:41:34.8217449Z 
2019-12-01T20:41:34.8217854Z ------------------------------------------
2019-12-01T20:41:34.8218075Z stderr:
2019-12-01T20:41:34.8218075Z stderr:
2019-12-01T20:41:34.8218414Z ------------------------------------------
2019-12-01T20:41:34.8218467Z error: cannot find macro `declare_lint` in this scope
2019-12-01T20:41:34.8218945Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:21:1
2019-12-01T20:41:34.8219284Z LL | declare_lint! {
2019-12-01T20:41:34.8219372Z    | ^^^^^^^^^^^^
2019-12-01T20:41:34.8219418Z 
2019-12-01T20:41:34.8219486Z error[E0425]: cannot find value `MISSING_WHITELISTED_ATTR` in this scope
2019-12-01T20:41:34.8219486Z error[E0425]: cannot find value `MISSING_WHITELISTED_ATTR` in this scope
2019-12-01T20:41:34.8219831Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:17:38
2019-12-01T20:41:34.8220035Z    |
2019-12-01T20:41:34.8220156Z LL |     reg.lint_store.register_lints(&[&MISSING_WHITELISTED_ATTR]);
2019-12-01T20:41:34.8220302Z 
2019-12-01T20:41:34.8220372Z error[E0425]: cannot find value `MISSING_WHITELISTED_ATTR` in this scope
2019-12-01T20:41:34.8220372Z error[E0425]: cannot find value `MISSING_WHITELISTED_ATTR` in this scope
2019-12-01T20:41:34.8220710Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:27:51
2019-12-01T20:41:34.8220760Z    |
2019-12-01T20:41:34.8220964Z LL | declare_lint_pass!(MissingWhitelistedAttrPass => [MISSING_WHITELISTED_ATTR]);
2019-12-01T20:41:34.8222248Z 
2019-12-01T20:41:34.8222299Z error[E0425]: cannot find value `MISSING_WHITELISTED_ATTR` in this scope
2019-12-01T20:41:34.8222299Z error[E0425]: cannot find value `MISSING_WHITELISTED_ATTR` in this scope
2019-12-01T20:41:34.8222837Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:45:26
2019-12-01T20:41:34.8223047Z    |
2019-12-01T20:41:34.8223232Z LL |             cx.span_lint(MISSING_WHITELISTED_ATTR, span,
2019-12-01T20:41:34.8223743Z 
2019-12-01T20:41:34.8223743Z 
2019-12-01T20:41:34.8224237Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T20:41:34.8224529Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:15:1
2019-12-01T20:41:34.8224635Z LL | #[plugin_registrar]
2019-12-01T20:41:34.8224683Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T20:41:34.8224885Z    |
2019-12-01T20:41:34.8225203Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T20:41:34.8225984Z 
2019-12-01T20:41:34.8226009Z 
2019-12-01T20:41:34.8226270Z ---- [ui] ui-fulldeps/lint-group-plugin-deny-cmdline.rs stdout ----
2019-12-01T20:41:34.8226305Z 
2019-12-01T20:41:34.8226595Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
2019-12-01T20:41:34.8226964Z status: exit code: 1
2019-12-01T20:41:34.8228130Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary"
2019-12-01T20:41:34.8228774Z ------------------------------------------
2019-12-01T20:41:34.8228950Z 
2019-12-01T20:41:34.8229347Z ------------------------------------------
2019-12-01T20:41:34.8229548Z stderr:
2019-12-01T20:41:34.8229548Z stderr:
2019-12-01T20:41:34.8231183Z ------------------------------------------
2019-12-01T20:41:34.8231429Z error: cannot find macro `declare_lint` in this scope
2019-12-01T20:41:34.8231788Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:15:1
2019-12-01T20:41:34.8231862Z    |
2019-12-01T20:41:34.8232321Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T20:41:34.8232662Z 
2019-12-01T20:41:34.8232731Z error: cannot find macro `declare_lint` in this scope
2019-12-01T20:41:34.8233076Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:17:1
2019-12-01T20:41:34.8233306Z    |
2019-12-01T20:41:34.8233306Z    |
2019-12-01T20:41:34.8233646Z LL | declare_lint!(PLEASE_LINT, Warn, "Warn about items named 'pleaselintme'");
2019-12-01T20:41:34.8233883Z 
2019-12-01T20:41:34.8233991Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8234334Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:19:29
2019-12-01T20:41:34.8234547Z    |
2019-12-01T20:41:34.8234547Z    |
2019-12-01T20:41:34.8234698Z LL | declare_lint_pass!(Pass => [TEST_LINT, PLEASE_LINT]);
2019-12-01T20:41:34.8234820Z 
2019-12-01T20:41:34.8234892Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T20:41:34.8235706Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:19:40
2019-12-01T20:41:34.8235788Z    |
2019-12-01T20:41:34.8235788Z    |
2019-12-01T20:41:34.8236033Z LL | declare_lint_pass!(Pass => [TEST_LINT, PLEASE_LINT]);
2019-12-01T20:41:34.8236239Z 
2019-12-01T20:41:34.8236303Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8236706Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:24:38
2019-12-01T20:41:34.8236935Z    |
2019-12-01T20:41:34.8236935Z    |
2019-12-01T20:41:34.8237375Z LL |             "lintme" => cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'"),
2019-12-01T20:41:34.8237635Z 
2019-12-01T20:41:34.8237794Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T20:41:34.8238122Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:25:44
2019-12-01T20:41:34.8238528Z    |
2019-12-01T20:41:34.8238528Z    |
2019-12-01T20:41:34.8257067Z LL |             "pleaselintme" => cx.span_lint(PLEASE_LINT, it.span, "item is named 'pleaselintme'"),
2019-12-01T20:41:34.8257620Z 
2019-12-01T20:41:34.8257699Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8258127Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:33:38
2019-12-01T20:41:34.8258183Z    |
2019-12-01T20:41:34.8258183Z    |
2019-12-01T20:41:34.8258411Z LL |     reg.lint_store.register_lints(&[&TEST_LINT, &PLEASE_LINT]);
2019-12-01T20:41:34.8258627Z 
2019-12-01T20:41:34.8258672Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T20:41:34.8259030Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:33:50
2019-12-01T20:41:34.8259102Z    |
2019-12-01T20:41:34.8259102Z    |
2019-12-01T20:41:34.8259282Z LL |     reg.lint_store.register_lints(&[&TEST_LINT, &PLEASE_LINT]);
2019-12-01T20:41:34.8259642Z 
2019-12-01T20:41:34.8259689Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8260035Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:36:26
2019-12-01T20:41:34.8260103Z    |
2019-12-01T20:41:34.8260103Z    |
2019-12-01T20:41:34.8260310Z LL |         vec![LintId::of(&TEST_LINT), LintId::of(&PLEASE_LINT)]);
2019-12-01T20:41:34.8260516Z 
2019-12-01T20:41:34.8260564Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T20:41:34.8260892Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:36:50
2019-12-01T20:41:34.8261122Z    |
2019-12-01T20:41:34.8261122Z    |
2019-12-01T20:41:34.8261271Z LL |         vec![LintId::of(&TEST_LINT), LintId::of(&PLEASE_LINT)]);
2019-12-01T20:41:34.8261511Z 
2019-12-01T20:41:34.8261511Z 
2019-12-01T20:41:34.8262079Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T20:41:34.8262586Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:31:1
2019-12-01T20:41:34.8262921Z LL | #[plugin_registrar]
2019-12-01T20:41:34.8262985Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T20:41:34.8263029Z    |
2019-12-01T20:41:34.8263074Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T20:41:34.8265004Z 
2019-12-01T20:41:34.8265209Z 
2019-12-01T20:41:34.8265779Z ---- [ui] ui-fulldeps/lint-group-plugin.rs stdout ----
2019-12-01T20:41:34.8265958Z 
2019-12-01T20:41:34.8266408Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
2019-12-01T20:41:34.8266620Z status: exit code: 1
2019-12-01T20:41:34.8267572Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary"
2019-12-01T20:41:34.8268431Z ------------------------------------------
2019-12-01T20:41:34.8268617Z 
2019-12-01T20:41:34.8268937Z ------------------------------------------
2019-12-01T20:41:34.8269143Z stderr:
2019-12-01T20:41:34.8269143Z stderr:
2019-12-01T20:41:34.8269442Z ------------------------------------------
2019-12-01T20:41:34.8269635Z error: cannot find macro `declare_lint` in this scope
2019-12-01T20:41:34.8270002Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:15:1
2019-12-01T20:41:34.8270056Z    |
2019-12-01T20:41:34.8270311Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T20:41:34.8270673Z 
2019-12-01T20:41:34.8271004Z error: cannot find macro `declare_lint` in this scope
2019-12-01T20:41:34.8274847Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:17:1
2019-12-01T20:41:34.8275326Z    |
2019-12-01T20:41:34.8275326Z    |
2019-12-01T20:41:34.8275916Z LL | declare_lint!(PLEASE_LINT, Warn, "Warn about items named 'pleaselintme'");
2019-12-01T20:41:34.8276499Z 
2019-12-01T20:41:34.8276636Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8277014Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:19:29
2019-12-01T20:41:34.8279237Z    |
2019-12-01T20:41:34.8279237Z    |
2019-12-01T20:41:34.8279325Z LL | declare_lint_pass!(Pass => [TEST_LINT, PLEASE_LINT]);
2019-12-01T20:41:34.8279433Z 
2019-12-01T20:41:34.8279477Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T20:41:34.8280149Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:19:40
2019-12-01T20:41:34.8280233Z    |
2019-12-01T20:41:34.8280233Z    |
2019-12-01T20:41:34.8280468Z LL | declare_lint_pass!(Pass => [TEST_LINT, PLEASE_LINT]);
2019-12-01T20:41:34.8281950Z 
2019-12-01T20:41:34.8282028Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8282467Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:24:38
2019-12-01T20:41:34.8282675Z    |
2019-12-01T20:41:34.8282675Z    |
2019-12-01T20:41:34.8283068Z LL |             "lintme" => cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'"),
2019-12-01T20:41:34.8283441Z 
2019-12-01T20:41:34.8283518Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T20:41:34.8283869Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:25:44
2019-12-01T20:41:34.8283922Z    |
2019-12-01T20:41:34.8283922Z    |
2019-12-01T20:41:34.8284206Z LL |             "pleaselintme" => cx.span_lint(PLEASE_LINT, it.span, "item is named 'pleaselintme'"),
2019-12-01T20:41:34.8286128Z 
2019-12-01T20:41:34.8286351Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8286788Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:33:38
2019-12-01T20:41:34.8287011Z    |
2019-12-01T20:41:34.8287011Z    |
2019-12-01T20:41:34.8287177Z LL |     reg.lint_store.register_lints(&[&TEST_LINT, &PLEASE_LINT]);
2019-12-01T20:41:34.8287322Z 
2019-12-01T20:41:34.8287386Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T20:41:34.8287760Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:33:50
2019-12-01T20:41:34.8287813Z    |
2019-12-01T20:41:34.8287813Z    |
2019-12-01T20:41:34.8288017Z LL |     reg.lint_store.register_lints(&[&TEST_LINT, &PLEASE_LINT]);
2019-12-01T20:41:34.8288153Z 
2019-12-01T20:41:34.8288222Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8288789Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:36:26
2019-12-01T20:41:34.8288996Z    |
2019-12-01T20:41:34.8288996Z    |
2019-12-01T20:41:34.8289105Z LL |         vec![LintId::of(&TEST_LINT), LintId::of(&PLEASE_LINT)]);
2019-12-01T20:41:34.8289248Z 
2019-12-01T20:41:34.8289295Z error[E0425]: cannot find value `PLEASE_LINT` in this scope
2019-12-01T20:41:34.8289666Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:36:50
2019-12-01T20:41:34.8289718Z    |
2019-12-01T20:41:34.8289718Z    |
2019-12-01T20:41:34.8289909Z LL |         vec![LintId::of(&TEST_LINT), LintId::of(&PLEASE_LINT)]);
2019-12-01T20:41:34.8291076Z 
2019-12-01T20:41:34.8291076Z 
2019-12-01T20:41:34.8291695Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T20:41:34.8291996Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:31:1
2019-12-01T20:41:34.8292487Z LL | #[plugin_registrar]
2019-12-01T20:41:34.8292608Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T20:41:34.8292673Z    |
2019-12-01T20:41:34.8292734Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T20:41:34.8293965Z 
2019-12-01T20:41:34.8294053Z 
2019-12-01T20:41:34.8294379Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-allow.rs stdout ----
2019-12-01T20:41:34.8294505Z 
2019-12-01T20:41:34.8294879Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T20:41:34.8295092Z status: exit code: 1
2019-12-01T20:41:34.8296131Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary"
2019-12-01T20:41:34.8296884Z ------------------------------------------
2019-12-01T20:41:34.8296960Z 
2019-12-01T20:41:34.8297209Z ------------------------------------------
2019-12-01T20:41:34.8297274Z stderr:
2019-12-01T20:41:34.8297274Z stderr:
2019-12-01T20:41:34.8297699Z ------------------------------------------
2019-12-01T20:41:34.8297810Z error: cannot find macro `declare_lint` in this scope
2019-12-01T20:41:34.8299681Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T20:41:34.8299919Z    |
2019-12-01T20:41:34.8300380Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T20:41:34.8300614Z 
2019-12-01T20:41:34.8300739Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8301059Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T20:41:34.8301269Z    |
2019-12-01T20:41:34.8301269Z    |
2019-12-01T20:41:34.8301385Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T20:41:34.8301491Z 
2019-12-01T20:41:34.8301562Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8301918Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T20:41:34.8302066Z    |
2019-12-01T20:41:34.8302066Z    |
2019-12-01T20:41:34.8302774Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T20:41:34.8305189Z 
2019-12-01T20:41:34.8305723Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8306144Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T20:41:34.8306385Z    |
2019-12-01T20:41:34.8306385Z    |
2019-12-01T20:41:34.8306639Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T20:41:34.8306751Z 
2019-12-01T20:41:34.8306751Z 
2019-12-01T20:41:34.8307249Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T20:41:34.8307539Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T20:41:34.8307649Z LL | #[plugin_registrar]
2019-12-01T20:41:34.8307899Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T20:41:34.8308129Z    |
2019-12-01T20:41:34.8308279Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T20:41:34.8360284Z 
2019-12-01T20:41:34.8360368Z 
2019-12-01T20:41:34.8360732Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-load.rs stdout ----
2019-12-01T20:41:34.8361074Z 
2019-12-01T20:41:34.8361522Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T20:41:34.8361951Z status: exit code: 1
2019-12-01T20:41:34.8363174Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary"
2019-12-01T20:41:34.8364234Z ------------------------------------------
2019-12-01T20:41:34.8364418Z 
2019-12-01T20:41:34.8364924Z ------------------------------------------
2019-12-01T20:41:34.8365190Z stderr:
2019-12-01T20:41:34.8365190Z stderr:
2019-12-01T20:41:34.8365499Z ------------------------------------------
2019-12-01T20:41:34.8365573Z error: cannot find macro `declare_lint` in this scope
2019-12-01T20:41:34.8366038Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T20:41:34.8366269Z    |
2019-12-01T20:41:34.8366621Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T20:41:34.8366922Z 
2019-12-01T20:41:34.8366973Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8367315Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T20:41:34.8367479Z    |
2019-12-01T20:41:34.8367479Z    |
2019-12-01T20:41:34.8367635Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T20:41:34.8367753Z 
2019-12-01T20:41:34.8367827Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8368157Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T20:41:34.8368371Z    |
2019-12-01T20:41:34.8368371Z    |
2019-12-01T20:41:34.8368814Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T20:41:34.8369352Z 
2019-12-01T20:41:34.8369557Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8369933Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T20:41:34.8370153Z    |
2019-12-01T20:41:34.8370153Z    |
2019-12-01T20:41:34.8370702Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T20:41:34.8370883Z 
2019-12-01T20:41:34.8370883Z 
2019-12-01T20:41:34.8371360Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T20:41:34.8371866Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T20:41:34.8372121Z LL | #[plugin_registrar]
2019-12-01T20:41:34.8372220Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T20:41:34.8372301Z    |
2019-12-01T20:41:34.8372347Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T20:41:34.8373633Z 
2019-12-01T20:41:34.8373834Z 
2019-12-01T20:41:34.8374184Z ---- [ui] ui-fulldeps/lint-plugin-deny-attr.rs stdout ----
2019-12-01T20:41:34.8374220Z 
2019-12-01T20:41:34.8374705Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T20:41:34.8374930Z status: exit code: 1
2019-12-01T20:41:34.8376026Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary"
2019-12-01T20:41:34.8376604Z ------------------------------------------
2019-12-01T20:41:34.8376814Z 
2019-12-01T20:41:34.8377172Z ------------------------------------------
2019-12-01T20:41:34.8377223Z stderr:
2019-12-01T20:41:34.8377223Z stderr:
2019-12-01T20:41:34.8377461Z ------------------------------------------
2019-12-01T20:41:34.8377513Z error: cannot find macro `declare_lint` in this scope
2019-12-01T20:41:34.8377980Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T20:41:34.8378199Z    |
2019-12-01T20:41:34.8378541Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T20:41:34.8378792Z 
2019-12-01T20:41:34.8379059Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8379404Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T20:41:34.8379470Z    |
2019-12-01T20:41:34.8379470Z    |
2019-12-01T20:41:34.8379515Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T20:41:34.8379862Z 
2019-12-01T20:41:34.8379911Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8380254Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T20:41:34.8380471Z    |
2019-12-01T20:41:34.8380471Z    |
2019-12-01T20:41:34.8380836Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T20:41:34.8381171Z 
2019-12-01T20:41:34.8381221Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8381560Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T20:41:34.8381934Z    |
2019-12-01T20:41:34.8381934Z    |
2019-12-01T20:41:34.8382102Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T20:41:34.8382224Z 
2019-12-01T20:41:34.8382224Z 
2019-12-01T20:41:34.8382698Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T20:41:34.8383197Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T20:41:34.8383849Z LL | #[plugin_registrar]
2019-12-01T20:41:34.8383952Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T20:41:34.8384018Z    |
2019-12-01T20:41:34.8384063Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T20:41:34.8385833Z 
2019-12-01T20:41:34.8385889Z 
2019-12-01T20:41:34.8386287Z ---- [ui] ui-fulldeps/lint-plugin-deny-cmdline.rs stdout ----
2019-12-01T20:41:34.8386429Z 
2019-12-01T20:41:34.8386757Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T20:41:34.8386979Z status: exit code: 1
2019-12-01T20:41:34.8388052Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary"
2019-12-01T20:41:34.8388763Z ------------------------------------------
2019-12-01T20:41:34.8388961Z 
2019-12-01T20:41:34.8389443Z ------------------------------------------
2019-12-01T20:41:34.8389640Z stderr:
2019-12-01T20:41:34.8389640Z stderr:
2019-12-01T20:41:34.8390046Z ------------------------------------------
2019-12-01T20:41:34.8390136Z error: cannot find macro `declare_lint` in this scope
2019-12-01T20:41:34.8390447Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T20:41:34.8390650Z    |
2019-12-01T20:41:34.8391058Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T20:41:34.8391427Z 
2019-12-01T20:41:34.8391496Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8391853Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T20:41:34.8391916Z    |
2019-12-01T20:41:34.8391916Z    |
2019-12-01T20:41:34.8391967Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T20:41:34.8392282Z 
2019-12-01T20:41:34.8392351Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8392702Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T20:41:34.8392898Z    |
2019-12-01T20:41:34.8392898Z    |
2019-12-01T20:41:34.8393232Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T20:41:34.8393613Z 
2019-12-01T20:41:34.8393690Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8394024Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T20:41:34.8394076Z    |
2019-12-01T20:41:34.8394076Z    |
2019-12-01T20:41:34.8394120Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T20:41:34.8394589Z 
2019-12-01T20:41:34.8394589Z 
2019-12-01T20:41:34.8395033Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T20:41:34.8395399Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T20:41:34.8395683Z LL | #[plugin_registrar]
2019-12-01T20:41:34.8395880Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T20:41:34.8395985Z    |
2019-12-01T20:41:34.8396069Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T20:41:34.8397268Z 
2019-12-01T20:41:34.8397417Z 
2019-12-01T20:41:34.8397855Z ---- [ui] ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----
2019-12-01T20:41:34.8398058Z 
2019-12-01T20:41:34.8398561Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T20:41:34.8398767Z status: exit code: 1
2019-12-01T20:41:34.8399691Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary"
2019-12-01T20:41:34.8400254Z ------------------------------------------
2019-12-01T20:41:34.8400333Z 
2019-12-01T20:41:34.8400607Z ------------------------------------------
2019-12-01T20:41:34.8400815Z stderr:
2019-12-01T20:41:34.8400815Z stderr:
2019-12-01T20:41:34.8401204Z ------------------------------------------
2019-12-01T20:41:34.8401402Z error: cannot find macro `declare_lint` in this scope
2019-12-01T20:41:34.8401760Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T20:41:34.8401813Z    |
2019-12-01T20:41:34.8402223Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T20:41:34.8402560Z 
2019-12-01T20:41:34.8402610Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8402974Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T20:41:34.8403167Z    |
2019-12-01T20:41:34.8403167Z    |
2019-12-01T20:41:34.8403283Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T20:41:34.8403426Z 
2019-12-01T20:41:34.8403609Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8403931Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T20:41:34.8404133Z    |
2019-12-01T20:41:34.8404133Z    |
2019-12-01T20:41:34.8404484Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T20:41:34.8404739Z 
2019-12-01T20:41:34.8404801Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8405248Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T20:41:34.8405298Z    |
2019-12-01T20:41:34.8405298Z    |
2019-12-01T20:41:34.8405402Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T20:41:34.8405527Z 
2019-12-01T20:41:34.8405527Z 
2019-12-01T20:41:34.8406044Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T20:41:34.8406684Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T20:41:34.8407034Z LL | #[plugin_registrar]
2019-12-01T20:41:34.8407104Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T20:41:34.8407183Z    |
2019-12-01T20:41:34.8407302Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T20:41:34.8408435Z 
2019-12-01T20:41:34.8408493Z 
2019-12-01T20:41:34.8408838Z ---- [ui] ui-fulldeps/lint-plugin-forbid-cmdline.rs stdout ----
2019-12-01T20:41:34.8408893Z 
2019-12-01T20:41:34.8409505Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T20:41:34.8409739Z status: exit code: 1
2019-12-01T20:41:34.8410666Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary"
2019-12-01T20:41:34.8411165Z ------------------------------------------
2019-12-01T20:41:34.8411333Z 
2019-12-01T20:41:34.8411667Z ------------------------------------------
2019-12-01T20:41:34.8411867Z stderr:
2019-12-01T20:41:34.8411867Z stderr:
2019-12-01T20:41:34.8412271Z ------------------------------------------
2019-12-01T20:41:34.8412497Z error: cannot find macro `declare_lint` in this scope
2019-12-01T20:41:34.8412812Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T20:41:34.8413023Z    |
2019-12-01T20:41:34.8413398Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items named 'lintme'");
2019-12-01T20:41:34.8413480Z 
2019-12-01T20:41:34.8413699Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8414112Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T20:41:34.8414303Z    |
2019-12-01T20:41:34.8414303Z    |
2019-12-01T20:41:34.8414442Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T20:41:34.8414563Z 
2019-12-01T20:41:34.8414642Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8414994Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T20:41:34.8415219Z    |
2019-12-01T20:41:34.8415219Z    |
2019-12-01T20:41:34.8415902Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T20:41:34.8416241Z 
2019-12-01T20:41:34.8416292Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8416604Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T20:41:34.8416654Z    |
2019-12-01T20:41:34.8416654Z    |
2019-12-01T20:41:34.8416718Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T20:41:34.8417039Z 
2019-12-01T20:41:34.8417039Z 
2019-12-01T20:41:34.8417495Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T20:41:34.8417960Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T20:41:34.8418484Z LL | #[plugin_registrar]
2019-12-01T20:41:34.8418574Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T20:41:34.8418663Z    |
2019-12-01T20:41:34.8418706Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T20:41:34.8419465Z 
2019-12-01T20:41:34.8419490Z 
2019-12-01T20:41:34.8419944Z ---- [ui] ui-fulldeps/lint-plugin.rs stdout ----
2019-12-01T20:41:34.8420137Z 
2019-12-01T20:41:34.8420510Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2019-12-01T20:41:34.8420567Z status: exit code: 1
2019-12-01T20:41:34.8421657Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary"
2019-12-01T20:41:34.8422213Z ------------------------------------------
2019-12-01T20:41:34.8422384Z 
2019-12-01T20:41:34.8422817Z ------------------------------------------
2019-12-01T20:41:34.8422869Z stderr:
2019-12-01T20:41:34.8422869Z stderr:
2019-12-01T20:41:34.8423293Z ------------------------------------------
2019-12-01T20:41:34.8424006Z error: cannot find macro `declare_lint` in this scope
2019-12-01T20:41:34.8425828Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:16:1
2019-12-01T20:41:34.8425917Z    |
2019-12-01T20:41:34.8426266Z LL | declare_lint!(TEST_LINT, Warn, "Warn about items namthread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-01T20:41:34.8426476Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-01T20:41:34.8426692Z ed 'lintme'");
2019-12-01T20:41:34.8426784Z 
2019-12-01T20:41:34.8426828Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8427083Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:18:29
2019-12-01T20:41:34.8427145Z    |
2019-12-01T20:41:34.8427145Z    |
2019-12-01T20:41:34.8427190Z LL | declare_lint_pass!(Pass => [TEST_LINT]);
2019-12-01T20:41:34.8427283Z 
2019-12-01T20:41:34.8427328Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8427601Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:26
2019-12-01T20:41:34.8427649Z    |
2019-12-01T20:41:34.8427649Z    |
2019-12-01T20:41:34.8427914Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2019-12-01T20:41:34.8428000Z 
2019-12-01T20:41:34.8428057Z error[E0425]: cannot find value `TEST_LINT` in this scope
2019-12-01T20:41:34.8428310Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:30:38
2019-12-01T20:41:34.8428356Z    |
2019-12-01T20:41:34.8428356Z    |
2019-12-01T20:41:34.8428413Z LL |     reg.lint_store.register_lints(&[&TEST_LINT]);
2019-12-01T20:41:34.8428495Z 
2019-12-01T20:41:34.8428495Z 
2019-12-01T20:41:34.8428878Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2019-12-01T20:41:34.8429155Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2019-12-01T20:41:34.8429467Z LL | #[plugin_registrar]
2019-12-01T20:41:34.8429515Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2019-12-01T20:41:34.8429558Z    |
2019-12-01T20:41:34.8429602Z    = note: `#[warn(deprecated)]` on by default
---
2019-12-01T20:41:34.8433384Z test result: FAILED. 56 passed; 11 failed; 0 ignored; 0 measured; 0 filtered out
2019-12-01T20:41:34.8433421Z 
2019-12-01T20:41:34.8433445Z 
2019-12-01T20:41:34.8433470Z 
2019-12-01T20:41:34.8435083Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-01T20:41:34.8435324Z 
2019-12-01T20:41:34.8435352Z 
2019-12-01T20:41:34.8435411Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-01T20:41:34.8435460Z Build completed unsuccessfully in 1:05:59
2019-12-01T20:41:34.8435460Z Build completed unsuccessfully in 1:05:59
2019-12-01T20:41:34.8435504Z == clock drift check ==
2019-12-01T20:41:34.8435563Z   local time: Sun Dec  1 20:41:34 UTC 2019
2019-12-01T20:41:34.8435608Z   network time: Sun, 01 Dec 2019 20:41:34 GMT
2019-12-01T20:41:34.8435652Z == end clock drift check ==
2019-12-01T20:41:36.1744188Z 
2019-12-01T20:41:36.1820276Z ##[error]Bash exited with code '1'.
2019-12-01T20:41:36.1859830Z ##[section]Starting: Checkout
2019-12-01T20:41:36.1861494Z ==============================================================================
2019-12-01T20:41:36.1861701Z Task         : Get sources
2019-12-01T20:41:36.1861765Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
