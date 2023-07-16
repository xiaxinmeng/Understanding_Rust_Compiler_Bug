plain
2020-03-17T22:07:03.2200077Z ========================== Starting Command Output ===========================
2020-03-17T22:07:03.2205595Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/09c3a7fa-8afb-458a-92a3-59f21b5af777.sh
2020-03-17T22:07:03.2206158Z 
2020-03-17T22:07:03.2211320Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-17T22:07:03.2234809Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67343/merge to s
2020-03-17T22:07:03.2238480Z Task         : Get sources
2020-03-17T22:07:03.2238830Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T22:07:03.2239144Z Version      : 1.0.0
2020-03-17T22:07:03.2239361Z Author       : Microsoft
---
2020-03-17T22:07:04.2146169Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-17T22:07:04.2152584Z ##[command]git config gc.auto 0
2020-03-17T22:07:04.2156877Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-17T22:07:04.2160812Z ##[command]git config --get-all http.proxy
2020-03-17T22:07:04.2167869Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67343/merge:refs/remotes/pull/67343/merge
---
2020-03-17T23:06:26.3278415Z .................................................................................................... 1700/9799
2020-03-17T23:06:30.5913609Z .................................................................................................... 1800/9799
2020-03-17T23:06:41.3950344Z ..............................................................................i..................... 1900/9799
2020-03-17T23:06:48.2525624Z .................................................................................................... 2000/9799
2020-03-17T23:06:56.0300629Z ....................................................................iiiii........................... 2100/9799
2020-03-17T23:07:15.8660273Z .................................................................................................... 2300/9799
2020-03-17T23:07:18.1126290Z .................................................................................................... 2400/9799
2020-03-17T23:07:20.9116904Z .................................................................................................... 2500/9799
2020-03-17T23:07:38.2160880Z .................................................................................................... 2600/9799
---
2020-03-17T23:10:24.2651879Z ........................................i...............i........................................... 5000/9799
2020-03-17T23:10:33.4251712Z .................................................................................................... 5100/9799
2020-03-17T23:10:39.9245023Z ............F......................................................................i................ 5200/9799
2020-03-17T23:10:45.8410796Z .................................................................................................... 5300/9799
2020-03-17T23:10:56.3266160Z ................................................................ii.ii........i...i.................. 5400/9799
2020-03-17T23:11:04.6038063Z ...i................................................................................................ 5600/9799
2020-03-17T23:11:14.6206481Z .................................................................................................... 5700/9799
2020-03-17T23:11:21.0554042Z .........................................................i.......................................... 5800/9799
2020-03-17T23:11:28.0267608Z .................................................................................................... 5900/9799
2020-03-17T23:11:28.0267608Z .................................................................................................... 5900/9799
2020-03-17T23:11:36.1386834Z .................................................................................................... 6000/9799
2020-03-17T23:11:44.4175009Z ...................................................ii...i..ii...........i........................... 6100/9799
2020-03-17T23:12:05.2283627Z .................................................................................................... 6300/9799
2020-03-17T23:12:12.6056791Z .................................................................................................... 6400/9799
2020-03-17T23:12:12.6056791Z .................................................................................................... 6400/9799
2020-03-17T23:12:20.6188647Z .................................................................................i..ii.............. 6500/9799
2020-03-17T23:12:45.7150886Z .................................................................................................... 6700/9799
2020-03-17T23:12:55.1731713Z ...............................................................................i.................... 6800/9799
2020-03-17T23:12:57.3021587Z .................................................................................................... 6900/9799
2020-03-17T23:12:59.4862601Z .................................................................................................... 7000/9799
---
2020-03-17T23:14:46.4721044Z .................................................................................................... 7800/9799
2020-03-17T23:14:52.1755416Z .................................................................................................... 7900/9799
2020-03-17T23:14:58.4037073Z ................................................................i................................... 8000/9799
2020-03-17T23:15:09.1047320Z .................................................................................................... 8100/9799
2020-03-17T23:15:14.9264560Z .............iiiiiiiiii.i........................................................................... 8200/9799
2020-03-17T23:15:29.5546958Z .................................................................................................... 8400/9799
2020-03-17T23:15:37.6262039Z .................................................................................................... 8500/9799
2020-03-17T23:15:51.9384526Z .................................................................................................... 8600/9799
2020-03-17T23:15:58.4858395Z .................................................................................................... 8700/9799
---
2020-03-17T23:17:59.8759022Z 
2020-03-17T23:17:59.8760202Z - warning: to use a constant of type `std::cell::Cell` in a pattern, `std::cell::Cell` must be annotated with `#[derive(PartialEq, Eq)]`
2020-03-17T23:17:59.8761198Z -   --> $DIR/issue-55511.rs:16:9
2020-03-17T23:17:59.8761841Z -    |
2020-03-17T23:17:59.8762497Z - LL |         <() as Foo<'static>>::C => { }
2020-03-17T23:17:59.8763823Z -    |
2020-03-17T23:17:59.8764419Z - note: the lint level is defined here
2020-03-17T23:17:59.8765115Z -   --> $DIR/issue-55511.rs:1:9
2020-03-17T23:17:59.8765686Z -    |
2020-03-17T23:17:59.8765686Z -    |
2020-03-17T23:17:59.8766293Z - LL | #![warn(indirect_structural_match)]
2020-03-17T23:17:59.8766998Z -    |         ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-17T23:17:59.8768933Z -    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-03-17T23:17:59.8770050Z -    = note: for more information, see issue #62411 <***/issues/62411>
2020-03-17T23:17:59.8770744Z 15 error[E0597]: `a` does not live long enough
2020-03-17T23:17:59.8771259Z 16   --> $DIR/issue-55511.rs:13:28
2020-03-17T23:17:59.8771498Z 17    |
2020-03-17T23:17:59.8771628Z 
2020-03-17T23:17:59.8771628Z 
2020-03-17T23:17:59.8771739Z 
2020-03-17T23:17:59.8771994Z The actual stderr differed from the expected stderr.
2020-03-17T23:17:59.8772995Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55511/issue-55511.stderr
2020-03-17T23:17:59.8773935Z To update references, rerun the tests and pass the `--bless` flag
2020-03-17T23:17:59.8774660Z To only update this specific test, also pass `--test-args issues/issue-55511.rs`
2020-03-17T23:17:59.8775147Z error: 1 errors occurred comparing output.
2020-03-17T23:17:59.8775414Z status: exit code: 1
2020-03-17T23:17:59.8775414Z status: exit code: 1
2020-03-17T23:17:59.8779724Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-55511.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55511" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55511/auxiliary"
2020-03-17T23:17:59.8781759Z ------------------------------------------
2020-03-17T23:17:59.8781963Z 
2020-03-17T23:17:59.8782855Z ------------------------------------------
2020-03-17T23:17:59.8783659Z stderr:
2020-03-17T23:17:59.8783659Z stderr:
2020-03-17T23:17:59.8784542Z ------------------------------------------
2020-03-17T23:17:59.8811574Z error[E0597]: `a` does not live long enough
2020-03-17T23:17:59.8812391Z   --> /checkout/src/test/ui/issues/issue-55511.rs:13:28
2020-03-17T23:17:59.8812662Z    |
2020-03-17T23:17:59.8812900Z LL |     let b = Some(Cell::new(&a));
2020-03-17T23:17:59.8813291Z    |                            ^^ borrowed value does not live long enough
2020-03-17T23:17:59.8813576Z ...
2020-03-17T23:17:59.8820158Z LL |         <() as Foo<'static>>::C => { }
2020-03-17T23:17:59.8820897Z    |         ----------------------- type annotation requires that `a` is borrowed for `'static`
2020-03-17T23:17:59.8821333Z LL | }
2020-03-17T23:17:59.8821742Z    | - `a` dropped here while still borrowed
2020-03-17T23:17:59.8821923Z 
2020-03-17T23:17:59.8822106Z error: aborting due to previous error
---
2020-03-17T23:17:59.8824848Z test result: FAILED. 9741 passed; 1 failed; 57 ignored; 0 measured; 0 filtered out
2020-03-17T23:17:59.8825117Z 
2020-03-17T23:17:59.8825268Z 
2020-03-17T23:17:59.8825389Z 
2020-03-17T23:17:59.8829184Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-17T23:17:59.8832110Z 
2020-03-17T23:17:59.8832211Z 
2020-03-17T23:17:59.8832759Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-17T23:17:59.8833196Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-17T23:17:59.8833196Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-17T23:17:59.8841636Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-17T23:17:59.8842029Z Build completed unsuccessfully in 1:05:44
2020-03-17T23:17:59.8897720Z == clock drift check ==
2020-03-17T23:17:59.8916240Z   local time: Tue Mar 17 23:17:59 UTC 2020
2020-03-17T23:18:00.4452247Z   network time: Tue, 17 Mar 2020 23:18:00 GMT
2020-03-17T23:18:00.4452598Z == end clock drift check ==
2020-03-17T23:18:00.9005167Z 
2020-03-17T23:18:00.9053244Z ##[error]Bash exited with code '1'.
2020-03-17T23:18:00.9078576Z ##[section]Finishing: Run build
2020-03-17T23:18:00.9139487Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67343/merge to s
2020-03-17T23:18:00.9145539Z Task         : Get sources
2020-03-17T23:18:00.9145951Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T23:18:00.9146309Z Version      : 1.0.0
2020-03-17T23:18:00.9146558Z Author       : Microsoft
2020-03-17T23:18:00.9146558Z Author       : Microsoft
2020-03-17T23:18:00.9146969Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-17T23:18:00.9147431Z ==============================================================================
2020-03-17T23:18:01.2939022Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-17T23:18:01.2987678Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67343/merge to s
2020-03-17T23:18:01.3097652Z Cleaning up task key
2020-03-17T23:18:01.3098926Z Start cleaning up orphan processes.
2020-03-17T23:18:01.3307520Z Terminate orphan process: pid (4077) (python)
2020-03-17T23:18:01.3507783Z ##[section]Finishing: Finalize Job
