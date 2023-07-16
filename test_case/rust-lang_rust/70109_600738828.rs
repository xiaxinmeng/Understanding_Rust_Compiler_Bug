plain
2020-03-18T15:32:54.7099462Z ========================== Starting Command Output ===========================
2020-03-18T15:32:54.7102452Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/18f48862-fa41-4893-af69-6a537efb1a5d.sh
2020-03-18T15:32:54.7102710Z 
2020-03-18T15:32:54.7107989Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-18T15:32:54.7128609Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70109/merge to s
2020-03-18T15:32:54.7151290Z Task         : Get sources
2020-03-18T15:32:54.7151670Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T15:32:54.7152014Z Version      : 1.0.0
2020-03-18T15:32:54.7152207Z Author       : Microsoft
---
2020-03-18T15:32:55.7038669Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-18T15:32:55.7043116Z ##[command]git config gc.auto 0
2020-03-18T15:32:55.7046859Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-18T15:32:55.7050590Z ##[command]git config --get-all http.proxy
2020-03-18T15:32:55.7058167Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70109/merge:refs/remotes/pull/70109/merge
---
2020-03-18T16:30:24.6242816Z .................................................................................................... 1700/9799
2020-03-18T16:30:28.6645618Z .................................................................................................... 1800/9799
2020-03-18T16:30:39.5495479Z ..........................................................................i......................... 1900/9799
2020-03-18T16:30:45.7102200Z .................................................................................................... 2000/9799
2020-03-18T16:30:53.5895826Z ................................................................iiiii............................... 2100/9799
2020-03-18T16:31:11.1656351Z .................................................................................................... 2300/9799
2020-03-18T16:31:13.3478223Z .................................................................................................... 2400/9799
2020-03-18T16:31:16.1888341Z .................................................................................................... 2500/9799
2020-03-18T16:31:35.7088337Z .................................................................................................... 2600/9799
---
2020-03-18T16:34:13.4497184Z .....................................i...............i.............................................. 5000/9799
2020-03-18T16:34:22.5402596Z .................................................................................................... 5100/9799
2020-03-18T16:34:28.6691001Z ................................................................................i................... 5200/9799
2020-03-18T16:34:33.9962363Z .................................................................................................... 5300/9799
2020-03-18T16:34:43.6446058Z .............................................................ii.ii........i...i..................... 5400/9799
2020-03-18T16:34:51.2831521Z i................................................................................................... 5600/9799
2020-03-18T16:35:00.7378381Z .................................................................................................... 5700/9799
2020-03-18T16:35:06.8643817Z .......................................................i............................................ 5800/9799
2020-03-18T16:35:13.2021225Z ......................................................F............................................. 5900/9799
2020-03-18T16:35:13.2021225Z ......................................................F............................................. 5900/9799
2020-03-18T16:35:21.2260477Z .................................................................................................... 6000/9799
2020-03-18T16:35:28.4839523Z .................................................ii...i..ii...........i............................. 6100/9799
2020-03-18T16:35:47.8309151Z .................................................................................................... 6300/9799
2020-03-18T16:35:54.6949356Z .................................................................................................... 6400/9799
2020-03-18T16:35:54.6949356Z .................................................................................................... 6400/9799
2020-03-18T16:36:02.7455551Z ...............................................................................i..ii................ 6500/9799
2020-03-18T16:36:26.2715974Z .................................................................................................... 6700/9799
2020-03-18T16:36:35.0567661Z .............................................................................i...................... 6800/9799
2020-03-18T16:36:36.9780796Z .................................................................................................... 6900/9799
2020-03-18T16:36:38.9259732Z .................................................................................................... 7000/9799
---
2020-03-18T16:38:19.3639279Z .................................................................................................... 7800/9799
2020-03-18T16:38:24.6390082Z .................................................................................................... 7900/9799
2020-03-18T16:38:30.4308943Z ................................................................i................................... 8000/9799
2020-03-18T16:38:40.4378804Z .................................................................................................... 8100/9799
2020-03-18T16:38:45.7531088Z .............iiiiiiiiii.i........................................................................... 8200/9799
2020-03-18T16:38:58.8975878Z .................................................................................................... 8400/9799
2020-03-18T16:39:06.8639182Z .................................................................................................... 8500/9799
2020-03-18T16:39:20.2623988Z .................................................................................................... 8600/9799
2020-03-18T16:39:26.3730635Z .................................................................................................... 8700/9799
---
2020-03-18T16:41:17.3853825Z + 
2020-03-18T16:41:17.3853954Z + note: trace_macro
2020-03-18T16:41:17.3854305Z +   --> $DIR/trace_faulty_macros.rs:38:13
2020-03-18T16:41:17.3854501Z +    |
2020-03-18T16:41:17.3854656Z + LL |     let a = pat_macro!();
2020-03-18T16:41:17.3855016Z +    |
2020-03-18T16:41:17.3855016Z +    |
2020-03-18T16:41:17.3855186Z +    = note: expanding `pat_macro! {  }`
2020-03-18T16:41:17.3855464Z +    = note: to `pat_macro ! (A { a : a, b : 0, c : _, .. }) ;`
2020-03-18T16:41:17.3855798Z +    = note: expanding `pat_macro! { A { a : a, b : 0, c : _, .. } }`
2020-03-18T16:41:17.3856123Z +    = note: to `A { a: a, b: 0, c: _, .. }`
2020-03-18T16:41:17.3856465Z + error: aborting due to 4 previous errors
2020-03-18T16:41:17.3856651Z 64 
2020-03-18T16:41:17.3856754Z 65 
2020-03-18T16:41:17.3856840Z 
2020-03-18T16:41:17.3856840Z 
2020-03-18T16:41:17.3856920Z 
2020-03-18T16:41:17.3857111Z The actual stderr differed from the expected stderr.
2020-03-18T16:41:17.3858071Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace_faulty_macros/trace_faulty_macros.stderr
2020-03-18T16:41:17.3858870Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T16:41:17.3859476Z To only update this specific test, also pass `--test-args macros/trace_faulty_macros.rs`
2020-03-18T16:41:17.3859912Z error: 1 errors occurred comparing output.
2020-03-18T16:41:17.3860162Z status: exit code: 1
2020-03-18T16:41:17.3860162Z status: exit code: 1
2020-03-18T16:41:17.3862582Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/trace_faulty_macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace_faulty_macros" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Z" "trace-macros" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace_faulty_macros/auxiliary"
2020-03-18T16:41:17.3864090Z ------------------------------------------
2020-03-18T16:41:17.3864238Z 
2020-03-18T16:41:17.3864562Z ------------------------------------------
2020-03-18T16:41:17.3864731Z stderr:
2020-03-18T16:41:17.3864731Z stderr:
2020-03-18T16:41:17.3865037Z ------------------------------------------
2020-03-18T16:41:17.3865277Z error: no rules expected the token `bcd`
2020-03-18T16:41:17.3865700Z   --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:7:26
2020-03-18T16:41:17.3865912Z    |
2020-03-18T16:41:17.3866083Z LL | macro_rules! my_faulty_macro {
2020-03-18T16:41:17.3866666Z    | ---------------------------- when calling this macro
2020-03-18T16:41:17.3866885Z LL |     () => {
2020-03-18T16:41:17.3867110Z LL |         my_faulty_macro!(bcd); //~ ERROR no rules
2020-03-18T16:41:17.3870130Z ...
2020-03-18T16:41:17.3870130Z ...
2020-03-18T16:41:17.3870307Z LL |     my_faulty_macro!();
2020-03-18T16:41:17.3871151Z    |
2020-03-18T16:41:17.3871780Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-03-18T16:41:17.3872056Z 
2020-03-18T16:41:17.3872185Z note: trace_macro
2020-03-18T16:41:17.3872185Z note: trace_macro
2020-03-18T16:41:17.3872583Z   --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:33:5
2020-03-18T16:41:17.3872813Z    |
2020-03-18T16:41:17.3872962Z LL |     my_faulty_macro!();
2020-03-18T16:41:17.3873318Z    |
2020-03-18T16:41:17.3873318Z    |
2020-03-18T16:41:17.3873494Z    = note: expanding `my_faulty_macro! {  }`
2020-03-18T16:41:17.3873734Z    = note: to `my_faulty_macro ! (bcd) ;`
2020-03-18T16:41:17.3873993Z    = note: expanding `my_faulty_macro! { bcd }`
2020-03-18T16:41:17.3874357Z error: recursion limit reached while expanding `my_recursive_macro!`
2020-03-18T16:41:17.3874838Z   --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:22:9
2020-03-18T16:41:17.3875068Z    |
2020-03-18T16:41:17.3875068Z    |
2020-03-18T16:41:17.3875267Z LL |         my_recursive_macro!(); //~ ERROR recursion limit
2020-03-18T16:41:17.3875687Z ...
2020-03-18T16:41:17.3875687Z ...
2020-03-18T16:41:17.3875837Z LL |     my_recursive_macro!();
2020-03-18T16:41:17.3876439Z    |
2020-03-18T16:41:17.3876439Z    |
2020-03-18T16:41:17.3876709Z    = help: consider adding a `#![recursion_limit="8"]` attribute to your crate (`trace_faulty_macros`)
2020-03-18T16:41:17.3877596Z 
2020-03-18T16:41:17.3877724Z note: trace_macro
2020-03-18T16:41:17.3878348Z   --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:34:5
2020-03-18T16:41:17.3878988Z    |
2020-03-18T16:41:17.3878988Z    |
2020-03-18T16:41:17.3879179Z LL |     my_recursive_macro!();
2020-03-18T16:41:17.3879570Z    |
2020-03-18T16:41:17.3879570Z    |
2020-03-18T16:41:17.3879798Z    = note: expanding `my_recursive_macro! {  }`
2020-03-18T16:41:17.3880071Z    = note: to `my_recursive_macro ! () ;`
2020-03-18T16:41:17.3880345Z    = note: expanding `my_recursive_macro! {  }`
2020-03-18T16:41:17.3880635Z    = note: to `my_recursive_macro ! () ;`
2020-03-18T16:41:17.3880909Z    = note: expanding `my_recursive_macro! {  }`
2020-03-18T16:41:17.3881181Z    = note: to `my_recursive_macro ! () ;`
2020-03-18T16:41:17.3881727Z    = note: expanding `my_recursive_macro! {  }`
2020-03-18T16:41:17.3882182Z    = note: to `my_recursive_macro ! () ;`
2020-03-18T16:41:17.3882383Z 
2020-03-18T16:41:17.3882599Z error: expected expression, found `A { a: a, b: 0, c: _, .. }`
2020-03-18T16:41:17.3883351Z    |
2020-03-18T16:41:17.3883351Z    |
2020-03-18T16:41:17.3883524Z LL |         $a //~ ERROR expected expression
2020-03-18T16:41:17.3883956Z ...
2020-03-18T16:41:17.3884122Z LL |     let a = pat_macro!();
2020-03-18T16:41:17.3884520Z    |             ------------ in this macro invocation
2020-03-18T16:41:17.3884708Z    |
---
2020-03-18T16:41:17.3887702Z    |
2020-03-18T16:41:17.3887876Z LL |     let a = pat_macro!();
2020-03-18T16:41:17.3888069Z    |             ^^^^^^^^^^^^
2020-03-18T16:41:17.3888220Z    |
2020-03-18T16:41:17.3888410Z    = note: expanding `pat_macro! {  }`
2020-03-18T16:41:17.3888695Z    = note: to `pat_macro ! (A { a : a, b : 0, c : _, .. }) ;`
2020-03-18T16:41:17.3889043Z    = note: expanding `pat_macro! { A { a : a, b : 0, c : _, .. } }`
2020-03-18T16:41:17.3889373Z    = note: to `A { a: a, b: 0, c: _, .. }`
2020-03-18T16:41:17.3889706Z error: aborting due to 4 previous errors
2020-03-18T16:41:17.3889856Z 
2020-03-18T16:41:17.3889961Z 
2020-03-18T16:41:17.3890293Z ------------------------------------------
---
2020-03-18T16:41:17.3906051Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-18T16:41:17.3906466Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-18T16:41:17.3906699Z 
2020-03-18T16:41:17.3906783Z 
2020-03-18T16:41:17.3910844Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-18T16:41:17.3913538Z 
2020-03-18T16:41:17.3913874Z 
2020-03-18T16:41:17.3914526Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-18T16:41:17.3914861Z Build completed unsuccessfully in 1:04:06
2020-03-18T16:41:17.3914861Z Build completed unsuccessfully in 1:04:06
2020-03-18T16:41:17.3963484Z == clock drift check ==
2020-03-18T16:41:17.3981786Z   local time: Wed Mar 18 16:41:17 UTC 2020
2020-03-18T16:41:17.9511606Z   network time: Wed, 18 Mar 2020 16:41:17 GMT
2020-03-18T16:41:17.9514162Z == end clock drift check ==
2020-03-18T16:41:18.4363722Z 
2020-03-18T16:41:18.4454968Z ##[error]Bash exited with code '1'.
2020-03-18T16:41:18.4475821Z ##[section]Finishing: Run build
2020-03-18T16:41:18.4534453Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70109/merge to s
2020-03-18T16:41:18.4540091Z Task         : Get sources
2020-03-18T16:41:18.4540440Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T16:41:18.4540796Z Version      : 1.0.0
2020-03-18T16:41:18.4541022Z Author       : Microsoft
2020-03-18T16:41:18.4541022Z Author       : Microsoft
2020-03-18T16:41:18.4541389Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-18T16:41:18.4541822Z ==============================================================================
2020-03-18T16:41:18.7689978Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-18T16:41:18.7735961Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70109/merge to s
2020-03-18T16:41:18.7823112Z Cleaning up task key
2020-03-18T16:41:18.7824169Z Start cleaning up orphan processes.
2020-03-18T16:41:18.8014128Z Terminate orphan process: pid (3351) (python)
2020-03-18T16:41:18.8258644Z ##[section]Finishing: Finalize Job
