plain
2020-01-27T16:18:02.7224755Z ========================== Starting Command Output ===========================
2020-01-27T16:18:02.7226774Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c62da082-4795-464a-881b-42917ccfb3e6.sh
2020-01-27T16:18:02.7226894Z 
2020-01-27T16:18:02.7229467Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-27T16:18:02.7235409Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68491/merge to s
2020-01-27T16:18:02.7237017Z Task         : Get sources
2020-01-27T16:18:02.7237052Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-27T16:18:02.7237087Z Version      : 1.0.0
2020-01-27T16:18:02.7237171Z Author       : Microsoft
---
2020-01-27T16:18:03.4886235Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-27T16:18:03.4898004Z ##[command]git config gc.auto 0
2020-01-27T16:18:03.4900328Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-27T16:18:03.4902199Z ##[command]git config --get-all http.proxy
2020-01-27T16:18:03.4908342Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68491/merge:refs/remotes/pull/68491/merge
---
2020-01-27T17:01:11.7137876Z .................................................................................................... 1700/9560
2020-01-27T17:01:16.3011378Z .................................................................................................... 1800/9560
2020-01-27T17:01:27.2484351Z ........................i........................................................................... 1900/9560
2020-01-27T17:01:33.4646877Z .................................................................................................... 2000/9560
2020-01-27T17:01:46.0753314Z ..............iiiii................................................................................. 2100/9560
2020-01-27T17:01:54.8512412Z .................................................................................................... 2300/9560
2020-01-27T17:01:57.0980845Z .................................................................................................... 2400/9560
2020-01-27T17:02:01.9563456Z .................................................................................................... 2500/9560
2020-01-27T17:02:19.3669053Z .................................................................................................... 2600/9560
---
2020-01-27T17:04:39.4055561Z .................................................................................................... 4800/9560
2020-01-27T17:04:44.0167922Z ..........................................................i...............i......................... 4900/9560
2020-01-27T17:04:51.2962145Z .................................................................................................... 5000/9560
2020-01-27T17:04:58.4533008Z .................................................................................................... 5100/9560
2020-01-27T17:05:02.9121015Z .i.................................................................................................. 5200/9560
2020-01-27T17:05:12.6488662Z ..........................................................................ii.ii........i...i........ 5300/9560
2020-01-27T17:05:20.3739680Z .............i...................................................................................... 5500/9560
2020-01-27T17:05:29.3072897Z .................................................................................................... 5600/9560
2020-01-27T17:05:35.0751212Z ..............................................................i..................................... 5700/9560
2020-01-27T17:05:41.5915015Z .................................................................................................... 5800/9560
2020-01-27T17:05:41.5915015Z .................................................................................................... 5800/9560
2020-01-27T17:05:48.6309701Z .................................................................................................... 5900/9560
2020-01-27T17:05:56.6681243Z .....................................................ii...i..ii...........i......................... 6000/9560
2020-01-27T17:06:18.0268278Z .................................................................................................... 6200/9560
2020-01-27T17:06:23.0355227Z .................................................................................................... 6300/9560
2020-01-27T17:06:27.4226841Z .................................................................................i...ii............. 6400/9560
2020-01-27T17:06:40.9643738Z .................................................................................................... 6500/9560
---
2020-01-27T17:08:40.8846379Z ............................................................................................F....... 7600/9560
2020-01-27T17:08:46.0099752Z .................................................................................................... 7700/9560
2020-01-27T17:08:52.6019469Z .................................................................................................... 7800/9560
2020-01-27T17:09:02.8818595Z .................................................................................................... 7900/9560
2020-01-27T17:09:08.8567243Z ...............iiiiiii.............................................................................. 8000/9560
2020-01-27T17:09:22.3896133Z .................................................................................................... 8200/9560
2020-01-27T17:09:32.0886770Z .................................................................................................... 8300/9560
2020-01-27T17:09:43.9008666Z .................................................................................................... 8400/9560
2020-01-27T17:09:50.0924184Z .................................................................................................... 8500/9560
---
2020-01-27T17:11:37.0714291Z 
2020-01-27T17:11:37.0714332Z 
2020-01-27T17:11:37.0714359Z 
2020-01-27T17:11:37.0714416Z The actual stderr differed from the expected stderr.
2020-01-27T17:11:37.0714725Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/repr-no-niche/repr-no-niche.stderr
2020-01-27T17:11:37.0714988Z To update references, rerun the tests and pass the `--bless` flag
2020-01-27T17:11:37.0715245Z To only update this specific test, also pass `--test-args repr/repr-no-niche.rs`
2020-01-27T17:11:37.0715345Z error: 1 errors occurred comparing output.
2020-01-27T17:11:37.0715394Z status: exit code: 0
2020-01-27T17:11:37.0715394Z status: exit code: 0
2020-01-27T17:11:37.0716340Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/repr/repr-no-niche.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/repr-no-niche/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/repr-no-niche/auxiliary"
2020-01-27T17:11:37.0716711Z ------------------------------------------
2020-01-27T17:11:37.0716745Z 
2020-01-27T17:11:37.0716978Z ------------------------------------------
2020-01-27T17:11:37.0717023Z stderr:
---
2020-01-27T17:11:37.0740893Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:349:22
2020-01-27T17:11:37.0741071Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-27T17:11:37.0756566Z 
2020-01-27T17:11:37.0756664Z 
2020-01-27T17:11:37.0758556Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-27T17:11:37.0758833Z 
2020-01-27T17:11:37.0758879Z 
2020-01-27T17:11:37.0764764Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-27T17:11:37.0764838Z Build completed unsuccessfully in 0:49:01
2020-01-27T17:11:37.0764838Z Build completed unsuccessfully in 0:49:01
2020-01-27T17:11:37.0825109Z == clock drift check ==
2020-01-27T17:11:37.0848604Z   local time: Mon Jan 27 17:11:37 UTC 2020
2020-01-27T17:11:37.6381938Z   network time: Mon, 27 Jan 2020 17:11:37 GMT
2020-01-27T17:11:37.6382175Z == end clock drift check ==
2020-01-27T17:11:38.2799560Z 
2020-01-27T17:11:38.2891564Z ##[error]Bash exited with code '1'.
2020-01-27T17:11:38.2904106Z ##[section]Finishing: Run build
2020-01-27T17:11:38.2923648Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68491/merge to s
2020-01-27T17:11:38.2925631Z Task         : Get sources
2020-01-27T17:11:38.2925698Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-27T17:11:38.2925748Z Version      : 1.0.0
2020-01-27T17:11:38.2925793Z Author       : Microsoft
2020-01-27T17:11:38.2925793Z Author       : Microsoft
2020-01-27T17:11:38.2925858Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-27T17:11:38.2925911Z ==============================================================================
2020-01-27T17:11:38.7224103Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-27T17:11:38.7265003Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68491/merge to s
2020-01-27T17:11:38.7389205Z Cleaning up task key
2020-01-27T17:11:38.7390593Z Start cleaning up orphan processes.
2020-01-27T17:11:38.7493116Z Terminate orphan process: pid (3613) (python)
2020-01-27T17:11:38.7776714Z ##[section]Finishing: Finalize Job
