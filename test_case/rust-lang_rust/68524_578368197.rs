plain
2020-01-25T01:36:42.4085519Z ========================== Starting Command Output ===========================
2020-01-25T01:36:42.4087413Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bba19e82-46f8-4821-9d88-94636356daa7.sh
2020-01-25T01:36:42.4087553Z 
2020-01-25T01:36:42.4090406Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-25T01:36:42.4097146Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-25T01:36:42.4098644Z Task         : Get sources
2020-01-25T01:36:42.4098678Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T01:36:42.4098754Z Version      : 1.0.0
2020-01-25T01:36:42.4098787Z Author       : Microsoft
---
2020-01-25T01:36:43.4091318Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-25T01:36:43.4103105Z ##[command]git config gc.auto 0
2020-01-25T01:36:43.4105715Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-25T01:36:43.4107906Z ##[command]git config --get-all http.proxy
2020-01-25T01:36:43.4114765Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68524/merge:refs/remotes/pull/68524/merge
---
2020-01-25T02:18:32.9833701Z .................................................................................................... 1700/9555
2020-01-25T02:18:38.5721889Z .................................................................................................... 1800/9555
2020-01-25T02:18:48.9488290Z ......................i............................................................................. 1900/9555
2020-01-25T02:18:55.2839086Z .................................................................................................... 2000/9555
2020-01-25T02:19:07.9959695Z ............iiiii................................................................................... 2100/9555
2020-01-25T02:19:16.2225492Z .................................................................................................... 2300/9555
2020-01-25T02:19:18.2608670Z .................................................................................................... 2400/9555
2020-01-25T02:19:22.8398220Z .................................................................................................... 2500/9555
2020-01-25T02:19:40.4977469Z .................................................................................................... 2600/9555
---
2020-01-25T02:21:57.0002972Z .............................................................i...............i...................... 4900/9555
2020-01-25T02:22:03.7525386Z .................................................................................................... 5000/9555
2020-01-25T02:22:10.6921549Z .................................................................................................... 5100/9555
2020-01-25T02:22:14.6915999Z ....i............................................................................................... 5200/9555
2020-01-25T02:22:24.2273994Z ............................................................................ii.ii........i...i...... 5300/9555
2020-01-25T02:22:31.8772980Z ..............i..................................................................................... 5500/9555
2020-01-25T02:22:40.7162932Z .................................................................................................... 5600/9555
2020-01-25T02:22:46.3263404Z ...............................................................i.................................... 5700/9555
2020-01-25T02:22:52.3466772Z .................................................................................................... 5800/9555
2020-01-25T02:22:52.3466772Z .................................................................................................... 5800/9555
2020-01-25T02:22:59.0874476Z .................................................................................................... 5900/9555
2020-01-25T02:23:07.3084705Z ......................................................ii...i..ii...........i........................ 6000/9555
2020-01-25T02:23:25.8994249Z .................................................................................................... 6200/9555
2020-01-25T02:23:29.8933342Z ........F........................................................................................... 6300/9555
2020-01-25T02:23:29.8933342Z ........F........................................................................................... 6300/9555
2020-01-25T02:23:33.8507843Z ..................................................................................i..ii............. 6400/9555
2020-01-25T02:23:54.8623917Z .................................................................................................... 6600/9555
2020-01-25T02:24:02.3200444Z ..........................................................i......................................... 6700/9555
2020-01-25T02:24:04.2577405Z .................................................................................................... 6800/9555
2020-01-25T02:24:06.2054620Z .........................................................i.......................................... 6900/9555
---
2020-01-25T02:25:31.9546447Z .................................................................................................... 7600/9555
2020-01-25T02:25:36.5451707Z .................................................................................................... 7700/9555
2020-01-25T02:25:42.1790548Z .................................................................................................... 7800/9555
2020-01-25T02:25:51.1792857Z .................................................................................................... 7900/9555
2020-01-25T02:25:56.4224553Z .............iiiiiii................................................................................ 8000/9555
2020-01-25T02:26:08.4530077Z .................................................................................................... 8200/9555
2020-01-25T02:26:17.4691938Z .................................................................................................... 8300/9555
2020-01-25T02:26:28.6333637Z .................................................................................................... 8400/9555
2020-01-25T02:26:34.2219958Z .................................................................................................... 8500/9555
---
2020-01-25T02:28:09.4936458Z -   --> $DIR/issue-55850.rs:28:9
2020-01-25T02:28:09.4936524Z + error[E0308]: mismatched types
2020-01-25T02:28:09.4936719Z +   --> $DIR/issue-55850.rs:18:44
2020-01-25T02:28:09.4936763Z 3    |
2020-01-25T02:28:09.4936965Z - LL |         yield &s[..]
2020-01-25T02:28:09.4937150Z -    |         ^^^^^^^-^^^^
2020-01-25T02:28:09.4937552Z -    |         |      `s` is borrowed here
2020-01-25T02:28:09.4937786Z -    |         yields a value referencing data owned by the current function
2020-01-25T02:28:09.4938103Z - 
2020-01-25T02:28:09.4939139Z - error[E0626]: borrow may still be in use when generator yields
2020-01-25T02:28:09.4939139Z - error[E0626]: borrow may still be in use when generator yields
2020-01-25T02:28:09.4939608Z -   --> $DIR/issue-55850.rs:28:16
2020-01-25T02:28:09.4939826Z + LL |         match Pin::new(&mut self.0).resume(()) {
2020-01-25T02:28:09.4940042Z +    |                                            ^^ expected associated type, found `()`
2020-01-25T02:28:09.4940229Z 12    |
2020-01-25T02:28:09.4940597Z - LL |         yield &s[..]
2020-01-25T02:28:09.4940987Z -    |         -------^---- possible yield occurs here
2020-01-25T02:28:09.4941202Z +    = note: expected associated type `<G as std::ops::Generator>::Resume`
2020-01-25T02:28:09.4941411Z +                     found unit type `()`
2020-01-25T02:28:09.4941610Z +    = note: consider constraining the associated type `<G as std::ops::Generator>::Resume` to `()` or calling a method that returns `<G as std::ops::Generator>::Resume`
2020-01-25T02:28:09.4942280Z 15 
2020-01-25T02:28:09.4942640Z - error: aborting due to 2 previous errors
2020-01-25T02:28:09.4943054Z + error: aborting due to previous error
2020-01-25T02:28:09.4943258Z 17 
---
2020-01-25T02:28:09.4948672Z 
2020-01-25T02:28:09.4948783Z 
2020-01-25T02:28:09.4948912Z The actual stderr differed from the expected stderr.
2020-01-25T02:28:09.4949311Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55850/issue-55850.stderr
2020-01-25T02:28:09.4949849Z To update references, rerun the tests and pass the `--bless` flag
2020-01-25T02:28:09.4950223Z To only update this specific test, also pass `--test-args nll/issue-55850.rs`
2020-01-25T02:28:09.4950474Z error: 1 errors occurred comparing output.
2020-01-25T02:28:09.4950630Z status: exit code: 1
2020-01-25T02:28:09.4950630Z status: exit code: 1
2020-01-25T02:28:09.4951464Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-55850.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55850" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55850/auxiliary" "-A" "unused"
2020-01-25T02:28:09.4951973Z ------------------------------------------
2020-01-25T02:28:09.4952113Z 
2020-01-25T02:28:09.4952410Z ------------------------------------------
2020-01-25T02:28:09.4952545Z stderr:
2020-01-25T02:28:09.4952545Z stderr:
2020-01-25T02:28:09.4952851Z ------------------------------------------
2020-01-25T02:28:09.4952994Z error[E0308]: mismatched types
2020-01-25T02:28:09.4953326Z   --> /checkout/src/test/ui/nll/issue-55850.rs:18:44
2020-01-25T02:28:09.4953480Z    |
2020-01-25T02:28:09.4953607Z LL |         match Pin::new(&mut self.0).resume(()) {
2020-01-25T02:28:09.4953740Z    |                                            ^^ expected associated type, found `()`
2020-01-25T02:28:09.4953881Z    |
2020-01-25T02:28:09.4954011Z    = note: expected associated type `<G as std::ops::Generator>::Resume`
2020-01-25T02:28:09.4954157Z                     found unit type `()`
2020-01-25T02:28:09.4954308Z    = note: consider constraining the associated type `<G as std::ops::Generator>::Resume` to `()` or calling a method that returns `<G as std::ops::Generator>::Resume`
2020-01-25T02:28:09.4956011Z 
2020-01-25T02:28:09.4956094Z error: aborting due to previous error
2020-01-25T02:28:09.4956187Z 
2020-01-25T02:28:09.4956476Z For more information about this error, try `rustc --explain E0308`.
---
2020-01-25T02:28:09.4957728Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:349:22
2020-01-25T02:28:09.4957800Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-25T02:28:09.4957834Z 
2020-01-25T02:28:09.4957876Z 
2020-01-25T02:28:09.4959608Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-25T02:28:09.4959938Z 
2020-01-25T02:28:09.4959968Z 
2020-01-25T02:28:09.4963763Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-25T02:28:09.4963851Z Build completed unsuccessfully in 0:46:33
2020-01-25T02:28:09.4963851Z Build completed unsuccessfully in 0:46:33
2020-01-25T02:28:09.5015170Z == clock drift check ==
2020-01-25T02:28:09.5036563Z   local time: Sat Jan 25 02:28:09 UTC 2020
2020-01-25T02:28:09.8019183Z   network time: Sat, 25 Jan 2020 02:28:09 GMT
2020-01-25T02:28:09.8022853Z == end clock drift check ==
2020-01-25T02:28:10.4444923Z 
2020-01-25T02:28:10.4538708Z ##[error]Bash exited with code '1'.
2020-01-25T02:28:10.4549477Z ##[section]Finishing: Run build
2020-01-25T02:28:10.4567745Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-25T02:28:10.4569506Z Task         : Get sources
2020-01-25T02:28:10.4569556Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T02:28:10.4569603Z Version      : 1.0.0
2020-01-25T02:28:10.4569659Z Author       : Microsoft
2020-01-25T02:28:10.4569659Z Author       : Microsoft
2020-01-25T02:28:10.4569705Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-25T02:28:10.4569754Z ==============================================================================
2020-01-25T02:28:10.8432744Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-25T02:28:10.8476981Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-25T02:28:10.8575159Z Cleaning up task key
2020-01-25T02:28:10.8575918Z Start cleaning up orphan processes.
2020-01-25T02:28:10.8667912Z Terminate orphan process: pid (3721) (python)
2020-01-25T02:28:10.8866375Z ##[section]Finishing: Finalize Job
