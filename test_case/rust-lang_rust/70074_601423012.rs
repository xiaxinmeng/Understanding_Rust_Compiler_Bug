plain
2020-03-19T20:11:12.8603732Z ========================== Starting Command Output ===========================
2020-03-19T20:11:12.8610221Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/147e5826-1d09-4edc-8f11-b605ff5d7019.sh
2020-03-19T20:11:12.8610793Z 
2020-03-19T20:11:12.8617183Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-19T20:11:12.8639270Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70074/merge to s
2020-03-19T20:11:12.8643022Z Task         : Get sources
2020-03-19T20:11:12.8643321Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T20:11:12.8643611Z Version      : 1.0.0
2020-03-19T20:11:12.8643808Z Author       : Microsoft
---
2020-03-19T20:11:14.0295866Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-19T20:11:14.0304677Z ##[command]git config gc.auto 0
2020-03-19T20:11:14.0310354Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-19T20:11:14.0314603Z ##[command]git config --get-all http.proxy
2020-03-19T20:11:14.0323718Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70074/merge:refs/remotes/pull/70074/merge
---
2020-03-19T21:10:29.3507536Z .................................................................................................... 1700/9805
2020-03-19T21:10:33.6540492Z .................................................................................................... 1800/9805
2020-03-19T21:10:45.0966251Z ..........................................................................i......................... 1900/9805
2020-03-19T21:10:51.4280538Z .................................................................................................... 2000/9805
2020-03-19T21:10:59.4790822Z ................................................................iiiii............................... 2100/9805
2020-03-19T21:11:17.5641462Z .................................................................................................... 2300/9805
2020-03-19T21:11:19.8024800Z .................................................................................................... 2400/9805
2020-03-19T21:11:22.7056398Z .................................................................................................... 2500/9805
2020-03-19T21:11:42.3760580Z .................................................................................................... 2600/9805
---
2020-03-19T21:14:18.5129244Z .....................................i...............i.............................................. 5000/9805
2020-03-19T21:14:27.1965740Z .................................................................................................... 5100/9805
2020-03-19T21:14:33.7515866Z ................................................................................i................... 5200/9805
2020-03-19T21:14:39.1115650Z .................................................................................................... 5300/9805
2020-03-19T21:14:49.2975030Z .............................................................ii.ii........i...i..................... 5400/9805
2020-03-19T21:14:57.0675387Z i................................................................................................... 5600/9805
2020-03-19T21:15:06.2640373Z .....i.............................................................................................. 5700/9805
2020-03-19T21:15:12.5038174Z .........................................................i.......................................... 5800/9805
2020-03-19T21:15:18.9351749Z .................................................................................................... 5900/9805
2020-03-19T21:15:18.9351749Z .................................................................................................... 5900/9805
2020-03-19T21:15:26.5934137Z .................................................................................................... 6000/9805
2020-03-19T21:15:34.6427787Z ...................................................ii...i..ii...........i........................... 6100/9805
2020-03-19T21:15:54.3931339Z .................................................................................................... 6300/9805
2020-03-19T21:16:01.1277440Z .................................................................................................... 6400/9805
2020-03-19T21:16:01.1277440Z .................................................................................................... 6400/9805
2020-03-19T21:16:08.4403279Z .................................................................................i..ii.............. 6500/9805
2020-03-19T21:16:32.7762831Z .................................................................................................... 6700/9805
2020-03-19T21:16:42.4024467Z ................................................................................i................... 6800/9805
2020-03-19T21:16:44.5999715Z .................................................................................................... 6900/9805
2020-03-19T21:16:46.6962278Z .................................................................................................... 7000/9805
---
2020-03-19T21:18:29.6692183Z .................................................................................................... 7800/9805
2020-03-19T21:18:34.9248922Z .................................................................................................... 7900/9805
2020-03-19T21:18:40.8626600Z ....................................................................i............................... 8000/9805
2020-03-19T21:18:50.5785850Z .................................................................................................... 8100/9805
2020-03-19T21:18:56.0121947Z .................iiiiiiiiii.i....................................................................... 8200/9805
2020-03-19T21:19:10.4364934Z ...........................................................................................F........ 8400/9805
2020-03-19T21:19:16.7293667Z .................................................................................................... 8500/9805
2020-03-19T21:19:31.9233700Z .................................................................................................... 8600/9805
2020-03-19T21:19:39.3603026Z .................................................................................................... 8700/9805
---
2020-03-19T21:21:38.0411114Z - error: aborting due to previous error
2020-03-19T21:21:38.0411485Z + error[E0601]: `main` function not found in crate `transitive_dep_span`
2020-03-19T21:21:38.0412122Z +   --> $DIR/transitive-dep-span.rs:11:1
2020-03-19T21:21:38.0412366Z +    |
2020-03-19T21:21:38.0412589Z + LL | / extern crate transitive_dep_two;
2020-03-19T21:21:38.0412834Z + LL | |
2020-03-19T21:21:38.0413084Z + LL | | transitive_dep_two::parse_error!();
2020-03-19T21:21:38.0413807Z +    | |___________________________________^ consider adding a `main` function to `$DIR/transitive-dep-span.rs`
2020-03-19T21:21:38.0414405Z + error: aborting due to 2 previous errors
2020-03-19T21:21:38.0414622Z + 
2020-03-19T21:21:38.0415136Z + For more information about this error, try `rustc --explain E0601`.
2020-03-19T21:21:38.0415410Z 20 
2020-03-19T21:21:38.0415410Z 20 
2020-03-19T21:21:38.0415526Z 
2020-03-19T21:21:38.0415633Z 
2020-03-19T21:21:38.0415882Z The actual stderr differed from the expected stderr.
2020-03-19T21:21:38.0416655Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/transitive-dep-span/transitive-dep-span.stderr
2020-03-19T21:21:38.0417390Z To update references, rerun the tests and pass the `--bless` flag
2020-03-19T21:21:38.0418077Z To only update this specific test, also pass `--test-args span/transitive-dep-span.rs`
2020-03-19T21:21:38.0423457Z error: 1 errors occurred comparing output.
2020-03-19T21:21:38.0423774Z status: exit code: 1
2020-03-19T21:21:38.0423774Z status: exit code: 1
2020-03-19T21:21:38.0426188Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/transitive-dep-span.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/transitive-dep-span" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Z" "macro-backtrace" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/transitive-dep-span/auxiliary"
2020-03-19T21:21:38.0428390Z ------------------------------------------
2020-03-19T21:21:38.0428591Z 
2020-03-19T21:21:38.0429000Z ------------------------------------------
2020-03-19T21:21:38.0429246Z stderr:
2020-03-19T21:21:38.0429246Z stderr:
2020-03-19T21:21:38.0429663Z ------------------------------------------
2020-03-19T21:21:38.0429993Z error: expected one of `!` or `::`, found `error`
2020-03-19T21:21:38.0430639Z   --> /checkout/src/test/ui/span/auxiliary/transitive_dep_three.rs:6:27
2020-03-19T21:21:38.0431890Z LL | /         macro_rules! parse_error {
2020-03-19T21:21:38.0432209Z LL | |             () => { parse error }
2020-03-19T21:21:38.0432586Z    | |                           ^^^^^ expected one of `!` or `::`
2020-03-19T21:21:38.0432911Z LL | |         }
2020-03-19T21:21:38.0432911Z LL | |         }
2020-03-19T21:21:38.0433547Z    | |_________- in this expansion of `transitive_dep_two::parse_error!`
2020-03-19T21:21:38.0434357Z   ::: /checkout/src/test/ui/span/transitive-dep-span.rs:13:1
2020-03-19T21:21:38.0434662Z    |
2020-03-19T21:21:38.0434662Z    |
2020-03-19T21:21:38.0435081Z LL |   transitive_dep_two::parse_error!(); //~ ERROR expected one of
2020-03-19T21:21:38.0435937Z    |   |
2020-03-19T21:21:38.0436143Z    |   in this macro invocation
2020-03-19T21:21:38.0436393Z    |   in this macro invocation
2020-03-19T21:21:38.0436561Z 
2020-03-19T21:21:38.0436561Z 
2020-03-19T21:21:38.0436852Z error[E0601]: `main` function not found in crate `transitive_dep_span`
2020-03-19T21:21:38.0437475Z   --> /checkout/src/test/ui/span/transitive-dep-span.rs:11:1
2020-03-19T21:21:38.0437745Z    |
2020-03-19T21:21:38.0437976Z LL | / extern crate transitive_dep_two;
2020-03-19T21:21:38.0438191Z LL | |
2020-03-19T21:21:38.0438479Z LL | | transitive_dep_two::parse_error!(); //~ ERROR expected one of
2020-03-19T21:21:38.0439318Z    | |___________________________________^ consider adding a `main` function to `/checkout/src/test/ui/span/transitive-dep-span.rs`
2020-03-19T21:21:38.0439910Z error: aborting due to 2 previous errors
2020-03-19T21:21:38.0440123Z 
2020-03-19T21:21:38.0440613Z For more information about this error, try `rustc --explain E0601`.
2020-03-19T21:21:38.0440858Z 
---
2020-03-19T21:21:38.0443020Z test result: FAILED. 9746 passed; 1 failed; 58 ignored; 0 measured; 0 filtered out
2020-03-19T21:21:38.0443322Z 
2020-03-19T21:21:38.0446703Z 
2020-03-19T21:21:38.0447247Z 
2020-03-19T21:21:38.0457758Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-19T21:21:38.0511742Z 
2020-03-19T21:21:38.0512323Z 
2020-03-19T21:21:38.0513254Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-19T21:21:38.0514152Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-19T21:21:38.0514152Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-19T21:21:38.0515159Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-19T21:21:38.0515525Z Build completed unsuccessfully in 1:06:07
2020-03-19T21:21:38.0531366Z == clock drift check ==
2020-03-19T21:21:38.0550581Z   local time: Thu Mar 19 21:21:38 UTC 2020
2020-03-19T21:21:38.3521803Z   network time: Thu, 19 Mar 2020 21:21:38 GMT
2020-03-19T21:21:38.3522176Z == end clock drift check ==
2020-03-19T21:21:38.7757871Z 
2020-03-19T21:21:38.7828603Z ##[error]Bash exited with code '1'.
2020-03-19T21:21:38.7845067Z ##[section]Finishing: Run build
2020-03-19T21:21:38.7905049Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70074/merge to s
2020-03-19T21:21:38.7912265Z Task         : Get sources
2020-03-19T21:21:38.7912652Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T21:21:38.7913027Z Version      : 1.0.0
2020-03-19T21:21:38.7913277Z Author       : Microsoft
2020-03-19T21:21:38.7913277Z Author       : Microsoft
2020-03-19T21:21:38.7913676Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-19T21:21:38.7914159Z ==============================================================================
2020-03-19T21:21:39.1178372Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-19T21:21:39.1223432Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70074/merge to s
2020-03-19T21:21:39.1305271Z Cleaning up task key
2020-03-19T21:21:39.1306335Z Start cleaning up orphan processes.
2020-03-19T21:21:39.1474892Z Terminate orphan process: pid (3489) (python)
2020-03-19T21:21:39.1627837Z ##[section]Finishing: Finalize Job
