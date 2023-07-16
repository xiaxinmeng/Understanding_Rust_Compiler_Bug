plain
2020-03-17T23:30:06.3708227Z ========================== Starting Command Output ===========================
2020-03-17T23:30:06.3712062Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3c0aac41-d6ed-4cb4-a2f8-bf13c694cd7e.sh
2020-03-17T23:30:06.3712304Z 
2020-03-17T23:30:06.3716845Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-17T23:30:06.3738040Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70087/merge to s
2020-03-17T23:30:06.3740917Z Task         : Get sources
2020-03-17T23:30:06.3741162Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T23:30:06.3741450Z Version      : 1.0.0
2020-03-17T23:30:06.3741611Z Author       : Microsoft
---
2020-03-17T23:30:07.3594410Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-17T23:30:07.3599143Z ##[command]git config gc.auto 0
2020-03-17T23:30:07.3602056Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-17T23:30:07.3604744Z ##[command]git config --get-all http.proxy
2020-03-17T23:30:07.3611360Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70087/merge:refs/remotes/pull/70087/merge
---
2020-03-18T00:28:56.6946539Z ................................................................................F................... 1700/9796
2020-03-18T00:29:00.8278791Z .................................................................................................... 1800/9796
2020-03-18T00:29:12.0728503Z ..........................................................................i......................... 1900/9796
2020-03-18T00:29:18.6433230Z .................................................................................................... 2000/9796
2020-03-18T00:29:26.7588655Z ................................................................iiiii............................... 2100/9796
2020-03-18T00:29:44.2150273Z .................................................................................................... 2300/9796
2020-03-18T00:29:46.3712546Z .................................................................................................... 2400/9796
2020-03-18T00:29:49.3100159Z .................................................................................................... 2500/9796
2020-03-18T00:30:09.5453654Z .................................................................................................... 2600/9796
---
2020-03-18T00:32:50.5195557Z ....................................i...............i............................................... 5000/9796
2020-03-18T00:32:59.7402425Z .................................................................................................... 5100/9796
2020-03-18T00:33:05.9785196Z ...............................................................................i.................... 5200/9796
2020-03-18T00:33:11.2781661Z .................................................................................................... 5300/9796
2020-03-18T00:33:21.0085593Z ............................................................ii.ii........i...i...................... 5400/9796
2020-03-18T00:33:25.2295377Z ...................................................................................................i 5500/9796
2020-03-18T00:33:38.4415127Z .................................................................................................... 5700/9796
2020-03-18T00:33:44.3764393Z .....................................................i.............................................. 5800/9796
2020-03-18T00:33:50.5787464Z .................................................................................................... 5900/9796
2020-03-18T00:33:59.4097460Z .................................................................................................... 6000/9796
2020-03-18T00:33:59.4097460Z .................................................................................................... 6000/9796
2020-03-18T00:34:05.2255420Z ...............................................ii...i..ii...........i............................... 6100/9796
2020-03-18T00:34:25.0421809Z .................................................................................................... 6300/9796
2020-03-18T00:34:31.8871931Z .................................................................................................... 6400/9796
2020-03-18T00:34:31.8871931Z .................................................................................................... 6400/9796
2020-03-18T00:34:40.5089366Z .............................................................................i..ii.................. 6500/9796
2020-03-18T00:35:02.8000706Z .................................................................................................... 6700/9796
2020-03-18T00:35:11.9061940Z ...........................................................................i........................ 6800/9796
2020-03-18T00:35:13.9311908Z .................................................................................................... 6900/9796
2020-03-18T00:35:16.0011969Z .................................................................................................... 7000/9796
---
2020-03-18T00:36:58.8261714Z .................................................................................................... 7800/9796
2020-03-18T00:37:04.4185569Z .................................................................................................... 7900/9796
2020-03-18T00:37:10.8968729Z .............................................................i...................................... 8000/9796
2020-03-18T00:37:20.5276253Z .................................................................................................... 8100/9796
2020-03-18T00:37:25.9329505Z ..........iiiiiiiiii.i.............................................................................. 8200/9796
2020-03-18T00:37:38.8710057Z .................................................................................................... 8400/9796
2020-03-18T00:37:46.2867916Z .................................................................................................... 8500/9796
2020-03-18T00:37:59.2811202Z .................................................................................................... 8600/9796
2020-03-18T00:38:05.7005209Z .................................................................................................... 8700/9796
---
2020-03-18T00:39:56.4503742Z 5 LL | |     let mut x = 0;
2020-03-18T00:39:56.4504024Z 
2020-03-18T00:39:56.4504293Z 
2020-03-18T00:39:56.4504643Z The actual stderr differed from the expected stderr.
2020-03-18T00:39:56.4505543Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_limit/const_eval_limit_reached/const_eval_limit_reached.stderr
2020-03-18T00:39:56.4506451Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T00:39:56.4507275Z To only update this specific test, also pass `--test-args consts/const_limit/const_eval_limit_reached.rs`
2020-03-18T00:39:56.4508121Z error: 1 errors occurred comparing output.
2020-03-18T00:39:56.4508493Z status: exit code: 1
2020-03-18T00:39:56.4508493Z status: exit code: 1
2020-03-18T00:39:56.4510568Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_limit/const_eval_limit_reached.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_limit/const_eval_limit_reached" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_limit/const_eval_limit_reached/auxiliary"
2020-03-18T00:39:56.4512520Z ------------------------------------------
2020-03-18T00:39:56.4512883Z 
2020-03-18T00:39:56.4513780Z ------------------------------------------
2020-03-18T00:39:56.4515684Z stderr:
2020-03-18T00:39:56.4515684Z stderr:
2020-03-18T00:39:56.4516993Z ------------------------------------------
2020-03-18T00:39:56.4517552Z error: any use of this value will cause an error
2020-03-18T00:39:56.4518330Z   --> /checkout/src/test/ui/consts/const_limit/const_eval_limit_reached.rs:8:11
2020-03-18T00:39:56.4518696Z    |
2020-03-18T00:39:56.4518963Z LL | / const X: usize = {
2020-03-18T00:39:56.4519236Z LL | |     let mut x = 0;
2020-03-18T00:39:56.4519506Z LL | |     while x != 1000 {
2020-03-18T00:39:56.4519848Z    | |           ^^^^^^^^^ exceeded interpreter time limit
2020-03-18T00:39:56.4520227Z LL | |         //~^ ERROR any use of this value will cause an error
2020-03-18T00:39:56.4520768Z LL | |     x
2020-03-18T00:39:56.4520992Z LL | | };
2020-03-18T00:39:56.4521389Z    | |__-
2020-03-18T00:39:56.4521658Z    |
---
2020-03-18T00:39:56.4534183Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-18T00:39:56.4534739Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-18T00:39:56.4534973Z 
2020-03-18T00:39:56.4535056Z 
2020-03-18T00:39:56.4539528Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-18T00:39:56.4542592Z 
2020-03-18T00:39:56.4542703Z 
2020-03-18T00:39:56.4545619Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-18T00:39:56.4546204Z Build completed unsuccessfully in 1:05:45
2020-03-18T00:39:56.4546204Z Build completed unsuccessfully in 1:05:45
2020-03-18T00:39:56.4597782Z == clock drift check ==
2020-03-18T00:39:56.4617878Z   local time: Wed Mar 18 00:39:56 UTC 2020
2020-03-18T00:39:56.7592021Z   network time: Wed, 18 Mar 2020 00:39:56 GMT
2020-03-18T00:39:56.7596784Z == end clock drift check ==
2020-03-18T00:39:57.1887001Z 
2020-03-18T00:39:57.1927871Z ##[error]Bash exited with code '1'.
2020-03-18T00:39:57.1945232Z ##[section]Finishing: Run build
2020-03-18T00:39:57.1997676Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70087/merge to s
2020-03-18T00:39:57.2003409Z Task         : Get sources
2020-03-18T00:39:57.2003906Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T00:39:57.2004559Z Version      : 1.0.0
2020-03-18T00:39:57.2005107Z Author       : Microsoft
2020-03-18T00:39:57.2005107Z Author       : Microsoft
2020-03-18T00:39:57.2005594Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-18T00:39:57.2005974Z ==============================================================================
2020-03-18T00:39:57.5296756Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-18T00:39:57.5335810Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70087/merge to s
2020-03-18T00:39:57.5420762Z Cleaning up task key
2020-03-18T00:39:57.5422079Z Start cleaning up orphan processes.
2020-03-18T00:39:57.5623097Z Terminate orphan process: pid (3796) (python)
2020-03-18T00:39:57.5804203Z ##[section]Finishing: Finalize Job
