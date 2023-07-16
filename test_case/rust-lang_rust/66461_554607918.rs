plain
2019-11-16T04:47:55.4545584Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-16T04:47:55.4745288Z ##[command]git config gc.auto 0
2019-11-16T04:47:55.4825827Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-16T04:47:55.4889966Z ##[command]git config --get-all http.proxy
2019-11-16T04:47:55.5037160Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66461/merge:refs/remotes/pull/66461/merge
---
2019-11-16T05:45:32.6480193Z .................................................................................................... 1500/9244
2019-11-16T05:45:38.9862087Z .................................................................................................... 1600/9244
2019-11-16T05:45:48.5781758Z .................................................................................................... 1700/9244
2019-11-16T05:45:57.6691519Z .....i.............................................................................................. 1800/9244
2019-11-16T05:46:04.6470973Z .........................................................................................iiiii...... 1900/9244
2019-11-16T05:46:26.4760567Z .................................................................................................... 2100/9244
2019-11-16T05:46:28.9464562Z .................................................................................................... 2200/9244
2019-11-16T05:46:31.6890721Z .................................................................................................... 2300/9244
2019-11-16T05:46:38.3559258Z .................................................................................................... 2400/9244
---
2019-11-16T05:49:38.1709288Z ..........F.............................................................................i........... 4700/9244
2019-11-16T05:49:45.0850615Z ....i............................................................................................... 4800/9244
2019-11-16T05:49:54.4177600Z .................................................................................................... 4900/9244
2019-11-16T05:49:59.9174228Z .................................................................................................... 5000/9244
2019-11-16T05:50:10.6858480Z ............................................................................................ii.ii... 5100/9244
2019-11-16T05:50:19.5256652Z ............................i....................................................................... 5300/9244
2019-11-16T05:50:28.0580908Z .................................................................................................... 5400/9244
2019-11-16T05:50:37.0893986Z ..........................................................................i......................... 5500/9244
2019-11-16T05:50:45.2826776Z .................................................................................................... 5600/9244
2019-11-16T05:50:45.2826776Z .................................................................................................... 5600/9244
2019-11-16T05:50:52.3903971Z .................................................................................................... 5700/9244
2019-11-16T05:51:03.2797795Z ............................................................ii...i..ii...........i.................. 5800/9244
2019-11-16T05:51:26.1972309Z .................................................................................................... 6000/9244
2019-11-16T05:51:34.7291719Z .................................................................................................... 6100/9244
2019-11-16T05:51:34.7291719Z .................................................................................................... 6100/9244
2019-11-16T05:51:40.3305538Z ...............................................................................i..ii................ 6200/9244
2019-11-16T05:52:09.5767855Z ..........................F......................................................................... 6400/9244
2019-11-16T05:52:12.8757427Z ...............................................i.................................................... 6500/9244
2019-11-16T05:52:15.2176054Z .................................................................................................... 6600/9244
2019-11-16T05:52:17.7433250Z ..................................i................................................................. 6700/9244
---
2019-11-16T05:57:12.1820424Z 
2019-11-16T05:57:12.1820535Z 
2019-11-16T05:57:12.1820698Z The actual stderr differed from the expected stderr.
2019-11-16T05:57:12.1821192Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45730/issue-45730.stderr
2019-11-16T05:57:12.1821600Z To update references, rerun the tests and pass the `--bless` flag
2019-11-16T05:57:12.1822061Z To only update this specific test, also pass `--test-args issues/issue-45730.rs`
2019-11-16T05:57:12.1822532Z error: 1 errors occurred comparing output.
2019-11-16T05:57:12.1822690Z status: exit code: 1
2019-11-16T05:57:12.1822690Z status: exit code: 1
2019-11-16T05:57:12.1823521Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-45730.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45730" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45730/auxiliary" "-A" "unused"
2019-11-16T05:57:12.1824113Z ------------------------------------------
2019-11-16T05:57:12.1824475Z 
2019-11-16T05:57:12.1824821Z ------------------------------------------
2019-11-16T05:57:12.1825402Z stderr:
2019-11-16T05:57:12.1825402Z stderr:
2019-11-16T05:57:12.1825917Z ------------------------------------------
2019-11-16T05:57:12.1826091Z error[E0641]: cannot cast to a pointer of an unknown kind
2019-11-16T05:57:12.1826798Z   --> /checkout/src/test/ui/issues/issue-45730.rs:3:23
2019-11-16T05:57:12.1827164Z    |
2019-11-16T05:57:12.1827303Z LL |     let x: *const _ = 0 as _; //~ ERROR cannot cast
2019-11-16T05:57:12.1830342Z    |                            |
2019-11-16T05:57:12.1830498Z    |                            help: consider giving more type information
2019-11-16T05:57:12.1831919Z    |
2019-11-16T05:57:12.1832093Z    = note: The type information given here is insufficient to check whether the pointer cast is valid
2019-11-16T05:57:12.1832093Z    = note: The type information given here is insufficient to check whether the pointer cast is valid
2019-11-16T05:57:12.1832222Z 
2019-11-16T05:57:12.1832387Z error[E0641]: cannot cast to a pointer of an unknown kind
2019-11-16T05:57:12.1833081Z   --> /checkout/src/test/ui/issues/issue-45730.rs:5:23
2019-11-16T05:57:12.1833274Z    |
2019-11-16T05:57:12.1833446Z LL |     let x: *const _ = 0 as *const _; //~ ERROR cannot cast
2019-11-16T05:57:12.1834001Z    |                            |
2019-11-16T05:57:12.1834176Z    |                            help: consider giving more type information
2019-11-16T05:57:12.1834320Z    |
2019-11-16T05:57:12.1834623Z    = note: The type information given here is insufficient to check whether the pointer cast is valid
2019-11-16T05:57:12.1834623Z    = note: The type information given here is insufficient to check whether the pointer cast is valid
2019-11-16T05:57:12.1834807Z 
2019-11-16T05:57:12.1834955Z error[E0641]: cannot cast to a pointer of an unknown kind
2019-11-16T05:57:12.1835946Z   --> /checkout/src/test/ui/issues/issue-45730.rs:8:13
2019-11-16T05:57:12.1836133Z    |
2019-11-16T05:57:12.1836278Z LL |     let x = 0 as *const i32 as *const _ as *mut _; //~ ERROR cannot cast
2019-11-16T05:57:12.1836842Z    |                                            |
2019-11-16T05:57:12.1836990Z    |                                            help: consider giving more type information
2019-11-16T05:57:12.1837147Z    |
2019-11-16T05:57:12.1837314Z    = note: The type information given here is insufficient to check whether the pointer cast is valid
---
2019-11-16T05:57:12.1841188Z + 
2019-11-16T05:57:12.1841214Z 
2019-11-16T05:57:12.1841239Z 
2019-11-16T05:57:12.1841283Z The actual stderr differed from the expected stderr.
2019-11-16T05:57:12.1841627Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/order-dependent-cast-inference/order-dependent-cast-inference.stderr
2019-11-16T05:57:12.1841886Z To update references, rerun the tests and pass the `--bless` flag
2019-11-16T05:57:12.1842411Z To only update this specific test, also pass `--test-args order-dependent-cast-inference.rs`
2019-11-16T05:57:12.1842511Z error: 1 errors occurred comparing output.
2019-11-16T05:57:12.1842553Z status: exit code: 1
2019-11-16T05:57:12.1842553Z status: exit code: 1
2019-11-16T05:57:12.1843322Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/order-dependent-cast-inference.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/order-dependent-cast-inference" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/order-dependent-cast-inference/auxiliary" "-A" "unused"
2019-11-16T05:57:12.1843639Z ------------------------------------------
2019-11-16T05:57:12.1843671Z 
2019-11-16T05:57:12.1843891Z ------------------------------------------
2019-11-16T05:57:12.1843934Z stderr:
2019-11-16T05:57:12.1843934Z stderr:
2019-11-16T05:57:12.1844161Z ------------------------------------------
2019-11-16T05:57:12.1844348Z error[E0641]: cannot cast to a pointer of an unknown kind
2019-11-16T05:57:12.1844621Z   --> /checkout/src/test/ui/order-dependent-cast-inference.rs:5:17
2019-11-16T05:57:12.1844686Z    |
2019-11-16T05:57:12.1844727Z LL |     let mut y = 0 as *const _;
2019-11-16T05:57:12.1844992Z    |                      |
2019-11-16T05:57:12.1845039Z    |                      help: consider giving more type information
2019-11-16T05:57:12.1845081Z    |
2019-11-16T05:57:12.1845147Z    = note: The type information given here is insufficient to check whether the pointer cast is valid
---
2019-11-16T05:57:12.1874463Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-16T05:57:12.1874542Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-16T05:57:12.9662436Z 
2019-11-16T05:57:12.9662831Z 
2019-11-16T05:57:12.9664691Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-16T05:57:12.9665141Z 
2019-11-16T05:57:12.9665186Z 
2019-11-16T05:57:12.9665576Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-16T05:57:12.9665692Z Build completed unsuccessfully in 1:03:10
2019-11-16T05:57:12.9665692Z Build completed unsuccessfully in 1:03:10
2019-11-16T05:57:12.9665759Z == clock drift check ==
2019-11-16T05:57:12.9665823Z   local time: Sat Nov 16 05:57:12 UTC 2019
2019-11-16T05:57:12.9665904Z   network time: Sat, 16 Nov 2019 05:57:12 GMT
2019-11-16T05:57:12.9665964Z == end clock drift check ==
2019-11-16T05:57:13.3640946Z 
2019-11-16T05:57:13.3734346Z ##[error]Bash exited with code '1'.
2019-11-16T05:57:13.3780644Z ##[section]Starting: Checkout
2019-11-16T05:57:13.3782248Z ==============================================================================
2019-11-16T05:57:13.3782297Z Task         : Get sources
2019-11-16T05:57:13.3782338Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
