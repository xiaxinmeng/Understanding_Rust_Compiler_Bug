plain
2019-11-17T15:25:10.9075415Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-17T15:25:10.9283881Z ##[command]git config gc.auto 0
2019-11-17T15:25:10.9364331Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-17T15:25:10.9431560Z ##[command]git config --get-all http.proxy
2019-11-17T15:25:10.9563678Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66492/merge:refs/remotes/pull/66492/merge
---
2019-11-17T16:22:44.7939994Z .................................................................................................... 1500/9251
2019-11-17T16:22:50.8125434Z .................................................................................................... 1600/9251
2019-11-17T16:22:59.4151800Z .................................................................................................... 1700/9251
2019-11-17T16:23:08.2920764Z ........i........................................................................................... 1800/9251
2019-11-17T16:23:14.8062519Z .............................................................................................iiiii.. 1900/9251
2019-11-17T16:23:36.1372566Z .................................................................................................... 2100/9251
2019-11-17T16:23:38.5253589Z .................................................................................................... 2200/9251
2019-11-17T16:23:40.9988663Z .................................................................................................... 2300/9251
2019-11-17T16:23:47.0378362Z .................................................................................................... 2400/9251
---
2019-11-17T16:27:29.9847441Z .................................................................................................... 5400/9251
2019-11-17T16:27:40.5624204Z ...............................................................................i.................... 5500/9251
2019-11-17T16:27:48.4865159Z .................................................................................................... 5600/9251
2019-11-17T16:27:54.9634501Z .................................................................................................... 5700/9251
2019-11-17T16:28:05.1658658Z .................................................................ii...i..ii...........i............. 5800/9251
2019-11-17T16:28:27.2191011Z .................................................................................................... 6000/9251
2019-11-17T16:28:35.6196241Z .................................................................................................... 6100/9251
2019-11-17T16:28:35.6196241Z .................................................................................................... 6100/9251
2019-11-17T16:28:40.3277643Z ....................................................................................i..ii........... 6200/9251
2019-11-17T16:29:07.6325643Z .................................................................................................... 6400/9251
2019-11-17T16:29:12.3775635Z ....................................................i............................................... 6500/9251
2019-11-17T16:29:14.5762260Z .................................................................................................... 6600/9251
2019-11-17T16:29:16.9658813Z .........................................i.......................................................... 6700/9251
---
2019-11-17T16:34:28.3153509Z  finished in 5.634
2019-11-17T16:34:28.3339358Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-17T16:34:28.5309701Z 
2019-11-17T16:34:28.5310652Z running 156 tests
2019-11-17T16:34:31.4223042Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/156
2019-11-17T16:34:33.3256977Z .i.i..iiii..............i.........iii.i.........ii......
2019-11-17T16:34:33.3258985Z 
2019-11-17T16:34:33.3264659Z  finished in 4.992
2019-11-17T16:34:33.3454952Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-17T16:34:33.5113114Z 
---
2019-11-17T16:34:35.3955405Z  finished in 2.049
2019-11-17T16:34:35.4135654Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-17T16:34:35.5683226Z 
2019-11-17T16:34:35.5683353Z running 9 tests
2019-11-17T16:34:35.5684219Z iiiiiiiii
2019-11-17T16:34:35.5684820Z 
2019-11-17T16:34:35.5684866Z  finished in 0.154
2019-11-17T16:34:35.5868033Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-17T16:34:35.7766502Z 
---
2019-11-17T16:34:54.4676335Z  finished in 18.880
2019-11-17T16:34:54.4890225Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-17T16:34:54.6800659Z 
2019-11-17T16:34:54.6801434Z running 123 tests
2019-11-17T16:35:17.7043264Z .iiiii...i.....i..i...i..i.i.i..i.ii.Fi.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-11-17T16:35:22.1219424Z i.i.i......iii.i.....ii
2019-11-17T16:35:22.1221051Z 
2019-11-17T16:35:22.1221545Z ---- [debuginfo-gdb+lldb] debuginfo/generator-locals.rs stdout ----
2019-11-17T16:35:22.1221769Z NOTE: compiletest thinks it is using GDB without native rust support
2019-11-17T16:35:22.1221921Z NOTE: compiletest thinks it is using GDB version 7011001
2019-11-17T16:35:22.1221921Z NOTE: compiletest thinks it is using GDB version 7011001
2019-11-17T16:35:22.1222040Z 
2019-11-17T16:35:22.1222176Z error: line not found in debugger output: $1 = 5
2019-11-17T16:35:22.1222638Z status: exit code: 0
2019-11-17T16:35:22.1223189Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/generator-locals/generator-locals.debugger.script"
2019-11-17T16:35:22.1224101Z ------------------------------------------
2019-11-17T16:35:22.1224504Z GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
2019-11-17T16:35:22.1224890Z Copyright (C) 2016 Free Software Foundation, Inc.
2019-11-17T16:35:22.1224890Z Copyright (C) 2016 Free Software Foundation, Inc.
2019-11-17T16:35:22.1225039Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2019-11-17T16:35:22.1225180Z This is free software: you are free to change and redistribute it.
2019-11-17T16:35:22.1225347Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2019-11-17T16:35:22.1225485Z and "show warranty" for details.
2019-11-17T16:35:22.1225886Z This GDB was configured as "x86_64-linux-gnu".
2019-11-17T16:35:22.1226073Z Type "show configuration" for configuration details.
2019-11-17T16:35:22.1226228Z For bug reporting instructions, please see:
2019-11-17T16:35:22.1226380Z <http://www.gnu.org/software/gdb/bugs/>.
2019-11-17T16:35:22.1226522Z Find the GDB manual and other documentation resources online at:
2019-11-17T16:35:22.1226663Z <http://www.gnu.org/software/gdb/documentation/>.
2019-11-17T16:35:22.1226817Z For help, type "help".
2019-11-17T16:35:22.1226966Z Type "apropos word" to search for commands related to "word".
2019-11-17T16:35:22.1227396Z Breakpoint 1 at 0xd48: file /checkout/src/test/debuginfo/generator-locals.rs, line 70.
2019-11-17T16:35:22.1227883Z Breakpoint 2 at 0xcdb: file /checkout/src/test/debuginfo/generator-locals.rs, line 74.
2019-11-17T16:35:22.1228341Z Breakpoint 3 at 0xd42: file /checkout/src/test/debuginfo/generator-locals.rs, line 78.
2019-11-17T16:35:22.1229847Z Breakpoint 4 at 0xc07: file /checkout/src/test/debuginfo/generator-locals.rs, line 84.
2019-11-17T16:35:22.1230095Z [Thread debugging using libthread_db enabled]
2019-11-17T16:35:22.1230536Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2019-11-17T16:35:22.1230719Z 
2019-11-17T16:35:22.1231202Z Breakpoint 2, generator_locals::main::_$u7b$$u7b$closure$u7d$$u7d$::he31ae142d31e1fbd () at /checkout/src/test/debuginfo/generator-locals.rs:74
2019-11-17T16:35:22.1231388Z 74         _zzz(); // #break
2019-11-17T16:35:22.1231543Z $1 = 7
2019-11-17T16:35:22.1231673Z $2 = 6
2019-11-17T16:35:22.1231799Z $3 = 7
2019-11-17T16:35:22.1232095Z 
2019-11-17T16:35:22.1232640Z Breakpoint 4, generator_locals::main::haac872261cb3af39 () at /checkout/src/test/debuginfo/generator-locals.rs:84
2019-11-17T16:35:22.1233664Z 84     _zzz(); // #break
2019-11-17T16:35:22.1233874Z $4 = 6
2019-11-17T16:35:22.1234430Z ------------------------------------------
2019-11-17T16:35:22.1234638Z stderr:
2019-11-17T16:35:22.1235010Z ------------------------------------------
2019-11-17T16:35:22.1235010Z ------------------------------------------
2019-11-17T16:35:22.1235670Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/generator-locals/generator-locals.debugger.script:17: Error in sourced command file:
2019-11-17T16:35:22.1235901Z No symbol "c" in current context.
2019-11-17T16:35:22.1236372Z ------------------------------------------
2019-11-17T16:35:22.1236544Z 
2019-11-17T16:35:22.1236656Z 
2019-11-17T16:35:22.1236764Z 
---
2019-11-17T16:35:22.1238521Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-17T16:35:22.1238710Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-17T16:35:22.1238846Z 
2019-11-17T16:35:22.1238954Z 
2019-11-17T16:35:22.1240654Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo-gdb+lldb" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-17T16:35:22.1241287Z 
2019-11-17T16:35:22.1241399Z 
2019-11-17T16:35:22.1241548Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-17T16:35:22.1241693Z Build completed unsuccessfully in 1:04:06
2019-11-17T16:35:22.1241693Z Build completed unsuccessfully in 1:04:06
2019-11-17T16:35:22.1284084Z == clock drift check ==
2019-11-17T16:35:22.1298952Z   local time: Sun Nov 17 16:35:22 UTC 2019
2019-11-17T16:35:22.1682739Z   network time: Sun, 17 Nov 2019 16:35:22 GMT
2019-11-17T16:35:22.1686017Z == end clock drift check ==
2019-11-17T16:35:23.8637927Z 
2019-11-17T16:35:23.8762890Z ##[error]Bash exited with code '1'.
2019-11-17T16:35:23.8807048Z ##[section]Starting: Checkout
2019-11-17T16:35:23.8809145Z ==============================================================================
2019-11-17T16:35:23.8809199Z Task         : Get sources
2019-11-17T16:35:23.8809261Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
