plain
2019-09-21T19:16:47.7364233Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-21T19:16:48.5947096Z ##[command]git config gc.auto 0
2019-09-21T19:16:48.5951862Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-21T19:16:48.5954107Z ##[command]git config --get-all http.proxy
2019-09-21T19:16:48.5958203Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64665/merge:refs/remotes/pull/64665/merge
---
2019-09-21T20:21:04.4559981Z .................................................................................................... 1500/9030
2019-09-21T20:21:10.5445574Z .................................................................................................... 1600/9030
2019-09-21T20:21:23.7159430Z .....................................................................i...............i.............. 1700/9030
2019-09-21T20:21:30.7170685Z .................................................................................................... 1800/9030
2019-09-21T20:21:46.3911309Z ............................................................iiiii................................... 1900/9030
2019-09-21T20:21:58.9085972Z .................................................................................................... 2100/9030
2019-09-21T20:22:01.4675743Z .................................................................................................... 2200/9030
2019-09-21T20:22:04.7276283Z .................................................................................................... 2300/9030
2019-09-21T20:22:13.3901784Z .................................................................................................... 2400/9030
---
2019-09-21T20:25:18.4405359Z ................................................i...............i................................... 4700/9030
2019-09-21T20:25:28.2615068Z .................................................................................................... 4800/9030
2019-09-21T20:25:37.2756987Z .................................................................................................... 4900/9030
2019-09-21T20:25:46.4394059Z .................................................................................................... 5000/9030
2019-09-21T20:25:54.5530594Z ..................................ii.ii............................................................. 5100/9030
2019-09-21T20:26:04.4036928Z .................................................................................................... 5300/9030
2019-09-21T20:26:15.5872745Z ..................................................................................................i. 5400/9030
2019-09-21T20:26:24.2208346Z .................................................................................................... 5500/9030
2019-09-21T20:26:29.3061393Z .................................................................................................... 5600/9030
2019-09-21T20:26:29.3061393Z .................................................................................................... 5600/9030
2019-09-21T20:26:40.5932569Z .............................................................................................ii...i. 5700/9030
2019-09-21T20:26:55.4468942Z .ii...........i..................................................................................... 5800/9030
2019-09-21T20:27:17.0932284Z .................................................................................................... 6000/9030
2019-09-21T20:27:24.7099984Z ...............................................................................................i..ii 6100/9030
2019-09-21T20:27:39.2921792Z .................................................................................................... 6200/9030
2019-09-21T20:27:53.3342608Z .................................................................................................... 6300/9030
---
2019-09-21T20:32:42.1058521Z  finished in 5.726
2019-09-21T20:32:42.1261046Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-21T20:32:42.3191845Z 
2019-09-21T20:32:42.3193511Z running 150 tests
2019-09-21T20:32:45.7310711Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-21T20:32:47.7673092Z ..iiii..............i.........iii.i.......ii......
2019-09-21T20:32:47.7673572Z 
2019-09-21T20:32:47.7678508Z  finished in 5.641
2019-09-21T20:32:47.7883939Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-21T20:32:47.9583624Z 
---
2019-09-21T20:32:50.0878688Z  finished in 2.299
2019-09-21T20:32:50.1086769Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-21T20:32:50.2627387Z 
2019-09-21T20:32:50.2628304Z running 9 tests
2019-09-21T20:32:50.2629233Z iiiiiiiii
2019-09-21T20:32:50.2630460Z 
2019-09-21T20:32:50.2630591Z  finished in 0.154
2019-09-21T20:32:50.8912820Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-21T20:32:50.8913012Z 
---
2019-09-21T20:33:08.8069922Z  finished in 18.520
2019-09-21T20:33:08.8293063Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-21T20:33:09.0021909Z 
2019-09-21T20:33:09.0022770Z running 123 tests
2019-09-21T20:33:33.1640074Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-21T20:33:37.8879838Z i.i.i......iii.i.....ii
2019-09-21T20:33:37.8900080Z 
2019-09-21T20:33:37.8907526Z  finished in 29.061
2019-09-21T20:33:37.8917998Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-21T20:33:37.8918339Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-09-21T20:33:37.8918339Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-09-21T20:33:37.9142301Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-21T20:33:38.0834083Z 
2019-09-21T20:33:38.0834919Z running 69 tests
2019-09-21T20:34:21.2411139Z ..............F.......................................FFFFFF...F....thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-21T20:34:21.2413478Z .
2019-09-21T20:34:21.2413719Z failures:
2019-09-21T20:34:21.2413793Z 
2019-09-21T20:34:21.2416138Z ---- [ui] ui-fulldeps/gated-plugin.rs stdout ----
2019-09-21T20:34:21.2416138Z ---- [ui] ui-fulldeps/gated-plugin.rs stdout ----
2019-09-21T20:34:21.2416938Z 
2019-09-21T20:34:21.2417691Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-09-21T20:34:21.2418000Z status: exit code: 1
2019-09-21T20:34:21.2418987Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary"
2019-09-21T20:34:21.2420320Z ------------------------------------------
2019-09-21T20:34:21.2420535Z 
2019-09-21T20:34:21.2420862Z ------------------------------------------
2019-09-21T20:34:21.2421033Z stderr:
2019-09-21T20:34:21.2421033Z stderr:
2019-09-21T20:34:21.2421550Z ------------------------------------------
2019-09-21T20:34:21.2422074Z error[E0599]: no method named `register_syntax_extension` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-09-21T20:34:21.2422549Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:19:9
2019-09-21T20:34:21.2422706Z    |
2019-09-21T20:34:21.2422841Z LL |     reg.register_syntax_extension(
2019-09-21T20:34:21.2423211Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut rustc_driver::rustc_plugin_impl::Registry<'_>`
2019-09-21T20:34:21.2423723Z error: aborting due to previous error
2019-09-21T20:34:21.2423837Z 
2019-09-21T20:34:21.2424346Z For more information about this error, try `rustc --explain E0599`.
2019-09-21T20:34:21.2424570Z 
2019-09-21T20:34:21.2424570Z 
2019-09-21T20:34:21.2424941Z ------------------------------------------
2019-09-21T20:34:21.2425279Z 
2019-09-21T20:34:21.2425435Z 
2019-09-21T20:34:21.2425986Z ---- [ui] ui-fulldeps/plugin-args-1.rs stdout ----
2019-09-21T20:34:21.2426684Z 
2019-09-21T20:34:21.2427191Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-09-21T20:34:21.2427401Z status: exit code: 1
2019-09-21T20:34:21.2428292Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/auxiliary"
2019-09-21T20:34:21.2429164Z ------------------------------------------
2019-09-21T20:34:21.2429331Z 
2019-09-21T20:34:21.2429722Z ------------------------------------------
2019-09-21T20:34:21.2430058Z stderr:
2019-09-21T20:34:21.2430058Z stderr:
2019-09-21T20:34:21.2430365Z ------------------------------------------
2019-09-21T20:34:21.2430767Z error[E0599]: no method named `register_syntax_extension` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-09-21T20:34:21.2431119Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:39:9
2019-09-21T20:34:21.2431302Z    |
2019-09-21T20:34:21.2431448Z LL |     reg.register_syntax_extension(Symbol::intern("plugin_args"), SyntaxExtension::default(
2019-09-21T20:34:21.2431978Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut rustc_driver::rustc_plugin_impl::Registry<'_>`
2019-09-21T20:34:21.2432226Z error: aborting due to previous error
2019-09-21T20:34:21.2432327Z 
2019-09-21T20:34:21.2432660Z For more information about this error, try `rustc --explain E0599`.
2019-09-21T20:34:21.2432789Z 
2019-09-21T20:34:21.2432789Z 
2019-09-21T20:34:21.2433074Z ------------------------------------------
2019-09-21T20:34:21.2433215Z 
2019-09-21T20:34:21.2433313Z 
2019-09-21T20:34:21.2433605Z ---- [ui] ui-fulldeps/plugin-args-2.rs stdout ----
2019-09-21T20:34:21.2433734Z 
2019-09-21T20:34:21.2434543Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-09-21T20:34:21.2434970Z status: exit code: 1
2019-09-21T20:34:21.2436802Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/auxiliary"
2019-09-21T20:34:21.2437818Z ------------------------------------------
2019-09-21T20:34:21.2438025Z 
2019-09-21T20:34:21.2438391Z ------------------------------------------
2019-09-21T20:34:21.2438603Z stderr:
2019-09-21T20:34:21.2438603Z stderr:
2019-09-21T20:34:21.2438956Z ------------------------------------------
2019-09-21T20:34:21.2439519Z error[E0599]: no method named `register_syntax_extension` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-09-21T20:34:21.2440953Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:39:9
2019-09-21T20:34:21.2442053Z    |
2019-09-21T20:34:21.2442102Z LL |     reg.register_syntax_extension(Symbol::intern("plugin_args"), SyntaxExtension::default(
2019-09-21T20:34:21.2442427Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut rustc_driver::rustc_plugin_impl::Registry<'_>`
2019-09-21T20:34:21.2442631Z error: aborting due to previous error
2019-09-21T20:34:21.2442668Z 
2019-09-21T20:34:21.2442918Z For more information about this error, try `rustc --explain E0599`.
2019-09-21T20:34:21.2442946Z 
2019-09-21T20:34:21.2442946Z 
2019-09-21T20:34:21.2443121Z ------------------------------------------
2019-09-21T20:34:21.2443146Z 
2019-09-21T20:34:21.2443166Z 
2019-09-21T20:34:21.2443367Z ---- [ui] ui-fulldeps/plugin-args-3.rs stdout ----
2019-09-21T20:34:21.2443394Z 
2019-09-21T20:34:21.2443611Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-09-21T20:34:21.2443754Z status: exit code: 1
2019-09-21T20:34:21.2444378Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/auxiliary"
2019-09-21T20:34:21.2444650Z ------------------------------------------
2019-09-21T20:34:21.2444676Z 
2019-09-21T20:34:21.2444871Z ------------------------------------------
2019-09-21T20:34:21.2444908Z stderr:
2019-09-21T20:34:21.2444908Z stderr:
2019-09-21T20:34:21.2445082Z ------------------------------------------
2019-09-21T20:34:21.2445346Z error[E0599]: no method named `register_syntax_extension` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-09-21T20:34:21.2445618Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:39:9
2019-09-21T20:34:21.2445659Z    |
2019-09-21T20:34:21.2445715Z LL |     reg.register_syntax_extension(Symbol::intern("plugin_args"), SyntaxExtension::default(
2019-09-21T20:34:21.2446528Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut rustc_driver::rustc_plugin_impl::Registry<'_>`
2019-09-21T20:34:21.2446620Z error: aborting due to previous error
2019-09-21T20:34:21.2446668Z 
2019-09-21T20:34:21.2446954Z For more information about this error, try `rustc --explain E0599`.
2019-09-21T20:34:21.2446989Z 
2019-09-21T20:34:21.2446989Z 
2019-09-21T20:34:21.2447207Z ------------------------------------------
2019-09-21T20:34:21.2447257Z 
2019-09-21T20:34:21.2447283Z 
2019-09-21T20:34:21.2447518Z ---- [ui] ui-fulldeps/plugin-as-extern-crate.rs stdout ----
2019-09-21T20:34:21.2447561Z 
2019-09-21T20:34:21.2447858Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-09-21T20:34:21.2447911Z status: exit code: 1
2019-09-21T20:34:21.2448691Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-as-extern-crate/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-as-extern-crate/auxiliary"
2019-09-21T20:34:21.2449035Z ------------------------------------------
2019-09-21T20:34:21.2449097Z 
2019-09-21T20:34:21.2449322Z ------------------------------------------
2019-09-21T20:34:21.2449367Z stderr:
2019-09-21T20:34:21.2449367Z stderr:
2019-09-21T20:34:21.2449751Z ------------------------------------------
2019-09-21T20:34:21.2450035Z error[E0599]: no method named `register_syntax_extension` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-09-21T20:34:21.2450429Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:19:9
2019-09-21T20:34:21.2450591Z    |
2019-09-21T20:34:21.2450638Z LL |     reg.register_syntax_extension(
2019-09-21T20:34:21.2451137Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut rustc_driver::rustc_plugin_impl::Registry<'_>`
2019-09-21T20:34:21.2451227Z error: aborting due to previous error
2019-09-21T20:34:21.2451253Z 
2019-09-21T20:34:21.2451469Z For more information about this error, try `rustc --explain E0599`.
2019-09-21T20:34:21.2451499Z 
2019-09-21T20:34:21.2451499Z 
2019-09-21T20:34:21.2451705Z ------------------------------------------
2019-09-21T20:34:21.2451810Z 
2019-09-21T20:34:21.2451832Z 
2019-09-21T20:34:21.2452235Z ---- [ui] ui-fulldeps/plugin-attr-register-deny.rs stdout ----
2019-09-21T20:34:21.2452284Z 
2019-09-21T20:34:21.2452536Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-09-21T20:34:21.2452584Z status: exit code: 1
2019-09-21T20:34:21.2453448Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny/auxiliary"
2019-09-21T20:34:21.2453912Z ------------------------------------------
2019-09-21T20:34:21.2453939Z 
2019-09-21T20:34:21.2454125Z ------------------------------------------
2019-09-21T20:34:21.2454163Z stderr:
2019-09-21T20:34:21.2454163Z stderr:
2019-09-21T20:34:21.2454539Z ------------------------------------------
2019-09-21T20:34:21.2454981Z error[E0599]: no method named `register_syntax_extension` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-09-21T20:34:21.2455205Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:19:9
2019-09-21T20:34:21.2455265Z    |
2019-09-21T20:34:21.2455483Z LL |     reg.register_syntax_extension(
2019-09-21T20:34:21.2455734Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut rustc_driver::rustc_plugin_impl::Registry<'_>`
2019-09-21T20:34:21.2455992Z error: aborting due to previous error
2019-09-21T20:34:21.2456018Z 
2019-09-21T20:34:21.2457008Z For more information about this error, try `rustc --explain E0599`.
2019-09-21T20:34:21.2457078Z 
2019-09-21T20:34:21.2457078Z 
2019-09-21T20:34:21.2457304Z ------------------------------------------
2019-09-21T20:34:21.2457335Z 
2019-09-21T20:34:21.2457361Z 
2019-09-21T20:34:21.2457604Z ---- [ui] ui-fulldeps/plugin-reexport.rs stdout ----
2019-09-21T20:34:21.2457637Z 
2019-09-21T20:34:21.2457919Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-09-21T20:34:21.2457980Z status: exit code: 1
2019-09-21T20:34:21.2458754Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-reexport/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-reexport/auxiliary"
2019-09-21T20:34:21.2459096Z ------------------------------------------
2019-09-21T20:34:21.2459129Z 
2019-09-21T20:34:21.2459351Z ------------------------------------------
2019-09-21T20:34:21.2459412Z stderr:
2019-09-21T20:34:21.2459412Z stderr:
2019-09-21T20:34:21.2459628Z ------------------------------------------
2019-09-21T20:34:21.2460208Z error[E0599]: no method named `register_syntax_extension` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-09-21T20:34:21.2460682Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:19:9
2019-09-21T20:34:21.2460722Z    |
2019-09-21T20:34:21.2460755Z LL |     reg.register_syntax_extension(
2019-09-21T20:34:21.2461004Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut rustc_driver::rustc_plugin_impl::Registry<'_>`
2019-09-21T20:34:21.2461246Z error: aborting due to previous error
2019-09-21T20:34:21.2461345Z 
2019-09-21T20:34:21.2461586Z For more information about this error, try `rustc --explain E0599`.
2019-09-21T20:34:21.2461614Z 
2019-09-21T20:34:21.2461614Z 
2019-09-21T20:34:21.2462190Z ------------------------------------------
2019-09-21T20:34:21.2462218Z 
2019-09-21T20:34:21.2462257Z 
2019-09-21T20:34:21.2462459Z ---- [ui] ui-fulldeps/roman-numerals-macro.rs stdout ----
2019-09-21T20:34:21.2462488Z 
2019-09-21T20:34:21.2462736Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" failed to compile: 
2019-09-21T20:34:21.2462800Z status: exit code: 1
2019-09-21T20:34:21.2463467Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary"
2019-09-21T20:34:21.2463767Z ------------------------------------------
2019-09-21T20:34:21.2463795Z 
2019-09-21T20:34:21.2464007Z ------------------------------------------
2019-09-21T20:34:21.2464046Z stderr:
2019-09-21T20:34:21.2464046Z stderr:
2019-09-21T20:34:21.2464243Z ------------------------------------------
2019-09-21T20:34:21.2464535Z error[E0599]: no method named `register_macro` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-09-21T20:34:21.2464922Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:67:9
2019-09-21T20:34:21.2464964Z    |
2019-09-21T20:34:21.2465178Z LL |     reg.register_macro("rn", expand_rn);
2019-09-21T20:34:21.2465560Z    |         ^^^^^^^^^^^^^^ method not found in `&mut rustc_driver::rustc_plugin_impl::Registry<'_>`
2019-09-21T20:34:21.2465645Z error: aborting due to previous error
2019-09-21T20:34:21.2465668Z 
2019-09-21T20:34:21.2466037Z For more information about this error, try `rustc --explain E0599`.
2019-09-21T20:34:21.2466064Z 
---
2019-09-21T20:34:21.2468912Z test result: FAILED. 61 passed; 8 failed; 0 ignored; 0 measured; 0 filtered out
2019-09-21T20:34:21.2468964Z 
2019-09-21T20:34:21.2468990Z 
2019-09-21T20:34:21.2469015Z 
2019-09-21T20:34:21.2470568Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-21T20:34:21.2470833Z 
2019-09-21T20:34:21.2470855Z 
2019-09-21T20:34:21.2470890Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-21T20:34:21.2470946Z Build completed unsuccessfully in 1:10:01
2019-09-21T20:34:21.2470946Z Build completed unsuccessfully in 1:10:01
2019-09-21T20:34:21.2494200Z == clock drift check ==
2019-09-21T20:34:21.2517745Z   local time: Sat Sep 21 20:34:21 UTC 2019
2019-09-21T20:34:21.5318635Z   network time: Sat, 21 Sep 2019 20:34:21 GMT
2019-09-21T20:34:21.5323385Z == end clock drift check ==
2019-09-21T20:34:23.0147529Z ##[error]Bash exited with code '1'.
2019-09-21T20:34:23.0208703Z ##[section]Starting: Checkout
2019-09-21T20:34:23.0210958Z ==============================================================================
2019-09-21T20:34:23.0211026Z Task         : Get sources
2019-09-21T20:34:23.0211068Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
