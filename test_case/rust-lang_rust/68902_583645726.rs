plain
2020-02-07T21:32:11.3510787Z ========================== Starting Command Output ===========================
2020-02-07T21:32:11.3513397Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cf080323-6798-4ea8-a74d-be0e3534768e.sh
2020-02-07T21:32:11.3513436Z 
2020-02-07T21:32:11.3515855Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-07T21:32:11.3520821Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68902/merge to s
2020-02-07T21:32:11.3522314Z Task         : Get sources
2020-02-07T21:32:11.3522344Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-07T21:32:11.3522379Z Version      : 1.0.0
2020-02-07T21:32:11.3522408Z Author       : Microsoft
---
2020-02-07T21:32:12.2106152Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-07T21:32:12.2187108Z ##[command]git config gc.auto 0
2020-02-07T21:32:12.2259765Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-07T21:32:12.2290256Z ##[command]git config --get-all http.proxy
2020-02-07T21:32:12.2459356Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68902/merge:refs/remotes/pull/68902/merge
---
2020-02-07T22:23:26.0822402Z .................................................................................................... 1700/9608
2020-02-07T22:23:30.2201664Z .................................................................................................... 1800/9608
2020-02-07T22:23:40.8974910Z .............................i...................................................................... 1900/9608
2020-02-07T22:23:47.1410045Z .................................................................................................... 2000/9608
2020-02-07T22:23:59.8317961Z ...................iiiii............................................................................ 2100/9608
2020-02-07T22:24:08.1084636Z .................................................................................................... 2300/9608
2020-02-07T22:24:10.2267647Z .................................................................................................... 2400/9608
2020-02-07T22:24:14.2699732Z .................................................................................................... 2500/9608
2020-02-07T22:24:32.5635402Z .................................................................................................... 2600/9608
---
2020-02-07T22:26:49.5764899Z .......................................................................i...............i............ 4900/9608
2020-02-07T22:26:56.3992410Z .................................................................................................... 5000/9608
2020-02-07T22:27:03.3002114Z .................................................................................................... 5100/9608
2020-02-07T22:27:07.2952453Z ..............i..................................................................................... 5200/9608
2020-02-07T22:27:16.7650864Z ........................................................................................ii.ii....... 5300/9608
2020-02-07T22:27:20.1499019Z .i...i.............................................................................................. 5400/9608
2020-02-07T22:27:30.6728437Z .................................................................................................... 5600/9608
2020-02-07T22:27:37.9332279Z ............................................................................i....................... 5700/9608
2020-02-07T22:27:44.3616068Z .................................................................................................... 5800/9608
2020-02-07T22:27:49.8216080Z .................................................................................................... 5900/9608
2020-02-07T22:27:49.8216080Z .................................................................................................... 5900/9608
2020-02-07T22:27:58.4899008Z ....................................................................ii...i..ii...........i.......... 6000/9608
2020-02-07T22:28:17.1854214Z .................................................................................................... 6200/9608
2020-02-07T22:28:23.8488256Z .................................................................................................... 6300/9608
2020-02-07T22:28:30.7521005Z ................................................................................................i..i 6400/9608
2020-02-07T22:28:47.1885053Z i................................................................................................... 6500/9608
---
2020-02-07T22:30:33.5314772Z .................................................................................................... 7600/9608
2020-02-07T22:30:37.7934707Z .................................................................................................... 7700/9608
2020-02-07T22:30:42.0598141Z .................................................................................................... 7800/9608
2020-02-07T22:30:49.7302434Z .................................................................................................... 7900/9608
2020-02-07T22:30:57.2977812Z ......................................................iiiiiii.i..................................... 8000/9608
2020-02-07T22:31:09.8907848Z .i.................................................................................................. 8200/9608
2020-02-07T22:31:14.3469962Z .................................................................................................... 8300/9608
2020-02-07T22:31:27.4060936Z .................................................................................................... 8400/9608
2020-02-07T22:31:34.4614632Z .................................................................................................... 8500/9608
---
2020-02-07T22:33:45.2028584Z  finished in 6.649
2020-02-07T22:33:45.2179621Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-07T22:33:45.3900996Z 
2020-02-07T22:33:45.3901632Z running 175 tests
2020-02-07T22:33:47.9454310Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/175
2020-02-07T22:33:50.1366248Z ...i.i.i...iii..iiiiiiiiiiiii.......................iii............ii......
2020-02-07T22:33:50.1366782Z 
2020-02-07T22:33:50.1374832Z  finished in 4.919
2020-02-07T22:33:50.1557988Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-07T22:33:50.3119989Z 
---
2020-02-07T22:33:52.2207609Z  finished in 2.065
2020-02-07T22:33:52.2373551Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-07T22:33:52.3813424Z 
2020-02-07T22:33:52.3813889Z running 9 tests
2020-02-07T22:33:52.3815386Z iiiiiiiii
2020-02-07T22:33:52.3815682Z 
2020-02-07T22:33:52.3819461Z  finished in 0.144
2020-02-07T22:33:52.4004372Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-07T22:33:52.5821008Z 
---
2020-02-07T22:34:10.8066917Z  finished in 18.406
2020-02-07T22:34:10.8294594Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-07T22:34:11.4899892Z 
2020-02-07T22:34:11.4900012Z running 116 tests
2020-02-07T22:34:23.2656618Z iiiiiFFiFFFF.i..i...i.Fi.iFi..i..i.FiiF..Fi.i..F.ii..........iiii.F..F.....iF....i..i.FFF...ii.i.ii. 100/116
2020-02-07T22:34:24.9372890Z ....iiii...FFiiF
2020-02-07T22:34:24.9373262Z 
2020-02-07T22:34:24.9405573Z ---- [debuginfo-gdb] debuginfo/borrowed-basic.rs stdout ----
2020-02-07T22:34:24.9405573Z ---- [debuginfo-gdb] debuginfo/borrowed-basic.rs stdout ----
2020-02-07T22:34:24.9405636Z NOTE: compiletest thinks it is using GDB with native rust support
2020-02-07T22:34:24.9405721Z NOTE: compiletest thinks it is using GDB version 8001000
2020-02-07T22:34:24.9405794Z error: line not found in debugger output: $1 = true
2020-02-07T22:34:24.9406184Z status: exit code: 0
2020-02-07T22:34:24.9406184Z status: exit code: 0
2020-02-07T22:34:24.9406611Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-basic.gdb/borrowed-basic.debugger.script"
2020-02-07T22:34:24.9406898Z ------------------------------------------
2020-02-07T22:34:24.9406898Z ------------------------------------------
2020-02-07T22:34:24.9407139Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-02-07T22:34:24.9407186Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-02-07T22:34:24.9407252Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-02-07T22:34:24.9407300Z This is free software: you are free to change and redistribute it.
2020-02-07T22:34:24.9407347Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-02-07T22:34:24.9407407Z and "show warranty" for details.
2020-02-07T22:34:24.9407621Z This GDB was configured as "x86_64-linux-gnu".
2020-02-07T22:34:24.9407669Z Type "show configuration" for configuration details.
2020-02-07T22:34:24.9407712Z For bug reporting instructions, please see:
2020-02-07T22:34:24.9407773Z <http://www.gnu.org/software/gdb/bugs/>.
2020-02-07T22:34:24.9407987Z Find the GDB manual and other documentation resources online at:
2020-02-07T22:34:24.9408044Z <http://www.gnu.org/software/gdb/documentation/>.
2020-02-07T22:34:24.9408363Z For help, type "help".
2020-02-07T22:34:24.9408406Z Type "apropos word" to search for commands related to "word".
2020-02-07T22:34:24.9409023Z Breakpoint 1 at 0x9bf: file /checkout/src/test/debuginfo/borrowed-basic.rs, line 162.
2020-02-07T22:34:24.9409083Z [Thread debugging using libthread_db enabled]
2020-02-07T22:34:24.9409275Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-02-07T22:34:24.9409303Z 
2020-02-07T22:34:24.9409520Z Breakpoint 1, borrowed_basic::main () at /checkout/src/test/debuginfo/borrowed-basic.rs:162
2020-02-07T22:34:24.9409559Z 162     zzz(); // #break
2020-02-07T22:34:24.9409742Z ------------------------------------------
2020-02-07T22:34:24.9409794Z stderr:
2020-02-07T22:34:24.9409955Z ------------------------------------------
2020-02-07T22:34:24.9409955Z ------------------------------------------
2020-02-07T22:34:24.9410212Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-basic.gdb/borrowed-basic.debugger.script:10: Error in sourced command file:
2020-02-07T22:34:24.9410303Z 
2020-02-07T22:34:24.9410470Z ------------------------------------------
2020-02-07T22:34:24.9410493Z 
2020-02-07T22:34:24.9410512Z 
2020-02-07T22:34:24.9410512Z 
2020-02-07T22:34:24.9410706Z ---- [debuginfo-gdb] debuginfo/associated-types.rs stdout ----
2020-02-07T22:34:24.9410746Z NOTE: compiletest thinks it is using GDB with native rust support
2020-02-07T22:34:24.9410785Z NOTE: compiletest thinks it is using GDB version 8001000
2020-02-07T22:34:24.9410864Z error: line not found in debugger output: $2 = 1
2020-02-07T22:34:24.9410899Z status: exit code: 0
2020-02-07T22:34:24.9410899Z status: exit code: 0
2020-02-07T22:34:24.9411202Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/associated-types.gdb/associated-types.debugger.script"
2020-02-07T22:34:24.9411426Z ------------------------------------------
2020-02-07T22:34:24.9411426Z ------------------------------------------
2020-02-07T22:34:24.9412157Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-02-07T22:34:24.9412224Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-02-07T22:34:24.9412265Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-02-07T22:34:24.9412310Z This is free software: you are free to change and redistribute it.
2020-02-07T22:34:24.9412368Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-02-07T22:34:24.9412407Z and "show warranty" for details.
2020-02-07T22:34:24.9412603Z This GDB was configured as "x86_64-linux-gnu".
2020-02-07T22:34:24.9412664Z Type "show configuration" for configuration details.
2020-02-07T22:34:24.9412701Z For bug reporting instructions, please see:
2020-02-07T22:34:24.9412738Z <http://www.gnu.org/software/gdb/bugs/>.
2020-02-07T22:34:24.9412801Z Find the GDB manual and other documentation resources online at:
2020-02-07T22:34:24.9412842Z <http://www.gnu.org/software/gdb/documentation/>.
2020-02-07T22:34:24.9412877Z For help, type "help".
2020-02-07T22:34:24.9412931Z Type "apropos word" to search for commands related to "word".
2020-02-07T22:34:24.9413160Z Breakpoint 1 at 0x96d: file /checkout/src/test/debuginfo/associated-types.rs, line 111.
2020-02-07T22:34:24.9413375Z Breakpoint 2 at 0x9e7: file /checkout/src/test/debuginfo/associated-types.rs, line 118.
2020-02-07T22:34:24.9413603Z Breakpoint 3 at 0xa34: file /checkout/src/test/debuginfo/associated-types.rs, line 122.
2020-02-07T22:34:24.9413813Z Breakpoint 4 at 0xaad: file /checkout/src/test/debuginfo/associated-types.rs, line 130.
2020-02-07T22:34:24.9414026Z Breakpoint 5 at 0xb5c: file /checkout/src/test/debuginfo/associated-types.rs, line 137.
2020-02-07T22:34:24.9414448Z Breakpoint 6 at 0xb49: file /checkout/src/test/debuginfo/associated-types.rs, line 140.
2020-02-07T22:34:24.9414491Z [Thread debugging using libthread_db enabled]
2020-02-07T22:34:24.9414809Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-02-07T22:34:24.9414868Z 
2020-02-07T22:34:24.9415127Z Breakpoint 1, associated_types::assoc_struct (arg=...) at /checkout/src/test/debuginfo/associated-types.rs:111
2020-02-07T22:34:24.9415249Z 111     zzz(); // #break
2020-02-07T22:34:24.9415453Z $1 = associated_types::Struct<i32> {b: -1, b1: 0}
2020-02-07T22:34:24.9415501Z 
2020-02-07T22:34:24.9415724Z Breakpoint 2, associated_types::assoc_local (x=1) at /checkout/src/test/debuginfo/associated-types.rs:118
2020-02-07T22:34:24.9415766Z 118     zzz(); // #break
2020-02-07T22:34:24.9415817Z $2 = <optimized out>
2020-02-07T22:34:24.9415849Z $3 = <optimized out>
2020-02-07T22:34:24.9415871Z 
2020-02-07T22:34:24.9416091Z Breakpoint 3, associated_types::assoc_arg (arg=2) at /checkout/src/test/debuginfo/associated-types.rs:122
2020-02-07T22:34:24.9416150Z 122     zzz(); // #break
2020-02-07T22:34:24.9416182Z $4 = 2
2020-02-07T22:34:24.9416203Z 
2020-02-07T22:34:24.9416454Z Breakpoint 4, associated_types::assoc_tuple (arg=...) at /checkout/src/test/debuginfo/associated-types.rs:130
2020-02-07T22:34:24.9416496Z 130     zzz(); // #break
2020-02-07T22:34:24.9416535Z $5 = (4, 5)
2020-02-07T22:34:24.9416558Z 
2020-02-07T22:34:24.9416801Z Breakpoint 5, associated_types::assoc_enum (arg=...) at /checkout/src/test/debuginfo/associated-types.rs:137
2020-02-07T22:34:24.9416844Z 137             zzz(); // #break
2020-02-07T22:34:24.9416878Z $6 = <optimized out>
2020-02-07T22:34:24.9416929Z $7 = <optimized out>
2020-02-07T22:34:24.9416951Z 
2020-02-07T22:34:24.9417172Z Breakpoint 6, associated_types::assoc_enum (arg=...) at /checkout/src/test/debuginfo/associated-types.rs:140
2020-02-07T22:34:24.9417215Z 140             zzz(); // #break
2020-02-07T22:34:24.9417268Z $8 = <optimized out>
2020-02-07T22:34:24.9417300Z $9 = <optimized out>
2020-02-07T22:34:24.9417335Z [Inferior 1 (process 6727) exited normally]
2020-02-07T22:34:24.9417557Z ------------------------------------------
2020-02-07T22:34:24.9417595Z stderr:
2020-02-07T22:34:24.9417766Z ------------------------------------------
2020-02-07T22:34:24.9417808Z 
2020-02-07T22:34:24.9417808Z 
2020-02-07T22:34:24.9417976Z ------------------------------------------
2020-02-07T22:34:24.9418008Z 
2020-02-07T22:34:24.9418028Z 
2020-02-07T22:34:24.9418238Z ---- [debuginfo-gdb] debuginfo/borrowed-c-style-enum.rs stdout ----
2020-02-07T22:34:24.9418283Z NOTE: compiletest thinks it is using GDB with native rust support
2020-02-07T22:34:24.9418326Z NOTE: compiletest thinks it is using GDB version 8001000
2020-02-07T22:34:24.9418352Z 
2020-02-07T22:34:24.9418409Z error: line not found in debugger output: $1 = borrowed_c_style_enum::ABC::TheA
2020-02-07T22:34:24.9418447Z status: exit code: 0
2020-02-07T22:34:24.9418746Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-c-style-enum.gdb/borrowed-c-style-enum.debugger.script"
2020-02-07T22:34:24.9419007Z ------------------------------------------
2020-02-07T22:34:24.9419007Z ------------------------------------------
2020-02-07T22:34:24.9419211Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-02-07T22:34:24.9419273Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-02-07T22:34:24.9419322Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-02-07T22:34:24.9419365Z This is free software: you are free to change and redistribute it.
2020-02-07T22:34:24.9419427Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-02-07T22:34:24.9419467Z and "show warranty" for details.
2020-02-07T22:34:24.9419664Z This GDB was configured as "x86_64-linux-gnu".
2020-02-07T22:34:24.9419725Z Type "show configuration" for configuration details.
2020-02-07T22:34:24.9419766Z For bug reporting instructions, please see:
2020-02-07T22:34:24.9419804Z <http://www.gnu.org/software/gdb/bugs/>.
2020-02-07T22:34:24.9419862Z Find the GDB manual and other documentation resources online at:
2020-02-07T22:34:24.9419977Z <http://www.gnu.org/software/gdb/documentation/>.
2020-02-07T22:34:24.9420022Z For help, type "help".
2020-02-07T22:34:24.9420062Z Type "apropos word" to search for commands related to "word".
2020-02-07T22:34:24.9420570Z Breakpoint 1 at 0x96f: file /checkout/src/test/debuginfo/borrowed-c-style-enum.rs, line 53.
2020-02-07T22:34:24.9420617Z [Thread debugging using libthread_db enabled]
2020-02-07T22:34:24.9420825Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-02-07T22:34:24.9420872Z 
2020-02-07T22:34:24.9421276Z Breakpoint 1, borrowed_c_style_enum::main () at /checkout/src/test/debuginfo/borrowed-c-style-enum.rs:53
2020-02-07T22:34:24.9421321Z 53     zzz(); // #break
2020-02-07T22:34:24.9421542Z ------------------------------------------
2020-02-07T22:34:24.9421580Z stderr:
2020-02-07T22:34:24.9421757Z ------------------------------------------
2020-02-07T22:34:24.9421757Z ------------------------------------------
2020-02-07T22:34:24.9422072Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-c-style-enum.gdb/borrowed-c-style-enum.debugger.script:10: Error in sourced command file:
2020-02-07T22:34:24.9422145Z 
2020-02-07T22:34:24.9422519Z ------------------------------------------
2020-02-07T22:34:24.9422556Z 
2020-02-07T22:34:24.9422577Z 
2020-02-07T22:34:24.9422577Z 
2020-02-07T22:34:24.9422783Z ---- [debuginfo-gdb] debuginfo/borrowed-struct.rs stdout ----
2020-02-07T22:34:24.9422848Z NOTE: compiletest thinks it is using GDB with native rust support
2020-02-07T22:34:24.9422895Z NOTE: compiletest thinks it is using GDB version 8001000
2020-02-07T22:34:24.9422962Z error: line not found in debugger output: $3 = 23.5
2020-02-07T22:34:24.9423021Z status: exit code: 0
2020-02-07T22:34:24.9423021Z status: exit code: 0
2020-02-07T22:34:24.9423338Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-struct.gdb/borrowed-struct.debugger.script"
2020-02-07T22:34:24.9423617Z ------------------------------------------
2020-02-07T22:34:24.9423617Z ------------------------------------------
2020-02-07T22:34:24.9423839Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-02-07T22:34:24.9423886Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-02-07T22:34:24.9424114Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-02-07T22:34:24.9424167Z This is free software: you are free to change and redistribute it.
2020-02-07T22:34:24.9424211Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-02-07T22:34:24.9424269Z and "show warranty" for details.
2020-02-07T22:34:24.9424466Z This GDB was configured as "x86_64-linux-gnu".
2020-02-07T22:34:24.9424509Z Type "show configuration" for configuration details.
2020-02-07T22:34:24.9424567Z For bug reporting instructions, please see:
2020-02-07T22:34:24.9424782Z <http://www.gnu.org/software/gdb/bugs/>.
2020-02-07T22:34:24.9424824Z Find the GDB manual and other documentation resources online at:
2020-02-07T22:34:24.9424867Z <http://www.gnu.org/software/gdb/documentation/>.
2020-02-07T22:34:24.9424931Z For help, type "help".
2020-02-07T22:34:24.9424973Z Type "apropos word" to search for commands related to "word".
2020-02-07T22:34:24.9425374Z Breakpoint 1 at 0xbfb: file /checkout/src/test/debuginfo/borrowed-struct.rs, line 87.
2020-02-07T22:34:24.9425445Z [Thread debugging using libthread_db enabled]
2020-02-07T22:34:24.9425659Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-02-07T22:34:24.9425688Z 
2020-02-07T22:34:24.9426087Z Breakpoint 1, borrowed_struct::main () at /checkout/src/test/debuginfo/borrowed-struct.rs:87
2020-02-07T22:34:24.9426315Z 87     zzz(); // #break
2020-02-07T22:34:24.9426353Z $1 = borrowed_struct::SomeStruct {x: 10, y: 23.5}
2020-02-07T22:34:24.9426407Z $2 = 10
2020-02-07T22:34:24.9426609Z ------------------------------------------
2020-02-07T22:34:24.9426647Z stderr:
2020-02-07T22:34:24.9426844Z ------------------------------------------
2020-02-07T22:34:24.9426844Z ------------------------------------------
2020-02-07T22:34:24.9427247Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-struct.gdb/borrowed-struct.debugger.script:12: Error in sourced command file:
2020-02-07T22:34:24.9427502Z 
2020-02-07T22:34:24.9427798Z ------------------------------------------
2020-02-07T22:34:24.9427826Z 
2020-02-07T22:34:24.9427848Z 
2020-02-07T22:34:24.9427848Z 
2020-02-07T22:34:24.9428055Z ---- [debuginfo-gdb] debuginfo/borrowed-unique-basic.rs stdout ----
2020-02-07T22:34:24.9428123Z NOTE: compiletest thinks it is using GDB with native rust support
2020-02-07T22:34:24.9428168Z NOTE: compiletest thinks it is using GDB version 8001000
2020-02-07T22:34:24.9428255Z error: line not found in debugger output: $1 = true
2020-02-07T22:34:24.9428297Z status: exit code: 0
2020-02-07T22:34:24.9428297Z status: exit code: 0
2020-02-07T22:34:24.9428623Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-unique-basic.gdb/borrowed-unique-basic.debugger.script"
2020-02-07T22:34:24.9428899Z ------------------------------------------
2020-02-07T22:34:24.9428899Z ------------------------------------------
2020-02-07T22:34:24.9429122Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-02-07T22:34:24.9429169Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-02-07T22:34:24.9429243Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-02-07T22:34:24.9429289Z This is free software: you are free to change and redistribute it.
2020-02-07T22:34:24.9429338Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-02-07T22:34:24.9429398Z and "show warranty" for details.
2020-02-07T22:34:24.9429610Z This GDB was configured as "x86_64-linux-gnu".
2020-02-07T22:34:24.9429657Z Type "show configuration" for configuration details.
2020-02-07T22:34:24.9429719Z For bug reporting instructions, please see:
2020-02-07T22:34:24.9429761Z <http://www.gnu.org/software/gdb/bugs/>.
2020-02-07T22:34:24.9429805Z Find the GDB manual and other documentation resources online at:
2020-02-07T22:34:24.9429874Z <http://www.gnu.org/software/gdb/documentation/>.
2020-02-07T22:34:24.9429916Z For help, type "help".
2020-02-07T22:34:24.9429958Z Type "apropos word" to search for commands related to "word".
2020-02-07T22:34:24.9430239Z Breakpoint 1 at 0xe3c: file /checkout/src/test/debuginfo/borrowed-unique-basic.rs, line 166.
2020-02-07T22:34:24.9430289Z [Thread debugging using libthread_db enabled]
2020-02-07T22:34:24.9431141Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-02-07T22:34:24.9431184Z 
2020-02-07T22:34:24.9431531Z Breakpoint 1, borrowed_unique_basic::main () at /checkout/src/test/debuginfo/borrowed-unique-basic.rs:166
2020-02-07T22:34:24.9431587Z 166     zzz(); // #break
2020-02-07T22:34:24.9431858Z ------------------------------------------
2020-02-07T22:34:24.9431904Z stderr:
2020-02-07T22:34:24.9432125Z ------------------------------------------
2020-02-07T22:34:24.9432125Z ------------------------------------------
2020-02-07T22:34:24.9432501Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-unique-basic.gdb/borrowed-unique-basic.debugger.script:10: Error in sourced command file:
2020-02-07T22:34:24.9432591Z 
2020-02-07T22:34:24.9432816Z ------------------------------------------
2020-02-07T22:34:24.9432878Z 
2020-02-07T22:34:24.9432904Z 
2020-02-07T22:34:24.9432904Z 
2020-02-07T22:34:24.9433147Z ---- [debuginfo-gdb] debuginfo/borrowed-tuple.rs stdout ----
2020-02-07T22:34:24.9433203Z NOTE: compiletest thinks it is using GDB with native rust support
2020-02-07T22:34:24.9433277Z NOTE: compiletest thinks it is using GDB version 8001000
2020-02-07T22:34:24.9433311Z 
2020-02-07T22:34:24.9433568Z error: line not found in debugger output: $1 = (-14, -19)
2020-02-07T22:34:24.9433620Z status: exit code: 0
2020-02-07T22:34:24.9434014Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-tuple.gdb/borrowed-tuple.debugger.script"
2020-02-07T22:34:24.9434814Z ------------------------------------------
2020-02-07T22:34:24.9434814Z ------------------------------------------
2020-02-07T22:34:24.9435256Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-02-07T22:34:24.9435302Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-02-07T22:34:24.9435420Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-02-07T22:34:24.9435486Z This is free software: you are free to change and redistribute it.
2020-02-07T22:34:24.9435531Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-02-07T22:34:24.9435573Z and "show warranty" for details.
2020-02-07T22:34:24.9435815Z This GDB was configured as "x86_64-linux-gnu".
2020-02-07T22:34:24.9435860Z Type "show configuration" for configuration details.
2020-02-07T22:34:24.9435901Z For bug reporting instructions, please see:
2020-02-07T22:34:24.9435960Z <http://www.gnu.org/software/gdb/bugs/>.
2020-02-07T22:34:24.9436004Z Find the GDB manual and other documentation resources online at:
2020-02-07T22:34:24.9436046Z <http://www.gnu.org/software/gdb/documentation/>.
2020-02-07T22:34:24.9436191Z For help, type "help".
2020-02-07T22:34:24.9436233Z Type "apropos word" to search for commands related to "word".
2020-02-07T22:34:24.9436472Z Breakpoint 1 at 0xbcc: file /checkout/src/test/debuginfo/borrowed-tuple.rs, line 52.
2020-02-07T22:34:24.9436545Z [Thread debugging using libthread_db enabled]
2020-02-07T22:34:24.9436766Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-02-07T22:34:24.9436796Z 
2020-02-07T22:34:24.9437022Z Breakpoint 1, borrowed_tuple::main () at /checkout/src/test/debuginfo/borrowed-tuple.rs:52
2020-02-07T22:34:24.9437084Z 52     zzz(); // #break
2020-02-07T22:34:24.9437341Z ------------------------------------------
2020-02-07T22:34:24.9437397Z stderr:
2020-02-07T22:34:24.9437582Z ------------------------------------------
2020-02-07T22:34:24.9437582Z ------------------------------------------
2020-02-07T22:34:24.9438022Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-tuple.gdb/borrowed-tuple.debugger.script:10: Error in sourced command file:
2020-02-07T22:34:24.9438094Z Cannot access memory at address 0xc1980000f7b4fff2
2020-02-07T22:34:24.9438461Z ------------------------------------------
2020-02-07T22:34:24.9438493Z 
2020-02-07T22:34:24.9438513Z 
2020-02-07T22:34:24.9438725Z ---- [debuginfo-gdb] debuginfo/destructured-fn-argument.rs stdout ----
2020-02-07T22:34:24.9438725Z ---- [debuginfo-gdb] debuginfo/destructured-fn-argument.rs stdout ----
2020-02-07T22:34:24.9438768Z NOTE: compiletest thinks it is using GDB with native rust support
2020-02-07T22:34:24.9438807Z NOTE: compiletest thinks it is using GDB version 8001000
2020-02-07T22:34:24.9438881Z error: line not found in debugger output: $4 = 3
2020-02-07T22:34:24.9438916Z status: exit code: 0
2020-02-07T22:34:24.9438916Z status: exit code: 0
2020-02-07T22:34:24.9439409Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/destructured-fn-argument.gdb/destructured-fn-argument.debugger.script"
2020-02-07T22:34:24.9439628Z ------------------------------------------
2020-02-07T22:34:24.9439628Z ------------------------------------------
2020-02-07T22:34:24.9439823Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-02-07T22:34:24.9440062Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-02-07T22:34:24.9440105Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-02-07T22:34:24.9440153Z This is free software: you are free to change and redistribute it.
2020-02-07T22:34:24.9440213Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-02-07T22:34:24.9440252Z and "show warranty" for details.
2020-02-07T22:34:24.9440446Z This GDB was configured as "x86_64-linux-gnu".
2020-02-07T22:34:24.9440506Z Type "show configuration" for configuration details.
2020-02-07T22:34:24.9440545Z For bug reporting instructions, please see:
2020-02-07T22:34:24.9440583Z <http://www.gnu.org/software/gdb/bugs/>.
2020-02-07T22:34:24.9440659Z Find the GDB manual and other documentation resources online at:
2020-02-07T22:34:24.9440700Z <http://www.gnu.org/software/gdb/documentation/>.
2020-02-07T22:34:24.9440811Z For help, type "help".
2020-02-07T22:34:24.9440873Z Type "apropos word" to search for commands related to "word".
2020-02-07T22:34:24.9441131Z Breakpoint 1 at 0x9db: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 380.
2020-02-07T22:34:24.9441458Z Breakpoint 2 at 0x9fc: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 384.
2020-02-07T22:34:24.9441709Z Breakpoint 3 at 0xa21: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 388.
2020-02-07T22:34:24.9441940Z Breakpoint 4 at 0xa4b: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 392.
2020-02-07T22:34:24.9442171Z Breakpoint 5 at 0xa76: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 396.
2020-02-07T22:34:24.9442427Z Breakpoint 6 at 0xa87: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 400.
2020-02-07T22:34:24.9442658Z Breakpoint 7 at 0xaa1: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 404.
2020-02-07T22:34:24.9443060Z Breakpoint 8 at 0xaca: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 408.
2020-02-07T22:34:24.9443310Z Breakpoint 9 at 0xaf6: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 412.
2020-02-07T22:34:24.9443547Z Breakpoint 10 at 0xb20: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 418.
2020-02-07T22:34:24.9443773Z Breakpoint 11 at 0xb48: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 422.
2020-02-07T22:34:24.9444022Z Breakpoint 12 at 0xb78: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 426.
2020-02-07T22:34:24.9444246Z Breakpoint 13 at 0xba6: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 430.
2020-02-07T22:34:24.9444471Z Breakpoint 14 at 0xbcf: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 434.
2020-02-07T22:34:24.9444719Z Breakpoint 15 at 0xc11: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 438.
2020-02-07T22:34:24.9444952Z Breakpoint 16 at 0xc39: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 442.
2020-02-07T22:34:24.9445197Z Breakpoint 17 at 0xc66: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 446.
2020-02-07T22:34:24.9445436Z Breakpoint 18 at 0xc79: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 450.
2020-02-07T22:34:24.9445660Z Breakpoint 19 at 0xc8a: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 454.
2020-02-07T22:34:24.9445902Z Breakpoint 20 at 0xcba: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 458.
2020-02-07T22:34:24.9446127Z Breakpoint 21 at 0xcee: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 462.
2020-02-07T22:34:24.9446353Z Breakpoint 22 at 0xd18: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 466.
2020-02-07T22:34:24.9446597Z Breakpoint 23 at 0x12b8: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 494.
2020-02-07T22:34:24.9446643Z [Thread debugging using libthread_db enabled]
2020-02-07T22:34:24.9446857Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-02-07T22:34:24.9446903Z 
2020-02-07T22:34:24.9447331Z Breakpoint 1, destructured_fn_argument::simple_tuple () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:380
2020-02-07T22:34:24.9447384Z 380     zzz(); // #break
2020-02-07T22:34:24.9447419Z $1 = 1
2020-02-07T22:34:24.9447471Z $2 = false
2020-02-07T22:34:24.9447493Z 
2020-02-07T22:34:24.9447900Z Breakpoint 2, destructured_fn_argument::nested_tuple () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:384
2020-02-07T22:34:24.9447961Z 384     zzz(); // #break
2020-02-07T22:34:24.9447995Z $3 = 2
2020-02-07T22:34:24.9448028Z $4 = <optimized out>
2020-02-07T22:34:24.9448061Z $5 = 4
2020-02-07T22:34:24.9448098Z 
2020-02-07T22:34:24.9448352Z Breakpoint 3, destructured_fn_argument::destructure_only_first_level () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:388
2020-02-07T22:34:24.9448397Z 388     zzz(); // #break
2020-02-07T22:34:24.9448512Z $6 = 5
2020-02-07T22:34:24.9448551Z $7 = (6, 7)
2020-02-07T22:34:24.9448572Z 
2020-02-07T22:34:24.9448847Z Breakpoint 4, destructured_fn_argument::struct_as_tuple_element () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:392
2020-02-07T22:34:24.9448995Z 392     zzz(); // #break
2020-02-07T22:34:24.9449028Z $8 = <optimized out>
2020-02-07T22:34:24.9449065Z $9 = destructured_fn_argument::Struct {a: 9, b: 10}
2020-02-07T22:34:24.9449117Z $10 = 11
2020-02-07T22:34:24.9449138Z 
2020-02-07T22:34:24.9449406Z Breakpoint 5, destructured_fn_argument::struct_pattern () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:396
2020-02-07T22:34:24.9449449Z 396     zzz(); // #break
2020-02-07T22:34:24.9449500Z $11 = 12
2020-02-07T22:34:24.9449532Z $12 = 13
2020-02-07T22:34:24.9449553Z 
2020-02-07T22:34:24.9449814Z Breakpoint 6, destructured_fn_argument::ignored_tuple_element () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:400
2020-02-07T22:34:24.9449866Z 400     zzz(); // #break
2020-02-07T22:34:24.9449899Z $13 = 14
2020-02-07T22:34:24.9449930Z $14 = 16
2020-02-07T22:34:24.9449967Z 
2020-02-07T22:34:24.9450213Z Breakpoint 7, destructured_fn_argument::ignored_struct_field () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:404
2020-02-07T22:34:24.9450265Z 404     zzz(); // #break
2020-02-07T22:34:24.9450314Z $15 = 18
2020-02-07T22:34:24.9450335Z 
2020-02-07T22:34:24.9450752Z Breakpoint 8, destructured_fn_argument::one_struct_destructured_one_not () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:408
2020-02-07T22:34:24.9450796Z 408     zzz(); // #break
2020-02-07T22:34:24.9450844Z $16 = 19
2020-02-07T22:34:24.9450874Z $17 = 20
2020-02-07T22:34:24.9450909Z $18 = destructured_fn_argument::Struct {a: 21, b: 22}
2020-02-07T22:34:24.9450932Z 
2020-02-07T22:34:24.9451197Z Breakpoint 9, destructured_fn_argument::different_order_of_struct_fields () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:412
2020-02-07T22:34:24.9451241Z 412     zzz(); // #break
2020-02-07T22:34:24.9451280Z $19 = 24
2020-02-07T22:34:24.9451328Z $20 = 23
2020-02-07T22:34:24.9451348Z 
2020-02-07T22:34:24.9458518Z Breakpoint 10, destructured_fn_argument::complex_nesting () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:418
2020-02-07T22:34:24.9458628Z 418     zzz(); // #break
2020-02-07T22:34:24.9458667Z $21 = 25
2020-02-07T22:34:24.9458703Z $22 = <optimized out>
2020-02-07T22:34:24.9458739Z $23 = <optimized out>
2020-02-07T22:34:24.9458792Z $24 = <optimized out>
2020-02-07T22:34:24.9458828Z $25 = 29
2020-02-07T22:34:24.9458863Z $26 = 30
2020-02-07T22:34:24.9458914Z $27 = <optimized out>
2020-02-07T22:34:24.9458950Z $28 = <optimized out>
2020-02-07T22:34:24.9458986Z $29 = <optimized out>
2020-02-07T22:34:24.9459010Z 
2020-02-07T22:34:24.9459350Z Breakpoint 11, destructured_fn_argument::managed_box () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:422
2020-02-07T22:34:24.9459397Z 422     zzz(); // #break
2020-02-07T22:34:24.9459434Z $30 = (34, 35)
2020-02-07T22:34:24.9459486Z 
2020-02-07T22:34:24.9459753Z Breakpoint 12, destructured_fn_argument::borrowed_pointer () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:426
2020-02-07T22:34:24.9459809Z 426     zzz(); // #break
2020-02-07T22:34:24.9459845Z $31 = (36, 37)
2020-02-07T22:34:24.9459886Z 
2020-02-07T22:34:24.9460160Z Breakpoint 13, destructured_fn_argument::contained_borrowed_pointer () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:430
2020-02-07T22:34:24.9460208Z 430     zzz(); // #break
2020-02-07T22:34:24.9460262Z $32 = 38
2020-02-07T22:34:24.9460285Z 
2020-02-07T22:34:24.9460538Z Breakpoint 14, destructured_fn_argument::unique_pointer () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:434
2020-02-07T22:34:24.9460583Z 434     zzz(); // #break
2020-02-07T22:34:24.9460636Z $33 = (40, 41, 42)
2020-02-07T22:34:24.9460658Z 
2020-02-07T22:34:24.9461078Z Breakpoint 15, destructured_fn_argument::ref_binding () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:438
2020-02-07T22:34:24.9461291Z 438     zzz(); // #break
2020-02-07T22:34:24.9461337Z $34 = (43, 44, 45)
2020-02-07T22:34:24.9461358Z 
2020-02-07T22:34:24.9461644Z Breakpoint 16, destructured_fn_argument::ref_binding_in_tuple () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:442
2020-02-07T22:34:24.9461793Z 442     zzz(); // #break
2020-02-07T22:34:24.9462014Z ------------------------------------------
2020-02-07T22:34:24.9462052Z stderr:
2020-02-07T22:34:24.9462243Z ------------------------------------------
2020-02-07T22:34:24.9462243Z ------------------------------------------
2020-02-07T22:34:24.9462513Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/destructured-fn-argument.gdb/destructured-fn-argument.debugger.script:81: Error in sourced command file:
2020-02-07T22:34:24.9462601Z 
2020-02-07T22:34:24.9462772Z ------------------------------------------
2020-02-07T22:34:24.9462796Z 
2020-02-07T22:34:24.9462816Z 
2020-02-07T22:34:24.9462816Z 
2020-02-07T22:34:24.9463029Z ---- [debuginfo-gdb] debuginfo/destructured-local.rs stdout ----
2020-02-07T22:34:24.9463075Z NOTE: compiletest thinks it is using GDB with native rust support
2020-02-07T22:34:24.9463115Z NOTE: compiletest thinks it is using GDB version 8001000
2020-02-07T22:34:24.9463196Z error: line not found in debugger output: $34 = (43, 44, 45)
2020-02-07T22:34:24.9463232Z status: exit code: 0
2020-02-07T22:34:24.9463232Z status: exit code: 0
2020-02-07T22:34:24.9463517Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/destructured-local.gdb/destructured-local.debugger.script"
2020-02-07T22:34:24.9463748Z ------------------------------------------
2020-02-07T22:34:24.9463748Z ------------------------------------------
2020-02-07T22:34:24.9463930Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-02-07T22:34:24.9463988Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-02-07T22:34:24.9464028Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-02-07T22:34:24.9464076Z This is free software: you are free to change and redistribute it.
2020-02-07T22:34:24.9464133Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-02-07T22:34:24.9464171Z and "show warranty" for details.
2020-02-07T22:34:24.9464360Z This GDB was configured as "x86_64-linux-gnu".
2020-02-07T22:34:24.9464419Z Type "show configuration" for configuration details.
2020-02-07T22:34:24.9464457Z For bug reporting instructions, please see:
2020-02-07T22:34:24.9464494Z <http://www.gnu.org/software/gdb/bugs/>.
2020-02-07T22:34:24.9464549Z Find the GDB manual and other documentation resources online at:
2020-02-07T22:34:24.9464588Z <http://www.gnu.org/software/gdb/documentation/>.
2020-02-07T22:34:24.9464623Z For help, type "help".
2020-02-07T22:34:24.9464678Z Type "apropos word" to search for commands related to "word".
2020-02-07T22:34:24.9464913Z Breakpoint 1 at 0x1162: file /checkout/src/test/debuginfo/destructured-local.rs, line 371.
2020-02-07T22:34:24.9464960Z [Thread debugging using libthread_db enabled]
2020-02-07T22:34:24.9465179Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-02-07T22:34:24.9465227Z 
2020-02-07T22:34:24.9465462Z Breakpoint 1, destructured_local::main () at /checkout/src/test/debuginfo/destructured-local.rs:371
2020-02-07T22:34:24.9465514Z 371     zzz(); // #break
2020-02-07T22:34:24.9465565Z $1 = 1
2020-02-07T22:34:24.9465599Z $2 = false
2020-02-07T22:34:24.9465631Z $3 = 2
2020-02-07T22:34:24.9465663Z $4 = 3
2020-02-07T22:34:24.9465714Z $5 = 4
2020-02-07T22:34:24.9465745Z $6 = 5
2020-02-07T22:34:24.9465779Z $7 = (6, 7)
2020-02-07T22:34:24.9465828Z $8 = 8
2020-02-07T22:34:24.9465865Z $9 = destructured_local::Struct {a: 9, b: 10}
2020-02-07T22:34:24.9465901Z $10 = 11
2020-02-07T22:34:24.9465950Z $11 = 12
2020-02-07T22:34:24.9465983Z $12 = 13
2020-02-07T22:34:24.9466016Z $13 = 14
2020-02-07T22:34:24.9466048Z $14 = 16
2020-02-07T22:34:24.9466096Z $15 = 18
2020-02-07T22:34:24.9466128Z $16 = 19
2020-02-07T22:34:24.9466161Z $17 = 20
2020-02-07T22:34:24.9466289Z $18 = destructured_local::Struct {a: 21, b: 22}
2020-02-07T22:34:24.9466332Z $19 = 24
2020-02-07T22:34:24.9466364Z $20 = 23
2020-02-07T22:34:24.9466395Z $21 = 25
2020-02-07T22:34:24.9466445Z $22 = 26
2020-02-07T22:34:24.9466526Z $23 = 27
2020-02-07T22:34:24.9466558Z $24 = 28
2020-02-07T22:34:24.9466608Z $25 = 29
2020-02-07T22:34:24.9466640Z $26 = 30
2020-02-07T22:34:24.9466673Z $27 = 31
2020-02-07T22:34:24.9466704Z $28 = 32
2020-02-07T22:34:24.9466753Z $29 = 33
2020-02-07T22:34:24.9466786Z $30 = (34, 35)
2020-02-07T22:34:24.9466819Z $31 = (36, 37)
2020-02-07T22:34:24.9466866Z $32 = 38
2020-02-07T22:34:24.9466900Z $33 = (40, 41, 42)
2020-02-07T22:34:24.9467147Z ------------------------------------------
2020-02-07T22:34:24.9467203Z stderr:
2020-02-07T22:34:24.9467387Z ------------------------------------------
2020-02-07T22:34:24.9467387Z ------------------------------------------
2020-02-07T22:34:24.9467662Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/destructured-local.gdb/destructured-local.debugger.script:43: Error in sourced command file:
2020-02-07T22:34:24.9467758Z 
2020-02-07T22:34:24.9468132Z ------------------------------------------
2020-02-07T22:34:24.9468158Z 
2020-02-07T22:34:24.9468186Z 
2020-02-07T22:34:24.9468186Z 
2020-02-07T22:34:24.9468409Z ---- [debuginfo-gdb] debuginfo/gdb-pretty-struct-and-enums.rs stdout ----
2020-02-07T22:34:24.9468455Z NOTE: compiletest thinks it is using GDB with native rust support
2020-02-07T22:34:24.9468499Z NOTE: compiletest thinks it is using GDB version 8001000
2020-02-07T22:34:24.9468542Z 
2020-02-07T22:34:24.9468583Z error: line not found in debugger output: $2 = gdb_pretty_struct_and_enums::EmptyStruct
2020-02-07T22:34:24.9468623Z status: exit code: 0
2020-02-07T22:34:24.9468958Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/gdb-pretty-struct-and-enums.gdb/gdb-pretty-struct-and-enums.debugger.script"
2020-02-07T22:34:24.9469203Z ------------------------------------------
2020-02-07T22:34:24.9469203Z ------------------------------------------
2020-02-07T22:34:24.9469426Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-02-07T22:34:24.9469470Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-02-07T22:34:24.9469522Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-02-07T22:34:24.9469582Z This is free software: you are free to change and redistribute it.
2020-02-07T22:34:24.9469627Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-02-07T22:34:24.9469669Z and "show warranty" for details.
2020-02-07T22:34:24.9469873Z This GDB was configured as "x86_64-linux-gnu".
---
2020-02-07T22:34:24.9569583Z test result: FAILED. 57 passed; 21 failed; 38 ignored; 0 measured; 0 filtered out
2020-02-07T22:34:24.9569619Z 
2020-02-07T22:34:24.9569640Z 
2020-02-07T22:34:24.9569660Z 
2020-02-07T22:34:24.9570994Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-07T22:34:24.9571244Z 
2020-02-07T22:34:24.9571285Z 
2020-02-07T22:34:24.9571321Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-07T22:34:24.9571359Z Build completed unsuccessfully in 0:56:38
2020-02-07T22:34:24.9571359Z Build completed unsuccessfully in 0:56:38
2020-02-07T22:34:24.9571412Z == clock drift check ==
2020-02-07T22:34:24.9571938Z   local time: thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-07T22:34:24.9571993Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-07T22:34:24.9572059Z Fri Feb  7 22:34:24 UTC 2020
2020-02-07T22:34:25.9885701Z   network time: Fri, 07 Feb 2020 22:34:25 GMT
2020-02-07T22:34:25.9886980Z == end clock drift check ==
2020-02-07T22:34:26.8803356Z 
2020-02-07T22:34:26.8912816Z ##[error]Bash exited with code '1'.
2020-02-07T22:34:26.8923283Z ##[section]Finishing: Run build
2020-02-07T22:34:26.8946836Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68902/merge to s
2020-02-07T22:34:26.8948492Z Task         : Get sources
2020-02-07T22:34:26.8948532Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-07T22:34:26.8948571Z Version      : 1.0.0
2020-02-07T22:34:26.8948627Z Author       : Microsoft
2020-02-07T22:34:26.8948627Z Author       : Microsoft
2020-02-07T22:34:26.8948665Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-07T22:34:26.8948706Z ==============================================================================
2020-02-07T22:34:27.2630377Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-07T22:34:27.2669544Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68902/merge to s
2020-02-07T22:34:27.2766982Z Cleaning up task key
2020-02-07T22:34:27.2767604Z Start cleaning up orphan processes.
2020-02-07T22:34:27.2857545Z Terminate orphan process: pid (3687) (python)
2020-02-07T22:34:27.3063059Z ##[section]Finishing: Finalize Job
