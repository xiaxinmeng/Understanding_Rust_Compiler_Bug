plain
2020-04-18T19:44:54.4665745Z ========================== Starting Command Output ===========================
2020-04-18T19:44:54.4671119Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fa96ba30-c101-4fa4-9dcc-2c4d29311c07.sh
2020-04-18T19:44:54.4671591Z 
2020-04-18T19:44:54.4676918Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-18T19:44:54.4696861Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71298/merge to s
2020-04-18T19:44:54.4700320Z Task         : Get sources
2020-04-18T19:44:54.4700619Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T19:44:54.4700931Z Version      : 1.0.0
2020-04-18T19:44:54.4701131Z Author       : Microsoft
---
2020-04-18T19:44:55.4697763Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-18T19:44:55.4703479Z ##[command]git config gc.auto 0
2020-04-18T19:44:55.4707184Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-18T19:44:55.4710604Z ##[command]git config --get-all http.proxy
2020-04-18T19:44:55.4716915Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71298/merge:refs/remotes/pull/71298/merge
---
2020-04-18T19:47:16.3832691Z  ---> 318032b5f0e2
2020-04-18T19:47:16.3833551Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-18T19:47:16.3834213Z  ---> Using cache
2020-04-18T19:47:16.3834608Z  ---> d44a858fd1ce
2020-04-18T19:47:16.3835627Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-18T19:47:16.3836737Z  ---> 58b910f50f5a
2020-04-18T19:47:16.3836961Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-18T19:47:16.3837357Z  ---> Using cache
2020-04-18T19:47:16.3837763Z  ---> ee7702aadba1
---
2020-04-18T19:47:16.4231341Z Looks like docker image is the same as before, not uploading
2020-04-18T19:47:24.3992411Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-18T19:47:24.4371123Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-18T19:47:24.4402188Z == clock drift check ==
2020-04-18T19:47:24.4426565Z   local time: Sat Apr 18 19:47:24 UTC 2020
2020-04-18T19:47:24.6527842Z   network time: Sat, 18 Apr 2020 19:47:24 GMT
2020-04-18T19:47:24.6544437Z Starting sccache server...
2020-04-18T19:47:24.7485365Z configure: processing command line
2020-04-18T19:47:24.7487933Z configure: 
2020-04-18T19:47:24.7489960Z configure: rust.dist-src        := False
---
2020-04-18T19:53:06.6273472Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T19:53:08.1864736Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T19:53:09.8693958Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T19:53:11.9791263Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T19:53:21.0688843Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T19:53:24.5258183Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T19:53:29.4452732Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T19:53:34.0956678Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T19:53:44.1570502Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T20:17:57.9909533Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T20:17:59.7707022Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T20:18:01.7121550Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T20:18:02.6643623Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T20:18:13.4260830Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T20:18:16.7214809Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T20:18:22.0342174Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T20:18:26.8197512Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T20:18:37.2146353Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T20:43:22.6352326Z .................................................................................................... 1700/9906
2020-04-18T20:43:26.7391157Z .................................................................................................... 1800/9906
2020-04-18T20:43:35.5703117Z .................................................................................................i.. 1900/9906
2020-04-18T20:43:43.5214562Z .................................................................................................... 2000/9906
2020-04-18T20:43:49.9249109Z .......................................................................................iiiii........ 2100/9906
2020-04-18T20:44:10.2660862Z .................................................................................................... 2300/9906
2020-04-18T20:44:12.4515689Z .................................................................................................... 2400/9906
2020-04-18T20:44:14.6952378Z .................................................................................................... 2500/9906
2020-04-18T20:44:20.7219609Z .................................................................................................... 2600/9906
---
2020-04-18T20:47:12.5254034Z .................................................................................................... 4900/9906
2020-04-18T20:47:17.6421148Z ...............................................................i...............i.................... 5000/9906
2020-04-18T20:47:25.4004985Z .................................................................................................... 5100/9906
2020-04-18T20:47:32.7914734Z .................................................................................................... 5200/9906
2020-04-18T20:47:38.0801554Z .........i.........................................................................................i 5300/9906
2020-04-18T20:47:48.3904526Z ...................................................................................................i 5400/9906
2020-04-18T20:47:53.4375599Z i.ii........i...i................................................................................... 5500/9906
2020-04-18T20:48:01.5423857Z ..............................................i..................................................... 5700/9906
2020-04-18T20:48:10.8613643Z ..............................................................................ii.................... 5800/9906
2020-04-18T20:48:18.0635363Z .................i.................................................................................. 5900/9906
2020-04-18T20:48:23.5968588Z .................................................................................................... 6000/9906
2020-04-18T20:48:23.5968588Z .................................................................................................... 6000/9906
2020-04-18T20:48:34.4262950Z .................................................................................................... 6100/9906
2020-04-18T20:48:45.1169507Z ...........ii...i..ii...........i................................................................... 6200/9906
2020-04-18T20:49:01.1271345Z .................................................................................................... 6400/9906
2020-04-18T20:49:07.6402966Z .................................................................................................... 6500/9906
2020-04-18T20:49:07.6402966Z .................................................................................................... 6500/9906
2020-04-18T20:49:23.9591142Z .........................................i..ii...................................................... 6600/9906
2020-04-18T20:49:46.8249037Z .................................................................................................... 6800/9906
2020-04-18T20:49:49.0412926Z ..........................................i......................................................... 6900/9906
2020-04-18T20:49:51.1407252Z .................................................................................................... 7000/9906
2020-04-18T20:49:53.6563185Z ..................................................................................i................. 7100/9906
---
2020-04-18T20:51:30.6398727Z .................................................................................................... 7800/9906
2020-04-18T20:51:35.1055508Z .................................................................................................... 7900/9906
2020-04-18T20:51:41.7027996Z .................................................................................................... 8000/9906
2020-04-18T20:51:47.5895685Z ................................................i................................................... 8100/9906
2020-04-18T20:51:57.9995652Z .................................................................................................iii 8200/9906
2020-04-18T20:52:03.7772705Z iii.iiiii.i......................................................................................... 8300/9906
2020-04-18T20:52:17.9833314Z .................................................................................................... 8500/9906
2020-04-18T20:52:26.4968941Z .................................................................................................... 8600/9906
2020-04-18T20:52:41.1102692Z .................................................................................................... 8700/9906
2020-04-18T20:52:48.2116308Z .................................................................................................... 8800/9906
---
2020-04-18T20:54:54.8804389Z ---- [run-fail] run-fail/divide-by-zero.rs stdout ----
2020-04-18T20:54:54.8804693Z 
2020-04-18T20:54:54.8804859Z error: compilation failed!
2020-04-18T20:54:54.8805051Z status: exit code: 1
2020-04-18T20:54:54.8807299Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/divide-by-zero.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/divide-by-zero/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/divide-by-zero/auxiliary"
2020-04-18T20:54:54.8808666Z ------------------------------------------
2020-04-18T20:54:54.8808828Z 
2020-04-18T20:54:54.8809202Z ------------------------------------------
2020-04-18T20:54:54.8809393Z stderr:
2020-04-18T20:54:54.8809393Z stderr:
2020-04-18T20:54:54.8809798Z ------------------------------------------
2020-04-18T20:54:54.8810043Z error: this operation will panic at runtime
2020-04-18T20:54:54.8810527Z  --> /checkout/src/test/run-fail/divide-by-zero.rs:5:14
2020-04-18T20:54:54.8810762Z   |
2020-04-18T20:54:54.8810912Z 5 |     let _z = 1 / y;
2020-04-18T20:54:54.8811338Z   |
2020-04-18T20:54:54.8811539Z   = note: `#[deny(unconditional_panic)]` on by default
2020-04-18T20:54:54.8811717Z 
2020-04-18T20:54:54.8811904Z error: aborting due to previous error
---
2020-04-18T20:54:54.8813183Z ---- [run-fail] run-fail/dst-raw-slice.rs stdout ----
2020-04-18T20:54:54.8813358Z 
2020-04-18T20:54:54.8813509Z error: compilation failed!
2020-04-18T20:54:54.8813717Z status: exit code: 1
2020-04-18T20:54:54.8815225Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/dst-raw-slice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/dst-raw-slice/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/dst-raw-slice/auxiliary"
2020-04-18T20:54:54.8816675Z ------------------------------------------
2020-04-18T20:54:54.8816843Z 
2020-04-18T20:54:54.8817235Z ------------------------------------------
2020-04-18T20:54:54.8817425Z stderr:
2020-04-18T20:54:54.8817425Z stderr:
2020-04-18T20:54:54.8817805Z ------------------------------------------
2020-04-18T20:54:54.8818066Z error: this operation will panic at runtime
2020-04-18T20:54:54.8818546Z  --> /checkout/src/test/run-fail/dst-raw-slice.rs:7:18
2020-04-18T20:54:54.8818755Z   |
2020-04-18T20:54:54.8818930Z 7 |         let _b = (*a)[3];
2020-04-18T20:54:54.8819486Z   |
2020-04-18T20:54:54.8819708Z   = note: `#[deny(unconditional_panic)]` on by default
2020-04-18T20:54:54.8819886Z 
2020-04-18T20:54:54.8820052Z error: aborting due to previous error
---
2020-04-18T20:54:54.8821416Z ---- [run-fail] run-fail/mod-zero.rs stdout ----
2020-04-18T20:54:54.8821612Z 
2020-04-18T20:54:54.8821765Z error: compilation failed!
2020-04-18T20:54:54.8821954Z status: exit code: 1
2020-04-18T20:54:54.8823472Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/mod-zero.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mod-zero/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/mod-zero/auxiliary"
2020-04-18T20:54:54.8824735Z ------------------------------------------
2020-04-18T20:54:54.8824903Z 
2020-04-18T20:54:54.8825273Z ------------------------------------------
2020-04-18T20:54:54.8825462Z stderr:
2020-04-18T20:54:54.8825462Z stderr:
2020-04-18T20:54:54.8825860Z ------------------------------------------
2020-04-18T20:54:54.8826108Z error: this operation will panic at runtime
2020-04-18T20:54:54.8826651Z  --> /checkout/src/test/run-fail/mod-zero.rs:5:14
2020-04-18T20:54:54.8826850Z   |
2020-04-18T20:54:54.8826998Z 5 |     let _z = 1 % y;
2020-04-18T20:54:54.8827515Z   |
2020-04-18T20:54:54.8827717Z   = note: `#[deny(unconditional_panic)]` on by default
2020-04-18T20:54:54.8827894Z 
2020-04-18T20:54:54.8828080Z error: aborting due to previous error
---
2020-04-18T20:54:54.8831097Z test result: FAILED. 136 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-18T20:54:54.8831334Z 
2020-04-18T20:54:54.8831422Z 
2020-04-18T20:54:54.8831510Z 
2020-04-18T20:54:54.8835021Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-18T20:54:54.8837480Z 
2020-04-18T20:54:54.8837573Z 
2020-04-18T20:54:54.8838117Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-18T20:54:54.8838498Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-18T20:54:54.8838498Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-18T20:54:54.8852100Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-18T20:54:54.8852849Z Build completed unsuccessfully in 1:05:46
2020-04-18T20:54:54.8910678Z == clock drift check ==
2020-04-18T20:54:54.8930558Z   local time: Sat Apr 18 20:54:54 UTC 2020
2020-04-18T20:54:55.0871559Z   network time: Sat, 18 Apr 2020 20:54:55 GMT
2020-04-18T20:54:55.5473796Z 
2020-04-18T20:54:55.5473796Z 
2020-04-18T20:54:55.5548671Z ##[error]Bash exited with code '1'.
2020-04-18T20:54:55.5591765Z ##[section]Finishing: Run build
2020-04-18T20:54:55.5642195Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71298/merge to s
2020-04-18T20:54:55.5647323Z Task         : Get sources
2020-04-18T20:54:55.5647655Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T20:54:55.5647957Z Version      : 1.0.0
2020-04-18T20:54:55.5648159Z Author       : Microsoft
2020-04-18T20:54:55.5648159Z Author       : Microsoft
2020-04-18T20:54:55.5648484Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-18T20:54:55.5648867Z ==============================================================================
2020-04-18T20:54:55.9163850Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-18T20:54:55.9205600Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71298/merge to s
2020-04-18T20:54:55.9298602Z Cleaning up task key
2020-04-18T20:54:55.9300036Z Start cleaning up orphan processes.
2020-04-18T20:54:55.9498709Z Terminate orphan process: pid (3723) (python)
2020-04-18T20:54:55.9670623Z ##[section]Finishing: Finalize Job
