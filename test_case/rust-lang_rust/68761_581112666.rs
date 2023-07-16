plain
2020-02-02T07:41:10.1799675Z ========================== Starting Command Output ===========================
2020-02-02T07:41:10.1801258Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/36e45b17-43a3-43c4-909c-c5101c34c516.sh
2020-02-02T07:41:10.1801300Z 
2020-02-02T07:41:10.1804267Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-02T07:41:10.1809549Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68761/merge to s
2020-02-02T07:41:10.1811759Z Task         : Get sources
2020-02-02T07:41:10.1811787Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-02T07:41:10.1811815Z Version      : 1.0.0
2020-02-02T07:41:10.1811842Z Author       : Microsoft
---
2020-02-02T07:41:11.0876278Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-02T07:41:11.0953553Z ##[command]git config gc.auto 0
2020-02-02T07:41:11.1034588Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-02T07:41:11.1090540Z ##[command]git config --get-all http.proxy
2020-02-02T07:41:11.1224218Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68761/merge:refs/remotes/pull/68761/merge
---
2020-02-02T08:35:44.7460823Z .................................................................................................... 1700/9563
2020-02-02T08:35:49.6715332Z .................................................................................................... 1800/9563
2020-02-02T08:36:01.8355310Z .........................i.......................................................................... 1900/9563
2020-02-02T08:36:08.7564826Z .................................................................................................... 2000/9563
2020-02-02T08:36:22.7636611Z ...............iiiii................................................................................ 2100/9563
2020-02-02T08:36:32.1992907Z .................................................................................................... 2300/9563
2020-02-02T08:36:34.5351623Z .................................................................................................... 2400/9563
2020-02-02T08:36:39.4544393Z .................................................................................................... 2500/9563
2020-02-02T08:36:59.5362356Z .................................................................................................... 2600/9563
---
2020-02-02T08:39:27.6579908Z .................................................................................................... 4800/9563
2020-02-02T08:39:32.5765638Z ..........................................................i...............i......................... 4900/9563
2020-02-02T08:39:40.1130014Z .................................................................................................... 5000/9563
2020-02-02T08:39:47.7521613Z .................................................................................................... 5100/9563
2020-02-02T08:39:52.4132873Z .i.................................................................................................. 5200/9563
2020-02-02T08:40:02.9935037Z ...........................................................................ii.ii........i...i....... 5300/9563
2020-02-02T08:40:11.3541485Z .............i...................................................................................... 5500/9563
2020-02-02T08:40:20.9771762Z .................................................................................................... 5600/9563
2020-02-02T08:40:27.0526212Z ..............................................................i..................................... 5700/9563
2020-02-02T08:40:33.8709196Z .................................................................................................... 5800/9563
2020-02-02T08:40:33.8709196Z .................................................................................................... 5800/9563
2020-02-02T08:40:41.0956158Z .................................................................................................... 5900/9563
2020-02-02T08:40:49.5869043Z .....................................................ii...i..ii...........i......................... 6000/9563
2020-02-02T08:41:10.5633782Z .................................................................................................... 6200/9563
2020-02-02T08:41:17.8907693Z .................................................................................................... 6300/9563
2020-02-02T08:41:17.8907693Z .................................................................................................... 6300/9563
2020-02-02T08:41:26.0351732Z .................................................................................i..ii.............. 6400/9563
2020-02-02T08:41:56.0184336Z .................................................................................................... 6600/9563
2020-02-02T08:42:01.2811919Z .........................................................i.......................................... 6700/9563
2020-02-02T08:42:03.3559257Z .................................................................................................... 6800/9563
2020-02-02T08:42:05.5685534Z ..........................................................i......................................... 6900/9563
---
2020-02-02T08:43:42.9068609Z .................................................................................................... 7600/9563
2020-02-02T08:43:47.8974350Z .................................................................................................... 7700/9563
2020-02-02T08:43:54.3749197Z .................................................................................................... 7800/9563
2020-02-02T08:44:04.6065694Z .................................................................................................... 7900/9563
2020-02-02T08:44:10.4488581Z ................iiiiiii.i........................................................................... 8000/9563
2020-02-02T08:44:24.3558605Z .................................................................................................... 8200/9563
2020-02-02T08:44:33.8968749Z .................................................................................................... 8300/9563
2020-02-02T08:44:46.9372506Z .................................................................................................... 8400/9563
2020-02-02T08:44:53.8545553Z .................................................................................................... 8500/9563
---
2020-02-02T08:46:45.8393482Z 
2020-02-02T08:46:45.8394273Z ---- [ui] ui/try-block/try-block-bad-type.rs stdout ----
2020-02-02T08:46:45.8394696Z diff of stderr:
2020-02-02T08:46:45.8394856Z 
2020-02-02T08:46:45.8395015Z 10              <i32 as std::convert::From<i16>>
2020-02-02T08:46:45.8395153Z 11              <i32 as std::convert::From<i8>>
2020-02-02T08:46:45.8395292Z 12              <i32 as std::convert::From<std::num::NonZeroI32>>
2020-02-02T08:46:45.8396684Z +            and 3 others
2020-02-02T08:46:45.8396836Z 14    = note: required by `std::convert::From::from`
2020-02-02T08:46:45.8397073Z 15 
2020-02-02T08:46:45.8397073Z 15 
2020-02-02T08:46:45.8397379Z 16 error[E0271]: type mismatch resolving `<std::result::Result<i32, i32> as std::ops::Try>::Ok == &str`
2020-02-02T08:46:45.8397842Z 
2020-02-02T08:46:45.8398141Z The actual stderr differed from the expected stderr.
2020-02-02T08:46:45.8398608Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/try-block-bad-type.stderr
2020-02-02T08:46:45.8398608Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/try-block-bad-type.stderr
2020-02-02T08:46:45.8399373Z To update references, rerun the tests and pass the `--bless` flag
2020-02-02T08:46:45.8400120Z To only update this specific test, also pass `--test-args try-block/try-block-bad-type.rs`
2020-02-02T08:46:45.8400432Z error: 1 errors occurred comparing output.
2020-02-02T08:46:45.8400562Z status: exit code: 1
2020-02-02T08:46:45.8400562Z status: exit code: 1
2020-02-02T08:46:45.8401701Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-block/try-block-bad-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/auxiliary" "-A" "unused"
2020-02-02T08:46:45.8402325Z ------------------------------------------
2020-02-02T08:46:45.8402486Z 
2020-02-02T08:46:45.8402815Z ------------------------------------------
2020-02-02T08:46:45.8403000Z stderr:
2020-02-02T08:46:45.8403000Z stderr:
2020-02-02T08:46:45.8403499Z ------------------------------------------
2020-02-02T08:46:45.8403890Z error[E0277]: `?` couldn't convert the error to `i32`
2020-02-02T08:46:45.8405163Z    |
2020-02-02T08:46:45.8405163Z    |
2020-02-02T08:46:45.8405487Z LL |         Err("")?; //~ ERROR `?` couldn't convert the error
2020-02-02T08:46:45.8405552Z    |                ^ the trait `std::convert::From<&str>` is not implemented for `i32`
2020-02-02T08:46:45.8405596Z    |
2020-02-02T08:46:45.8405664Z    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
2020-02-02T08:46:45.8405721Z    = help: the following implementations were found:
2020-02-02T08:46:45.8405785Z              <i32 as std::convert::From<bool>>
2020-02-02T08:46:45.8405828Z              <i32 as std::convert::From<i16>>
2020-02-02T08:46:45.8405870Z              <i32 as std::convert::From<i8>>
2020-02-02T08:46:45.8405915Z              <i32 as std::convert::From<std::num::NonZeroI32>>
2020-02-02T08:46:45.8406016Z    = note: required by `std::convert::From::from`
2020-02-02T08:46:45.8406045Z 
2020-02-02T08:46:45.8406045Z 
2020-02-02T08:46:45.8406323Z error[E0271]: type mismatch resolving `<std::result::Result<i32, i32> as std::ops::Try>::Ok == &str`
2020-02-02T08:46:45.8407009Z    |
2020-02-02T08:46:45.8407066Z LL |         "" //~ ERROR type mismatch
2020-02-02T08:46:45.8407066Z LL |         "" //~ ERROR type mismatch
2020-02-02T08:46:45.8407107Z    |         ^^ expected `i32`, found `&str`
2020-02-02T08:46:45.8407133Z 
2020-02-02T08:46:45.8407300Z error[E0271]: type mismatch resolving `<std::result::Result<i32, i32> as std::ops::Try>::Ok == ()`
2020-02-02T08:46:45.8407662Z    |
2020-02-02T08:46:45.8407662Z    |
2020-02-02T08:46:45.8407707Z LL |     let res: Result<i32, i32> = try { }; //~ ERROR type mismatch
2020-02-02T08:46:45.8407774Z    |                                       ^ expected `i32`, found `()`
2020-02-02T08:46:45.8407847Z error[E0277]: the trait bound `(): std::ops::Try` is not satisfied
2020-02-02T08:46:45.8408084Z   --> /checkout/src/test/ui/try-block/try-block-bad-type.rs:17:23
2020-02-02T08:46:45.8408148Z    |
2020-02-02T08:46:45.8408148Z    |
2020-02-02T08:46:45.8408193Z LL |     let res: () = try { }; //~ the trait bound `(): std::ops::Try` is not satisfied
2020-02-02T08:46:45.8408393Z    |
2020-02-02T08:46:45.8408434Z    = note: required by `std::ops::Try::from_ok`
2020-02-02T08:46:45.8408463Z 
2020-02-02T08:46:45.8408533Z error[E0277]: the trait bound `i32: std::ops::Try` is not satisfied
2020-02-02T08:46:45.8408533Z error[E0277]: the trait bound `i32: std::ops::Try` is not satisfied
2020-02-02T08:46:45.8408800Z   --> /checkout/src/test/ui/try-block/try-block-bad-type.rs:19:24
2020-02-02T08:46:45.8408845Z    |
2020-02-02T08:46:45.8408892Z LL |     let res: i32 = try { 5 }; //~ ERROR the trait bound `i32: std::ops::Try` is not satisfied
2020-02-02T08:46:45.8408962Z    |                        ^^^^^ the trait `std::ops::Try` is not implemented for `i32`
2020-02-02T08:46:45.8409047Z    = note: required by `std::ops::Try::from_ok`
2020-02-02T08:46:45.8409093Z 
2020-02-02T08:46:45.8409134Z error: aborting due to 5 previous errors
2020-02-02T08:46:45.8409169Z 
---
2020-02-02T08:46:45.8424338Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-02T08:46:45.8424430Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-02T08:46:45.8436907Z 
2020-02-02T08:46:45.8437132Z 
2020-02-02T08:46:45.8439428Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-02T08:46:45.8439806Z 
2020-02-02T08:46:45.8439854Z 
2020-02-02T08:46:45.8447861Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-02T08:46:45.8447923Z Build completed unsuccessfully in 0:59:50
2020-02-02T08:46:45.8447923Z Build completed unsuccessfully in 0:59:50
2020-02-02T08:46:45.8502651Z == clock drift check ==
2020-02-02T08:46:46.4878044Z   local time: Sun Feb  2 08:46:45 UTC 2020
2020-02-02T08:46:46.4878853Z   network time: Sun, 02 Feb 2020 08:46:46 GMT
2020-02-02T08:46:46.4879040Z == end clock drift check ==
2020-02-02T08:46:46.5415208Z 
2020-02-02T08:46:46.5527789Z ##[error]Bash exited with code '1'.
2020-02-02T08:46:46.5540322Z ##[section]Finishing: Run build
2020-02-02T08:46:46.5567620Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68761/merge to s
2020-02-02T08:46:46.5569662Z Task         : Get sources
2020-02-02T08:46:46.5569707Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-02T08:46:46.5569929Z Version      : 1.0.0
2020-02-02T08:46:46.5569968Z Author       : Microsoft
2020-02-02T08:46:46.5569968Z Author       : Microsoft
2020-02-02T08:46:46.5570011Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-02T08:46:46.5570059Z ==============================================================================
2020-02-02T08:46:46.9946335Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-02T08:46:46.9949021Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68761/merge to s
2020-02-02T08:46:47.0058344Z Cleaning up task key
2020-02-02T08:46:47.0059099Z Start cleaning up orphan processes.
2020-02-02T08:46:47.0174074Z Terminate orphan process: pid (4583) (python)
2020-02-02T08:46:47.0455542Z ##[section]Finishing: Finalize Job
