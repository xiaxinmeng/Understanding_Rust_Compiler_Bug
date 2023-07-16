plain
2020-04-22T19:44:45.9530176Z ========================== Starting Command Output ===========================
2020-04-22T19:44:45.9535390Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/700a45bf-b0e7-4051-bab7-28604050d868.sh
2020-04-22T19:44:45.9535907Z 
2020-04-22T19:44:45.9540051Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-22T19:44:45.9567587Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71235/merge to s
2020-04-22T19:44:45.9570963Z Task         : Get sources
2020-04-22T19:44:45.9571240Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T19:44:45.9571490Z Version      : 1.0.0
2020-04-22T19:44:45.9571659Z Author       : Microsoft
---
2020-04-22T19:44:46.9406036Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-22T19:44:46.9414984Z ##[command]git config gc.auto 0
2020-04-22T19:44:46.9431887Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-22T19:44:46.9435392Z ##[command]git config --get-all http.proxy
2020-04-22T19:44:46.9441731Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71235/merge:refs/remotes/pull/71235/merge
---
2020-04-22T19:46:54.6059432Z  ---> 318032b5f0e2
2020-04-22T19:46:54.6060610Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-22T19:46:54.6064091Z  ---> Using cache
2020-04-22T19:46:54.6064827Z  ---> d44a858fd1ce
2020-04-22T19:46:54.6066251Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-22T19:46:54.6070857Z  ---> 58b910f50f5a
2020-04-22T19:46:54.6071525Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-22T19:46:54.6075326Z  ---> Using cache
2020-04-22T19:46:54.6076069Z  ---> ee7702aadba1
---
2020-04-22T19:46:54.6591078Z Looks like docker image is the same as before, not uploading
2020-04-22T19:47:02.4767966Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-22T19:47:02.5084013Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-22T19:47:02.5108643Z == clock drift check ==
2020-04-22T19:47:02.5119400Z   local time: Wed Apr 22 19:47:02 UTC 2020
2020-04-22T19:47:02.8046035Z   network time: Wed, 22 Apr 2020 19:47:02 GMT
2020-04-22T19:47:02.8052260Z Starting sccache server...
2020-04-22T19:47:02.8953925Z configure: processing command line
2020-04-22T19:47:02.8954404Z configure: 
2020-04-22T19:47:02.8955335Z configure: rust.dist-src        := False
---
2020-04-22T19:52:33.3759061Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T19:52:34.9594159Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T19:52:36.5934541Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T19:52:38.6366123Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T19:52:47.4205889Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T19:52:50.6243969Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T19:52:55.3117466Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T19:52:59.7165596Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T19:53:09.3019438Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T20:17:02.2767192Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T20:17:04.1136593Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T20:17:06.0845617Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T20:17:06.2678719Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T20:17:17.7946831Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T20:17:20.1561568Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T20:17:25.3699712Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T20:17:30.2435675Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T20:17:41.3404053Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T20:41:39.7472321Z .................................................................................................... 1700/9916
2020-04-22T20:41:44.3455892Z .................................................................................................... 1800/9916
2020-04-22T20:41:53.2693674Z .................................................................................................... 1900/9916
2020-04-22T20:42:01.6194852Z ....i............................................................................................... 2000/9916
2020-04-22T20:42:07.9873948Z ..............................................................................................iiiii. 2100/9916
2020-04-22T20:42:28.4333805Z .................................................................................................... 2300/9916
2020-04-22T20:42:30.6654764Z .................................................................................................... 2400/9916
2020-04-22T20:42:33.1079400Z .................................................................................................... 2500/9916
2020-04-22T20:42:39.0477128Z .................................................................................................... 2600/9916
---
2020-04-22T20:45:42.0846504Z .................................................................................................... 5100/9916
2020-04-22T20:45:49.5696371Z .................................................................................................... 5200/9916
2020-04-22T20:45:54.2976713Z .................i.................................................................................. 5300/9916
2020-04-22T20:46:04.4973635Z .......i............................................................................................ 5400/9916
2020-04-22T20:46:10.0470722Z .......ii.ii........i...i........................................................................... 5500/9916
2020-04-22T20:46:17.8091342Z ......................................................i............................................. 5700/9916
2020-04-22T20:46:26.9320237Z .......................................................................................ii........... 5800/9916
2020-04-22T20:46:33.7239672Z ..........................i......................................................................... 5900/9916
2020-04-22T20:46:39.1463225Z .................................................................................................... 6000/9916
2020-04-22T20:46:39.1463225Z .................................................................................................... 6000/9916
2020-04-22T20:46:49.8627804Z .................................................................................................... 6100/9916
2020-04-22T20:46:59.9017390Z ....................ii...i..ii...........i.......................................................... 6200/9916
2020-04-22T20:47:15.7846806Z .................................................................................................... 6400/9916
2020-04-22T20:47:22.5287727Z .................................................................................................... 6500/9916
2020-04-22T20:47:22.5287727Z .................................................................................................... 6500/9916
2020-04-22T20:47:45.3232921Z ..................................................i..ii............................................. 6600/9916
2020-04-22T20:48:06.2922262Z .................................................................................................... 6800/9916
2020-04-22T20:48:08.6991699Z ...................................................i................................................ 6900/9916
2020-04-22T20:48:10.7480786Z .................................................................................................... 7000/9916
2020-04-22T20:48:12.9353010Z ...........................................................................................i........ 7100/9916
---
2020-04-22T20:49:53.1405619Z .................................................................................................... 7900/9916
2020-04-22T20:49:59.4346293Z .................................................................................................... 8000/9916
2020-04-22T20:50:05.3117441Z .........................................................i.......................................... 8100/9916
2020-04-22T20:50:15.3249670Z .................................................................................................... 8200/9916
2020-04-22T20:50:20.8284183Z ......iiiiii.iiiii.i................................................................................ 8300/9916
2020-04-22T20:50:34.6211131Z .................................................................................................... 8500/9916
2020-04-22T20:50:42.4528979Z .................................................................................................... 8600/9916
2020-04-22T20:50:55.7328414Z .................................................................................................... 8700/9916
2020-04-22T20:51:02.3604160Z .................................................................................................... 8800/9916
---
2020-04-22T20:52:51.9966352Z 13 note: these named lifetimes are available to use
2020-04-22T20:52:51.9967387Z -   --> $DIR/issue-65285-incorrect-explicit-lifetime-name-needed.rs:8:9
2020-04-22T20:52:51.9968210Z +   --> $DIR/issue-65285-incorrect-explicit-lifetime-name-needed.rs:9:12
2020-04-22T20:52:51.9968675Z 15    |
2020-04-22T20:52:51.9969243Z 16 LL | trait X<'a, K: 'a> {
2020-04-22T20:52:51.9969933Z 
2020-04-22T20:52:51.9970184Z 
2020-04-22T20:52:51.9970555Z The actual stderr differed from the expected stderr.
2020-04-22T20:52:51.9971504Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed/issue-65285-incorrect-explicit-lifetime-name-needed.stderr
2020-04-22T20:52:51.9971504Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed/issue-65285-incorrect-explicit-lifetime-name-needed.stderr
2020-04-22T20:52:51.9972684Z To update references, rerun the tests and pass the `--bless` flag
2020-04-22T20:52:51.9973623Z To only update this specific test, also pass `--test-args generics/issue-65285-incorrect-explicit-lifetime-name-needed.rs`
2020-04-22T20:52:51.9974784Z error: 1 errors occurred comparing output.
2020-04-22T20:52:51.9975192Z status: exit code: 1
2020-04-22T20:52:51.9975192Z status: exit code: 1
2020-04-22T20:52:51.9977579Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed/auxiliary"
2020-04-22T20:52:51.9981349Z ------------------------------------------
2020-04-22T20:52:51.9981497Z 
2020-04-22T20:52:51.9981833Z ------------------------------------------
2020-04-22T20:52:51.9982007Z stderr:
2020-04-22T20:52:51.9982007Z stderr:
2020-04-22T20:52:51.9982855Z ------------------------------------------
2020-04-22T20:52:51.9983301Z error[E0637]: `&` without an explicit lifetime name cannot be used here
2020-04-22T20:52:51.9984010Z   --> /checkout/src/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed.rs:5:37
2020-04-22T20:52:51.9984304Z    |
2020-04-22T20:52:51.9984533Z LL | fn should_error<T>() where T : Into<&u32> {}
2020-04-22T20:52:51.9985062Z 
2020-04-22T20:52:51.9985239Z error[E0106]: missing lifetime specifier
2020-04-22T20:52:51.9986252Z   --> /checkout/src/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed.rs:9:21
2020-04-22T20:52:51.9986524Z    |
2020-04-22T20:52:51.9986524Z    |
2020-04-22T20:52:51.9986895Z LL |     fn foo<'b, L: X<&'b Nested<K>>>();
2020-04-22T20:52:51.9987364Z    |
2020-04-22T20:52:51.9987538Z note: these named lifetimes are available to use
2020-04-22T20:52:51.9988089Z   --> /checkout/src/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed.rs:9:12
2020-04-22T20:52:51.9988350Z    |
2020-04-22T20:52:51.9988350Z    |
2020-04-22T20:52:51.9988644Z LL | trait X<'a, K: 'a> {
2020-04-22T20:52:51.9988828Z    |         ^^
2020-04-22T20:52:51.9989178Z LL |     fn foo<'b, L: X<&'b Nested<K>>>();
2020-04-22T20:52:51.9989595Z help: consider using one of the available lifetimes here
2020-04-22T20:52:51.9989780Z    |
2020-04-22T20:52:51.9989780Z    |
2020-04-22T20:52:51.9990323Z LL |     fn foo<'b, L: X<'lifetime, &'b Nested<K>>>();
2020-04-22T20:52:51.9990719Z 
2020-04-22T20:52:51.9990878Z error[E0106]: missing lifetime specifier
2020-04-22T20:52:51.9991438Z   --> /checkout/src/test/ui/generics/issue-65285-incorrect-explicit-lifetime-name-needed.rs:13:17
2020-04-22T20:52:51.9991728Z    |
2020-04-22T20:52:51.9991728Z    |
2020-04-22T20:52:51.9992175Z LL | fn bar<'b, L: X<&'b Nested<i32>>>(){}
2020-04-22T20:52:51.9992767Z    |
2020-04-22T20:52:51.9993123Z help: consider using the `'b` lifetime
2020-04-22T20:52:51.9993303Z    |
2020-04-22T20:52:51.9993303Z    |
2020-04-22T20:52:51.9993692Z LL | fn bar<'b, L: X<'b, &'b Nested<i32>>>(){}
2020-04-22T20:52:51.9994050Z 
2020-04-22T20:52:51.9994225Z error: aborting due to 3 previous errors
2020-04-22T20:52:51.9994398Z 
2020-04-22T20:52:51.9994608Z Some errors have detailed explanations: E0106, E0637.
---
2020-04-22T20:52:51.9997504Z test result: FAILED. 9854 passed; 1 failed; 61 ignored; 0 measured; 0 filtered out
2020-04-22T20:52:51.9997742Z 
2020-04-22T20:52:52.0007473Z 
2020-04-22T20:52:52.0007595Z 
2020-04-22T20:52:52.0010926Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-22T20:52:52.0013352Z 
2020-04-22T20:52:52.0013444Z 
2020-04-22T20:52:52.0013990Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-22T20:52:52.0014330Z Build completed unsuccessfully in 1:04:10
2020-04-22T20:52:52.0014330Z Build completed unsuccessfully in 1:04:10
2020-04-22T20:52:52.0014895Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-22T20:52:52.0015281Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-22T20:52:52.0060840Z == clock drift check ==
2020-04-22T20:52:52.0077887Z   local time: Wed Apr 22 20:52:52 UTC 2020
2020-04-22T20:52:52.3050508Z   network time: Wed, 22 Apr 2020 20:52:52 GMT
2020-04-22T20:52:52.7456655Z 
2020-04-22T20:52:52.7456655Z 
2020-04-22T20:52:52.7543661Z ##[error]Bash exited with code '1'.
2020-04-22T20:52:52.7569420Z ##[section]Finishing: Run build
2020-04-22T20:52:52.7625758Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71235/merge to s
2020-04-22T20:52:52.7631893Z Task         : Get sources
2020-04-22T20:52:52.7632256Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T20:52:52.7632608Z Version      : 1.0.0
2020-04-22T20:52:52.7632868Z Author       : Microsoft
2020-04-22T20:52:52.7632868Z Author       : Microsoft
2020-04-22T20:52:52.7633247Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-22T20:52:52.7633672Z ==============================================================================
2020-04-22T20:52:53.1022887Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-22T20:52:53.1068741Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71235/merge to s
2020-04-22T20:52:53.1151808Z Cleaning up task key
2020-04-22T20:52:53.1153520Z Start cleaning up orphan processes.
2020-04-22T20:52:53.1339198Z Terminate orphan process: pid (3646) (python)
2020-04-22T20:52:53.1511904Z ##[section]Finishing: Finalize Job
