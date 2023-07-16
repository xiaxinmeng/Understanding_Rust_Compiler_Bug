plain
2020-02-02T06:10:55.3153808Z ========================== Starting Command Output ===========================
2020-02-02T06:10:55.3155564Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/23952cc8-aa17-4a10-8644-d1ef450ebd3d.sh
2020-02-02T06:10:55.3155601Z 
2020-02-02T06:10:55.3158567Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-02T06:10:55.3165139Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68760/merge to s
2020-02-02T06:10:55.3166841Z Task         : Get sources
2020-02-02T06:10:55.3166866Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-02T06:10:55.3166892Z Version      : 1.0.0
2020-02-02T06:10:55.3166955Z Author       : Microsoft
---
2020-02-02T06:10:56.9375574Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-02T06:10:56.9388310Z ##[command]git config gc.auto 0
2020-02-02T06:10:56.9391427Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-02T06:10:56.9394174Z ##[command]git config --get-all http.proxy
2020-02-02T06:10:56.9400317Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68760/merge:refs/remotes/pull/68760/merge
---
2020-02-02T06:58:21.1505130Z .................................................................................................... 1700/9563
2020-02-02T06:58:25.1827212Z .................................................................................................... 1800/9563
2020-02-02T06:58:35.3791847Z .........................i.......................................................................... 1900/9563
2020-02-02T06:58:41.1727710Z .................................................................................................... 2000/9563
2020-02-02T06:58:52.9348201Z ...............iiiii................................................................................ 2100/9563
2020-02-02T06:59:01.1065150Z .................................................................................................... 2300/9563
2020-02-02T06:59:03.2226869Z .................................................................................................... 2400/9563
2020-02-02T06:59:07.5552594Z .................................................................................................... 2500/9563
2020-02-02T06:59:25.2401264Z .................................................................................................... 2600/9563
---
2020-02-02T07:01:31.6350739Z .................................................................................................... 4800/9563
2020-02-02T07:01:35.8229637Z ..........................................................i...............i......................... 4900/9563
2020-02-02T07:01:42.5792633Z .................................................................................................... 5000/9563
2020-02-02T07:01:49.2218191Z .................................................................................................... 5100/9563
2020-02-02T07:01:53.3493192Z .i.................................................................................................. 5200/9563
2020-02-02T07:02:02.7503938Z ...........................................................................ii.ii........i...i....... 5300/9563
2020-02-02T07:02:10.0448884Z .............i...................................................................................... 5500/9563
2020-02-02T07:02:18.5133635Z .................................................................................................... 5600/9563
2020-02-02T07:02:24.1856155Z ..............................................................i..................................... 5700/9563
2020-02-02T07:02:30.4220660Z .................................................................................................... 5800/9563
2020-02-02T07:02:30.4220660Z .................................................................................................... 5800/9563
2020-02-02T07:02:37.3931562Z .................................................................................................... 5900/9563
2020-02-02T07:02:45.0950815Z .....................................................ii...i..ii...........i......................... 6000/9563
2020-02-02T07:03:03.7432615Z .................................................................................................... 6200/9563
2020-02-02T07:03:09.7097416Z .................................................................................................... 6300/9563
2020-02-02T07:03:09.7097416Z .................................................................................................... 6300/9563
2020-02-02T07:03:16.6160303Z .................................................................................i..ii.............. 6400/9563
2020-02-02T07:03:42.7571741Z .................................................................................................... 6600/9563
2020-02-02T07:03:47.5579868Z .........................................................i.......................................... 6700/9563
2020-02-02T07:03:49.4827347Z .................................................................................................... 6800/9563
2020-02-02T07:03:51.3950299Z ..........................................................i......................................... 6900/9563
---
2020-02-02T07:05:17.6666464Z .................................................................................................... 7600/9563
2020-02-02T07:05:21.8700552Z .................................................................................................... 7700/9563
2020-02-02T07:05:27.2305583Z .................................................................................................... 7800/9563
2020-02-02T07:05:36.0731974Z .................................................................................................... 7900/9563
2020-02-02T07:05:40.9550324Z .................iiiiiiii........................................................................... 8000/9563
2020-02-02T07:05:52.8835621Z .................................................................................................... 8200/9563
2020-02-02T07:06:01.0185688Z .................................................................................................... 8300/9563
2020-02-02T07:06:12.5570147Z .................................................................................................... 8400/9563
2020-02-02T07:06:18.3742760Z .................................................................................................... 8500/9563
---
2020-02-02T07:07:56.0235037Z 39 error[E0502]: cannot borrow `a` as immutable because it is also borrowed as mutable
2020-02-02T07:07:56.0235401Z -   --> $DIR/two-phase-nonrecv-autoref.rs:120:27
2020-02-02T07:07:56.0235746Z +   --> $DIR/two-phase-nonrecv-autoref.rs:112:27
2020-02-02T07:07:56.0235925Z 41    |
2020-02-02T07:07:56.0236062Z 42 LL |     double_access(&mut a, &a);
2020-02-02T07:07:56.0236391Z 43    |     ------------- ------  ^^ immutable borrow occurs here
2020-02-02T07:07:56.0236715Z 46    |     mutable borrow later used by call
2020-02-02T07:07:56.0236848Z 47 
2020-02-02T07:07:56.0237005Z 48 error[E0502]: cannot borrow `i` as immutable because it is also borrowed as mutable
2020-02-02T07:07:56.0237334Z -   --> $DIR/two-phase-nonrecv-autoref.rs:146:7
2020-02-02T07:07:56.0237334Z -   --> $DIR/two-phase-nonrecv-autoref.rs:146:7
2020-02-02T07:07:56.0237666Z +   --> $DIR/two-phase-nonrecv-autoref.rs:138:7
2020-02-02T07:07:56.0237838Z 50    |
2020-02-02T07:07:56.0237970Z 51 LL |     i[i[3]] = 4;
2020-02-02T07:07:56.0238250Z 52    |     --^----
2020-02-02T07:07:56.0238543Z 56    |     mutable borrow later used here
2020-02-02T07:07:56.0238672Z 57 
2020-02-02T07:07:56.0238823Z 58 error[E0502]: cannot borrow `i` as immutable because it is also borrowed as mutable
2020-02-02T07:07:56.0239131Z -   --> $DIR/two-phase-nonrecv-autoref.rs:151:7
2020-02-02T07:07:56.0239131Z -   --> $DIR/two-phase-nonrecv-autoref.rs:151:7
2020-02-02T07:07:56.0239457Z +   --> $DIR/two-phase-nonrecv-autoref.rs:143:7
2020-02-02T07:07:56.0239633Z 60    |
2020-02-02T07:07:56.0239763Z 61 LL |     i[i[3]] = i[4];
2020-02-02T07:07:56.0240061Z 62    |     --^----
2020-02-02T07:07:56.0240338Z 
2020-02-02T07:07:56.0240472Z The actual stderr differed from the expected stderr.
2020-02-02T07:07:56.0240472Z The actual stderr differed from the expected stderr.
2020-02-02T07:07:56.0241035Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-nonrecv-autoref.nll/two-phase-nonrecv-autoref.nll.stderr
2020-02-02T07:07:56.0241466Z To update references, rerun the tests and pass the `--bless` flag
2020-02-02T07:07:56.0241894Z To only update this specific test, also pass `--test-args borrowck/two-phase-nonrecv-autoref.rs`
2020-02-02T07:07:56.0242070Z 
2020-02-02T07:07:56.0242215Z error in revision `nll`: 1 errors occurred comparing output.
2020-02-02T07:07:56.0242355Z status: exit code: 1
2020-02-02T07:07:56.0243253Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-nonrecv-autoref.nll" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-nonrecv-autoref.nll/auxiliary" "-A" "unused"
2020-02-02T07:07:56.0244006Z ------------------------------------------
2020-02-02T07:07:56.0244160Z 
2020-02-02T07:07:56.0244509Z ------------------------------------------
2020-02-02T07:07:56.0244674Z stderr:
2020-02-02T07:07:56.0244674Z stderr:
2020-02-02T07:07:56.0245015Z ------------------------------------------
2020-02-02T07:07:56.0245204Z error[E0499]: cannot borrow `*f` as mutable more than once at a time
2020-02-02T07:07:56.0245746Z    |
2020-02-02T07:07:56.0245899Z LL |         f(f(10));
2020-02-02T07:07:56.0245899Z LL |         f(f(10));
2020-02-02T07:07:56.0246233Z    |         - ^ second mutable borrow occurs here
2020-02-02T07:07:56.0246565Z    |         first mutable borrow occurs here
2020-02-02T07:07:56.0246716Z    |         first borrow later used by call
2020-02-02T07:07:56.0246867Z 
2020-02-02T07:07:56.0247008Z error[E0382]: use of moved value: `f`
2020-02-02T07:07:56.0247008Z error[E0382]: use of moved value: `f`
2020-02-02T07:07:56.0247368Z   --> /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:59:11
2020-02-02T07:07:56.0247553Z    |
2020-02-02T07:07:56.0247887Z LL |     fn twice_ten_so<F: FnOnce(i32) -> i32>(f: Box<F>) {
2020-02-02T07:07:56.0248931Z    |                                            - move occurs because `f` has type `std::boxed::Box<F>`, which does not implement the `Copy` trait
2020-02-02T07:07:56.0249181Z LL |         f(f(10));
2020-02-02T07:07:56.0249526Z    |         - ^ value used here after move
2020-02-02T07:07:56.0249852Z    |         value moved here
2020-02-02T07:07:56.0249990Z 
2020-02-02T07:07:56.0249990Z 
2020-02-02T07:07:56.0250131Z error[E0499]: cannot borrow `*f` as mutable more than once at a time
2020-02-02T07:07:56.0250688Z    |
2020-02-02T07:07:56.0250842Z LL |         f(f(10));
2020-02-02T07:07:56.0250842Z LL |         f(f(10));
2020-02-02T07:07:56.0251174Z    |         - ^ second mutable borrow occurs here
2020-02-02T07:07:56.0251492Z    |         first mutable borrow occurs here
2020-02-02T07:07:56.0251633Z    |         first borrow later used by call
2020-02-02T07:07:56.0251755Z 
2020-02-02T07:07:56.0251910Z error[E0382]: use of moved value: `f`
2020-02-02T07:07:56.0251910Z error[E0382]: use of moved value: `f`
2020-02-02T07:07:56.0252263Z   --> /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:73:11
2020-02-02T07:07:56.0252426Z    |
2020-02-02T07:07:56.0252781Z LL |     fn twice_ten_oo(f: Box<dyn FnOnce(i32) -> i32>) {
2020-02-02T07:07:56.0253217Z    |                     - move occurs because `f` has type `std::boxed::Box<dyn std::ops::FnOnce(i32) -> i32>`, which does not implement the `Copy` trait
2020-02-02T07:07:56.0253403Z LL |         f(f(10));
2020-02-02T07:07:56.0253752Z    |         - ^ value used here after move
2020-02-02T07:07:56.0254205Z    |         value moved here
2020-02-02T07:07:56.0254349Z 
2020-02-02T07:07:56.0254490Z error[E0502]: cannot borrow `a` as immutable because it is also borrowed as mutable
2020-02-02T07:07:56.0254874Z   --> /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:112:27
2020-02-02T07:07:56.0254874Z   --> /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:112:27
2020-02-02T07:07:56.0255063Z    |
2020-02-02T07:07:56.0255197Z LL |     double_access(&mut a, &a);
2020-02-02T07:07:56.0255550Z    |     ------------- ------  ^^ immutable borrow occurs here
2020-02-02T07:07:56.0255944Z    |     |             mutable borrow occurs here
2020-02-02T07:07:56.0256099Z    |     mutable borrow later used by call
2020-02-02T07:07:56.0256222Z 
2020-02-02T07:07:56.0256363Z error[E0502]: cannot borrow `i` as immutable because it is also borrowed as mutable
2020-02-02T07:07:56.0256363Z error[E0502]: cannot borrow `i` as immutable because it is also borrowed as mutable
2020-02-02T07:07:56.0256900Z   --> /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:138:7
2020-02-02T07:07:56.0257074Z    |
2020-02-02T07:07:56.0257223Z LL |     i[i[3]] = 4;
2020-02-02T07:07:56.0257547Z    |     --^----
2020-02-02T07:07:56.0257855Z    |     | immutable borrow occurs here
2020-02-02T07:07:56.0258009Z    |     mutable borrow occurs here
2020-02-02T07:07:56.0258148Z    |     mutable borrow later used here
2020-02-02T07:07:56.0258267Z 
2020-02-02T07:07:56.0258267Z 
2020-02-02T07:07:56.0258423Z error[E0502]: cannot borrow `i` as immutable because it is also borrowed as mutable
2020-02-02T07:07:56.0258776Z   --> /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:143:7
2020-02-02T07:07:56.0258938Z    |
2020-02-02T07:07:56.0259090Z LL |     i[i[3]] = i[4];
2020-02-02T07:07:56.0259385Z    |     --^----
2020-02-02T07:07:56.0259700Z    |     | immutable borrow occurs here
2020-02-02T07:07:56.0259855Z    |     mutable borrow occurs here
2020-02-02T07:07:56.0259991Z    |     mutable borrow later used here
2020-02-02T07:07:56.0260126Z 
---
2020-02-02T07:07:56.0266347Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-02T07:07:56.0266427Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-02T07:07:56.0273243Z 
2020-02-02T07:07:56.0273337Z 
2020-02-02T07:07:56.0274905Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-02T07:07:56.0275142Z 
2020-02-02T07:07:56.0275167Z 
2020-02-02T07:07:56.0281314Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-02T07:07:56.0281455Z Build completed unsuccessfully in 0:52:17
2020-02-02T07:07:56.0281455Z Build completed unsuccessfully in 0:52:17
2020-02-02T07:07:56.0328777Z == clock drift check ==
2020-02-02T07:07:56.0346086Z   local time: Sun Feb  2 07:07:56 UTC 2020
2020-02-02T07:07:56.3044743Z   network time: Sun, 02 Feb 2020 07:07:56 GMT
2020-02-02T07:07:56.3045550Z == end clock drift check ==
2020-02-02T07:07:56.7413267Z 
2020-02-02T07:07:56.7512130Z ##[error]Bash exited with code '1'.
2020-02-02T07:07:56.7581558Z ##[section]Finishing: Run build
2020-02-02T07:07:56.7614053Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68760/merge to s
2020-02-02T07:07:56.7616694Z Task         : Get sources
2020-02-02T07:07:56.7616735Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-02T07:07:56.7616995Z Version      : 1.0.0
2020-02-02T07:07:56.7617031Z Author       : Microsoft
2020-02-02T07:07:56.7617031Z Author       : Microsoft
2020-02-02T07:07:56.7617071Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-02T07:07:56.7617131Z ==============================================================================
2020-02-02T07:07:57.1200112Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-02T07:07:57.1232169Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68760/merge to s
2020-02-02T07:07:57.1326408Z Cleaning up task key
2020-02-02T07:07:57.1327353Z Start cleaning up orphan processes.
2020-02-02T07:07:57.1604480Z Terminate orphan process: pid (6104) (python)
2020-02-02T07:07:57.1628294Z ##[section]Finishing: Finalize Job
