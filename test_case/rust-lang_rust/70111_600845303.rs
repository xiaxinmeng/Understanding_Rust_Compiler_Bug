plain
2020-03-18T19:29:58.2939041Z ========================== Starting Command Output ===========================
2020-03-18T19:29:58.2942454Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4d041b4b-0151-482b-8707-5940cfa49577.sh
2020-03-18T19:29:58.2942731Z 
2020-03-18T19:29:58.2945967Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-18T19:29:58.2963398Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70111/merge to s
2020-03-18T19:29:58.2966412Z Task         : Get sources
2020-03-18T19:29:58.2966699Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T19:29:58.2966999Z Version      : 1.0.0
2020-03-18T19:29:58.2967190Z Author       : Microsoft
---
2020-03-18T19:29:59.2931244Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-18T19:29:59.2936181Z ##[command]git config gc.auto 0
2020-03-18T19:29:59.2939659Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-18T19:29:59.2942734Z ##[command]git config --get-all http.proxy
2020-03-18T19:29:59.2948588Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70111/merge:refs/remotes/pull/70111/merge
---
2020-03-18T20:23:09.0777967Z .................................................................................................... 1700/9796
2020-03-18T20:23:13.0560621Z .................................................................................................... 1800/9796
2020-03-18T20:23:23.4806593Z ..........................................................................i......................... 1900/9796
2020-03-18T20:23:29.2978276Z .................................................................................................... 2000/9796
2020-03-18T20:23:36.5859077Z ................................................................iiiii............................... 2100/9796
2020-03-18T20:23:52.4736168Z .................................................................................................... 2300/9796
2020-03-18T20:23:54.4874536Z .................................................................................................... 2400/9796
2020-03-18T20:23:57.3086803Z .................................................................................................... 2500/9796
2020-03-18T20:24:15.4110195Z .................................................................................................... 2600/9796
---
2020-03-18T20:26:39.3101992Z ....................................i...............i............................................... 5000/9796
2020-03-18T20:26:47.2763127Z .................................................................................................... 5100/9796
2020-03-18T20:26:52.9361591Z ...............................................................................i.................... 5200/9796
2020-03-18T20:26:57.8137900Z .................................................................................................... 5300/9796
2020-03-18T20:27:06.6948143Z ............................................................ii.ii........i...i...................... 5400/9796
2020-03-18T20:27:10.4378494Z ...................................................................................................i 5500/9796
2020-03-18T20:27:22.1623422Z .................................................................................................... 5700/9796
2020-03-18T20:27:27.6625098Z .....................................................i.............................................. 5800/9796
2020-03-18T20:27:33.2413487Z .................................................................................................... 5900/9796
2020-03-18T20:27:41.6367772Z .................................................................................................... 6000/9796
2020-03-18T20:27:41.6367772Z .................................................................................................... 6000/9796
2020-03-18T20:27:47.3782330Z ...............................................ii...i..ii...........i............................... 6100/9796
2020-03-18T20:28:05.7808982Z .................................................................................................... 6300/9796
2020-03-18T20:28:09.1412987Z .................................................................................................... 6400/9796
2020-03-18T20:28:09.1412987Z .................................................................................................... 6400/9796
2020-03-18T20:28:13.2417890Z .............................................................................i..ii.................. 6500/9796
2020-03-18T20:28:33.4621079Z .................................................................................................... 6700/9796
2020-03-18T20:28:41.8380489Z ...........................................................................i........................ 6800/9796
2020-03-18T20:28:43.6882969Z .................................................................................................... 6900/9796
2020-03-18T20:28:45.6404712Z .................................................................................................... 7000/9796
---
2020-03-18T20:30:19.7987359Z .................................................................................................... 7800/9796
2020-03-18T20:30:24.4144274Z .................................................................................................... 7900/9796
2020-03-18T20:30:30.2208576Z .............................................................i...................................... 8000/9796
2020-03-18T20:30:39.4594609Z .................................................................................................... 8100/9796
2020-03-18T20:30:44.6871031Z ..........iiiiiiiiii.i.............................................................................. 8200/9796
2020-03-18T20:30:57.1598589Z .................................................................................................... 8400/9796
2020-03-18T20:31:04.3743357Z .................................................................................................... 8500/9796
2020-03-18T20:31:16.5491345Z .................................................................................................... 8600/9796
2020-03-18T20:31:22.4560805Z .................................................................................................... 8700/9796
---
2020-03-18T20:33:31.4967771Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-03-18T20:33:31.5150811Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-18T20:33:31.7218002Z 
2020-03-18T20:33:31.7220118Z running 183 tests
2020-03-18T20:33:34.3205925Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/183
2020-03-18T20:33:36.6970335Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-03-18T20:33:36.6978169Z 
2020-03-18T20:33:36.6985121Z  finished in 5.183
2020-03-18T20:33:36.6995434Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-03-18T20:33:36.7173554Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-18T20:33:38.7377606Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-03-18T20:33:38.7560564Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-18T20:33:38.9001533Z 
2020-03-18T20:33:38.9001969Z running 9 tests
2020-03-18T20:33:38.9003135Z iiiiiiiii
2020-03-18T20:33:38.9004237Z 
2020-03-18T20:33:38.9009630Z  finished in 0.144
2020-03-18T20:33:38.9014254Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-03-18T20:33:38.9203852Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-18T20:33:59.2360925Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-03-18T20:33:59.5728999Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-18T20:33:59.7522420Z 
2020-03-18T20:33:59.7523519Z running 115 tests
2020-03-18T20:34:12.5371535Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i.Fi.......ii.i.ii.. 100/115
2020-03-18T20:34:14.0468281Z ...iiii.....ii.
2020-03-18T20:34:14.0469905Z 
2020-03-18T20:34:14.0474104Z ---- [debuginfo-gdb] debuginfo/pretty-std-collections.rs stdout ----
2020-03-18T20:34:14.0474104Z ---- [debuginfo-gdb] debuginfo/pretty-std-collections.rs stdout ----
2020-03-18T20:34:14.0476368Z NOTE: compiletest thinks it is using GDB with native rust support
2020-03-18T20:34:14.0476665Z NOTE: compiletest thinks it is using GDB version 8001000
2020-03-18T20:34:14.0476836Z 
2020-03-18T20:34:14.0477167Z error: line not found in debugger output: $1 = BTreeSet<i32>(len: 15) = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14}
2020-03-18T20:34:14.0477511Z status: exit code: 0
2020-03-18T20:34:14.0478436Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std-collections.gdb/pretty-std-collections.debugger.script"
2020-03-18T20:34:14.0479412Z ------------------------------------------
2020-03-18T20:34:14.0479412Z ------------------------------------------
2020-03-18T20:34:14.0484190Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-03-18T20:34:14.0484509Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-03-18T20:34:14.0484819Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-03-18T20:34:14.0485273Z This is free software: you are free to change and redistribute it.
2020-03-18T20:34:14.0485609Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-03-18T20:34:14.0485867Z and "show warranty" for details.
2020-03-18T20:34:14.0486547Z This GDB was configured as "x86_64-linux-gnu".
2020-03-18T20:34:14.0486839Z Type "show configuration" for configuration details.
2020-03-18T20:34:14.0487104Z For bug reporting instructions, please see:
2020-03-18T20:34:14.0487354Z <http://www.gnu.org/software/gdb/bugs/>.
2020-03-18T20:34:14.0487657Z Find the GDB manual and other documentation resources online at:
2020-03-18T20:34:14.0487956Z <http://www.gnu.org/software/gdb/documentation/>.
2020-03-18T20:34:14.0488192Z For help, type "help".
2020-03-18T20:34:14.0488459Z Type "apropos word" to search for commands related to "word".
2020-03-18T20:34:14.0489212Z Breakpoint 1 at 0x151c7: file /checkout/src/test/debuginfo/pretty-std-collections.rs, line 71.
2020-03-18T20:34:14.0489577Z [Thread debugging using libthread_db enabled]
2020-03-18T20:34:14.0490118Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-03-18T20:34:14.0490338Z 
2020-03-18T20:34:14.0490866Z Breakpoint 1, pretty_std_collections::main () at /checkout/src/test/debuginfo/pretty-std-collections.rs:71
2020-03-18T20:34:14.0491234Z 71     zzz(); // #break
2020-03-18T20:34:14.0491439Z $1 = BTreeSet<i32>(len: 15)
2020-03-18T20:34:14.0491659Z $2 = BTreeSet<i32>(len: 0)
2020-03-18T20:34:14.0491907Z $3 = BTreeMap<i32, i32>(len: 15)
2020-03-18T20:34:14.0492148Z $4 = BTreeMap<i32, u32>(len: 0)
2020-03-18T20:34:14.0492415Z $5 = VecDeque<i32>(len: 3, cap: 8) = {5, 3, 7}
2020-03-18T20:34:14.0492886Z $6 = VecDeque<i32>(len: 7, cap: 8) = {2, 3, 4, 5, 6, 7, 8}
2020-03-18T20:34:14.0495122Z 
2020-03-18T20:34:14.0495122Z 
2020-03-18T20:34:14.0495303Z  Inferior 1 [process 1698] will be killed.
2020-03-18T20:34:14.0495483Z 
2020-03-18T20:34:14.0495689Z Quit anyway? (y or n) [answered Y; input not from terminal]
2020-03-18T20:34:14.0496299Z ------------------------------------------
2020-03-18T20:34:14.0496509Z stderr:
2020-03-18T20:34:14.0496859Z ------------------------------------------
2020-03-18T20:34:14.0496859Z ------------------------------------------
2020-03-18T20:34:14.0497361Z Python Exception <class 'gdb.error'> There is no member named Some.: 
2020-03-18T20:34:14.0497952Z Python Exception <class 'gdb.error'> There is no member named Some.: 
2020-03-18T20:34:14.0498520Z ------------------------------------------
2020-03-18T20:34:14.0498681Z 
2020-03-18T20:34:14.0498905Z 
2020-03-18T20:34:14.0498999Z 
---
2020-03-18T20:34:14.0500948Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-18T20:34:14.0501359Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-18T20:34:14.0501582Z 
2020-03-18T20:34:14.0501672Z 
2020-03-18T20:34:14.0505372Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-18T20:34:14.0508025Z 
2020-03-18T20:34:14.0508120Z 
2020-03-18T20:34:14.0508621Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-18T20:34:14.0508973Z Build completed unsuccessfully in 1:00:10
2020-03-18T20:34:14.0508973Z Build completed unsuccessfully in 1:00:10
2020-03-18T20:34:14.0554870Z == clock drift check ==
2020-03-18T20:34:14.0592899Z   local time: Wed Mar 18 20:34:14 UTC 2020
2020-03-18T20:34:14.6103154Z   network time: Wed, 18 Mar 2020 20:34:14 GMT
2020-03-18T20:34:14.6103809Z == end clock drift check ==
2020-03-18T20:34:15.8365073Z 
2020-03-18T20:34:15.8420180Z ##[error]Bash exited with code '1'.
2020-03-18T20:34:15.8432800Z ##[section]Finishing: Run build
2020-03-18T20:34:15.8487339Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70111/merge to s
2020-03-18T20:34:15.8492163Z Task         : Get sources
2020-03-18T20:34:15.8492508Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T20:34:15.8492809Z Version      : 1.0.0
2020-03-18T20:34:15.8493022Z Author       : Microsoft
2020-03-18T20:34:15.8493022Z Author       : Microsoft
2020-03-18T20:34:15.8493374Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-18T20:34:15.8493756Z ==============================================================================
2020-03-18T20:34:16.1482686Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-18T20:34:16.1523063Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70111/merge to s
2020-03-18T20:34:16.1596800Z Cleaning up task key
2020-03-18T20:34:16.1597992Z Start cleaning up orphan processes.
2020-03-18T20:34:16.1749971Z Terminate orphan process: pid (3987) (python)
2020-03-18T20:34:16.1889775Z ##[section]Finishing: Finalize Job
