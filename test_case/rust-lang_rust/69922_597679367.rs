plain
2020-03-11T13:33:38.4887097Z ========================== Starting Command Output ===========================
2020-03-11T13:33:38.4896527Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/031f5ab9-331a-42aa-b9d8-612fe829327b.sh
2020-03-11T13:33:38.4896809Z 
2020-03-11T13:33:38.4901138Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-11T13:33:38.4922168Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69922/merge to s
2020-03-11T13:33:38.4925911Z Task         : Get sources
2020-03-11T13:33:38.4926213Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-11T13:33:38.4926508Z Version      : 1.0.0
2020-03-11T13:33:38.4928686Z Author       : Microsoft
---
2020-03-11T13:33:39.5013011Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-11T13:33:39.5021866Z ##[command]git config gc.auto 0
2020-03-11T13:33:39.5030975Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-11T13:33:39.5037059Z ##[command]git config --get-all http.proxy
2020-03-11T13:33:39.5046302Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69922/merge:refs/remotes/pull/69922/merge
---
2020-03-11T14:38:34.2065169Z .................................................................................................... 1700/9759
2020-03-11T14:38:39.4693889Z .................................................................................................... 1800/9759
2020-03-11T14:38:52.2165425Z ................................................................i................................... 1900/9759
2020-03-11T14:38:59.8696302Z .................................................................................................... 2000/9759
2020-03-11T14:39:15.7189071Z ......................................................iiiii......................................... 2100/9759
2020-03-11T14:39:27.6967080Z .................................................................................................... 2300/9759
2020-03-11T14:39:30.1476785Z .................................................................................................... 2400/9759
2020-03-11T14:39:33.6563568Z .................................................................................................... 2500/9759
2020-03-11T14:39:56.4865824Z .................................................................................................... 2600/9759
---
2020-03-11T14:42:45.7205899Z .......................i...............i............................................................ 5000/9759
2020-03-11T14:42:56.2509742Z .................................................................................................... 5100/9759
2020-03-11T14:43:02.1248657Z ..................................................................i................................. 5200/9759
2020-03-11T14:43:08.5641000Z .................................................................................................... 5300/9759
2020-03-11T14:43:18.2922921Z ...............................................ii.ii........i...i................................... 5400/9759
2020-03-11T14:43:26.9994058Z .................................................................................................... 5600/9759
2020-03-11T14:43:37.1133874Z .................................................................................................... 5700/9759
2020-03-11T14:43:44.0450833Z ......................................i............................................................. 5800/9759
2020-03-11T14:43:50.5259242Z .................................................................................................... 5900/9759
2020-03-11T14:43:50.5259242Z .................................................................................................... 5900/9759
2020-03-11T14:44:01.8095355Z .................................................................................................... 6000/9759
2020-03-11T14:44:11.5584264Z ...............................ii...i..ii...........i............................................... 6100/9759
2020-03-11T14:44:29.8274943Z .................................................................................................... 6300/9759
2020-03-11T14:44:37.1451929Z .................................................................................................... 6400/9759
2020-03-11T14:44:37.1451929Z .................................................................................................... 6400/9759
2020-03-11T14:44:46.3994177Z ..............................................................i..ii................................. 6500/9759
2020-03-11T14:45:13.6363649Z .................................................................................................... 6700/9759
2020-03-11T14:45:18.8655912Z ..........................................................i......................................... 6800/9759
2020-03-11T14:45:21.1063925Z .................................................................................................... 6900/9759
2020-03-11T14:45:23.3532547Z .........................................................................................i.......... 7000/9759
---
2020-03-11T14:47:08.2301823Z .................................................................................................... 7700/9759
2020-03-11T14:47:12.6864258Z .................................................................................................... 7800/9759
2020-03-11T14:47:18.9830536Z .................................................................................................... 7900/9759
2020-03-11T14:47:25.8980967Z .......................................i............................................................ 8000/9759
2020-03-11T14:47:35.7814750Z ........................................................................................iiiiiiiiii.i 8100/9759
2020-03-11T14:47:52.4741603Z ................................i......i............................................................ 8300/9759
2020-03-11T14:47:57.4369778Z .................................................................................................... 8400/9759
2020-03-11T14:48:09.0882018Z .................................................................................................... 8500/9759
2020-03-11T14:48:21.8416368Z .................................................................................................... 8600/9759
---
2020-03-11T14:50:28.8524287Z 
2020-03-11T14:50:28.8525622Z ---- [ui] ui/proc-macro/expand-to-unstable.rs stdout ----
2020-03-11T14:50:28.8525968Z diff of stderr:
2020-03-11T14:50:28.8526143Z 
2020-03-11T14:50:28.8527078Z - error[E0658]: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library
2020-03-11T14:50:28.8528348Z + error[E0425]: cannot find function `init` in module `std::intrinsics`
2020-03-11T14:50:28.8529197Z 3    |
2020-03-11T14:50:28.8529376Z 4 LL | #[derive(Unstable)]
2020-03-11T14:50:28.8529746Z 
2020-03-11T14:50:28.8530135Z -    |          ^^^^^^^^
---
2020-03-11T14:50:28.8534914Z 
2020-03-11T14:50:28.8535013Z 
2020-03-11T14:50:28.8535226Z The actual stderr differed from the expected stderr.
2020-03-11T14:50:28.8536287Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-to-unstable/expand-to-unstable.stderr
2020-03-11T14:50:28.8536985Z To update references, rerun the tests and pass the `--bless` flag
2020-03-11T14:50:28.8537607Z To only update this specific test, also pass `--test-args proc-macro/expand-to-unstable.rs`
2020-03-11T14:50:28.8538093Z error: 1 errors occurred comparing output.
2020-03-11T14:50:28.8538337Z status: exit code: 1
2020-03-11T14:50:28.8538337Z status: exit code: 1
2020-03-11T14:50:28.8540401Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/expand-to-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-to-unstable" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-to-unstable/auxiliary"
2020-03-11T14:50:28.8542106Z ------------------------------------------
2020-03-11T14:50:28.8542308Z 
2020-03-11T14:50:28.8542682Z ------------------------------------------
2020-03-11T14:50:28.8542892Z stderr:
---
2020-03-11T14:50:28.8576051Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-11T14:50:28.8577382Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-11T14:50:28.8589960Z 
2020-03-11T14:50:28.8590467Z 
2020-03-11T14:50:28.8594665Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-11T14:50:28.8605575Z 
2020-03-11T14:50:28.8605697Z 
2020-03-11T14:50:28.8610995Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-11T14:50:28.8611339Z Build completed unsuccessfully in 1:09:43
2020-03-11T14:50:28.8611339Z Build completed unsuccessfully in 1:09:43
2020-03-11T14:50:28.8671116Z == clock drift check ==
2020-03-11T14:50:28.8724821Z   local time: Wed Mar 11 14:50:28 UTC 2020
2020-03-11T14:50:29.0397582Z   network time: Wed, 11 Mar 2020 14:50:29 GMT
2020-03-11T14:50:29.0398858Z == end clock drift check ==
2020-03-11T14:50:29.5187364Z 
2020-03-11T14:50:29.5275530Z ##[error]Bash exited with code '1'.
2020-03-11T14:50:29.5314735Z ##[section]Finishing: Run build
2020-03-11T14:50:29.5413411Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69922/merge to s
2020-03-11T14:50:29.5418585Z Task         : Get sources
2020-03-11T14:50:29.5418917Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-11T14:50:29.5419226Z Version      : 1.0.0
2020-03-11T14:50:29.5419454Z Author       : Microsoft
2020-03-11T14:50:29.5419454Z Author       : Microsoft
2020-03-11T14:50:29.5420012Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-11T14:50:29.5420396Z ==============================================================================
2020-03-11T14:50:29.9051504Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-11T14:50:29.9099982Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69922/merge to s
2020-03-11T14:50:29.9194555Z Cleaning up task key
2020-03-11T14:50:29.9195783Z Start cleaning up orphan processes.
2020-03-11T14:50:29.9377108Z Terminate orphan process: pid (4150) (python)
2020-03-11T14:50:29.9633736Z ##[section]Finishing: Finalize Job
