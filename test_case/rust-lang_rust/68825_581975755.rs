plain
2020-02-04T14:48:57.9515384Z ========================== Starting Command Output ===========================
2020-02-04T14:48:57.9516521Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/49b44902-07e5-4696-807e-330a350d6ba9.sh
2020-02-04T14:48:57.9516549Z 
2020-02-04T14:48:57.9518646Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-04T14:48:57.9523450Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68825/merge to s
2020-02-04T14:48:57.9524952Z Task         : Get sources
2020-02-04T14:48:57.9524980Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-04T14:48:57.9525022Z Version      : 1.0.0
2020-02-04T14:48:57.9525049Z Author       : Microsoft
---
2020-02-04T14:48:59.7636292Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-04T14:48:59.7646859Z ##[command]git config gc.auto 0
2020-02-04T14:48:59.7648699Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-04T14:48:59.7650091Z ##[command]git config --get-all http.proxy
2020-02-04T14:48:59.7655044Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68825/merge:refs/remotes/pull/68825/merge
---
2020-02-04T15:41:49.7245602Z .................................................................................................... 1700/9583
2020-02-04T15:41:54.3820359Z .................................................................................................... 1800/9583
2020-02-04T15:42:06.0447452Z ............................i....................................................................... 1900/9583
2020-02-04T15:42:12.7398441Z .................................................................................................... 2000/9583
2020-02-04T15:42:26.2425675Z ..................iiiii............................................................................. 2100/9583
2020-02-04T15:42:35.4988356Z .................................................................................................... 2300/9583
2020-02-04T15:42:37.7660410Z .................................................................................................... 2400/9583
2020-02-04T15:42:42.3464858Z .................................................................................................... 2500/9583
2020-02-04T15:43:02.0274697Z .................................................................................................... 2600/9583
---
2020-02-04T15:45:34.3350069Z .............................................................i...............i...................... 4900/9583
2020-02-04T15:45:41.6324479Z .................................................................................................... 5000/9583
2020-02-04T15:45:49.2428242Z .................................................................................................... 5100/9583
2020-02-04T15:45:53.6851039Z ....i............................................................................................... 5200/9583
2020-02-04T15:46:04.2280952Z ..............................................................................ii.ii........i...i.... 5300/9583
2020-02-04T15:46:12.3289585Z ................i................................................................................... 5500/9583
2020-02-04T15:46:21.3238950Z .................................................................................................... 5600/9583
2020-02-04T15:46:27.8492637Z .................................................................i.................................. 5700/9583
2020-02-04T15:46:34.7163818Z .................................................................................................... 5800/9583
2020-02-04T15:46:34.7163818Z .................................................................................................... 5800/9583
2020-02-04T15:46:41.8827892Z .................................................................................................... 5900/9583
2020-02-04T15:46:50.5920706Z ........................................................ii...i..ii...........i...................... 6000/9583
2020-02-04T15:47:11.0548227Z .................................................................................................... 6200/9583
2020-02-04T15:47:18.1628495Z .................................................................................................... 6300/9583
2020-02-04T15:47:18.1628495Z .................................................................................................... 6300/9583
2020-02-04T15:47:25.8548696Z ....................................................................................i..ii........... 6400/9583
2020-02-04T15:47:50.3185461Z .................................................................................................... 6600/9583
2020-02-04T15:47:59.2254437Z ........................................................................i........................... 6700/9583
2020-02-04T15:48:01.2880973Z .................................................................................................... 6800/9583
2020-02-04T15:48:03.4360884Z ..........................................................................i......................... 6900/9583
---
2020-02-04T15:49:37.8123349Z .................................................................................................... 7600/9583
2020-02-04T15:49:42.3091780Z .................................................................................................... 7700/9583
2020-02-04T15:49:48.7992536Z .................................................................................................... 7800/9583
2020-02-04T15:49:56.9018424Z .................................................................................................... 7900/9583
2020-02-04T15:50:03.8863707Z ...................................iiiiiii.i........................................................ 8000/9583
2020-02-04T15:50:17.8632648Z .................................................................................................... 8200/9583
2020-02-04T15:50:25.6395818Z .................................................................................................... 8300/9583
2020-02-04T15:50:38.7788726Z .................................................................................................... 8400/9583
2020-02-04T15:50:45.8568866Z .................................................................................................... 8500/9583
---
2020-02-04T15:52:34.8245151Z 
2020-02-04T15:52:34.8245554Z error[E0689]: can't call method `try_into` on ambiguous numeric type `{integer}`
2020-02-04T15:52:34.8245906Z   --> $DIR/numeric-cast-3.rs:5:38
2020-02-04T15:52:34.8246086Z    |
2020-02-04T15:52:34.8246227Z LL |     #[should_panic] let b: usize = 0.try_into().unwrap();
2020-02-04T15:52:34.8246661Z    |
2020-02-04T15:52:34.8247053Z help: you must specify a concrete type for this numeric value, like `i32`
2020-02-04T15:52:34.8247123Z    |
2020-02-04T15:52:34.8247123Z    |
2020-02-04T15:52:34.8247169Z LL |     #[should_panic] let b: usize = 0_i32.try_into().unwrap();
2020-02-04T15:52:34.8247268Z 
2020-02-04T15:52:34.8247309Z error: aborting due to 2 previous errors
2020-02-04T15:52:34.8247336Z 
2020-02-04T15:52:34.8247856Z For more information about this error, try `rustc --explain E0689`.
2020-02-04T15:52:34.8247856Z For more information about this error, try `rustc --explain E0689`.
2020-02-04T15:52:34.8247891Z 
2020-02-04T15:52:34.8247914Z 
2020-02-04T15:52:34.8247936Z 
2020-02-04T15:52:34.8247993Z The actual stderr differed from the expected stderr.
2020-02-04T15:52:34.8248282Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numeric/numeric-cast-3/numeric-cast-3.stderr
2020-02-04T15:52:34.8248510Z To update references, rerun the tests and pass the `--bless` flag
2020-02-04T15:52:34.8248773Z To only update this specific test, also pass `--test-args numeric/numeric-cast-3.rs`
2020-02-04T15:52:34.8248855Z error: 1 errors occurred comparing output.
2020-02-04T15:52:34.8248895Z status: exit code: 1
2020-02-04T15:52:34.8248895Z status: exit code: 1
2020-02-04T15:52:34.8249641Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numeric/numeric-cast-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numeric/numeric-cast-3" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numeric/numeric-cast-3/auxiliary" "-A" "unused"
2020-02-04T15:52:34.8249941Z ------------------------------------------
2020-02-04T15:52:34.8249971Z 
2020-02-04T15:52:34.8250172Z ------------------------------------------
2020-02-04T15:52:34.8250234Z stderr:
---
2020-02-04T15:52:34.8251357Z 
2020-02-04T15:52:34.8251603Z error[E0689]: can't call method `try_into` on ambiguous numeric type `{integer}`
2020-02-04T15:52:34.8251819Z   --> /checkout/src/test/ui/numeric/numeric-cast-3.rs:5:38
2020-02-04T15:52:34.8251871Z    |
2020-02-04T15:52:34.8251927Z LL |     #[should_panic] let b: usize = 0.try_into().unwrap();
2020-02-04T15:52:34.8252012Z    |
2020-02-04T15:52:34.8252072Z help: you must specify a concrete type for this numeric value, like `i32`
2020-02-04T15:52:34.8252112Z    |
2020-02-04T15:52:34.8252112Z    |
2020-02-04T15:52:34.8252309Z LL |     #[should_panic] let b: usize = 0_i32.try_into().unwrap();
2020-02-04T15:52:34.8252411Z 
2020-02-04T15:52:34.8252450Z error: aborting due to 2 previous errors
2020-02-04T15:52:34.8252476Z 
2020-02-04T15:52:34.8252753Z For more information about this error, try `rustc --explain E0689`.
---
2020-02-04T15:52:34.8266445Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-04T15:52:34.8266674Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-04T15:52:34.8276089Z 
2020-02-04T15:52:34.8276337Z 
2020-02-04T15:52:34.8278245Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-04T15:52:34.8278726Z 
2020-02-04T15:52:34.8278870Z 
2020-02-04T15:52:34.8285079Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-04T15:52:34.8285284Z Build completed unsuccessfully in 0:58:17
2020-02-04T15:52:34.8285284Z Build completed unsuccessfully in 0:58:17
2020-02-04T15:52:34.8336744Z == clock drift check ==
2020-02-04T15:52:34.8355624Z   local time: Tue Feb  4 15:52:34 UTC 2020
2020-02-04T15:52:35.1052097Z   network time: Tue, 04 Feb 2020 15:52:35 GMT
2020-02-04T15:52:35.1058557Z == end clock drift check ==
2020-02-04T15:52:35.4820732Z 
2020-02-04T15:52:35.4890432Z ##[error]Bash exited with code '1'.
2020-02-04T15:52:35.4906232Z ##[section]Finishing: Run build
2020-02-04T15:52:35.4937044Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68825/merge to s
2020-02-04T15:52:35.4939422Z Task         : Get sources
2020-02-04T15:52:35.4939472Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-04T15:52:35.4939539Z Version      : 1.0.0
2020-02-04T15:52:35.4939759Z Author       : Microsoft
2020-02-04T15:52:35.4939759Z Author       : Microsoft
2020-02-04T15:52:35.4939805Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-04T15:52:35.4939914Z ==============================================================================
2020-02-04T15:52:35.9170054Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-04T15:52:35.9208048Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68825/merge to s
2020-02-04T15:52:35.9310002Z Cleaning up task key
2020-02-04T15:52:35.9310631Z Start cleaning up orphan processes.
2020-02-04T15:52:35.9420486Z Terminate orphan process: pid (3745) (python)
2020-02-04T15:52:35.9667736Z ##[section]Finishing: Finalize Job
