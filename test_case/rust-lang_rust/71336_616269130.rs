plain
2020-04-20T00:55:05.9425771Z ========================== Starting Command Output ===========================
2020-04-20T00:55:05.9428280Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b59d0e1b-921b-45dd-9b23-c2f3ef9316db.sh
2020-04-20T00:55:05.9428593Z 
2020-04-20T00:55:05.9431688Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-20T00:55:05.9452902Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71336/merge to s
2020-04-20T00:55:05.9457045Z Task         : Get sources
2020-04-20T00:55:05.9457373Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-20T00:55:05.9457695Z Version      : 1.0.0
2020-04-20T00:55:05.9457912Z Author       : Microsoft
---
2020-04-20T00:55:06.9377404Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-20T00:55:06.9383812Z ##[command]git config gc.auto 0
2020-04-20T00:55:06.9387615Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-20T00:55:06.9390983Z ##[command]git config --get-all http.proxy
2020-04-20T00:55:06.9397132Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71336/merge:refs/remotes/pull/71336/merge
---
2020-04-20T00:58:42.3891819Z  ---> 318032b5f0e2
2020-04-20T00:58:42.3893567Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-20T00:58:42.3901537Z  ---> Using cache
2020-04-20T00:58:42.3901931Z  ---> d44a858fd1ce
2020-04-20T00:58:42.3902884Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-20T00:58:42.3909675Z  ---> 58b910f50f5a
2020-04-20T00:58:42.3910256Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-20T00:58:42.3917432Z  ---> Using cache
2020-04-20T00:58:42.3918151Z  ---> ee7702aadba1
---
2020-04-20T00:58:42.5379215Z Looks like docker image is the same as before, not uploading
2020-04-20T00:58:50.2549373Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-20T00:58:50.2847911Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-20T00:58:50.2879102Z == clock drift check ==
2020-04-20T00:58:50.2887952Z   local time: Mon Apr 20 00:58:50 UTC 2020
2020-04-20T00:58:50.4712538Z   network time: Mon, 20 Apr 2020 00:58:50 GMT
2020-04-20T00:58:50.4740294Z Starting sccache server...
2020-04-20T00:58:50.5603291Z configure: processing command line
2020-04-20T00:58:50.5608250Z configure: 
2020-04-20T00:58:50.5610344Z configure: rust.dist-src        := False
---
2020-04-20T01:04:03.4535420Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-20T01:04:04.9151397Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-20T01:04:06.5348645Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-20T01:04:07.6105628Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-20T01:04:16.7399713Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-20T01:04:18.5525355Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-20T01:04:22.8645488Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-20T01:04:26.9080174Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-20T01:04:37.4758253Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-20T01:26:47.4545351Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-20T01:26:49.0398896Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-20T01:26:50.8737682Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-20T01:26:50.9007737Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-20T01:27:01.4988556Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-20T01:27:03.5383105Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-20T01:27:08.2807857Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-20T01:27:12.6477932Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-20T01:27:23.4484516Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-20T01:49:59.1530773Z .................................................................................................... 1600/9909
2020-04-20T01:50:05.8475177Z .................................................................................................... 1700/9909
2020-04-20T01:50:10.1105646Z .................................................................................................... 1800/9909
2020-04-20T01:50:19.1071029Z .....F.............................................................................................. 1900/9909
2020-04-20T01:50:27.4836897Z i................................................................................................... 2000/9909
2020-04-20T01:50:33.7696337Z ..........................................................................................iiiii..... 2100/9909
2020-04-20T01:50:53.5136782Z .................................................................................................... 2300/9909
2020-04-20T01:50:55.7673664Z .................................................................................................... 2400/9909
2020-04-20T01:50:58.0784741Z .................................................................................................... 2500/9909
2020-04-20T01:51:04.1435546Z .................................................................................................... 2600/9909
---
2020-04-20T01:53:55.4824867Z ..................................................................i...............i................. 5000/9909
2020-04-20T01:54:02.9370618Z .................................................................................................... 5100/9909
2020-04-20T01:54:09.8256335Z .................................................................................................... 5200/9909
2020-04-20T01:54:14.8716562Z ............i....................................................................................... 5300/9909
2020-04-20T01:54:24.0955556Z ..i................................................................................................. 5400/9909
2020-04-20T01:54:29.0797054Z ..ii.ii........i...i................................................................................ 5500/9909
2020-04-20T01:54:36.6576649Z .................................................i.................................................. 5700/9909
2020-04-20T01:54:45.4432814Z .................................................................................ii................. 5800/9909
2020-04-20T01:54:52.4270433Z ....................i............................................................................... 5900/9909
2020-04-20T01:54:57.6190816Z .................................................................................................... 6000/9909
2020-04-20T01:54:57.6190816Z .................................................................................................... 6000/9909
2020-04-20T01:55:08.0527776Z .................................................................................................... 6100/9909
2020-04-20T01:55:18.0386358Z ..............ii...i..ii...........i................................................................ 6200/9909
2020-04-20T01:55:31.1284790Z .................................................................................................... 6400/9909
2020-04-20T01:55:34.5569456Z .................................................................................................... 6500/9909
2020-04-20T01:55:34.5569456Z .................................................................................................... 6500/9909
2020-04-20T01:55:45.0520737Z ............................................i..ii................................................... 6600/9909
2020-04-20T01:56:06.4499960Z .................................................................................................... 6800/9909
2020-04-20T01:56:08.6081628Z .............................................i...................................................... 6900/9909
2020-04-20T01:56:10.6728969Z .................................................................................................... 7000/9909
2020-04-20T01:56:12.7537560Z .....................................................................................i.............. 7100/9909
---
2020-04-20T01:57:48.1464890Z .................................................................................................... 7900/9909
2020-04-20T01:57:55.0763050Z .................................................................................................... 8000/9909
2020-04-20T01:57:59.9669404Z ...................................................i................................................ 8100/9909
2020-04-20T01:58:10.1028633Z .................................................................................................... 8200/9909
2020-04-20T01:58:15.4875362Z .iiiiiiiiiii.i...................................................................................... 8300/9909
2020-04-20T01:58:29.1966550Z .................................................................................................... 8500/9909
2020-04-20T01:58:37.1529912Z .................................................................................................... 8600/9909
2020-04-20T01:58:51.2313025Z .................................................................................................... 8700/9909
2020-04-20T01:58:57.7153042Z .................................................................................................... 8800/9909
---
2020-04-20T02:00:45.0485751Z 23 
2020-04-20T02:00:45.0486015Z 
2020-04-20T02:00:45.0486282Z 
2020-04-20T02:00:45.0486662Z The actual stderr differed from the expected stderr.
2020-04-20T02:00:45.0487502Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/inline_asm/inline_asm.stderr
2020-04-20T02:00:45.0488314Z To update references, rerun the tests and pass the `--bless` flag
2020-04-20T02:00:45.0489120Z To only update this specific test, also pass `--test-args consts/miri_unleashed/inline_asm.rs`
2020-04-20T02:00:45.0489901Z error: 1 errors occurred comparing output.
2020-04-20T02:00:45.0490319Z status: exit code: 1
2020-04-20T02:00:45.0490319Z status: exit code: 1
2020-04-20T02:00:45.0492660Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/inline_asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/inline_asm" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/inline_asm/auxiliary"
2020-04-20T02:00:45.0496578Z ------------------------------------------
2020-04-20T02:00:45.0496867Z 
2020-04-20T02:00:45.0497374Z ------------------------------------------
2020-04-20T02:00:45.0497691Z stderr:
2020-04-20T02:00:45.0497691Z stderr:
2020-04-20T02:00:45.0498187Z ------------------------------------------
2020-04-20T02:00:45.0498555Z warning: skipping const checks
2020-04-20T02:00:45.0499163Z   --> /checkout/src/test/ui/consts/miri_unleashed/inline_asm.rs:9:1
2020-04-20T02:00:45.0499518Z    |
2020-04-20T02:00:45.0499900Z LL | / static TEST_BAD: () = { //~ WARN: skipping const checks
2020-04-20T02:00:45.0500373Z LL | |     unsafe { llvm_asm!("xor %eax, %eax" ::: "eax"); }
2020-04-20T02:00:45.0501092Z LL | |     //~^ ERROR could not evaluate static initializer
2020-04-20T02:00:45.0501486Z LL | |     //~| NOTE in this expansion of llvm_asm!
2020-04-20T02:00:45.0501869Z LL | |     //~| NOTE inline assembly is not supported
2020-04-20T02:00:45.0502460Z    | |__^
2020-04-20T02:00:45.0502672Z 
2020-04-20T02:00:45.0503006Z error[E0080]: could not evaluate static initializer
2020-04-20T02:00:45.0503726Z   --> /checkout/src/test/ui/consts/miri_unleashed/inline_asm.rs:10:14
2020-04-20T02:00:45.0503726Z   --> /checkout/src/test/ui/consts/miri_unleashed/inline_asm.rs:10:14
2020-04-20T02:00:45.0504089Z    |
2020-04-20T02:00:45.0504457Z LL |     unsafe { llvm_asm!("xor %eax, %eax" ::: "eax"); }
2020-04-20T02:00:45.0504930Z    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ inline assembly is not supported
2020-04-20T02:00:45.0505975Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-04-20T02:00:45.0506364Z 
2020-04-20T02:00:45.0506704Z error: aborting due to previous error; 1 warning emitted
2020-04-20T02:00:45.0507002Z 
---
2020-04-20T02:00:45.0539854Z test result: FAILED. 9847 passed; 1 failed; 61 ignored; 0 measured; 0 filtered out
2020-04-20T02:00:45.0540242Z 
2020-04-20T02:00:45.0540510Z 
2020-04-20T02:00:45.0540711Z 
2020-04-20T02:00:45.0544396Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-20T02:00:45.0548792Z 
2020-04-20T02:00:45.0549021Z 
2020-04-20T02:00:45.0549752Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-20T02:00:45.0550416Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-20T02:00:45.0550416Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-20T02:00:45.0551201Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-20T02:00:45.0551668Z Build completed unsuccessfully in 1:00:18
2020-04-20T02:00:45.0576039Z == clock drift check ==
2020-04-20T02:00:45.0596701Z   local time: Mon Apr 20 02:00:45 UTC 2020
2020-04-20T02:00:45.3790736Z   network time: Mon, 20 Apr 2020 02:00:45 GMT
2020-04-20T02:00:45.8550209Z 
2020-04-20T02:00:45.8550209Z 
2020-04-20T02:00:45.8635043Z ##[error]Bash exited with code '1'.
2020-04-20T02:00:45.8651418Z ##[section]Finishing: Run build
2020-04-20T02:00:45.8708371Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71336/merge to s
2020-04-20T02:00:45.8713752Z Task         : Get sources
2020-04-20T02:00:45.8714071Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-20T02:00:45.8714362Z Version      : 1.0.0
2020-04-20T02:00:45.8714590Z Author       : Microsoft
2020-04-20T02:00:45.8714590Z Author       : Microsoft
2020-04-20T02:00:45.8714920Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-20T02:00:45.8715293Z ==============================================================================
2020-04-20T02:00:46.2186772Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-20T02:00:46.2232750Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71336/merge to s
2020-04-20T02:00:46.2327973Z Cleaning up task key
2020-04-20T02:00:46.2329216Z Start cleaning up orphan processes.
2020-04-20T02:00:46.2522716Z Terminate orphan process: pid (3897) (python)
2020-04-20T02:00:46.2715107Z ##[section]Finishing: Finalize Job
