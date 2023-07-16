plain
2020-01-25T00:02:47.9515794Z ========================== Starting Command Output ===========================
2020-01-25T00:02:47.9517481Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b2319394-bd5e-4a8f-94be-a120f7e7f7e3.sh
2020-01-25T00:02:47.9517521Z 
2020-01-25T00:02:47.9522229Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-25T00:02:47.9528338Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-25T00:02:47.9530348Z Task         : Get sources
2020-01-25T00:02:47.9530380Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T00:02:47.9530411Z Version      : 1.0.0
2020-01-25T00:02:47.9530485Z Author       : Microsoft
---
2020-01-25T00:02:48.7595079Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-25T00:02:48.7697837Z ##[command]git config gc.auto 0
2020-01-25T00:02:48.7802692Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-25T00:02:48.7831709Z ##[command]git config --get-all http.proxy
2020-01-25T00:02:48.7975328Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68524/merge:refs/remotes/pull/68524/merge
---
2020-01-25T00:58:37.2792845Z .................................................................................................... 1700/9555
2020-01-25T00:58:43.2199652Z .................................................................................................... 1800/9555
2020-01-25T00:58:54.4548557Z ......................i............................................................................. 1900/9555
2020-01-25T00:59:01.1949979Z .................................................................................................... 2000/9555
2020-01-25T00:59:14.0077808Z ............iiiii................................F.................................................. 2100/9555
2020-01-25T00:59:23.2688396Z .................................................................................................... 2300/9555
2020-01-25T00:59:25.6383867Z .................................................................................................... 2400/9555
2020-01-25T00:59:30.7275746Z .................................................................................................... 2500/9555
2020-01-25T00:59:50.7852781Z .................................................................................................... 2600/9555
---
2020-01-25T01:02:25.7905215Z .............................................................i...............i...................... 4900/9555
2020-01-25T01:02:33.1981325Z .................................................................................................... 5000/9555
2020-01-25T01:02:41.0793607Z .................................................................................................... 5100/9555
2020-01-25T01:02:45.4939532Z ....i............................................................................................... 5200/9555
2020-01-25T01:02:56.5206730Z ............................................................................ii.ii........i...i...... 5300/9555
2020-01-25T01:03:05.2817734Z ..............i..................................................................................... 5500/9555
2020-01-25T01:03:14.9014859Z .................................................................................................... 5600/9555
2020-01-25T01:03:21.3596942Z ...............................................................i.................................... 5700/9555
2020-01-25T01:03:28.3392517Z .................................................................................................... 5800/9555
2020-01-25T01:03:28.3392517Z .................................................................................................... 5800/9555
2020-01-25T01:03:35.7904732Z .................................................................................................... 5900/9555
2020-01-25T01:03:44.3704807Z ......................................................ii...i..ii...........i........................ 6000/9555
2020-01-25T01:04:06.0263879Z .................................................................................................... 6200/9555
2020-01-25T01:04:14.0689062Z ......F............................................................................................. 6300/9555
2020-01-25T01:04:14.0689062Z ......F............................................................................................. 6300/9555
2020-01-25T01:04:22.3836646Z ..................................................................................i..ii............. 6400/9555
2020-01-25T01:04:48.7882481Z .................................................................................................... 6600/9555
2020-01-25T01:04:57.4651713Z ..........................................................i......................................... 6700/9555
2020-01-25T01:04:59.6731545Z .................................................................................................... 6800/9555
2020-01-25T01:05:01.8921002Z .........................................................i.......................................... 6900/9555
---
2020-01-25T01:06:41.5500308Z .................................................................................................... 7600/9555
2020-01-25T01:06:46.7954036Z .................................................................................................... 7700/9555
2020-01-25T01:06:53.5659457Z .................................................................................................... 7800/9555
2020-01-25T01:07:04.0016179Z .................................................................................................... 7900/9555
2020-01-25T01:07:09.9787894Z .............iiiiiii................................................................................ 8000/9555
2020-01-25T01:07:24.1184859Z .................................................................................................... 8200/9555
2020-01-25T01:07:34.5557876Z .................................................................................................... 8300/9555
2020-01-25T01:07:47.4507400Z .................................................................................................... 8400/9555
2020-01-25T01:07:54.1011996Z .................................................................................................... 8500/9555
---
2020-01-25T01:09:47.9824969Z ---- [ui] ui/drop/dynamic-drop.rs stdout ----
2020-01-25T01:09:47.9825328Z 
2020-01-25T01:09:47.9825974Z error: test compilation failed although it shouldn't!
2020-01-25T01:09:47.9826334Z status: exit code: 1
2020-01-25T01:09:47.9827449Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/drop/dynamic-drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop/auxiliary"
2020-01-25T01:09:47.9828468Z ------------------------------------------
2020-01-25T01:09:47.9828790Z 
2020-01-25T01:09:47.9829375Z ------------------------------------------
2020-01-25T01:09:47.9829739Z stderr:
2020-01-25T01:09:47.9829739Z stderr:
2020-01-25T01:09:47.9830286Z ------------------------------------------
2020-01-25T01:09:47.9830632Z error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2020-01-25T01:09:47.9831420Z   --> /checkout/src/test/ui/drop/dynamic-drop.rs:187:28
2020-01-25T01:09:47.9831833Z    |
2020-01-25T01:09:47.9832112Z LL |         Pin::new(&mut gen).resume();
2020-01-25T01:09:47.9833297Z    |
2020-01-25T01:09:47.9833297Z    |
2020-01-25T01:09:47.9833576Z help: expected the unit value `()`; create it with empty parentheses
2020-01-25T01:09:47.9833881Z    |
2020-01-25T01:09:47.9834146Z LL |         Pin::new(&mut gen).resume(());
2020-01-25T01:09:47.9834660Z 
2020-01-25T01:09:47.9834929Z error: aborting due to previous error
2020-01-25T01:09:47.9835156Z 
2020-01-25T01:09:47.9835809Z For more information about this error, try `rustc --explain E0061`.
---
2020-01-25T01:09:47.9839610Z -   --> $DIR/issue-55850.rs:28:9
2020-01-25T01:09:47.9840002Z + error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2020-01-25T01:09:47.9840547Z +   --> $DIR/issue-55850.rs:18:37
2020-01-25T01:09:47.9840886Z 3    |
2020-01-25T01:09:47.9841443Z - LL |         yield &s[..]
2020-01-25T01:09:47.9842032Z -    |         ^^^^^^^-^^^^
2020-01-25T01:09:47.9844480Z -    |         |      `s` is borrowed here
2020-01-25T01:09:47.9845130Z -    |         yields a value referencing data owned by the current function
2020-01-25T01:09:47.9845130Z -    |         yields a value referencing data owned by the current function
2020-01-25T01:09:47.9845503Z + LL |         match Pin::new(&mut self.0).resume() {
2020-01-25T01:09:47.9846084Z 9 
2020-01-25T01:09:47.9847361Z - error[E0626]: borrow may still be in use when generator yields
2020-01-25T01:09:47.9848043Z -   --> $DIR/issue-55850.rs:28:16
2020-01-25T01:09:47.9848556Z -    |
2020-01-25T01:09:47.9848556Z -    |
2020-01-25T01:09:47.9849029Z - LL |         yield &s[..]
2020-01-25T01:09:47.9849562Z -    |         -------^---- possible yield occurs here
2020-01-25T01:09:47.9850004Z 15 
2020-01-25T01:09:47.9850479Z - error: aborting due to 2 previous errors
2020-01-25T01:09:47.9850943Z - 
2020-01-25T01:09:47.9851454Z - Some errors have detailed explanations: E0515, E0626.
2020-01-25T01:09:47.9851454Z - Some errors have detailed explanations: E0515, E0626.
2020-01-25T01:09:47.9852004Z - For more information about an error, try `rustc --explain E0515`.
2020-01-25T01:09:47.9852874Z + For more information about this error, try `rustc --explain E0061`.
2020-01-25T01:09:47.9853168Z 20 
2020-01-25T01:09:47.9853354Z 
2020-01-25T01:09:47.9853509Z 
2020-01-25T01:09:47.9853682Z The actual stderr differed from the expected stderr.
2020-01-25T01:09:47.9854215Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55850/issue-55850.stderr
2020-01-25T01:09:47.9854765Z To update references, rerun the tests and pass the `--bless` flag
2020-01-25T01:09:47.9855317Z To only update this specific test, also pass `--test-args nll/issue-55850.rs`
2020-01-25T01:09:47.9857764Z error: 1 errors occurred comparing output.
2020-01-25T01:09:47.9857951Z status: exit code: 1
2020-01-25T01:09:47.9857951Z status: exit code: 1
2020-01-25T01:09:47.9859094Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-55850.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55850" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55850/auxiliary" "-A" "unused"
2020-01-25T01:09:47.9860124Z ------------------------------------------
2020-01-25T01:09:47.9860343Z 
2020-01-25T01:09:47.9860828Z ------------------------------------------
2020-01-25T01:09:47.9861225Z stderr:
2020-01-25T01:09:47.9861225Z stderr:
2020-01-25T01:09:47.9861714Z ------------------------------------------
2020-01-25T01:09:47.9861985Z error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2020-01-25T01:09:47.9862896Z   --> /checkout/src/test/ui/nll/issue-55850.rs:18:37
2020-01-25T01:09:47.9863202Z    |
2020-01-25T01:09:47.9863414Z LL |         match Pin::new(&mut self.0).resume() {
2020-01-25T01:09:47.9863750Z 
2020-01-25T01:09:47.9863958Z error: aborting due to previous error
2020-01-25T01:09:47.9864108Z 
2020-01-25T01:09:47.9864611Z For more information about this error, try `rustc --explain E0061`.
---
2020-01-25T01:09:47.9869439Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:349:22
2020-01-25T01:09:47.9869679Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-25T01:09:47.9877425Z 
2020-01-25T01:09:47.9877784Z 
2020-01-25T01:09:47.9879834Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-25T01:09:47.9880394Z 
2020-01-25T01:09:47.9880534Z 
2020-01-25T01:09:47.9885466Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-25T01:09:47.9886059Z Build completed unsuccessfully in 1:01:27
2020-01-25T01:09:47.9886059Z Build completed unsuccessfully in 1:01:27
2020-01-25T01:09:47.9936253Z == clock drift check ==
2020-01-25T01:09:47.9955246Z   local time: Sat Jan 25 01:09:47 UTC 2020
2020-01-25T01:09:48.1545739Z   network time: Sat, 25 Jan 2020 01:09:48 GMT
2020-01-25T01:09:48.1548149Z == end clock drift check ==
2020-01-25T01:09:48.5175039Z 
2020-01-25T01:09:48.5290410Z ##[error]Bash exited with code '1'.
2020-01-25T01:09:48.5303441Z ##[section]Finishing: Run build
2020-01-25T01:09:48.5325741Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-25T01:09:48.5327709Z Task         : Get sources
2020-01-25T01:09:48.5327759Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T01:09:48.5327826Z Version      : 1.0.0
2020-01-25T01:09:48.5328024Z Author       : Microsoft
2020-01-25T01:09:48.5328024Z Author       : Microsoft
2020-01-25T01:09:48.5328071Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-25T01:09:48.5328142Z ==============================================================================
2020-01-25T01:09:48.9508505Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-25T01:09:48.9550558Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-25T01:09:48.9663171Z Cleaning up task key
2020-01-25T01:09:48.9663946Z Start cleaning up orphan processes.
2020-01-25T01:09:48.9772775Z Terminate orphan process: pid (3565) (python)
2020-01-25T01:09:49.0055554Z ##[section]Finishing: Finalize Job
