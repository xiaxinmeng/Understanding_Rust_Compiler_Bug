plain
2020-03-21T19:45:32.3625084Z ========================== Starting Command Output ===========================
2020-03-21T19:45:32.3647077Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2fbbe573-08e4-4f99-8617-32783fdbfde0.sh
2020-03-21T19:45:32.3821424Z 
2020-03-21T19:45:32.3916158Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-21T19:45:32.3933831Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70241/merge to s
2020-03-21T19:45:32.3937391Z Task         : Get sources
2020-03-21T19:45:32.3937650Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-21T19:45:32.3937904Z Version      : 1.0.0
2020-03-21T19:45:32.3938076Z Author       : Microsoft
---
2020-03-21T19:45:33.1465713Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-21T19:45:33.1470854Z ##[command]git config gc.auto 0
2020-03-21T19:45:33.1475417Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-21T19:45:33.1478895Z ##[command]git config --get-all http.proxy
2020-03-21T19:45:33.1485117Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70241/merge:refs/remotes/pull/70241/merge
---
2020-03-21T20:42:38.6643111Z .................................................................................................... 1700/9816
2020-03-21T20:42:42.7590162Z .................................................................................................... 1800/9816
2020-03-21T20:42:53.2451768Z ..............................................................................i..................... 1900/9816
2020-03-21T20:42:59.5813208Z .................................................................................................... 2000/9816
2020-03-21T20:43:07.0793911Z ....................................................................iiiii........................... 2100/9816
2020-03-21T20:43:26.1698832Z .................................................................................................... 2300/9816
2020-03-21T20:43:28.3323722Z .................................................................................................... 2400/9816
2020-03-21T20:43:30.9288316Z .................................................................................................... 2500/9816
2020-03-21T20:43:46.9523998Z .................................................................................................... 2600/9816
---
2020-03-21T20:46:37.1273539Z ..........................................i...............i......................................... 5000/9816
2020-03-21T20:46:45.9818890Z .................................................................................................... 5100/9816
2020-03-21T20:46:52.2837177Z ......................................................................................i............. 5200/9816
2020-03-21T20:46:57.9878139Z .................................................................................................... 5300/9816
2020-03-21T20:47:07.9335284Z ....................................................................ii.ii........i...i.............. 5400/9816
2020-03-21T20:47:15.7927424Z ........i........................................................................................... 5600/9816
2020-03-21T20:47:25.0561305Z .............i...................................................................................... 5700/9816
2020-03-21T20:47:30.9091390Z ..............................ii...................................i................................ 5800/9816
2020-03-21T20:47:37.6294251Z .................................................................................................... 5900/9816
2020-03-21T20:47:37.6294251Z .................................................................................................... 5900/9816
2020-03-21T20:47:44.2859229Z .................................................................................................... 6000/9816
2020-03-21T20:47:53.2987316Z .............................................................ii...i..ii...........i................. 6100/9816
2020-03-21T20:48:13.4423956Z .................................................................................................... 6300/9816
2020-03-21T20:48:20.7544259Z .................................................................................................... 6400/9816
2020-03-21T20:48:20.7544259Z .................................................................................................... 6400/9816
2020-03-21T20:48:27.1137269Z ...........................................................................................i..ii.... 6500/9816
2020-03-21T20:48:50.1239748Z .................................................................................................... 6700/9816
2020-03-21T20:49:00.8315716Z ..........................................................................................i......... 6800/9816
2020-03-21T20:49:03.0688263Z .................................................................................................... 6900/9816
2020-03-21T20:49:05.3248104Z .................................................................................................... 7000/9816
---
2020-03-21T20:50:49.8374180Z .................................................................................................... 7800/9816
2020-03-21T20:50:54.7627585Z .................................................................................................... 7900/9816
2020-03-21T20:51:00.7350626Z ..............................................................................i..................... 8000/9816
2020-03-21T20:51:10.7913121Z .................................................................................................... 8100/9816
2020-03-21T20:51:16.4508903Z ...........................iiiiiiiiii.i............................................................. 8200/9816
2020-03-21T20:51:31.0290380Z .................................................................................................... 8400/9816
2020-03-21T20:51:36.4539805Z .................................................................................................... 8500/9816
2020-03-21T20:51:51.9668033Z .................................................................................................... 8600/9816
2020-03-21T20:51:59.9295064Z .................................................................................................... 8700/9816
---
2020-03-21T20:53:57.8484512Z diff of stderr:
2020-03-21T20:53:57.8484806Z 
2020-03-21T20:53:57.8485452Z 2   --> $DIR/write-to-static-mut-in-static.rs:2:33
2020-03-21T20:53:57.8485892Z 3    |
2020-03-21T20:53:57.8486293Z 4 LL | pub static mut B: () = unsafe { A = 1; };
2020-03-21T20:53:57.8487115Z -    |                                 ^^^^^ tried to modify a static's initial value from another static's initializer
2020-03-21T20:53:57.8488056Z +    |                                 ^^^^^ modifying a static's initial value from another static's initializer
2020-03-21T20:53:57.8489194Z 7 error[E0391]: cycle detected when const-evaluating `C`
2020-03-21T20:53:57.8489929Z 8   --> $DIR/write-to-static-mut-in-static.rs:5:34
2020-03-21T20:53:57.8490353Z 
2020-03-21T20:53:57.8490612Z 
2020-03-21T20:53:57.8490612Z 
2020-03-21T20:53:57.8490985Z The actual stderr differed from the expected stderr.
2020-03-21T20:53:57.8491880Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/write-to-static-mut-in-static/write-to-static-mut-in-static.stderr
2020-03-21T20:53:57.8492757Z To update references, rerun the tests and pass the `--bless` flag
2020-03-21T20:53:57.8493595Z To only update this specific test, also pass `--test-args write-to-static-mut-in-static.rs`
2020-03-21T20:53:57.8494396Z error: 1 errors occurred comparing output.
2020-03-21T20:53:57.8494775Z status: exit code: 1
2020-03-21T20:53:57.8494775Z status: exit code: 1
2020-03-21T20:53:57.8496823Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/write-to-static-mut-in-static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/write-to-static-mut-in-static" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/write-to-static-mut-in-static/auxiliary"
2020-03-21T20:53:57.8498774Z ------------------------------------------
2020-03-21T20:53:57.8499869Z 
2020-03-21T20:53:57.8500556Z ------------------------------------------
2020-03-21T20:53:57.8500894Z stderr:
2020-03-21T20:53:57.8500894Z stderr:
2020-03-21T20:53:57.8501401Z ------------------------------------------
2020-03-21T20:53:57.8501793Z error[E0080]: could not evaluate static initializer
2020-03-21T20:53:57.8502603Z   --> /checkout/src/test/ui/write-to-static-mut-in-static.rs:2:33
2020-03-21T20:53:57.8502964Z    |
2020-03-21T20:53:57.8503257Z LL | pub static mut B: () = unsafe { A = 1; };
2020-03-21T20:53:57.8503965Z    |                                 ^^^^^ modifying a static's initial value from another static's initializer
2020-03-21T20:53:57.8505082Z error[E0391]: cycle detected when const-evaluating `C`
2020-03-21T20:53:57.8505853Z   --> /checkout/src/test/ui/write-to-static-mut-in-static.rs:5:34
2020-03-21T20:53:57.8507632Z    |
2020-03-21T20:53:57.8507632Z    |
2020-03-21T20:53:57.8507951Z LL | pub static mut C: u32 = unsafe { C = 1; 0 };
2020-03-21T20:53:57.8508571Z    |
2020-03-21T20:53:57.8508571Z    |
2020-03-21T20:53:57.8509109Z note: ...which requires const-evaluating `C`...
2020-03-21T20:53:57.8510090Z    |
2020-03-21T20:53:57.8510090Z    |
2020-03-21T20:53:57.8510400Z LL | pub static mut C: u32 = unsafe { C = 1; 0 };
2020-03-21T20:53:57.8511325Z    = note: ...which again requires const-evaluating `C`, completing the cycle
2020-03-21T20:53:57.8511325Z    = note: ...which again requires const-evaluating `C`, completing the cycle
2020-03-21T20:53:57.8511971Z note: cycle used when const-evaluating + checking `C`
2020-03-21T20:53:57.8512979Z    |
2020-03-21T20:53:57.8512979Z    |
2020-03-21T20:53:57.8513270Z LL | pub static mut C: u32 = unsafe { C = 1; 0 };
2020-03-21T20:53:57.8513867Z 
2020-03-21T20:53:57.8514132Z error: aborting due to 2 previous errors
2020-03-21T20:53:57.8514373Z 
2020-03-21T20:53:57.8514686Z Some errors have detailed explanations: E0080, E0391.
---
2020-03-21T20:53:57.8526085Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-21T20:53:57.8526651Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-21T20:53:57.8541232Z 
2020-03-21T20:53:57.8541497Z 
2020-03-21T20:53:57.8547022Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-21T20:53:57.8554361Z 
2020-03-21T20:53:57.8554674Z 
2020-03-21T20:53:57.8555498Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-21T20:53:57.8555832Z Build completed unsuccessfully in 1:04:28
2020-03-21T20:53:57.8555832Z Build completed unsuccessfully in 1:04:28
2020-03-21T20:53:57.8601798Z == clock drift check ==
2020-03-21T20:53:57.8617910Z   local time: Sat Mar 21 20:53:57 UTC 2020
2020-03-21T20:53:58.0057377Z   network time: Sat, 21 Mar 2020 20:53:58 GMT
2020-03-21T20:53:58.0061840Z == end clock drift check ==
2020-03-21T20:53:58.4077455Z 
2020-03-21T20:53:58.4150982Z ##[error]Bash exited with code '1'.
2020-03-21T20:53:58.4163539Z ##[section]Finishing: Run build
2020-03-21T20:53:58.4207149Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70241/merge to s
2020-03-21T20:53:58.4211991Z Task         : Get sources
2020-03-21T20:53:58.4212374Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-21T20:53:58.4212739Z Version      : 1.0.0
2020-03-21T20:53:58.4212973Z Author       : Microsoft
2020-03-21T20:53:58.4212973Z Author       : Microsoft
2020-03-21T20:53:58.4213351Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-21T20:53:58.4213823Z ==============================================================================
2020-03-21T20:53:58.7594857Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-21T20:53:58.7641134Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70241/merge to s
2020-03-21T20:53:58.7725677Z Cleaning up task key
2020-03-21T20:53:58.7726801Z Start cleaning up orphan processes.
2020-03-21T20:53:58.7901701Z Terminate orphan process: pid (4812) (python)
2020-03-21T20:53:58.8083740Z ##[section]Finishing: Finalize Job
