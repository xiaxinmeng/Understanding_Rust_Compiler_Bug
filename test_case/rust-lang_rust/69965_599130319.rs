plain
2020-03-14T19:12:20.6863991Z ========================== Starting Command Output ===========================
2020-03-14T19:12:20.6868876Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9d264d47-fc87-4a68-9d6e-328efb07a223.sh
2020-03-14T19:12:20.6869404Z 
2020-03-14T19:12:20.6874345Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-14T19:12:20.6894778Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69965/merge to s
2020-03-14T19:12:20.6898243Z Task         : Get sources
2020-03-14T19:12:20.6898521Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-14T19:12:20.6898765Z Version      : 1.0.0
2020-03-14T19:12:20.6898933Z Author       : Microsoft
---
2020-03-14T19:12:21.6757994Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-14T19:12:21.6762545Z ##[command]git config gc.auto 0
2020-03-14T19:12:21.6765645Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-14T19:12:21.6768493Z ##[command]git config --get-all http.proxy
2020-03-14T19:12:21.6774060Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69965/merge:refs/remotes/pull/69965/merge
---
2020-03-14T20:08:19.8700205Z ..................................................................................................F. 1700/9771
2020-03-14T20:08:24.0398467Z .................................................................................................... 1800/9771
2020-03-14T20:08:34.8073652Z ...................................................................i................................ 1900/9771
2020-03-14T20:08:41.0175348Z .................................................................................................... 2000/9771
2020-03-14T20:08:54.1717807Z .........................................................iiiii...................................... 2100/9771
2020-03-14T20:09:03.8261967Z .................................................................................................... 2300/9771
2020-03-14T20:09:05.8109464Z .................................................................................................... 2400/9771
2020-03-14T20:09:08.5361194Z .................................................................................................... 2500/9771
2020-03-14T20:09:29.0545409Z .................................................................................................... 2600/9771
---
2020-03-14T20:11:56.6953703Z .............................i...............i...................................................... 5000/9771
2020-03-14T20:12:05.7355169Z .................................................................................................... 5100/9771
2020-03-14T20:12:11.4953126Z ........................................................................i........................... 5200/9771
2020-03-14T20:12:16.6793593Z .................................................................................................... 5300/9771
2020-03-14T20:12:25.7218111Z .....................................................ii.ii........i...i............................. 5400/9771
2020-03-14T20:12:33.0542957Z .................................................................................................... 5600/9771
2020-03-14T20:12:41.7544158Z .................................................................................................... 5700/9771
2020-03-14T20:12:47.3025257Z .............................................i...................................................... 5800/9771
2020-03-14T20:12:53.1749597Z .................................................................................................... 5900/9771
2020-03-14T20:12:53.1749597Z .................................................................................................... 5900/9771
2020-03-14T20:13:02.2617322Z .................................................................................................... 6000/9771
2020-03-14T20:13:07.6744681Z .......................................ii...i..ii...........i....................................... 6100/9771
2020-03-14T20:13:26.1521918Z .................................................................................................... 6300/9771
2020-03-14T20:13:32.4206062Z .................................................................................................... 6400/9771
2020-03-14T20:13:32.4206062Z .................................................................................................... 6400/9771
2020-03-14T20:13:41.0169691Z .....................................................................i..ii.......................... 6500/9771
2020-03-14T20:14:02.5115702Z .................................................................................................... 6700/9771
2020-03-14T20:14:10.3411673Z ...................................................................i................................ 6800/9771
2020-03-14T20:14:12.1144131Z .................................................................................................... 6900/9771
2020-03-14T20:14:14.0349604Z .................................................................................................... 7000/9771
---
2020-03-14T20:15:49.0126893Z .................................................................................................... 7800/9771
2020-03-14T20:15:54.3555343Z .................................................................................................... 7900/9771
2020-03-14T20:15:59.6288589Z ...................................................i................................................ 8000/9771
2020-03-14T20:16:09.1259913Z .................................................................................................... 8100/9771
2020-03-14T20:16:14.0090300Z iiiiiiiiii.i........................................................................................ 8200/9771
2020-03-14T20:16:26.7141140Z .................................................................................................... 8400/9771
2020-03-14T20:16:36.5794538Z .................................................................................................... 8500/9771
2020-03-14T20:16:48.0391957Z .................................................................................................... 8600/9771
2020-03-14T20:16:53.3172564Z .................................................................................................... 8700/9771
---
2020-03-14T20:18:34.2516663Z 24 
2020-03-14T20:18:34.2516920Z 
2020-03-14T20:18:34.2517177Z 
2020-03-14T20:18:34.2517552Z The actual stderr differed from the expected stderr.
2020-03-14T20:18:34.2518426Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/short-circuit.stock/short-circuit.stock.stderr
2020-03-14T20:18:34.2519319Z To update references, rerun the tests and pass the `--bless` flag
2020-03-14T20:18:34.2520150Z To only update this specific test, also pass `--test-args consts/control-flow/short-circuit.rs`
2020-03-14T20:18:34.2520599Z 
2020-03-14T20:18:34.2521298Z error in revision `stock`: 1 errors occurred comparing output.
2020-03-14T20:18:34.2521769Z status: exit code: 1
2020-03-14T20:18:34.2524148Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/control-flow/short-circuit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "stock" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/short-circuit.stock" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/short-circuit.stock/auxiliary"
2020-03-14T20:18:34.2528279Z ------------------------------------------
2020-03-14T20:18:34.2528436Z 
2020-03-14T20:18:34.2528747Z ------------------------------------------
2020-03-14T20:18:34.2529273Z stderr:
2020-03-14T20:18:34.2529273Z stderr:
2020-03-14T20:18:34.2529788Z ------------------------------------------
2020-03-14T20:18:34.2530054Z error: any use of this value will cause an error
2020-03-14T20:18:34.2530591Z   --> /checkout/src/test/ui/consts/control-flow/short-circuit.rs:10:25
2020-03-14T20:18:34.2530839Z    |
2020-03-14T20:18:34.2531134Z LL | const _: bool = true || panic!();  //[stock]~ ERROR any use of this value will cause an error
2020-03-14T20:18:34.2531886Z    |                         |
2020-03-14T20:18:34.2532717Z    |                         the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/consts/control-flow/short-circuit.rs:10:25
2020-03-14T20:18:34.2533123Z    |
2020-03-14T20:18:34.2533326Z    = note: `#[deny(const_err)]` on by default
2020-03-14T20:18:34.2533326Z    = note: `#[deny(const_err)]` on by default
2020-03-14T20:18:34.2534055Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-03-14T20:18:34.2541592Z 
2020-03-14T20:18:34.2541830Z error: any use of this value will cause an error
2020-03-14T20:18:34.2542513Z   --> /checkout/src/test/ui/consts/control-flow/short-circuit.rs:11:26
2020-03-14T20:18:34.2542815Z    |
2020-03-14T20:18:34.2543147Z LL | const _: bool = false && panic!(); //[stock]~ ERROR any use of this value will cause an error
2020-03-14T20:18:34.2543998Z    |                          |
2020-03-14T20:18:34.2544731Z    |                          the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/consts/control-flow/short-circuit.rs:11:26
2020-03-14T20:18:34.2545171Z    |
2020-03-14T20:18:34.2545748Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-03-14T20:18:34.2545748Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-03-14T20:18:34.2546077Z 
2020-03-14T20:18:34.2546289Z error: fatal error triggered by #[rustc_error]
2020-03-14T20:18:34.2546854Z   --> /checkout/src/test/ui/consts/control-flow/short-circuit.rs:14:1
2020-03-14T20:18:34.2547130Z    |
2020-03-14T20:18:34.2547414Z LL | fn main() {} //[if_match]~ ERROR fatal error triggered by #[rustc_error]
2020-03-14T20:18:34.2547869Z 
2020-03-14T20:18:34.2548070Z error: aborting due to 3 previous errors
2020-03-14T20:18:34.2548255Z 
2020-03-14T20:18:34.2548354Z 
---
2020-03-14T20:18:34.2551291Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-14T20:18:34.2551749Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-14T20:18:34.2552020Z 
2020-03-14T20:18:34.2552120Z 
2020-03-14T20:18:34.2556059Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-14T20:18:34.2559010Z 
2020-03-14T20:18:34.2559101Z 
2020-03-14T20:18:34.2559321Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-14T20:18:34.2559640Z Build completed unsuccessfully in 1:00:36
2020-03-14T20:18:34.2559640Z Build completed unsuccessfully in 1:00:36
2020-03-14T20:18:34.2599258Z == clock drift check ==
2020-03-14T20:18:34.2623087Z   local time: Sat Mar 14 20:18:34 UTC 2020
2020-03-14T20:18:34.5580113Z   network time: Sat, 14 Mar 2020 20:18:34 GMT
2020-03-14T20:18:34.5586097Z == end clock drift check ==
2020-03-14T20:18:35.0189892Z 
2020-03-14T20:18:35.0229301Z ##[error]Bash exited with code '1'.
2020-03-14T20:18:35.0243215Z ##[section]Finishing: Run build
2020-03-14T20:18:35.0290539Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69965/merge to s
2020-03-14T20:18:35.0295436Z Task         : Get sources
2020-03-14T20:18:35.0295758Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-14T20:18:35.0296058Z Version      : 1.0.0
2020-03-14T20:18:35.0296259Z Author       : Microsoft
2020-03-14T20:18:35.0296259Z Author       : Microsoft
2020-03-14T20:18:35.0296587Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-14T20:18:35.0296974Z ==============================================================================
2020-03-14T20:18:35.3565332Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-14T20:18:35.3609541Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69965/merge to s
2020-03-14T20:18:35.3688050Z Cleaning up task key
2020-03-14T20:18:35.3689354Z Start cleaning up orphan processes.
2020-03-14T20:18:35.3870914Z Terminate orphan process: pid (4122) (python)
2020-03-14T20:18:35.4040498Z ##[section]Finishing: Finalize Job
