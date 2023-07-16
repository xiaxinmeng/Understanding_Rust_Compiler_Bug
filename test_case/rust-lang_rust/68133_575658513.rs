plain
2020-01-17T13:49:23.0577386Z ========================== Starting Command Output ===========================
2020-01-17T13:49:23.0599778Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fc784c46-f7c0-452c-a5b3-06b3b5bdb08b.sh
2020-01-17T13:49:23.1372984Z 
2020-01-17T13:49:23.1470235Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-17T13:49:23.1475163Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68133/merge to s
2020-01-17T13:49:23.1476556Z Task         : Get sources
2020-01-17T13:49:23.1476581Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-17T13:49:23.1476618Z Version      : 1.0.0
2020-01-17T13:49:23.1476641Z Author       : Microsoft
---
2020-01-17T13:49:23.9466714Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-17T13:49:23.9548254Z ##[command]git config gc.auto 0
2020-01-17T13:49:23.9620986Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-17T13:49:23.9674804Z ##[command]git config --get-all http.proxy
2020-01-17T13:49:23.9801847Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68133/merge:refs/remotes/pull/68133/merge
---
2020-01-17T14:42:57.3133166Z .................................................................................................... 1700/9528
2020-01-17T14:43:05.1589836Z .................................................................................................... 1800/9528
2020-01-17T14:43:14.1356581Z ...........i........................................................................................ 1900/9528
2020-01-17T14:43:20.9269742Z .................................................................................................... 2000/9528
2020-01-17T14:43:35.8386338Z .iiiii.............................................................................................. 2100/9528
2020-01-17T14:43:43.6715071Z .................................................................................................... 2300/9528
2020-01-17T14:43:45.8783459Z .................................................................................................... 2400/9528
2020-01-17T14:43:51.2043180Z .................................................................................................... 2500/9528
2020-01-17T14:44:10.1874829Z .................................................................................................... 2600/9528
---
2020-01-17T14:46:40.8039192Z ..............................................i...............i..................................... 4900/9528
2020-01-17T14:46:49.1801238Z .................................................................................................... 5000/9528
2020-01-17T14:46:55.5718419Z .........................................................................................i.......... 5100/9528
2020-01-17T14:47:00.4267488Z .................................................................................................... 5200/9528
2020-01-17T14:47:10.5973417Z .............................................................ii.ii...........i...................... 5300/9528
2020-01-17T14:47:14.6764897Z ..................................................................................................i. 5400/9528
2020-01-17T14:47:28.8011036Z .................................................................................................... 5600/9528
2020-01-17T14:47:34.8279917Z ..............................................i..................................................... 5700/9528
2020-01-17T14:47:41.4026360Z .................................................................................................... 5800/9528
2020-01-17T14:47:51.2004710Z .................................................................................................... 5900/9528
2020-01-17T14:47:51.2004710Z .................................................................................................... 5900/9528
2020-01-17T14:47:59.4751241Z .....................................ii...i..ii...........i......................................... 6000/9528
2020-01-17T14:48:18.1138354Z .................................................................................................... 6200/9528
2020-01-17T14:48:25.8001759Z .................................................................................................... 6300/9528
2020-01-17T14:48:25.8001759Z .................................................................................................... 6300/9528
2020-01-17T14:48:35.3792974Z .................................................................i..ii.............................. 6400/9528
2020-01-17T14:49:02.3133755Z .................................................................................................... 6600/9528
2020-01-17T14:49:04.4680501Z .........................................i.......................................................... 6700/9528
2020-01-17T14:49:06.4172225Z .................................................................................................... 6800/9528
2020-01-17T14:49:08.7563496Z .........................................i.......................................................... 6900/9528
---
2020-01-17T14:50:40.7503320Z .................................................................................................... 7500/9528
2020-01-17T14:50:45.0820256Z .................................................................................................... 7600/9528
2020-01-17T14:50:50.7777592Z .................................................................................................... 7700/9528
2020-01-17T14:50:56.8249484Z .................................................................................................... 7800/9528
2020-01-17T14:51:06.6367396Z ............................................................................................iiii.... 7900/9528
2020-01-17T14:51:22.2458380Z ..........................i......i.................................................................. 8100/9528
2020-01-17T14:51:26.7973531Z .................................................................................................... 8200/9528
2020-01-17T14:51:38.3881254Z .................................................................................................... 8300/9528
2020-01-17T14:51:49.4323448Z .................................................................................................... 8400/9528
---
2020-01-17T14:54:08.8566799Z  finished in 6.955
2020-01-17T14:54:08.8739474Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-17T14:54:09.0255388Z 
2020-01-17T14:54:09.0255575Z running 166 tests
2020-01-17T14:54:11.9117002Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-17T14:54:13.9749854Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-17T14:54:13.9751820Z 
2020-01-17T14:54:13.9757801Z  finished in 5.101
2020-01-17T14:54:13.9928820Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-17T14:54:14.1426157Z 
---
2020-01-17T14:54:15.9846580Z  finished in 1.991
2020-01-17T14:54:16.0030492Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-17T14:54:16.6345504Z 
2020-01-17T14:54:16.6345607Z running 9 tests
2020-01-17T14:54:16.6346310Z iiiiiiiii
2020-01-17T14:54:16.6346609Z 
2020-01-17T14:54:16.6346660Z  finished in 0.147
2020-01-17T14:54:16.6346894Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-17T14:54:16.6346925Z 
---
2020-01-17T14:54:36.1340453Z  finished in 19.011
2020-01-17T14:54:36.1341612Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-17T14:54:36.1341662Z 
2020-01-17T14:54:36.1341707Z running 116 tests
2020-01-17T14:54:58.2142492Z .iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii 100/116
2020-01-17T14:55:01.4632339Z .....iiii.....ii
2020-01-17T14:55:01.4633671Z 
2020-01-17T14:55:01.4637557Z  finished in 26.265
2020-01-17T14:55:01.4644218Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-17T14:55:01.4645240Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-17T14:55:25.5325805Z failures:
2020-01-17T14:55:25.5325907Z 
2020-01-17T14:55:25.5326228Z ---- [ui] ui-fulldeps/issue-40001.rs stdout ----
2020-01-17T14:55:25.5326258Z 
2020-01-17T14:55:25.5326661Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
2020-01-17T14:55:25.5326843Z status: exit code: 1
2020-01-17T14:55:25.5327817Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
2020-01-17T14:55:25.5328381Z ------------------------------------------
2020-01-17T14:55:25.5328554Z 
2020-01-17T14:55:25.5328967Z ------------------------------------------
2020-01-17T14:55:25.5329113Z stderr:
2020-01-17T14:55:25.5329113Z stderr:
2020-01-17T14:55:25.5329416Z ------------------------------------------
2020-01-17T14:55:25.5329456Z error[E0432]: unresolved import `syntax::print`
2020-01-17T14:55:25.5329792Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:17:13
2020-01-17T14:55:25.5330277Z LL | use syntax::print::pprust;
2020-01-17T14:55:25.5330343Z    |             ^^^^^ could not find `print` in `syntax`
2020-01-17T14:55:25.5330384Z 
2020-01-17T14:55:25.5330439Z warning: unused `#[macro_use]` import
2020-01-17T14:55:25.5330439Z warning: unused `#[macro_use]` import
2020-01-17T14:55:25.5330692Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:6:1
2020-01-17T14:55:25.5331413Z LL | #[macro_use] extern crate rustc_lint;
2020-01-17T14:55:25.5331481Z    | ^^^^^^^^^^^^
2020-01-17T14:55:25.5331577Z    |
2020-01-17T14:55:25.5331624Z    = note: `#[warn(unused_imports)]` on by default
2020-01-17T14:55:25.5331624Z    = note: `#[warn(unused_imports)]` on by default
2020-01-17T14:55:25.5331654Z 
2020-01-17T14:55:25.5331717Z warning: unused imports: `LintArray`, `LintPass`
2020-01-17T14:55:25.5332185Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:14:31
2020-01-17T14:55:25.5332372Z LL | use rustc_lint::{LateContext, LintPass, LintArray, LateLintPass, LintContext};
2020-01-17T14:55:25.5332424Z    |                               ^^^^^^^^  ^^^^^^^^^
2020-01-17T14:55:25.5332481Z 
2020-01-17T14:55:25.5332481Z 
2020-01-17T14:55:25.5332993Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-17T14:55:25.5333269Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:19:1
2020-01-17T14:55:25.5333564Z LL | #[plugin_registrar]
2020-01-17T14:55:25.5333647Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-17T14:55:25.5333733Z    |
2020-01-17T14:55:25.5333797Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-17T14:55:25.5335065Z ---- [ui] ui-fulldeps/mod_dir_path_canonicalized.rs stdout ----
2020-01-17T14:55:25.5335108Z 
2020-01-17T14:55:25.5335278Z error: test compilation failed although it shouldn't!
2020-01-17T14:55:25.5335314Z status: exit code: 1
2020-01-17T14:55:25.5336071Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/mod_dir_path_canonicalized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/auxiliary"
2020-01-17T14:55:25.5337847Z ------------------------------------------
2020-01-17T14:55:25.5337989Z 
2020-01-17T14:55:25.5338261Z ------------------------------------------
2020-01-17T14:55:25.5338417Z stderr:
---
2020-01-17T14:55:25.5339921Z 
2020-01-17T14:55:25.5340047Z error[E0425]: cannot find function `with_default_globals` in crate `syntax`
2020-01-17T14:55:25.5340366Z   --> /checkout/src/test/ui-fulldeps/mod_dir_path_canonicalized.rs:20:13
2020-01-17T14:55:25.5341675Z    |
2020-01-17T14:55:25.5341731Z LL |     syntax::with_default_globals(|| parse());
2020-01-17T14:55:25.5341825Z    |
2020-01-17T14:55:25.5341935Z help: possible candidate is found in another module, you can import it into scope
2020-01-17T14:55:25.5341981Z    |
2020-01-17T14:55:25.5342037Z LL | use syntax::attr::with_default_globals;
---
2020-01-17T14:55:25.5343164Z ---- [ui] ui-fulldeps/pprust-expr-roundtrip.rs stdout ----
2020-01-17T14:55:25.5343196Z 
2020-01-17T14:55:25.5343431Z error: test compilation failed although it shouldn't!
2020-01-17T14:55:25.5343480Z status: exit code: 1
2020-01-17T14:55:25.5344575Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/auxiliary"
2020-01-17T14:55:25.5345228Z ------------------------------------------
2020-01-17T14:55:25.5345268Z 
2020-01-17T14:55:25.5345429Z ------------------------------------------
2020-01-17T14:55:25.5345463Z stderr:
---
2020-01-17T14:55:25.5346364Z 
2020-01-17T14:55:25.5346399Z error[E0425]: cannot find function `with_default_globals` in crate `syntax`
2020-01-17T14:55:25.5346629Z   --> /checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs:206:13
2020-01-17T14:55:25.5346666Z    |
2020-01-17T14:55:25.5346702Z LL |     syntax::with_default_globals(|| run());
2020-01-17T14:55:25.5346790Z    |
2020-01-17T14:55:25.5346932Z help: possible candidate is found in another module, you can import it into scope
2020-01-17T14:55:25.5346993Z    |
2020-01-17T14:55:25.5347027Z LL | use syntax::attr::with_default_globals;
---
2020-01-17T14:55:25.5347722Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
2020-01-17T14:55:25.5347763Z    | 
2020-01-17T14:55:25.5347817Z   ::: /checkout/src/libcore/fmt/mod.rs:277:20
2020-01-17T14:55:25.5347851Z    |
2020-01-17T14:55:25.5348060Z LL |     pub fn new<'b, T>(x: &'b T, f: fn(&T, &mut Formatter<'_>) -> Result) -> ArgumentV1<'b> {
2020-01-17T14:55:25.5348277Z    |                    - required by this bound in `std::fmt::ArgumentV1::<'a>::new`
2020-01-17T14:55:25.5348360Z    = help: the trait `std::marker::Sized` is not implemented for `str`
2020-01-17T14:55:25.5348606Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2020-01-17T14:55:25.5348883Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2020-01-17T14:55:25.5348918Z 
---
2020-01-17T14:55:25.5350594Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-17T14:55:25.5350637Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-17T14:55:25.5350662Z 
2020-01-17T14:55:25.5350682Z 
2020-01-17T14:55:25.5352890Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-17T14:55:25.5353163Z 
2020-01-17T14:55:25.5353210Z 
2020-01-17T14:55:25.5354305Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-17T14:55:25.5354473Z Build completed unsuccessfully in 1:00:26
2020-01-17T14:55:25.5354473Z Build completed unsuccessfully in 1:00:26
2020-01-17T14:55:25.5396318Z == clock drift check ==
2020-01-17T14:55:25.5415721Z   local time: Fri Jan 17 14:55:25 UTC 2020
2020-01-17T14:55:25.8357396Z   network time: Fri, 17 Jan 2020 14:55:25 GMT
2020-01-17T14:55:25.8360769Z == end clock drift check ==
2020-01-17T14:55:26.6707082Z 
2020-01-17T14:55:26.6800769Z ##[error]Bash exited with code '1'.
2020-01-17T14:55:26.6834997Z ##[section]Finishing: Run build
2020-01-17T14:55:26.6854742Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68133/merge to s
2020-01-17T14:55:26.6856442Z Task         : Get sources
2020-01-17T14:55:26.6856481Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-17T14:55:26.6856689Z Version      : 1.0.0
2020-01-17T14:55:26.6856739Z Author       : Microsoft
2020-01-17T14:55:26.6856739Z Author       : Microsoft
2020-01-17T14:55:26.6856775Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-17T14:55:26.6856824Z ==============================================================================
2020-01-17T14:55:27.0808865Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-17T14:55:27.0849961Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68133/merge to s
2020-01-17T14:55:27.0960024Z Cleaning up task key
2020-01-17T14:55:27.0961605Z Start cleaning up orphan processes.
2020-01-17T14:55:27.1100270Z Terminate orphan process: pid (3567) (python)
2020-01-17T14:55:27.1374457Z ##[section]Finishing: Finalize Job
