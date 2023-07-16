plain
2020-03-17T18:00:46.2376628Z ========================== Starting Command Output ===========================
2020-03-17T18:00:46.2379933Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/51b668ff-9e3d-4ddc-95b6-5e011d9b7db7.sh
2020-03-17T18:00:46.2380267Z 
2020-03-17T18:00:46.2384877Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-17T18:00:46.2407736Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67343/merge to s
2020-03-17T18:00:46.2411884Z Task         : Get sources
2020-03-17T18:00:46.2412208Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T18:00:46.2412524Z Version      : 1.0.0
2020-03-17T18:00:46.2412768Z Author       : Microsoft
---
2020-03-17T18:00:47.2529047Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-17T18:00:47.2534354Z ##[command]git config gc.auto 0
2020-03-17T18:00:47.2538212Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-17T18:00:47.2541762Z ##[command]git config --get-all http.proxy
2020-03-17T18:00:47.2548453Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67343/merge:refs/remotes/pull/67343/merge
---
2020-03-17T19:03:23.7382874Z .................................................................................................... 1700/9799
2020-03-17T19:03:27.8662703Z .................................................................................................... 1800/9799
2020-03-17T19:03:38.5307731Z ..............................................................................i..................... 1900/9799
2020-03-17T19:03:44.8583920Z .................................................................................................... 2000/9799
2020-03-17T19:03:52.3258111Z ....................................................................iiiii........................... 2100/9799
2020-03-17T19:04:11.3527500Z .................................................................................................... 2300/9799
2020-03-17T19:04:13.5161496Z .................................................................................................... 2400/9799
2020-03-17T19:04:16.3091636Z .................................................................................................... 2500/9799
2020-03-17T19:04:33.6799647Z .................................................................................................... 2600/9799
---
2020-03-17T19:07:21.0172851Z ........................................i...............i........................................... 5000/9799
2020-03-17T19:07:30.0343484Z .................................................................................................... 5100/9799
2020-03-17T19:07:36.5711511Z ............F......................................................................i................ 5200/9799
2020-03-17T19:07:42.4999254Z .................................................................................................... 5300/9799
2020-03-17T19:07:53.1179581Z ................................................................ii.ii........i...i.................. 5400/9799
2020-03-17T19:08:01.3950387Z ...i................................................................................................ 5600/9799
2020-03-17T19:08:11.1235922Z .................................................................................................... 5700/9799
2020-03-17T19:08:17.7248957Z .........................................................i.......................................... 5800/9799
2020-03-17T19:08:24.5783549Z .................................................................................................... 5900/9799
2020-03-17T19:08:24.5783549Z .................................................................................................... 5900/9799
2020-03-17T19:08:32.5760571Z .................................................................................................... 6000/9799
2020-03-17T19:08:40.6045668Z ...................................................ii...i..ii...........i........................... 6100/9799
2020-03-17T19:09:01.0552767Z .................................................................................................... 6300/9799
2020-03-17T19:09:08.7004085Z .................................................................................................... 6400/9799
2020-03-17T19:09:08.7004085Z .................................................................................................... 6400/9799
2020-03-17T19:09:16.4969071Z .................................................................................i..ii.............. 6500/9799
2020-03-17T19:09:45.4729903Z .................................................................................................... 6700/9799
2020-03-17T19:09:55.0508294Z ...............................................................................i.................... 6800/9799
2020-03-17T19:09:57.1681460Z .................................................................................................... 6900/9799
2020-03-17T19:09:59.3523948Z .................................................................................................... 7000/9799
---
2020-03-17T19:11:45.3154789Z .................................................................................................... 7800/9799
2020-03-17T19:11:50.8332384Z .................................................................................................... 7900/9799
2020-03-17T19:11:56.8864625Z ................................................................i................................... 8000/9799
2020-03-17T19:12:07.1770121Z .................................................................................................... 8100/9799
2020-03-17T19:12:12.8588054Z .............iiiiiiiiii.i........................................................................... 8200/9799
2020-03-17T19:12:26.9064601Z .................................................................................................... 8400/9799
2020-03-17T19:12:34.7521427Z .................................................................................................... 8500/9799
2020-03-17T19:12:49.0091908Z .................................................................................................... 8600/9799
2020-03-17T19:12:55.6926742Z .................................................................................................... 8700/9799
---
2020-03-17T19:14:51.0296544Z 
2020-03-17T19:14:51.0297246Z - warning: to use a constant of type `std::cell::Cell` in a pattern, `std::cell::Cell` must be annotated with `#[derive(PartialEq, Eq)]`
2020-03-17T19:14:51.0297921Z -   --> $DIR/issue-55511.rs:16:9
2020-03-17T19:14:51.0298270Z -    |
2020-03-17T19:14:51.0298686Z - LL |         <() as Foo<'static>>::C => { }
2020-03-17T19:14:51.0299512Z -    |
2020-03-17T19:14:51.0299895Z - note: the lint level is defined here
2020-03-17T19:14:51.0300640Z -   --> $DIR/issue-55511.rs:1:9
2020-03-17T19:14:51.0300999Z -    |
2020-03-17T19:14:51.0300999Z -    |
2020-03-17T19:14:51.0301378Z - LL | #![warn(indirect_structural_match)]
2020-03-17T19:14:51.0301814Z -    |         ^^^^^^^^^^^^^^^^^^^^^^^^^
2020-03-17T19:14:51.0302531Z -    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2020-03-17T19:14:51.0303440Z -    = note: for more information, see issue #62411 <***/issues/62411>
2020-03-17T19:14:51.0304270Z 15 error[E0597]: `a` does not live long enough
2020-03-17T19:14:51.0304721Z 16   --> $DIR/issue-55511.rs:13:28
2020-03-17T19:14:51.0304942Z 17    |
2020-03-17T19:14:51.0305056Z 
2020-03-17T19:14:51.0305056Z 
2020-03-17T19:14:51.0305154Z 
2020-03-17T19:14:51.0305365Z The actual stderr differed from the expected stderr.
2020-03-17T19:14:51.0306175Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55511/issue-55511.stderr
2020-03-17T19:14:51.0306855Z To update references, rerun the tests and pass the `--bless` flag
2020-03-17T19:14:51.0307458Z To only update this specific test, also pass `--test-args issues/issue-55511.rs`
2020-03-17T19:14:51.0307920Z error: 1 errors occurred comparing output.
2020-03-17T19:14:51.0308160Z status: exit code: 1
2020-03-17T19:14:51.0308160Z status: exit code: 1
2020-03-17T19:14:51.0310133Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-55511.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55511" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55511/auxiliary"
2020-03-17T19:14:51.0311763Z ------------------------------------------
2020-03-17T19:14:51.0311947Z 
2020-03-17T19:14:51.0312319Z ------------------------------------------
2020-03-17T19:14:51.0312524Z stderr:
2020-03-17T19:14:51.0312524Z stderr:
2020-03-17T19:14:51.0312912Z ------------------------------------------
2020-03-17T19:14:51.0313189Z error[E0597]: `a` does not live long enough
2020-03-17T19:14:51.0313690Z   --> /checkout/src/test/ui/issues/issue-55511.rs:13:28
2020-03-17T19:14:51.0313945Z    |
2020-03-17T19:14:51.0314156Z LL |     let b = Some(Cell::new(&a));
2020-03-17T19:14:51.0314488Z    |                            ^^ borrowed value does not live long enough
2020-03-17T19:14:51.0314760Z ...
2020-03-17T19:14:51.0315161Z LL |         <() as Foo<'static>>::C => { }
2020-03-17T19:14:51.0315771Z    |         ----------------------- type annotation requires that `a` is borrowed for `'static`
2020-03-17T19:14:51.0316214Z LL | }
2020-03-17T19:14:51.0316601Z    | - `a` dropped here while still borrowed
2020-03-17T19:14:51.0316799Z 
2020-03-17T19:14:51.0316989Z error: aborting due to previous error
---
2020-03-17T19:14:51.0320533Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-17T19:14:51.0320955Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-17T19:14:51.0335087Z 
2020-03-17T19:14:51.0335242Z 
2020-03-17T19:14:51.0339959Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-17T19:14:51.0342888Z 
2020-03-17T19:14:51.0342992Z 
2020-03-17T19:14:51.0348371Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-17T19:14:51.0348722Z Build completed unsuccessfully in 1:08:06
2020-03-17T19:14:51.0348722Z Build completed unsuccessfully in 1:08:06
2020-03-17T19:14:51.0405384Z == clock drift check ==
2020-03-17T19:14:51.0422622Z   local time: Tue Mar 17 19:14:51 UTC 2020
2020-03-17T19:14:51.5987892Z   network time: Tue, 17 Mar 2020 19:14:51 GMT
2020-03-17T19:14:51.5988317Z == end clock drift check ==
2020-03-17T19:14:52.0579972Z 
2020-03-17T19:14:52.0657452Z ##[error]Bash exited with code '1'.
2020-03-17T19:14:52.0671745Z ##[section]Finishing: Run build
2020-03-17T19:14:52.0726278Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67343/merge to s
2020-03-17T19:14:52.0731346Z Task         : Get sources
2020-03-17T19:14:52.0731696Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T19:14:52.0732036Z Version      : 1.0.0
2020-03-17T19:14:52.0732260Z Author       : Microsoft
2020-03-17T19:14:52.0732260Z Author       : Microsoft
2020-03-17T19:14:52.0732617Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-17T19:14:52.0733050Z ==============================================================================
2020-03-17T19:14:52.4141669Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-17T19:14:52.4193407Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67343/merge to s
2020-03-17T19:14:52.4294972Z Cleaning up task key
2020-03-17T19:14:52.4296366Z Start cleaning up orphan processes.
2020-03-17T19:14:52.4632929Z Terminate orphan process: pid (3917) (python)
2020-03-17T19:14:52.4705326Z ##[section]Finishing: Finalize Job
