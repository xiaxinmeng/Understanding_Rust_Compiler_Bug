plain
2020-01-24T19:58:06.6507547Z ========================== Starting Command Output ===========================
2020-01-24T19:58:06.6509819Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4215d50a-bf07-4476-bd04-5f5b0f54e15b.sh
2020-01-24T19:58:06.6509978Z 
2020-01-24T19:58:06.6513236Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-24T19:58:06.6518996Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68498/merge to s
2020-01-24T19:58:06.6520655Z Task         : Get sources
2020-01-24T19:58:06.6520692Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-24T19:58:06.6520726Z Version      : 1.0.0
2020-01-24T19:58:06.6520802Z Author       : Microsoft
---
2020-01-24T19:58:07.4337238Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-24T19:58:07.4403643Z ##[command]git config gc.auto 0
2020-01-24T19:58:07.4474597Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-24T19:58:07.4536651Z ##[command]git config --get-all http.proxy
2020-01-24T19:58:07.4671711Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68498/merge:refs/remotes/pull/68498/merge
---
2020-01-24T20:48:38.0685207Z .................................................................................................... 1700/9551
2020-01-24T20:48:43.8683117Z .................................................................................................... 1800/9551
2020-01-24T20:48:54.4808601Z .....................i.............................................................................. 1900/9551
2020-01-24T20:49:01.4201596Z .................................................................................................... 2000/9551
2020-01-24T20:49:16.3053100Z ...........iiiii.................................................................................... 2100/9551
2020-01-24T20:49:26.0257922Z .................................................................................................... 2300/9551
2020-01-24T20:49:28.5408329Z .................................................................................................... 2400/9551
2020-01-24T20:49:34.2515971Z .................................................................................................... 2500/9551
2020-01-24T20:49:54.3616970Z .................................................................................................... 2600/9551
---
2020-01-24T20:52:28.9001756Z .......................................................i...............i............................ 4900/9551
2020-01-24T20:52:36.6736039Z .................................................................................................... 5000/9551
2020-01-24T20:52:43.6904325Z ..................................................................................................i. 5100/9551
2020-01-24T20:52:48.4635309Z .................................................................................................... 5200/9551
2020-01-24T20:52:58.2483717Z ......................................................................ii.ii........i...i............ 5300/9551
2020-01-24T20:53:06.7167390Z ........i........................................................................................... 5500/9551
2020-01-24T20:53:15.9304984Z .................................................................................................... 5600/9551
2020-01-24T20:53:21.9298783Z .........................................................i.......................................... 5700/9551
2020-01-24T20:53:28.4521829Z .................................................................................................... 5800/9551
2020-01-24T20:53:28.4521829Z .................................................................................................... 5800/9551
2020-01-24T20:53:37.3092934Z .................................................................................................... 5900/9551
2020-01-24T20:53:43.8427965Z ................................................ii...i..ii...........i.............................. 6000/9551
2020-01-24T20:54:04.6409895Z .................................................................................................... 6200/9551
2020-01-24T20:54:09.1313766Z .................................................................................................... 6300/9551
2020-01-24T20:54:09.1313766Z .................................................................................................... 6300/9551
2020-01-24T20:54:13.7272727Z ............................................................................i..ii................... 6400/9551
2020-01-24T20:54:38.3977255Z .................................................................................................... 6600/9551
2020-01-24T20:54:42.9754258Z ....................................................i............................................... 6700/9551
2020-01-24T20:54:45.1572023Z .................................................................................................... 6800/9551
2020-01-24T20:54:47.2575988Z ...................................................i................................................ 6900/9551
---
2020-01-24T20:56:20.0823442Z .................................................................................................... 7600/9551
2020-01-24T20:56:25.5369835Z .................................................................................................... 7700/9551
2020-01-24T20:56:31.6267094Z .................................................................................................... 7800/9551
2020-01-24T20:56:41.4366048Z .................................................................................................... 7900/9551
2020-01-24T20:56:47.1220307Z .......iiiiiii...................................................................................... 8000/9551
2020-01-24T20:57:01.0210732Z .................................................................................................... 8200/9551
2020-01-24T20:57:11.9890540Z .................................................................................................... 8300/9551
2020-01-24T20:57:24.4867258Z .................................................................................................... 8400/9551
2020-01-24T20:57:30.6258861Z .................................................................................................... 8500/9551
---
2020-01-24T20:59:19.8298434Z 1 error[E0631]: type mismatch in closure arguments
2020-01-24T20:59:19.8298801Z -   --> $DIR/issue-57611-trait-alias.rs:16:5
2020-01-24T20:59:19.8299180Z +   --> $DIR/issue-57611-trait-alias.rs:17:5
2020-01-24T20:59:19.8299337Z 3    |
2020-01-24T20:59:19.8299700Z 4 LL |     type Bar = impl Baz<Self, Self>;
2020-01-24T20:59:19.8300103Z 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected signature of `for<'r> fn(&'r X) -> _`
2020-01-24T20:59:19.8300407Z 9    |
2020-01-24T20:59:19.8300569Z 10    = note: the return type of a function must have a statically known size
2020-01-24T20:59:19.8300714Z 11 
2020-01-24T20:59:19.8300714Z 11 
2020-01-24T20:59:19.8301210Z - error[E0271]: type mismatch resolving `for<'r> <[closure@$DIR/issue-57611-trait-alias.rs:20:9: 20:14] as std::ops::FnOnce<(&'r X,)>>::Output == &'r X`
2020-01-24T20:59:19.8301581Z -   --> $DIR/issue-57611-trait-alias.rs:16:5
2020-01-24T20:59:19.8302064Z + error[E0271]: type mismatch resolving `for<'r> <[closure@$DIR/issue-57611-trait-alias.rs:21:9: 21:14] as std::ops::FnOnce<(&'r X,)>>::Output == &'r X`
2020-01-24T20:59:19.8302427Z +   --> $DIR/issue-57611-trait-alias.rs:17:5
2020-01-24T20:59:19.8302584Z 14    |
2020-01-24T20:59:19.8302739Z 15 LL |     type Bar = impl Baz<Self, Self>;
2020-01-24T20:59:19.8302902Z 16    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected bound lifetime parameter, found concrete lifetime
2020-01-24T20:59:19.8303176Z 
2020-01-24T20:59:19.8303319Z The actual stderr differed from the expected stderr.
2020-01-24T20:59:19.8303778Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias/issue-57611-trait-alias.stderr
2020-01-24T20:59:19.8303778Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias/issue-57611-trait-alias.stderr
2020-01-24T20:59:19.8304174Z To update references, rerun the tests and pass the `--bless` flag
2020-01-24T20:59:19.8304591Z To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-57611-trait-alias.rs`
2020-01-24T20:59:19.8304891Z error: 1 errors occurred comparing output.
2020-01-24T20:59:19.8305032Z status: exit code: 1
2020-01-24T20:59:19.8305032Z status: exit code: 1
2020-01-24T20:59:19.8306096Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-57611-trait-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias/auxiliary" "-A" "unused"
2020-01-24T20:59:19.8306668Z ------------------------------------------
2020-01-24T20:59:19.8306809Z 
2020-01-24T20:59:19.8307140Z ------------------------------------------
2020-01-24T20:59:19.8307309Z stderr:
2020-01-24T20:59:19.8307309Z stderr:
2020-01-24T20:59:19.8307638Z ------------------------------------------
2020-01-24T20:59:19.8307797Z error[E0631]: type mismatch in closure arguments
2020-01-24T20:59:19.8308188Z   --> /checkout/src/test/ui/type-alias-impl-trait/issue-57611-trait-alias.rs:17:5
2020-01-24T20:59:19.8308359Z    |
2020-01-24T20:59:19.8308523Z LL |     type Bar = impl Baz<Self, Self>; //~ ERROR type mismatch in closure arguments
2020-01-24T20:59:19.8308903Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected signature of `for<'r> fn(&'r X) -> _`
2020-01-24T20:59:19.8309325Z LL |         |x| x
2020-01-24T20:59:19.8309675Z    |         ----- found signature of `fn(_) -> _`
2020-01-24T20:59:19.8309831Z    |
2020-01-24T20:59:19.8309992Z    = note: the return type of a function must have a statically known size
2020-01-24T20:59:19.8309992Z    = note: the return type of a function must have a statically known size
2020-01-24T20:59:19.8310118Z 
2020-01-24T20:59:19.8310624Z error[E0271]: type mismatch resolving `for<'r> <[closure@/checkout/src/test/ui/type-alias-impl-trait/issue-57611-trait-alias.rs:21:9: 21:14] as std::ops::FnOnce<(&'r X,)>>::Output == &'r X`
2020-01-24T20:59:19.8311204Z    |
2020-01-24T20:59:19.8311204Z    |
2020-01-24T20:59:19.8311364Z LL |     type Bar = impl Baz<Self, Self>; //~ ERROR type mismatch in closure arguments
2020-01-24T20:59:19.8311601Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected bound lifetime parameter, found concrete lifetime
2020-01-24T20:59:19.8311914Z    = note: the return type of a function must have a statically known size
2020-01-24T20:59:19.8312040Z 
2020-01-24T20:59:19.8312180Z error: aborting due to 2 previous errors
2020-01-24T20:59:19.8312317Z 
---
2020-01-24T20:59:19.8336056Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:349:22
2020-01-24T20:59:19.8336318Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-24T20:59:19.8348527Z 
2020-01-24T20:59:19.8348740Z 
2020-01-24T20:59:19.8351687Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-24T20:59:19.8351958Z 
2020-01-24T20:59:19.8351989Z 
2020-01-24T20:59:19.8361778Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-24T20:59:19.8362059Z Build completed unsuccessfully in 0:56:04
2020-01-24T20:59:19.8362059Z Build completed unsuccessfully in 0:56:04
2020-01-24T20:59:19.8417930Z == clock drift check ==
2020-01-24T20:59:20.5244346Z   local time: Fri Jan 24 20:59:19 UTC 2020
2020-01-24T20:59:20.5245513Z   network time: Fri, 24 Jan 2020 20:59:20 GMT
2020-01-24T20:59:20.5245865Z == end clock drift check ==
2020-01-24T20:59:20.5246016Z 
2020-01-24T20:59:20.5300751Z ##[error]Bash exited with code '1'.
2020-01-24T20:59:20.5312064Z ##[section]Finishing: Run build
2020-01-24T20:59:20.5331744Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68498/merge to s
2020-01-24T20:59:20.5342720Z Task         : Get sources
2020-01-24T20:59:20.5342771Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-24T20:59:20.5342836Z Version      : 1.0.0
2020-01-24T20:59:20.5342881Z Author       : Microsoft
2020-01-24T20:59:20.5342881Z Author       : Microsoft
2020-01-24T20:59:20.5342931Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-24T20:59:20.5343129Z ==============================================================================
2020-01-24T20:59:20.9494931Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-24T20:59:20.9543978Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68498/merge to s
2020-01-24T20:59:20.9648401Z Cleaning up task key
2020-01-24T20:59:20.9649188Z Start cleaning up orphan processes.
2020-01-24T20:59:20.9769098Z Terminate orphan process: pid (3823) (python)
2020-01-24T20:59:21.0030829Z ##[section]Finishing: Finalize Job
