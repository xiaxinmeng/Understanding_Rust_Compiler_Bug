plain
2019-09-10T19:46:01.2337871Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-10T19:46:01.2503261Z ##[command]git config gc.auto 0
2019-09-10T19:46:01.2573446Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-10T19:46:01.2630307Z ##[command]git config --get-all http.proxy
2019-09-10T19:46:01.2744736Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64360/merge:refs/remotes/pull/64360/merge
---
2019-09-10T20:45:44.3871192Z .................................................................................................... 1500/9007
2019-09-10T20:45:49.9960632Z .................................................................................................... 1600/9007
2019-09-10T20:46:02.3203450Z ........................................................i...............i........................... 1700/9007
2019-09-10T20:46:09.8249747Z .................................................................................................... 1800/9007
2019-09-10T20:46:23.9868974Z ...............................................iiiii................................................ 1900/9007
2019-09-10T20:46:34.5603378Z .................................................................................................... 2100/9007
2019-09-10T20:46:36.8940017Z .................................................................................................... 2200/9007
2019-09-10T20:46:40.4148059Z .................................................................................................... 2300/9007
2019-09-10T20:46:47.9337268Z .................................................................................................... 2400/9007
---
2019-09-10T20:49:39.3922156Z ..................................i...............i................................................. 4700/9007
2019-09-10T20:49:50.4564564Z .................................................................................................... 4800/9007
2019-09-10T20:49:56.6611604Z .................................................................................................... 4900/9007
2019-09-10T20:50:06.4737287Z .................................................................................................... 5000/9007
2019-09-10T20:50:12.0797411Z .................ii.ii.............................................................................. 5100/9007
2019-09-10T20:50:21.3202471Z .................................................................................................... 5300/9007
2019-09-10T20:50:30.2306074Z ................................................................................i................... 5400/9007
2019-09-10T20:50:37.0826531Z .................................................................................................... 5500/9007
2019-09-10T20:50:42.3103897Z .................................................................................................... 5600/9007
2019-09-10T20:50:42.3103897Z .................................................................................................... 5600/9007
2019-09-10T20:50:51.5950384Z ..........................................................................ii...i..ii...........i.... 5700/9007
2019-09-10T20:51:13.4838028Z .................................................................................................... 5900/9007
2019-09-10T20:51:21.8273717Z .................................................................................................... 6000/9007
2019-09-10T20:51:21.8273717Z .................................................................................................... 6000/9007
2019-09-10T20:51:26.3685753Z ............................................................................i..ii................... 6100/9007
2019-09-10T20:51:52.6747279Z .................................................................................................... 6300/9007
2019-09-10T20:51:54.4536529Z ...................................i................................................................ 6400/9007
2019-09-10T20:51:56.3139244Z .................................................................................................... 6500/9007
2019-09-10T20:51:58.4687472Z .......i............................................................................................ 6600/9007
---
2019-09-10T20:55:32.0409004Z 
2019-09-10T20:55:32.0410120Z ---- [ui] ui/const-generics/foreign-item-const-parameter.rs stdout ----
2019-09-10T20:55:32.0410220Z diff of stderr:
2019-09-10T20:55:32.0410256Z 
2019-09-10T20:55:32.0410303Z 12 LL |     fn foo<const X: usize>();
2019-09-10T20:55:32.0410659Z 14    |
2019-09-10T20:55:32.0410928Z -    = help: use specialization instead of const parameters by replacing them with concrete const
2019-09-10T20:55:32.0410991Z +    = help: use specialization instead of const parameters by replacing them with concrete consts
2019-09-10T20:55:32.0411061Z 16 
2019-09-10T20:55:32.0411061Z 16 
2019-09-10T20:55:32.0411157Z 17 error[E0044]: foreign items may not have type or const parameters
2019-09-10T20:55:32.0411402Z 18   --> $DIR/foreign-item-const-parameter.rs:7:5
2019-09-10T20:55:32.0411455Z 
2019-09-10T20:55:32.0411501Z 20 LL |     fn bar<T, const X: usize>(_: T);
2019-09-10T20:55:32.0411824Z 22    |
2019-09-10T20:55:32.0412105Z -    = help: use specialization instead of type or const parameters by replacing them with concrete type or const
2019-09-10T20:55:32.0412172Z +    = help: use specialization instead of type or const parameters by replacing them with concrete types or consts
2019-09-10T20:55:32.0412240Z 24 
2019-09-10T20:55:32.0412240Z 24 
2019-09-10T20:55:32.0412286Z 25 error: aborting due to 2 previous errors
2019-09-10T20:55:32.0412328Z 26 
2019-09-10T20:55:32.0412355Z 
2019-09-10T20:55:32.0412381Z 
2019-09-10T20:55:32.0412446Z The actual stderr differed from the expected stderr.
2019-09-10T20:55:32.0412802Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/foreign-item-const-parameter/foreign-item-const-parameter.stderr
2019-09-10T20:55:32.0413075Z To update references, rerun the tests and pass the `--bless` flag
2019-09-10T20:55:32.0413815Z To only update this specific test, also pass `--test-args const-generics/foreign-item-const-parameter.rs`
2019-09-10T20:55:32.0413875Z error: 1 errors occurred comparing output.
2019-09-10T20:55:32.0413925Z status: exit code: 1
2019-09-10T20:55:32.0413925Z status: exit code: 1
2019-09-10T20:55:32.0414499Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/foreign-item-const-parameter.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/foreign-item-const-parameter" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/foreign-item-const-parameter/auxiliary" "-A" "unused"
2019-09-10T20:55:32.0414754Z ------------------------------------------
2019-09-10T20:55:32.0414778Z 
2019-09-10T20:55:32.0414950Z ------------------------------------------
2019-09-10T20:55:32.0414984Z stderr:
---
2019-09-10T20:55:32.0415608Z 
2019-09-10T20:55:32.0415913Z error[E0044]: foreign items may not have const parameters
2019-09-10T20:55:32.0416151Z   --> /checkout/src/test/ui/const-generics/foreign-item-const-parameter.rs:5:5
2019-09-10T20:55:32.0416207Z    |
2019-09-10T20:55:32.0416244Z LL |     fn foo<const X: usize>(); //~ ERROR foreign items may not have const parameters
2019-09-10T20:55:32.0416470Z    |
2019-09-10T20:55:32.0416507Z    = help: use specialization instead of const parameters by replacing them with concrete consts
2019-09-10T20:55:32.0416533Z 
2019-09-10T20:55:32.0416567Z error[E0044]: foreign items may not have type or const parameters
2019-09-10T20:55:32.0416567Z error[E0044]: foreign items may not have type or const parameters
2019-09-10T20:55:32.0416770Z   --> /checkout/src/test/ui/const-generics/foreign-item-const-parameter.rs:7:5
2019-09-10T20:55:32.0416806Z    |
2019-09-10T20:55:32.0416844Z LL |     fn bar<T, const X: usize>(_: T); //~ ERROR foreign items may not have type or const parameters
2019-09-10T20:55:32.0417104Z    |
2019-09-10T20:55:32.0417143Z    = help: use specialization instead of type or const parameters by replacing them with concrete types or consts
2019-09-10T20:55:32.0417189Z 
2019-09-10T20:55:32.0417224Z error: aborting due to 2 previous errors
---
2019-09-10T20:55:32.0438980Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-10T20:55:32.0439475Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-10T20:55:32.0453499Z 
2019-09-10T20:55:32.0454173Z 
2019-09-10T20:55:32.0464717Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-10T20:55:32.0464965Z 
2019-09-10T20:55:32.0465007Z 
2019-09-10T20:55:32.0470227Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-10T20:55:32.0470472Z Build completed unsuccessfully in 1:02:26
2019-09-10T20:55:32.0470472Z Build completed unsuccessfully in 1:02:26
2019-09-10T20:55:32.0524845Z == clock drift check ==
2019-09-10T20:55:32.0543165Z   local time: Tue Sep 10 20:55:32 UTC 2019
2019-09-10T20:55:32.2025814Z   network time: Tue, 10 Sep 2019 20:55:32 GMT
2019-09-10T20:55:32.2028178Z == end clock drift check ==
2019-09-10T20:55:33.0470589Z ##[error]Bash exited with code '1'.
2019-09-10T20:55:33.0510036Z ##[section]Starting: Checkout
2019-09-10T20:55:33.0511947Z ==============================================================================
2019-09-10T20:55:33.0512004Z Task         : Get sources
2019-09-10T20:55:33.0512071Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
