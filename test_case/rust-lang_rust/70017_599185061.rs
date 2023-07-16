plain
2020-03-15T07:44:42.2942310Z ========================== Starting Command Output ===========================
2020-03-15T07:44:42.2945336Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ad8bf09e-f5a3-4a9a-aee1-99c626612256.sh
2020-03-15T07:44:42.2945625Z 
2020-03-15T07:44:42.2950068Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-15T07:44:42.2978869Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70017/merge to s
2020-03-15T07:44:42.2982574Z Task         : Get sources
2020-03-15T07:44:42.2982916Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-15T07:44:42.2983227Z Version      : 1.0.0
2020-03-15T07:44:42.2983438Z Author       : Microsoft
---
2020-03-15T07:44:43.3126121Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-15T07:44:43.3132196Z ##[command]git config gc.auto 0
2020-03-15T07:44:43.3136480Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-15T07:44:43.3140406Z ##[command]git config --get-all http.proxy
2020-03-15T07:44:43.3147308Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70017/merge:refs/remotes/pull/70017/merge
---
2020-03-15T08:55:49.2116715Z .................................................................................................... 1700/9773
2020-03-15T08:55:54.1223953Z .................................................................................................... 1800/9773
2020-03-15T08:56:06.9653867Z .....................................................................i.............................. 1900/9773
2020-03-15T08:56:14.3398571Z .................................................................................................... 2000/9773
2020-03-15T08:56:31.1480490Z ...........................................................iiiii.................................... 2100/9773
2020-03-15T08:56:42.9460256Z .................................................................................................... 2300/9773
2020-03-15T08:56:45.6688650Z .................................................................................................... 2400/9773
2020-03-15T08:56:49.1311280Z .................................................................................................... 2500/9773
2020-03-15T08:57:12.1514823Z .................................................................................................... 2600/9773
---
2020-03-15T09:00:12.7592526Z ...............................i...............i.................................................... 5000/9773
2020-03-15T09:00:23.7325589Z .................................................................................................... 5100/9773
2020-03-15T09:00:31.4065522Z ..........................................................................i......................... 5200/9773
2020-03-15T09:00:37.6952554Z .................................................................................................... 5300/9773
2020-03-15T09:00:49.2382415Z .......................................................ii.ii........i...i........................... 5400/9773
2020-03-15T09:00:58.6093651Z .................................................................................................... 5600/9773
2020-03-15T09:01:09.5989853Z .................................................................................................... 5700/9773
2020-03-15T09:01:16.5690502Z ...............................................i.................................................... 5800/9773
2020-03-15T09:01:23.9735415Z .................................................................................................... 5900/9773
2020-03-15T09:01:23.9735415Z .................................................................................................... 5900/9773
2020-03-15T09:01:35.5051888Z .................................................................................................... 6000/9773
2020-03-15T09:01:42.1621107Z .........................................ii...i..ii...........i..................................... 6100/9773
2020-03-15T09:02:04.7858272Z .................................................................................................... 6300/9773
2020-03-15T09:02:12.6185644Z .................................................................................................... 6400/9773
2020-03-15T09:02:12.6185644Z .................................................................................................... 6400/9773
2020-03-15T09:02:23.0015845Z .......................................................................i..ii........................ 6500/9773
2020-03-15T09:02:42.1816560Z ........................test [ui] ui/mpsc_stress.rs has been running for over 60 seconds
2020-03-15T09:03:01.6581458Z .................................................................................................... 6700/9773
2020-03-15T09:03:11.4571955Z .....................................................................i.............................. 6800/9773
2020-03-15T09:03:13.8248217Z .................................................................................................... 6900/9773
2020-03-15T09:03:16.4777504Z .................................................................................................... 7000/9773
---
2020-03-15T09:05:18.4012496Z .................................................................................................... 7800/9773
2020-03-15T09:05:25.5958018Z .................................................................................................... 7900/9773
2020-03-15T09:05:32.6926243Z .....................................................i.............................................. 8000/9773
2020-03-15T09:05:44.8100674Z .................................................................................................... 8100/9773
2020-03-15T09:05:51.2011855Z ..iiiiiiiiii.i...................................................................................... 8200/9773
2020-03-15T09:06:07.0969192Z .................................................................................................... 8400/9773
2020-03-15T09:06:19.9477666Z .................................................................................................... 8500/9773
2020-03-15T09:06:35.5439301Z .................................................................................................... 8600/9773
2020-03-15T09:06:42.7289161Z .................................................................................................... 8700/9773
---
2020-03-15T09:08:54.9388315Z 
2020-03-15T09:08:54.9388927Z 
2020-03-15T09:08:54.9389176Z 
2020-03-15T09:08:54.9389587Z The actual stderr differed from the expected stderr.
2020-03-15T09:08:54.9391064Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/issue-69201.unused/issue-69201.unused.stderr
2020-03-15T09:08:54.9392526Z To update references, rerun the tests and pass the `--bless` flag
2020-03-15T09:08:54.9394058Z To only update this specific test, also pass `--test-args associated-const/issue-69201.rs`
2020-03-15T09:08:54.9395297Z error in revision `unused`: 1 errors occurred comparing output.
2020-03-15T09:08:54.9396107Z status: exit code: 1
2020-03-15T09:08:54.9396107Z status: exit code: 1
2020-03-15T09:08:54.9400424Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-const/issue-69201.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "unused" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/issue-69201.unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Copt-level=2" "--emit" "link" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/issue-69201.unused/auxiliary"
2020-03-15T09:08:54.9402964Z ------------------------------------------
2020-03-15T09:08:54.9403337Z 
2020-03-15T09:08:54.9403932Z ------------------------------------------
2020-03-15T09:08:54.9404357Z stderr:
---
2020-03-15T09:08:54.9418781Z 
2020-03-15T09:08:54.9418905Z 
2020-03-15T09:08:54.9419014Z 
2020-03-15T09:08:54.9419244Z The actual stderr differed from the expected stderr.
2020-03-15T09:08:54.9420060Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/issue-69201.used/issue-69201.used.stderr
2020-03-15T09:08:54.9420828Z To update references, rerun the tests and pass the `--bless` flag
2020-03-15T09:08:54.9421517Z To only update this specific test, also pass `--test-args associated-const/issue-69201.rs`
2020-03-15T09:08:54.9422087Z error in revision `used`: 1 errors occurred comparing output.
2020-03-15T09:08:54.9422602Z status: exit code: 1
2020-03-15T09:08:54.9422602Z status: exit code: 1
2020-03-15T09:08:54.9425069Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-const/issue-69201.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "used" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/issue-69201.used" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Copt-level=2" "--emit" "link" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/issue-69201.used/auxiliary"
2020-03-15T09:08:54.9427012Z ------------------------------------------
2020-03-15T09:08:54.9427211Z 
2020-03-15T09:08:54.9427626Z ------------------------------------------
2020-03-15T09:08:54.9427878Z stderr:
---
2020-03-15T09:08:54.9436781Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-15T09:08:54.9437263Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-15T09:08:54.9437525Z 
2020-03-15T09:08:54.9437633Z 
2020-03-15T09:08:54.9441847Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-15T09:08:54.9444839Z 
2020-03-15T09:08:54.9445128Z 
2020-03-15T09:08:54.9445395Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-15T09:08:54.9445763Z Build completed unsuccessfully in 1:16:35
2020-03-15T09:08:54.9445763Z Build completed unsuccessfully in 1:16:35
2020-03-15T09:08:54.9453555Z == clock drift check ==
2020-03-15T09:08:54.9494908Z   local time: Sun Mar 15 09:08:54 UTC 2020
2020-03-15T09:08:55.2451794Z   network time: Sun, 15 Mar 2020 09:08:55 GMT
2020-03-15T09:08:55.2452664Z == end clock drift check ==
2020-03-15T09:08:55.8286663Z 
2020-03-15T09:08:55.8435683Z ##[error]Bash exited with code '1'.
2020-03-15T09:08:55.8450459Z ##[section]Finishing: Run build
2020-03-15T09:08:55.8506264Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70017/merge to s
2020-03-15T09:08:55.8511667Z Task         : Get sources
2020-03-15T09:08:55.8512010Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-15T09:08:55.8512341Z Version      : 1.0.0
2020-03-15T09:08:55.8512580Z Author       : Microsoft
2020-03-15T09:08:55.8512580Z Author       : Microsoft
2020-03-15T09:08:55.8512933Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-15T09:08:55.8513355Z ==============================================================================
2020-03-15T09:08:56.2315546Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-15T09:08:56.2368816Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70017/merge to s
2020-03-15T09:08:56.2466046Z Cleaning up task key
2020-03-15T09:08:56.2467403Z Start cleaning up orphan processes.
2020-03-15T09:08:56.2724942Z Terminate orphan process: pid (4894) (python)
2020-03-15T09:08:56.2932495Z ##[section]Finishing: Finalize Job
