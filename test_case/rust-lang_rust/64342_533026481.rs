plain
2019-09-19T07:14:29.7501890Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-19T07:14:29.7695287Z ##[command]git config gc.auto 0
2019-09-19T07:14:29.7767701Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-19T07:14:29.7821367Z ##[command]git config --get-all http.proxy
2019-09-19T07:14:29.7963474Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64342/merge:refs/remotes/pull/64342/merge
---
2019-09-19T08:18:26.6257551Z .................................................................................................... 1500/9024
2019-09-19T08:18:32.7731332Z .................................................................................................... 1600/9024
2019-09-19T08:18:45.6651595Z .................................................................i...............i.................. 1700/9024
2019-09-19T08:18:52.9752508Z .................................................................................................... 1800/9024
2019-09-19T08:19:08.4458573Z ........................................................iiiii....................................... 1900/9024
2019-09-19T08:19:20.5023041Z .................................................................................................... 2100/9024
2019-09-19T08:19:23.0924853Z .................................................................................................... 2200/9024
2019-09-19T08:19:26.4406036Z .................................................................................................... 2300/9024
2019-09-19T08:19:35.0261299Z .................................................................................................... 2400/9024
---
2019-09-19T08:22:38.8340462Z .............................................i..............i....................................... 4700/9024
2019-09-19T08:22:49.7208358Z .................................................................................................... 4800/9024
2019-09-19T08:22:56.7782035Z .................................................................................................... 4900/9024
2019-09-19T08:23:06.5475981Z .................................................................................................... 5000/9024
2019-09-19T08:23:14.5457363Z ............................ii.ii................................................................... 5100/9024
2019-09-19T08:23:24.7675134Z .....................................................FF............................................. 5300/9024
2019-09-19T08:23:35.3212432Z ............................................................................................i....... 5400/9024
2019-09-19T08:23:43.7837051Z .................................................................................................... 5500/9024
2019-09-19T08:23:48.7355686Z .................................................................................................... 5600/9024
2019-09-19T08:23:48.7355686Z .................................................................................................... 5600/9024
2019-09-19T08:23:59.6990369Z ........................................................................................ii..i..ii... 5700/9024
2019-09-19T08:24:26.0434626Z .................................................................................................... 5900/9024
2019-09-19T08:24:36.4876156Z .................................................................................................... 6000/9024
2019-09-19T08:24:36.4876156Z .................................................................................................... 6000/9024
2019-09-19T08:24:43.7634077Z .........................................................................................i..ii...... 6100/9024
2019-09-19T08:25:12.8463515Z .................................................................................................... 6300/9024
2019-09-19T08:25:17.2496551Z ................................................i................................................... 6400/9024
2019-09-19T08:25:19.5620763Z .................................................................................................... 6500/9024
2019-09-19T08:25:22.1155429Z ....................i............................................................................... 6600/9024
---
2019-09-19T08:29:34.8938779Z 
2019-09-19T08:29:34.8939229Z ---- [ui] ui/lint/must_use-array.rs stdout ----
2019-09-19T08:29:34.8939298Z diff of stderr:
2019-09-19T08:29:34.8939581Z 
2019-09-19T08:29:34.8939825Z - error: unused array of `S` that must be used
2019-09-19T08:29:34.8939869Z + error: unused arrays of `S` that must be used
2019-09-19T08:29:34.8940119Z 3    |
2019-09-19T08:29:34.8940315Z 4 LL |     singleton();
2019-09-19T08:29:34.8940338Z 
2019-09-19T08:29:34.8940338Z 
2019-09-19T08:29:34.8940390Z 10 LL | #![deny(unused_must_use)]
2019-09-19T08:29:34.8940471Z 12 
2019-09-19T08:29:34.8940674Z - error: unused array of `S` that must be used
2019-09-19T08:29:34.8940674Z - error: unused array of `S` that must be used
2019-09-19T08:29:34.8940715Z + error: unused arrays of `S` that must be used
2019-09-19T08:29:34.8940927Z 15    |
2019-09-19T08:29:34.8940978Z 16 LL |     many();
2019-09-19T08:29:34.8941000Z 
2019-09-19T08:29:34.8941032Z 17    |     ^^^^^^^
2019-09-19T08:29:34.8941032Z 17    |     ^^^^^^^
2019-09-19T08:29:34.8941081Z 18 
2019-09-19T08:29:34.8944374Z - error: unused array of `S` in tuple element 0 that must be used
2019-09-19T08:29:34.8944471Z + error: unused arrays of `S` in tuple element 0 that must be used
2019-09-19T08:29:34.8944833Z 21    |
2019-09-19T08:29:34.8944833Z 21    |
2019-09-19T08:29:34.8944877Z 22 LL |     ([S], 0, ());
2019-09-19T08:29:34.8945130Z 23    |      ^^^
2019-09-19T08:29:34.8945168Z 24 
2019-09-19T08:29:34.8945551Z - error: unused array of implementers of `T` that must be used
2019-09-19T08:29:34.8945551Z - error: unused array of implementers of `T` that must be used
2019-09-19T08:29:34.8945598Z + error: unused arrays of implementers of `T` that must be used
2019-09-19T08:29:34.8945838Z 27    |
2019-09-19T08:29:34.8945873Z 28 LL |     array_of_impl_trait();
2019-09-19T08:29:34.8945917Z 
2019-09-19T08:29:34.8945952Z 29    |     ^^^^^^^^^^^^^^^^^^^^^^
2019-09-19T08:29:34.8945952Z 29    |     ^^^^^^^^^^^^^^^^^^^^^^
2019-09-19T08:29:34.8945986Z 30 
2019-09-19T08:29:34.8946207Z - error: unused array of boxed `T` trait objects in tuple element 1 that must be used
2019-09-19T08:29:34.8946272Z + error: unused arrays of boxed `T` trait objects in tuple element 1 that must be used
2019-09-19T08:29:34.8946503Z 33    |
2019-09-19T08:29:34.8946556Z 34 LL |     impl_array();
2019-09-19T08:29:34.8946579Z 
2019-09-19T08:29:34.8946613Z 35    |     ^^^^^^^^^^^^^
2019-09-19T08:29:34.8946613Z 35    |     ^^^^^^^^^^^^^
2019-09-19T08:29:34.8946644Z 36 
2019-09-19T08:29:34.8946860Z - error: unused array of arrays of arrays of `S` that must be used
2019-09-19T08:29:34.8946915Z + error: unused arrays of arrays of arrays of `S` that must be used
2019-09-19T08:29:34.8947154Z 39    |
2019-09-19T08:29:34.8947154Z 39    |
2019-09-19T08:29:34.8947190Z 40 LL |     array_of_arrays_of_arrays();
2019-09-19T08:29:34.8947234Z 
2019-09-19T08:29:34.8947289Z The actual stderr differed from the expected stderr.
2019-09-19T08:29:34.8947687Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-array/must_use-array.stderr
2019-09-19T08:29:34.8947687Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-array/must_use-array.stderr
2019-09-19T08:29:34.8947885Z To update references, rerun the tests and pass the `--bless` flag
2019-09-19T08:29:34.8948121Z To only update this specific test, also pass `--test-args lint/must_use-array.rs`
2019-09-19T08:29:34.8948185Z error: 1 errors occurred comparing output.
2019-09-19T08:29:34.8948220Z status: exit code: 1
2019-09-19T08:29:34.8948220Z status: exit code: 1
2019-09-19T08:29:34.8948988Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/must_use-array.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-array" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-array/auxiliary" "-A" "unused"
2019-09-19T08:29:34.8949322Z ------------------------------------------
2019-09-19T08:29:34.8949437Z 
2019-09-19T08:29:34.8949642Z ------------------------------------------
2019-09-19T08:29:34.8949700Z stderr:
2019-09-19T08:29:34.8949700Z stderr:
2019-09-19T08:29:34.8949878Z ------------------------------------------
2019-09-19T08:29:34.8949920Z error: unused arrays of `S` that must be used
2019-09-19T08:29:34.8950163Z    |
2019-09-19T08:29:34.8950163Z    |
2019-09-19T08:29:34.8950212Z LL |     singleton(); //~ ERROR unused array of `S` that must be used
2019-09-19T08:29:34.8950299Z    |
2019-09-19T08:29:34.8950332Z note: lint level defined here
2019-09-19T08:29:34.8950528Z   --> /checkout/src/test/ui/lint/must_use-array.rs:1:9
2019-09-19T08:29:34.8950585Z    |
2019-09-19T08:29:34.8950585Z    |
2019-09-19T08:29:34.8950618Z LL | #![deny(unused_must_use)]
2019-09-19T08:29:34.8950692Z 
2019-09-19T08:29:34.8950692Z 
2019-09-19T08:29:34.8950727Z error: unused arrays of `S` that must be used
2019-09-19T08:29:34.8950966Z    |
2019-09-19T08:29:34.8950966Z    |
2019-09-19T08:29:34.8951023Z LL |     many(); //~ ERROR unused array of `S` that must be used
2019-09-19T08:29:34.8951083Z 
2019-09-19T08:29:34.8951083Z 
2019-09-19T08:29:34.8951136Z error: unused arrays of `S` in tuple element 0 that must be used
2019-09-19T08:29:34.8951965Z    |
2019-09-19T08:29:34.8951965Z    |
2019-09-19T08:29:34.8952048Z LL |     ([S], 0, ()); //~ ERROR unused array of `S` in tuple element 0 that must be used
2019-09-19T08:29:34.8952135Z 
2019-09-19T08:29:34.8952181Z error: unused arrays of implementers of `T` that must be used
2019-09-19T08:29:34.8952466Z   --> /checkout/src/test/ui/lint/must_use-array.rs:42:5
2019-09-19T08:29:34.8952515Z    |
2019-09-19T08:29:34.8952515Z    |
2019-09-19T08:29:34.8952566Z LL |     array_of_impl_trait(); //~ ERROR unused array of implementers of `T` that must be used
2019-09-19T08:29:34.8952677Z 
2019-09-19T08:29:34.8952677Z 
2019-09-19T08:29:34.8952726Z error: unused arrays of boxed `T` trait objects in tuple element 1 that must be used
2019-09-19T08:29:34.8953048Z    |
2019-09-19T08:29:34.8953089Z LL |     impl_array();
2019-09-19T08:29:34.8953133Z    |     ^^^^^^^^^^^^^
2019-09-19T08:29:34.8953180Z 
2019-09-19T08:29:34.8953180Z 
2019-09-19T08:29:34.8953236Z error: unused arrays of arrays of arrays of `S` that must be used
2019-09-19T08:29:34.8953538Z    |
2019-09-19T08:29:34.8953538Z    |
2019-09-19T08:29:34.8953602Z LL |     array_of_arrays_of_arrays();
2019-09-19T08:29:34.8953677Z 
2019-09-19T08:29:34.8953741Z error: aborting due to 6 previous errors
2019-09-19T08:29:34.8953770Z 
2019-09-19T08:29:34.8953796Z 
2019-09-19T08:29:34.8953796Z 
2019-09-19T08:29:34.8954025Z ------------------------------------------
2019-09-19T08:29:34.8954058Z 
2019-09-19T08:29:34.8954113Z 
2019-09-19T08:29:34.8954348Z ---- [ui] ui/lint/must_use-trait.rs stdout ----
2019-09-19T08:29:34.8954399Z diff of stderr:
2019-09-19T08:29:34.8954426Z 
2019-09-19T08:29:34.8954691Z - error: unused implementer of `Critical` that must be used
2019-09-19T08:29:34.8954746Z + error: unused implementers of `Critical` that must be used
2019-09-19T08:29:34.8955275Z 3    |
2019-09-19T08:29:34.8955275Z 3    |
2019-09-19T08:29:34.8955324Z 4 LL |     get_critical();
2019-09-19T08:29:34.8955348Z 
2019-09-19T08:29:34.8955383Z 10 LL | #![deny(unused_must_use)]
2019-09-19T08:29:34.8955473Z 12 
2019-09-19T08:29:34.8955473Z 12 
2019-09-19T08:29:34.8955707Z - error: unused boxed `Critical` trait object that must be used
2019-09-19T08:29:34.8955773Z + error: unused boxed `Critical` trait objects that must be used
2019-09-19T08:29:34.8956004Z 15    |
2019-09-19T08:29:34.8956004Z 15    |
2019-09-19T08:29:34.8956040Z 16 LL |     get_boxed_critical();
2019-09-19T08:29:34.8956201Z 17    |     ^^^^^^^^^^^^^^^^^^^^^
2019-09-19T08:29:34.8956234Z 18 
2019-09-19T08:29:34.8956234Z 18 
2019-09-19T08:29:34.8956485Z - error: unused boxed boxed `Critical` trait object that must be used
2019-09-19T08:29:34.8956534Z + error: unused boxed boxed `Critical` trait objects that must be used
2019-09-19T08:29:34.8956785Z 21    |
2019-09-19T08:29:34.8956823Z 22 LL |     get_nested_boxed_critical();
2019-09-19T08:29:34.8956847Z 
2019-09-19T08:29:34.8956882Z 23    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-19T08:29:34.8956882Z 23    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-19T08:29:34.8956934Z 24 
2019-09-19T08:29:34.8957316Z - error: unused boxed `Critical` trait object in tuple element 1 that must be used
2019-09-19T08:29:34.8957366Z + error: unused boxed `Critical` trait objects in tuple element 1 that must be used
2019-09-19T08:29:34.8957601Z 27    |
2019-09-19T08:29:34.8957601Z 27    |
2019-09-19T08:29:34.8957635Z 28 LL |     get_critical_tuple();
2019-09-19T08:29:34.8957715Z 29    |     ^^^^^^^^^^^^^^^^^^^^^
2019-09-19T08:29:34.8957747Z 30 
2019-09-19T08:29:34.8957747Z 30 
2019-09-19T08:29:34.8958116Z - error: unused implementer of `Critical` in tuple element 2 that must be used
2019-09-19T08:29:34.8958180Z + error: unused implementers of `Critical` in tuple element 2 that must be used
2019-09-19T08:29:34.8958401Z 33    |
2019-09-19T08:29:34.8958401Z 33    |
2019-09-19T08:29:34.8958434Z 34 LL |     get_critical_tuple();
2019-09-19T08:29:34.8958493Z 
2019-09-19T08:29:34.8958562Z The actual stderr differed from the expected stderr.
2019-09-19T08:29:34.8958991Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-trait/must_use-trait.stderr
2019-09-19T08:29:34.8958991Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-trait/must_use-trait.stderr
2019-09-19T08:29:34.8959223Z To update references, rerun the tests and pass the `--bless` flag
2019-09-19T08:29:34.8959441Z To only update this specific test, also pass `--test-args lint/must_use-trait.rs`
2019-09-19T08:29:34.8959536Z error: 1 errors occurred comparing output.
2019-09-19T08:29:34.8959573Z status: exit code: 1
2019-09-19T08:29:34.8959573Z status: exit code: 1
2019-09-19T08:29:34.8960159Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/must_use-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-trait" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_use-trait/auxiliary" "-A" "unused"
2019-09-19T08:29:34.8960608Z ------------------------------------------
2019-09-19T08:29:34.8960661Z 
2019-09-19T08:29:34.8960849Z ------------------------------------------
2019-09-19T08:29:34.8960887Z stderr:
2019-09-19T08:29:34.8960887Z stderr:
2019-09-19T08:29:34.8961085Z ------------------------------------------
2019-09-19T08:29:34.8961138Z error: unused implementers of `Critical` that must be used
2019-09-19T08:29:34.8961943Z    |
2019-09-19T08:29:34.8961943Z    |
2019-09-19T08:29:34.8961995Z LL |     get_critical(); //~ ERROR unused implementer of `Critical` that must be used
2019-09-19T08:29:34.8962108Z    |
2019-09-19T08:29:34.8962277Z note: lint level defined here
2019-09-19T08:29:34.8962570Z   --> /checkout/src/test/ui/lint/must_use-trait.rs:1:9
2019-09-19T08:29:34.8962619Z    |
2019-09-19T08:29:34.8962619Z    |
2019-09-19T08:29:34.8962682Z LL | #![deny(unused_must_use)]
2019-09-19T08:29:34.8962757Z 
2019-09-19T08:29:34.8962757Z 
2019-09-19T08:29:34.8962823Z error: unused boxed `Critical` trait objects that must be used
2019-09-19T08:29:34.8963126Z    |
2019-09-19T08:29:34.8963126Z    |
2019-09-19T08:29:34.8963178Z LL |     get_boxed_critical(); //~ ERROR unused boxed `Critical` trait object that must be used
2019-09-19T08:29:34.8963371Z 
2019-09-19T08:29:34.8963371Z 
2019-09-19T08:29:34.8963418Z error: unused boxed boxed `Critical` trait objects that must be used
2019-09-19T08:29:34.8963763Z    |
2019-09-19T08:29:34.8963815Z LL |     get_nested_boxed_critical();
2019-09-19T08:29:34.8963879Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-19T08:29:34.8963909Z 
2019-09-19T08:29:34.8963909Z 
2019-09-19T08:29:34.8963957Z error: unused boxed `Critical` trait objects in tuple element 1 that must be used
2019-09-19T08:29:34.8964605Z    |
2019-09-19T08:29:34.8964605Z    |
2019-09-19T08:29:34.8964658Z LL |     get_critical_tuple(); //~ ERROR unused boxed `Critical` trait object in tuple element 1
2019-09-19T08:29:34.8964739Z 
2019-09-19T08:29:34.8964739Z 
2019-09-19T08:29:34.8964819Z error: unused implementers of `Critical` in tuple element 2 that must be used
2019-09-19T08:29:34.8965428Z    |
2019-09-19T08:29:34.8965428Z    |
2019-09-19T08:29:34.8965496Z LL |     get_critical_tuple(); //~ ERROR unused boxed `Critical` trait object in tuple element 1
2019-09-19T08:29:34.8965570Z 
2019-09-19T08:29:34.8965636Z error: aborting due to 5 previous errors
2019-09-19T08:29:34.8965664Z 
2019-09-19T08:29:34.8965688Z 
---
2019-09-19T08:29:34.8973098Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-19T08:29:34.8973182Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-19T08:29:34.8995304Z 
2019-09-19T08:29:34.8995394Z 
2019-09-19T08:29:34.8997086Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-19T08:29:34.8997357Z 
2019-09-19T08:29:34.8997385Z 
2019-09-19T08:29:34.9006966Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-19T08:29:34.9007028Z Build completed unsuccessfully in 1:07:58
2019-09-19T08:29:34.9007028Z Build completed unsuccessfully in 1:07:58
2019-09-19T08:29:34.9049821Z == clock drift check ==
2019-09-19T08:29:34.9066661Z   local time: Thu Sep 19 08:29:34 UTC 2019
2019-09-19T08:29:35.0578245Z   network time: Thu, 19 Sep 2019 08:29:35 GMT
2019-09-19T08:29:35.0581074Z == end clock drift check ==
2019-09-19T08:29:36.1320247Z ##[error]Bash exited with code '1'.
2019-09-19T08:29:36.1365449Z ##[section]Starting: Checkout
2019-09-19T08:29:36.1367472Z ==============================================================================
2019-09-19T08:29:36.1367677Z Task         : Get sources
2019-09-19T08:29:36.1367714Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
