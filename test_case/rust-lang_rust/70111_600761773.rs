plain
2020-03-18T16:16:24.5180171Z ========================== Starting Command Output ===========================
2020-03-18T16:16:24.5183974Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b6ed9950-bf29-4261-b711-d82be7800f15.sh
2020-03-18T16:16:24.5184280Z 
2020-03-18T16:16:24.5188552Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-18T16:16:24.5207344Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70111/merge to s
2020-03-18T16:16:24.5210556Z Task         : Get sources
2020-03-18T16:16:24.5210887Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T16:16:24.5211188Z Version      : 1.0.0
2020-03-18T16:16:24.5211389Z Author       : Microsoft
---
2020-03-18T16:16:25.5114414Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-18T16:16:25.5123354Z ##[command]git config gc.auto 0
2020-03-18T16:16:25.5126936Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-18T16:16:25.5130184Z ##[command]git config --get-all http.proxy
2020-03-18T16:16:25.5136074Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70111/merge:refs/remotes/pull/70111/merge
---
2020-03-18T17:12:18.8328926Z .................................................................................................... 1700/9796
2020-03-18T17:12:23.2213048Z .................................................................................................... 1800/9796
2020-03-18T17:12:34.5888700Z ..........................................................................i......................... 1900/9796
2020-03-18T17:12:40.9633370Z .................................................................................................... 2000/9796
2020-03-18T17:12:48.8063320Z ................................................................iiiii............................... 2100/9796
2020-03-18T17:13:06.0995977Z .................................................................................................... 2300/9796
2020-03-18T17:13:08.3171736Z .................................................................................................... 2400/9796
2020-03-18T17:13:11.2214659Z .................................................................................................... 2500/9796
2020-03-18T17:13:30.6863355Z .................................................................................................... 2600/9796
---
2020-03-18T17:16:08.4274437Z ....................................i...............i............................................... 5000/9796
2020-03-18T17:16:17.1106357Z .................................................................................................... 5100/9796
2020-03-18T17:16:23.2737192Z ...............................................................................i.................... 5200/9796
2020-03-18T17:16:28.5582631Z .................................................................................................... 5300/9796
2020-03-18T17:16:38.0591138Z ............................................................ii.ii........i...i...................... 5400/9796
2020-03-18T17:16:42.0978170Z ...................................................................................................i 5500/9796
2020-03-18T17:16:54.7603806Z .................................................................................................... 5700/9796
2020-03-18T17:17:00.6410840Z .....................................................i.............................................. 5800/9796
2020-03-18T17:17:06.8472120Z .................................................................................................... 5900/9796
2020-03-18T17:17:15.9601836Z .................................................................................................... 6000/9796
2020-03-18T17:17:15.9601836Z .................................................................................................... 6000/9796
2020-03-18T17:17:22.0914785Z ...............................................ii...i..ii...........i............................... 6100/9796
2020-03-18T17:17:42.0777057Z .................................................................................................... 6300/9796
2020-03-18T17:17:49.0872765Z .................................................................................................... 6400/9796
2020-03-18T17:17:49.0872765Z .................................................................................................... 6400/9796
2020-03-18T17:17:53.4585504Z .............................................................................i..ii.................. 6500/9796
2020-03-18T17:18:14.8026891Z .................................................................................................... 6700/9796
2020-03-18T17:18:23.7186721Z ...........................................................................i........................ 6800/9796
2020-03-18T17:18:25.7652971Z .................................................................................................... 6900/9796
2020-03-18T17:18:27.9089331Z .................................................................................................... 7000/9796
---
2020-03-18T17:20:07.1897215Z .................................................................................................... 7800/9796
2020-03-18T17:20:12.2825838Z .................................................................................................... 7900/9796
2020-03-18T17:20:18.1268278Z .............................................................i...................................... 8000/9796
2020-03-18T17:20:27.6352706Z .................................................................................................... 8100/9796
2020-03-18T17:20:33.0328519Z ..........iiiiiiiiii.i.............................................................................. 8200/9796
2020-03-18T17:20:46.3090952Z .................................................................................................... 8400/9796
2020-03-18T17:20:54.3785382Z .................................................................................................... 8500/9796
2020-03-18T17:21:08.1795717Z .................................................................................................... 8600/9796
2020-03-18T17:21:14.7586435Z .................................................................................................... 8700/9796
---
2020-03-18T17:23:30.3961500Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-03-18T17:23:30.4204572Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-18T17:23:30.6135963Z 
2020-03-18T17:23:30.6136329Z running 183 tests
2020-03-18T17:23:33.4943103Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/183
2020-03-18T17:23:36.0919235Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-03-18T17:23:36.0930147Z 
2020-03-18T17:23:36.0930321Z  finished in 5.672
2020-03-18T17:23:36.0933889Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-03-18T17:23:36.1132478Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-18T17:23:38.2680004Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-03-18T17:23:38.2886922Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-18T17:23:38.4464850Z 
2020-03-18T17:23:38.4465262Z running 9 tests
2020-03-18T17:23:38.4466627Z iiiiiiiii
2020-03-18T17:23:38.4472281Z 
2020-03-18T17:23:38.4472519Z  finished in 0.158
2020-03-18T17:23:38.4478822Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-03-18T17:23:38.4687057Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-18T17:23:59.1859911Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-03-18T17:23:59.2091832Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-18T17:23:59.3948438Z 
2020-03-18T17:23:59.3948868Z running 115 tests
2020-03-18T17:24:12.7769953Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i.Fi.......ii.i.ii.. 100/115
2020-03-18T17:24:14.2745764Z ...iiii.....ii.
2020-03-18T17:24:14.2748152Z 
2020-03-18T17:24:14.2753263Z ---- [debuginfo-gdb] debuginfo/pretty-std-collections.rs stdout ----
2020-03-18T17:24:14.2753263Z ---- [debuginfo-gdb] debuginfo/pretty-std-collections.rs stdout ----
2020-03-18T17:24:14.2755699Z NOTE: compiletest thinks it is using GDB with native rust support
2020-03-18T17:24:14.2756054Z NOTE: compiletest thinks it is using GDB version 8001000
2020-03-18T17:24:14.2756278Z 
2020-03-18T17:24:14.2756646Z error: line not found in debugger output: $1 = BTreeSet<i32>(len: 15) = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14}
2020-03-18T17:24:14.2757087Z status: exit code: 0
2020-03-18T17:24:14.2761175Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std-collections.gdb/pretty-std-collections.debugger.script"
2020-03-18T17:24:14.2762196Z ------------------------------------------
2020-03-18T17:24:14.2762196Z ------------------------------------------
2020-03-18T17:24:14.2762703Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-03-18T17:24:14.2762999Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-03-18T17:24:14.2763366Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-03-18T17:24:14.2763794Z This is free software: you are free to change and redistribute it.
2020-03-18T17:24:14.2764174Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-03-18T17:24:14.2764482Z and "show warranty" for details.
2020-03-18T17:24:14.2764938Z This GDB was configured as "x86_64-linux-gnu".
2020-03-18T17:24:14.2765389Z Type "show configuration" for configuration details.
2020-03-18T17:24:14.2765682Z For bug reporting instructions, please see:
2020-03-18T17:24:14.2765967Z <http://www.gnu.org/software/gdb/bugs/>.
2020-03-18T17:24:14.2766365Z Find the GDB manual and other documentation resources online at:
2020-03-18T17:24:14.2766685Z <http://www.gnu.org/software/gdb/documentation/>.
2020-03-18T17:24:14.2766950Z For help, type "help".
2020-03-18T17:24:14.2767216Z Type "apropos word" to search for commands related to "word".
2020-03-18T17:24:14.2767861Z Breakpoint 1 at 0x10a9b: file /checkout/src/test/debuginfo/pretty-std-collections.rs, line 63.
2020-03-18T17:24:14.2768255Z [Thread debugging using libthread_db enabled]
2020-03-18T17:24:14.2768785Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-03-18T17:24:14.2769020Z 
2020-03-18T17:24:14.2769803Z Breakpoint 1, pretty_std_collections::main () at /checkout/src/test/debuginfo/pretty-std-collections.rs:63
2020-03-18T17:24:14.2770572Z 63     zzz(); // #break
2020-03-18T17:24:14.2770795Z $1 = BTreeSet<i32>(len: 15)
2020-03-18T17:24:14.2771050Z $2 = BTreeMap<i32, i32>(len: 15)
2020-03-18T17:24:14.2771476Z $3 = VecDeque<i32>(len: 3, cap: 8) = {5, 3, 7}
2020-03-18T17:24:14.2771820Z $4 = VecDeque<i32>(len: 7, cap: 8) = {2, 3, 4, 5, 6, 7, 8}
2020-03-18T17:24:14.2772283Z 
2020-03-18T17:24:14.2772283Z 
2020-03-18T17:24:14.2772474Z  Inferior 1 [process 1697] will be killed.
2020-03-18T17:24:14.2772647Z 
2020-03-18T17:24:14.2772869Z Quit anyway? (y or n) [answered Y; input not from terminal]
2020-03-18T17:24:14.2773490Z ------------------------------------------
2020-03-18T17:24:14.2773696Z stderr:
2020-03-18T17:24:14.2774083Z ------------------------------------------
2020-03-18T17:24:14.2774083Z ------------------------------------------
2020-03-18T17:24:14.2774625Z Python Exception <class 'gdb.error'> There is no member named node.: 
2020-03-18T17:24:14.2775243Z Python Exception <class 'gdb.error'> There is no member named node.: 
2020-03-18T17:24:14.2775877Z ------------------------------------------
2020-03-18T17:24:14.2776052Z 
2020-03-18T17:24:14.2776151Z 
2020-03-18T17:24:14.2776248Z 
---
2020-03-18T17:24:14.2778300Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-18T17:24:14.2778721Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-18T17:24:14.2779689Z 
2020-03-18T17:24:14.2779799Z 
2020-03-18T17:24:14.2784177Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-18T17:24:14.2787066Z 
2020-03-18T17:24:14.2787233Z 
2020-03-18T17:24:14.2787774Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-18T17:24:14.2788169Z Build completed unsuccessfully in 1:03:17
2020-03-18T17:24:14.2788169Z Build completed unsuccessfully in 1:03:17
2020-03-18T17:24:14.2835357Z == clock drift check ==
2020-03-18T17:24:14.2852776Z   local time: Wed Mar 18 17:24:14 UTC 2020
2020-03-18T17:24:14.8362654Z   network time: Wed, 18 Mar 2020 17:24:14 GMT
2020-03-18T17:24:14.8369422Z == end clock drift check ==
2020-03-18T17:24:16.1248440Z 
2020-03-18T17:24:16.1352661Z ##[error]Bash exited with code '1'.
2020-03-18T17:24:16.1367041Z ##[section]Finishing: Run build
2020-03-18T17:24:16.1423592Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70111/merge to s
2020-03-18T17:24:16.1428876Z Task         : Get sources
2020-03-18T17:24:16.1429251Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T17:24:16.1429586Z Version      : 1.0.0
2020-03-18T17:24:16.1429835Z Author       : Microsoft
2020-03-18T17:24:16.1429835Z Author       : Microsoft
2020-03-18T17:24:16.1430218Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-18T17:24:16.1430630Z ==============================================================================
2020-03-18T17:24:16.4760254Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-18T17:24:16.4801324Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70111/merge to s
2020-03-18T17:24:16.4895962Z Cleaning up task key
2020-03-18T17:24:16.4897256Z Start cleaning up orphan processes.
2020-03-18T17:24:16.5078650Z Terminate orphan process: pid (4471) (python)
2020-03-18T17:24:16.5242477Z ##[section]Finishing: Finalize Job
