plain
2020-01-16T20:06:48.2310770Z ========================== Starting Command Output ===========================
2020-01-16T20:06:48.2312227Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f4a560de-d659-4b70-a36c-943dceb8f175.sh
2020-01-16T20:06:48.2312262Z 
2020-01-16T20:06:48.2314844Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-16T20:06:48.2321395Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67688/merge to s
2020-01-16T20:06:48.2323037Z Task         : Get sources
2020-01-16T20:06:48.2323069Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-16T20:06:48.2323194Z Version      : 1.0.0
2020-01-16T20:06:48.2323276Z Author       : Microsoft
---
2020-01-16T20:06:49.2145723Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-16T20:06:49.2157121Z ##[command]git config gc.auto 0
2020-01-16T20:06:49.2160498Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-16T20:06:49.2166203Z ##[command]git config --get-all http.proxy
2020-01-16T20:06:49.2175096Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67688/merge:refs/remotes/pull/67688/merge
---
2020-01-16T21:05:23.9310958Z .................................................................................................... 1700/9524
2020-01-16T21:05:32.6722130Z .................................................................................................... 1800/9524
2020-01-16T21:05:42.6474702Z ...........i........................................................................................ 1900/9524
2020-01-16T21:05:50.0738490Z .................................................................................................... 2000/9524
2020-01-16T21:06:07.3131614Z .iiiii..........................................................................F................... 2100/9524
2020-01-16T21:06:16.1336702Z ......................................F............................................................. 2300/9524
2020-01-16T21:06:18.6738682Z .................................................................................................... 2400/9524
2020-01-16T21:06:24.5577884Z .................................................................................................... 2500/9524
2020-01-16T21:06:46.1793881Z .................................................................................................... 2600/9524
---
2020-01-16T21:09:35.6582804Z ............................................i...............i....................................... 4900/9524
2020-01-16T21:09:45.0254884Z .................................................................................................... 5000/9524
2020-01-16T21:09:52.3234926Z .......................................................................................i............ 5100/9524
2020-01-16T21:09:57.7618082Z .................................................................................................... 5200/9524
2020-01-16T21:10:09.1660772Z ...........................................................ii.ii...........i........................ 5300/9524
2020-01-16T21:10:18.7053952Z .................................................................................................... 5500/9524
2020-01-16T21:10:29.3121982Z .................................................................................................... 5600/9524
2020-01-16T21:10:36.0511117Z ............................................i....................................................... 5700/9524
2020-01-16T21:10:43.2462357Z .................................................................................................... 5800/9524
2020-01-16T21:10:43.2462357Z .................................................................................................... 5800/9524
2020-01-16T21:10:54.4519338Z .................................................................................................... 5900/9524
2020-01-16T21:11:04.3113087Z ...................................ii...i...ii..........i........................................... 6000/9524
2020-01-16T21:11:24.3504904Z .................................................................................................... 6200/9524
2020-01-16T21:11:32.9123357Z .................................................................................................... 6300/9524
2020-01-16T21:11:32.9123357Z .................................................................................................... 6300/9524
2020-01-16T21:11:42.1304568Z ...............................................................i..ii................................ 6400/9524
2020-01-16T21:12:12.2306641Z ......................................................................F.F........................... 6600/9524
2020-01-16T21:12:14.6172051Z .......................................i............................................................ 6700/9524
2020-01-16T21:12:16.8908459Z .................................................................................................... 6800/9524
2020-01-16T21:12:19.5608302Z .......................................i............................................................ 6900/9524
---
2020-01-16T21:14:03.1788063Z .................................................................................................... 7500/9524
2020-01-16T21:14:07.8450726Z .................................................................................................... 7600/9524
2020-01-16T21:14:14.2471617Z .................................................................................................... 7700/9524
2020-01-16T21:14:21.3066171Z .................................................................................................... 7800/9524
2020-01-16T21:14:32.0631112Z .........................................................................................iiii....... 7900/9524
2020-01-16T21:14:49.5846898Z ......................i......i...................................................................... 8100/9524
2020-01-16T21:14:54.9063713Z .................................................................................................... 8200/9524
2020-01-16T21:15:08.6805445Z .................................................................................................... 8300/9524
2020-01-16T21:15:20.0587103Z .................................................................................................... 8400/9524
---
2020-01-16T21:17:27.7897543Z 
2020-01-16T21:17:27.7898803Z ---- [ui] ui/duplicate_entry_error.rs stdout ----
2020-01-16T21:17:27.7898873Z diff of stderr:
2020-01-16T21:17:27.7898935Z 
2020-01-16T21:17:27.7899558Z - error[E0152]: found duplicate lang item `panic_impl`
2020-01-16T21:17:27.7900333Z + error[E0152]: duplicate lang item found: `panic_impl`.
2020-01-16T21:17:27.7900734Z 3    |
2020-01-16T21:17:27.7900734Z 3    |
2020-01-16T21:17:27.7900985Z 4 LL | / fn panic_impl(info: &PanicInfo) -> ! {
2020-01-16T21:17:27.7901823Z 
2020-01-16T21:17:27.7901887Z The actual stderr differed from the expected stderr.
2020-01-16T21:17:27.7902354Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate_entry_error/duplicate_entry_error.stderr
2020-01-16T21:17:27.7902354Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate_entry_error/duplicate_entry_error.stderr
2020-01-16T21:17:27.7902900Z To update references, rerun the tests and pass the `--bless` flag
2020-01-16T21:17:27.7903407Z To only update this specific test, also pass `--test-args duplicate_entry_error.rs`
2020-01-16T21:17:27.7903706Z error: 1 errors occurred comparing output.
2020-01-16T21:17:27.7903757Z status: exit code: 1
2020-01-16T21:17:27.7903757Z status: exit code: 1
2020-01-16T21:17:27.7905165Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/duplicate_entry_error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate_entry_error" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate_entry_error/auxiliary" "-A" "unused"
2020-01-16T21:17:27.7911322Z ------------------------------------------
2020-01-16T21:17:27.7911547Z 
2020-01-16T21:17:27.7911917Z ------------------------------------------
2020-01-16T21:17:27.7912034Z stderr:
2020-01-16T21:17:27.7912034Z stderr:
2020-01-16T21:17:27.7912928Z ------------------------------------------
2020-01-16T21:17:27.7913156Z error[E0152]: duplicate lang item found: `panic_impl`.
2020-01-16T21:17:27.7913736Z    |
2020-01-16T21:17:27.7913736Z    |
2020-01-16T21:17:27.7946621Z LL | / fn panic_impl(info: &PanicInfo) -> ! {
2020-01-16T21:17:27.7949273Z LL | | //~^ ERROR: found duplicate lang item `panic_impl`
2020-01-16T21:17:27.7949412Z LL | | }
2020-01-16T21:17:27.7949456Z    | |_^
2020-01-16T21:17:27.7949518Z    |
2020-01-16T21:17:27.7949518Z    |
2020-01-16T21:17:27.7949572Z    = note: first defined in crate `std` (which `duplicate_entry_error` depends on)
2020-01-16T21:17:27.7949700Z error: aborting due to previous error
2020-01-16T21:17:27.7949733Z 
2020-01-16T21:17:27.7950169Z For more information about this error, try `rustc --explain E0152`.
2020-01-16T21:17:27.7950217Z 
2020-01-16T21:17:27.7950217Z 
2020-01-16T21:17:27.7950482Z ------------------------------------------
2020-01-16T21:17:27.7950517Z 
2020-01-16T21:17:27.7950544Z 
2020-01-16T21:17:27.7950790Z ---- [ui] ui/error-codes/E0152.rs stdout ----
2020-01-16T21:17:27.7950840Z diff of stderr:
2020-01-16T21:17:27.7950888Z 
2020-01-16T21:17:27.7951134Z - error[E0152]: found duplicate lang item `arc`
2020-01-16T21:17:27.7951188Z + error[E0152]: duplicate lang item found: `arc`.
2020-01-16T21:17:27.7951430Z 2   --> $DIR/E0152.rs:4:1
2020-01-16T21:17:27.7951524Z 4 LL | struct Foo;
2020-01-16T21:17:27.7951554Z 
2020-01-16T21:17:27.7951598Z 
2020-01-16T21:17:27.7951647Z The actual stderr differed from the expected stderr.
2020-01-16T21:17:27.7951647Z The actual stderr differed from the expected stderr.
2020-01-16T21:17:27.7951968Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0152/E0152.stderr
2020-01-16T21:17:27.7952497Z To update references, rerun the tests and pass the `--bless` flag
2020-01-16T21:17:27.7952909Z To only update this specific test, also pass `--test-args error-codes/E0152.rs`
2020-01-16T21:17:27.7953008Z error: 1 errors occurred comparing output.
2020-01-16T21:17:27.7953076Z status: exit code: 1
2020-01-16T21:17:27.7953076Z status: exit code: 1
2020-01-16T21:17:27.7954519Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0152.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0152" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0152/auxiliary" "-A" "unused"
2020-01-16T21:17:27.7955040Z ------------------------------------------
2020-01-16T21:17:27.7955078Z 
2020-01-16T21:17:27.7955349Z ------------------------------------------
2020-01-16T21:17:27.7955399Z stderr:
2020-01-16T21:17:27.7955399Z stderr:
2020-01-16T21:17:27.7955633Z ------------------------------------------
2020-01-16T21:17:27.7955705Z error[E0152]: duplicate lang item found: `arc`.
2020-01-16T21:17:27.7956012Z    |
2020-01-16T21:17:27.7956077Z LL | struct Foo; //~ ERROR E0152
2020-01-16T21:17:27.7956126Z    | ^^^^^^^^^^^
2020-01-16T21:17:27.7956170Z    |
---
2020-01-16T21:17:27.7956982Z 
2020-01-16T21:17:27.7957266Z ---- [ui] ui/panic-handler/panic-handler-duplicate.rs stdout ----
2020-01-16T21:17:27.7957322Z diff of stderr:
2020-01-16T21:17:27.7957351Z 
2020-01-16T21:17:27.7957604Z - error[E0152]: found duplicate lang item `panic_impl`
2020-01-16T21:17:27.7957679Z + error[E0152]: duplicate lang item found: `panic_impl`.
2020-01-16T21:17:27.7957974Z 3    |
2020-01-16T21:17:27.7957974Z 3    |
2020-01-16T21:17:27.7958229Z 4 LL | / fn panic2(info: &PanicInfo) -> ! {
2020-01-16T21:17:27.7958291Z 
2020-01-16T21:17:27.7958340Z The actual stderr differed from the expected stderr.
2020-01-16T21:17:27.7958719Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-duplicate/panic-handler-duplicate.stderr
2020-01-16T21:17:27.7958719Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-duplicate/panic-handler-duplicate.stderr
2020-01-16T21:17:27.7958995Z To update references, rerun the tests and pass the `--bless` flag
2020-01-16T21:17:27.7959314Z To only update this specific test, also pass `--test-args panic-handler/panic-handler-duplicate.rs`
2020-01-16T21:17:27.7959426Z error: 1 errors occurred comparing output.
2020-01-16T21:17:27.7959475Z status: exit code: 1
2020-01-16T21:17:27.7959475Z status: exit code: 1
2020-01-16T21:17:27.7961064Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-handler/panic-handler-duplicate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-duplicate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-duplicate/auxiliary" "-A" "unused"
2020-01-16T21:17:27.7961661Z ------------------------------------------
2020-01-16T21:17:27.7961698Z 
2020-01-16T21:17:27.7961936Z ------------------------------------------
2020-01-16T21:17:27.7962095Z stderr:
2020-01-16T21:17:27.7962095Z stderr:
2020-01-16T21:17:27.7962375Z ------------------------------------------
2020-01-16T21:17:27.7962429Z error[E0152]: duplicate lang item found: `panic_impl`.
2020-01-16T21:17:27.7962779Z    |
2020-01-16T21:17:27.7962779Z    |
2020-01-16T21:17:27.7963072Z LL | / fn panic2(info: &PanicInfo) -> ! { //~ ERROR found duplicate lang item `panic_impl`
2020-01-16T21:17:27.7963190Z LL | | }
2020-01-16T21:17:27.7963233Z    | |_^
2020-01-16T21:17:27.7963275Z    |
2020-01-16T21:17:27.7963338Z note: first defined here
2020-01-16T21:17:27.7963338Z note: first defined here
2020-01-16T21:17:27.7963612Z   --> /checkout/src/test/ui/panic-handler/panic-handler-duplicate.rs:10:1
2020-01-16T21:17:27.7963674Z    |
2020-01-16T21:17:27.7963921Z LL | / fn panic(info: &PanicInfo) -> ! {
2020-01-16T21:17:27.7964343Z LL | | }
2020-01-16T21:17:27.7964385Z    | |_^
2020-01-16T21:17:27.7964434Z 
2020-01-16T21:17:27.7964491Z error: aborting due to previous error
---
2020-01-16T21:17:27.7965230Z 
2020-01-16T21:17:27.7965500Z ---- [ui] ui/panic-handler/panic-handler-std.rs stdout ----
2020-01-16T21:17:27.7965552Z diff of stderr:
2020-01-16T21:17:27.7965583Z 
2020-01-16T21:17:27.7965832Z - error[E0152]: found duplicate lang item `panic_impl`
2020-01-16T21:17:27.7965905Z + error[E0152]: duplicate lang item found: `panic_impl`.
2020-01-16T21:17:27.7966198Z 3    |
2020-01-16T21:17:27.7966198Z 3    |
2020-01-16T21:17:27.7966453Z 4 LL | / fn panic(info: PanicInfo) -> ! {
2020-01-16T21:17:27.7966514Z 
2020-01-16T21:17:27.7966562Z The actual stderr differed from the expected stderr.
2020-01-16T21:17:27.7966929Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-std/panic-handler-std.stderr
2020-01-16T21:17:27.7966929Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-std/panic-handler-std.stderr
2020-01-16T21:17:27.7967207Z To update references, rerun the tests and pass the `--bless` flag
2020-01-16T21:17:27.7967545Z To only update this specific test, also pass `--test-args panic-handler/panic-handler-std.rs`
2020-01-16T21:17:27.7967658Z error: 1 errors occurred comparing output.
2020-01-16T21:17:27.7967708Z status: exit code: 1
2020-01-16T21:17:27.7967708Z status: exit code: 1
2020-01-16T21:17:27.7968677Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-handler/panic-handler-std.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-std" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-std/auxiliary" "-A" "unused"
2020-01-16T21:17:27.7969079Z ------------------------------------------
2020-01-16T21:17:27.7969117Z 
2020-01-16T21:17:27.7969379Z ------------------------------------------
2020-01-16T21:17:27.7969431Z stderr:
2020-01-16T21:17:27.7969431Z stderr:
2020-01-16T21:17:27.7969704Z ------------------------------------------
2020-01-16T21:17:27.7969761Z error[E0152]: duplicate lang item found: `panic_impl`.
2020-01-16T21:17:27.7970124Z    |
2020-01-16T21:17:27.7970124Z    |
2020-01-16T21:17:27.7970379Z LL | / fn panic(info: PanicInfo) -> ! {
2020-01-16T21:17:27.7970645Z LL | | }
2020-01-16T21:17:27.7970691Z    | |_^
2020-01-16T21:17:27.7970735Z    |
2020-01-16T21:17:27.7970735Z    |
2020-01-16T21:17:27.7970874Z    = note: first defined in crate `std` (which `panic_handler_std` depends on)
2020-01-16T21:17:27.7970969Z error: argument should be `&PanicInfo`
2020-01-16T21:17:27.7971300Z   --> /checkout/src/test/ui/panic-handler/panic-handler-std.rs:7:16
2020-01-16T21:17:27.7971375Z    |
2020-01-16T21:17:27.7971375Z    |
2020-01-16T21:17:27.7971628Z LL | fn panic(info: PanicInfo) -> ! {
2020-01-16T21:17:27.7971716Z 
2020-01-16T21:17:27.7971781Z error: aborting due to 2 previous errors
2020-01-16T21:17:27.7971813Z 
2020-01-16T21:17:27.7972100Z For more information about this error, try `rustc --explain E0152`.
---
2020-01-16T21:17:27.7974571Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-16T21:17:27.7974650Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-16T21:17:27.7999287Z 
2020-01-16T21:17:27.8001056Z 
2020-01-16T21:17:27.8007850Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-16T21:17:27.8008186Z 
2020-01-16T21:17:27.8008227Z 
2020-01-16T21:17:27.8017759Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-16T21:17:27.8017845Z Build completed unsuccessfully in 1:04:47
2020-01-16T21:17:27.8017845Z Build completed unsuccessfully in 1:04:47
2020-01-16T21:17:27.8075425Z == clock drift check ==
2020-01-16T21:17:27.8104689Z   local time: Thu Jan 16 21:17:27 UTC 2020
2020-01-16T21:17:27.9788931Z   network time: Thu, 16 Jan 2020 21:17:27 GMT
2020-01-16T21:17:27.9789349Z == end clock drift check ==
2020-01-16T21:17:28.5892899Z 
2020-01-16T21:17:28.6000951Z ##[error]Bash exited with code '1'.
2020-01-16T21:17:28.6013056Z ##[section]Finishing: Run build
2020-01-16T21:17:28.6032562Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67688/merge to s
2020-01-16T21:17:28.6034863Z Task         : Get sources
2020-01-16T21:17:28.6034908Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-16T21:17:28.6034970Z Version      : 1.0.0
2020-01-16T21:17:28.6035011Z Author       : Microsoft
2020-01-16T21:17:28.6035011Z Author       : Microsoft
2020-01-16T21:17:28.6035055Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-16T21:17:28.6035121Z ==============================================================================
2020-01-16T21:17:29.0503829Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-16T21:17:29.0545121Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67688/merge to s
2020-01-16T21:17:29.0667066Z Cleaning up task key
2020-01-16T21:17:29.0667822Z Start cleaning up orphan processes.
2020-01-16T21:17:29.0792175Z Terminate orphan process: pid (3454) (python)
2020-01-16T21:17:29.1032810Z ##[section]Finishing: Finalize Job
