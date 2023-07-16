plain
2020-02-15T07:26:18.8611018Z ========================== Starting Command Output ===========================
2020-02-15T07:26:18.8612577Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9751870f-15ab-4b81-ad3f-0544e94498fd.sh
2020-02-15T07:26:18.8612610Z 
2020-02-15T07:26:18.8614823Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-15T07:26:18.8620992Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69181/merge to s
2020-02-15T07:26:18.8622750Z Task         : Get sources
2020-02-15T07:26:18.8622789Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-15T07:26:18.8622826Z Version      : 1.0.0
2020-02-15T07:26:18.8622863Z Author       : Microsoft
---
2020-02-15T07:26:19.8451590Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-15T07:26:19.8465873Z ##[command]git config gc.auto 0
2020-02-15T07:26:19.8470234Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-15T07:26:19.8474378Z ##[command]git config --get-all http.proxy
2020-02-15T07:26:19.8484052Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69181/merge:refs/remotes/pull/69181/merge
---
2020-02-15T08:22:41.2675219Z .................................................................................................... 1700/9647
2020-02-15T08:22:46.1251452Z .................................................................................................... 1800/9647
2020-02-15T08:22:57.6219595Z .................................i.................................................................. 1900/9647
2020-02-15T08:23:04.9949356Z .................................................................................................... 2000/9647
2020-02-15T08:23:18.9652687Z .......................iiiii........................................................................ 2100/9647
2020-02-15T08:23:28.6625029Z .................................................................................................... 2300/9647
2020-02-15T08:23:31.0527259Z .................................................................................................... 2400/9647
2020-02-15T08:23:35.6215564Z .................................................................................................... 2500/9647
2020-02-15T08:23:56.0622023Z .................................................................................................... 2600/9647
---
2020-02-15T08:26:28.6606310Z ................................................................................i...............i... 4900/9647
2020-02-15T08:26:35.8305006Z .................................................................................................... 5000/9647
2020-02-15T08:26:44.0682405Z .................................................................................................... 5100/9647
2020-02-15T08:26:48.6276688Z ......................i............................................................................. 5200/9647
2020-02-15T08:26:58.5843676Z ................................................................................................ii.i 5300/9647
2020-02-15T08:27:03.4948437Z i........i...i...................................................................................... 5400/9647
2020-02-15T08:27:12.7853478Z .................................................................................................... 5600/9647
2020-02-15T08:27:22.7965788Z .....................................................................................i.............. 5700/9647
2020-02-15T08:27:30.4726627Z .................................................................................................... 5800/9647
2020-02-15T08:27:35.7828937Z ...................................................................................i................ 5900/9647
2020-02-15T08:27:35.7828937Z ...................................................................................i................ 5900/9647
2020-02-15T08:27:45.2746651Z .............................................................................ii...i..ii...........i. 6000/9647
2020-02-15T08:28:06.9021607Z .................................................................................................... 6200/9647
2020-02-15T08:28:12.2471709Z .................................................................................................... 6300/9647
2020-02-15T08:28:16.3594968Z .................................................................................................... 6400/9647
2020-02-15T08:28:16.3594968Z .................................................................................................... 6400/9647
2020-02-15T08:28:29.1075049Z .....i..ii.......................................................................................... 6500/9647
2020-02-15T08:28:48.2472525Z .............................................................................................i...... 6700/9647
2020-02-15T08:28:50.4011812Z .................................................................................................... 6800/9647
2020-02-15T08:28:52.6373673Z .................................................................................................... 6900/9647
2020-02-15T08:28:55.1385642Z ...i................................................................................................ 7000/9647
---
2020-02-15T08:30:27.5600122Z .................................................................................................... 7600/9647
2020-02-15T08:30:32.0780098Z .................................................................................................... 7700/9647
2020-02-15T08:30:38.1535150Z .................................................................................................... 7800/9647
2020-02-15T08:30:44.9980926Z .................................................................................................... 7900/9647
2020-02-15T08:30:54.0744882Z .....................................................................................iiiiiii.i...... 8000/9647
2020-02-15T08:31:09.7396886Z .........................i......i................................................................... 8200/9647
2020-02-15T08:31:14.6998106Z .................................................................................................... 8300/9647
2020-02-15T08:31:26.2074204Z .................................................................................................... 8400/9647
2020-02-15T08:31:38.1312080Z .................................................................................................... 8500/9647
---
2020-02-15T08:33:59.9936918Z  finished in 7.083
2020-02-15T08:34:00.0151167Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-15T08:34:00.2128360Z 
2020-02-15T08:34:00.2129661Z running 178 tests
2020-02-15T08:34:03.0859236Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/178
2020-02-15T08:34:05.2663175Z ...i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-02-15T08:34:05.2667125Z 
2020-02-15T08:34:05.2667189Z  finished in 5.251
2020-02-15T08:34:05.2864997Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-15T08:34:05.4538281Z 
---
2020-02-15T08:34:07.3630420Z  finished in 2.076
2020-02-15T08:34:07.3822686Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-15T08:34:07.5428118Z 
2020-02-15T08:34:07.5428589Z running 9 tests
2020-02-15T08:34:07.5429382Z iiiiiiiii
2020-02-15T08:34:07.5429764Z 
2020-02-15T08:34:07.5429815Z  finished in 0.160
2020-02-15T08:34:07.5631798Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-15T08:34:07.7460247Z 
---
2020-02-15T08:34:27.1161714Z  finished in 19.553
2020-02-15T08:34:27.1391831Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-15T08:34:27.3227921Z 
2020-02-15T08:34:27.3228310Z running 116 tests
2020-02-15T08:34:40.1244994Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-15T08:34:41.9631502Z ....iiii.....ii.
2020-02-15T08:34:41.9633248Z 
2020-02-15T08:34:41.9639961Z  finished in 14.825
2020-02-15T08:34:41.9645527Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-15T08:34:41.9646167Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-02-15T08:41:17.8976519Z 
2020-02-15T08:41:17.8976781Z ------------------------------------------
2020-02-15T08:41:17.8976839Z stderr:
2020-02-15T08:41:17.8977112Z ------------------------------------------
2020-02-15T08:41:17.8977458Z thread 'rustc' panicked at 'could not lift for printing', src/librustc/ty/print/pretty.rs:1690:1
2020-02-15T08:41:17.8977594Z 
2020-02-15T08:41:17.8977853Z ------------------------------------------
2020-02-15T08:41:17.8977892Z 
2020-02-15T08:41:17.8977923Z 
2020-02-15T08:41:17.8977923Z 
2020-02-15T08:41:17.8978402Z ---- [rustdoc] rustdoc/issue-16019.rs stdout ----
2020-02-15T08:41:17.8980605Z 
2020-02-15T08:41:17.8980667Z error: rustdoc failed!
2020-02-15T08:41:17.8980723Z status: exit code: 1
2020-02-15T08:41:17.8981561Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-16019/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-16019" "/checkout/src/test/rustdoc/issue-16019.rs"
2020-02-15T08:41:17.8982133Z ------------------------------------------
2020-02-15T08:41:17.8982180Z 
2020-02-15T08:41:17.8982460Z ------------------------------------------
2020-02-15T08:41:17.8982518Z stderr:
2020-02-15T08:41:17.8982518Z stderr:
2020-02-15T08:41:17.8982794Z ------------------------------------------
2020-02-15T08:41:17.8983130Z thread 'rustc' panicked at 'could not lift for printing', src/librustc/ty/print/pretty.rs:1690:1
2020-02-15T08:41:17.8983367Z 
2020-02-15T08:41:17.8983642Z ------------------------------------------
2020-02-15T08:41:17.8983681Z 
2020-02-15T08:41:17.8983713Z 
2020-02-15T08:41:17.8983713Z 
2020-02-15T08:41:17.8983995Z ---- [rustdoc] rustdoc/issue-33302.rs stdout ----
2020-02-15T08:41:17.8984036Z 
2020-02-15T08:41:17.8984088Z error: rustdoc failed!
2020-02-15T08:41:17.8984143Z status: exit code: 1
2020-02-15T08:41:17.8984852Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-33302/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-33302" "/checkout/src/test/rustdoc/issue-33302.rs"
2020-02-15T08:41:17.8985217Z ------------------------------------------
2020-02-15T08:41:17.8985258Z 
2020-02-15T08:41:17.8985517Z ------------------------------------------
2020-02-15T08:41:17.8985584Z stderr:
2020-02-15T08:41:17.8985584Z stderr:
2020-02-15T08:41:17.8985858Z ------------------------------------------
2020-02-15T08:41:17.8986189Z thread 'rustc' panicked at 'could not lift for printing', src/librustc/ty/print/pretty.rs:1690:1
2020-02-15T08:41:17.8986320Z 
2020-02-15T08:41:17.8986579Z ------------------------------------------
2020-02-15T08:41:17.8986617Z 
2020-02-15T08:41:17.8986649Z 
2020-02-15T08:41:17.8986649Z 
2020-02-15T08:41:17.8986942Z ---- [rustdoc] rustdoc/issue-43869.rs stdout ----
2020-02-15T08:41:17.8986982Z 
2020-02-15T08:41:17.8987034Z error: rustdoc failed!
2020-02-15T08:41:17.8987089Z status: exit code: 1
2020-02-15T08:41:17.8987793Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43869/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43869" "/checkout/src/test/rustdoc/issue-43869.rs"
2020-02-15T08:41:17.8988164Z ------------------------------------------
2020-02-15T08:41:17.8988255Z 
2020-02-15T08:41:17.8988519Z ------------------------------------------
2020-02-15T08:41:17.8988576Z stderr:
2020-02-15T08:41:17.8988576Z stderr:
2020-02-15T08:41:17.8988849Z ------------------------------------------
2020-02-15T08:41:17.8988912Z warning: unnecessary parentheses around type
2020-02-15T08:41:17.8989199Z  --> /checkout/src/test/rustdoc/issue-43869.rs:5:15
2020-02-15T08:41:17.8989275Z   |
2020-02-15T08:41:17.8989544Z 5 | pub fn h() -> (impl Iterator<Item=u8>) {
2020-02-15T08:41:17.8989685Z   |
2020-02-15T08:41:17.8989744Z   = note: `#[warn(unused_parens)]` on by default
2020-02-15T08:41:17.8989781Z 
2020-02-15T08:41:17.8989781Z 
2020-02-15T08:41:17.8990107Z thread 'rustc' panicked at 'could not lift for printing', src/librustc/ty/print/pretty.rs:1690:1
2020-02-15T08:41:17.8990254Z 
2020-02-15T08:41:17.8990513Z ------------------------------------------
2020-02-15T08:41:17.8990566Z 
2020-02-15T08:41:17.8990599Z 
2020-02-15T08:41:17.8990599Z 
2020-02-15T08:41:17.8990868Z ---- [rustdoc] rustdoc/issue-53812.rs stdout ----
2020-02-15T08:41:17.8990908Z 
2020-02-15T08:41:17.8990960Z error: rustdoc failed!
2020-02-15T08:41:17.8991029Z status: exit code: 1
2020-02-15T08:41:17.8991785Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-53812/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-53812" "/checkout/src/test/rustdoc/issue-53812.rs"
2020-02-15T08:41:17.8992167Z ------------------------------------------
2020-02-15T08:41:17.8992271Z 
2020-02-15T08:41:17.8992556Z ------------------------------------------
2020-02-15T08:41:17.8992612Z stderr:
2020-02-15T08:41:17.8992612Z stderr:
2020-02-15T08:41:17.8992871Z ------------------------------------------
2020-02-15T08:41:17.8993198Z thread 'rustc' panicked at 'could not lift for printing', src/librustc/ty/print/pretty.rs:1690:1
2020-02-15T08:41:17.8993327Z 
2020-02-15T08:41:17.8993586Z ------------------------------------------
2020-02-15T08:41:17.8993650Z 
2020-02-15T08:41:17.8993682Z 
2020-02-15T08:41:17.8993682Z 
2020-02-15T08:41:17.8993952Z ---- [rustdoc] rustdoc/issue-60482.rs stdout ----
2020-02-15T08:41:17.8993992Z 
2020-02-15T08:41:17.8994058Z error: rustdoc failed!
2020-02-15T08:41:17.8994111Z status: exit code: 1
2020-02-15T08:41:17.8994957Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-60482/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-60482" "/checkout/src/test/rustdoc/issue-60482.rs"
2020-02-15T08:41:17.8995485Z ------------------------------------------
2020-02-15T08:41:17.8995523Z 
2020-02-15T08:41:17.8995798Z ------------------------------------------
2020-02-15T08:41:17.8995856Z stderr:
2020-02-15T08:41:17.8995856Z stderr:
2020-02-15T08:41:17.8996113Z ------------------------------------------
2020-02-15T08:41:17.8996465Z thread 'rustc' panicked at 'could not lift for printing', src/librustc/ty/print/pretty.rs:1690:1
2020-02-15T08:41:17.8996581Z 
2020-02-15T08:41:17.8996841Z ------------------------------------------
2020-02-15T08:41:17.8996894Z 
2020-02-15T08:41:17.8996925Z 
---
2020-02-15T08:41:17.8998488Z 
2020-02-15T08:41:17.8998762Z ------------------------------------------
2020-02-15T08:41:17.8998817Z stderr:
2020-02-15T08:41:17.8999072Z ------------------------------------------
2020-02-15T08:41:17.8999416Z thread 'rustc' panicked at 'could not lift for printing', src/librustc/ty/print/pretty.rs:1690:1
2020-02-15T08:41:17.8999540Z 
2020-02-15T08:41:17.8999798Z ------------------------------------------
2020-02-15T08:41:17.8999850Z 
2020-02-15T08:41:17.8999882Z 
---
2020-02-15T08:41:17.9002446Z 
2020-02-15T08:41:17.9002493Z 
2020-02-15T08:41:17.9002820Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-15T08:41:17.9002891Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-15T08:41:17.9004822Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-15T08:41:17.9005108Z 
2020-02-15T08:41:17.9005158Z 
2020-02-15T08:41:17.9005293Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-15T08:41:17.9005358Z Build completed unsuccessfully in 1:09:01
2020-02-15T08:41:17.9005358Z Build completed unsuccessfully in 1:09:01
2020-02-15T08:41:17.9039103Z == clock drift check ==
2020-02-15T08:41:17.9069358Z   local time: Sat Feb 15 08:41:17 UTC 2020
2020-02-15T08:41:18.2017170Z   network time: Sat, 15 Feb 2020 08:41:18 GMT
2020-02-15T08:41:18.2026745Z == end clock drift check ==
2020-02-15T08:41:19.5952955Z 
2020-02-15T08:41:19.6040768Z ##[error]Bash exited with code '1'.
2020-02-15T08:41:19.6053604Z ##[section]Finishing: Run build
2020-02-15T08:41:19.6073427Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69181/merge to s
2020-02-15T08:41:19.6075753Z Task         : Get sources
2020-02-15T08:41:19.6075825Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-15T08:41:19.6075877Z Version      : 1.0.0
2020-02-15T08:41:19.6075924Z Author       : Microsoft
2020-02-15T08:41:19.6075924Z Author       : Microsoft
2020-02-15T08:41:19.6075991Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-15T08:41:19.6076048Z ==============================================================================
2020-02-15T08:41:20.0100452Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-15T08:41:20.0143233Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69181/merge to s
2020-02-15T08:41:20.0252757Z Cleaning up task key
2020-02-15T08:41:20.0253545Z Start cleaning up orphan processes.
2020-02-15T08:41:20.0357674Z Terminate orphan process: pid (3862) (python)
2020-02-15T08:41:20.0625282Z ##[section]Finishing: Finalize Job
