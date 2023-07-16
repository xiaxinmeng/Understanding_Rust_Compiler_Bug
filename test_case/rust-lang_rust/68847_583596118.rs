plain
2020-02-07T19:07:57.7962302Z ========================== Starting Command Output ===========================
2020-02-07T19:07:57.7964272Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/875bcc45-f426-42cf-b09d-8c4ae4aaa168.sh
2020-02-07T19:07:57.7964314Z 
2020-02-07T19:07:57.7966856Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-07T19:07:57.7973797Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68847/merge to s
2020-02-07T19:07:57.7975355Z Task         : Get sources
2020-02-07T19:07:57.7975401Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-07T19:07:57.7975434Z Version      : 1.0.0
2020-02-07T19:07:57.7975466Z Author       : Microsoft
---
2020-02-07T19:07:58.7911728Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-07T19:07:58.7924267Z ##[command]git config gc.auto 0
2020-02-07T19:07:58.7927799Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-07T19:07:58.7930535Z ##[command]git config --get-all http.proxy
2020-02-07T19:07:58.7937870Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68847/merge:refs/remotes/pull/68847/merge
---
2020-02-07T20:13:36.2459267Z .................................................................................................... 1700/9609
2020-02-07T20:13:41.3631122Z .................................................................................................... 1800/9609
2020-02-07T20:13:54.7444544Z .............................i...................................................................... 1900/9609
2020-02-07T20:14:02.2864264Z .................................................................................................... 2000/9609
2020-02-07T20:14:17.9273957Z ...................iiiii............................................................................ 2100/9609
2020-02-07T20:14:28.8739496Z .................................................................................................... 2300/9609
2020-02-07T20:14:31.0759270Z .................................................................................................... 2400/9609
2020-02-07T20:14:36.2420712Z .................................................................................................... 2500/9609
2020-02-07T20:14:59.5662772Z .................................................................................................... 2600/9609
---
2020-02-07T20:17:50.2406029Z .....................................................................i...............i.............. 4900/9609
2020-02-07T20:17:58.6719606Z .................................................................................................... 5000/9609
2020-02-07T20:18:07.4614301Z .................................................................................................... 5100/9609
2020-02-07T20:18:12.4493776Z ............i....................................................................................... 5200/9609
2020-02-07T20:18:24.2495490Z ......................................................................................ii.ii........i 5300/9609
2020-02-07T20:18:32.4445547Z ........................i........................................................................... 5500/9609
2020-02-07T20:18:41.3215543Z .................................................................................................... 5600/9609
2020-02-07T20:18:50.3763367Z ..........................................................................i......................... 5700/9609
2020-02-07T20:18:58.5024624Z .................................................................................................... 5800/9609
2020-02-07T20:18:58.5024624Z .................................................................................................... 5800/9609
2020-02-07T20:19:05.6889357Z .................................................................................................... 5900/9609
2020-02-07T20:19:16.9197704Z .................................................................ii...i..ii...........i............. 6000/9609
2020-02-07T20:19:39.9997671Z .................................................................................................... 6200/9609
2020-02-07T20:19:48.3925443Z .................................................................................................... 6300/9609
2020-02-07T20:19:48.3925443Z .................................................................................................... 6300/9609
2020-02-07T20:19:57.3163760Z .............................................................................................i..ii.. 6400/9609
2020-02-07T20:20:27.6456522Z .................................................................................................... 6600/9609
2020-02-07T20:20:38.6969361Z ................................................................................i................... 6700/9609
2020-02-07T20:20:40.9975008Z .................................................................................................... 6800/9609
2020-02-07T20:20:43.3793437Z .......................................................................................i............ 6900/9609
---
2020-02-07T20:22:34.0521030Z .................................................................................................... 7600/9609
2020-02-07T20:22:39.2488726Z .................................................................................................... 7700/9609
2020-02-07T20:22:45.6005323Z .................................................................................................... 7800/9609
2020-02-07T20:22:54.5340287Z .........i..................F....................................................................... 7900/9609
2020-02-07T20:23:04.3766653Z ........................................................iiiiiii.i................................... 8000/9609
2020-02-07T20:23:20.2675296Z ...i................................................................................................ 8200/9609
2020-02-07T20:23:25.7918193Z .................................................................................................... 8300/9609
2020-02-07T20:23:42.2921613Z .................................................................................................... 8400/9609
2020-02-07T20:23:51.2357985Z .................................................................................................... 8500/9609
---
2020-02-07T20:26:01.2408793Z 
2020-02-07T20:26:01.2410050Z ---- [ui] ui/rfc-2632-const-trait-impl/stability.rs stdout ----
2020-02-07T20:26:01.2410433Z diff of stderr:
2020-02-07T20:26:01.2410680Z 
2020-02-07T20:26:01.2411625Z - error[E0723]: can only call other `const fn` within a `const fn`, but `const <Int as std::ops::Add>::add` is not stable as `const fn`
2020-02-07T20:26:01.2412120Z -   --> $DIR/stability.rs:21:5
2020-02-07T20:26:01.2412678Z + error[E0545]: `issue` must be a non-zero numeric string or "none"
2020-02-07T20:26:01.2413158Z +   --> $DIR/stability.rs:9:47
2020-02-07T20:26:01.2413417Z 3    |
2020-02-07T20:26:01.2413991Z - LL |     Int(1i32) + Int(2i32)
2020-02-07T20:26:01.2414468Z -    |     ^^^^^^^^^^^^^^^^^^^^^
2020-02-07T20:26:01.2414730Z + LL | #[rustc_const_unstable(feature = "const_add", issue = "0")]
2020-02-07T20:26:01.2415739Z +    |                                                       |
2020-02-07T20:26:01.2415739Z +    |                                                       |
2020-02-07T20:26:01.2416064Z +    |                                                       `issue` must not be "0", use "none" instead
2020-02-07T20:26:01.2416280Z + 
2020-02-07T20:26:01.2416734Z + error[E0545]: `issue` must be a non-zero numeric string or "none"
2020-02-07T20:26:01.2417404Z +   --> $DIR/stability.rs:27:41
2020-02-07T20:26:01.2417884Z 6    |
2020-02-07T20:26:01.2418624Z -    = note: for more information, see issue ***/issues/57563
2020-02-07T20:26:01.2419193Z -    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-07T20:26:01.2419496Z + LL | #[rustc_const_unstable(feature = "bar", issue = "0")]
2020-02-07T20:26:01.2420248Z +    |                                                 |
2020-02-07T20:26:01.2420248Z +    |                                                 |
2020-02-07T20:26:01.2420474Z +    |                                                 `issue` must not be "0", use "none" instead
2020-02-07T20:26:01.2421386Z - error: aborting due to previous error
2020-02-07T20:26:01.2421656Z + error: aborting due to 2 previous errors
2020-02-07T20:26:01.2421880Z 11 
2020-02-07T20:26:01.2422328Z - For more information about this error, try `rustc --explain E0723`.
2020-02-07T20:26:01.2422328Z - For more information about this error, try `rustc --explain E0723`.
2020-02-07T20:26:01.2422600Z 13 
2020-02-07T20:26:01.2422782Z 
2020-02-07T20:26:01.2422954Z 
2020-02-07T20:26:01.2423144Z The actual stderr differed from the expected stderr.
2020-02-07T20:26:01.2423679Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/stability/stability.stderr
2020-02-07T20:26:01.2424356Z To update references, rerun the tests and pass the `--bless` flag
2020-02-07T20:26:01.2426159Z To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/stability.rs`
2020-02-07T20:26:01.2426584Z error: 1 errors occurred comparing output.
2020-02-07T20:26:01.2426726Z status: exit code: 1
2020-02-07T20:26:01.2426726Z status: exit code: 1
2020-02-07T20:26:01.2428848Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/stability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/stability" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/stability/auxiliary" "-A" "unused"
2020-02-07T20:26:01.2430159Z ------------------------------------------
2020-02-07T20:26:01.2430956Z 
2020-02-07T20:26:01.2431863Z ------------------------------------------
2020-02-07T20:26:01.2432057Z stderr:
2020-02-07T20:26:01.2432057Z stderr:
2020-02-07T20:26:01.2432406Z ------------------------------------------
2020-02-07T20:26:01.2433839Z error[E0545]: `issue` must be a non-zero numeric string or "none"
2020-02-07T20:26:01.2434320Z   --> /checkout/src/test/ui/rfc-2632-const-trait-impl/stability.rs:9:47
2020-02-07T20:26:01.2435511Z    |
2020-02-07T20:26:01.2435693Z LL | #[rustc_const_unstable(feature = "const_add", issue = "0")]
2020-02-07T20:26:01.2436356Z    |                                                       |
2020-02-07T20:26:01.2436356Z    |                                                       |
2020-02-07T20:26:01.2436500Z    |                                                       `issue` must not be "0", use "none" instead
2020-02-07T20:26:01.2436852Z 
2020-02-07T20:26:01.2437580Z error[E0545]: `issue` must be a non-zero numeric string or "none"
2020-02-07T20:26:01.2438268Z   --> /checkout/src/test/ui/rfc-2632-const-trait-impl/stability.rs:27:41
2020-02-07T20:26:01.2438483Z    |
2020-02-07T20:26:01.2438653Z LL | #[rustc_const_unstable(feature = "bar", issue = "0")]
2020-02-07T20:26:01.2439910Z    |                                                 |
2020-02-07T20:26:01.2439910Z    |                                                 |
2020-02-07T20:26:01.2440085Z    |                                                 `issue` must not be "0", use "none" instead
2020-02-07T20:26:01.2440363Z error: aborting due to 2 previous errors
2020-02-07T20:26:01.2440480Z 
2020-02-07T20:26:01.2440594Z 
2020-02-07T20:26:01.2442516Z ------------------------------------------
---
2020-02-07T20:26:01.2444080Z test result: FAILED. 9555 passed; 1 failed; 53 ignored; 0 measured; 0 filtered out
2020-02-07T20:26:01.2444828Z 
2020-02-07T20:26:01.2450446Z 
2020-02-07T20:26:01.2450648Z 
2020-02-07T20:26:01.2452672Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-07T20:26:01.2453441Z 
2020-02-07T20:26:01.2453562Z 
2020-02-07T20:26:01.2453720Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-07T20:26:01.2453864Z Build completed unsuccessfully in 1:11:12
2020-02-07T20:26:01.2453864Z Build completed unsuccessfully in 1:11:12
2020-02-07T20:26:01.2454502Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-07T20:26:01.2454712Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-07T20:26:01.2505108Z == clock drift check ==
2020-02-07T20:26:01.2531260Z   local time: Fri Feb  7 20:26:01 UTC 2020
2020-02-07T20:26:01.8049500Z   network time: Fri, 07 Feb 2020 20:26:01 GMT
2020-02-07T20:26:01.8054878Z == end clock drift check ==
2020-02-07T20:26:02.2394681Z 
2020-02-07T20:26:02.2500532Z ##[error]Bash exited with code '1'.
2020-02-07T20:26:02.2515253Z ##[section]Finishing: Run build
2020-02-07T20:26:02.2536478Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68847/merge to s
2020-02-07T20:26:02.2538721Z Task         : Get sources
2020-02-07T20:26:02.2538769Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-07T20:26:02.2538836Z Version      : 1.0.0
2020-02-07T20:26:02.2538879Z Author       : Microsoft
2020-02-07T20:26:02.2538879Z Author       : Microsoft
2020-02-07T20:26:02.2538926Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-07T20:26:02.2538995Z ==============================================================================
2020-02-07T20:26:02.7335652Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-07T20:26:02.7379126Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68847/merge to s
2020-02-07T20:26:02.7507027Z Cleaning up task key
2020-02-07T20:26:02.7507774Z Start cleaning up orphan processes.
2020-02-07T20:26:02.7661798Z Terminate orphan process: pid (4062) (python)
2020-02-07T20:26:02.7936522Z ##[section]Finishing: Finalize Job
