plain
2020-03-21T22:18:05.7808226Z ========================== Starting Command Output ===========================
2020-03-21T22:18:05.7813588Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2afb6cc7-3ee7-4bdf-98d8-cc13576faab9.sh
2020-03-21T22:18:05.7814128Z 
2020-03-21T22:18:05.7819647Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-21T22:18:05.7848945Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70248/merge to s
2020-03-21T22:18:05.7853564Z Task         : Get sources
2020-03-21T22:18:05.7853855Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-21T22:18:05.7854135Z Version      : 1.0.0
2020-03-21T22:18:05.7854326Z Author       : Microsoft
---
2020-03-21T22:18:06.7714665Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-21T22:18:06.7724103Z ##[command]git config gc.auto 0
2020-03-21T22:18:06.7734810Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-21T22:18:06.7738985Z ##[command]git config --get-all http.proxy
2020-03-21T22:18:06.7746857Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70248/merge:refs/remotes/pull/70248/merge
---
2020-03-21T23:18:12.7990935Z .................................................................................................... 1700/9815
2020-03-21T23:18:17.0098882Z .................................................................................................... 1800/9815
2020-03-21T23:18:28.3400908Z .............................................................................i...................... 1900/9815
2020-03-21T23:18:34.9078489Z .................................................................................................... 2000/9815
2020-03-21T23:18:42.6786944Z ...................................................................iiiii............................ 2100/9815
2020-03-21T23:19:02.1762768Z .................................................................................................... 2300/9815
2020-03-21T23:19:04.3923746Z .................................................................................................... 2400/9815
2020-03-21T23:19:07.1870077Z .................................................................................................... 2500/9815
2020-03-21T23:19:24.5634009Z .................................................................................................... 2600/9815
---
2020-03-21T23:22:13.5913884Z .........................................i...............i.......................................... 5000/9815
2020-03-21T23:22:22.6533562Z .................................................................................................... 5100/9815
2020-03-21T23:22:29.2878945Z .....................................................................................i.............. 5200/9815
2020-03-21T23:22:35.1828960Z .................................................................................................... 5300/9815
2020-03-21T23:22:45.5851006Z ...................................................................ii.ii........i...i............... 5400/9815
2020-03-21T23:22:53.7378989Z .......i............................................................................................ 5600/9815
2020-03-21T23:23:03.6478250Z ............i....................................................................................... 5700/9815
2020-03-21T23:23:09.4943571Z .............................ii...................................i................................. 5800/9815
2020-03-21T23:23:16.3235239Z .................................................................................................... 5900/9815
2020-03-21T23:23:16.3235239Z .................................................................................................... 5900/9815
2020-03-21T23:23:23.1872358Z .................................................................................................... 6000/9815
2020-03-21T23:23:32.2687094Z ............................................................ii...i..ii...........i.................. 6100/9815
2020-03-21T23:23:52.4434378Z .................................................................................................... 6300/9815
2020-03-21T23:23:59.5696299Z .................................................................................................... 6400/9815
2020-03-21T23:23:59.5696299Z .................................................................................................... 6400/9815
2020-03-21T23:24:06.9643158Z ..........................................................................................i..ii..... 6500/9815
2020-03-21T23:24:31.8931694Z .................................................................................................... 6700/9815
2020-03-21T23:24:42.6157786Z .........................................................................................i.......... 6800/9815
2020-03-21T23:24:44.8207558Z .................................................................................................... 6900/9815
2020-03-21T23:24:47.1053784Z .................................................................................................... 7000/9815
---
2020-03-21T23:26:36.2968102Z .................................................................................................... 7800/9815
2020-03-21T23:26:41.3988628Z .................................................................................................... 7900/9815
2020-03-21T23:26:47.6654696Z .............................................................................i...................... 8000/9815
2020-03-21T23:26:58.1884680Z .................................................................................................... 8100/9815
2020-03-21T23:27:03.7439943Z ..........................iiiiiiiiii.i.............................................................. 8200/9815
2020-03-21T23:27:18.4273282Z .................................................................................................... 8400/9815
2020-03-21T23:27:23.9245016Z .................................................................................................... 8500/9815
2020-03-21T23:27:39.8396671Z .................................................................................................... 8600/9815
2020-03-21T23:27:47.2996158Z .................................................................................................... 8700/9815
---
2020-03-21T23:30:12.2230335Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-03-21T23:30:12.2407664Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-21T23:30:12.4582781Z 
2020-03-21T23:30:12.4583448Z running 183 tests
2020-03-21T23:30:15.1757982Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/183
2020-03-21T23:30:17.8292540Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-03-21T23:30:17.8300801Z 
2020-03-21T23:30:17.8302817Z  finished in 5.588
2020-03-21T23:30:17.8329347Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-03-21T23:30:17.8509541Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-21T23:30:19.9137781Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-03-21T23:30:19.9331824Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-21T23:30:20.0911686Z 
2020-03-21T23:30:20.0912023Z running 9 tests
2020-03-21T23:30:20.0914319Z iiiiiiiii
2020-03-21T23:30:20.0915321Z 
2020-03-21T23:30:20.0915519Z  finished in 0.158
2020-03-21T23:30:20.0922234Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-03-21T23:30:20.1102081Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-21T23:30:40.2988029Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-03-21T23:30:40.3195827Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-21T23:30:40.5198170Z 
2020-03-21T23:30:40.5199083Z running 115 tests
2020-03-21T23:30:54.3859481Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-03-21T23:30:55.8987758Z ...iiii.....ii.
2020-03-21T23:30:55.8989015Z 
2020-03-21T23:30:55.8992223Z  finished in 15.579
2020-03-21T23:30:55.8997930Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-03-21T23:30:55.9000964Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-21T23:31:35.1908315Z ---- [ui] ui-fulldeps/mod_dir_path_canonicalized.rs stdout ----
2020-03-21T23:31:35.1908565Z 
2020-03-21T23:31:35.1909036Z error: test compilation failed although it shouldn't!
2020-03-21T23:31:35.1909333Z status: exit code: 1
2020-03-21T23:31:35.1911760Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/mod_dir_path_canonicalized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/auxiliary"
2020-03-21T23:31:35.1915021Z ------------------------------------------
2020-03-21T23:31:35.1915238Z 
2020-03-21T23:31:35.1915694Z ------------------------------------------
2020-03-21T23:31:35.1915949Z stderr:
2020-03-21T23:31:35.1915949Z stderr:
2020-03-21T23:31:35.1916362Z ------------------------------------------
2020-03-21T23:31:35.1916732Z error[E0061]: this function takes 3 arguments but 2 arguments were supplied
2020-03-21T23:31:35.1917443Z   --> /checkout/src/test/ui-fulldeps/mod_dir_path_canonicalized.rs:31:22
2020-03-21T23:31:35.1917746Z    |
2020-03-21T23:31:35.1918025Z LL |     let mut parser = new_parser_from_file(&parse_session, &path);
2020-03-21T23:31:35.1919082Z    |                      |
2020-03-21T23:31:35.1919357Z    |                      expected 3 arguments
2020-03-21T23:31:35.1919574Z 
2020-03-21T23:31:35.1919778Z error: aborting due to previous error
---
2020-03-21T23:31:35.1925042Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-21T23:31:35.1925534Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-21T23:31:35.1925798Z 
2020-03-21T23:31:35.1925907Z 
2020-03-21T23:31:35.1930930Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-21T23:31:35.1934052Z 
2020-03-21T23:31:35.1934163Z 
2020-03-21T23:31:35.1938209Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-21T23:31:35.1938785Z Build completed unsuccessfully in 1:09:06
2020-03-21T23:31:35.1938785Z Build completed unsuccessfully in 1:09:06
2020-03-21T23:31:35.1988648Z == clock drift check ==
2020-03-21T23:31:35.2005233Z   local time: Sat Mar 21 23:31:35 UTC 2020
2020-03-21T23:31:35.7552736Z   network time: Sat, 21 Mar 2020 23:31:35 GMT
2020-03-21T23:31:35.7556582Z == end clock drift check ==
2020-03-21T23:31:36.6322435Z 
2020-03-21T23:31:36.6399563Z ##[error]Bash exited with code '1'.
2020-03-21T23:31:36.6414074Z ##[section]Finishing: Run build
2020-03-21T23:31:36.6466178Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70248/merge to s
2020-03-21T23:31:36.6471662Z Task         : Get sources
2020-03-21T23:31:36.6472029Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-21T23:31:36.6472352Z Version      : 1.0.0
2020-03-21T23:31:36.6472597Z Author       : Microsoft
2020-03-21T23:31:36.6472597Z Author       : Microsoft
2020-03-21T23:31:36.6472965Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-21T23:31:36.6473377Z ==============================================================================
2020-03-21T23:31:36.9871264Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-21T23:31:36.9914100Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70248/merge to s
2020-03-21T23:31:37.0000208Z Cleaning up task key
2020-03-21T23:31:37.0001464Z Start cleaning up orphan processes.
2020-03-21T23:31:37.0218678Z Terminate orphan process: pid (3547) (python)
2020-03-21T23:31:37.0389383Z ##[section]Finishing: Finalize Job
