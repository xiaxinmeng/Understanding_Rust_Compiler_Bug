plain
2020-03-22T20:45:37.3021637Z ========================== Starting Command Output ===========================
2020-03-22T20:45:37.3026837Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d2c40610-17e8-46e7-b08f-c18034ee116e.sh
2020-03-22T20:45:37.3027305Z 
2020-03-22T20:45:37.3031879Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-22T20:45:37.3051059Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70282/merge to s
2020-03-22T20:45:37.3054006Z Task         : Get sources
2020-03-22T20:45:37.3054259Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T20:45:37.3054518Z Version      : 1.0.0
2020-03-22T20:45:37.3054686Z Author       : Microsoft
---
2020-03-22T20:45:38.2918266Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-22T20:45:38.2930148Z ##[command]git config gc.auto 0
2020-03-22T20:45:38.2935110Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-22T20:45:38.2939535Z ##[command]git config --get-all http.proxy
2020-03-22T20:45:38.2946509Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70282/merge:refs/remotes/pull/70282/merge
---
2020-03-22T21:45:09.0431295Z .................................................................................................... 1700/9821
2020-03-22T21:45:13.3719441Z .................................................................................................... 1800/9821
2020-03-22T21:45:23.5826387Z ................................................................................i................... 1900/9821
2020-03-22T21:45:30.8345860Z .................................................................................................... 2000/9821
2020-03-22T21:45:37.7993890Z ......................................................................iiiii......................... 2100/9821
2020-03-22T21:45:58.5259475Z .................................................................................................... 2300/9821
2020-03-22T21:46:00.8392530Z .................................................................................................... 2400/9821
2020-03-22T21:46:03.4194915Z .................................................................................................... 2500/9821
2020-03-22T21:46:17.3110624Z .................................................................................................... 2600/9821
---
2020-03-22T21:49:07.5395784Z ............................................i...............i....................................... 5000/9821
2020-03-22T21:49:16.4563389Z .................................................................................................... 5100/9821
2020-03-22T21:49:23.7433728Z ........................................................................................i........... 5200/9821
2020-03-22T21:49:29.8340313Z .................................................................................................... 5300/9821
2020-03-22T21:49:40.8335524Z .......................................................................ii.ii........i...i........... 5400/9821
2020-03-22T21:49:49.3389905Z ...........i........................................................................................ 5600/9821
2020-03-22T21:49:58.5469542Z ................i................................................................................... 5700/9821
2020-03-22T21:50:05.2980938Z .................................ii...................................i............................. 5800/9821
2020-03-22T21:50:12.3597917Z .................................................................................................... 5900/9821
2020-03-22T21:50:12.3597917Z .................................................................................................... 5900/9821
2020-03-22T21:50:19.0215847Z .................................................................................................... 6000/9821
2020-03-22T21:50:28.3062451Z ................................................................ii...i..ii...........i.............. 6100/9821
2020-03-22T21:50:49.3408082Z .................................................................................................... 6300/9821
2020-03-22T21:50:56.0816765Z .................................................................................................... 6400/9821
2020-03-22T21:50:56.0816765Z .................................................................................................... 6400/9821
2020-03-22T21:51:03.5642181Z ..............................................................................................i..ii. 6500/9821
2020-03-22T21:51:31.1193614Z .................................................................................................... 6700/9821
2020-03-22T21:51:42.0838683Z .............................................................................................i...... 6800/9821
2020-03-22T21:51:44.2242991Z .................................................................................................... 6900/9821
2020-03-22T21:51:46.4245391Z .................................................................................................... 7000/9821
---
2020-03-22T21:53:31.8185731Z .................................................................................................... 7800/9821
2020-03-22T21:53:36.3991657Z .................................................................................................... 7900/9821
2020-03-22T21:53:42.5359655Z ...................................................................................i................ 8000/9821
2020-03-22T21:53:51.0347964Z .................................................................................................... 8100/9821
2020-03-22T21:53:57.8130514Z ................................iiiiiiiiii.i........................................................ 8200/9821
2020-03-22T21:54:12.2151058Z .................................................................................................... 8400/9821
2020-03-22T21:54:17.4985349Z .................................................................................................... 8500/9821
2020-03-22T21:54:33.0103022Z .................................................................................................... 8600/9821
2020-03-22T21:54:41.3197524Z .................................................................................................... 8700/9821
---
2020-03-22T21:57:06.4400615Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-03-22T21:57:06.4401619Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-22T21:57:06.4402612Z 
2020-03-22T21:57:06.4402811Z running 183 tests
2020-03-22T21:57:09.4367674Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/183
2020-03-22T21:57:11.6061426Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-03-22T21:57:11.6065393Z 
2020-03-22T21:57:11.6072182Z  finished in 5.772
2020-03-22T21:57:11.6083034Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-03-22T21:57:11.6306429Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-22T21:57:13.7812883Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-03-22T21:57:13.8015174Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-22T21:57:13.9707654Z 
2020-03-22T21:57:13.9708270Z running 9 tests
2020-03-22T21:57:13.9709634Z iiiiiiiii
2020-03-22T21:57:13.9711208Z 
2020-03-22T21:57:13.9711588Z  finished in 0.169
2020-03-22T21:57:13.9716319Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-03-22T21:57:13.9970459Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-22T21:57:34.3137575Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-03-22T21:57:34.3340777Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-22T21:57:34.5150874Z 
2020-03-22T21:57:34.5151234Z running 115 tests
2020-03-22T21:57:48.6787340Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..iF......ii.i.ii.. 100/115
2020-03-22T21:57:50.5174648Z ...iiii.....ii.
2020-03-22T21:57:50.5184853Z 
2020-03-22T21:57:50.5186044Z ---- [debuginfo-gdb] debuginfo/pretty-std-collections.rs stdout ----
2020-03-22T21:57:50.5186591Z NOTE: compiletest thinks it is using GDB with native rust support
2020-03-22T21:57:50.5186970Z NOTE: compiletest thinks it is using GDB version 8001000
2020-03-22T21:57:50.5186970Z NOTE: compiletest thinks it is using GDB version 8001000
2020-03-22T21:57:50.5187197Z 
2020-03-22T21:57:50.5188104Z error: line not found in debugger output: $5 = BTreeMap<bool, core::option::Option<bool>>(len: 2) = {[false] = core::option::Option<bool>::None, [true] = core::option::Option<bool>::Some(true)}
2020-03-22T21:57:50.5188861Z status: exit code: 0
2020-03-22T21:57:50.5189880Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std-collections.gdb/pretty-std-collections.debugger.script"
2020-03-22T21:57:50.5190864Z ------------------------------------------
2020-03-22T21:57:50.5190864Z ------------------------------------------
2020-03-22T21:57:50.5191400Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-03-22T21:57:50.5191748Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-03-22T21:57:50.5192149Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-03-22T21:57:50.5192615Z This is free software: you are free to change and redistribute it.
2020-03-22T21:57:50.5193032Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-03-22T21:57:50.5193379Z and "show warranty" for details.
2020-03-22T21:57:50.5193880Z This GDB was configured as "x86_64-linux-gnu".
2020-03-22T21:57:50.5194232Z Type "show configuration" for configuration details.
2020-03-22T21:57:50.5194545Z For bug reporting instructions, please see:
2020-03-22T21:57:50.5194838Z <http://www.gnu.org/software/gdb/bugs/>.
2020-03-22T21:57:50.5195194Z Find the GDB manual and other documentation resources online at:
2020-03-22T21:57:50.5195549Z <http://www.gnu.org/software/gdb/documentation/>.
2020-03-22T21:57:50.5195821Z For help, type "help".
2020-03-22T21:57:50.5196130Z Type "apropos word" to search for commands related to "word".
2020-03-22T21:57:50.5196851Z Breakpoint 1 at 0x2f6df: file /checkout/src/test/debuginfo/pretty-std-collections.rs, line 88.
2020-03-22T21:57:50.5197270Z [Thread debugging using libthread_db enabled]
2020-03-22T21:57:50.5197888Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-03-22T21:57:50.5198156Z 
2020-03-22T21:57:50.5198802Z Breakpoint 1, pretty_std_collections::main () at /checkout/src/test/debuginfo/pretty-std-collections.rs:88
2020-03-22T21:57:50.5199242Z 88     zzz(); // #break
2020-03-22T21:57:50.5199593Z $1 = BTreeSet<i32>(len: 15) = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14}
2020-03-22T21:57:50.5199952Z $2 = BTreeSet<i32>(len: 0)
2020-03-22T21:57:50.5200527Z $3 = BTreeMap<i32, i32>(len: 15) = {[0] = 0, [1] = 1, [2] = 2, [3] = 3, [4] = 4, [5] = 5, [6] = 6, [7] = 7, [8] = 8, [9] = 9, [10] = 10, [11] = 11, [12] = 12, [13] = 13, [14] = 14}
2020-03-22T21:57:50.5201095Z $4 = BTreeMap<i32, u32>(len: 0)
2020-03-22T21:57:50.5201689Z $5 = BTreeMap<bool, core::option::Option<bool>>(len: 2) = {[false] = core::option::Option<bool>::Some(2), [true] = core::option::Option<bool>::Some(true)}
2020-03-22T21:57:50.5204094Z $6 = BTreeMap<i32, pretty_std_collections::MyLeafNode>(len: 15) = {[0] = pretty_std_collections::MyLeafNode (0), [1] = pretty_std_collections::MyLeafNode (1), [2] = pretty_std_collections::MyLeafNode (2), [3] = pretty_std_collections::MyLeafNode (3), [4] = pretty_std_collections::MyLeafNode (4), [5] = pretty_std_collections::MyLeafNode (5), [6] = pretty_std_collections::MyLeafNode (6), [7] = pretty_std_collections::MyLeafNode (7), [8] = pretty_std_collections::MyLeafNode (8), [9] = pretty_std_collections::MyLeafNode (9), [10] = pretty_std_collections::MyLeafNode (10), [11] = pretty_std_collections::MyLeafNode (11), [12] = pretty_std_collections::MyLeafNode (12), [13] = pretty_std_collections::MyLeafNode (13), [14] = pretty_std_collections::MyLeafNode (14)}
2020-03-22T21:57:50.5206362Z $7 = VecDeque<i32>(len: 3, cap: 8) = {5, 3, 7}
2020-03-22T21:57:50.5206731Z $8 = VecDeque<i32>(len: 7, cap: 8) = {2, 3, 4, 5, 6, 7, 8}
2020-03-22T21:57:50.5207241Z 
2020-03-22T21:57:50.5207241Z 
2020-03-22T21:57:50.5207451Z  Inferior 1 [process 1981] will be killed.
2020-03-22T21:57:50.5207642Z 
2020-03-22T21:57:50.5207902Z Quit anyway? (y or n) [answered Y; input not from terminal]
2020-03-22T21:57:50.5209035Z ------------------------------------------
2020-03-22T21:57:50.5209280Z stderr:
2020-03-22T21:57:50.5209908Z ------------------------------------------
2020-03-22T21:57:50.5210118Z 
---
2020-03-22T21:57:50.5212448Z test result: FAILED. 76 passed; 1 failed; 38 ignored; 0 measured; 0 filtered out
2020-03-22T21:57:50.5212751Z 
2020-03-22T21:57:50.5212863Z 
2020-03-22T21:57:50.5212970Z 
2020-03-22T21:57:50.5217281Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-22T21:57:50.5220383Z 
2020-03-22T21:57:50.5220498Z 
2020-03-22T21:57:50.5221073Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-22T21:57:50.5221541Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-22T21:57:50.5221541Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-22T21:57:50.5231750Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-22T21:57:50.5232190Z Build completed unsuccessfully in 1:06:43
2020-03-22T21:57:50.5282054Z == clock drift check ==
2020-03-22T21:57:50.5301056Z   local time: Sun Mar 22 21:57:50 UTC 2020
2020-03-22T21:57:50.6969373Z   network time: Sun, 22 Mar 2020 21:57:50 GMT
2020-03-22T21:57:50.6974984Z == end clock drift check ==
2020-03-22T21:57:52.0510122Z 
2020-03-22T21:57:52.0586567Z ##[error]Bash exited with code '1'.
2020-03-22T21:57:52.0603552Z ##[section]Finishing: Run build
2020-03-22T21:57:52.0659485Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70282/merge to s
2020-03-22T21:57:52.0665080Z Task         : Get sources
2020-03-22T21:57:52.0665491Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T21:57:52.0665851Z Version      : 1.0.0
2020-03-22T21:57:52.0666103Z Author       : Microsoft
2020-03-22T21:57:52.0666103Z Author       : Microsoft
2020-03-22T21:57:52.0666515Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-22T21:57:52.0666974Z ==============================================================================
2020-03-22T21:57:52.4355917Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-22T21:57:52.4403429Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70282/merge to s
2020-03-22T21:57:52.4512774Z Cleaning up task key
2020-03-22T21:57:52.4514540Z Start cleaning up orphan processes.
2020-03-22T21:57:52.4779973Z Terminate orphan process: pid (3984) (python)
2020-03-22T21:57:52.4998663Z ##[section]Finishing: Finalize Job
