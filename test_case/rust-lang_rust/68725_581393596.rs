plain
2020-02-03T11:22:29.4178831Z ========================== Starting Command Output ===========================
2020-02-03T11:22:29.4182784Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/90b70172-6a06-49c1-a6fa-2e282e7f6ae7.sh
2020-02-03T11:22:29.4183001Z 
2020-02-03T11:22:29.4187839Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-03T11:22:29.4194619Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68725/merge to s
2020-02-03T11:22:29.4196471Z Task         : Get sources
2020-02-03T11:22:29.4196507Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-03T11:22:29.4196596Z Version      : 1.0.0
2020-02-03T11:22:29.4196632Z Author       : Microsoft
---
2020-02-03T11:22:30.4457329Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-03T11:22:30.4470280Z ##[command]git config gc.auto 0
2020-02-03T11:22:30.4473287Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-03T11:22:30.4475622Z ##[command]git config --get-all http.proxy
2020-02-03T11:22:30.4483555Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68725/merge:refs/remotes/pull/68725/merge
---
2020-02-03T12:21:20.5453356Z .................................................................................................... 1700/9568
2020-02-03T12:21:25.4870760Z .................................................................................................... 1800/9568
2020-02-03T12:21:38.6165313Z ...........................i........................................................................ 1900/9568
2020-02-03T12:21:46.0018648Z .................................................................................................... 2000/9568
2020-02-03T12:22:01.1547477Z .................iiiii.............................................................................. 2100/9568
2020-02-03T12:22:12.2145173Z .................................................................................................... 2300/9568
2020-02-03T12:22:14.9228152Z .................................................................................................... 2400/9568
2020-02-03T12:22:20.2823572Z .................................................................................................... 2500/9568
2020-02-03T12:22:42.3775031Z .................................................................................................... 2600/9568
---
2020-02-03T12:25:24.6463522Z ............................................................i...............i....................... 4900/9568
2020-02-03T12:25:32.4727186Z .................................................................................................... 5000/9568
2020-02-03T12:25:40.5654758Z .................................................................................................... 5100/9568
2020-02-03T12:25:45.2299987Z ...i................................................................................................ 5200/9568
2020-02-03T12:25:56.5153164Z .............................................................................ii.ii........i...i..... 5300/9568
2020-02-03T12:26:05.0933133Z ...............i.................................................................................... 5500/9568
2020-02-03T12:26:14.7945177Z .................................................................................................... 5600/9568
2020-02-03T12:26:21.4376237Z ................................................................i................................... 5700/9568
2020-02-03T12:26:28.7640061Z .................................................................................................... 5800/9568
2020-02-03T12:26:28.7640061Z .................................................................................................... 5800/9568
2020-02-03T12:26:36.4414785Z .................................................................................................... 5900/9568
2020-02-03T12:26:45.8051793Z .......................................................ii...i..ii...........i....................... 6000/9568
2020-02-03T12:27:07.5557364Z .................................................................................................... 6200/9568
2020-02-03T12:27:15.0287000Z .................................................................................................... 6300/9568
2020-02-03T12:27:15.0287000Z .................................................................................................... 6300/9568
2020-02-03T12:27:23.2635690Z ...................................................................................i..ii............ 6400/9568
2020-02-03T12:27:49.7171880Z .................................................................................................... 6600/9568
2020-02-03T12:27:58.4112293Z ...........................................................i........................................ 6700/9568
2020-02-03T12:28:00.5375488Z .................................................................................................... 6800/9568
2020-02-03T12:28:02.8323335Z .............................................................i...................................... 6900/9568
---
2020-02-03T12:29:45.4657957Z .................................................................................................... 7600/9568
2020-02-03T12:29:50.7043922Z .................................................................................................... 7700/9568
2020-02-03T12:29:57.5731564Z .................................................................................................... 7800/9568
2020-02-03T12:30:08.1976587Z .................................................................................................... 7900/9568
2020-02-03T12:30:14.8283950Z .....................iiiiiii..i..................................................................... 8000/9568
2020-02-03T12:30:29.7234589Z .................................................................................................... 8200/9568
2020-02-03T12:30:39.0140451Z .................................................................................................... 8300/9568
2020-02-03T12:30:53.8641573Z .................................................................................................... 8400/9568
2020-02-03T12:31:00.8661956Z .................................................................................................... 8500/9568
---
2020-02-03T12:33:29.3573540Z  finished in 7.820
2020-02-03T12:33:29.3797320Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-03T12:33:29.5830538Z 
2020-02-03T12:33:29.5831955Z running 172 tests
2020-02-03T12:33:32.7461642Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/172
2020-02-03T12:33:35.2293554Z ...i.i.i...iii..iiiiiiiiii.......................iii............ii......
2020-02-03T12:33:35.2296812Z 
2020-02-03T12:33:35.2299723Z  finished in 5.850
2020-02-03T12:33:35.2518643Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-03T12:33:35.4274156Z 
---
2020-02-03T12:33:37.5554778Z  finished in 2.303
2020-02-03T12:33:37.5756986Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-03T12:33:37.7469704Z 
2020-02-03T12:33:37.7470135Z running 9 tests
2020-02-03T12:33:37.7471206Z iiiiiiiii
2020-02-03T12:33:37.7471687Z 
2020-02-03T12:33:37.7471730Z  finished in 0.171
2020-02-03T12:33:37.7735421Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-03T12:33:38.0022010Z 
---
2020-02-03T12:33:58.9303443Z  finished in 21.156
2020-02-03T12:33:58.9551003Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-03T12:33:59.1686903Z 
2020-02-03T12:33:59.1687732Z running 116 tests
2020-02-03T12:34:13.0866468Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-03T12:34:15.0163247Z ....iiii.....ii.
2020-02-03T12:34:15.0164598Z 
2020-02-03T12:34:15.0167846Z  finished in 16.061
2020-02-03T12:34:15.0172396Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-03T12:34:15.0172972Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-02-03T12:34:15.0172972Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-02-03T12:34:15.0412853Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-03T12:34:15.2306537Z 
2020-02-03T12:34:15.2306963Z running 63 tests
2020-02-03T12:34:50.6384764Z .......................F.F......F..FFFFFFFFFFF.................
2020-02-03T12:34:50.6385129Z 
2020-02-03T12:34:50.6398188Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-03T12:34:50.6402054Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-03T12:34:50.6407068Z ---- [ui] ui-fulldeps/issue-15778-fail.rs stdout ----
2020-02-03T12:34:50.6407068Z ---- [ui] ui-fulldeps/issue-15778-fail.rs stdout ----
2020-02-03T12:34:50.6407389Z 
2020-02-03T12:34:50.6407783Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" failed to compile: 
2020-02-03T12:34:50.6407847Z status: exit code: 1
2020-02-03T12:34:50.6408765Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary"
2020-02-03T12:34:50.6409703Z ------------------------------------------
2020-02-03T12:34:50.6409744Z 
2020-02-03T12:34:50.6410034Z ------------------------------------------
2020-02-03T12:34:50.6410086Z stderr:
---
2020-02-03T12:34:50.6412486Z    |
2020-02-03T12:34:50.6412556Z LL | use rustc_lint::{LateContext, LintContext, LintPass, LateLintPass, LintArray};
2020-02-03T12:34:50.6412611Z    |                               ^^^^^^^^^^^  ^^^^^^^^                ^^^^^^^^^
2020-02-03T12:34:50.6412642Z 
2020-02-03T12:34:50.6413139Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-03T12:34:50.6413466Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:35:1
2020-02-03T12:34:50.6413585Z LL | #[plugin_registrar]
2020-02-03T12:34:50.6414080Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-03T12:34:50.6414124Z    |
2020-02-03T12:34:50.6414295Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6414295Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6414333Z 
2020-02-03T12:34:50.6415181Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::LateContext<'_, '_>` in the current scope
2020-02-03T12:34:50.6415525Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:29:16
2020-02-03T12:34:50.6415579Z    |
2020-02-03T12:34:50.6415625Z LL |             cx.span_lint(CRATE_NOT_OKAY, krate.span,
2020-02-03T12:34:50.6415996Z 
2020-02-03T12:34:50.6416040Z error: aborting due to previous error
2020-02-03T12:34:50.6416069Z 
2020-02-03T12:34:50.6416372Z For more information about this error, try `rustc --explain E0599`.
2020-02-03T12:34:50.6416372Z For more information about this error, try `rustc --explain E0599`.
2020-02-03T12:34:50.6416411Z 
2020-02-03T12:34:50.6416673Z ------------------------------------------
2020-02-03T12:34:50.6416723Z 
2020-02-03T12:34:50.6416748Z 
2020-02-03T12:34:50.6417048Z ---- [ui] ui-fulldeps/issue-15778-pass.rs stdout ----
2020-02-03T12:34:50.6417096Z 
2020-02-03T12:34:50.6417426Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" failed to compile: 
2020-02-03T12:34:50.6417505Z status: exit code: 1
2020-02-03T12:34:50.6418368Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary"
2020-02-03T12:34:50.6419224Z ------------------------------------------
2020-02-03T12:34:50.6419272Z 
2020-02-03T12:34:50.6420005Z ------------------------------------------
2020-02-03T12:34:50.6420068Z stderr:
---
2020-02-03T12:34:50.6421912Z    |
2020-02-03T12:34:50.6421981Z LL | use rustc_lint::{LateContext, LintContext, LintPass, LateLintPass};
2020-02-03T12:34:50.6422038Z    |                               ^^^^^^^^^^^
2020-02-03T12:34:50.6422067Z 
2020-02-03T12:34:50.6422512Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-03T12:34:50.6423274Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:62:1
2020-02-03T12:34:50.6423411Z LL | #[plugin_registrar]
2020-02-03T12:34:50.6423460Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-03T12:34:50.6423504Z    |
2020-02-03T12:34:50.6423564Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6423564Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6423595Z 
2020-02-03T12:34:50.6424286Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::LateContext<'_, '_>` in the current scope
2020-02-03T12:34:50.6424680Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:31:28
2020-02-03T12:34:50.6424890Z    |
2020-02-03T12:34:50.6424938Z LL |                           cx.span_lint(CRATE_NOT_OKAY, krate.span,
2020-02-03T12:34:50.6425480Z ...
2020-02-03T12:34:50.6425480Z ...
2020-02-03T12:34:50.6425522Z LL | / fake_lint_pass! {
2020-02-03T12:34:50.6425564Z LL | |     PassOkay,
2020-02-03T12:34:50.6425629Z LL | |     Symbol::intern("rustc_crate_okay")
2020-02-03T12:34:50.6425965Z    | |_- in this macro invocation
2020-02-03T12:34:50.6426024Z 
2020-02-03T12:34:50.6426366Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::LateContext<'_, '_>` in the current scope
2020-02-03T12:34:50.6426672Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:31:28
2020-02-03T12:34:50.6426672Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:31:28
2020-02-03T12:34:50.6426742Z    |
2020-02-03T12:34:50.6426792Z LL |                           cx.span_lint(CRATE_NOT_OKAY, krate.span,
2020-02-03T12:34:50.6427201Z ...
2020-02-03T12:34:50.6427201Z ...
2020-02-03T12:34:50.6427255Z LL | / fake_lint_pass! {
2020-02-03T12:34:50.6427298Z LL | |     PassRedBlue,
2020-02-03T12:34:50.6427347Z LL | |     Symbol::intern("rustc_crate_red"), Symbol::intern("rustc_crate_blue")
2020-02-03T12:34:50.6427670Z    | |_- in this macro invocation
2020-02-03T12:34:50.6427707Z 
2020-02-03T12:34:50.6428061Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::LateContext<'_, '_>` in the current scope
2020-02-03T12:34:50.6428376Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:31:28
2020-02-03T12:34:50.6428376Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:31:28
2020-02-03T12:34:50.6428428Z    |
2020-02-03T12:34:50.6428474Z LL |                           cx.span_lint(CRATE_NOT_OKAY, krate.span,
2020-02-03T12:34:50.6428884Z ...
2020-02-03T12:34:50.6428884Z ...
2020-02-03T12:34:50.6428925Z LL | / fake_lint_pass! {
2020-02-03T12:34:50.6428988Z LL | |     PassRedBlue,
2020-02-03T12:34:50.6429047Z LL | |     Symbol::intern("rustc_crate_red"), Symbol::intern("rustc_crate_blue")
2020-02-03T12:34:50.6429367Z    | |_- in this macro invocation
2020-02-03T12:34:50.6429402Z 
2020-02-03T12:34:50.6429736Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::LateContext<'_, '_>` in the current scope
2020-02-03T12:34:50.6430059Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:31:28
2020-02-03T12:34:50.6430059Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:31:28
2020-02-03T12:34:50.6430112Z    |
2020-02-03T12:34:50.6430158Z LL |                           cx.span_lint(CRATE_NOT_OKAY, krate.span,
2020-02-03T12:34:50.6430551Z ...
2020-02-03T12:34:50.6430551Z ...
2020-02-03T12:34:50.6430592Z LL | / fake_lint_pass! {
2020-02-03T12:34:50.6430634Z LL | |     PassGreyGreen,
2020-02-03T12:34:50.6430714Z LL | |     Symbol::intern("rustc_crate_grey"), Symbol::intern("rustc_crate_green")
2020-02-03T12:34:50.6431029Z    | |_- in this macro invocation
2020-02-03T12:34:50.6431084Z 
2020-02-03T12:34:50.6431430Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::LateContext<'_, '_>` in the current scope
2020-02-03T12:34:50.6431737Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:31:28
2020-02-03T12:34:50.6431737Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:31:28
2020-02-03T12:34:50.6431789Z    |
2020-02-03T12:34:50.6431856Z LL |                           cx.span_lint(CRATE_NOT_OKAY, krate.span,
2020-02-03T12:34:50.6432231Z ...
2020-02-03T12:34:50.6432231Z ...
2020-02-03T12:34:50.6432292Z LL | / fake_lint_pass! {
2020-02-03T12:34:50.6432335Z LL | |     PassGreyGreen,
2020-02-03T12:34:50.6432384Z LL | |     Symbol::intern("rustc_crate_grey"), Symbol::intern("rustc_crate_green")
2020-02-03T12:34:50.6432832Z    | |_- in this macro invocation
2020-02-03T12:34:50.6432869Z 
2020-02-03T12:34:50.6432988Z error: aborting due to 5 previous errors
2020-02-03T12:34:50.6433048Z 
2020-02-03T12:34:50.6433048Z 
2020-02-03T12:34:50.6433369Z For more information about this error, try `rustc --explain E0599`.
2020-02-03T12:34:50.6433407Z 
2020-02-03T12:34:50.6433667Z ------------------------------------------
2020-02-03T12:34:50.6433724Z 
2020-02-03T12:34:50.6433752Z 
2020-02-03T12:34:50.6434022Z ---- [ui] ui-fulldeps/issue-40001.rs stdout ----
2020-02-03T12:34:50.6434058Z 
2020-02-03T12:34:50.6434400Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
2020-02-03T12:34:50.6434459Z status: exit code: 1
2020-02-03T12:34:50.6435321Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
2020-02-03T12:34:50.6435720Z ------------------------------------------
2020-02-03T12:34:50.6435779Z 
2020-02-03T12:34:50.6436047Z ------------------------------------------
2020-02-03T12:34:50.6436097Z stderr:
---
2020-02-03T12:34:50.6437471Z    |
2020-02-03T12:34:50.6437519Z LL | use rustc_lint::{LateContext, LintPass, LintArray, LateLintPass, LintContext};
2020-02-03T12:34:50.6437574Z    |                               ^^^^^^^^  ^^^^^^^^^                ^^^^^^^^^^^
2020-02-03T12:34:50.6437623Z 
2020-02-03T12:34:50.6438039Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-03T12:34:50.6438380Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:20:1
2020-02-03T12:34:50.6438477Z LL | #[plugin_registrar]
2020-02-03T12:34:50.6438554Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-03T12:34:50.6438599Z    |
2020-02-03T12:34:50.6438642Z    = note: `#[warn(deprecated)]` on by default
---
2020-02-03T12:34:50.6440662Z 
2020-02-03T12:34:50.6440689Z 
2020-02-03T12:34:50.6441013Z ---- [ui] ui-fulldeps/lint-group-plugin-deny-cmdline.rs stdout ----
2020-02-03T12:34:50.6441127Z 
2020-02-03T12:34:50.6441517Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
2020-02-03T12:34:50.6441577Z status: exit code: 1
2020-02-03T12:34:50.6442459Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary"
2020-02-03T12:34:50.6442936Z ------------------------------------------
2020-02-03T12:34:50.6443010Z 
2020-02-03T12:34:50.6443285Z ------------------------------------------
2020-02-03T12:34:50.6443335Z stderr:
---
2020-02-03T12:34:50.6444698Z    |
2020-02-03T12:34:50.6444747Z LL | use rustc_lint::{LateContext, LintContext, LintPass, LateLintPass, LintArray, LintId};
2020-02-03T12:34:50.6444825Z    |                               ^^^^^^^^^^^  ^^^^^^^^                ^^^^^^^^^
2020-02-03T12:34:50.6444859Z 
2020-02-03T12:34:50.6445266Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-03T12:34:50.6445613Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:31:1
2020-02-03T12:34:50.6445710Z LL | #[plugin_registrar]
2020-02-03T12:34:50.6445780Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-03T12:34:50.6445824Z    |
2020-02-03T12:34:50.6445868Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6445868Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6445897Z 
2020-02-03T12:34:50.6446257Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::LateContext<'_, '_>` in the current scope
2020-02-03T12:34:50.6446582Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:24:28
2020-02-03T12:34:50.6446634Z    |
2020-02-03T12:34:50.6446968Z LL |             "lintme" => cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'"),
2020-02-03T12:34:50.6447332Z 
2020-02-03T12:34:50.6447683Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::LateContext<'_, '_>` in the current scope
2020-02-03T12:34:50.6447991Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:25:34
2020-02-03T12:34:50.6448043Z    |
2020-02-03T12:34:50.6448043Z    |
2020-02-03T12:34:50.6448375Z LL |             "pleaselintme" => cx.span_lint(PLEASE_LINT, it.span, "item is named 'pleaselintme'"),
2020-02-03T12:34:50.6448848Z 
2020-02-03T12:34:50.6448893Z error: aborting due to 2 previous errors
2020-02-03T12:34:50.6448945Z 
2020-02-03T12:34:50.6449341Z For more information about this error, try `rustc --explain E0599`.
2020-02-03T12:34:50.6449341Z For more information about this error, try `rustc --explain E0599`.
2020-02-03T12:34:50.6449388Z 
2020-02-03T12:34:50.6449676Z ------------------------------------------
2020-02-03T12:34:50.6449735Z 
2020-02-03T12:34:50.6451106Z 
2020-02-03T12:34:50.6451871Z ---- [ui] ui-fulldeps/lint-group-plugin.rs stdout ----
2020-02-03T12:34:50.6451918Z 
2020-02-03T12:34:50.6452284Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
2020-02-03T12:34:50.6452345Z status: exit code: 1
2020-02-03T12:34:50.6453228Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary"
2020-02-03T12:34:50.6453633Z ------------------------------------------
2020-02-03T12:34:50.6453699Z 
2020-02-03T12:34:50.6454027Z ------------------------------------------
2020-02-03T12:34:50.6454078Z stderr:
---
2020-02-03T12:34:50.6455458Z    |
2020-02-03T12:34:50.6455508Z LL | use rustc_lint::{LateContext, LintContext, LintPass, LateLintPass, LintArray, LintId};
2020-02-03T12:34:50.6455579Z    |                               ^^^^^^^^^^^  ^^^^^^^^                ^^^^^^^^^
2020-02-03T12:34:50.6455613Z 
2020-02-03T12:34:50.6456029Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-03T12:34:50.6456417Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:31:1
2020-02-03T12:34:50.6456526Z LL | #[plugin_registrar]
2020-02-03T12:34:50.6456600Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-03T12:34:50.6456663Z    |
2020-02-03T12:34:50.6456711Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6456711Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6456743Z 
2020-02-03T12:34:50.6457145Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::LateContext<'_, '_>` in the current scope
2020-02-03T12:34:50.6457483Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:24:28
2020-02-03T12:34:50.6457539Z    |
2020-02-03T12:34:50.6457889Z LL |             "lintme" => cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'"),
2020-02-03T12:34:50.6458280Z 
2020-02-03T12:34:50.6458659Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::LateContext<'_, '_>` in the current scope
2020-02-03T12:34:50.6458997Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:25:34
2020-02-03T12:34:50.6459052Z    |
2020-02-03T12:34:50.6459052Z    |
2020-02-03T12:34:50.6461236Z LL |             "pleaselintme" => cx.span_lint(PLEASE_LINT, it.span, "item is named 'pleaselintme'"),
2020-02-03T12:34:50.6466353Z 
2020-02-03T12:34:50.6466404Z error: aborting due to 2 previous errors
2020-02-03T12:34:50.6466459Z 
2020-02-03T12:34:50.6466867Z For more information about this error, try `rustc --explain E0599`.
2020-02-03T12:34:50.6466867Z For more information about this error, try `rustc --explain E0599`.
2020-02-03T12:34:50.6466908Z 
2020-02-03T12:34:50.6467177Z ------------------------------------------
2020-02-03T12:34:50.6467237Z 
2020-02-03T12:34:50.6467262Z 
2020-02-03T12:34:50.6467548Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-allow.rs stdout ----
2020-02-03T12:34:50.6467586Z 
2020-02-03T12:34:50.6467935Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-02-03T12:34:50.6467992Z status: exit code: 1
2020-02-03T12:34:50.6468877Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary"
2020-02-03T12:34:50.6469284Z ------------------------------------------
2020-02-03T12:34:50.6469343Z 
2020-02-03T12:34:50.6469617Z ------------------------------------------
2020-02-03T12:34:50.6469668Z stderr:
---
2020-02-03T12:34:50.6471043Z    |
2020-02-03T12:34:50.6471092Z LL | use rustc_lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-02-03T12:34:50.6471162Z    |                                ^^^^^^^^^^^                           ^^^^^^^^^
2020-02-03T12:34:50.6471196Z 
2020-02-03T12:34:50.6471618Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-03T12:34:50.6471994Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-02-03T12:34:50.6472113Z LL | #[plugin_registrar]
2020-02-03T12:34:50.6480125Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-03T12:34:50.6480255Z    |
2020-02-03T12:34:50.6480300Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6480300Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6480603Z 
2020-02-03T12:34:50.6493176Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-03T12:34:50.6493681Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:16
2020-02-03T12:34:50.6493736Z    |
2020-02-03T12:34:50.6494055Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2020-02-03T12:34:50.6494380Z 
2020-02-03T12:34:50.6494444Z warning: unused import: `LintPass`
2020-02-03T12:34:50.6494718Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:45
2020-02-03T12:34:50.6494967Z    |
---
2020-02-03T12:34:50.6495910Z 
2020-02-03T12:34:50.6495952Z 
2020-02-03T12:34:50.6496211Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-load.rs stdout ----
2020-02-03T12:34:50.6496244Z 
2020-02-03T12:34:50.6496542Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-02-03T12:34:50.6496614Z status: exit code: 1
2020-02-03T12:34:50.6497473Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary"
2020-02-03T12:34:50.6497852Z ------------------------------------------
2020-02-03T12:34:50.6497902Z 
2020-02-03T12:34:50.6498149Z ------------------------------------------
2020-02-03T12:34:50.6498194Z stderr:
---
2020-02-03T12:34:50.6499457Z    |
2020-02-03T12:34:50.6499505Z LL | use rustc_lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-02-03T12:34:50.6499558Z    |                                ^^^^^^^^^^^                           ^^^^^^^^^
2020-02-03T12:34:50.6499606Z 
2020-02-03T12:34:50.6500006Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-03T12:34:50.6500302Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-02-03T12:34:50.6500422Z LL | #[plugin_registrar]
2020-02-03T12:34:50.6500477Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-03T12:34:50.6500536Z    |
2020-02-03T12:34:50.6500579Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6500579Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6500609Z 
2020-02-03T12:34:50.6500945Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-03T12:34:50.6501225Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:16
2020-02-03T12:34:50.6501273Z    |
2020-02-03T12:34:50.6501553Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2020-02-03T12:34:50.6501874Z 
2020-02-03T12:34:50.6501916Z warning: unused import: `LintPass`
2020-02-03T12:34:50.6502205Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:45
2020-02-03T12:34:50.6502352Z    |
---
2020-02-03T12:34:50.6503243Z 
2020-02-03T12:34:50.6503268Z 
2020-02-03T12:34:50.6503539Z ---- [ui] ui-fulldeps/lint-plugin-deny-attr.rs stdout ----
2020-02-03T12:34:50.6503572Z 
2020-02-03T12:34:50.6504129Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-02-03T12:34:50.6504212Z status: exit code: 1
2020-02-03T12:34:50.6505119Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary"
2020-02-03T12:34:50.6505473Z ------------------------------------------
2020-02-03T12:34:50.6505507Z 
2020-02-03T12:34:50.6505763Z ------------------------------------------
2020-02-03T12:34:50.6505808Z stderr:
---
2020-02-03T12:34:50.6507074Z    |
2020-02-03T12:34:50.6507121Z LL | use rustc_lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-02-03T12:34:50.6507173Z    |                                ^^^^^^^^^^^                           ^^^^^^^^^
2020-02-03T12:34:50.6507206Z 
2020-02-03T12:34:50.6507594Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-03T12:34:50.6507894Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-02-03T12:34:50.6508001Z LL | #[plugin_registrar]
2020-02-03T12:34:50.6508058Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-03T12:34:50.6508118Z    |
2020-02-03T12:34:50.6508162Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6508162Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6508194Z 
2020-02-03T12:34:50.6508506Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-03T12:34:50.6508799Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:16
2020-02-03T12:34:50.6508846Z    |
2020-02-03T12:34:50.6509111Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2020-02-03T12:34:50.6509445Z 
2020-02-03T12:34:50.6509487Z warning: unused import: `LintPass`
2020-02-03T12:34:50.6509899Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:45
2020-02-03T12:34:50.6509966Z    |
---
2020-02-03T12:34:50.6510893Z 
2020-02-03T12:34:50.6510918Z 
2020-02-03T12:34:50.6511191Z ---- [ui] ui-fulldeps/lint-plugin-deny-cmdline.rs stdout ----
2020-02-03T12:34:50.6511224Z 
2020-02-03T12:34:50.6511524Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-02-03T12:34:50.6511576Z status: exit code: 1
2020-02-03T12:34:50.6512457Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary"
2020-02-03T12:34:50.6512806Z ------------------------------------------
2020-02-03T12:34:50.6512840Z 
2020-02-03T12:34:50.6513077Z ------------------------------------------
2020-02-03T12:34:50.6513139Z stderr:
---
2020-02-03T12:34:50.6514372Z    |
2020-02-03T12:34:50.6514436Z LL | use rustc_lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-02-03T12:34:50.6514490Z    |                                ^^^^^^^^^^^                           ^^^^^^^^^
2020-02-03T12:34:50.6514522Z 
2020-02-03T12:34:50.6514899Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-03T12:34:50.6515195Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-02-03T12:34:50.6515310Z LL | #[plugin_registrar]
2020-02-03T12:34:50.6515358Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-03T12:34:50.6515402Z    |
2020-02-03T12:34:50.6515461Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6515461Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6515491Z 
2020-02-03T12:34:50.6515806Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-03T12:34:50.6516099Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:16
2020-02-03T12:34:50.6516145Z    |
2020-02-03T12:34:50.6516415Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2020-02-03T12:34:50.6516748Z 
2020-02-03T12:34:50.6516790Z warning: unused import: `LintPass`
2020-02-03T12:34:50.6517175Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:45
2020-02-03T12:34:50.6517240Z    |
---
2020-02-03T12:34:50.6518136Z 
2020-02-03T12:34:50.6518161Z 
2020-02-03T12:34:50.6518418Z ---- [ui] ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----
2020-02-03T12:34:50.6518470Z 
2020-02-03T12:34:50.6518769Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-02-03T12:34:50.6518832Z status: exit code: 1
2020-02-03T12:34:50.6519696Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary"
2020-02-03T12:34:50.6520043Z ------------------------------------------
2020-02-03T12:34:50.6520076Z 
2020-02-03T12:34:50.6520315Z ------------------------------------------
2020-02-03T12:34:50.6520377Z stderr:
---
2020-02-03T12:34:50.6521611Z    |
2020-02-03T12:34:50.6521658Z LL | use rustc_lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-02-03T12:34:50.6521727Z    |                                ^^^^^^^^^^^                           ^^^^^^^^^
2020-02-03T12:34:50.6521759Z 
2020-02-03T12:34:50.6522141Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-03T12:34:50.6522444Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-02-03T12:34:50.6522561Z LL | #[plugin_registrar]
2020-02-03T12:34:50.6522609Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-03T12:34:50.6522652Z    |
2020-02-03T12:34:50.6522694Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6522694Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6522742Z 
2020-02-03T12:34:50.6523056Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-03T12:34:50.6523333Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:16
2020-02-03T12:34:50.6523396Z    |
2020-02-03T12:34:50.6523663Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2020-02-03T12:34:50.6524090Z 
2020-02-03T12:34:50.6524140Z warning: unused import: `LintPass`
2020-02-03T12:34:50.6524468Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:45
2020-02-03T12:34:50.6524595Z    |
---
2020-02-03T12:34:50.6525510Z 
2020-02-03T12:34:50.6525538Z 
2020-02-03T12:34:50.6525817Z ---- [ui] ui-fulldeps/lint-plugin-forbid-cmdline.rs stdout ----
2020-02-03T12:34:50.6525853Z 
2020-02-03T12:34:50.6526193Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-02-03T12:34:50.6526259Z status: exit code: 1
2020-02-03T12:34:50.6527145Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary"
2020-02-03T12:34:50.6527537Z ------------------------------------------
2020-02-03T12:34:50.6527574Z 
2020-02-03T12:34:50.6527835Z ------------------------------------------
2020-02-03T12:34:50.6527884Z stderr:
---
2020-02-03T12:34:50.6529243Z    |
2020-02-03T12:34:50.6529294Z LL | use rustc_lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-02-03T12:34:50.6529368Z    |                                ^^^^^^^^^^^                           ^^^^^^^^^
2020-02-03T12:34:50.6529404Z 
2020-02-03T12:34:50.6529802Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-03T12:34:50.6530137Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-02-03T12:34:50.6530236Z LL | #[plugin_registrar]
2020-02-03T12:34:50.6530306Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-03T12:34:50.6530353Z    |
2020-02-03T12:34:50.6530399Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6530399Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6530432Z 
2020-02-03T12:34:50.6530786Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-03T12:34:50.6531086Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:16
2020-02-03T12:34:50.6531136Z    |
2020-02-03T12:34:50.6531444Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2020-02-03T12:34:50.6531867Z 
2020-02-03T12:34:50.6531932Z warning: unused import: `LintPass`
2020-02-03T12:34:50.6532323Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:45
2020-02-03T12:34:50.6532379Z    |
---
2020-02-03T12:34:50.6533276Z 
2020-02-03T12:34:50.6533323Z 
2020-02-03T12:34:50.6533589Z ---- [ui] ui-fulldeps/lint-plugin.rs stdout ----
2020-02-03T12:34:50.6533625Z 
2020-02-03T12:34:50.6533941Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" failed to compile: 
2020-02-03T12:34:50.6534026Z status: exit code: 1
2020-02-03T12:34:50.6541819Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary"
2020-02-03T12:34:50.6542279Z ------------------------------------------
2020-02-03T12:34:50.6542314Z 
2020-02-03T12:34:50.6542584Z ------------------------------------------
2020-02-03T12:34:50.6542630Z stderr:
---
2020-02-03T12:34:50.6547766Z    |
2020-02-03T12:34:50.6547814Z LL | use rustc_lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, LintArray};
2020-02-03T12:34:50.6547867Z    |                                ^^^^^^^^^^^                           ^^^^^^^^^
2020-02-03T12:34:50.6547917Z 
2020-02-03T12:34:50.6561529Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-03T12:34:50.6562115Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:28:1
2020-02-03T12:34:50.6562253Z LL | #[plugin_registrar]
2020-02-03T12:34:50.6562301Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-03T12:34:50.6562363Z    |
2020-02-03T12:34:50.6562406Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6562406Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6562439Z 
2020-02-03T12:34:50.6562793Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-03T12:34:50.6563078Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:23:16
2020-02-03T12:34:50.6563125Z    |
2020-02-03T12:34:50.6563392Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2020-02-03T12:34:50.6563899Z 
2020-02-03T12:34:50.6563949Z warning: unused import: `LintPass`
2020-02-03T12:34:50.6564419Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-plugin-test.rs:13:45
2020-02-03T12:34:50.6564481Z    |
---
2020-02-03T12:34:50.6565380Z 
2020-02-03T12:34:50.6565407Z 
2020-02-03T12:34:50.6565706Z ---- [ui] ui-fulldeps/lint-tool-cmdline-allow.rs stdout ----
2020-02-03T12:34:50.6565741Z 
2020-02-03T12:34:50.6566059Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" failed to compile: 
2020-02-03T12:34:50.6566143Z status: exit code: 1
2020-02-03T12:34:50.6567021Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary"
2020-02-03T12:34:50.6567403Z ------------------------------------------
2020-02-03T12:34:50.6567438Z 
2020-02-03T12:34:50.6567717Z ------------------------------------------
2020-02-03T12:34:50.6567776Z stderr:
---
2020-02-03T12:34:50.6569102Z    |
2020-02-03T12:34:50.6569171Z LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass, LintId};
2020-02-03T12:34:50.6569228Z    |                                               ^^^^^^^^^  ^^^^^^^^^^^
2020-02-03T12:34:50.6569264Z 
2020-02-03T12:34:50.6569699Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-03T12:34:50.6570014Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:41:1
2020-02-03T12:34:50.6570128Z LL | #[plugin_registrar]
2020-02-03T12:34:50.6570180Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-03T12:34:50.6570244Z    |
2020-02-03T12:34:50.6570291Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6570291Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6570323Z 
2020-02-03T12:34:50.6570657Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-03T12:34:50.6570971Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:33:16
2020-02-03T12:34:50.6571021Z    |
2020-02-03T12:34:50.6571308Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2020-02-03T12:34:50.6571765Z 
2020-02-03T12:34:50.6572190Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-03T12:34:50.6572516Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:36:16
2020-02-03T12:34:50.6572588Z    |
2020-02-03T12:34:50.6572588Z    |
2020-02-03T12:34:50.6572880Z LL |             cx.span_lint(TEST_GROUP, it.span, "item is named 'lintmetoo'");
2020-02-03T12:34:50.6573236Z 
2020-02-03T12:34:50.6573282Z warning: unused import: `LintPass`
2020-02-03T12:34:50.6573575Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:11:71
2020-02-03T12:34:50.6573643Z    |
---
2020-02-03T12:34:50.6574532Z 
2020-02-03T12:34:50.6574559Z 
2020-02-03T12:34:50.6574821Z ---- [ui] ui-fulldeps/lint-tool-test.rs stdout ----
2020-02-03T12:34:50.6574872Z 
2020-02-03T12:34:50.6575193Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" failed to compile: 
2020-02-03T12:34:50.6575249Z status: exit code: 1
2020-02-03T12:34:50.6576127Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary"
2020-02-03T12:34:50.6576513Z ------------------------------------------
2020-02-03T12:34:50.6576549Z 
2020-02-03T12:34:50.6576807Z ------------------------------------------
2020-02-03T12:34:50.6576873Z stderr:
---
2020-02-03T12:34:50.6578202Z    |
2020-02-03T12:34:50.6578253Z LL | use rustc_lint::{EarlyContext, EarlyLintPass, LintArray, LintContext, LintPass, LintId};
2020-02-03T12:34:50.6578331Z    |                                               ^^^^^^^^^  ^^^^^^^^^^^
2020-02-03T12:34:50.6578368Z 
2020-02-03T12:34:50.6578779Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-02-03T12:34:50.6579085Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:41:1
2020-02-03T12:34:50.6579197Z LL | #[plugin_registrar]
2020-02-03T12:34:50.6579250Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-03T12:34:50.6579297Z    |
2020-02-03T12:34:50.6579437Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6579437Z    = note: `#[warn(deprecated)]` on by default
2020-02-03T12:34:50.6579487Z 
2020-02-03T12:34:50.6579921Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-03T12:34:50.6580245Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:33:16
2020-02-03T12:34:50.6580315Z    |
2020-02-03T12:34:50.6580603Z LL |             cx.span_lint(TEST_LINT, it.span, "item is named 'lintme'");
2020-02-03T12:34:50.6580939Z 
2020-02-03T12:34:50.6581291Z error[E0599]: no method named `span_lint` found for reference `&rustc_lint::EarlyContext<'_>` in the current scope
2020-02-03T12:34:50.6581587Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:36:16
2020-02-03T12:34:50.6581638Z    |
2020-02-03T12:34:50.6581638Z    |
2020-02-03T12:34:50.6581947Z LL |             cx.span_lint(TEST_GROUP, it.span, "item is named 'lintmetoo'");
2020-02-03T12:34:50.6582297Z 
2020-02-03T12:34:50.6582362Z warning: unused import: `LintPass`
2020-02-03T12:34:50.6582732Z   --> /checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs:11:71
2020-02-03T12:34:50.6582783Z    |
---
2020-02-03T12:34:50.6588190Z test result: FAILED. 49 passed; 14 failed; 0 ignored; 0 measured; 0 filtered out
2020-02-03T12:34:50.6588227Z 
2020-02-03T12:34:50.6588264Z 
2020-02-03T12:34:50.6588309Z 
2020-02-03T12:34:50.6590584Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-03T12:34:50.6591141Z 
2020-02-03T12:34:50.6591177Z 
2020-02-03T12:34:50.6591231Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-03T12:34:50.6591310Z Build completed unsuccessfully in 1:06:37
2020-02-03T12:34:50.6591310Z Build completed unsuccessfully in 1:06:37
2020-02-03T12:34:50.6591365Z == clock drift check ==
2020-02-03T12:34:50.6591419Z   local time: Mon Feb  3 12:34:50 UTC 2020
2020-02-03T12:34:50.9517115Z   network time: Mon, 03 Feb 2020 12:34:50 GMT
2020-02-03T12:34:50.9517223Z == end clock drift check ==
2020-02-03T12:34:52.0196548Z 
2020-02-03T12:34:52.0301082Z ##[error]Bash exited with code '1'.
2020-02-03T12:34:52.0314441Z ##[section]Finishing: Run build
2020-02-03T12:34:52.0339983Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68725/merge to s
2020-02-03T12:34:52.0341884Z Task         : Get sources
2020-02-03T12:34:52.0341950Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-03T12:34:52.0342000Z Version      : 1.0.0
2020-02-03T12:34:52.0342046Z Author       : Microsoft
2020-02-03T12:34:52.0342046Z Author       : Microsoft
2020-02-03T12:34:52.0342113Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-03T12:34:52.0342169Z ==============================================================================
2020-02-03T12:34:52.5203389Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-03T12:34:52.5244499Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68725/merge to s
2020-02-03T12:34:52.5351134Z Cleaning up task key
2020-02-03T12:34:52.5351890Z Start cleaning up orphan processes.
2020-02-03T12:34:52.5456827Z Terminate orphan process: pid (3534) (python)
2020-02-03T12:34:52.5687836Z ##[section]Finishing: Finalize Job
