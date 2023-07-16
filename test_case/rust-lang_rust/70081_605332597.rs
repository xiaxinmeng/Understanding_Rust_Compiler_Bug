plain
2020-03-27T20:57:51.8643939Z ========================== Starting Command Output ===========================
2020-03-27T20:57:51.8648297Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f3dd1e17-6bec-434e-b5c4-35ff34fb4e46.sh
2020-03-27T20:57:51.8648784Z 
2020-03-27T20:57:51.8654878Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-27T20:57:51.8682717Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70081/merge to s
2020-03-27T20:57:51.8687230Z Task         : Get sources
2020-03-27T20:57:51.8687677Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-27T20:57:51.8688099Z Version      : 1.0.0
2020-03-27T20:57:51.8688373Z Author       : Microsoft
---
2020-03-27T20:57:52.8804088Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-27T20:57:52.8810532Z ##[command]git config gc.auto 0
2020-03-27T20:57:52.8815289Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-27T20:57:52.8819403Z ##[command]git config --get-all http.proxy
2020-03-27T20:57:52.8826268Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70081/merge:refs/remotes/pull/70081/merge
---
2020-03-27T21:04:30.3231094Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-27T21:04:31.8008644Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-27T21:04:32.2741107Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-27T21:04:40.8915826Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-27T21:04:42.3787970Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-27T21:04:43.6463083Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-27T21:04:51.7091033Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-27T21:05:20.9196326Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-27T21:05:52.4503499Z    Compiling rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-27T21:07:32.3408335Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
2020-03-27T21:24:52.3748624Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-27T21:24:54.1703179Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-27T21:24:55.1876448Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-27T21:25:04.9274045Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-27T21:25:07.0564406Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-27T21:25:08.4128322Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-27T21:25:18.0806216Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-27T21:25:54.3714166Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-27T21:26:34.8437234Z    Compiling rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-27T21:28:32.9597701Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
2020-03-27T21:47:57.9818931Z 8 
2020-03-27T21:47:57.9819295Z + warning: unnecessary braces around const expression
2020-03-27T21:47:57.9823041Z +   --> $DIR/issue-70125-2.rs:16:10
2020-03-27T21:47:57.9823385Z +    |
2020-03-27T21:47:57.9825513Z + LL | impl Foo<{3}> for () {}
2020-03-27T21:47:57.9825962Z +    |          ^^^ help: remove these braces
2020-03-27T21:47:57.9828171Z +    = note: `#[warn(unused_braces)]` on by default
2020-03-27T21:47:57.9829871Z + 
2020-03-27T21:47:57.9830152Z 9 
2020-03-27T21:47:57.9832694Z 
2020-03-27T21:47:57.9832694Z 
2020-03-27T21:47:58.0248457Z .F. 1400/9852
2020-03-27T21:48:04.1132736Z .................................................................................................... 1500/9852
2020-03-27T21:48:09.0005379Z .................................................................................................... 1600/9852
2020-03-27T21:48:14.6844192Z .................................................................................................... 1700/9852
2020-03-27T21:48:18.4219678Z .................................................................................................... 1800/9852
2020-03-27T21:48:27.5290908Z ..........................................................................................i......... 1900/9852
2020-03-27T21:48:33.8259733Z .................................................................................................... 2000/9852
2020-03-27T21:48:39.6812368Z ................................................................................iiiii............... 2100/9852
2020-03-27T21:48:58.8520774Z .................................................................................................... 2300/9852
2020-03-27T21:49:00.9308467Z .................................................................................................... 2400/9852
2020-03-27T21:49:03.1878483Z .................................................................................................... 2500/9852
2020-03-27T21:49:09.2178501Z .................................................................................................... 2600/9852
---
2020-03-27T21:51:47.5244069Z ......................................................i...............i............................. 5000/9852
2020-03-27T21:51:55.0730878Z .................................................................................................... 5100/9852
2020-03-27T21:52:02.1644780Z ...................................................................................................i 5200/9852
2020-03-27T21:52:07.1780928Z .................................................................................................... 5300/9852
2020-03-27T21:52:17.7033898Z ....................................................................................ii.ii........i.. 5400/9852
2020-03-27T21:52:21.1716544Z .i.................................................................................................. 5500/9852
2020-03-27T21:52:30.0932156Z .............................i...................................................................... 5700/9852
2020-03-27T21:52:39.4280247Z ................................................ii....................................i............. 5800/9852
2020-03-27T21:52:46.7725904Z .................................................................................................... 5900/9852
2020-03-27T21:52:51.5541838Z .................................................................................................... 6000/9852
2020-03-27T21:52:51.5541838Z .................................................................................................... 6000/9852
2020-03-27T21:53:00.3087436Z ................................................................................ii...i..ii.......... 6100/9852
2020-03-27T21:53:12.0398054Z .i.................................................................................................. 6200/9852
2020-03-27T21:53:22.3381945Z .................................................................................................... 6400/9852
2020-03-27T21:53:25.5945900Z .................................................................................................... 6500/9852
2020-03-27T21:53:25.5945900Z .................................................................................................... 6500/9852
2020-03-27T21:53:36.9083479Z ..........i..ii..................................................................................... 6600/9852
2020-03-27T21:53:55.6437281Z .................................................................................................... 6800/9852
2020-03-27T21:53:57.5723477Z ..........i......................................................................................... 6900/9852
2020-03-27T21:53:59.5195504Z .................................................................................................... 7000/9852
2020-03-27T21:54:01.6287902Z ..............................................i..................................................... 7100/9852
---
2020-03-27T21:55:32.8113138Z .................................................................................................... 7800/9852
2020-03-27T21:55:37.4965013Z .................................................................................................... 7900/9852
2020-03-27T21:55:43.9816546Z .................................................................................................... 8000/9852
2020-03-27T21:55:50.8675840Z ...i................................................................................................ 8100/9852
2020-03-27T21:55:58.2710845Z ....................................................iiiiiiiiii.i.................................... 8200/9852
2020-03-27T21:56:11.1652227Z ...i................................................................................................ 8400/9852
2020-03-27T21:56:15.8885229Z .................................................................................................... 8500/9852
2020-03-27T21:56:27.8355847Z .................................................................................................... 8600/9852
2020-03-27T21:56:36.6464820Z .................................................................................................... 8700/9852
---
2020-03-27T21:58:22.1039572Z diff of stderr:
2020-03-27T21:58:22.1039865Z 
2020-03-27T21:58:22.1040148Z 
2020-03-27T21:58:22.1040525Z The actual stderr differed from the expected stderr.
2020-03-27T21:58:22.1041414Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-70125-2/issue-70125-2.stderr
2020-03-27T21:58:22.1042280Z To update references, rerun the tests and pass the `--bless` flag
2020-03-27T21:58:22.1043880Z To only update this specific test, also pass `--test-args const-generics/issues/issue-70125-2.rs`
2020-03-27T21:58:22.1044706Z error: 1 errors occurred comparing output.
2020-03-27T21:58:22.1045043Z status: exit code: 0
2020-03-27T21:58:22.1045043Z status: exit code: 0
2020-03-27T21:58:22.1047339Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-70125-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-70125-2/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-70125-2/auxiliary"
2020-03-27T21:58:22.1049055Z ------------------------------------------
2020-03-27T21:58:22.1049342Z 
2020-03-27T21:58:22.1049799Z ------------------------------------------
2020-03-27T21:58:22.1050098Z stderr:
---
2020-03-27T21:58:22.1053501Z 
2020-03-27T21:58:22.1053792Z warning: unnecessary braces around const expression
2020-03-27T21:58:22.1054413Z   --> /checkout/src/test/ui/const-generics/issues/issue-70125-2.rs:16:10
2020-03-27T21:58:22.1054777Z    |
2020-03-27T21:58:22.1055035Z LL | impl Foo<{3}> for () {}
2020-03-27T21:58:22.1055378Z    |          ^^^ help: remove these braces
2020-03-27T21:58:22.1055967Z    = note: `#[warn(unused_braces)]` on by default
2020-03-27T21:58:22.1056234Z 
2020-03-27T21:58:22.1056436Z 
2020-03-27T21:58:22.1056880Z ------------------------------------------
---
2020-03-27T21:58:22.1064405Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-27T21:58:22.1064970Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-27T21:58:22.1079077Z 
2020-03-27T21:58:22.1079396Z 
2020-03-27T21:58:22.1083602Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-27T21:58:22.1087284Z 
2020-03-27T21:58:22.1087391Z 
2020-03-27T21:58:22.1091028Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-27T21:58:22.1091420Z Build completed unsuccessfully in 0:57:06
2020-03-27T21:58:22.1091420Z Build completed unsuccessfully in 0:57:06
2020-03-27T21:58:22.1144269Z == clock drift check ==
2020-03-27T21:58:22.1159855Z   local time: Fri Mar 27 21:58:22 UTC 2020
2020-03-27T21:58:22.4108297Z   network time: Fri, 27 Mar 2020 21:58:22 GMT
2020-03-27T21:58:22.4114585Z == end clock drift check ==
2020-03-27T21:58:22.8344320Z 
2020-03-27T21:58:22.8379510Z ##[error]Bash exited with code '1'.
2020-03-27T21:58:22.8401138Z ##[section]Finishing: Run build
2020-03-27T21:58:22.8444089Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70081/merge to s
2020-03-27T21:58:22.8448854Z Task         : Get sources
2020-03-27T21:58:22.8449172Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-27T21:58:22.8449472Z Version      : 1.0.0
2020-03-27T21:58:22.8449690Z Author       : Microsoft
2020-03-27T21:58:22.8449690Z Author       : Microsoft
2020-03-27T21:58:22.8450010Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-27T21:58:22.8450382Z ==============================================================================
2020-03-27T21:58:23.1475299Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-27T21:58:23.1525029Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70081/merge to s
2020-03-27T21:58:23.1601833Z Cleaning up task key
2020-03-27T21:58:23.1602994Z Start cleaning up orphan processes.
2020-03-27T21:58:23.1764190Z Terminate orphan process: pid (3940) (python)
2020-03-27T21:58:23.1918824Z ##[section]Finishing: Finalize Job
