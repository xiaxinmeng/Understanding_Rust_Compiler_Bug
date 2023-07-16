plain
2020-01-28T07:01:13.7861333Z ========================== Starting Command Output ===========================
2020-01-28T07:01:13.7862832Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a39617b3-95a0-4a18-9202-11101aba45b8.sh
2020-01-28T07:01:13.7862867Z 
2020-01-28T07:01:13.7866318Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-28T07:01:13.7872036Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68491/merge to s
2020-01-28T07:01:13.7873589Z Task         : Get sources
2020-01-28T07:01:13.7873623Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-28T07:01:13.7873697Z Version      : 1.0.0
2020-01-28T07:01:13.7873730Z Author       : Microsoft
---
2020-01-28T07:01:14.5748009Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-28T07:01:14.5844616Z ##[command]git config gc.auto 0
2020-01-28T07:01:14.5901556Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-28T07:01:14.5950277Z ##[command]git config --get-all http.proxy
2020-01-28T07:01:14.6079538Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68491/merge:refs/remotes/pull/68491/merge
---
2020-01-28T07:50:39.5379938Z .................................................................................................... 1700/9561
2020-01-28T07:50:44.0220500Z .................................................................................................... 1800/9561
2020-01-28T07:50:55.3498834Z .........................i.......................................................................... 1900/9561
2020-01-28T07:51:01.7578122Z .................................................................................................... 2000/9561
2020-01-28T07:51:14.6130540Z ...............iiiii................................................................................ 2100/9561
2020-01-28T07:51:23.5054133Z .................................................................................................... 2300/9561
2020-01-28T07:51:25.6495661Z .................................................................................................... 2400/9561
2020-01-28T07:51:30.2016408Z .................................................................................................... 2500/9561
2020-01-28T07:51:48.4933088Z .................................................................................................... 2600/9561
---
2020-01-28T07:54:11.5007038Z .................................................................................................... 4800/9561
2020-01-28T07:54:16.3691860Z ...........................................................i...............i........................ 4900/9561
2020-01-28T07:54:23.6910496Z .................................................................................................... 5000/9561
2020-01-28T07:54:30.9799698Z .................................................................................................... 5100/9561
2020-01-28T07:54:35.4939996Z ..i................................................................................................. 5200/9561
2020-01-28T07:54:45.6422767Z ...........................................................................ii.ii........i...i....... 5300/9561
2020-01-28T07:54:53.7096120Z ..............i..................................................................................... 5500/9561
2020-01-28T07:55:02.4195236Z .................................................................................................... 5600/9561
2020-01-28T07:55:08.6918018Z ...............................................................i.................................... 5700/9561
2020-01-28T07:55:15.3534133Z .................................................................................................... 5800/9561
2020-01-28T07:55:15.3534133Z .................................................................................................... 5800/9561
2020-01-28T07:55:22.4674129Z .................................................................................................... 5900/9561
2020-01-28T07:55:30.5916878Z ......................................................ii...i..ii...........i........................ 6000/9561
2020-01-28T07:55:49.9409435Z .................................................................................................... 6200/9561
2020-01-28T07:55:53.6237435Z .................................................................................................... 6300/9561
2020-01-28T07:55:53.6237435Z .................................................................................................... 6300/9561
2020-01-28T07:55:57.5666147Z ..................................................................................i..ii............. 6400/9561
2020-01-28T07:56:19.0044100Z .................................................................................................... 6600/9561
2020-01-28T07:56:26.7610292Z ..........................................................i......................................... 6700/9561
2020-01-28T07:56:28.7172322Z .................................................................................................... 6800/9561
2020-01-28T07:56:30.8026595Z .........................................................i.......................................... 6900/9561
---
2020-01-28T07:58:02.6163108Z ...................................................................F................................ 7600/9561
2020-01-28T07:58:07.6732299Z .................................................................................................... 7700/9561
2020-01-28T07:58:13.8579282Z .................................................................................................... 7800/9561
2020-01-28T07:58:24.0203403Z .................................................................................................... 7900/9561
2020-01-28T07:58:29.6773611Z ................iiiiiii............................................................................. 8000/9561
2020-01-28T07:58:42.8516188Z .................................................................................................... 8200/9561
2020-01-28T07:58:52.0990259Z .................................................................................................... 8300/9561
2020-01-28T07:59:04.3705817Z .................................................................................................... 8400/9561
2020-01-28T07:59:10.6603478Z .................................................................................................... 8500/9561
---
2020-01-28T08:00:51.6543321Z 
2020-01-28T08:00:51.6544023Z ---- [ui] ui/repr/feature-gate-no-niche.rs stdout ----
2020-01-28T08:00:51.6544220Z diff of stderr:
2020-01-28T08:00:51.6544341Z 
2020-01-28T08:00:51.6544471Z 4 LL | #[repr(no_niche)]
2020-01-28T08:00:51.6544729Z 6    |
2020-01-28T08:00:51.6544729Z 6    |
2020-01-28T08:00:51.6545251Z -    = note: for more information, see ***/issues/68303
2020-01-28T08:00:51.6545822Z +    = note: for more information, see ***/issues/68491
2020-01-28T08:00:51.6545992Z 8    = help: add `#![feature(no_niche)]` to the crate attributes to enable
2020-01-28T08:00:51.6546130Z 9 
2020-01-28T08:00:51.6546260Z 10 error[E0658]: the attribute `repr(no_niche)` is currently unstable
2020-01-28T08:00:51.6546381Z 
2020-01-28T08:00:51.6546520Z 13 LL | #[repr(transparent, no_niche)]
2020-01-28T08:00:51.6546779Z 15    |
2020-01-28T08:00:51.6546779Z 15    |
2020-01-28T08:00:51.6547151Z -    = note: for more information, see ***/issues/68303
2020-01-28T08:00:51.6547547Z +    = note: for more information, see ***/issues/68491
2020-01-28T08:00:51.6547696Z 17    = help: add `#![feature(no_niche)]` to the crate attributes to enable
2020-01-28T08:00:51.6547821Z 18 
2020-01-28T08:00:51.6547963Z 19 error[E0658]: the attribute `repr(no_niche)` is currently unstable
2020-01-28T08:00:51.6548067Z 
2020-01-28T08:00:51.6548188Z 22 LL | #[repr(no_niche)]
2020-01-28T08:00:51.6548450Z 24    |
2020-01-28T08:00:51.6548450Z 24    |
2020-01-28T08:00:51.6548821Z -    = note: for more information, see ***/issues/68303
2020-01-28T08:00:51.6549203Z +    = note: for more information, see ***/issues/68491
2020-01-28T08:00:51.6549351Z 26    = help: add `#![feature(no_niche)]` to the crate attributes to enable
2020-01-28T08:00:51.6549488Z 27 
2020-01-28T08:00:51.6549610Z 28 error[E0658]: the attribute `repr(no_niche)` is currently unstable
2020-01-28T08:00:51.6549714Z 
2020-01-28T08:00:51.6549849Z 31 LL | #[repr(no_niche)]
2020-01-28T08:00:51.6550085Z 33    |
2020-01-28T08:00:51.6550085Z 33    |
2020-01-28T08:00:51.6550460Z -    = note: for more information, see ***/issues/68303
2020-01-28T08:00:51.6550833Z +    = note: for more information, see ***/issues/68491
2020-01-28T08:00:51.6550994Z 35    = help: add `#![feature(no_niche)]` to the crate attributes to enable
2020-01-28T08:00:51.6551257Z 37 error: aborting due to 4 previous errors
2020-01-28T08:00:51.6551362Z 
2020-01-28T08:00:51.6551461Z 
2020-01-28T08:00:51.6551584Z The actual stderr differed from the expected stderr.
2020-01-28T08:00:51.6551584Z The actual stderr differed from the expected stderr.
2020-01-28T08:00:51.6551995Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/feature-gate-no-niche/feature-gate-no-niche.stderr
2020-01-28T08:00:51.6552340Z To update references, rerun the tests and pass the `--bless` flag
2020-01-28T08:00:51.6552712Z To only update this specific test, also pass `--test-args repr/feature-gate-no-niche.rs`
2020-01-28T08:00:51.6552958Z error: 1 errors occurred comparing output.
2020-01-28T08:00:51.6553092Z status: exit code: 1
2020-01-28T08:00:51.6553092Z status: exit code: 1
2020-01-28T08:00:51.6554021Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/repr/feature-gate-no-niche.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/feature-gate-no-niche" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/feature-gate-no-niche/auxiliary" "-A" "unused"
2020-01-28T08:00:51.6554550Z ------------------------------------------
2020-01-28T08:00:51.6554669Z 
2020-01-28T08:00:51.6555002Z ------------------------------------------
2020-01-28T08:00:51.6555139Z stderr:
2020-01-28T08:00:51.6555139Z stderr:
2020-01-28T08:00:51.6555436Z ------------------------------------------
2020-01-28T08:00:51.6555597Z error[E0658]: the attribute `repr(no_niche)` is currently unstable
2020-01-28T08:00:51.6555921Z   --> /checkout/src/test/ui/repr/feature-gate-no-niche.rs:4:8
2020-01-28T08:00:51.6556072Z    |
2020-01-28T08:00:51.6556193Z LL | #[repr(no_niche)]
2020-01-28T08:00:51.6556443Z    |
2020-01-28T08:00:51.6556443Z    |
2020-01-28T08:00:51.6556884Z    = note: for more information, see ***/issues/68491
2020-01-28T08:00:51.6557046Z    = help: add `#![feature(no_niche)]` to the crate attributes to enable
2020-01-28T08:00:51.6557152Z 
2020-01-28T08:00:51.6557286Z error[E0658]: the attribute `repr(no_niche)` is currently unstable
2020-01-28T08:00:51.6557624Z   --> /checkout/src/test/ui/repr/feature-gate-no-niche.rs:8:21
2020-01-28T08:00:51.6557760Z    |
2020-01-28T08:00:51.6557881Z LL | #[repr(transparent, no_niche)]
2020-01-28T08:00:51.6558132Z    |
2020-01-28T08:00:51.6558132Z    |
2020-01-28T08:00:51.6558484Z    = note: for more information, see ***/issues/68491
2020-01-28T08:00:51.6558647Z    = help: add `#![feature(no_niche)]` to the crate attributes to enable
2020-01-28T08:00:51.6558751Z 
2020-01-28T08:00:51.6558875Z error[E0658]: the attribute `repr(no_niche)` is currently unstable
2020-01-28T08:00:51.6559215Z   --> /checkout/src/test/ui/repr/feature-gate-no-niche.rs:12:8
2020-01-28T08:00:51.6559357Z    |
2020-01-28T08:00:51.6559476Z LL | #[repr(no_niche)]
2020-01-28T08:00:51.6559725Z    |
2020-01-28T08:00:51.6559725Z    |
2020-01-28T08:00:51.6560103Z    = note: for more information, see ***/issues/68491
2020-01-28T08:00:51.6560252Z    = help: add `#![feature(no_niche)]` to the crate attributes to enable
2020-01-28T08:00:51.6560356Z 
2020-01-28T08:00:51.6560495Z error[E0658]: the attribute `repr(no_niche)` is currently unstable
2020-01-28T08:00:51.6560821Z   --> /checkout/src/test/ui/repr/feature-gate-no-niche.rs:16:8
2020-01-28T08:00:51.6560955Z    |
2020-01-28T08:00:51.6561088Z LL | #[repr(no_niche)]
2020-01-28T08:00:51.6561320Z    |
2020-01-28T08:00:51.6561320Z    |
2020-01-28T08:00:51.6561685Z    = note: for more information, see ***/issues/68491
2020-01-28T08:00:51.6561876Z    = help: add `#![feature(no_niche)]` to the crate attributes to enable
2020-01-28T08:00:51.6562221Z error: aborting due to 4 previous errors
2020-01-28T08:00:51.6562587Z 
2020-01-28T08:00:51.6562970Z For more information about this error, try `rustc --explain E0658`.
2020-01-28T08:00:51.6563486Z 
---
2020-01-28T08:00:51.6585242Z test result: FAILED. 9509 passed; 1 failed; 51 ignored; 0 measured; 0 filtered out
2020-01-28T08:00:51.6585336Z 
2020-01-28T08:00:51.6589356Z 
2020-01-28T08:00:51.6589424Z 
2020-01-28T08:00:51.6591299Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-28T08:00:51.6591602Z 
2020-01-28T08:00:51.6591643Z 
2020-01-28T08:00:51.6591942Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:349:22
2020-01-28T08:00:51.6592003Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-28T08:00:51.6592003Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-28T08:00:51.6594714Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-28T08:00:51.6594776Z Build completed unsuccessfully in 0:54:09
2020-01-28T08:00:51.6646691Z == clock drift check ==
2020-01-28T08:00:52.4629068Z   local time: Tue Jan 28 08:00:51 UTC 2020
2020-01-28T08:00:52.4643224Z   network time: Tue, 28 Jan 2020 08:00:52 GMT
2020-01-28T08:00:52.4643335Z == end clock drift check ==
2020-01-28T08:00:52.6214252Z 
2020-01-28T08:00:52.6296627Z ##[error]Bash exited with code '1'.
2020-01-28T08:00:52.6307412Z ##[section]Finishing: Run build
2020-01-28T08:00:52.6326509Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68491/merge to s
2020-01-28T08:00:52.6328304Z Task         : Get sources
2020-01-28T08:00:52.6328350Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-28T08:00:52.6328396Z Version      : 1.0.0
2020-01-28T08:00:52.6328452Z Author       : Microsoft
2020-01-28T08:00:52.6328452Z Author       : Microsoft
2020-01-28T08:00:52.6328498Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-28T08:00:52.6328550Z ==============================================================================
2020-01-28T08:00:53.0152238Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-28T08:00:53.0192739Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68491/merge to s
2020-01-28T08:00:53.0328128Z Cleaning up task key
2020-01-28T08:00:53.0329248Z Start cleaning up orphan processes.
2020-01-28T08:00:53.0426914Z Terminate orphan process: pid (4219) (python)
2020-01-28T08:00:53.0620426Z ##[section]Finishing: Finalize Job
