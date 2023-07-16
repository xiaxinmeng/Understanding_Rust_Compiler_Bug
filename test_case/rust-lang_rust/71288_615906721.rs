plain
2020-04-18T16:19:00.6570812Z ========================== Starting Command Output ===========================
2020-04-18T16:19:00.6573865Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d1aa127b-d5f7-4a40-adb7-fa3889df2788.sh
2020-04-18T16:19:00.6574254Z 
2020-04-18T16:19:00.6578843Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-18T16:19:00.6597279Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71288/merge to s
2020-04-18T16:19:00.6601695Z Task         : Get sources
2020-04-18T16:19:00.6601982Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T16:19:00.6602251Z Version      : 1.0.0
2020-04-18T16:19:00.6602436Z Author       : Microsoft
---
2020-04-18T16:19:01.6436623Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-18T16:19:01.6446032Z ##[command]git config gc.auto 0
2020-04-18T16:19:01.6452063Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-18T16:19:01.6457664Z ##[command]git config --get-all http.proxy
2020-04-18T16:19:01.6468374Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71288/merge:refs/remotes/pull/71288/merge
---
2020-04-18T16:21:56.5121056Z  ---> 318032b5f0e2
2020-04-18T16:21:56.5121741Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-18T16:21:56.5122290Z  ---> Using cache
2020-04-18T16:21:56.5122602Z  ---> d44a858fd1ce
2020-04-18T16:21:56.5123453Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-18T16:21:56.5127336Z  ---> 58b910f50f5a
2020-04-18T16:21:56.5127548Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-18T16:21:56.5127868Z  ---> Using cache
2020-04-18T16:21:56.5128166Z  ---> ee7702aadba1
---
2020-04-18T16:21:57.1930059Z Looks like docker image is the same as before, not uploading
2020-04-18T16:22:02.8642646Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-18T16:22:02.8989508Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-18T16:22:02.9016480Z == clock drift check ==
2020-04-18T16:22:02.9026118Z   local time: Sat Apr 18 16:22:02 UTC 2020
2020-04-18T16:22:03.2276940Z   network time: Sat, 18 Apr 2020 16:22:03 GMT
2020-04-18T16:22:03.2298088Z Starting sccache server...
2020-04-18T16:22:03.3009912Z configure: processing command line
2020-04-18T16:22:03.3018767Z configure: 
2020-04-18T16:22:03.3019849Z configure: rust.dist-src        := False
---
2020-04-18T16:26:36.8380098Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T16:26:38.0822781Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T16:26:39.4462453Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T16:26:40.0825897Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T16:26:48.1725645Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T16:26:49.8923457Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T16:26:53.7344833Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T16:26:57.4141630Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T16:27:05.8684768Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T16:47:34.8670117Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T16:47:36.3499505Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T16:47:38.0420382Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T16:47:38.0684538Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T16:47:47.9455491Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T16:47:49.8610743Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T16:47:54.2746495Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T16:47:58.4447825Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T16:48:08.3679740Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T17:09:01.3688370Z .................................................................................................... 1700/9905
2020-04-18T17:09:05.0962560Z .................................................................................................... 1800/9905
2020-04-18T17:09:13.4253143Z .................................................................................................i.. 1900/9905
2020-04-18T17:09:20.4642862Z .................................................................................................... 2000/9905
2020-04-18T17:09:26.2717064Z .......................................................................................iiiii........ 2100/9905
2020-04-18T17:09:43.9944702Z .................................................................................................... 2300/9905
2020-04-18T17:09:45.9666005Z .................................................................................................... 2400/9905
2020-04-18T17:09:48.0416602Z .................................................................................................... 2500/9905
2020-04-18T17:09:53.5381249Z .................................................................................................... 2600/9905
---
2020-04-18T17:12:25.2173069Z .................................................................................................... 4900/9905
2020-04-18T17:12:29.7910625Z ...............................................................i...............i.................... 5000/9905
2020-04-18T17:12:36.5737824Z .................................................................................................... 5100/9905
2020-04-18T17:12:42.9109452Z .................................................................................................... 5200/9905
2020-04-18T17:12:47.6385182Z .........i.........................................................................................i 5300/9905
2020-04-18T17:12:56.6867802Z ...................................................................................................i 5400/9905
2020-04-18T17:13:01.1546307Z i.ii........i...i................................................................................... 5500/9905
2020-04-18T17:13:08.3450269Z .............................................i...................................................... 5700/9905
2020-04-18T17:13:16.6752312Z .............................................................................ii..................... 5800/9905
2020-04-18T17:13:22.9854874Z ................i................................................................................... 5900/9905
2020-04-18T17:13:27.9564527Z .................................................................................................... 6000/9905
2020-04-18T17:13:27.9564527Z .................................................................................................... 6000/9905
2020-04-18T17:13:37.5829497Z .................................................................................................... 6100/9905
2020-04-18T17:13:47.4140251Z ..........ii...i..ii...........i.................................................................... 6200/9905
2020-04-18T17:14:01.8145416Z .................................................................................................... 6400/9905
2020-04-18T17:14:04.9357949Z .................................................................................................... 6500/9905
2020-04-18T17:14:04.9357949Z .................................................................................................... 6500/9905
2020-04-18T17:14:14.9840678Z ........................................i..ii....................................................... 6600/9905
2020-04-18T17:14:34.4483708Z .................................................................................................... 6800/9905
2020-04-18T17:14:36.3419944Z .........................................i.......................................................... 6900/9905
2020-04-18T17:14:38.2209266Z .................................................................................................... 7000/9905
2020-04-18T17:14:40.1588775Z .................................................................................i.F................ 7100/9905
---
2020-04-18T17:16:05.2913930Z .................................................................................................... 7800/9905
2020-04-18T17:16:09.3542892Z .................................................................................................... 7900/9905
2020-04-18T17:16:15.4094598Z .................................................................................................... 8000/9905
2020-04-18T17:16:20.8092639Z ...............................................i.................................................... 8100/9905
2020-04-18T17:16:29.8741004Z ................................................................................................iiii 8200/9905
2020-04-18T17:16:35.0668551Z ii.iiiii.i.......................................................................................... 8300/9905
2020-04-18T17:16:47.3047893Z .................................................................................................... 8500/9905
2020-04-18T17:16:54.9608629Z .................................................................................................... 8600/9905
2020-04-18T17:17:08.0696402Z .................................................................................................... 8700/9905
2020-04-18T17:17:14.9884394Z .................................................................................................... 8800/9905
---
2020-04-18T17:18:56.7909090Z failures:
2020-04-18T17:18:56.7942676Z 
2020-04-18T17:18:56.7943518Z ---- [ui] ui/parser/mod_file_not_exist.rs stdout ----
2020-04-18T17:18:56.7943825Z 
2020-04-18T17:18:56.7944774Z error: /checkout/src/test/ui/parser/mod_file_not_exist.rs:3: unexpected help message: '3:1: 3:21: to create the module `not_a_real_file` here, create file "/checkout/src/test/ui/parser/not_a_real_file.rs"'
2020-04-18T17:18:56.7945322Z 
2020-04-18T17:18:56.7946279Z error: /checkout/src/test/ui/parser/mod_file_not_exist.rs:3: unexpected help message: '3:1: 3:21: if there is `mod not_a_real_file` elsewhere in the crate already, import it with `use crate::` instead'
2020-04-18T17:18:56.7946852Z 
2020-04-18T17:18:56.8013729Z error: /checkout/src/test/ui/parser/mod_file_not_exist.rs:3: expected help message not found: to create the module `not_a_real_file`, create file
2020-04-18T17:18:56.8014258Z error: 2 unexpected errors found, 1 expected errors not found
2020-04-18T17:18:56.8021215Z status: exit code: 1
2020-04-18T17:18:56.8021215Z status: exit code: 1
2020-04-18T17:18:56.8023385Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/mod_file_not_exist.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mod_file_not_exist" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mod_file_not_exist/auxiliary"
2020-04-18T17:18:56.8024726Z     Error {
2020-04-18T17:18:56.8024914Z         line_num: 3,
2020-04-18T17:18:56.8025096Z         kind: Some(
2020-04-18T17:18:56.8025265Z             Help,
2020-04-18T17:18:56.8025265Z             Help,
2020-04-18T17:18:56.8025431Z         ),
2020-04-18T17:18:56.8025774Z         msg: "3:1: 3:21: to create the module `not_a_real_file` here, create file \"/checkout/src/test/ui/parser/not_a_real_file.rs\"",
2020-04-18T17:18:56.8026456Z     Error {
2020-04-18T17:18:56.8026619Z         line_num: 3,
2020-04-18T17:18:56.8026800Z         kind: Some(
2020-04-18T17:18:56.8026986Z             Help,
2020-04-18T17:18:56.8026986Z             Help,
2020-04-18T17:18:56.8027136Z         ),
2020-04-18T17:18:56.8027510Z         msg: "3:1: 3:21: if there is `mod not_a_real_file` elsewhere in the crate already, import it with `use crate::` instead",
2020-04-18T17:18:56.8028013Z ]
2020-04-18T17:18:56.8028108Z 
2020-04-18T17:18:56.8028482Z not found errors (from test file): [
2020-04-18T17:18:56.8028687Z     Error {
2020-04-18T17:18:56.8028687Z     Error {
2020-04-18T17:18:56.8028840Z         line_num: 3,
2020-04-18T17:18:56.8029008Z         kind: Some(
2020-04-18T17:18:56.8029180Z             Help,
2020-04-18T17:18:56.8029320Z         ),
2020-04-18T17:18:56.8029532Z         msg: "to create the module `not_a_real_file`, create file",
2020-04-18T17:18:56.8029870Z ]
2020-04-18T17:18:56.8029957Z 
2020-04-18T17:18:56.8030505Z thread '[ui] ui/parser/mod_file_not_exist.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1460:13
2020-04-18T17:18:56.8030909Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
---
2020-04-18T17:18:56.8032308Z 
2020-04-18T17:18:56.8032752Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-18T17:18:56.8032963Z 
2020-04-18T17:18:56.8033048Z 
2020-04-18T17:18:56.8036173Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-18T17:18:56.8039990Z 
2020-04-18T17:18:56.8040079Z 
2020-04-18T17:18:56.8040612Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-18T17:18:56.8040924Z Build completed unsuccessfully in 0:55:28
2020-04-18T17:18:56.8040924Z Build completed unsuccessfully in 0:55:28
2020-04-18T17:18:56.8041662Z == clock drift check ==
2020-04-18T17:18:56.8057786Z   local time: Sat Apr 18 17:18:56 UTC 2020
2020-04-18T17:18:56.9731589Z   network time: Sat, 18 Apr 2020 17:18:56 GMT
2020-04-18T17:18:57.4470788Z 
2020-04-18T17:18:57.4470788Z 
2020-04-18T17:18:57.4546688Z ##[error]Bash exited with code '1'.
2020-04-18T17:18:57.4560917Z ##[section]Finishing: Run build
2020-04-18T17:18:57.4612722Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71288/merge to s
2020-04-18T17:18:57.4617641Z Task         : Get sources
2020-04-18T17:18:57.4617969Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T17:18:57.4618255Z Version      : 1.0.0
2020-04-18T17:18:57.4618459Z Author       : Microsoft
2020-04-18T17:18:57.4618459Z Author       : Microsoft
2020-04-18T17:18:57.4618804Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-18T17:18:57.4619162Z ==============================================================================
2020-04-18T17:18:57.7832089Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-18T17:18:57.7887810Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71288/merge to s
2020-04-18T17:18:57.8017675Z Cleaning up task key
2020-04-18T17:18:57.8018912Z Start cleaning up orphan processes.
2020-04-18T17:18:57.8193907Z Terminate orphan process: pid (3682) (python)
2020-04-18T17:18:57.8348934Z ##[section]Finishing: Finalize Job
