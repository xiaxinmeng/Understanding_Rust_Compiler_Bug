plain
2020-03-18T23:26:37.3016310Z ========================== Starting Command Output ===========================
2020-03-18T23:26:37.3020656Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/090506d7-64fa-47de-888b-75246e5ad21c.sh
2020-03-18T23:26:37.3021113Z 
2020-03-18T23:26:37.3024925Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-18T23:26:37.3045874Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70111/merge to s
2020-03-18T23:26:37.3049735Z Task         : Get sources
2020-03-18T23:26:37.3050016Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T23:26:37.3050285Z Version      : 1.0.0
2020-03-18T23:26:37.3050537Z Author       : Microsoft
---
2020-03-18T23:26:39.4836608Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-18T23:26:39.4844015Z ##[command]git config gc.auto 0
2020-03-18T23:26:39.4847985Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-18T23:26:39.4851248Z ##[command]git config --get-all http.proxy
2020-03-18T23:26:39.4859080Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70111/merge:refs/remotes/pull/70111/merge
---
2020-03-19T00:28:10.8142223Z .................................................................................................... 1700/9796
2020-03-19T00:28:15.3194513Z .................................................................................................... 1800/9796
2020-03-19T00:28:27.3734600Z ..........................................................................i......................... 1900/9796
2020-03-19T00:28:34.0600922Z .................................................................................................... 2000/9796
2020-03-19T00:28:42.6519805Z ................................................................iiiii............................... 2100/9796
2020-03-19T00:29:01.2981477Z .................................................................................................... 2300/9796
2020-03-19T00:29:03.7235220Z .................................................................................................... 2400/9796
2020-03-19T00:29:06.8928631Z .................................................................................................... 2500/9796
2020-03-19T00:29:28.2352850Z .................................................................................................... 2600/9796
---
2020-03-19T00:32:22.1037966Z ....................................i...............i............................................... 5000/9796
2020-03-19T00:32:31.6058840Z .................................................................................................... 5100/9796
2020-03-19T00:32:38.9971408Z ...............................................................................i.................... 5200/9796
2020-03-19T00:32:44.6984644Z .................................................................................................... 5300/9796
2020-03-19T00:32:55.4316852Z ............................................................ii.ii........i...i...................... 5400/9796
2020-03-19T00:32:59.9794370Z ...................................................................................................i 5500/9796
2020-03-19T00:33:14.6239592Z .................................................................................................... 5700/9796
2020-03-19T00:33:21.3836117Z .....................................................i.............................................. 5800/9796
2020-03-19T00:33:28.4601835Z .................................................................................................... 5900/9796
2020-03-19T00:33:38.7877520Z .................................................................................................... 6000/9796
2020-03-19T00:33:38.7877520Z .................................................................................................... 6000/9796
2020-03-19T00:33:45.4985407Z ...............................................ii...i..ii...........i............................... 6100/9796
2020-03-19T00:34:07.0351175Z .................................................................................................... 6300/9796
2020-03-19T00:34:14.8183627Z .................................................................................................... 6400/9796
2020-03-19T00:34:14.8183627Z .................................................................................................... 6400/9796
2020-03-19T00:34:24.1641603Z .............................................................................i..ii.................. 6500/9796
2020-03-19T00:34:48.4250831Z .................................................................................................... 6700/9796
2020-03-19T00:34:58.0884009Z ...........................................................................i........................ 6800/9796
2020-03-19T00:35:00.2043131Z .................................................................................................... 6900/9796
2020-03-19T00:35:02.4816917Z .................................................................................................... 7000/9796
---
2020-03-19T00:36:51.5261564Z .................................................................................................... 7800/9796
2020-03-19T00:36:57.0672442Z .................................................................................................... 7900/9796
2020-03-19T00:37:03.6149439Z .............................................................i...................................... 8000/9796
2020-03-19T00:37:14.1656827Z .................................................................................................... 8100/9796
2020-03-19T00:37:20.3030009Z ..........iiiiiiiiii.i.............................................................................. 8200/9796
2020-03-19T00:37:35.2970210Z .................................................................................................... 8400/9796
2020-03-19T00:37:43.9672387Z .................................................................................................... 8500/9796
2020-03-19T00:37:58.5091228Z .................................................................................................... 8600/9796
2020-03-19T00:38:05.7058157Z .................................................................................................... 8700/9796
---
2020-03-19T00:40:38.3034174Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-03-19T00:40:38.3219815Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-19T00:40:38.5490102Z 
2020-03-19T00:40:38.5490439Z running 183 tests
2020-03-19T00:40:41.6395632Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/183
2020-03-19T00:40:44.5333052Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-03-19T00:40:44.5336135Z 
2020-03-19T00:40:44.5342198Z  finished in 6.212
2020-03-19T00:40:44.5349036Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-03-19T00:40:44.5544344Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-19T00:40:46.9831548Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-03-19T00:40:47.0021032Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-19T00:40:47.1747102Z 
2020-03-19T00:40:47.1748140Z running 9 tests
2020-03-19T00:40:47.1749240Z iiiiiiiii
2020-03-19T00:40:47.1750273Z 
2020-03-19T00:40:47.1750580Z  finished in 0.172
2020-03-19T00:40:47.1755894Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-03-19T00:40:47.1954116Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-19T00:41:09.3812076Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-03-19T00:41:09.4060128Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-19T00:41:10.2370545Z 
2020-03-19T00:41:10.2370963Z running 115 tests
2020-03-19T00:41:25.2488182Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i.Fi.......ii.i.ii.. 100/115
2020-03-19T00:41:27.0738169Z ...iiii.....ii.
2020-03-19T00:41:27.0738874Z 
2020-03-19T00:41:27.0739380Z ---- [debuginfo-gdb] debuginfo/pretty-std-collections.rs stdout ----
2020-03-19T00:41:27.0739380Z ---- [debuginfo-gdb] debuginfo/pretty-std-collections.rs stdout ----
2020-03-19T00:41:27.0739960Z NOTE: compiletest thinks it is using GDB with native rust support
2020-03-19T00:41:27.0740337Z NOTE: compiletest thinks it is using GDB version 8001000
2020-03-19T00:41:27.0740547Z 
2020-03-19T00:41:27.0740925Z error: line not found in debugger output: $1 = BTreeSet<i32>(len: 15) = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14}
2020-03-19T00:41:27.0741371Z status: exit code: 0
2020-03-19T00:41:27.0742249Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std-collections.gdb/pretty-std-collections.debugger.script"
2020-03-19T00:41:27.0743127Z ------------------------------------------
2020-03-19T00:41:27.0743127Z ------------------------------------------
2020-03-19T00:41:27.0743603Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-03-19T00:41:27.0743907Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-03-19T00:41:27.0744300Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-03-19T00:41:27.0744723Z This is free software: you are free to change and redistribute it.
2020-03-19T00:41:27.0745115Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-03-19T00:41:27.0745446Z and "show warranty" for details.
2020-03-19T00:41:27.0745892Z This GDB was configured as "x86_64-linux-gnu".
2020-03-19T00:41:27.0746202Z Type "show configuration" for configuration details.
2020-03-19T00:41:27.0746510Z For bug reporting instructions, please see:
2020-03-19T00:41:27.0746786Z <http://www.gnu.org/software/gdb/bugs/>.
2020-03-19T00:41:27.0747351Z Find the GDB manual and other documentation resources online at:
2020-03-19T00:41:27.0747777Z <http://www.gnu.org/software/gdb/documentation/>.
2020-03-19T00:41:27.0748036Z For help, type "help".
2020-03-19T00:41:27.0748311Z Type "apropos word" to search for commands related to "word".
2020-03-19T00:41:27.0749203Z Breakpoint 1 at 0x15167: file /checkout/src/test/debuginfo/pretty-std-collections.rs, line 71.
2020-03-19T00:41:27.0749614Z [Thread debugging using libthread_db enabled]
2020-03-19T00:41:27.0750174Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-03-19T00:41:27.0750435Z 
2020-03-19T00:41:27.0751018Z Breakpoint 1, pretty_std_collections::main () at /checkout/src/test/debuginfo/pretty-std-collections.rs:71
2020-03-19T00:41:27.0751629Z 71     zzz(); // #break
2020-03-19T00:41:27.0751874Z $1 = BTreeSet<i32>(len: 15)
2020-03-19T00:41:27.0752119Z $2 = BTreeSet<i32>(len: 0)
2020-03-19T00:41:27.0752373Z $3 = BTreeMap<i32, i32>(len: 15)
2020-03-19T00:41:27.0753354Z $4 = BTreeMap<i32, u32>(len: 0)
2020-03-19T00:41:27.0753673Z $5 = VecDeque<i32>(len: 3, cap: 8) = {5, 3, 7}
2020-03-19T00:41:27.0754016Z $6 = VecDeque<i32>(len: 7, cap: 8) = {2, 3, 4, 5, 6, 7, 8}
2020-03-19T00:41:27.0754485Z 
2020-03-19T00:41:27.0754485Z 
2020-03-19T00:41:27.0754682Z  Inferior 1 [process 1698] will be killed.
2020-03-19T00:41:27.0754862Z 
2020-03-19T00:41:27.0755232Z Quit anyway? (y or n) [answered Y; input not from terminal]
2020-03-19T00:41:27.0755946Z ------------------------------------------
2020-03-19T00:41:27.0756162Z stderr:
2020-03-19T00:41:27.0756559Z ------------------------------------------
2020-03-19T00:41:27.0756741Z 
2020-03-19T00:41:27.0756741Z 
2020-03-19T00:41:27.0757653Z   core::option::Some (alloc::collections::btree::node::Root<i32, ()> {node: alloc::collections::btree::node::BoxedNode<i32, ()> {ptr: core::ptr::unique::Unique<alloc::collections::btree::node::LeafNode<i32, ()>> {pointer: 0x555555779bc0, _marker: core::marker::PhantomData<alloc::collections::btree::node::LeafNode<i32, ()>>}}, height: 1})
2020-03-19T00:41:27.0758951Z Python Exception <class 'gdb.error'> There is no member named Some.: 
2020-03-19T00:41:27.0759594Z Python Exception <class 'gdb.error'> There is no member named Some.: 
2020-03-19T00:41:27.0760829Z ------------------------------------------
2020-03-19T00:41:27.0761011Z 
2020-03-19T00:41:27.0761112Z 
2020-03-19T00:41:27.0761212Z 
2020-03-19T00:41:27.0761212Z 
2020-03-19T00:41:27.0761357Z failures:
2020-03-19T00:41:27.0761802Z     [debuginfo-gdb] debuginfo/pretty-std-collections.rs
2020-03-19T00:41:27.0762013Z 
2020-03-19T00:41:27.0762527Z test result: FAILED. 76 passed; 1 failed; 38 ignored; 0 measured; 0 filtered out
2020-03-19T00:41:27.0762807Z 
2020-03-19T00:41:27.0762935Z 
2020-03-19T00:41:27.0763035Z 
2020-03-19T00:41:27.0766997Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-19T00:41:27.0770324Z 
2020-03-19T00:41:27.0770428Z 
2020-03-19T00:41:27.0771019Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-19T00:41:27.0771462Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-19T00:41:27.0771462Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-19T00:41:27.0777141Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-19T00:41:27.0777577Z Build completed unsuccessfully in 1:10:30
2020-03-19T00:41:27.0849486Z == clock drift check ==
2020-03-19T00:41:27.0860445Z   local time: Thu Mar 19 00:41:27 UTC 2020
2020-03-19T00:41:27.3837542Z   network time: Thu, 19 Mar 2020 00:41:27 GMT
2020-03-19T00:41:27.3847502Z == end clock drift check ==
2020-03-19T00:41:28.2641203Z 
2020-03-19T00:41:28.2725899Z ##[error]Bash exited with code '1'.
2020-03-19T00:41:28.2743156Z ##[section]Finishing: Run build
2020-03-19T00:41:28.2802844Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70111/merge to s
2020-03-19T00:41:28.2808686Z Task         : Get sources
2020-03-19T00:41:28.2809030Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T00:41:28.2809548Z Version      : 1.0.0
2020-03-19T00:41:28.2809778Z Author       : Microsoft
2020-03-19T00:41:28.2809778Z Author       : Microsoft
2020-03-19T00:41:28.2810145Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-19T00:41:28.2810590Z ==============================================================================
2020-03-19T00:41:28.6538939Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-19T00:41:28.6586986Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70111/merge to s
2020-03-19T00:41:28.6686133Z Cleaning up task key
2020-03-19T00:41:28.6687630Z Start cleaning up orphan processes.
2020-03-19T00:41:28.6892503Z Terminate orphan process: pid (4065) (python)
2020-03-19T00:41:28.7079390Z ##[section]Finishing: Finalize Job
