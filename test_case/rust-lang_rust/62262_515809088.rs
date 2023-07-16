plain
2019-07-28T23:20:48.4960993Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-28T23:20:48.5178587Z ##[command]git config gc.auto 0
2019-07-28T23:20:48.5263657Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-28T23:20:48.5314840Z ##[command]git config --get-all http.proxy
2019-07-28T23:20:48.5465827Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62262/merge:refs/remotes/pull/62262/merge
---
2019-07-28T23:21:22.9782127Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-28T23:21:22.9784610Z 
2019-07-28T23:21:22.9786474Z   git checkout -b <new-branch-name>
2019-07-28T23:21:22.9787878Z 
2019-07-28T23:21:22.9788756Z HEAD is now at a8cccd968 Merge 57957a66b409058982044f1ff5b80ff31aad02ea into 4560cb830fce63fcffdc4558f4281aaac6a3a1ba
2019-07-28T23:21:23.0060056Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-28T23:21:23.0063631Z ==============================================================================
2019-07-28T23:21:23.0063810Z Task         : Bash
2019-07-28T23:21:23.0063857Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-29T00:24:34.6905876Z .................................................................................................... 1400/8803
2019-07-29T00:24:40.8308414Z .................................................................................................... 1500/8803
2019-07-29T00:24:53.8555322Z ................................................................i...............i................... 1600/8803
2019-07-29T00:25:01.6529829Z .................................................................................................... 1700/8803
2019-07-29T00:25:17.0770896Z ..................................................iiiii............................................. 1800/8803
2019-07-29T00:25:28.6193452Z .................................................................................................... 2000/8803
2019-07-29T00:25:31.2868718Z .................................................................................................... 2100/8803
2019-07-29T00:25:35.3883692Z .................................................................................................... 2200/8803
2019-07-29T00:25:42.1509504Z .................................................................................................... 2300/8803
---
2019-07-29T00:29:36.0013668Z .................................................................................................... 5200/8803
2019-07-29T00:29:47.1905327Z .................................................................................................... 5300/8803
2019-07-29T00:29:55.1505717Z ..i................................................................................................. 5400/8803
2019-07-29T00:30:00.4995222Z .................................................................................................... 5500/8803
2019-07-29T00:30:13.0243088Z ................................................................................................ii.. 5600/8803
2019-07-29T00:30:28.0760063Z .i..ii...........i.................................................................................. 5700/8803
2019-07-29T00:30:46.7449720Z .................................................................................................... 5900/8803
2019-07-29T00:30:51.6122777Z ................................................................................................i..i 6000/8803
2019-07-29T00:31:06.3441645Z i................................................................................................... 6100/8803
2019-07-29T00:31:23.3903617Z .................................................................................................... 6200/8803
---
2019-07-29T00:36:25.3917038Z  finished in 23.309
2019-07-29T00:36:25.4102520Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-29T00:36:25.5767274Z 
2019-07-29T00:36:25.5767471Z running 146 tests
2019-07-29T00:36:29.0004546Z i....iii......iii..iiii....i............................i..i................i....i.........ii.i.i..i 100/146
2019-07-29T00:36:30.9210903Z iii..............i.........iii.i......ii......
2019-07-29T00:36:30.9214287Z 
2019-07-29T00:36:30.9214555Z  finished in 5.511
2019-07-29T00:36:30.9437409Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-29T00:36:31.1174461Z 
---
2019-07-29T00:36:33.3272003Z  finished in 2.383
2019-07-29T00:36:33.3479307Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-29T00:36:33.5191809Z 
2019-07-29T00:36:33.5194677Z running 9 tests
2019-07-29T00:36:33.5195905Z iiiiiiiii
2019-07-29T00:36:33.5198597Z 
2019-07-29T00:36:33.5198848Z  finished in 0.171
2019-07-29T00:36:33.5392953Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-29T00:36:33.7058881Z 
2019-07-29T00:36:33.7058881Z 
2019-07-29T00:36:33.7059039Z running 104 tests
2019-07-29T00:36:51.6777784Z .....F.............................................................................................. 100/104
2019-07-29T00:36:52.2918583Z ..F.
2019-07-29T00:36:52.2921024Z failures:
2019-07-29T00:36:52.2921086Z 
2019-07-29T00:36:52.2922031Z ---- [incremental] incremental/change_crate_order/main.rs stdout ----
2019-07-29T00:36:52.2922083Z 
2019-07-29T00:36:52.2922334Z error in revision `rpass1`: compilation failed!
2019-07-29T00:36:52.2922444Z status: exit code: 1
2019-07-29T00:36:52.2923671Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_order/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_order/main/auxiliary"
2019-07-29T00:36:52.2924528Z ------------------------------------------
2019-07-29T00:36:52.2924854Z 
2019-07-29T00:36:52.2925425Z ------------------------------------------
2019-07-29T00:36:52.2925609Z stderr:
2019-07-29T00:36:52.2925609Z stderr:
2019-07-29T00:36:52.2925929Z ------------------------------------------
2019-07-29T00:36:52.2926114Z error: unused arithmetic operation that must be used
2019-07-29T00:36:52.2926433Z   --> /checkout/src/test/incremental/change_crate_order/main.rs:23:5
2019-07-29T00:36:52.2926653Z    |
2019-07-29T00:36:52.2926725Z LL |     A + B;
2019-07-29T00:36:52.2926805Z    |     ^^^^^
2019-07-29T00:36:52.2926845Z    |
2019-07-29T00:36:52.2926887Z    = note: `#[deny(unused_must_use)]` on by default
2019-07-29T00:36:52.2927085Z error: aborting due to previous error
2019-07-29T00:36:52.2927138Z 
2019-07-29T00:36:52.2927162Z 
2019-07-29T00:36:52.2927575Z ------------------------------------------
2019-07-29T00:36:52.2927575Z ------------------------------------------
2019-07-29T00:36:52.2927626Z 
2019-07-29T00:36:52.2927788Z 
2019-07-29T00:36:52.2928110Z ---- [incremental] incremental/warnings-reemitted.rs stdout ----
2019-07-29T00:36:52.2928143Z 
2019-07-29T00:36:52.2928830Z error in revision `cfail1`: test compilation failed although it shouldn't!
2019-07-29T00:36:52.2929031Z status: exit code: 1
2019-07-29T00:36:52.2930461Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/warnings-reemitted.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/warnings-reemitted/warnings-reemitted.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/warnings-reemitted" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Coverflow-checks=on" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/warnings-reemitted/auxiliary"
2019-07-29T00:36:52.2931145Z ------------------------------------------
2019-07-29T00:36:52.2931181Z 
2019-07-29T00:36:52.2931441Z ------------------------------------------
2019-07-29T00:36:52.2931488Z stderr:
2019-07-29T00:36:52.2931488Z stderr:
2019-07-29T00:36:52.2931927Z ------------------------------------------
2019-07-29T00:36:52.2932145Z error: unused arithmetic operation that must be used
2019-07-29T00:36:52.2932484Z   --> /checkout/src/test/incremental/warnings-reemitted.rs:9:5
2019-07-29T00:36:52.2932687Z    |
2019-07-29T00:36:52.2933166Z LL |     255u8 + 1; //~ WARNING this expression will panic at run-time
2019-07-29T00:36:52.2933505Z    |
2019-07-29T00:36:52.2933505Z    |
2019-07-29T00:36:52.2933561Z    = note: `#[deny(unused_must_use)]` on by default
2019-07-29T00:36:52.2933665Z error: aborting due to previous error
2019-07-29T00:36:52.2933690Z 
2019-07-29T00:36:52.2933856Z 
2019-07-29T00:36:52.2934183Z ------------------------------------------
---
2019-07-29T00:36:52.2936204Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-07-29T00:36:52.2936258Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-29T00:36:52.2936441Z 
2019-07-29T00:36:52.2936500Z 
2019-07-29T00:36:52.2937884Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-29T00:36:52.2938298Z 
2019-07-29T00:36:52.2938325Z 
2019-07-29T00:36:52.2947912Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-29T00:36:52.2948102Z Build completed unsuccessfully in 1:09:11
2019-07-29T00:36:52.2948102Z Build completed unsuccessfully in 1:09:11
2019-07-29T00:36:55.7766558Z ##[error]Bash exited with code '1'.
2019-07-29T00:36:55.7809331Z ##[section]Starting: Checkout
2019-07-29T00:36:55.7811213Z ==============================================================================
2019-07-29T00:36:55.7811289Z Task         : Get sources
2019-07-29T00:36:55.7811342Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
