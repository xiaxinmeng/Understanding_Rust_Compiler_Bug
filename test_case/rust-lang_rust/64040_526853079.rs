plain
2019-08-31T16:36:07.0377700Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-31T16:36:07.0580889Z ##[command]git config gc.auto 0
2019-08-31T16:36:07.0635669Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-31T16:36:07.0703938Z ##[command]git config --get-all http.proxy
2019-08-31T16:36:07.0845896Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64040/merge:refs/remotes/pull/64040/merge
---
2019-08-31T17:39:17.5572546Z .................................................................................................... 1500/8983
2019-08-31T17:39:23.1561277Z .................................................................................................... 1600/8983
2019-08-31T17:39:35.9956716Z .................................................i...............i.................................. 1700/8983
2019-08-31T17:39:44.2138921Z .................................................................................................... 1800/8983
2019-08-31T17:39:58.6866699Z ........................................iiiii....................................................... 1900/8983
2019-08-31T17:40:09.5495334Z .................................................................................................... 2100/8983
2019-08-31T17:40:12.1392923Z .................................................................................................... 2200/8983
2019-08-31T17:40:16.2008135Z .................................................................................................... 2300/8983
2019-08-31T17:40:23.8446730Z .................................................................................................... 2400/8983
---
2019-08-31T17:43:23.0766285Z ...........................i...............i........................................................ 4700/8983
2019-08-31T17:43:35.1439578Z .................................................................................................... 4800/8983
2019-08-31T17:43:41.2975554Z .................................................................................................... 4900/8983
2019-08-31T17:43:52.0094768Z .................................................................................................... 5000/8983
2019-08-31T17:43:57.7300842Z ........ii.ii....................................................................................... 5100/8983
2019-08-31T17:44:10.6167335Z .................................................................................................... 5300/8983
2019-08-31T17:44:18.9714497Z .......................................................................i............................ 5400/8983
2019-08-31T17:44:26.3766243Z .................................................................................................... 5500/8983
2019-08-31T17:44:33.3567157Z .................................................................................................... 5600/8983
2019-08-31T17:44:33.3567157Z .................................................................................................... 5600/8983
2019-08-31T17:44:43.9628053Z .................................................................ii...i..ii...........i............. 5700/8983
2019-08-31T17:45:09.9038645Z .................................................................................................... 5900/8983
2019-08-31T17:45:19.2800789Z .................................................................................................... 6000/8983
2019-08-31T17:45:19.2800789Z .................................................................................................... 6000/8983
2019-08-31T17:45:25.3321220Z ..................................................................i..ii............................. 6100/8983
2019-08-31T17:45:55.0750179Z .................................................................................................... 6300/8983
2019-08-31T17:45:57.1597542Z .....................i.............................................................................. 6400/8983
2019-08-31T17:45:59.3500868Z .............................................................................................i...... 6500/8983
2019-08-31T17:46:02.0620829Z .................................................................................................... 6600/8983
---
2019-08-31T17:50:43.8640145Z  finished in 19.207
2019-08-31T17:50:43.8826694Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-31T17:50:44.0468319Z 
2019-08-31T17:50:44.0469319Z running 149 tests
2019-08-31T17:50:47.2799713Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/149
2019-08-31T17:50:49.2066868Z ..iiii..............i.........iii.i......ii......
2019-08-31T17:50:49.2067934Z 
2019-08-31T17:50:49.2071807Z  finished in 5.324
2019-08-31T17:50:49.2260448Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-31T17:50:49.3878096Z 
---
2019-08-31T17:50:51.4700108Z  finished in 2.244
2019-08-31T17:50:51.4899584Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-31T17:50:51.6495502Z 
2019-08-31T17:50:51.6495738Z running 9 tests
2019-08-31T17:50:51.6496544Z iiiiiiiii
2019-08-31T17:50:51.6496937Z 
2019-08-31T17:50:51.6496983Z  finished in 0.159
2019-08-31T17:50:51.6700084Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-31T17:50:51.8366805Z 
---
2019-08-31T17:51:10.0520931Z  finished in 18.382
2019-08-31T17:51:10.0742853Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-31T17:51:10.2420250Z 
2019-08-31T17:51:10.2420750Z running 123 tests
2019-08-31T17:51:34.1676014Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-08-31T17:51:38.8795773Z i.i.i......iii.i.....ii
2019-08-31T17:51:38.8796314Z 
2019-08-31T17:51:38.8796430Z  finished in 28.805
2019-08-31T17:51:38.8808285Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-31T17:51:38.8808624Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-08-31T17:59:11.1059675Z 
2019-08-31T17:59:11.1059886Z failures:
2019-08-31T17:59:11.1060089Z 
2019-08-31T17:59:11.1060591Z ---- /checkout/src/test/rustdoc/async-move-doctest.rs -  (line 8) stdout ----
2019-08-31T17:59:11.1060855Z error[E0658]: async blocks are unstable
2019-08-31T17:59:11.1061718Z  --> <anon>:4:10
2019-08-31T17:59:11.1062004Z   |
2019-08-31T17:59:11.1062208Z 4 |     drop(async move {});
2019-08-31T17:59:11.1062627Z   |
2019-08-31T17:59:11.1062627Z   |
2019-08-31T17:59:11.1063234Z   = note: for more information, see ***/issues/50547
2019-08-31T17:59:11.1063542Z   = help: add `#![feature(async_await)]` to the crate attributes to enable
2019-08-31T17:59:11.1064156Z error: aborting due to previous error
2019-08-31T17:59:11.1064369Z 
2019-08-31T17:59:11.1064876Z For more information about this error, try `rustc --explain E0658`.
2019-08-31T17:59:11.1065345Z Couldn't compile the test.
---
2019-08-31T17:59:11.1069755Z ---- [rustdoc] rustdoc/edition-flag.rs stdout ----
2019-08-31T17:59:11.1070002Z 
2019-08-31T17:59:11.1070202Z error: rustdoc failed!
2019-08-31T17:59:11.1070425Z status: exit code: 101
2019-08-31T17:59:11.1071584Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/edition-flag/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/edition-flag" "/checkout/src/test/rustdoc/edition-flag.rs" "--test" "-Z" "unstable-options" "--edition=2018"
2019-08-31T17:59:11.1072385Z ------------------------------------------
2019-08-31T17:59:11.1072623Z 
2019-08-31T17:59:11.1072854Z running 1 test
2019-08-31T17:59:11.1073315Z test /checkout/src/test/rustdoc/edition-flag.rs - main (line 4) ... FAILED
2019-08-31T17:59:11.1073315Z test /checkout/src/test/rustdoc/edition-flag.rs - main (line 4) ... FAILED
2019-08-31T17:59:11.1073552Z 
2019-08-31T17:59:11.1073786Z failures:
2019-08-31T17:59:11.1073962Z 
2019-08-31T17:59:11.1074420Z ---- /checkout/src/test/rustdoc/edition-flag.rs - main (line 4) stdout ----
2019-08-31T17:59:11.1074715Z error[E0658]: async blocks are unstable
2019-08-31T17:59:11.1075126Z  --> <anon>:3:13
2019-08-31T17:59:11.1075610Z 3 |     let _ = async { };
2019-08-31T17:59:11.1075804Z   |             ^^^^^^^^^
2019-08-31T17:59:11.1075997Z   |
2019-08-31T17:59:11.1075997Z   |
2019-08-31T17:59:11.1076548Z   = note: for more information, see ***/issues/50547
2019-08-31T17:59:11.1076827Z   = help: add `#![feature(async_await)]` to the crate attributes to enable
2019-08-31T17:59:11.1077425Z error: aborting due to previous error
2019-08-31T17:59:11.1077610Z 
2019-08-31T17:59:11.1078167Z For more information about this error, try `rustc --explain E0658`.
2019-08-31T17:59:11.1078658Z Couldn't compile the test.
---
2019-08-31T17:59:11.1083370Z ---- [rustdoc] rustdoc/issue-43153.rs stdout ----
2019-08-31T17:59:11.1084077Z 
2019-08-31T17:59:11.1084295Z error: rustdoc failed!
2019-08-31T17:59:11.1084446Z status: exit code: 101
2019-08-31T17:59:11.1085185Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43153/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43153" "/checkout/src/test/rustdoc/issue-43153.rs" "--test"
2019-08-31T17:59:11.1085963Z ------------------------------------------
2019-08-31T17:59:11.1086141Z 
2019-08-31T17:59:11.1086285Z running 1 test
2019-08-31T17:59:11.1086665Z test /checkout/src/test/rustdoc/issue-43153.rs - Foo (line 6) ... FAILED
2019-08-31T17:59:11.1086665Z test /checkout/src/test/rustdoc/issue-43153.rs - Foo (line 6) ... FAILED
2019-08-31T17:59:11.1086839Z 
2019-08-31T17:59:11.1086982Z failures:
2019-08-31T17:59:11.1087098Z 
2019-08-31T17:59:11.1087506Z ---- /checkout/src/test/rustdoc/issue-43153.rs - Foo (line 6) stdout ----
2019-08-31T17:59:11.1087970Z thread 'rustc' panicked at 'cannot resolve relative path in non-file source `<anon>`', src/libsyntax/ext/base.rs:928:26
2019-08-31T17:59:11.1088307Z 
2019-08-31T17:59:11.1088447Z error: internal compiler error: unexpected panic
2019-08-31T17:59:11.1088567Z 
2019-08-31T17:59:11.1088741Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-31T17:59:11.1088741Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-31T17:59:11.1088867Z 
2019-08-31T17:59:11.1089439Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-31T17:59:11.1089920Z 
2019-08-31T17:59:11.1090424Z note: rustc 1.38.0-beta.1 (e450539c2 2019-08-13) running on x86_64-unknown-linux-gnu
2019-08-31T17:59:11.1091216Z note: compiler flags: --crate-type bin
2019-08-31T17:59:11.1091404Z 
2019-08-31T17:59:11.1091743Z Couldn't compile the test.
2019-08-31T17:59:11.1091889Z 
---
2019-08-31T17:59:11.1097346Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-31T17:59:11.1097530Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-31T17:59:11.1097678Z 
2019-08-31T17:59:11.1097795Z 
2019-08-31T17:59:11.1099554Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-31T17:59:11.1100176Z 
2019-08-31T17:59:11.1100295Z 
2019-08-31T17:59:11.1100457Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-31T17:59:11.1100613Z Build completed unsuccessfully in 1:16:11
2019-08-31T17:59:11.1100613Z Build completed unsuccessfully in 1:16:11
2019-08-31T17:59:11.1115151Z == clock drift check ==
2019-08-31T17:59:11.1127943Z   local time: Sat Aug 31 17:59:11 UTC 2019
2019-08-31T17:59:11.2628029Z   network time: Sat, 31 Aug 2019 17:59:11 GMT
2019-08-31T17:59:11.2628152Z == end clock drift check ==
2019-08-31T17:59:13.4194092Z ##[error]Bash exited with code '1'.
2019-08-31T17:59:13.4246150Z ##[section]Starting: Checkout
2019-08-31T17:59:13.4248004Z ==============================================================================
2019-08-31T17:59:13.4248058Z Task         : Get sources
2019-08-31T17:59:13.4248289Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
