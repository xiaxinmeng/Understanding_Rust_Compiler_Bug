plain
2020-01-18T18:22:11.2285948Z ========================== Starting Command Output ===========================
2020-01-18T18:22:11.2301632Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/273c7689-906d-47a5-b9b8-dfd77f8f00e9.sh
2020-01-18T18:22:11.6568537Z 
2020-01-18T18:22:11.6643265Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T18:22:11.6649153Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68133/merge to s
2020-01-18T18:22:11.6650662Z Task         : Get sources
2020-01-18T18:22:11.6650734Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T18:22:11.6650762Z Version      : 1.0.0
2020-01-18T18:22:11.6650788Z Author       : Microsoft
---
2020-01-18T18:22:16.7306322Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T18:22:16.7581801Z ##[command]git config gc.auto 0
2020-01-18T18:22:16.7647480Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T18:22:16.7713354Z ##[command]git config --get-all http.proxy
2020-01-18T18:22:16.7840913Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68133/merge:refs/remotes/pull/68133/merge
---
2020-01-18T19:09:35.9779161Z .................................................................................................... 1700/9539
2020-01-18T19:09:42.2982898Z .................................................................................................... 1800/9539
2020-01-18T19:09:51.4174291Z .................i.................................................................................. 1900/9539
2020-01-18T19:09:57.5896723Z .................................................................................................... 2000/9539
2020-01-18T19:10:10.9974183Z .......iiiii........................................................................................ 2100/9539
2020-01-18T19:10:18.7259488Z .................................................................................................... 2300/9539
2020-01-18T19:10:20.8085885Z .................................................................................................... 2400/9539
2020-01-18T19:10:25.3956786Z .................................................................................................... 2500/9539
2020-01-18T19:10:42.9517186Z .................................................................................................... 2600/9539
---
2020-01-18T19:12:59.6788454Z ....................................................i...............i............................... 4900/9539
2020-01-18T19:13:06.5811797Z .................................................................................................... 5000/9539
2020-01-18T19:13:13.3188942Z ...............................................................................................i.... 5100/9539
2020-01-18T19:13:18.0370095Z .................................................................................................... 5200/9539
2020-01-18T19:13:27.4007378Z ...................................................................ii.ii...........i................ 5300/9539
2020-01-18T19:13:35.8599607Z ....i............................................................................................... 5500/9539
2020-01-18T19:13:44.7717064Z .................................................................................................... 5600/9539
2020-01-18T19:13:50.7541060Z .....................................................i.............................................. 5700/9539
2020-01-18T19:13:57.0577713Z .................................................................................................... 5800/9539
2020-01-18T19:13:57.0577713Z .................................................................................................... 5800/9539
2020-01-18T19:14:06.2178975Z .................................................................................................... 5900/9539
2020-01-18T19:14:12.2120193Z ............................................ii...i..ii...........i.................................. 6000/9539
2020-01-18T19:14:32.3707940Z .................................................................................................... 6200/9539
2020-01-18T19:14:40.1455956Z .................................................................................................... 6300/9539
2020-01-18T19:14:40.1455956Z .................................................................................................... 6300/9539
2020-01-18T19:14:48.8070045Z ........................................................................i..ii....................... 6400/9539
2020-01-18T19:15:13.5946851Z .................................................................................................... 6600/9539
2020-01-18T19:15:16.0774785Z ................................................i................................................... 6700/9539
2020-01-18T19:15:17.9114554Z .................................................................................................... 6800/9539
2020-01-18T19:15:19.8930491Z ................................................i................................................... 6900/9539
---
2020-01-18T19:16:46.0063117Z .................................................................................................... 7500/9539
2020-01-18T19:16:50.1799016Z .................................................................................................... 7600/9539
2020-01-18T19:16:55.0458264Z .................................................................................................... 7700/9539
2020-01-18T19:17:00.7476602Z .................................................................................................... 7800/9539
2020-01-18T19:17:10.2757652Z ...................................................................................................i 7900/9539
2020-01-18T19:17:15.4329277Z iiiiii.............................................................................................. 8000/9539
2020-01-18T19:17:28.6912749Z .................................................................................................... 8200/9539
2020-01-18T19:17:38.5854475Z .................................................................................................... 8300/9539
2020-01-18T19:17:49.4092424Z .................................................................................................... 8400/9539
2020-01-18T19:17:54.4049677Z .................................................................................................... 8500/9539
---
2020-01-18T19:19:58.2879368Z  finished in 6.559
2020-01-18T19:19:58.3037700Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-18T19:19:58.4699035Z 
2020-01-18T19:19:58.4700809Z running 166 tests
2020-01-18T19:20:01.1139676Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-18T19:20:03.1551167Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-18T19:20:03.1552905Z 
2020-01-18T19:20:03.1557308Z  finished in 4.851
2020-01-18T19:20:03.1737012Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-18T19:20:03.3196702Z 
---
2020-01-18T19:20:05.0464094Z  finished in 1.872
2020-01-18T19:20:05.0654477Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-18T19:20:05.2069851Z 
2020-01-18T19:20:05.2070494Z running 9 tests
2020-01-18T19:20:05.2071817Z iiiiiiiii
2020-01-18T19:20:05.2072156Z 
2020-01-18T19:20:05.2073784Z  finished in 0.141
2020-01-18T19:20:05.2232313Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-18T19:20:05.4013256Z 
---
2020-01-18T19:20:22.6199701Z  finished in 17.396
2020-01-18T19:20:22.6444647Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-18T19:20:22.8021509Z 
2020-01-18T19:20:22.8021809Z running 116 tests
2020-01-18T19:20:44.1362709Z .iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii 100/116
2020-01-18T19:20:46.9950585Z .....iiii.....ii
2020-01-18T19:20:46.9952099Z 
2020-01-18T19:20:46.9953154Z  finished in 24.350
2020-01-18T19:20:46.9958157Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-18T19:20:46.9958497Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-18T19:21:21.0426963Z failures:
2020-01-18T19:21:21.0433111Z 
2020-01-18T19:21:21.0433594Z ---- [ui] ui-fulldeps/issue-40001.rs stdout ----
2020-01-18T19:21:21.0434034Z 
2020-01-18T19:21:21.0434359Z error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
2020-01-18T19:21:21.0434422Z status: exit code: 1
2020-01-18T19:21:21.0435165Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
2020-01-18T19:21:21.0435463Z ------------------------------------------
2020-01-18T19:21:21.0435492Z 
2020-01-18T19:21:21.0435851Z ------------------------------------------
2020-01-18T19:21:21.0435910Z stderr:
2020-01-18T19:21:21.0435910Z stderr:
2020-01-18T19:21:21.0436438Z ------------------------------------------
2020-01-18T19:21:21.0436482Z error[E0432]: unresolved import `syntax::print`
2020-01-18T19:21:21.0436699Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:17:13
2020-01-18T19:21:21.0436773Z LL | use syntax::print::pprust;
2020-01-18T19:21:21.0436827Z    |             ^^^^^ could not find `print` in `syntax`
2020-01-18T19:21:21.0436850Z 
2020-01-18T19:21:21.0436884Z warning: unused `#[macro_use]` import
2020-01-18T19:21:21.0436884Z warning: unused `#[macro_use]` import
2020-01-18T19:21:21.0437076Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:6:1
2020-01-18T19:21:21.0437164Z LL | #[macro_use] extern crate rustc_lint;
2020-01-18T19:21:21.0437198Z    | ^^^^^^^^^^^^
2020-01-18T19:21:21.0437247Z    |
2020-01-18T19:21:21.0437281Z    = note: `#[warn(unused_imports)]` on by default
2020-01-18T19:21:21.0437281Z    = note: `#[warn(unused_imports)]` on by default
2020-01-18T19:21:21.0437310Z 
2020-01-18T19:21:21.0437344Z warning: unused imports: `LintArray`, `LintPass`
2020-01-18T19:21:21.0437560Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:14:31
2020-01-18T19:21:21.0437640Z LL | use rustc_lint::{LateContext, LintPass, LintArray, LateLintPass, LintContext};
2020-01-18T19:21:21.0437699Z    |                               ^^^^^^^^  ^^^^^^^^^
2020-01-18T19:21:21.0437722Z 
2020-01-18T19:21:21.0437722Z 
2020-01-18T19:21:21.0438068Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/pull/64675
2020-01-18T19:21:21.0438297Z   --> /checkout/src/test/ui-fulldeps/auxiliary/issue-40001-plugin.rs:19:1
2020-01-18T19:21:21.0438367Z LL | #[plugin_registrar]
2020-01-18T19:21:21.0438422Z    | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-01-18T19:21:21.0438455Z    |
2020-01-18T19:21:21.0438488Z    = note: `#[warn(deprecated)]` on by default
---
2020-01-18T19:21:21.0442782Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-18T19:21:21.0443373Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-18T19:21:21.0443644Z 
2020-01-18T19:21:21.0443748Z 
2020-01-18T19:21:21.0445214Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-18T19:21:21.0445578Z 
2020-01-18T19:21:21.0445602Z 
2020-01-18T19:21:21.0450508Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-18T19:21:21.0450828Z Build completed unsuccessfully in 0:53:32
2020-01-18T19:21:21.0450828Z Build completed unsuccessfully in 0:53:32
2020-01-18T19:21:21.0504081Z == clock drift check ==
2020-01-18T19:21:21.0519987Z   local time: Sat Jan 18 19:21:21 UTC 2020
2020-01-18T19:21:21.3378602Z   network time: Sat, 18 Jan 2020 19:21:21 GMT
2020-01-18T19:21:21.3378684Z == end clock drift check ==
2020-01-18T19:21:22.2817942Z 
2020-01-18T19:21:22.2904253Z ##[error]Bash exited with code '1'.
2020-01-18T19:21:22.2913942Z ##[section]Finishing: Run build
2020-01-18T19:21:22.2948421Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68133/merge to s
2020-01-18T19:21:22.2950015Z Task         : Get sources
2020-01-18T19:21:22.2950050Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T19:21:22.2950104Z Version      : 1.0.0
2020-01-18T19:21:22.2950134Z Author       : Microsoft
2020-01-18T19:21:22.2950134Z Author       : Microsoft
2020-01-18T19:21:22.2950169Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-18T19:21:22.2950223Z ==============================================================================
2020-01-18T19:21:22.6486627Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-18T19:21:22.6533758Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68133/merge to s
2020-01-18T19:21:22.6643267Z Cleaning up task key
2020-01-18T19:21:22.6643970Z Start cleaning up orphan processes.
2020-01-18T19:21:22.6742661Z Terminate orphan process: pid (3691) (python)
2020-01-18T19:21:22.7395238Z ##[section]Finishing: Finalize Job
