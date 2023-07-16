plain
2019-08-12T18:58:37.8552072Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-12T18:58:37.8730136Z ##[command]git config gc.auto 0
2019-08-12T18:58:37.8807199Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-12T18:58:37.8857854Z ##[command]git config --get-all http.proxy
2019-08-12T18:58:37.8988671Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63470/merge:refs/remotes/pull/63470/merge
---
2019-08-12T18:59:13.5557268Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-12T18:59:13.5557320Z 
2019-08-12T18:59:13.5557745Z   git checkout -b <new-branch-name>
2019-08-12T18:59:13.5557775Z 
2019-08-12T18:59:13.5557825Z HEAD is now at d29203c7d Merge 0be63bd405b1a78f07e528b60972a0e05ee9d1f4 into 60960a260f7b5c695fd0717311d72ce62dd4eb43
2019-08-12T18:59:13.5718160Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-12T18:59:13.5721160Z ==============================================================================
2019-08-12T18:59:13.5721206Z Task         : Bash
2019-08-12T18:59:13.5721240Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-12T19:57:52.7030279Z .................................................................................................... 1300/8875
2019-08-12T19:57:59.2473769Z .................................................................................................... 1400/8875
2019-08-12T19:58:05.4539230Z .................................................................................................... 1500/8875
2019-08-12T19:58:16.0749222Z ....................................................................................i............... 1600/8875
2019-08-12T19:58:23.5643833Z i................................................................................................... 1700/8875
2019-08-12T19:58:30.0542706Z ...........................................................................iiiii.................... 1800/8875
2019-08-12T19:58:51.5059152Z .................................................................................................... 2000/8875
2019-08-12T19:58:53.8218187Z .................................................................................................... 2100/8875
2019-08-12T19:58:56.3277882Z .................................................................................................... 2200/8875
2019-08-12T19:59:03.6203121Z .................................................................................................... 2300/8875
---
2019-08-12T20:02:49.0796314Z ......................................................F............................................. 5300/8875
2019-08-12T20:02:56.0246796Z ........i........................................................................................... 5400/8875
2019-08-12T20:03:01.1637736Z .................................................................................................... 5500/8875
2019-08-12T20:03:12.8930664Z .................................................................................................... 5600/8875
2019-08-12T20:03:26.6177343Z ...ii...i..ii...........i........................................................................... 5700/8875
2019-08-12T20:03:41.1958596Z .................................................................................................... 5900/8875
2019-08-12T20:03:45.7230242Z .................................................................................................... 6000/8875
2019-08-12T20:03:45.7230242Z .................................................................................................... 6000/8875
2019-08-12T20:03:59.3523629Z ....i..ii........................................................................................... 6100/8875
2019-08-12T20:04:17.3355668Z ...............................................i.................................................... 6300/8875
2019-08-12T20:04:19.3514211Z .................................................................................................... 6400/8875
2019-08-12T20:04:21.6889439Z ...................i................................................................................ 6500/8875
2019-08-12T20:04:25.9978187Z .................................................................................................... 6600/8875
---
2019-08-12T20:08:16.9345838Z failures:
2019-08-12T20:08:16.9367396Z 
2019-08-12T20:08:16.9368023Z ---- [ui] ui/macros/builtin-std-paths.rs stdout ----
2019-08-12T20:08:16.9368691Z 
2019-08-12T20:08:16.9369270Z error: test compilation failed although it shouldn't!
2019-08-12T20:08:16.9369785Z status: exit code: 1
2019-08-12T20:08:16.9370864Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/builtin-std-paths.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/builtin-std-paths" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/builtin-std-paths/auxiliary" "-A" "unused"
2019-08-12T20:08:16.9372149Z ------------------------------------------
2019-08-12T20:08:16.9372400Z 
2019-08-12T20:08:16.9372779Z ------------------------------------------
2019-08-12T20:08:16.9373001Z stderr:
2019-08-12T20:08:16.9373001Z stderr:
2019-08-12T20:08:16.9373718Z ------------------------------------------
2019-08-12T20:08:16.9373985Z error[E0433]: failed to resolve: could not find `Hash` in `hash`
2019-08-12T20:08:16.9374669Z    |
2019-08-12T20:08:16.9374834Z LL |     core::hash::Hash,
2019-08-12T20:08:16.9374834Z LL |     core::hash::Hash,
2019-08-12T20:08:16.9375019Z    |                 ^^^^ could not find `Hash` in `hash`
2019-08-12T20:08:16.9375164Z 
2019-08-12T20:08:16.9375328Z error[E0433]: failed to resolve: could not find `Hash` in `hash`
2019-08-12T20:08:16.9375929Z    |
2019-08-12T20:08:16.9376501Z LL |     std::hash::Hash,
2019-08-12T20:08:16.9376501Z LL |     std::hash::Hash,
2019-08-12T20:08:16.9376681Z    |                ^^^^ could not find `Hash` in `hash`
2019-08-12T20:08:16.9377175Z error: aborting due to 2 previous errors
2019-08-12T20:08:16.9377299Z 
2019-08-12T20:08:16.9377716Z For more information about this error, try `rustc --explain E0433`.
2019-08-12T20:08:16.9377849Z 
---
2019-08-12T20:08:16.9400139Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-12T20:08:16.9400428Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-12T20:08:16.9416666Z 
2019-08-12T20:08:16.9418035Z 
2019-08-12T20:08:16.9420739Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-12T20:08:16.9421235Z 
2019-08-12T20:08:16.9421287Z 
2019-08-12T20:08:16.9430791Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-12T20:08:16.9430932Z Build completed unsuccessfully in 1:02:46
2019-08-12T20:08:16.9430932Z Build completed unsuccessfully in 1:02:46
2019-08-12T20:08:18.1170799Z ##[error]Bash exited with code '1'.
2019-08-12T20:08:18.1212444Z ##[section]Starting: Checkout
2019-08-12T20:08:18.1214185Z ==============================================================================
2019-08-12T20:08:18.1214230Z Task         : Get sources
2019-08-12T20:08:18.1214284Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
