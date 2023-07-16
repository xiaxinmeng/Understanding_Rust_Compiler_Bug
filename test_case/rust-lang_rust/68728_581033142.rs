plain
2020-02-01T12:51:48.9765256Z ========================== Starting Command Output ===========================
2020-02-01T12:51:48.9766896Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b7e87c55-1875-4696-a3f3-f85531c42751.sh
2020-02-01T12:51:48.9766935Z 
2020-02-01T12:51:48.9769633Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-01T12:51:48.9777285Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68728/merge to s
2020-02-01T12:51:48.9779132Z Task         : Get sources
2020-02-01T12:51:48.9779169Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-01T12:51:48.9779260Z Version      : 1.0.0
2020-02-01T12:51:48.9779298Z Author       : Microsoft
---
2020-02-01T12:51:49.9763966Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-01T12:51:49.9774072Z ##[command]git config gc.auto 0
2020-02-01T12:51:49.9776348Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-01T12:51:49.9778220Z ##[command]git config --get-all http.proxy
2020-02-01T12:51:49.9784751Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68728/merge:refs/remotes/pull/68728/merge
---
2020-02-01T13:50:05.2455433Z .................................................................................................... 1700/9567
2020-02-01T13:50:10.2155348Z .................................................................................................... 1800/9567
2020-02-01T13:50:23.0111035Z .........................i.......................................................................... 1900/9567
2020-02-01T13:50:30.0814774Z .................................................................................................... 2000/9567
2020-02-01T13:50:44.9246092Z ...............iiiii................................................................................ 2100/9567
2020-02-01T13:50:54.6643679Z .................................................................................................... 2300/9567
2020-02-01T13:50:57.0778971Z .................................................................................................... 2400/9567
2020-02-01T13:51:02.2282509Z .................................................................................................... 2500/9567
2020-02-01T13:51:23.4879478Z .................................................................................................... 2600/9567
---
2020-02-01T13:53:58.0224751Z .................................................................................................... 4800/9567
2020-02-01T13:54:03.1079474Z ..........................................................i...............i......................... 4900/9567
2020-02-01T13:54:10.9485102Z .................................................................................................... 5000/9567
2020-02-01T13:54:19.0951882Z .................................................................................................... 5100/9567
2020-02-01T13:54:23.9115736Z .i.................................................................................................. 5200/9567
2020-02-01T13:54:34.8379506Z ..........................................................................ii.ii........i...i........ 5300/9567
2020-02-01T13:54:43.3428878Z ............i....................................................................................... 5500/9567
2020-02-01T13:54:53.5392146Z .................................................................................................... 5600/9567
2020-02-01T13:54:59.8747915Z .............................................................i...................................... 5700/9567
2020-02-01T13:55:07.0139358Z .................................................................................................... 5800/9567
2020-02-01T13:55:07.0139358Z .................................................................................................... 5800/9567
2020-02-01T13:55:15.1292288Z .................................................................................................... 5900/9567
2020-02-01T13:55:24.1128976Z ....................................................ii...i..ii............i......................... 6000/9567
2020-02-01T13:55:45.9237861Z .................................................................................................... 6200/9567
2020-02-01T13:55:53.5251141Z .................................................................................................... 6300/9567
2020-02-01T13:55:53.5251141Z .................................................................................................... 6300/9567
2020-02-01T13:56:02.5933289Z ................................................................................i..ii............... 6400/9567
2020-02-01T13:56:33.7686592Z .................................................................................................... 6600/9567
2020-02-01T13:56:39.5292762Z ........................................................i........................................... 6700/9567
2020-02-01T13:56:41.6719748Z .................................................................................................... 6800/9567
2020-02-01T13:56:43.9656809Z ..............................................................i..................................... 6900/9567
---
2020-02-01T13:58:26.2488273Z .................................................................................................... 7600/9567
2020-02-01T13:58:31.3885255Z .................................................................................................... 7700/9567
2020-02-01T13:58:38.0411553Z .................................................................................................... 7800/9567
2020-02-01T13:58:48.3509946Z .................................................................................................... 7900/9567
2020-02-01T13:58:54.9150418Z ....................iiiiiii.i....................................................................... 8000/9567
2020-02-01T13:59:09.5403696Z .................................................................................................... 8200/9567
2020-02-01T13:59:18.8227624Z .................................................................................................... 8300/9567
2020-02-01T13:59:32.8700335Z .................................................................................................... 8400/9567
2020-02-01T13:59:40.2949064Z .................................................................................................... 8500/9567
---
2020-02-01T14:02:04.4587169Z  finished in 7.305
2020-02-01T14:02:04.4759939Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-01T14:02:04.6375314Z 
2020-02-01T14:02:04.6377155Z running 169 tests
2020-02-01T14:02:07.7210509Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/169
2020-02-01T14:02:10.0014558Z i.i.i...iii..iiiiiiiiii.......................iii............ii......
2020-02-01T14:02:10.0015444Z 
2020-02-01T14:02:10.0019288Z  finished in 5.526
2020-02-01T14:02:10.0199782Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-01T14:02:10.1877575Z 
---
2020-02-01T14:02:12.1934849Z  finished in 2.173
2020-02-01T14:02:12.2122035Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-01T14:02:12.3707258Z 
2020-02-01T14:02:12.3707514Z running 9 tests
2020-02-01T14:02:12.3708298Z iiiiiiiii
2020-02-01T14:02:12.3708721Z 
2020-02-01T14:02:12.3708766Z  finished in 0.158
2020-02-01T14:02:12.3893624Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-01T14:02:12.5545235Z 
---
2020-02-01T14:02:32.8872379Z  finished in 20.497
2020-02-01T14:02:32.9060224Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-01T14:02:33.0749787Z 
2020-02-01T14:02:33.0754386Z running 116 tests
2020-02-01T14:02:46.7056417Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-01T14:02:48.5966627Z ....iiii.....ii.
2020-02-01T14:02:48.5969342Z 
2020-02-01T14:02:48.5974207Z  finished in 15.691
2020-02-01T14:02:48.5979048Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-01T14:02:48.5979442Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-02-01T14:03:17.8956141Z ---- [ui] ui-fulldeps/pprust-expr-roundtrip.rs stdout ----
2020-02-01T14:03:17.8956185Z 
2020-02-01T14:03:17.8956477Z error: test compilation failed although it shouldn't!
2020-02-01T14:03:17.8956535Z status: exit code: 1
2020-02-01T14:03:17.8957423Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/auxiliary"
2020-02-01T14:03:17.8957801Z ------------------------------------------
2020-02-01T14:03:17.8958065Z 
2020-02-01T14:03:17.8958364Z ------------------------------------------
2020-02-01T14:03:17.8958419Z stderr:
2020-02-01T14:03:17.8958419Z stderr:
2020-02-01T14:03:17.8958675Z ------------------------------------------
2020-02-01T14:03:17.8958737Z error[E0433]: failed to resolve: use of undeclared type or module `IsAsync`
2020-02-01T14:03:17.8959026Z   --> /checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs:122:43
2020-02-01T14:03:17.8959104Z    |
2020-02-01T14:03:17.8959157Z LL | ...                   IsAsync::NotAsync,
2020-02-01T14:03:17.8959216Z    |                       ^^^^^^^ use of undeclared type or module `IsAsync`
2020-02-01T14:03:17.8959465Z error: aborting due to previous error
2020-02-01T14:03:17.8959510Z 
2020-02-01T14:03:17.8959832Z For more information about this error, try `rustc --explain E0433`.
2020-02-01T14:03:17.8959872Z 
---
2020-02-01T14:03:17.8975279Z test result: FAILED. 62 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-02-01T14:03:17.8975323Z 
2020-02-01T14:03:17.8975357Z 
2020-02-01T14:03:17.8975388Z 
2020-02-01T14:03:17.8976899Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unkthread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-01T14:03:17.8977087Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-01T14:03:17.8977703Z nown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-01T14:03:17.8977860Z 
2020-02-01T14:03:17.8977892Z 
2020-02-01T14:03:17.8982487Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-01T14:03:17.8982567Z Build completed unsuccessfully in 1:05:44
2020-02-01T14:03:17.8982567Z Build completed unsuccessfully in 1:05:44
2020-02-01T14:03:17.9035446Z == clock drift check ==
2020-02-01T14:03:17.9057705Z   local time: Sat Feb  1 14:03:17 UTC 2020
2020-02-01T14:03:18.4577151Z   network time: Sat, 01 Feb 2020 14:03:18 GMT
2020-02-01T14:03:18.4580830Z == end clock drift check ==
2020-02-01T14:03:19.3706379Z 
2020-02-01T14:03:19.3840399Z ##[error]Bash exited with code '1'.
2020-02-01T14:03:19.3853407Z ##[section]Finishing: Run build
2020-02-01T14:03:19.3875369Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68728/merge to s
2020-02-01T14:03:19.3877232Z Task         : Get sources
2020-02-01T14:03:19.3877281Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-01T14:03:19.3877482Z Version      : 1.0.0
2020-02-01T14:03:19.3877524Z Author       : Microsoft
2020-02-01T14:03:19.3877524Z Author       : Microsoft
2020-02-01T14:03:19.3877571Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-01T14:03:19.3877637Z ==============================================================================
2020-02-01T14:03:19.8671368Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-01T14:03:19.8714002Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68728/merge to s
2020-02-01T14:03:19.8823852Z Cleaning up task key
2020-02-01T14:03:19.8824628Z Start cleaning up orphan processes.
2020-02-01T14:03:19.8934482Z Terminate orphan process: pid (4016) (python)
2020-02-01T14:03:19.9186940Z ##[section]Finishing: Finalize Job
