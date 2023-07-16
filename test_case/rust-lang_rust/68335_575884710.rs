plain
2020-01-18T08:56:39.8964940Z ========================== Starting Command Output ===========================
2020-01-18T08:56:39.8966487Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/601fb5a8-91bc-418c-8c20-b784604ac488.sh
2020-01-18T08:56:39.8966526Z 
2020-01-18T08:56:39.8969440Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T08:56:39.8976170Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68335/merge to s
2020-01-18T08:56:39.8977831Z Task         : Get sources
2020-01-18T08:56:39.8977863Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T08:56:39.8977940Z Version      : 1.0.0
2020-01-18T08:56:39.8977971Z Author       : Microsoft
---
2020-01-18T08:56:40.9619910Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T08:56:40.9634938Z ##[command]git config gc.auto 0
2020-01-18T08:56:40.9637299Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T08:56:40.9641086Z ##[command]git config --get-all http.proxy
2020-01-18T08:56:40.9650820Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68335/merge:refs/remotes/pull/68335/merge
---
2020-01-18T09:52:42.5884675Z .................................................................................................... 1700/9535
2020-01-18T09:52:50.6247943Z .................................................................................................... 1800/9535
2020-01-18T09:53:00.4797295Z .............i...................................................................................... 1900/9535
2020-01-18T09:53:07.6285224Z .................................................................................................... 2000/9535
2020-01-18T09:53:23.7791698Z ...iiiii............................................................................................ 2100/9535
2020-01-18T09:53:32.5814396Z .................................................................................................... 2300/9535
2020-01-18T09:53:35.0918382Z .................................................................................................... 2400/9535
2020-01-18T09:53:40.7764566Z .................................................................................................... 2500/9535
2020-01-18T09:54:01.6181410Z .................................................................................................... 2600/9535
2020-01-18T09:54:01.6181410Z .................................................................................................... 2600/9535
2020-01-18T09:54:04.2516366Z .................................................................................................... 2700/9535
2020-01-18T09:54:15.0206048Z ............................................................................i....................... 2800/9535
2020-01-18T09:54:21.5563118Z .................................................................................................... 2900/9535
2020-01-18T09:54:29.8013134Z .................................................................................................... 3000/9535
2020-01-18T09:54:34.9097987Z .............i...................................................................................... 3100/9535
2020-01-18T09:54:43.8312738Z .................................................................................................... 3200/9535
2020-01-18T09:54:48.7240356Z .................................................................................................... 3300/9535
2020-01-18T09:54:56.6928225Z .ii................................................................................................. 3400/9535
2020-01-18T09:55:12.7539930Z ............................................................................................i....... 3600/9535
2020-01-18T09:55:19.9668565Z .......................................i............................................................ 3700/9535
2020-01-18T09:55:25.7179391Z .................................................................................................... 3800/9535
2020-01-18T09:55:31.3702231Z .................................................................................................... 3900/9535
---
2020-01-18T09:56:43.6966806Z ................................................i...............i................................... 4900/9535
2020-01-18T09:56:51.9630091Z .................................................................................................... 5000/9535
2020-01-18T09:56:59.6315616Z ...........................................................................................i........ 5100/9535
2020-01-18T09:57:04.7487963Z .................................................................................................... 5200/9535
2020-01-18T09:57:15.6274216Z ...............................................................ii.ii...........i.................... 5300/9535
2020-01-18T09:57:24.8704989Z i................................................................................................... 5500/9535
2020-01-18T09:57:35.2911846Z .................................................................................................... 5600/9535
2020-01-18T09:57:41.6128882Z .................................................i.................................................. 5700/9535
2020-01-18T09:57:48.7098749Z .................................................................................................... 5800/9535
2020-01-18T09:57:48.7098749Z .................................................................................................... 5800/9535
2020-01-18T09:57:58.9998536Z .................................................................................................... 5900/9535
2020-01-18T09:58:05.7660464Z ........................................ii...i..ii...........i...................................... 6000/9535
2020-01-18T09:58:28.4257840Z .................................................................................................... 6200/9535
2020-01-18T09:58:36.6175104Z .................................................................................................... 6300/9535
2020-01-18T09:58:36.6175104Z .................................................................................................... 6300/9535
2020-01-18T09:58:46.8088368Z ....................................................................i..ii........................... 6400/9535
2020-01-18T09:59:15.9141120Z .................................................................................................... 6600/9535
2020-01-18T09:59:18.3095617Z ............................................i....................................................... 6700/9535
2020-01-18T09:59:20.5330183Z .................................................................................................... 6800/9535
2020-01-18T09:59:23.0581990Z ............................................i....................................................... 6900/9535
---
2020-01-18T10:01:02.3448347Z .................................................................................................... 7500/9535
2020-01-18T10:01:07.0263418Z .................................................................................................... 7600/9535
2020-01-18T10:01:13.0697319Z .................................................................................................... 7700/9535
2020-01-18T10:01:19.6712865Z .................................................................................................... 7800/9535
2020-01-18T10:01:30.3700409Z ...............................................................................................iiiii 7900/9535
2020-01-18T10:01:37.1016444Z ii.................................................................................................. 8000/9535
2020-01-18T10:01:52.0922679Z .................................................................................................... 8200/9535
2020-01-18T10:02:04.0563886Z .................................................................................................... 8300/9535
2020-01-18T10:02:16.4604729Z .................................................................................................... 8400/9535
2020-01-18T10:02:22.4090824Z .................................................................................................... 8500/9535
---
2020-01-18T10:04:19.5889621Z 
2020-01-18T10:04:19.5890134Z ---- [ui] ui/recursion/issue-38591-non-regular-dropck-recursion.rs stdout ----
2020-01-18T10:04:19.5890195Z diff of stderr:
2020-01-18T10:04:19.5890253Z 
2020-01-18T10:04:19.5892762Z - error: reached the recursion limit while instantiating `std::ptr::real_drop_in_place::<S<fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(u32))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))>> - shim(Some(S<fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(u32))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))>))`
2020-01-18T10:04:19.5895928Z + error: reached the recursion limit while instantiating `std::intrinsics::drop_in_place::<S<fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(u32))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))>> - shim(Some(S<fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(u32))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))>))`
2020-01-18T10:04:19.5896379Z 3 error: aborting due to previous error
2020-01-18T10:04:19.5896422Z 4 
2020-01-18T10:04:19.5896470Z 
2020-01-18T10:04:19.5896496Z 
2020-01-18T10:04:19.5896496Z 
2020-01-18T10:04:19.5896541Z The actual stderr differed from the expected stderr.
2020-01-18T10:04:19.5896960Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-38591-non-regular-dropck-recursion/issue-38591-non-regular-dropck-recursion.stderr
2020-01-18T10:04:19.5897241Z To update references, rerun the tests and pass the `--bless` flag
2020-01-18T10:04:19.5897575Z To only update this specific test, also pass `--test-args recursion/issue-38591-non-regular-dropck-recursion.rs`
2020-01-18T10:04:19.5897682Z error: 1 errors occurred comparing output.
2020-01-18T10:04:19.5897730Z status: exit code: 1
2020-01-18T10:04:19.5897730Z status: exit code: 1
2020-01-18T10:04:19.5898660Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/recursion/issue-38591-non-regular-dropck-recursion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-38591-non-regular-dropck-recursion" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-38591-non-regular-dropck-recursion/auxiliary" "-A" "unused"
2020-01-18T10:04:19.5899027Z ------------------------------------------
2020-01-18T10:04:19.5899064Z 
2020-01-18T10:04:19.5899304Z ------------------------------------------
2020-01-18T10:04:19.5899353Z stderr:
2020-01-18T10:04:19.5899353Z stderr:
2020-01-18T10:04:19.5899607Z ------------------------------------------
2020-01-18T10:04:19.5902283Z error: reached the recursion limit while instantiating `std::intrinsics::drop_in_place::<S<fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(u32))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))>> - shim(Some(S<fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(u32))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))>))`
2020-01-18T10:04:19.5902714Z error: aborting due to previous error
2020-01-18T10:04:19.5902746Z 
2020-01-18T10:04:19.5902774Z 
2020-01-18T10:04:19.5903049Z ------------------------------------------
---
2020-01-18T10:04:19.5922400Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-18T10:04:19.5922464Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-18T10:04:19.5935637Z 
2020-01-18T10:04:19.5942273Z 
2020-01-18T10:04:19.5944497Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-18T10:04:19.5945062Z 
2020-01-18T10:04:19.5945115Z 
2020-01-18T10:04:19.5951497Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-18T10:04:19.5951591Z Build completed unsuccessfully in 1:01:48
2020-01-18T10:04:19.5951591Z Build completed unsuccessfully in 1:01:48
2020-01-18T10:04:19.6012846Z == clock drift check ==
2020-01-18T10:04:19.6029706Z   local time: Sat Jan 18 10:04:19 UTC 2020
2020-01-18T10:04:19.8931724Z   network time: Sat, 18 Jan 2020 10:04:19 GMT
2020-01-18T10:04:19.8935429Z == end clock drift check ==
2020-01-18T10:04:20.5007598Z 
2020-01-18T10:04:20.5101263Z ##[error]Bash exited with code '1'.
2020-01-18T10:04:20.5114087Z ##[section]Finishing: Run build
2020-01-18T10:04:20.5139117Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68335/merge to s
2020-01-18T10:04:20.5140935Z Task         : Get sources
2020-01-18T10:04:20.5140983Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T10:04:20.5141044Z Version      : 1.0.0
2020-01-18T10:04:20.5141086Z Author       : Microsoft
2020-01-18T10:04:20.5141086Z Author       : Microsoft
2020-01-18T10:04:20.5141133Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-18T10:04:20.5141183Z ==============================================================================
2020-01-18T10:04:20.9600975Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-18T10:04:20.9642774Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68335/merge to s
2020-01-18T10:04:20.9758929Z Cleaning up task key
2020-01-18T10:04:20.9761109Z Start cleaning up orphan processes.
2020-01-18T10:04:20.9875621Z Terminate orphan process: pid (4177) (python)
2020-01-18T10:04:21.0160715Z ##[section]Finishing: Finalize Job
