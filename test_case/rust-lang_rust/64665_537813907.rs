plain
2019-10-03T05:39:31.4986352Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-03T05:39:31.5234916Z ##[command]git config gc.auto 0
2019-10-03T05:39:31.5325413Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-03T05:39:31.5384852Z ##[command]git config --get-all http.proxy
2019-10-03T05:39:31.5540066Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64665/merge:refs/remotes/pull/64665/merge
---
2019-10-03T06:42:56.4834008Z .................................................................................................... 1500/9092
2019-10-03T06:43:03.4560930Z .................................................................................................... 1600/9092
2019-10-03T06:43:13.2369151Z .................................................................................................... 1700/9092
2019-10-03T06:43:22.1957652Z ...i...............i................................................................................ 1800/9092
2019-10-03T06:43:29.4818577Z ..............................................................................................iiiii. 1900/9092
2019-10-03T06:43:52.6856056Z .................................................................................................... 2100/9092
2019-10-03T06:43:55.2197230Z .................................................................................................... 2200/9092
2019-10-03T06:43:57.9017734Z .................................................................................................... 2300/9092
2019-10-03T06:44:04.4653664Z .................................................................................................... 2400/9092
---
2019-10-03T06:47:06.8702960Z .................................................................................i...............i.. 4700/9092
2019-10-03T06:47:15.3506169Z .................................................................................................... 4800/9092
2019-10-03T06:47:25.9391317Z .................................................................................................... 4900/9092
2019-10-03T06:47:32.0833834Z .................................................................................................... 5000/9092
2019-10-03T06:47:43.7573011Z .........................................................................ii.ii...................... 5100/9092
2019-10-03T06:47:53.6714248Z .................................................................................................... 5300/9092
2019-10-03T06:48:03.6671039Z .................................................................................................... 5400/9092
2019-10-03T06:48:11.1335207Z .......................................i............................................................ 5500/9092
2019-10-03T06:48:17.6819648Z .................................................................................................... 5600/9092
2019-10-03T06:48:17.6819648Z .................................................................................................... 5600/9092
2019-10-03T06:48:28.9777709Z .................................................................................................... 5700/9092
2019-10-03T06:48:40.2742696Z ....................................ii...i..ii............i......................................... 5800/9092
2019-10-03T06:49:03.1445331Z .................................................................................................... 6000/9092
2019-10-03T06:49:12.6873974Z .................................................................................................... 6100/9092
2019-10-03T06:49:12.6873974Z .................................................................................................... 6100/9092
2019-10-03T06:49:26.5919787Z .......................................i..ii........................................................ 6200/9092
2019-10-03T06:49:48.2846075Z ...................................................................................................i 6400/9092
2019-10-03T06:49:50.5636317Z .................................................................................................... 6500/9092
2019-10-03T06:49:52.8921531Z .......................................................................i............................ 6600/9092
2019-10-03T06:49:56.0092367Z .................................................................................................... 6700/9092
---
2019-10-03T06:54:36.3206653Z  finished in 5.587
2019-10-03T06:54:36.3426969Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-03T06:54:36.5174059Z 
2019-10-03T06:54:36.5174388Z running 150 tests
2019-10-03T06:54:39.8934018Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-10-03T06:54:41.9101029Z ..iiii..............i.........iii.i.......ii......
2019-10-03T06:54:41.9101646Z 
2019-10-03T06:54:41.9106954Z  finished in 5.569
2019-10-03T06:54:41.9295501Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-03T06:54:42.0894954Z 
---
2019-10-03T06:54:44.2368783Z  finished in 2.307
2019-10-03T06:54:44.2564738Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-03T06:54:44.4192411Z 
2019-10-03T06:54:44.4192956Z running 9 tests
2019-10-03T06:54:44.4193792Z iiiiiiiii
2019-10-03T06:54:44.4194110Z 
2019-10-03T06:54:44.4194148Z  finished in 0.162
2019-10-03T06:54:44.4402342Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-03T06:54:44.6079952Z 
---
2019-10-03T06:55:03.2194166Z  finished in 18.779
2019-10-03T06:55:03.2413604Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-03T06:55:03.4180341Z 
2019-10-03T06:55:03.4180618Z running 123 tests
2019-10-03T06:55:27.8594169Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-03T06:55:32.6360836Z i.i.i......iii.i.....ii
2019-10-03T06:55:32.6362908Z 
2019-10-03T06:55:32.6364657Z  finished in 29.395
2019-10-03T06:55:32.6376934Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-03T06:55:32.6377590Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-10-03T06:56:16.8646009Z failures:
2019-10-03T06:56:16.8646654Z 
2019-10-03T06:56:16.8663498Z thread 'main' panicked at 'Some tests failed---- [ui] ui-fulldeps/gated-plugin.rs stdout ----
2019-10-03T06:56:16.8667880Z 
2019-10-03T06:56:16.8669071Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-03T06:56:16.8669592Z status: exit code: 1
2019-10-03T06:56:16.8671113Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/gated-plugin/auxiliary"
2019-10-03T06:56:16.8672041Z ------------------------------------------
2019-10-03T06:56:16.8672988Z 
2019-10-03T06:56:16.8673545Z ------------------------------------------
2019-10-03T06:56:16.8674435Z stderr:
2019-10-03T06:56:16.8674435Z stderr:
2019-10-03T06:56:16.8675302Z ------------------------------------------
2019-10-03T06:56:16.8676070Z error[E0599]: no method named `register_syntax_extension` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-03T06:56:16.8676625Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:19:9
2019-10-03T06:56:16.8676903Z    |
2019-10-03T06:56:16.8677125Z LL |     reg.register_syntax_extension(
2019-10-03T06:56:16.8682538Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut rustc_driver::rustc_plugin_impl::Registry<'_>`
2019-10-03T06:56:16.8682899Z error: aborting due to previous error
2019-10-03T06:56:16.8682943Z 
2019-10-03T06:56:16.8685409Z For more information about this error, try `rustc --explain E0599`.
2019-10-03T06:56:16.8686038Z 
2019-10-03T06:56:16.8686038Z 
2019-10-03T06:56:16.8686516Z ------------------------------------------
2019-10-03T06:56:16.8686687Z 
2019-10-03T06:56:16.8686809Z 
2019-10-03T06:56:16.8687187Z ---- [ui] ui-fulldeps/plugin-args-1.rs stdout ----
2019-10-03T06:56:16.8687347Z 
2019-10-03T06:56:16.8687959Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-03T06:56:16.8688144Z status: exit code: 1
2019-10-03T06:56:16.8689009Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-1/auxiliary"
2019-10-03T06:56:16.8689622Z ------------------------------------------
2019-10-03T06:56:16.8689795Z 
2019-10-03T06:56:16.8690641Z ------------------------------------------
2019-10-03T06:56:16.8690852Z stderr:
2019-10-03T06:56:16.8690852Z stderr:
2019-10-03T06:56:16.8691220Z ------------------------------------------
2019-10-03T06:56:16.8691788Z error[E0599]: no method named `register_syntax_extension` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-03T06:56:16.8692243Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:39:9
2019-10-03T06:56:16.8692444Z    |
2019-10-03T06:56:16.8692609Z LL |     reg.register_syntax_extension(Symbol::intern("plugin_args"), SyntaxExtension::default(
2019-10-03T06:56:16.8693077Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut rustc_driver::rustc_plugin_impl::Registry<'_>`
2019-10-03T06:56:16.8693745Z error: aborting due to previous error
2019-10-03T06:56:16.8693859Z 
2019-10-03T06:56:16.8694205Z For more information about this error, try `rustc --explain E0599`.
2019-10-03T06:56:16.8694357Z 
2019-10-03T06:56:16.8694357Z 
2019-10-03T06:56:16.8694676Z ------------------------------------------
2019-10-03T06:56:16.8694820Z 
2019-10-03T06:56:16.8694952Z 
2019-10-03T06:56:16.8695299Z ---- [ui] ui-fulldeps/plugin-args-2.rs stdout ----
2019-10-03T06:56:16.8695444Z 
2019-10-03T06:56:16.8695817Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-03T06:56:16.8695984Z status: exit code: 1
2019-10-03T06:56:16.8696779Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-2/auxiliary"
2019-10-03T06:56:16.8697383Z ------------------------------------------
2019-10-03T06:56:16.8697528Z 
2019-10-03T06:56:16.8697861Z ------------------------------------------
2019-10-03T06:56:16.8698199Z stderr:
2019-10-03T06:56:16.8698199Z stderr:
2019-10-03T06:56:16.8698528Z ------------------------------------------
2019-10-03T06:56:16.8699007Z error[E0599]: no method named `register_syntax_extension` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-03T06:56:16.8699621Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:39:9
2019-10-03T06:56:16.8699816Z    |
2019-10-03T06:56:16.8700829Z LL |     reg.register_syntax_extension(Symbol::intern("plugin_args"), SyntaxExtension::default(
2019-10-03T06:56:16.8701409Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut rustc_driver::rustc_plugin_impl::Registry<'_>`
2019-10-03T06:56:16.8701753Z error: aborting due to previous error
2019-10-03T06:56:16.8701877Z 
2019-10-03T06:56:16.8702294Z For more information about this error, try `rustc --explain E0599`.
2019-10-03T06:56:16.8702464Z 
2019-10-03T06:56:16.8702464Z 
2019-10-03T06:56:16.8702829Z ------------------------------------------
2019-10-03T06:56:16.8702994Z 
2019-10-03T06:56:16.8703119Z 
2019-10-03T06:56:16.8703671Z ---- [ui] ui-fulldeps/plugin-args-3.rs stdout ----
2019-10-03T06:56:16.8703843Z 
2019-10-03T06:56:16.8704230Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" failed to compile: 
2019-10-03T06:56:16.8704417Z status: exit code: 1
2019-10-03T06:56:16.8705258Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-args-3/auxiliary"
2019-10-03T06:56:16.8705810Z ------------------------------------------
2019-10-03T06:56:16.8705981Z 
2019-10-03T06:56:16.8706307Z ------------------------------------------
2019-10-03T06:56:16.8706474Z stderr:
2019-10-03T06:56:16.8706474Z stderr:
2019-10-03T06:56:16.8706822Z ------------------------------------------
2019-10-03T06:56:16.8707470Z error[E0599]: no method named `register_syntax_extension` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-03T06:56:16.8707872Z   --> /checkout/src/test/ui-fulldeps/auxiliary/plugin-args.rs:39:9
2019-10-03T06:56:16.8708042Z    |
2019-10-03T06:56:16.8708184Z LL |     reg.register_syntax_extension(Symbol::intern("plugin_args"), SyntaxExtension::default(
2019-10-03T06:56:16.8708585Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut rustc_driver::rustc_plugin_impl::Registry<'_>`
2019-10-03T06:56:16.8708885Z error: aborting due to previous error
2019-10-03T06:56:16.8708995Z 
2019-10-03T06:56:16.8709337Z For more information about this error, try `rustc --explain E0599`.
2019-10-03T06:56:16.8709505Z 
2019-10-03T06:56:16.8709505Z 
2019-10-03T06:56:16.8709843Z ------------------------------------------
2019-10-03T06:56:16.8710389Z 
2019-10-03T06:56:16.8710520Z 
2019-10-03T06:56:16.8710932Z ---- [ui] ui-fulldeps/plugin-as-extern-crate.rs stdout ----
2019-10-03T06:56:16.8711126Z 
2019-10-03T06:56:16.8711589Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-03T06:56:16.8711796Z status: exit code: 1
2019-10-03T06:56:16.8712995Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-as-extern-crate/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-as-extern-crate/auxiliary"
2019-10-03T06:56:16.8713861Z ------------------------------------------
2019-10-03T06:56:16.8714179Z 
2019-10-03T06:56:16.8714532Z ------------------------------------------
2019-10-03T06:56:16.8714918Z stderr:
2019-10-03T06:56:16.8714918Z stderr:
2019-10-03T06:56:16.8715265Z ------------------------------------------
2019-10-03T06:56:16.8715838Z error[E0599]: no method named `register_syntax_extension` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-03T06:56:16.8716308Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:19:9
2019-10-03T06:56:16.8716510Z    |
2019-10-03T06:56:16.8716655Z LL |     reg.register_syntax_extension(
2019-10-03T06:56:16.8717050Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut rustc_driver::rustc_plugin_impl::Registry<'_>`
2019-10-03T06:56:16.8717368Z error: aborting due to previous error
2019-10-03T06:56:16.8717481Z 
2019-10-03T06:56:16.8718007Z For more information about this error, try `rustc --explain E0599`.
2019-10-03T06:56:16.8718180Z 
2019-10-03T06:56:16.8718180Z 
2019-10-03T06:56:16.8718526Z ------------------------------------------
2019-10-03T06:56:16.8718687Z 
2019-10-03T06:56:16.8718799Z 
2019-10-03T06:56:16.8719135Z ---- [ui] ui-fulldeps/plugin-attr-register-deny.rs stdout ----
2019-10-03T06:56:16.8719280Z 
2019-10-03T06:56:16.8719663Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-03T06:56:16.8720551Z status: exit code: 1
2019-10-03T06:56:16.8721757Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-attr-register-deny/auxiliary"
2019-10-03T06:56:16.8722405Z ------------------------------------------
2019-10-03T06:56:16.8722571Z 
2019-10-03T06:56:16.8723002Z ------------------------------------------
2019-10-03T06:56:16.8723202Z stderr:
2019-10-03T06:56:16.8723202Z stderr:
2019-10-03T06:56:16.8723729Z ------------------------------------------
2019-10-03T06:56:16.8724183Z error[E0599]: no method named `register_syntax_extension` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-03T06:56:16.8724600Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:19:9
2019-10-03T06:56:16.8724990Z    |
2019-10-03T06:56:16.8725142Z LL |     reg.register_syntax_extension(
2019-10-03T06:56:16.8730779Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut rustc_driver::rustc_plugin_impl::Registry<'_>`
2019-10-03T06:56:16.8731282Z error: aborting due to previous error
2019-10-03T06:56:16.8739587Z 
2019-10-03T06:56:16.8744126Z For more information about this error, try `rustc --explain E0599`.
2019-10-03T06:56:16.8744371Z 
2019-10-03T06:56:16.8744371Z 
2019-10-03T06:56:16.8744650Z ------------------------------------------
2019-10-03T06:56:16.8744682Z 
2019-10-03T06:56:16.8744705Z 
2019-10-03T06:56:16.8745129Z ---- [ui] ui-fulldeps/plugin-reexport.rs stdout ----
2019-10-03T06:56:16.8745162Z 
2019-10-03T06:56:16.8745447Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" failed to compile: 
2019-10-03T06:56:16.8745503Z status: exit code: 1
2019-10-03T06:56:16.8746234Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-reexport/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/plugin-reexport/auxiliary"
2019-10-03T06:56:16.8746990Z ------------------------------------------
2019-10-03T06:56:16.8747131Z 
2019-10-03T06:56:16.8747543Z ------------------------------------------
2019-10-03T06:56:16.8747588Z stderr:
2019-10-03T06:56:16.8747588Z stderr:
2019-10-03T06:56:16.8747811Z ------------------------------------------
2019-10-03T06:56:16.8748117Z error[E0599]: no method named `register_syntax_extension` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-03T06:56:16.8748368Z   --> /checkout/src/test/ui-fulldeps/auxiliary/attr-plugin-test.rs:19:9
2019-10-03T06:56:16.8748435Z    |
2019-10-03T06:56:16.8748637Z LL |     reg.register_syntax_extension(
2019-10-03T06:56:16.8749054Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut rustc_driver::rustc_plugin_impl::Registry<'_>`
2019-10-03T06:56:16.8749152Z error: aborting due to previous error
2019-10-03T06:56:16.8749178Z 
2019-10-03T06:56:16.8749393Z For more information about this error, try `rustc --explain E0599`.
2019-10-03T06:56:16.8749440Z 
2019-10-03T06:56:16.8749440Z 
2019-10-03T06:56:16.8749637Z ------------------------------------------
2019-10-03T06:56:16.8749667Z 
2019-10-03T06:56:16.8749689Z 
2019-10-03T06:56:16.8750302Z ---- [ui] ui-fulldeps/roman-numerals-macro.rs stdout ----
2019-10-03T06:56:16.8750342Z 
2019-10-03T06:56:16.8750654Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" failed to compile: 
2019-10-03T06:56:16.8750708Z status: exit code: 1
2019-10-03T06:56:16.8751507Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/roman-numerals-macro/auxiliary"
2019-10-03T06:56:16.8751851Z ------------------------------------------
2019-10-03T06:56:16.8751885Z 
2019-10-03T06:56:16.8752104Z ------------------------------------------
2019-10-03T06:56:16.8752167Z stderr:
2019-10-03T06:56:16.8752167Z stderr:
2019-10-03T06:56:16.8752383Z ------------------------------------------
2019-10-03T06:56:16.8753110Z error[E0599]: no method named `register_macro` found for type `&mut rustc_driver::rustc_plugin_impl::Registry<'_>` in the current scope
2019-10-03T06:56:16.8753413Z   --> /checkout/src/test/ui-fulldeps/auxiliary/roman-numerals.rs:67:9
2019-10-03T06:56:16.8753631Z    |
2019-10-03T06:56:16.8753677Z LL |     reg.register_macro("rn", expand_rn);
2019-10-03T06:56:16.8753946Z    |         ^^^^^^^^^^^^^^ method not found in `&mut rustc_driver::rustc_plugin_impl::Registry<'_>`
2019-10-03T06:56:16.8754029Z error: aborting due to previous error
2019-10-03T06:56:16.8754056Z 
2019-10-03T06:56:16.8754302Z For more information about this error, try `rustc --explain E0599`.
2019-10-03T06:56:16.8754334Z 
---
2019-10-03T06:56:16.8757260Z ', src/tools/compiletest/src/main.rs:537:22
2019-10-03T06:56:16.8757322Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-03T06:56:16.8757353Z 
2019-10-03T06:56:16.8757377Z 
2019-10-03T06:56:16.8759288Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-03T06:56:16.8759519Z 
2019-10-03T06:56:16.8759546Z 
2019-10-03T06:56:16.8759607Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-03T06:56:16.8759660Z Build completed unsuccessfully in 1:09:33
2019-10-03T06:56:16.8759660Z Build completed unsuccessfully in 1:09:33
2019-10-03T06:56:16.8759703Z == clock drift check ==
2019-10-03T06:56:16.8770647Z   local time: Thu Oct  3 06:56:16 UTC 2019
2019-10-03T06:56:17.0572967Z   network time: Thu, 03 Oct 2019 06:56:17 GMT
2019-10-03T06:56:17.0575450Z == end clock drift check ==
2019-10-03T06:56:18.5095395Z ##[error]Bash exited with code '1'.
2019-10-03T06:56:18.5164729Z ##[section]Starting: Checkout
2019-10-03T06:56:18.5166542Z ==============================================================================
2019-10-03T06:56:18.5166616Z Task         : Get sources
2019-10-03T06:56:18.5166661Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
