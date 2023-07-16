plain
2020-04-19T20:56:33.0687324Z ========================== Starting Command Output ===========================
2020-04-19T20:56:33.0689723Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fbd6fbaa-71b4-4773-a4e0-0491842c3554.sh
2020-04-19T20:56:33.0689967Z 
2020-04-19T20:56:33.0694083Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-19T20:56:33.0713025Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71333/merge to s
2020-04-19T20:56:33.0716568Z Task         : Get sources
2020-04-19T20:56:33.0716849Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T20:56:33.0717132Z Version      : 1.0.0
2020-04-19T20:56:33.0717337Z Author       : Microsoft
---
2020-04-19T20:56:34.3557005Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-19T20:56:35.3473000Z ##[command]git config gc.auto 0
2020-04-19T20:56:35.3476938Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-19T20:56:35.3480286Z ##[command]git config --get-all http.proxy
2020-04-19T20:56:35.3487355Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71333/merge:refs/remotes/pull/71333/merge
---
2020-04-19T20:59:38.7964501Z  ---> 318032b5f0e2
2020-04-19T20:59:38.7965388Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-19T20:59:38.7966102Z  ---> Using cache
2020-04-19T20:59:38.7966552Z  ---> d44a858fd1ce
2020-04-19T20:59:38.7967597Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-19T20:59:38.7968845Z  ---> 58b910f50f5a
2020-04-19T20:59:38.7969183Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-19T20:59:38.7969646Z  ---> Using cache
2020-04-19T20:59:38.7970093Z  ---> ee7702aadba1
---
2020-04-19T20:59:38.8309420Z Looks like docker image is the same as before, not uploading
2020-04-19T20:59:45.2230601Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-19T20:59:45.2556981Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-19T20:59:45.2580825Z == clock drift check ==
2020-04-19T20:59:45.2592129Z   local time: Sun Apr 19 20:59:45 UTC 2020
2020-04-19T20:59:45.3057565Z   network time: Sun, 19 Apr 2020 20:59:45 GMT
2020-04-19T20:59:45.3085498Z Starting sccache server...
2020-04-19T20:59:45.3914697Z configure: processing command line
2020-04-19T20:59:45.3915348Z configure: 
2020-04-19T20:59:45.3916762Z configure: rust.dist-src        := False
---
2020-04-19T21:04:52.3789594Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T21:04:53.8197824Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T21:04:55.3988938Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T21:04:56.4974440Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T21:05:05.1195536Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T21:05:07.5603360Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T21:05:11.7246003Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T21:05:15.6398808Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T21:05:24.7487272Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T21:26:16.3371789Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T21:26:17.8582625Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T21:26:19.6124777Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T21:26:19.6603149Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T21:26:29.6650305Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T21:26:31.5553190Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T21:26:35.9690267Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T21:26:40.1050432Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T21:26:50.1957397Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T21:47:10.1227853Z .................................................................................................... 1700/9908
2020-04-19T21:47:13.8041080Z .................................................................................................... 1800/9908
2020-04-19T21:47:22.1673035Z ...................................................................................................i 1900/9908
2020-04-19T21:47:29.2583895Z .................................................................................................... 2000/9908
2020-04-19T21:47:34.9788140Z .........................................................................................iiiii...... 2100/9908
2020-04-19T21:47:52.9112365Z .................................................................................................... 2300/9908
2020-04-19T21:47:54.9191587Z .................................................................................................... 2400/9908
2020-04-19T21:47:56.9929745Z .................................................................................................... 2500/9908
2020-04-19T21:48:02.6742326Z .................................................................................................... 2600/9908
---
2020-04-19T21:50:33.4078127Z .................................................................i...............i.................. 5000/9908
2020-04-19T21:50:39.9098732Z .................................................................................................... 5100/9908
2020-04-19T21:50:45.9584805Z .................................................................................................... 5200/9908
2020-04-19T21:50:50.4051726Z ...........i........................................................................................ 5300/9908
2020-04-19T21:50:58.7469888Z .i.................................................................................................. 5400/9908
2020-04-19T21:51:03.1540619Z .ii.ii........i...i................................................................................. 5500/9908
2020-04-19T21:51:10.0505367Z ................................................i................................................... 5700/9908
2020-04-19T21:51:18.1280757Z ................................................................................ii.................. 5800/9908
2020-04-19T21:51:24.5480088Z ...................i................................................................................ 5900/9908
2020-04-19T21:51:29.3620500Z .................................................................................................... 6000/9908
2020-04-19T21:51:29.3620500Z .................................................................................................... 6000/9908
2020-04-19T21:51:38.9833935Z .................................................................................................... 6100/9908
2020-04-19T21:51:48.3191402Z .............ii...i..i.i..........i................................................................. 6200/9908
2020-04-19T21:52:01.1586486Z .................................................................................................... 6400/9908
2020-04-19T21:52:04.2200753Z .................................................................................................... 6500/9908
2020-04-19T21:52:04.2200753Z .................................................................................................... 6500/9908
2020-04-19T21:52:13.8239688Z ...........................................i..ii.................................................... 6600/9908
2020-04-19T21:52:32.8053125Z .................................................................................................... 6800/9908
2020-04-19T21:52:34.9413537Z ............................................i....................................................... 6900/9908
2020-04-19T21:52:36.9756114Z .................................................................................................... 7000/9908
2020-04-19T21:52:38.9841520Z ....................................................................................i............... 7100/9908
---
2020-04-19T21:54:04.3072136Z .................................................................................................... 7800/9908
2020-04-19T21:54:08.8651901Z .................................................................................................... 7900/9908
2020-04-19T21:54:15.0321093Z .................................................................................................... 8000/9908
2020-04-19T21:54:20.2590158Z ..................................................i................................................. 8100/9908
2020-04-19T21:54:29.5007231Z ...................................................................................................i 8200/9908
2020-04-19T21:54:34.4480821Z iiiii.iiiii.i....................................................................................... 8300/9908
2020-04-19T21:54:46.3910334Z .................................................................................................... 8500/9908
2020-04-19T21:54:53.5290725Z .................................................................................................... 8600/9908
2020-04-19T21:55:05.7149065Z .................................................................................................... 8700/9908
2020-04-19T21:55:11.5329221Z .................................................................................................... 8800/9908
---
2020-04-19T21:56:40.8803829Z ............i....................................................................................... 9900/9908
2020-04-19T21:56:45.6429718Z ........
2020-04-19T21:56:45.6430114Z failures:
2020-04-19T21:56:45.6481851Z 
2020-04-19T21:56:45.6482743Z ---- [ui] ui/bogus-error-#39161.rs stdout ----
2020-04-19T21:56:45.6483334Z error: test compilation failed although it shouldn't!
2020-04-19T21:56:45.6483591Z status: exit code: 1
2020-04-19T21:56:45.6483591Z status: exit code: 1
2020-04-19T21:56:45.6485263Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/bogus-error-#39161.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bogus-error-#39161/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bogus-error-#39161/auxiliary"
2020-04-19T21:56:45.6486704Z ------------------------------------------
2020-04-19T21:56:45.6486851Z 
2020-04-19T21:56:45.6487195Z ------------------------------------------
2020-04-19T21:56:45.6487371Z stderr:
2020-04-19T21:56:45.6487371Z stderr:
2020-04-19T21:56:45.6487783Z ------------------------------------------
2020-04-19T21:56:45.6488051Z error: invalid character `#` in crate name: `bogus_error_#39161`
2020-04-19T21:56:45.6488413Z error: aborting due to previous error
2020-04-19T21:56:45.6488552Z 
2020-04-19T21:56:45.6488636Z 
2020-04-19T21:56:45.6488975Z ------------------------------------------
2020-04-19T21:56:45.6488975Z ------------------------------------------
2020-04-19T21:56:45.6489119Z 
2020-04-19T21:56:45.6489202Z 
2020-04-19T21:56:45.6489284Z 
2020-04-19T21:56:45.6489416Z failures:
2020-04-19T21:56:45.6489741Z     [ui] ui/bogus-error-#39161.rs
2020-04-19T21:56:45.6490316Z test result: FAILED. 9846 passed; 1 failed; 61 ignored; 0 measured; 0 filtered out
2020-04-19T21:56:45.6490550Z 
2020-04-19T21:56:45.6531957Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-19T21:56:45.6532496Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-19T21:56:45.6532496Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-19T21:56:45.6552086Z 
2020-04-19T21:56:45.6561938Z 
2020-04-19T21:56:45.6565379Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-19T21:56:45.6567503Z 
2020-04-19T21:56:45.6567592Z 
2020-04-19T21:56:45.6568070Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-19T21:56:45.6568383Z Build completed unsuccessfully in 0:55:23
2020-04-19T21:56:45.6568383Z Build completed unsuccessfully in 0:55:23
2020-04-19T21:56:45.6613821Z == clock drift check ==
2020-04-19T21:56:45.6634035Z   local time: Sun Apr 19 21:56:45 UTC 2020
2020-04-19T21:56:45.8177963Z   network time: Sun, 19 Apr 2020 21:56:45 GMT
2020-04-19T21:56:46.3644905Z 
2020-04-19T21:56:46.3644905Z 
2020-04-19T21:56:46.3711014Z ##[error]Bash exited with code '1'.
2020-04-19T21:56:46.3723986Z ##[section]Finishing: Run build
2020-04-19T21:56:46.3766198Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71333/merge to s
2020-04-19T21:56:46.3770782Z Task         : Get sources
2020-04-19T21:56:46.3771067Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T21:56:46.3771345Z Version      : 1.0.0
2020-04-19T21:56:46.3771536Z Author       : Microsoft
2020-04-19T21:56:46.3771536Z Author       : Microsoft
2020-04-19T21:56:46.3771835Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-19T21:56:46.3772183Z ==============================================================================
2020-04-19T21:56:46.6674612Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-19T21:56:46.6723241Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71333/merge to s
2020-04-19T21:56:46.6812687Z Cleaning up task key
2020-04-19T21:56:46.6813843Z Start cleaning up orphan processes.
2020-04-19T21:56:46.6976507Z Terminate orphan process: pid (3701) (python)
2020-04-19T21:56:46.7123189Z ##[section]Finishing: Finalize Job
