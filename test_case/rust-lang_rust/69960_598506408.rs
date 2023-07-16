plain
2020-03-13T00:27:30.8408038Z ========================== Starting Command Output ===========================
2020-03-13T00:27:30.8412744Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9b0ce68f-45d8-4a3c-9084-e400b9735d06.sh
2020-03-13T00:27:30.8413105Z 
2020-03-13T00:27:30.8417177Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-13T00:27:30.8431723Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69960/merge to s
2020-03-13T00:27:30.8434502Z Task         : Get sources
2020-03-13T00:27:30.8434723Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T00:27:30.8434934Z Version      : 1.0.0
2020-03-13T00:27:30.8435078Z Author       : Microsoft
---
2020-03-13T00:27:32.1071362Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-13T00:27:32.1080531Z ##[command]git config gc.auto 0
2020-03-13T00:27:32.1084813Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-13T00:27:32.1088731Z ##[command]git config --get-all http.proxy
2020-03-13T00:27:32.1096643Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69960/merge:refs/remotes/pull/69960/merge
---
2020-03-13T01:25:45.8633377Z .................................................................................................... 1700/9766
2020-03-13T01:25:50.0914059Z .................................................................................................... 1800/9766
2020-03-13T01:26:01.1944575Z ................................................................i................................... 1900/9766
2020-03-13T01:26:07.9161995Z .................................................................................................... 2000/9766
2020-03-13T01:26:21.8799394Z ......................................................iiiii......................................... 2100/9766
2020-03-13T01:26:31.6844720Z .................................................................................................... 2300/9766
2020-03-13T01:26:33.8280746Z .................................................................................................... 2400/9766
2020-03-13T01:26:36.8527410Z .................................................................................................... 2500/9766
2020-03-13T01:26:57.2291448Z .................................................................................................... 2600/9766
---
2020-03-13T01:29:30.9805198Z .........................i...............i.......................................................... 5000/9766
2020-03-13T01:29:40.3330289Z .................................................................................................... 5100/9766
2020-03-13T01:29:45.6447591Z ....................................................................i............................... 5200/9766
2020-03-13T01:29:51.3528293Z .................................................................................................... 5300/9766
2020-03-13T01:30:00.1156823Z .................................................ii.ii........i...i................................. 5400/9766
2020-03-13T01:30:08.0160928Z .................................................................................................... 5600/9766
2020-03-13T01:30:17.7283382Z .................................................................................................... 5700/9766
2020-03-13T01:30:24.2773887Z ........................................i........................................................... 5800/9766
2020-03-13T01:30:30.7214935Z .................................................................................................... 5900/9766
2020-03-13T01:30:30.7214935Z .................................................................................................... 5900/9766
2020-03-13T01:30:41.0247753Z .................................................................................................... 6000/9766
2020-03-13T01:30:49.8785715Z .................................ii...i..ii...........i............................................. 6100/9766
2020-03-13T01:31:06.6008834Z .................................................................................................... 6300/9766
2020-03-13T01:31:13.4853075Z .................................................................................................... 6400/9766
2020-03-13T01:31:13.4853075Z .................................................................................................... 6400/9766
2020-03-13T01:31:24.5660784Z ................................................................i..ii............................... 6500/9766
2020-03-13T01:31:52.3138257Z .................................................................................................... 6700/9766
2020-03-13T01:31:58.0961560Z ..............................................................i..................................... 6800/9766
2020-03-13T01:32:00.2607453Z .................................................................................................... 6900/9766
2020-03-13T01:32:02.1977841Z ................................................................................................i... 7000/9766
---
2020-03-13T01:33:36.2340415Z .................................................................................................... 7700/9766
2020-03-13T01:33:40.3669324Z .................................................................................................... 7800/9766
2020-03-13T01:33:46.0125428Z .................................................................................................... 7900/9766
2020-03-13T01:33:51.7406052Z ..............................................i..................................................... 8000/9766
2020-03-13T01:34:01.3205527Z ...............................................................................................iiiii 8100/9766
2020-03-13T01:34:06.8907640Z iiiii.i............................................................................................. 8200/9766
2020-03-13T01:34:20.5726873Z .................................................................................................... 8400/9766
2020-03-13T01:34:31.1164926Z .................................................................................................... 8500/9766
2020-03-13T01:34:42.6780152Z .................................................................................................... 8600/9766
2020-03-13T01:34:48.0702664Z .................................................................................................... 8700/9766
---
2020-03-13T01:36:41.0562971Z -    |              entering unreachable code
2020-03-13T01:36:41.0563461Z +    |              transmuting to uninhabited type
2020-03-13T01:36:41.0563969Z 8    |              inside call to `foo` at $DIR/validate_uninhabited_zsts.rs:14:26
2020-03-13T01:36:41.0564506Z 9 ...
2020-03-13T01:36:41.0565038Z 10 LL | const FOO: [Empty; 3] = [foo(); 3];
2020-03-13T01:36:41.0565436Z 
2020-03-13T01:36:41.0565675Z The actual stderr differed from the expected stderr.
2020-03-13T01:36:41.0566759Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts/validate_uninhabited_zsts.stderr
2020-03-13T01:36:41.0566759Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts/validate_uninhabited_zsts.stderr
2020-03-13T01:36:41.0567509Z To update references, rerun the tests and pass the `--bless` flag
2020-03-13T01:36:41.0568215Z To only update this specific test, also pass `--test-args consts/const-eval/validate_uninhabited_zsts.rs`
2020-03-13T01:36:41.0568705Z error: 1 errors occurred comparing output.
2020-03-13T01:36:41.0568967Z status: exit code: 1
2020-03-13T01:36:41.0568967Z status: exit code: 1
2020-03-13T01:36:41.0571149Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts/auxiliary"
2020-03-13T01:36:41.0572938Z ------------------------------------------
2020-03-13T01:36:41.0573120Z 
2020-03-13T01:36:41.0573516Z ------------------------------------------
2020-03-13T01:36:41.0573727Z stderr:
---
2020-03-13T01:36:41.0576441Z    |              |
2020-03-13T01:36:41.0576700Z    |              transmuting to uninhabited type
2020-03-13T01:36:41.0577404Z    |              inside call to `foo` at /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:14:26
2020-03-13T01:36:41.0577758Z ...
2020-03-13T01:36:41.0577985Z LL | const FOO: [Empty; 3] = [foo(); 3];
2020-03-13T01:36:41.0578643Z    |
2020-03-13T01:36:41.0578852Z note: the lint level is defined here
2020-03-13T01:36:41.0579420Z   --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:13:8
2020-03-13T01:36:41.0579705Z    |
2020-03-13T01:36:41.0579705Z    |
2020-03-13T01:36:41.0579889Z LL | #[warn(const_err)]
2020-03-13T01:36:41.0580085Z    |        ^^^^^^^^^
2020-03-13T01:36:41.0580225Z 
2020-03-13T01:36:41.0580643Z error[E0080]: it is undefined behavior to use this value
2020-03-13T01:36:41.0583919Z   --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:17:1
2020-03-13T01:36:41.0584227Z    |
2020-03-13T01:36:41.0584505Z LL | const BAR: [Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
2020-03-13T01:36:41.0585104Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Empty at [0]
2020-03-13T01:36:41.0585536Z    |
2020-03-13T01:36:41.0586528Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-03-13T01:36:41.0587493Z warning: the type `!` does not permit zero-initialization
2020-03-13T01:36:41.0588123Z   --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:5:14
2020-03-13T01:36:41.0588413Z    |
2020-03-13T01:36:41.0589322Z LL |     unsafe { std::mem::transmute(()) }
---
2020-03-13T01:36:41.0591657Z 
2020-03-13T01:36:41.0597392Z warning: the type `Empty` does not permit zero-initialization
2020-03-13T01:36:41.0598129Z   --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:17:35
2020-03-13T01:36:41.0598651Z    |
2020-03-13T01:36:41.0598934Z LL | const BAR: [Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
2020-03-13T01:36:41.0599652Z    |                                   |
2020-03-13T01:36:41.0600001Z    |                                   this code causes undefined behavior when executed
2020-03-13T01:36:41.0600535Z    |                                   help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
2020-03-13T01:36:41.0600933Z    |
---
2020-03-13T01:36:41.0631918Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-13T01:36:41.0633283Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-13T01:36:41.0645460Z 
2020-03-13T01:36:41.0645834Z 
2020-03-13T01:36:41.0650190Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-13T01:36:41.0653389Z 
2020-03-13T01:36:41.0653592Z 
2020-03-13T01:36:41.0686044Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-13T01:36:41.0686436Z Build completed unsuccessfully in 1:03:45
2020-03-13T01:36:41.0686436Z Build completed unsuccessfully in 1:03:45
2020-03-13T01:36:41.0705390Z == clock drift check ==
2020-03-13T01:36:41.0721061Z   local time: Fri Mar 13 01:36:41 UTC 2020
2020-03-13T01:36:41.6252820Z   network time: Fri, 13 Mar 2020 01:36:41 GMT
2020-03-13T01:36:41.6253392Z == end clock drift check ==
2020-03-13T01:36:42.0016757Z 
2020-03-13T01:36:42.0095011Z ##[error]Bash exited with code '1'.
2020-03-13T01:36:42.0109099Z ##[section]Finishing: Run build
2020-03-13T01:36:42.0161923Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69960/merge to s
2020-03-13T01:36:42.0166991Z Task         : Get sources
2020-03-13T01:36:42.0167305Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T01:36:42.0167597Z Version      : 1.0.0
2020-03-13T01:36:42.0167817Z Author       : Microsoft
2020-03-13T01:36:42.0167817Z Author       : Microsoft
2020-03-13T01:36:42.0168481Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-13T01:36:42.0169172Z ==============================================================================
2020-03-13T01:36:42.3599437Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-13T01:36:42.3640369Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69960/merge to s
2020-03-13T01:36:42.3723915Z Cleaning up task key
2020-03-13T01:36:42.3724989Z Start cleaning up orphan processes.
2020-03-13T01:36:42.3881337Z Terminate orphan process: pid (5559) (python)
2020-03-13T01:36:42.4119820Z ##[section]Finishing: Finalize Job
