plain
2020-01-26T16:33:22.5665444Z ========================== Starting Command Output ===========================
2020-01-26T16:33:22.5666951Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/821100fb-31e0-4ccb-869b-8cb3788ea74e.sh
2020-01-26T16:33:22.5666985Z 
2020-01-26T16:33:22.5669943Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-26T16:33:22.5676344Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68554/merge to s
2020-01-26T16:33:22.5678081Z Task         : Get sources
2020-01-26T16:33:22.5678163Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-26T16:33:22.5678197Z Version      : 1.0.0
2020-01-26T16:33:22.5678233Z Author       : Microsoft
---
2020-01-26T16:33:23.3807610Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-26T16:33:23.3881903Z ##[command]git config gc.auto 0
2020-01-26T16:33:23.3961825Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-26T16:33:23.4012969Z ##[command]git config --get-all http.proxy
2020-01-26T16:33:23.4154884Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68554/merge:refs/remotes/pull/68554/merge
---
2020-01-26T17:22:27.4655396Z .................................................................................................... 1700/9556
2020-01-26T17:22:32.9886912Z .................................................................................................... 1800/9556
2020-01-26T17:22:43.2877648Z .......................i............................................................................ 1900/9556
2020-01-26T17:22:49.5699360Z .................................................................................................... 2000/9556
2020-01-26T17:23:02.1250974Z .............iiiii.........................................................................F........ 2100/9556
2020-01-26T17:23:10.7261860Z ..................................................F................................................. 2300/9556
2020-01-26T17:23:12.8326997Z .................................................................................................... 2400/9556
2020-01-26T17:23:17.5256312Z .................................................................................................... 2500/9556
2020-01-26T17:23:35.4130905Z .................................................................................................... 2600/9556
---
2020-01-26T17:25:57.7831882Z .................................................................................................... 4800/9556
2020-01-26T17:26:02.5225773Z ..........................................................i...............i......................... 4900/9556
2020-01-26T17:26:09.7642703Z .................................................................................................... 5000/9556
2020-01-26T17:26:16.8117866Z .................................................................................................... 5100/9556
2020-01-26T17:26:21.2148028Z .i.................................................................................................. 5200/9556
2020-01-26T17:26:30.8665083Z ..........................................................................ii.ii........i...i........ 5300/9556
2020-01-26T17:26:38.6946046Z ............i....................................................................................... 5500/9556
2020-01-26T17:26:47.4361879Z .................................................................................................... 5600/9556
2020-01-26T17:26:53.1007807Z .............................................................i...................................... 5700/9556
2020-01-26T17:26:59.4981708Z .................................................................................................... 5800/9556
2020-01-26T17:26:59.4981708Z .................................................................................................... 5800/9556
2020-01-26T17:27:06.6921709Z .................................................................................................... 5900/9556
2020-01-26T17:27:14.3311689Z ....................................................ii...i..ii...........i.......................... 6000/9556
2020-01-26T17:27:34.3716742Z .................................................................................................... 6200/9556
2020-01-26T17:27:38.4842880Z .................................................................................................... 6300/9556
2020-01-26T17:27:38.4842880Z .................................................................................................... 6300/9556
2020-01-26T17:27:42.5793755Z ................................................................................i..ii............... 6400/9556
2020-01-26T17:28:05.7166283Z ........................................................................................FF.......... 6600/9556
2020-01-26T17:28:10.4940788Z ........................................................i........................................... 6700/9556
2020-01-26T17:28:12.5568726Z .................................................................................................... 6800/9556
2020-01-26T17:28:14.6075081Z .......................................................i............................................ 6900/9556
---
2020-01-26T17:29:43.0977895Z .................................................................................................... 7600/9556
2020-01-26T17:29:47.8307113Z .................................................................................................... 7700/9556
2020-01-26T17:29:53.7123313Z .................................................................................................... 7800/9556
2020-01-26T17:30:03.0735576Z .................................................................................................... 7900/9556
2020-01-26T17:30:08.6693640Z ...........iiiiiii.................................................................................. 8000/9556
2020-01-26T17:30:21.4778597Z .................................................................................................... 8200/9556
2020-01-26T17:30:30.9433242Z .................................................................................................... 8300/9556
2020-01-26T17:30:42.6826879Z .................................................................................................... 8400/9556
2020-01-26T17:30:48.4896259Z .................................................................................................... 8500/9556
---
2020-01-26T17:32:31.2683557Z - error[E0152]: found duplicate lang item `panic_impl`
2020-01-26T17:32:31.2683779Z + error[E0152]: found duplicate lang item `panic_impl`.
2020-01-26T17:32:31.2684159Z 2   --> $DIR/duplicate_entry_error.rs:10:1
2020-01-26T17:32:31.2684397Z 3    |
2020-01-26T17:32:31.2684778Z 4 LL | / fn panic_impl(info: &PanicInfo) -> ! {
2020-01-26T17:32:31.2685136Z 
2020-01-26T17:32:31.2685349Z The actual stderr differed from the expected stderr.
2020-01-26T17:32:31.2685809Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate_entry_error/duplicate_entry_error.stderr
2020-01-26T17:32:31.2685809Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate_entry_error/duplicate_entry_error.stderr
2020-01-26T17:32:31.2686250Z To update references, rerun the tests and pass the `--bless` flag
2020-01-26T17:32:31.2686780Z To only update this specific test, also pass `--test-args duplicate_entry_error.rs`
2020-01-26T17:32:31.2687288Z error: 1 errors occurred comparing output.
2020-01-26T17:32:31.2687452Z status: exit code: 1
2020-01-26T17:32:31.2687452Z status: exit code: 1
2020-01-26T17:32:31.2688272Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/duplicate_entry_error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate_entry_error" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate_entry_error/auxiliary" "-A" "unused"
2020-01-26T17:32:31.2689359Z ------------------------------------------
2020-01-26T17:32:31.2689527Z 
2020-01-26T17:32:31.2689851Z ------------------------------------------
2020-01-26T17:32:31.2690060Z stderr:
2020-01-26T17:32:31.2690060Z stderr:
2020-01-26T17:32:31.2690389Z ------------------------------------------
2020-01-26T17:32:31.2690575Z error[E0152]: found duplicate lang item `panic_impl`.
2020-01-26T17:32:31.2690936Z   --> /checkout/src/test/ui/duplicate_entry_error.rs:10:1
2020-01-26T17:32:31.2691118Z    |
2020-01-26T17:32:31.2691445Z LL | / fn panic_impl(info: &PanicInfo) -> ! {
2020-01-26T17:32:31.2691645Z LL | | //~^ ERROR: found duplicate lang item `panic_impl`
2020-01-26T17:32:31.2691989Z LL | | }
2020-01-26T17:32:31.2692150Z    | |_^
2020-01-26T17:32:31.2692308Z    |
2020-01-26T17:32:31.2692308Z    |
2020-01-26T17:32:31.2692492Z    = note: first defined in crate `std` (which `duplicate_entry_error` depends on)
2020-01-26T17:32:31.2692811Z error: aborting due to previous error
2020-01-26T17:32:31.2692958Z 
2020-01-26T17:32:31.2693319Z For more information about this error, try `rustc --explain E0152`.
2020-01-26T17:32:31.2693494Z 
---
2020-01-26T17:32:31.2697102Z 4 LL | struct Foo;
2020-01-26T17:32:31.2697288Z 
2020-01-26T17:32:31.2697476Z 
2020-01-26T17:32:31.2697682Z The actual stderr differed from the expected stderr.
2020-01-26T17:32:31.2698182Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0152/E0152.stderr
2020-01-26T17:32:31.2701962Z To update references, rerun the tests and pass the `--bless` flag
2020-01-26T17:32:31.2702649Z To only update this specific test, also pass `--test-args error-codes/E0152.rs`
2020-01-26T17:32:31.2703839Z error: 1 errors occurred comparing output.
2020-01-26T17:32:31.2704032Z status: exit code: 1
2020-01-26T17:32:31.2704032Z status: exit code: 1
2020-01-26T17:32:31.2705063Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0152.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0152" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0152/auxiliary" "-A" "unused"
2020-01-26T17:32:31.2705656Z ------------------------------------------
2020-01-26T17:32:31.2705807Z 
2020-01-26T17:32:31.2706179Z ------------------------------------------
2020-01-26T17:32:31.2706438Z stderr:
---
2020-01-26T17:32:31.2710940Z - error[E0152]: found duplicate lang item `panic_impl`
2020-01-26T17:32:31.2711117Z + error[E0152]: found duplicate lang item `panic_impl`.
2020-01-26T17:32:31.2711442Z 2   --> $DIR/panic-handler-duplicate.rs:15:1
2020-01-26T17:32:31.2711658Z 3    |
2020-01-26T17:32:31.2712169Z 4 LL | / fn panic2(info: &PanicInfo) -> ! {
2020-01-26T17:32:31.2712444Z 
2020-01-26T17:32:31.2712612Z The actual stderr differed from the expected stderr.
2020-01-26T17:32:31.2713140Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-duplicate/panic-handler-duplicate.stderr
2020-01-26T17:32:31.2713140Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-duplicate/panic-handler-duplicate.stderr
2020-01-26T17:32:31.2713552Z To update references, rerun the tests and pass the `--bless` flag
2020-01-26T17:32:31.2715050Z To only update this specific test, also pass `--test-args panic-handler/panic-handler-duplicate.rs`
2020-01-26T17:32:31.2715274Z error: 1 errors occurred comparing output.
2020-01-26T17:32:31.2715436Z status: exit code: 1
2020-01-26T17:32:31.2715436Z status: exit code: 1
2020-01-26T17:32:31.2716358Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-handler/panic-handler-duplicate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-duplicate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-duplicate/auxiliary" "-A" "unused"
2020-01-26T17:32:31.2716719Z ------------------------------------------
2020-01-26T17:32:31.2716754Z 
2020-01-26T17:32:31.2716980Z ------------------------------------------
2020-01-26T17:32:31.2717055Z stderr:
2020-01-26T17:32:31.2717055Z stderr:
2020-01-26T17:32:31.2717477Z ------------------------------------------
2020-01-26T17:32:31.2717534Z error[E0152]: found duplicate lang item `panic_impl`.
2020-01-26T17:32:31.2717827Z   --> /checkout/src/test/ui/panic-handler/panic-handler-duplicate.rs:15:1
2020-01-26T17:32:31.2717879Z    |
2020-01-26T17:32:31.2718156Z LL | / fn panic2(info: &PanicInfo) -> ! { //~ ERROR found duplicate lang item `panic_impl`
2020-01-26T17:32:31.2718272Z LL | | }
2020-01-26T17:32:31.2718315Z    | |_^
2020-01-26T17:32:31.2718356Z    |
2020-01-26T17:32:31.2718422Z note: first defined here
2020-01-26T17:32:31.2718422Z note: first defined here
2020-01-26T17:32:31.2718681Z   --> /checkout/src/test/ui/panic-handler/panic-handler-duplicate.rs:10:1
2020-01-26T17:32:31.2718730Z    |
2020-01-26T17:32:31.2718966Z LL | / fn panic(info: &PanicInfo) -> ! {
2020-01-26T17:32:31.2719058Z LL | | }
2020-01-26T17:32:31.2719115Z    | |_^
2020-01-26T17:32:31.2719143Z 
2020-01-26T17:32:31.2719269Z error: aborting due to previous error
---
2020-01-26T17:32:31.2720464Z - error[E0152]: found duplicate lang item `panic_impl`
2020-01-26T17:32:31.2720536Z + error[E0152]: found duplicate lang item `panic_impl`.
2020-01-26T17:32:31.2720759Z 2   --> $DIR/panic-handler-std.rs:7:1
2020-01-26T17:32:31.2720806Z 3    |
2020-01-26T17:32:31.2721046Z 4 LL | / fn panic(info: PanicInfo) -> ! {
2020-01-26T17:32:31.2721105Z 
2020-01-26T17:32:31.2721151Z The actual stderr differed from the expected stderr.
2020-01-26T17:32:31.2721496Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-std/panic-handler-std.stderr
2020-01-26T17:32:31.2721496Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-std/panic-handler-std.stderr
2020-01-26T17:32:31.2721762Z To update references, rerun the tests and pass the `--bless` flag
2020-01-26T17:32:31.2722040Z To only update this specific test, also pass `--test-args panic-handler/panic-handler-std.rs`
2020-01-26T17:32:31.2722145Z error: 1 errors occurred comparing output.
2020-01-26T17:32:31.2722192Z status: exit code: 1
2020-01-26T17:32:31.2722192Z status: exit code: 1
2020-01-26T17:32:31.2723042Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-handler/panic-handler-std.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-std" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-handler/panic-handler-std/auxiliary" "-A" "unused"
2020-01-26T17:32:31.2723390Z ------------------------------------------
2020-01-26T17:32:31.2723423Z 
2020-01-26T17:32:31.2723648Z ------------------------------------------
2020-01-26T17:32:31.2723712Z stderr:
2020-01-26T17:32:31.2723712Z stderr:
2020-01-26T17:32:31.2723932Z ------------------------------------------
2020-01-26T17:32:31.2723983Z error[E0152]: found duplicate lang item `panic_impl`.
2020-01-26T17:32:31.2724235Z   --> /checkout/src/test/ui/panic-handler/panic-handler-std.rs:7:1
2020-01-26T17:32:31.2724304Z    |
2020-01-26T17:32:31.2724525Z LL | / fn panic(info: PanicInfo) -> ! {
2020-01-26T17:32:31.2724635Z LL | | }
2020-01-26T17:32:31.2724677Z    | |_^
2020-01-26T17:32:31.2724718Z    |
2020-01-26T17:32:31.2724718Z    |
2020-01-26T17:32:31.2724785Z    = note: first defined in crate `std` (which `panic_handler_std` depends on)
2020-01-26T17:32:31.2724929Z error: argument should be `&PanicInfo`
2020-01-26T17:32:31.2725197Z   --> /checkout/src/test/ui/panic-handler/panic-handler-std.rs:7:16
2020-01-26T17:32:31.2725264Z    |
2020-01-26T17:32:31.2725264Z    |
2020-01-26T17:32:31.2725481Z LL | fn panic(info: PanicInfo) -> ! {
2020-01-26T17:32:31.2725577Z 
2020-01-26T17:32:31.2725623Z error: aborting due to 2 previous errors
2020-01-26T17:32:31.2725652Z 
2020-01-26T17:32:31.2725898Z For more information about this error, try `rustc --explain E0152`.
---
2020-01-26T17:32:31.2727761Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:349:22
2020-01-26T17:32:31.2727820Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-26T17:32:31.2784022Z 
2020-01-26T17:32:31.2784179Z 
2020-01-26T17:32:31.2786115Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-26T17:32:31.2786378Z 
2020-01-26T17:32:31.2786412Z 
2020-01-26T17:32:31.2786466Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-26T17:32:31.2786533Z Build completed unsuccessfully in 0:53:42
2020-01-26T17:32:31.2786533Z Build completed unsuccessfully in 0:53:42
2020-01-26T17:32:31.2789294Z == clock drift check ==
2020-01-26T17:32:31.2807714Z   local time: Sun Jan 26 17:32:31 UTC 2020
2020-01-26T17:32:31.5819330Z   network time: Sun, 26 Jan 2020 17:32:31 GMT
2020-01-26T17:32:31.5826297Z == end clock drift check ==
2020-01-26T17:32:32.0083032Z 
2020-01-26T17:32:32.0163048Z ##[error]Bash exited with code '1'.
2020-01-26T17:32:32.0174113Z ##[section]Finishing: Run build
2020-01-26T17:32:32.0212494Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68554/merge to s
2020-01-26T17:32:32.0214481Z Task         : Get sources
2020-01-26T17:32:32.0214533Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-26T17:32:32.0214583Z Version      : 1.0.0
2020-01-26T17:32:32.0214658Z Author       : Microsoft
2020-01-26T17:32:32.0214658Z Author       : Microsoft
2020-01-26T17:32:32.0214708Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-26T17:32:32.0214761Z ==============================================================================
2020-01-26T17:32:32.4238224Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-26T17:32:32.4281892Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68554/merge to s
2020-01-26T17:32:32.4387478Z Cleaning up task key
2020-01-26T17:32:32.4388151Z Start cleaning up orphan processes.
2020-01-26T17:32:32.4477148Z Terminate orphan process: pid (3506) (python)
2020-01-26T17:32:32.4675081Z ##[section]Finishing: Finalize Job
