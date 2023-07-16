plain
2020-01-20T07:31:55.6441436Z ========================== Starting Command Output ===========================
2020-01-20T07:31:55.6456758Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d6b875e8-8d7c-4054-bfc4-86b4fde8b7df.sh
2020-01-20T07:31:55.6662239Z 
2020-01-20T07:31:55.6713980Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-20T07:31:55.6761422Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68241/merge to s
2020-01-20T07:31:55.6762644Z Task         : Get sources
2020-01-20T07:31:55.6762669Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-20T07:31:55.6762723Z Version      : 1.0.0
2020-01-20T07:31:55.6762747Z Author       : Microsoft
---
2020-01-20T07:31:56.4334048Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-20T07:31:56.4427253Z ##[command]git config gc.auto 0
2020-01-20T07:31:56.5556178Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-20T07:31:56.5604893Z ##[command]git config --get-all http.proxy
2020-01-20T07:31:56.5729447Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68241/merge:refs/remotes/pull/68241/merge
---
2020-01-20T08:21:12.7809611Z .................................................................................................... 1700/9539
2020-01-20T08:21:18.6717071Z .................................................................................................... 1800/9539
2020-01-20T08:21:29.8032476Z ...................i................................................................................ 1900/9539
2020-01-20T08:21:36.5002586Z .................................................................................................... 2000/9539
2020-01-20T08:21:50.5734950Z .........iiiii...................................................................................... 2100/9539
2020-01-20T08:21:59.5396446Z .................................................................................................... 2300/9539
2020-01-20T08:22:01.8286329Z .................................................................................................... 2400/9539
2020-01-20T08:22:07.0435346Z .................................................................................................... 2500/9539
2020-01-20T08:22:26.3714647Z .................................................................................................... 2600/9539
---
2020-01-20T08:24:55.5896925Z .....................................................i...............i.............................. 4900/9539
2020-01-20T08:25:02.9916770Z .................................................................................................... 5000/9539
2020-01-20T08:25:10.2883100Z ................................................................................................i... 5100/9539
2020-01-20T08:25:15.0405417Z .................................................................................................... 5200/9539
2020-01-20T08:25:24.6059730Z ....................................................................ii.ii...........i............... 5300/9539
2020-01-20T08:25:32.7304299Z .....i.............................................................................................. 5500/9539
2020-01-20T08:25:41.8432411Z .................................................................................................... 5600/9539
2020-01-20T08:25:47.8023951Z ......................................................i............................................. 5700/9539
2020-01-20T08:25:54.0697338Z .................................................................................................... 5800/9539
2020-01-20T08:25:54.0697338Z .................................................................................................... 5800/9539
2020-01-20T08:26:03.0032172Z ....................................................i............................................... 5900/9539
2020-01-20T08:26:09.4439765Z .............................................ii...i..ii...........i................................. 6000/9539
2020-01-20T08:26:29.4303333Z .................................................................................................... 6200/9539
2020-01-20T08:26:36.8667669Z .......................................FF........................................................... 6300/9539
2020-01-20T08:26:36.8667669Z .......................................FF........................................................... 6300/9539
2020-01-20T08:26:45.7703650Z .........................................................................i..ii...................... 6400/9539
2020-01-20T08:27:13.9134870Z .................................................................................................... 6600/9539
2020-01-20T08:27:16.8920484Z .................................................i.................................................. 6700/9539
2020-01-20T08:27:18.7337360Z .................................................................................................... 6800/9539
2020-01-20T08:27:20.7737806Z ................................................i................................................... 6900/9539
---
2020-01-20T08:28:51.6309618Z .................................................................................................... 7600/9539
2020-01-20T08:28:56.6293059Z .................................................................................................... 7700/9539
2020-01-20T08:29:02.3362057Z .................................................................................................... 7800/9539
2020-01-20T08:29:12.2137082Z .................................................................................................... 7900/9539
2020-01-20T08:29:17.6611037Z iiiiiii............................................................................................. 8000/9539
2020-01-20T08:29:31.0165756Z .................................................................................................... 8200/9539
2020-01-20T08:29:41.2735952Z .................................................................................................... 8300/9539
2020-01-20T08:29:52.3922332Z .................................................................................................... 8400/9539
2020-01-20T08:29:57.4517644Z .................................................................................................... 8500/9539
---
2020-01-20T08:31:40.4469349Z ---- [ui] ui/nll/polonius/issue-46589.rs stdout ----
2020-01-20T08:31:40.4469527Z 
2020-01-20T08:31:40.4469900Z error: test compilation failed although it shouldn't!
2020-01-20T08:31:40.4470062Z status: exit code: 1
2020-01-20T08:31:40.4470952Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/polonius/issue-46589.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius/issue-46589" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-Z" "polonius" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius/issue-46589/auxiliary" "-A" "unused"
2020-01-20T08:31:40.4471547Z ------------------------------------------
2020-01-20T08:31:40.4471685Z 
2020-01-20T08:31:40.4471991Z ------------------------------------------
2020-01-20T08:31:40.4472177Z stderr:
2020-01-20T08:31:40.4472177Z stderr:
2020-01-20T08:31:40.4472475Z ------------------------------------------
2020-01-20T08:31:40.4472626Z error[E0499]: cannot borrow `**other` as mutable more than once at a time
2020-01-20T08:31:40.4472989Z   --> /checkout/src/test/ui/nll/polonius/issue-46589.rs:25:21
2020-01-20T08:31:40.4473144Z    |
2020-01-20T08:31:40.4473256Z LL |         *other = match (*other).get_self() {
2020-01-20T08:31:40.4473601Z    |                        -------- first mutable borrow occurs here
2020-01-20T08:31:40.4473755Z LL |             Some(s) => s,
2020-01-20T08:31:40.4473886Z LL |             None => (*other).new_self()
2020-01-20T08:31:40.4474110Z    |                     |
2020-01-20T08:31:40.4474236Z    |                     second mutable borrow occurs here
2020-01-20T08:31:40.4474513Z    |                     first borrow later used here
2020-01-20T08:31:40.4474655Z 
---
2020-01-20T08:31:40.4477897Z 28 
2020-01-20T08:31:40.4478038Z + error[E0506]: cannot assign to `*x` because it is borrowed
2020-01-20T08:31:40.4478360Z +   --> $DIR/polonius-smoke-test.rs:29:9
2020-01-20T08:31:40.4478512Z +    |
2020-01-20T08:31:40.4478892Z + LL | pub fn position_dependent_outlives(x: &mut i32, cond: bool) -> &mut i32 {
2020-01-20T08:31:40.4479474Z + LL |     let y = &mut *x;
2020-01-20T08:31:40.4479474Z + LL |     let y = &mut *x;
2020-01-20T08:31:40.4480005Z +    |             ------- borrow of `*x` occurs here
2020-01-20T08:31:40.4480168Z + LL |     if cond {
2020-01-20T08:31:40.4480278Z + LL |         return y;
2020-01-20T08:31:40.4480630Z +    |                - returning this value requires that `*x` is borrowed for `'1`
2020-01-20T08:31:40.4480781Z + LL |     } else {
2020-01-20T08:31:40.4480890Z + LL |         *x = 0;
2020-01-20T08:31:40.4481023Z +    |         ^^^^^^ assignment to borrowed `*x` occurs here
2020-01-20T08:31:40.4481254Z 29 error[E0505]: cannot move out of `s` because it is borrowed
2020-01-20T08:31:40.4481560Z 30   --> $DIR/polonius-smoke-test.rs:43:5
2020-01-20T08:31:40.4481705Z 31    |
2020-01-20T08:31:40.4481799Z 
---
2020-01-20T08:31:40.4484347Z 44 
2020-01-20T08:31:40.4484370Z 
2020-01-20T08:31:40.4484390Z 
2020-01-20T08:31:40.4484443Z The actual stderr differed from the expected stderr.
2020-01-20T08:31:40.4484721Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius/polonius-smoke-test/polonius-smoke-test.stderr
2020-01-20T08:31:40.4484949Z To update references, rerun the tests and pass the `--bless` flag
2020-01-20T08:31:40.4485221Z To only update this specific test, also pass `--test-args nll/polonius/polonius-smoke-test.rs`
2020-01-20T08:31:40.4485287Z error: 1 errors occurred comparing output.
2020-01-20T08:31:40.4485340Z status: exit code: 1
2020-01-20T08:31:40.4485340Z status: exit code: 1
2020-01-20T08:31:40.4486051Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/polonius/polonius-smoke-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius/polonius-smoke-test" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-Zpolonius" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius/polonius-smoke-test/auxiliary" "-A" "unused"
2020-01-20T08:31:40.4486491Z ------------------------------------------
2020-01-20T08:31:40.4486519Z 
2020-01-20T08:31:40.4486733Z ------------------------------------------
2020-01-20T08:31:40.4486771Z stderr:
2020-01-20T08:31:40.4486771Z stderr:
2020-01-20T08:31:40.4486958Z ------------------------------------------
2020-01-20T08:31:40.4487017Z error[E0515]: cannot return reference to local variable `x`
2020-01-20T08:31:40.4487335Z   --> /checkout/src/test/ui/nll/polonius/polonius-smoke-test.rs:7:5
2020-01-20T08:31:40.4487380Z    |
2020-01-20T08:31:40.4487431Z LL |     &x //~ ERROR
2020-01-20T08:31:40.4487470Z    |     ^^ returns a reference to data owned by the current function
2020-01-20T08:31:40.4487531Z error[E0503]: cannot use `x` because it was mutably borrowed
2020-01-20T08:31:40.4487769Z   --> /checkout/src/test/ui/nll/polonius/polonius-smoke-test.rs:13:13
2020-01-20T08:31:40.4487808Z    |
2020-01-20T08:31:40.4487842Z LL |     let y = &mut x;
2020-01-20T08:31:40.4487842Z LL |     let y = &mut x;
2020-01-20T08:31:40.4488063Z    |             ------ borrow of `x` occurs here
2020-01-20T08:31:40.4488105Z LL |     let z = x; //~ ERROR
2020-01-20T08:31:40.4488140Z    |             ^ use of borrowed `x`
2020-01-20T08:31:40.4488191Z LL |     let w = y;
2020-01-20T08:31:40.4488385Z    |             - borrow later used here
2020-01-20T08:31:40.4488449Z error[E0505]: cannot move out of `x` because it is borrowed
2020-01-20T08:31:40.4488697Z   --> /checkout/src/test/ui/nll/polonius/polonius-smoke-test.rs:19:13
2020-01-20T08:31:40.4488736Z    |
2020-01-20T08:31:40.4488736Z    |
2020-01-20T08:31:40.4488938Z LL | pub fn use_while_mut_fr(x: &mut i32) -> &mut i32 {
2020-01-20T08:31:40.4489222Z LL |     let y = &mut *x;
2020-01-20T08:31:40.4489222Z LL |     let y = &mut *x;
2020-01-20T08:31:40.4489425Z    |             ------- borrow of `*x` occurs here
2020-01-20T08:31:40.4489482Z LL |     let z = x; //~ ERROR
2020-01-20T08:31:40.4489520Z    |             ^ move out of `x` occurs here
2020-01-20T08:31:40.4489794Z    |     - returning this value requires that `*x` is borrowed for `'1`
2020-01-20T08:31:40.4489823Z 
2020-01-20T08:31:40.4489861Z error[E0506]: cannot assign to `*x` because it is borrowed
2020-01-20T08:31:40.4490077Z   --> /checkout/src/test/ui/nll/polonius/polonius-smoke-test.rs:29:9
2020-01-20T08:31:40.4490077Z   --> /checkout/src/test/ui/nll/polonius/polonius-smoke-test.rs:29:9
2020-01-20T08:31:40.4490133Z    |
2020-01-20T08:31:40.4490363Z LL | pub fn position_dependent_outlives(x: &mut i32, cond: bool) -> &mut i32 {
2020-01-20T08:31:40.4490596Z    |                                       - let's call the lifetime of this reference `'1`
2020-01-20T08:31:40.4490658Z LL |     let y = &mut *x;
2020-01-20T08:31:40.4490883Z    |             ------- borrow of `*x` occurs here
2020-01-20T08:31:40.4490977Z LL |         return y;
2020-01-20T08:31:40.4491219Z    |                - returning this value requires that `*x` is borrowed for `'1`
2020-01-20T08:31:40.4491263Z LL |     } else {
2020-01-20T08:31:40.4491308Z LL |         *x = 0;
2020-01-20T08:31:40.4491308Z LL |         *x = 0;
2020-01-20T08:31:40.4491365Z    |         ^^^^^^ assignment to borrowed `*x` occurs here
2020-01-20T08:31:40.4491429Z error[E0505]: cannot move out of `s` because it is borrowed
2020-01-20T08:31:40.4491855Z   --> /checkout/src/test/ui/nll/polonius/polonius-smoke-test.rs:43:5
2020-01-20T08:31:40.4491896Z    |
2020-01-20T08:31:40.4491937Z LL |     let r = &mut *s;
2020-01-20T08:31:40.4491937Z LL |     let r = &mut *s;
2020-01-20T08:31:40.4492142Z    |             ------- borrow of `*s` occurs here
2020-01-20T08:31:40.4492201Z LL |     let tmp = foo(&r);
2020-01-20T08:31:40.4492236Z LL |     s; //~ ERROR
2020-01-20T08:31:40.4492272Z    |     ^ move out of `s` occurs here
2020-01-20T08:31:40.4492517Z    |     --- borrow later used here
2020-01-20T08:31:40.4492544Z 
2020-01-20T08:31:40.4492581Z error: aborting due to 5 previous errors
2020-01-20T08:31:40.4492623Z 
---
2020-01-20T08:31:40.4499175Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-20T08:31:40.4499259Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-20T08:31:40.4499287Z 
2020-01-20T08:31:40.4499308Z 
2020-01-20T08:31:40.4500671Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-20T08:31:40.4500881Z 
2020-01-20T08:31:40.4500905Z 
2020-01-20T08:31:40.4549885Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-20T08:31:40.4550032Z Build completed unsuccessfully in 0:54:26
2020-01-20T08:31:40.4550032Z Build completed unsuccessfully in 0:54:26
2020-01-20T08:31:40.4553801Z == clock drift check ==
2020-01-20T08:31:40.4561176Z   local time: Mon Jan 20 08:31:40 UTC 2020
2020-01-20T08:31:40.4980895Z   network time: Mon, 20 Jan 2020 08:31:40 GMT
2020-01-20T08:31:40.4982980Z == end clock drift check ==
2020-01-20T08:31:40.8617999Z 
2020-01-20T08:31:40.8755042Z ##[error]Bash exited with code '1'.
2020-01-20T08:31:40.8766322Z ##[section]Finishing: Run build
2020-01-20T08:31:40.8791812Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68241/merge to s
2020-01-20T08:31:40.8794171Z Task         : Get sources
2020-01-20T08:31:40.8794212Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-20T08:31:40.8794264Z Version      : 1.0.0
2020-01-20T08:31:40.8794316Z Author       : Microsoft
2020-01-20T08:31:40.8794316Z Author       : Microsoft
2020-01-20T08:31:40.8794359Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-20T08:31:40.8794400Z ==============================================================================
2020-01-20T08:31:41.2560238Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-20T08:31:41.2595354Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68241/merge to s
2020-01-20T08:31:41.2705899Z Cleaning up task key
2020-01-20T08:31:41.2706538Z Start cleaning up orphan processes.
2020-01-20T08:31:41.2791855Z Terminate orphan process: pid (3617) (python)
2020-01-20T08:31:41.3055310Z ##[section]Finishing: Finalize Job
