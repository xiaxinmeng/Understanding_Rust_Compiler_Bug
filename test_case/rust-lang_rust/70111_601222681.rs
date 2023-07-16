plain
2020-03-19T13:35:31.6437067Z ========================== Starting Command Output ===========================
2020-03-19T13:35:31.6439557Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9704562c-d9b8-4f4d-9e22-915748b1bc7d.sh
2020-03-19T13:35:31.6439784Z 
2020-03-19T13:35:31.6444279Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-19T13:35:31.6462613Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70111/merge to s
2020-03-19T13:35:31.6465806Z Task         : Get sources
2020-03-19T13:35:31.6466037Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T13:35:31.6466313Z Version      : 1.0.0
2020-03-19T13:35:31.6466466Z Author       : Microsoft
---
2020-03-19T13:35:34.1072330Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-19T13:35:34.1234636Z ##[command]git config gc.auto 0
2020-03-19T13:35:34.1271717Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-19T13:35:34.1305160Z ##[command]git config --get-all http.proxy
2020-03-19T13:35:34.1399394Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70111/merge:refs/remotes/pull/70111/merge
---
2020-03-19T14:37:23.4926126Z .................................................................................................... 1700/9803
2020-03-19T14:37:27.9478783Z .................................................................................................... 1800/9803
2020-03-19T14:37:40.0194106Z ..........................................................................i......................... 1900/9803
2020-03-19T14:37:46.3485084Z .................................................................................................... 2000/9803
2020-03-19T14:37:54.7321888Z ................................................................iiiii............................... 2100/9803
2020-03-19T14:38:13.3033531Z .................................................................................................... 2300/9803
2020-03-19T14:38:15.6182588Z .................................................................................................... 2400/9803
2020-03-19T14:38:18.6829106Z .................................................................................................... 2500/9803
2020-03-19T14:38:39.6393757Z .................................................................................................... 2600/9803
---
2020-03-19T14:41:26.3342465Z .....................................i...............i.............................................. 5000/9803
2020-03-19T14:41:35.8335493Z .................................................................................................... 5100/9803
2020-03-19T14:41:42.2783759Z ................................................................................i................... 5200/9803
2020-03-19T14:41:47.9686298Z .................................................................................................... 5300/9803
2020-03-19T14:41:58.7623237Z .............................................................ii.ii........i...i..................... 5400/9803
2020-03-19T14:42:06.9713100Z i................................................................................................... 5600/9803
2020-03-19T14:42:16.2534713Z .....i.............................................................................................. 5700/9803
2020-03-19T14:42:22.6710930Z ........................................................i........................................... 5800/9803
2020-03-19T14:42:29.2808528Z .................................................................................................... 5900/9803
2020-03-19T14:42:29.2808528Z .................................................................................................... 5900/9803
2020-03-19T14:42:37.2153390Z .................................................................................................... 6000/9803
2020-03-19T14:42:45.1590568Z ..................................................ii...i..ii............i........................... 6100/9803
2020-03-19T14:43:06.4567457Z .................................................................................................... 6300/9803
2020-03-19T14:43:13.7954604Z .................................................................................................... 6400/9803
2020-03-19T14:43:13.7954604Z .................................................................................................... 6400/9803
2020-03-19T14:43:22.3605011Z ................................................................................i..ii............... 6500/9803
2020-03-19T14:43:46.6383919Z .................................................................................................... 6700/9803
2020-03-19T14:43:56.2501854Z ...............................................................................i.................... 6800/9803
2020-03-19T14:43:58.4295384Z .................................................................................................... 6900/9803
2020-03-19T14:44:00.5850763Z .................................................................................................... 7000/9803
---
2020-03-19T14:45:48.6310595Z .................................................................................................... 7800/9803
2020-03-19T14:45:54.3782573Z .................................................................................................... 7900/9803
2020-03-19T14:46:00.8003801Z ..................................................................i................................. 8000/9803
2020-03-19T14:46:11.5312785Z .................................................................................................... 8100/9803
2020-03-19T14:46:17.2088897Z ...............iiiiiiiiii.i......................................................................... 8200/9803
2020-03-19T14:46:31.9278639Z .................................................................................................... 8400/9803
2020-03-19T14:46:38.3551377Z .................................................................................................... 8500/9803
2020-03-19T14:46:55.0020020Z .................................................................................................... 8600/9803
2020-03-19T14:47:02.1947417Z .................................................................................................... 8700/9803
---
2020-03-19T14:49:32.6390784Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-03-19T14:49:32.6601658Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-19T14:49:32.8623519Z 
2020-03-19T14:49:32.8623863Z running 183 tests
2020-03-19T14:49:35.8152995Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/183
2020-03-19T14:49:38.7177140Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-03-19T14:49:38.7180533Z 
2020-03-19T14:49:38.7186302Z  finished in 6.058
2020-03-19T14:49:38.7191927Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-03-19T14:49:38.7420017Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-19T14:49:41.0433473Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-03-19T14:49:41.0604517Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-19T14:49:42.0222466Z 
2020-03-19T14:49:42.0222719Z running 9 tests
2020-03-19T14:49:42.0223778Z iiiiiiiii
2020-03-19T14:49:42.0224788Z 
2020-03-19T14:49:42.0224941Z  finished in 0.158
2020-03-19T14:49:42.0225555Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-03-19T14:49:42.0226292Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-19T14:50:03.0040358Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-03-19T14:50:03.0262362Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-19T14:50:03.2115561Z 
2020-03-19T14:50:03.2115893Z running 115 tests
2020-03-19T14:50:16.9055819Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i.Fi.......ii.i.ii.. 100/115
2020-03-19T14:50:18.7966833Z ...iiii.....ii.
2020-03-19T14:50:18.7968109Z 
2020-03-19T14:50:18.7969763Z ---- [debuginfo-gdb] debuginfo/pretty-std-collections.rs stdout ----
2020-03-19T14:50:18.7970111Z NOTE: compiletest thinks it is using GDB with native rust support
2020-03-19T14:50:18.7970441Z NOTE: compiletest thinks it is using GDB version 8001000
2020-03-19T14:50:18.7970441Z NOTE: compiletest thinks it is using GDB version 8001000
2020-03-19T14:50:18.7970621Z 
2020-03-19T14:50:18.7971106Z error: line not found in debugger output: $3 = BTreeMap<i32, i32>(len: 15) = {[0] = 0, [1] = 1, [2] = 2, [3] = 3, [4] = 4, [5] = 5, [6] = 6, [7] = 7, [8] = 8, [9] = 9, [10] = 10, [11] = 11, [12] = 12, [13] = 13, [14] = 14}
2020-03-19T14:50:18.7971646Z status: exit code: 0
2020-03-19T14:50:18.7972638Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std-collections.gdb/pretty-std-collections.debugger.script"
2020-03-19T14:50:18.7973680Z ------------------------------------------
2020-03-19T14:50:18.7973680Z ------------------------------------------
2020-03-19T14:50:18.7974119Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-03-19T14:50:18.7974585Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-03-19T14:50:18.7974964Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-03-19T14:50:18.7975348Z This is free software: you are free to change and redistribute it.
2020-03-19T14:50:18.7975707Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-03-19T14:50:18.7976014Z and "show warranty" for details.
2020-03-19T14:50:18.7976985Z This GDB was configured as "x86_64-linux-gnu".
2020-03-19T14:50:18.7977278Z Type "show configuration" for configuration details.
2020-03-19T14:50:18.7977566Z For bug reporting instructions, please see:
2020-03-19T14:50:18.7977819Z <http://www.gnu.org/software/gdb/bugs/>.
2020-03-19T14:50:18.7978624Z Find the GDB manual and other documentation resources online at:
2020-03-19T14:50:18.7979055Z <http://www.gnu.org/software/gdb/documentation/>.
2020-03-19T14:50:18.7979298Z For help, type "help".
2020-03-19T14:50:18.7980624Z Type "apropos word" to search for commands related to "word".
2020-03-19T14:50:18.7981362Z Breakpoint 1 at 0x15167: file /checkout/src/test/debuginfo/pretty-std-collections.rs, line 71.
2020-03-19T14:50:18.7981698Z [Thread debugging using libthread_db enabled]
2020-03-19T14:50:18.7982185Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-03-19T14:50:18.7982414Z 
2020-03-19T14:50:18.7982929Z Breakpoint 1, pretty_std_collections::main () at /checkout/src/test/debuginfo/pretty-std-collections.rs:71
2020-03-19T14:50:18.7983264Z 71     zzz(); // #break
2020-03-19T14:50:18.7983559Z $1 = BTreeSet<i32>(len: 15) = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14}
2020-03-19T14:50:18.7983847Z $2 = BTreeSet<i32>(len: 0)
2020-03-19T14:50:18.7984065Z $3 = BTreeMap<i32, i32>(len: 15)
2020-03-19T14:50:18.7984302Z $4 = BTreeMap<i32, u32>(len: 0)
2020-03-19T14:50:18.7984575Z $5 = VecDeque<i32>(len: 3, cap: 8) = {5, 3, 7}
2020-03-19T14:50:18.7984870Z $6 = VecDeque<i32>(len: 7, cap: 8) = {2, 3, 4, 5, 6, 7, 8}
2020-03-19T14:50:18.7985274Z 
2020-03-19T14:50:18.7985274Z 
2020-03-19T14:50:18.7985441Z  Inferior 1 [process 1852] will be killed.
2020-03-19T14:50:18.7985594Z 
2020-03-19T14:50:18.7985788Z Quit anyway? (y or n) [answered Y; input not from terminal]
2020-03-19T14:50:18.7986317Z ------------------------------------------
2020-03-19T14:50:18.7986499Z stderr:
2020-03-19T14:50:18.7986841Z ------------------------------------------
2020-03-19T14:50:18.7986841Z ------------------------------------------
2020-03-19T14:50:18.7987332Z Python Exception <class 'gdb.error'> There is no member named Some.: 
2020-03-19T14:50:18.7987872Z ------------------------------------------
2020-03-19T14:50:18.7988045Z 
2020-03-19T14:50:18.7988131Z 
2020-03-19T14:50:18.7988216Z 
---
2020-03-19T14:50:18.7990041Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-19T14:50:18.7990411Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-19T14:50:18.7991605Z 
2020-03-19T14:50:18.7991715Z 
2020-03-19T14:50:18.7995230Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-19T14:50:18.7997705Z 
2020-03-19T14:50:18.7997952Z 
2020-03-19T14:50:18.7998513Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-19T14:50:18.7998857Z Build completed unsuccessfully in 1:09:15
2020-03-19T14:50:18.7998857Z Build completed unsuccessfully in 1:09:15
2020-03-19T14:50:18.8036065Z == clock drift check ==
2020-03-19T14:50:18.8072351Z   local time: Thu Mar 19 14:50:18 UTC 2020
2020-03-19T14:50:19.0958017Z   network time: Thu, 19 Mar 2020 14:50:19 GMT
2020-03-19T14:50:19.0961362Z == end clock drift check ==
2020-03-19T14:50:20.2544843Z 
2020-03-19T14:50:20.2625353Z ##[error]Bash exited with code '1'.
2020-03-19T14:50:20.2640710Z ##[section]Finishing: Run build
2020-03-19T14:50:20.2696328Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70111/merge to s
2020-03-19T14:50:20.2702031Z Task         : Get sources
2020-03-19T14:50:20.2702392Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T14:50:20.2702711Z Version      : 1.0.0
2020-03-19T14:50:20.2702922Z Author       : Microsoft
2020-03-19T14:50:20.2702922Z Author       : Microsoft
2020-03-19T14:50:20.2703290Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-19T14:50:20.2703689Z ==============================================================================
2020-03-19T14:50:20.6278287Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-19T14:50:20.6287796Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70111/merge to s
2020-03-19T14:50:20.6379383Z Cleaning up task key
2020-03-19T14:50:20.6380865Z Start cleaning up orphan processes.
2020-03-19T14:50:20.6576037Z Terminate orphan process: pid (3603) (python)
2020-03-19T14:50:20.6788845Z ##[section]Finishing: Finalize Job
