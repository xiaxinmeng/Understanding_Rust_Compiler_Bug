plain
2020-04-15T21:16:32.4484256Z ========================== Starting Command Output ===========================
2020-04-15T21:16:32.4550740Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/92b00161-4851-4df6-8c60-fd17a38669f8.sh
2020-04-15T21:16:32.4553309Z 
2020-04-15T21:16:32.4562346Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-15T21:16:32.4583077Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71170/merge to s
2020-04-15T21:16:32.4585861Z Task         : Get sources
2020-04-15T21:16:32.4586306Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-15T21:16:32.4586695Z Version      : 1.0.0
2020-04-15T21:16:32.4586843Z Author       : Microsoft
---
2020-04-15T21:16:33.4408060Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-15T21:16:33.4412564Z ##[command]git config gc.auto 0
2020-04-15T21:16:33.4416121Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-15T21:16:33.4419677Z ##[command]git config --get-all http.proxy
2020-04-15T21:16:33.4426293Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71170/merge:refs/remotes/pull/71170/merge
---
2020-04-15T21:18:34.8312984Z  ---> f58a2bb1e753
2020-04-15T21:18:34.8316408Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-15T21:18:34.8321163Z  ---> Using cache
2020-04-15T21:18:34.8323001Z  ---> d079cc6b6db8
2020-04-15T21:18:34.8323765Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-15T21:18:34.8332694Z  ---> 4183ca46ee56
2020-04-15T21:18:34.8332879Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-15T21:18:34.8380474Z  ---> Using cache
2020-04-15T21:18:34.8380872Z  ---> 69e7f8a2a2fb
---
2020-04-15T21:18:34.8730971Z Looks like docker image is the same as before, not uploading
2020-04-15T21:18:42.4137230Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-15T21:18:42.4434621Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-15T21:18:42.4462550Z == clock drift check ==
2020-04-15T21:18:42.4485839Z   local time: Wed Apr 15 21:18:42 UTC 2020
2020-04-15T21:18:42.6422040Z   network time: Wed, 15 Apr 2020 21:18:42 GMT
2020-04-15T21:18:42.6436431Z Starting sccache server...
2020-04-15T21:18:42.7214299Z configure: processing command line
2020-04-15T21:18:42.7217820Z configure: 
2020-04-15T21:18:42.7218691Z configure: rust.dist-src        := False
---
2020-04-15T21:23:49.0078359Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-15T21:23:50.4616184Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-15T21:23:51.9763497Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-15T21:23:53.7591695Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-15T21:24:02.2814653Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-15T21:24:05.4758783Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-15T21:24:09.8722951Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-15T21:24:13.9218850Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-15T21:24:22.6513773Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-15T21:46:55.8159550Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-15T21:46:57.5416599Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-15T21:46:59.5577811Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-15T21:47:02.3368778Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-15T21:47:11.9459459Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-15T21:47:16.2540371Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-15T21:47:21.5172060Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-15T21:47:27.1160520Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-15T21:47:37.1726650Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-15T22:12:53.3465005Z .................................................................................................... 1700/9897
2020-04-15T22:12:57.5003123Z .................................................................................................... 1800/9897
2020-04-15T22:13:05.5350267Z .................................................................................................... 1900/9897
2020-04-15T22:13:13.1033335Z ......i............................................................................................. 2000/9897
2020-04-15T22:13:19.0450669Z ................................................................................................iiii 2100/9897
2020-04-15T22:13:33.5255630Z i................................................................................................... 2200/9897
2020-04-15T22:13:41.0832101Z .................................................................................................... 2400/9897
2020-04-15T22:13:43.0530637Z .................................................................................................... 2500/9897
2020-04-15T22:13:48.2135693Z .................................................................................................... 2600/9897
2020-04-15T22:14:06.8797370Z .................................................................................................... 2700/9897
---
2020-04-15T22:16:38.9876217Z .................................................................................................... 5100/9897
2020-04-15T22:16:46.4006191Z .................................................................................................... 5200/9897
2020-04-15T22:16:50.5081589Z ..................i................................................................................. 5300/9897
2020-04-15T22:16:59.8159524Z ........i........................................................................................... 5400/9897
2020-04-15T22:17:05.0407731Z ........ii.ii........i...i.......................................................................... 5500/9897
2020-04-15T22:17:12.0113681Z ......................................................i............................................. 5700/9897
2020-04-15T22:17:21.0953180Z ..........................................................................ii........................ 5800/9897
2020-04-15T22:17:27.2052855Z .............i...................................................................................... 5900/9897
2020-04-15T22:17:32.3949608Z .................................................................................................... 6000/9897
2020-04-15T22:17:32.3949608Z .................................................................................................... 6000/9897
2020-04-15T22:17:42.4840030Z .................................................................................................... 6100/9897
2020-04-15T22:17:52.8672271Z .......ii...i..ii...........i....................................................................... 6200/9897
2020-04-15T22:18:07.7563288Z .................................................................................................... 6400/9897
2020-04-15T22:18:13.8559510Z .................................................................................................... 6500/9897
2020-04-15T22:18:13.8559510Z .................................................................................................... 6500/9897
2020-04-15T22:18:25.5406988Z .....................................i..ii.......................................................... 6600/9897
2020-04-15T22:18:46.4246959Z .................................................................................................... 6800/9897
2020-04-15T22:18:48.4552690Z .....................................i.............................................................. 6900/9897
2020-04-15T22:18:50.4240492Z .................................................................................................... 7000/9897
2020-04-15T22:18:52.4504040Z ............................................................................i....................... 7100/9897
---
2020-04-15T22:20:27.7975373Z .................................................................................................... 7800/9897
2020-04-15T22:20:31.8394437Z .................................................................................................... 7900/9897
2020-04-15T22:20:38.0468814Z .................................................................................................... 8000/9897
2020-04-15T22:20:43.9510500Z ..........................................i......................................................... 8100/9897
2020-04-15T22:20:52.9369367Z ..........................................................................................iiiiii.iii 8200/9897
2020-04-15T22:20:58.7863871Z ii.i................................................................................................ 8300/9897
2020-04-15T22:21:11.0517965Z .................................................................................................... 8500/9897
2020-04-15T22:21:19.9127147Z .................................................................................................... 8600/9897
2020-04-15T22:21:32.5361111Z .................................................................................................... 8700/9897
2020-04-15T22:21:38.6238256Z .................................................................................................... 8800/9897
---
2020-04-15T22:23:24.0537814Z ---- [ui] ui/fn/dyn-fn-alignment.rs stdout ----
2020-04-15T22:23:24.0538293Z 
2020-04-15T22:23:24.0538567Z error: test run failed!
2020-04-15T22:23:24.0539024Z status: exit code: 101
2020-04-15T22:23:24.0540024Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/dyn-fn-alignment/a"
2020-04-15T22:23:24.0541012Z ------------------------------------------
2020-04-15T22:23:24.0541319Z 
2020-04-15T22:23:24.0541952Z ------------------------------------------
2020-04-15T22:23:24.0542323Z stderr:
2020-04-15T22:23:24.0542323Z stderr:
2020-04-15T22:23:24.0542848Z ------------------------------------------
2020-04-15T22:23:24.0543544Z thread 'main' panicked at 'assertion failed: `(left == right)`
2020-04-15T22:23:24.0543969Z   left: `48`,
2020-04-15T22:23:24.0544672Z  right: `0`: addr: 0x7ffde875e030', /checkout/src/test/ui/fn/dyn-fn-alignment.rs:23:5
2020-04-15T22:23:24.0545727Z 
2020-04-15T22:23:24.0546233Z ------------------------------------------
2020-04-15T22:23:24.0546533Z 
2020-04-15T22:23:24.0546894Z 
---
2020-04-15T22:23:24.0557000Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-15T22:23:24.0557719Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-15T22:23:24.0567145Z 
2020-04-15T22:23:24.0567649Z 
2020-04-15T22:23:24.0571409Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-15T22:23:24.0574454Z 
2020-04-15T22:23:24.0574807Z 
2020-04-15T22:23:24.0578923Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-15T22:23:24.0579275Z Build completed unsuccessfully in 1:03:08
2020-04-15T22:23:24.0579275Z Build completed unsuccessfully in 1:03:08
2020-04-15T22:23:24.0626854Z == clock drift check ==
2020-04-15T22:23:24.0645922Z   local time: Wed Apr 15 22:23:24 UTC 2020
2020-04-15T22:23:25.1125313Z   network time: Wed, 15 Apr 2020 22:23:24 GMT
2020-04-15T22:23:25.1172750Z 
2020-04-15T22:23:25.1172750Z 
2020-04-15T22:23:25.1207482Z ##[error]Bash exited with code '1'.
2020-04-15T22:23:25.1219512Z ##[section]Finishing: Run build
2020-04-15T22:23:25.1272073Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71170/merge to s
2020-04-15T22:23:25.1277775Z Task         : Get sources
2020-04-15T22:23:25.1278290Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-15T22:23:25.1278572Z Version      : 1.0.0
2020-04-15T22:23:25.1278928Z Author       : Microsoft
2020-04-15T22:23:25.1278928Z Author       : Microsoft
2020-04-15T22:23:25.1279249Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-15T22:23:25.1279595Z ==============================================================================
2020-04-15T22:23:25.4511513Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-15T22:23:25.4516011Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71170/merge to s
2020-04-15T22:23:25.4619834Z Cleaning up task key
2020-04-15T22:23:25.4620975Z Start cleaning up orphan processes.
2020-04-15T22:23:25.4800272Z Terminate orphan process: pid (3286) (python)
2020-04-15T22:23:25.5022785Z ##[section]Finishing: Finalize Job
