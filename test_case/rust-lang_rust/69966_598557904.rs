plain
2020-03-13T04:36:41.0525261Z ========================== Starting Command Output ===========================
2020-03-13T04:36:41.0529138Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f390b8e8-d0a4-4ff2-af8e-14b4f34ffbbd.sh
2020-03-13T04:36:41.0529448Z 
2020-03-13T04:36:41.0533794Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-13T04:36:41.0551833Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69966/merge to s
2020-03-13T04:36:41.0554803Z Task         : Get sources
2020-03-13T04:36:41.0555093Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T04:36:41.0555389Z Version      : 1.0.0
2020-03-13T04:36:41.0555582Z Author       : Microsoft
---
2020-03-13T04:36:42.0445094Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-13T04:36:42.0453760Z ##[command]git config gc.auto 0
2020-03-13T04:36:42.0457099Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-13T04:36:42.0460204Z ##[command]git config --get-all http.proxy
2020-03-13T04:36:42.0465964Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69966/merge:refs/remotes/pull/69966/merge
---
2020-03-13T05:26:57.0875523Z .................................................................................................... 1700/9772
2020-03-13T05:27:00.6880965Z .................................................................................................... 1800/9772
2020-03-13T05:27:09.7500636Z ...................................................................i................................ 1900/9772
2020-03-13T05:27:15.0916463Z .................................................................................................... 2000/9772
2020-03-13T05:27:26.3336384Z .........................................................iiiii...................................... 2100/9772
2020-03-13T05:27:34.2704313Z .................................................................................................... 2300/9772
2020-03-13T05:27:35.9360929Z .................................................................................................... 2400/9772
2020-03-13T05:27:38.2607224Z .................................................................................................... 2500/9772
2020-03-13T05:27:54.8953589Z .................................................................................................... 2600/9772
---
2020-03-13T05:29:55.3489771Z ............................i...............i....................................................... 5000/9772
2020-03-13T05:30:02.7906573Z .................................................................................................... 5100/9772
2020-03-13T05:30:07.0659064Z .......................................................................i............................ 5200/9772
2020-03-13T05:30:11.3376050Z .................................................................................................... 5300/9772
2020-03-13T05:30:18.5471092Z ....................................................ii.ii........i...i.............................. 5400/9772
2020-03-13T05:30:24.7496080Z .................................................................................................... 5600/9772
2020-03-13T05:30:31.9685229Z .................................................................................................... 5700/9772
2020-03-13T05:30:36.6453635Z ............................................i....................................................... 5800/9772
2020-03-13T05:30:41.4219600Z .................................................................................................... 5900/9772
2020-03-13T05:30:41.4219600Z .................................................................................................... 5900/9772
2020-03-13T05:30:49.0757305Z .................................................................................................... 6000/9772
2020-03-13T05:30:53.7025630Z ......................................ii...i..ii...........i........................................ 6100/9772
2020-03-13T05:31:09.4312382Z .................................................................................................... 6300/9772
2020-03-13T05:31:12.5423799Z .................................................................................................... 6400/9772
2020-03-13T05:31:12.5423799Z .................................................................................................... 6400/9772
2020-03-13T05:31:16.4001654Z .....................................................................i..ii.......................... 6500/9772
2020-03-13T05:31:33.9776341Z .................................................................................................... 6700/9772
2020-03-13T05:31:40.5088632Z ...................................................................i................................ 6800/9772
2020-03-13T05:31:42.0248113Z .................................................................................................... 6900/9772
2020-03-13T05:31:43.7540753Z .................................................................................................... 7000/9772
---
2020-03-13T05:33:01.3764953Z .................................................................................................... 7800/9772
2020-03-13T05:33:05.8783911Z .................................................................................................... 7900/9772
2020-03-13T05:33:10.2932142Z ...................................................i................................................ 8000/9772
2020-03-13T05:33:18.2602809Z .................................................................................................... 8100/9772
2020-03-13T05:33:22.3920493Z iiiiiiiiii.i........................................................................................ 8200/9772
2020-03-13T05:33:32.9280337Z .................................................................................................... 8400/9772
2020-03-13T05:33:40.9745620Z .................................................................................................... 8500/9772
2020-03-13T05:33:51.1040100Z .................................................................................................... 8600/9772
2020-03-13T05:33:55.8454582Z .................................................................................................... 8700/9772
---
2020-03-13T05:35:20.6837426Z 
2020-03-13T05:35:20.6838858Z ---- [ui] ui/asm/issue-69092.rs stdout ----
2020-03-13T05:35:20.6840813Z diff of stderr:
2020-03-13T05:35:20.6841077Z 
2020-03-13T05:35:20.6841363Z 2         .ascii "Xen
2020-03-13T05:35:20.6841988Z 4 
2020-03-13T05:35:20.6842718Z -   --> $DIR/issue-69092.rs:7:14
2020-03-13T05:35:20.6843467Z +   --> $DIR/issue-69092.rs:8:14
2020-03-13T05:35:20.6843834Z 6    |
2020-03-13T05:35:20.6843834Z 6    |
2020-03-13T05:35:20.6844220Z 7 LL |     unsafe { asm!(".ascii \"Xen\0\""); }
2020-03-13T05:35:20.6845034Z 
2020-03-13T05:35:20.6845228Z 
2020-03-13T05:35:20.6845609Z The actual stderr differed from the expected stderr.
2020-03-13T05:35:20.6847005Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/issue-69092/issue-69092.stderr
2020-03-13T05:35:20.6847005Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/issue-69092/issue-69092.stderr
2020-03-13T05:35:20.6848523Z To update references, rerun the tests and pass the `--bless` flag
2020-03-13T05:35:20.6849564Z To only update this specific test, also pass `--test-args asm/issue-69092.rs`
2020-03-13T05:35:20.6850472Z error: 1 errors occurred comparing output.
2020-03-13T05:35:20.6850919Z status: exit code: 1
2020-03-13T05:35:20.6850919Z status: exit code: 1
2020-03-13T05:35:20.6854218Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/issue-69092.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/issue-69092" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/issue-69092/auxiliary"
2020-03-13T05:35:20.6856946Z ------------------------------------------
2020-03-13T05:35:20.6857563Z 
2020-03-13T05:35:20.6858334Z ------------------------------------------
2020-03-13T05:35:20.6858713Z stderr:
2020-03-13T05:35:20.6858713Z stderr:
2020-03-13T05:35:20.6859382Z ------------------------------------------
2020-03-13T05:35:20.6860349Z error: <inline asm>:1:9: error: expected string in '.ascii' directive
2020-03-13T05:35:20.6860874Z         .ascii "Xen
2020-03-13T05:35:20.6861447Z 
2020-03-13T05:35:20.6862169Z   --> /checkout/src/test/ui/asm/issue-69092.rs:8:14
2020-03-13T05:35:20.6862762Z    |
2020-03-13T05:35:20.6862762Z    |
2020-03-13T05:35:20.6863147Z LL |     unsafe { asm!(".ascii \"Xen\0\""); }
2020-03-13T05:35:20.6863945Z 
2020-03-13T05:35:20.6864276Z error: aborting due to previous error
2020-03-13T05:35:20.6864591Z 
2020-03-13T05:35:20.6864768Z 
2020-03-13T05:35:20.6864768Z 
2020-03-13T05:35:20.6865442Z ------------------------------------------
2020-03-13T05:35:20.6865748Z 
2020-03-13T05:35:20.6865925Z 
2020-03-13T05:35:20.6866620Z ---- [ui] ui/traits/issue-56229.rs stdout ----
2020-03-13T05:35:20.6866944Z 
2020-03-13T05:35:20.6867657Z error: test compilation failed although it shouldn't!
2020-03-13T05:35:20.6868380Z status: exit code: 101
2020-03-13T05:35:20.6871028Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-56229.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-56229" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-56229/auxiliary"
2020-03-13T05:35:20.6872383Z ------------------------------------------
2020-03-13T05:35:20.6872526Z 
2020-03-13T05:35:20.6872835Z ------------------------------------------
2020-03-13T05:35:20.6872999Z stderr:
2020-03-13T05:35:20.6872999Z stderr:
2020-03-13T05:35:20.6873397Z ------------------------------------------
2020-03-13T05:35:20.6873823Z error: internal compiler error: src/librustc_infer/traits/codegen/mod.rs:61: Encountered error `Unimplemented` selecting `Binder(<dyn Dyn<Odd> as std::convert::AsRef<Even>>)` during codegen
2020-03-13T05:35:20.6874483Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:880:9
2020-03-13T05:35:20.6874770Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-13T05:35:20.6874935Z 
2020-03-13T05:35:20.6875089Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-13T05:35:20.6875089Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-13T05:35:20.6875230Z 
2020-03-13T05:35:20.6875747Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-13T05:35:20.6876351Z note: rustc 1.43.0-nightly (fd01a0bde 2020-03-13) running on x86_64-unknown-linux-gnu
2020-03-13T05:35:20.6876529Z 
2020-03-13T05:35:20.6876529Z 
2020-03-13T05:35:20.6876996Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-13T05:35:20.6877418Z error: aborting due to previous error
2020-03-13T05:35:20.6877549Z 
2020-03-13T05:35:20.6877617Z 
2020-03-13T05:35:20.6877874Z ------------------------------------------
---
2020-03-13T05:35:20.6879238Z test result: FAILED. 9713 passed; 2 failed; 57 ignored; 0 measured; 0 filtered out
2020-03-13T05:35:20.6879427Z 
2020-03-13T05:35:20.6893046Z 
2020-03-13T05:35:20.6893208Z 
2020-03-13T05:35:20.6896521Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-13T05:35:20.6898406Z 
2020-03-13T05:35:20.6898476Z 
2020-03-13T05:35:20.6898836Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-13T05:35:20.6899144Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-13T05:35:20.6899144Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-13T05:35:20.6899409Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-13T05:35:20.6899638Z Build completed unsuccessfully in 0:52:37
2020-03-13T05:35:20.6941033Z == clock drift check ==
2020-03-13T05:35:20.6956447Z   local time: Fri Mar 13 05:35:20 UTC 2020
2020-03-13T05:35:20.9841814Z   network time: Fri, 13 Mar 2020 05:35:20 GMT
2020-03-13T05:35:20.9850452Z == end clock drift check ==
2020-03-13T05:35:21.4970324Z 
2020-03-13T05:35:21.5060209Z ##[error]Bash exited with code '1'.
2020-03-13T05:35:21.5070660Z ##[section]Finishing: Run build
2020-03-13T05:35:21.5106634Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69966/merge to s
2020-03-13T05:35:21.5110460Z Task         : Get sources
2020-03-13T05:35:21.5110703Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T05:35:21.5110925Z Version      : 1.0.0
2020-03-13T05:35:21.5111106Z Author       : Microsoft
2020-03-13T05:35:21.5111106Z Author       : Microsoft
2020-03-13T05:35:21.5111359Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-13T05:35:21.5111645Z ==============================================================================
2020-03-13T05:35:21.7605731Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-13T05:35:21.7645420Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69966/merge to s
2020-03-13T05:35:21.7705792Z Cleaning up task key
2020-03-13T05:35:21.7706727Z Start cleaning up orphan processes.
2020-03-13T05:35:21.7839111Z Terminate orphan process: pid (4001) (python)
2020-03-13T05:35:21.7968389Z ##[section]Finishing: Finalize Job
