plain
2020-02-06T19:29:19.8821963Z ========================== Starting Command Output ===========================
2020-02-06T19:29:19.8823643Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e50a53b9-0af2-402b-878d-7920f582e4ae.sh
2020-02-06T19:29:19.8823685Z 
2020-02-06T19:29:19.8826457Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-06T19:29:19.8833686Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68902/merge to s
2020-02-06T19:29:19.8835740Z Task         : Get sources
2020-02-06T19:29:19.8835777Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-06T19:29:19.8835814Z Version      : 1.0.0
2020-02-06T19:29:19.8835902Z Author       : Microsoft
---
2020-02-06T19:29:20.9352699Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-06T19:29:20.9526430Z ##[command]git config gc.auto 0
2020-02-06T19:29:20.9567788Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-06T19:29:20.9624645Z ##[command]git config --get-all http.proxy
2020-02-06T19:29:20.9778703Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68902/merge:refs/remotes/pull/68902/merge
---
2020-02-06T20:30:47.7686999Z .................................................................................................... 1700/9596
2020-02-06T20:30:53.0167590Z .................................................................................................... 1800/9596
2020-02-06T20:31:06.0530218Z .............................i...................................................................... 1900/9596
2020-02-06T20:31:13.6498949Z .................................................................................................... 2000/9596
2020-02-06T20:31:28.5757407Z ...................iiiii............................................................................ 2100/9596
2020-02-06T20:31:39.3654314Z .................................................................................................... 2300/9596
2020-02-06T20:31:42.1936528Z .................................................................................................... 2400/9596
2020-02-06T20:31:47.5725905Z .................................................................................................... 2500/9596
2020-02-06T20:32:10.0774002Z .................................................................................................... 2600/9596
---
2020-02-06T20:35:00.6097831Z ..............................................................i...............i..................... 4900/9596
2020-02-06T20:35:08.8113224Z .................................................................................................... 5000/9596
2020-02-06T20:35:17.5629611Z .................................................................................................... 5100/9596
2020-02-06T20:35:22.6187393Z .....i.............................................................................................. 5200/9596
2020-02-06T20:35:34.3811120Z ...............................................................................ii.ii........i...i... 5300/9596
2020-02-06T20:35:43.5405487Z .................i.................................................................................. 5500/9596
2020-02-06T20:35:53.1178314Z .................................................................................................... 5600/9596
2020-02-06T20:36:00.3139731Z ..................................................................i................................. 5700/9596
2020-02-06T20:36:07.9713817Z .................................................................................................... 5800/9596
2020-02-06T20:36:07.9713817Z .................................................................................................... 5800/9596
2020-02-06T20:36:15.4092735Z .................................................................................................... 5900/9596
2020-02-06T20:36:25.4286551Z ..........................................................ii...i..ii...........i.................... 6000/9596
2020-02-06T20:36:47.2782989Z .................................................................................................... 6200/9596
2020-02-06T20:36:55.2127322Z .................................................................................................... 6300/9596
2020-02-06T20:36:55.2127322Z .................................................................................................... 6300/9596
2020-02-06T20:37:01.9111650Z ......................................................................................i..ii......... 6400/9596
2020-02-06T20:37:25.6925880Z .................................................................................................... 6600/9596
2020-02-06T20:37:35.3069777Z .........................................................................i.......................... 6700/9596
2020-02-06T20:37:37.6145222Z .................................................................................................... 6800/9596
2020-02-06T20:37:39.8601961Z ................................................................................i................... 6900/9596
---
2020-02-06T20:39:23.3622640Z .................................................................................................... 7600/9596
2020-02-06T20:39:28.4319767Z .................................................................................................... 7700/9596
2020-02-06T20:39:35.9282130Z .................................................................................................... 7800/9596
2020-02-06T20:39:44.3655314Z .................................................................................................... 7900/9596
2020-02-06T20:39:52.6280673Z ...........................................iiiiiii.i................................................ 8000/9596
2020-02-06T20:40:07.8867354Z .................................................................................................... 8200/9596
2020-02-06T20:40:14.1790776Z .................................................................................................... 8300/9596
2020-02-06T20:40:30.2222115Z .................................................................................................... 8400/9596
2020-02-06T20:40:37.5889741Z .................................................................................................... 8500/9596
---
2020-02-06T20:43:03.7830616Z  finished in 7.304
2020-02-06T20:43:03.8051395Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-06T20:43:04.0274788Z 
2020-02-06T20:43:04.0276736Z running 172 tests
2020-02-06T20:43:07.0469502Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/172
2020-02-06T20:43:09.5779585Z ...i.i.i...iii..iiiiiiiiii.......................iii............ii......
2020-02-06T20:43:09.5783653Z 
2020-02-06T20:43:09.5785571Z  finished in 5.773
2020-02-06T20:43:09.5993579Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-06T20:43:09.7786468Z 
---
2020-02-06T20:43:11.7675117Z  finished in 2.168
2020-02-06T20:43:11.7900152Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-06T20:43:11.9505843Z 
2020-02-06T20:43:11.9506125Z running 9 tests
2020-02-06T20:43:11.9506982Z iiiiiiiii
2020-02-06T20:43:11.9507335Z 
2020-02-06T20:43:11.9507381Z  finished in 0.160
2020-02-06T20:43:11.9724815Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-06T20:43:12.1640228Z 
---
2020-02-06T20:43:32.3562290Z  finished in 20.383
2020-02-06T20:43:32.3820021Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-06T20:43:32.5639525Z 
2020-02-06T20:43:32.5639754Z running 116 tests
2020-02-06T20:43:46.7355248Z iiiiiFFiFFFF.i..i...i.FiFi.i..i..i.FiiF..Fi.i..F.ii....F.FF..iiii.F..F....FiF....i..i.FFFF..ii.i.ii. 100/116
2020-02-06T20:43:48.6418880Z ....iiii...FFiiF
2020-02-06T20:43:48.6420941Z 
2020-02-06T20:43:48.6460415Z ---- [debuginfo-gdb] debuginfo/borrowed-basic.rs stdout ----
2020-02-06T20:43:48.6460415Z ---- [debuginfo-gdb] debuginfo/borrowed-basic.rs stdout ----
2020-02-06T20:43:48.6461203Z NOTE: compiletest thinks it is using GDB with native rust support
2020-02-06T20:43:48.6461266Z NOTE: compiletest thinks it is using GDB version 8001000
2020-02-06T20:43:48.6461348Z error: line not found in debugger output: $1 = true
2020-02-06T20:43:48.6461410Z status: exit code: 0
2020-02-06T20:43:48.6461410Z status: exit code: 0
2020-02-06T20:43:48.6462168Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-basic.gdb/borrowed-basic.debugger.script"
2020-02-06T20:43:48.6462732Z ------------------------------------------
2020-02-06T20:43:48.6462732Z ------------------------------------------
2020-02-06T20:43:48.6463019Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-02-06T20:43:48.6463087Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-02-06T20:43:48.6463162Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-02-06T20:43:48.6463223Z This is free software: you are free to change and redistribute it.
2020-02-06T20:43:48.6463454Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-02-06T20:43:48.6463522Z and "show warranty" for details.
2020-02-06T20:43:48.6463804Z This GDB was configured as "x86_64-linux-gnu".
2020-02-06T20:43:48.6463857Z Type "show configuration" for configuration details.
2020-02-06T20:43:48.6463923Z For bug reporting instructions, please see:
2020-02-06T20:43:48.6463972Z <http://www.gnu.org/software/gdb/bugs/>.
2020-02-06T20:43:48.6464023Z Find the GDB manual and other documentation resources online at:
2020-02-06T20:43:48.6464249Z <http://www.gnu.org/software/gdb/documentation/>.
2020-02-06T20:43:48.6464299Z For help, type "help".
2020-02-06T20:43:48.6464347Z Type "apropos word" to search for commands related to "word".
2020-02-06T20:43:48.6464681Z Breakpoint 1 at 0x98a: file /checkout/src/test/debuginfo/borrowed-basic.rs, line 162.
2020-02-06T20:43:48.6464754Z [Thread debugging using libthread_db enabled]
2020-02-06T20:43:48.6465226Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-02-06T20:43:48.6465272Z 
2020-02-06T20:43:48.6465566Z Breakpoint 1, borrowed_basic::main () at /checkout/src/test/debuginfo/borrowed-basic.rs:162
2020-02-06T20:43:48.6465761Z 162     zzz(); // #break
2020-02-06T20:43:48.6466073Z ------------------------------------------
2020-02-06T20:43:48.6466142Z stderr:
2020-02-06T20:43:48.6466357Z ------------------------------------------
2020-02-06T20:43:48.6466357Z ------------------------------------------
2020-02-06T20:43:48.6466878Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-basic.gdb/borrowed-basic.debugger.script:10: Error in sourced command file:
2020-02-06T20:43:48.6466964Z Cannot access memory at address 0xffffffffffffff01
2020-02-06T20:43:48.6467390Z ------------------------------------------
2020-02-06T20:43:48.6467428Z 
2020-02-06T20:43:48.6467473Z 
2020-02-06T20:43:48.6467750Z ---- [debuginfo-gdb] debuginfo/associated-types.rs stdout ----
2020-02-06T20:43:48.6467750Z ---- [debuginfo-gdb] debuginfo/associated-types.rs stdout ----
2020-02-06T20:43:48.6467818Z NOTE: compiletest thinks it is using GDB with native rust support
2020-02-06T20:43:48.6467886Z NOTE: compiletest thinks it is using GDB version 8001000
2020-02-06T20:43:48.6467963Z error: line not found in debugger output: $2 = 1
2020-02-06T20:43:48.6468166Z status: exit code: 0
2020-02-06T20:43:48.6468166Z status: exit code: 0
2020-02-06T20:43:48.6468589Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/associated-types.gdb/associated-types.debugger.script"
2020-02-06T20:43:48.6469055Z ------------------------------------------
2020-02-06T20:43:48.6469055Z ------------------------------------------
2020-02-06T20:43:48.6469331Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-02-06T20:43:48.6469384Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-02-06T20:43:48.6469572Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-02-06T20:43:48.6469833Z This is free software: you are free to change and redistribute it.
2020-02-06T20:43:48.6469971Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-02-06T20:43:48.6470028Z and "show warranty" for details.
2020-02-06T20:43:48.6470357Z This GDB was configured as "x86_64-linux-gnu".
2020-02-06T20:43:48.6470587Z Type "show configuration" for configuration details.
2020-02-06T20:43:48.6470643Z For bug reporting instructions, please see:
2020-02-06T20:43:48.6470710Z <http://www.gnu.org/software/gdb/bugs/>.
2020-02-06T20:43:48.6470760Z Find the GDB manual and other documentation resources online at:
2020-02-06T20:43:48.6470810Z <http://www.gnu.org/software/gdb/documentation/>.
2020-02-06T20:43:48.6470856Z For help, type "help".
2020-02-06T20:43:48.6470919Z Type "apropos word" to search for commands related to "word".
2020-02-06T20:43:48.6471434Z Breakpoint 1 at 0x90d: file /checkout/src/test/debuginfo/associated-types.rs, line 111.
2020-02-06T20:43:48.6471746Z Breakpoint 2 at 0x987: file /checkout/src/test/debuginfo/associated-types.rs, line 118.
2020-02-06T20:43:48.6472221Z Breakpoint 3 at 0x9d4: file /checkout/src/test/debuginfo/associated-types.rs, line 122.
2020-02-06T20:43:48.6472532Z Breakpoint 4 at 0xa4d: file /checkout/src/test/debuginfo/associated-types.rs, line 130.
2020-02-06T20:43:48.6473125Z Breakpoint 5 at 0xafc: file /checkout/src/test/debuginfo/associated-types.rs, line 137.
2020-02-06T20:43:48.6473509Z Breakpoint 6 at 0xae9: file /checkout/src/test/debuginfo/associated-types.rs, line 140.
2020-02-06T20:43:48.6473564Z [Thread debugging using libthread_db enabled]
2020-02-06T20:43:48.6473997Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-02-06T20:43:48.6474059Z 
2020-02-06T20:43:48.6474380Z Breakpoint 1, associated_types::assoc_struct (arg=...) at /checkout/src/test/debuginfo/associated-types.rs:111
2020-02-06T20:43:48.6474447Z 111     zzz(); // #break
2020-02-06T20:43:48.6474928Z $1 = associated_types::Struct<i32> {b: -1, b1: 0}
2020-02-06T20:43:48.6474968Z 
2020-02-06T20:43:48.6475262Z Breakpoint 2, associated_types::assoc_local (x=1) at /checkout/src/test/debuginfo/associated-types.rs:118
2020-02-06T20:43:48.6475316Z 118     zzz(); // #break
2020-02-06T20:43:48.6475533Z $2 = <optimized out>
2020-02-06T20:43:48.6475579Z $3 = <optimized out>
2020-02-06T20:43:48.6475607Z 
2020-02-06T20:43:48.6475956Z Breakpoint 3, associated_types::assoc_arg (arg=2) at /checkout/src/test/debuginfo/associated-types.rs:122
2020-02-06T20:43:48.6476010Z 122     zzz(); // #break
2020-02-06T20:43:48.6476051Z $4 = 2
2020-02-06T20:43:48.6476078Z 
2020-02-06T20:43:48.6476578Z Breakpoint 4, associated_types::assoc_tuple (arg=...) at /checkout/src/test/debuginfo/associated-types.rs:130
2020-02-06T20:43:48.6476639Z 130     zzz(); // #break
2020-02-06T20:43:48.6476682Z $5 = (4, 5)
2020-02-06T20:43:48.6476709Z 
2020-02-06T20:43:48.6477028Z Breakpoint 5, associated_types::assoc_enum (arg=...) at /checkout/src/test/debuginfo/associated-types.rs:137
2020-02-06T20:43:48.6477105Z 137             zzz(); // #break
2020-02-06T20:43:48.6477149Z $6 = <optimized out>
2020-02-06T20:43:48.6477218Z $7 = <optimized out>
2020-02-06T20:43:48.6477246Z 
2020-02-06T20:43:48.6477527Z Breakpoint 6, associated_types::assoc_enum (arg=...) at /checkout/src/test/debuginfo/associated-types.rs:140
2020-02-06T20:43:48.6477598Z 140             zzz(); // #break
2020-02-06T20:43:48.6477642Z $8 = <optimized out>
2020-02-06T20:43:48.6477684Z $9 = <optimized out>
2020-02-06T20:43:48.6477728Z [Inferior 1 (process 6557) exited normally]
2020-02-06T20:43:48.6477988Z ------------------------------------------
2020-02-06T20:43:48.6478034Z stderr:
2020-02-06T20:43:48.6478261Z ------------------------------------------
2020-02-06T20:43:48.6478293Z 
2020-02-06T20:43:48.6478293Z 
2020-02-06T20:43:48.6478508Z ------------------------------------------
2020-02-06T20:43:48.6478663Z 
2020-02-06T20:43:48.6478690Z 
2020-02-06T20:43:48.6478976Z ---- [debuginfo-gdb] debuginfo/borrowed-struct.rs stdout ----
2020-02-06T20:43:48.6479031Z NOTE: compiletest thinks it is using GDB with native rust support
2020-02-06T20:43:48.6479159Z NOTE: compiletest thinks it is using GDB version 8001000
2020-02-06T20:43:48.6479261Z error: line not found in debugger output: $3 = 23.5
2020-02-06T20:43:48.6479306Z status: exit code: 0
2020-02-06T20:43:48.6479306Z status: exit code: 0
2020-02-06T20:43:48.6479672Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-struct.gdb/borrowed-struct.debugger.script"
2020-02-06T20:43:48.6479961Z ------------------------------------------
2020-02-06T20:43:48.6479961Z ------------------------------------------
2020-02-06T20:43:48.6480192Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-02-06T20:43:48.6480261Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-02-06T20:43:48.6480315Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-02-06T20:43:48.6480377Z This is free software: you are free to change and redistribute it.
2020-02-06T20:43:48.6480449Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-02-06T20:43:48.6480504Z and "show warranty" for details.
2020-02-06T20:43:48.6480732Z This GDB was configured as "x86_64-linux-gnu".
2020-02-06T20:43:48.6480801Z Type "show configuration" for configuration details.
2020-02-06T20:43:48.6480850Z For bug reporting instructions, please see:
2020-02-06T20:43:48.6480896Z <http://www.gnu.org/software/gdb/bugs/>.
2020-02-06T20:43:48.6480961Z Find the GDB manual and other documentation resources online at:
2020-02-06T20:43:48.6481012Z <http://www.gnu.org/software/gdb/documentation/>.
2020-02-06T20:43:48.6481059Z For help, type "help".
2020-02-06T20:43:48.6481122Z Type "apropos word" to search for commands related to "word".
2020-02-06T20:43:48.6481392Z Breakpoint 1 at 0xd7b: file /checkout/src/test/debuginfo/borrowed-struct.rs, line 87.
2020-02-06T20:43:48.6481455Z [Thread debugging using libthread_db enabled]
2020-02-06T20:43:48.6481724Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-02-06T20:43:48.6481760Z 
2020-02-06T20:43:48.6482033Z Breakpoint 1, borrowed_struct::main () at /checkout/src/test/debuginfo/borrowed-struct.rs:87
2020-02-06T20:43:48.6482087Z 87     zzz(); // #break
2020-02-06T20:43:48.6482151Z $1 = borrowed_struct::SomeStruct {x: 10, y: 23.5}
2020-02-06T20:43:48.6482194Z $2 = 10
2020-02-06T20:43:48.6482438Z ------------------------------------------
2020-02-06T20:43:48.6482500Z stderr:
2020-02-06T20:43:48.6482712Z ------------------------------------------
2020-02-06T20:43:48.6482712Z ------------------------------------------
2020-02-06T20:43:48.6483036Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-struct.gdb/borrowed-struct.debugger.script:12: Error in sourced command file:
2020-02-06T20:43:48.6483139Z 
2020-02-06T20:43:48.6483390Z ------------------------------------------
2020-02-06T20:43:48.6483429Z 
2020-02-06T20:43:48.6483471Z 
2020-02-06T20:43:48.6483471Z 
2020-02-06T20:43:48.6483714Z ---- [debuginfo-gdb] debuginfo/borrowed-c-style-enum.rs stdout ----
2020-02-06T20:43:48.6483770Z NOTE: compiletest thinks it is using GDB with native rust support
2020-02-06T20:43:48.6483844Z NOTE: compiletest thinks it is using GDB version 8001000
2020-02-06T20:43:48.6483876Z 
2020-02-06T20:43:48.6483925Z error: line not found in debugger output: $1 = borrowed_c_style_enum::ABC::TheA
2020-02-06T20:43:48.6483973Z status: exit code: 0
2020-02-06T20:43:48.6484352Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-c-style-enum.gdb/borrowed-c-style-enum.debugger.script"
2020-02-06T20:43:48.6484624Z ------------------------------------------
2020-02-06T20:43:48.6484624Z ------------------------------------------
2020-02-06T20:43:48.6484874Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-02-06T20:43:48.6484926Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-02-06T20:43:48.6485054Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-02-06T20:43:48.6485125Z This is free software: you are free to change and redistribute it.
2020-02-06T20:43:48.6485230Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-02-06T20:43:48.6485284Z and "show warranty" for details.
2020-02-06T20:43:48.6485548Z This GDB was configured as "x86_64-linux-gnu".
2020-02-06T20:43:48.6485601Z Type "show configuration" for configuration details.
2020-02-06T20:43:48.6485649Z For bug reporting instructions, please see:
2020-02-06T20:43:48.6485711Z <http://www.gnu.org/software/gdb/bugs/>.
2020-02-06T20:43:48.6485762Z Find the GDB manual and other documentation resources online at:
2020-02-06T20:43:48.6485813Z <http://www.gnu.org/software/gdb/documentation/>.
2020-02-06T20:43:48.6485858Z For help, type "help".
2020-02-06T20:43:48.6485921Z Type "apropos word" to search for commands related to "word".
2020-02-06T20:43:48.6486193Z Breakpoint 1 at 0x8ef: file /checkout/src/test/debuginfo/borrowed-c-style-enum.rs, line 53.
2020-02-06T20:43:48.6486260Z [Thread debugging using libthread_db enabled]
2020-02-06T20:43:48.6486530Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-02-06T20:43:48.6486573Z 
2020-02-06T20:43:48.6486853Z Breakpoint 1, borrowed_c_style_enum::main () at /checkout/src/test/debuginfo/borrowed-c-style-enum.rs:53
2020-02-06T20:43:48.6486922Z 53     zzz(); // #break
2020-02-06T20:43:48.6487163Z ------------------------------------------
2020-02-06T20:43:48.6487208Z stderr:
2020-02-06T20:43:48.6487435Z ------------------------------------------
2020-02-06T20:43:48.6487435Z ------------------------------------------
2020-02-06T20:43:48.6487769Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-c-style-enum.gdb/borrowed-c-style-enum.debugger.script:10: Error in sourced command file:
2020-02-06T20:43:48.6487829Z Cannot access memory at address 0x55555549b9020100
2020-02-06T20:43:48.6488088Z ------------------------------------------
2020-02-06T20:43:48.6488128Z 
2020-02-06T20:43:48.6488153Z 
2020-02-06T20:43:48.6488401Z ---- [debuginfo-gdb] debuginfo/borrowed-tuple.rs stdout ----
2020-02-06T20:43:48.6488401Z ---- [debuginfo-gdb] debuginfo/borrowed-tuple.rs stdout ----
2020-02-06T20:43:48.6488464Z NOTE: compiletest thinks it is using GDB with native rust support
2020-02-06T20:43:48.6488515Z NOTE: compiletest thinks it is using GDB version 8001000
2020-02-06T20:43:48.6488546Z 
2020-02-06T20:43:48.6488795Z error: line not found in debugger output: $1 = (-14, -19)
2020-02-06T20:43:48.6488846Z status: exit code: 0
2020-02-06T20:43:48.6489184Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-tuple.gdb/borrowed-tuple.debugger.script"
2020-02-06T20:43:48.6489470Z ------------------------------------------
2020-02-06T20:43:48.6489470Z ------------------------------------------
2020-02-06T20:43:48.6489702Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-02-06T20:43:48.6489769Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-02-06T20:43:48.6489823Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-02-06T20:43:48.6489883Z This is free software: you are free to change and redistribute it.
2020-02-06T20:43:48.6489961Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-02-06T20:43:48.6490011Z and "show warranty" for details.
2020-02-06T20:43:48.6490239Z This GDB was configured as "x86_64-linux-gnu".
2020-02-06T20:43:48.6490307Z Type "show configuration" for configuration details.
2020-02-06T20:43:48.6490358Z For bug reporting instructions, please see:
2020-02-06T20:43:48.6490404Z <http://www.gnu.org/software/gdb/bugs/>.
2020-02-06T20:43:48.6490454Z Find the GDB manual and other documentation resources online at:
2020-02-06T20:43:48.6490519Z <http://www.gnu.org/software/gdb/documentation/>.
2020-02-06T20:43:48.6490565Z For help, type "help".
2020-02-06T20:43:48.6490612Z Type "apropos word" to search for commands related to "word".
2020-02-06T20:43:48.6490895Z Breakpoint 1 at 0xcac: file /checkout/src/test/debuginfo/borrowed-tuple.rs, line 52.
2020-02-06T20:43:48.6491026Z [Thread debugging using libthread_db enabled]
2020-02-06T20:43:48.6491298Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-02-06T20:43:48.6491351Z 
2020-02-06T20:43:48.6491677Z Breakpoint 1, borrowed_tuple::main () at /checkout/src/test/debuginfo/borrowed-tuple.rs:52
2020-02-06T20:43:48.6491735Z 52     zzz(); // #break
2020-02-06T20:43:48.6492015Z ------------------------------------------
2020-02-06T20:43:48.6492060Z stderr:
2020-02-06T20:43:48.6492268Z ------------------------------------------
2020-02-06T20:43:48.6492268Z ------------------------------------------
2020-02-06T20:43:48.6492603Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-tuple.gdb/borrowed-tuple.debugger.script:10: Error in sourced command file:
2020-02-06T20:43:48.6492662Z Cannot access memory at address 0xc1980000f7b4fff2
2020-02-06T20:43:48.6492921Z ------------------------------------------
2020-02-06T20:43:48.6492963Z 
2020-02-06T20:43:48.6492989Z 
2020-02-06T20:43:48.6493228Z ---- [debuginfo-gdb] debuginfo/borrowed-unique-basic.rs stdout ----
2020-02-06T20:43:48.6493228Z ---- [debuginfo-gdb] debuginfo/borrowed-unique-basic.rs stdout ----
2020-02-06T20:43:48.6493299Z NOTE: compiletest thinks it is using GDB with native rust support
2020-02-06T20:43:48.6493360Z NOTE: compiletest thinks it is using GDB version 8001000
2020-02-06T20:43:48.6493435Z error: line not found in debugger output: $1 = true
2020-02-06T20:43:48.6493494Z status: exit code: 0
2020-02-06T20:43:48.6493494Z status: exit code: 0
2020-02-06T20:43:48.6493855Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-unique-basic.gdb/borrowed-unique-basic.debugger.script"
2020-02-06T20:43:48.6494139Z ------------------------------------------
2020-02-06T20:43:48.6494139Z ------------------------------------------
2020-02-06T20:43:48.6494371Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-02-06T20:43:48.6494424Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-02-06T20:43:48.6494493Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-02-06T20:43:48.6494554Z This is free software: you are free to change and redistribute it.
2020-02-06T20:43:48.6494607Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-02-06T20:43:48.6494678Z and "show warranty" for details.
2020-02-06T20:43:48.6494906Z This GDB was configured as "x86_64-linux-gnu".
2020-02-06T20:43:48.6494959Z Type "show configuration" for configuration details.
2020-02-06T20:43:48.6495007Z For bug reporting instructions, please see:
2020-02-06T20:43:48.6495071Z <http://www.gnu.org/software/gdb/bugs/>.
2020-02-06T20:43:48.6495121Z Find the GDB manual and other documentation resources online at:
2020-02-06T20:43:48.6495170Z <http://www.gnu.org/software/gdb/documentation/>.
2020-02-06T20:43:48.6495231Z For help, type "help".
2020-02-06T20:43:48.6495279Z Type "apropos word" to search for commands related to "word".
2020-02-06T20:43:48.6495557Z Breakpoint 1 at 0x176c: file /checkout/src/test/debuginfo/borrowed-unique-basic.rs, line 166.
2020-02-06T20:43:48.6495637Z [Thread debugging using libthread_db enabled]
2020-02-06T20:43:48.6495891Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-02-06T20:43:48.6495926Z 
2020-02-06T20:43:48.6496339Z Breakpoint 1, borrowed_unique_basic::main () at /checkout/src/test/debuginfo/borrowed-unique-basic.rs:166
2020-02-06T20:43:48.6496390Z 166     zzz(); // #break
2020-02-06T20:43:48.6496658Z ------------------------------------------
2020-02-06T20:43:48.6496717Z stderr:
2020-02-06T20:43:48.6496916Z ------------------------------------------
2020-02-06T20:43:48.6496916Z ------------------------------------------
2020-02-06T20:43:48.6497226Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/borrowed-unique-basic.gdb/borrowed-unique-basic.debugger.script:10: Error in sourced command file:
2020-02-06T20:43:48.6497323Z 
2020-02-06T20:43:48.6497519Z ------------------------------------------
2020-02-06T20:43:48.6497548Z 
2020-02-06T20:43:48.6497663Z 
2020-02-06T20:43:48.6497663Z 
2020-02-06T20:43:48.6497908Z ---- [debuginfo-gdb] debuginfo/destructured-fn-argument.rs stdout ----
2020-02-06T20:43:48.6497960Z NOTE: compiletest thinks it is using GDB with native rust support
2020-02-06T20:43:48.6498084Z NOTE: compiletest thinks it is using GDB version 8001000
2020-02-06T20:43:48.6498162Z error: line not found in debugger output: $2 = false
2020-02-06T20:43:48.6498204Z status: exit code: 0
2020-02-06T20:43:48.6498204Z status: exit code: 0
2020-02-06T20:43:48.6498675Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/destructured-fn-argument.gdb/destructured-fn-argument.debugger.script"
2020-02-06T20:43:48.6498912Z ------------------------------------------
2020-02-06T20:43:48.6498912Z ------------------------------------------
2020-02-06T20:43:48.6499129Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-02-06T20:43:48.6499174Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-02-06T20:43:48.6499220Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-02-06T20:43:48.6499288Z This is free software: you are free to change and redistribute it.
2020-02-06T20:43:48.6499336Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-02-06T20:43:48.6499384Z and "show warranty" for details.
2020-02-06T20:43:48.6499599Z This GDB was configured as "x86_64-linux-gnu".
2020-02-06T20:43:48.6499644Z Type "show configuration" for configuration details.
2020-02-06T20:43:48.6499685Z For bug reporting instructions, please see:
2020-02-06T20:43:48.6499739Z <http://www.gnu.org/software/gdb/bugs/>.
2020-02-06T20:43:48.6499783Z Find the GDB manual and other documentation resources online at:
2020-02-06T20:43:48.6499827Z <http://www.gnu.org/software/gdb/documentation/>.
2020-02-06T20:43:48.6499866Z For help, type "help".
2020-02-06T20:43:48.6499922Z Type "apropos word" to search for commands related to "word".
2020-02-06T20:43:48.6500165Z Breakpoint 1 at 0xbe5: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 380.
2020-02-06T20:43:48.6500414Z Breakpoint 2 at 0xc08: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 384.
2020-02-06T20:43:48.6500671Z Breakpoint 3 at 0xc31: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 388.
2020-02-06T20:43:48.6500918Z Breakpoint 4 at 0xc57: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 392.
2020-02-06T20:43:48.6501160Z Breakpoint 5 at 0xc82: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 396.
2020-02-06T20:43:48.6501416Z Breakpoint 6 at 0xc94: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 400.
2020-02-06T20:43:48.6501653Z Breakpoint 7 at 0xcad: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 404.
2020-02-06T20:43:48.6501905Z Breakpoint 8 at 0xcd7: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 408.
2020-02-06T20:43:48.6502146Z Breakpoint 9 at 0xd01: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 412.
2020-02-06T20:43:48.6502395Z Breakpoint 10 at 0xd2d: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 418.
2020-02-06T20:43:48.6502654Z Breakpoint 11 at 0xd58: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 422.
2020-02-06T20:43:48.6502906Z Breakpoint 12 at 0xd88: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 426.
2020-02-06T20:43:48.6503148Z Breakpoint 13 at 0xdae: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 430.
2020-02-06T20:43:48.6503403Z Breakpoint 14 at 0xddf: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 434.
2020-02-06T20:43:48.6503648Z Breakpoint 15 at 0xe21: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 438.
2020-02-06T20:43:48.6503888Z Breakpoint 16 at 0xe49: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 442.
2020-02-06T20:43:48.6504146Z Breakpoint 17 at 0xe72: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 446.
2020-02-06T20:43:48.6504387Z Breakpoint 18 at 0xe85: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 450.
2020-02-06T20:43:48.6504716Z Breakpoint 19 at 0xe9a: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 454.
2020-02-06T20:43:48.6505052Z Breakpoint 20 at 0xec5: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 458.
2020-02-06T20:43:48.6505318Z Breakpoint 21 at 0xeef: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 462.
2020-02-06T20:43:48.6505560Z Breakpoint 22 at 0xf13: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 466.
2020-02-06T20:43:48.6505823Z Breakpoint 23 at 0x14b3: file /checkout/src/test/debuginfo/destructured-fn-argument.rs, line 494.
2020-02-06T20:43:48.6505872Z [Thread debugging using libthread_db enabled]
2020-02-06T20:43:48.6506091Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-02-06T20:43:48.6506137Z 
2020-02-06T20:43:48.6506393Z Breakpoint 1, destructured_fn_argument::simple_tuple () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:380
2020-02-06T20:43:48.6506447Z 380     zzz(); // #break
2020-02-06T20:43:48.6506499Z $1 = 1
2020-02-06T20:43:48.6506536Z $2 = <optimized out>
2020-02-06T20:43:48.6506560Z 
2020-02-06T20:43:48.6506823Z Breakpoint 2, destructured_fn_argument::nested_tuple () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:384
2020-02-06T20:43:48.6506887Z 384     zzz(); // #break
2020-02-06T20:43:48.6506922Z $3 = 2
2020-02-06T20:43:48.6506956Z $4 = 3
2020-02-06T20:43:48.6507006Z $5 = <optimized out>
2020-02-06T20:43:48.6507031Z 
2020-02-06T20:43:48.6507301Z Breakpoint 3, destructured_fn_argument::destructure_only_first_level () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:388
2020-02-06T20:43:48.6507349Z 388     zzz(); // #break
2020-02-06T20:43:48.6507400Z $6 = 5
2020-02-06T20:43:48.6507435Z $7 = (6, 7)
2020-02-06T20:43:48.6507459Z 
2020-02-06T20:43:48.6507737Z Breakpoint 4, destructured_fn_argument::struct_as_tuple_element () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:392
2020-02-06T20:43:48.6507794Z 392     zzz(); // #break
2020-02-06T20:43:48.6507830Z $8 = 8
2020-02-06T20:43:48.6507869Z $9 = destructured_fn_argument::Struct {a: 9, b: 10}
2020-02-06T20:43:48.6507924Z $10 = <optimized out>
2020-02-06T20:43:48.6507955Z 
2020-02-06T20:43:48.6508216Z Breakpoint 5, destructured_fn_argument::struct_pattern () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:396
2020-02-06T20:43:48.6508278Z 396     zzz(); // #break
2020-02-06T20:43:48.6508316Z $11 = 12
2020-02-06T20:43:48.6508353Z $12 = <optimized out>
2020-02-06T20:43:48.6508376Z 
2020-02-06T20:43:48.6508654Z Breakpoint 6, destructured_fn_argument::ignored_tuple_element () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:400
2020-02-06T20:43:48.6508702Z 400     zzz(); // #break
2020-02-06T20:43:48.6508738Z $13 = 14
2020-02-06T20:43:48.6508790Z $14 = <optimized out>
2020-02-06T20:43:48.6508813Z 
2020-02-06T20:43:48.6509073Z Breakpoint 7, destructured_fn_argument::ignored_struct_field () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:404
2020-02-06T20:43:48.6509126Z 404     zzz(); // #break
2020-02-06T20:43:48.6509209Z $15 = <optimized out>
2020-02-06T20:43:48.6509233Z 
2020-02-06T20:43:48.6509510Z Breakpoint 8, destructured_fn_argument::one_struct_destructured_one_not () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:408
2020-02-06T20:43:48.6509576Z 408     zzz(); // #break
2020-02-06T20:43:48.6509612Z $16 = 19
2020-02-06T20:43:48.6509648Z $17 = <optimized out>
2020-02-06T20:43:48.6509688Z $18 = destructured_fn_argument::Struct {a: 21, b: 22}
2020-02-06T20:43:48.6509730Z 
2020-02-06T20:43:48.6510002Z Breakpoint 9, destructured_fn_argument::different_order_of_struct_fields () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:412
2020-02-06T20:43:48.6510050Z 412     zzz(); // #break
2020-02-06T20:43:48.6510102Z $19 = 24
2020-02-06T20:43:48.6510138Z $20 = <optimized out>
2020-02-06T20:43:48.6510162Z 
2020-02-06T20:43:48.6510421Z Breakpoint 10, destructured_fn_argument::complex_nesting () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:418
2020-02-06T20:43:48.6510556Z 418     zzz(); // #break
2020-02-06T20:43:48.6510592Z $21 = 25
2020-02-06T20:43:48.6510628Z $22 = <optimized out>
2020-02-06T20:43:48.6510728Z $23 = <optimized out>
2020-02-06T20:43:48.6510769Z $24 = 28
2020-02-06T20:43:48.6510804Z $25 = 29
2020-02-06T20:43:48.6510853Z $26 = <optimized out>
2020-02-06T20:43:48.6510890Z $27 = <optimized out>
2020-02-06T20:43:48.6510926Z $28 = <optimized out>
2020-02-06T20:43:48.6510961Z $29 = <optimized out>
2020-02-06T20:43:48.6511000Z 
2020-02-06T20:43:48.6511281Z Breakpoint 11, destructured_fn_argument::managed_box () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:422
2020-02-06T20:43:48.6511328Z 422     zzz(); // #break
2020-02-06T20:43:48.6511381Z $30 = (34, 35)
2020-02-06T20:43:48.6511406Z 
2020-02-06T20:43:48.6511662Z Breakpoint 12, destructured_fn_argument::borrowed_pointer () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:426
2020-02-06T20:43:48.6511717Z 426     zzz(); // #break
2020-02-06T20:43:48.6511769Z $31 = (36, 37)
2020-02-06T20:43:48.6511793Z 
2020-02-06T20:43:48.6512068Z Breakpoint 13, destructured_fn_argument::contained_borrowed_pointer () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:430
2020-02-06T20:43:48.6512131Z 430     zzz(); // #break
2020-02-06T20:43:48.6512168Z $32 = <optimized out>
2020-02-06T20:43:48.6512192Z 
2020-02-06T20:43:48.6512452Z Breakpoint 14, destructured_fn_argument::unique_pointer () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:434
2020-02-06T20:43:48.6512513Z 434     zzz(); // #break
2020-02-06T20:43:48.6512551Z $33 = (40, 41, 42)
2020-02-06T20:43:48.6512575Z 
2020-02-06T20:43:48.6519531Z Breakpoint 15, destructured_fn_argument::ref_binding () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:438
2020-02-06T20:43:48.6519739Z 438     zzz(); // #break
2020-02-06T20:43:48.6519974Z $34 = (43, 44, 45)
2020-02-06T20:43:48.6520009Z 
2020-02-06T20:43:48.6520610Z Breakpoint 16, destructured_fn_argument::ref_binding_in_tuple () at /checkout/src/test/debuginfo/destructured-fn-argument.rs:442
2020-02-06T20:43:48.6520672Z 442     zzz(); // #break
2020-02-06T20:43:48.6521075Z ------------------------------------------
2020-02-06T20:43:48.6521140Z stderr:
2020-02-06T20:43:48.6521353Z ------------------------------------------
2020-02-06T20:43:48.6521353Z ------------------------------------------
2020-02-06T20:43:48.6522019Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/destructured-fn-argument.gdb/destructured-fn-argument.debugger.script:81: Error in sourced command file:
2020-02-06T20:43:48.6522236Z 
2020-02-06T20:43:48.6522445Z ------------------------------------------
2020-02-06T20:43:48.6522475Z 
2020-02-06T20:43:48.6522516Z 
2020-02-06T20:43:48.6522516Z 
2020-02-06T20:43:48.6522748Z ---- [debuginfo-gdb] debuginfo/destructured-local.rs stdout ----
2020-02-06T20:43:48.6522917Z NOTE: compiletest thinks it is using GDB with native rust support
2020-02-06T20:43:48.6522981Z NOTE: compiletest thinks it is using GDB version 8001000
2020-02-06T20:43:48.6523075Z error: line not found in debugger output: $34 = (43, 44, 45)
2020-02-06T20:43:48.6523121Z status: exit code: 0
2020-02-06T20:43:48.6523121Z status: exit code: 0
2020-02-06T20:43:48.6523507Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/destructured-local.gdb/destructured-local.debugger.script"
2020-02-06T20:43:48.6523784Z ------------------------------------------
2020-02-06T20:43:48.6523784Z ------------------------------------------
2020-02-06T20:43:48.6524033Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-02-06T20:43:48.6524085Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-02-06T20:43:48.6524139Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-02-06T20:43:48.6524207Z This is free software: you are free to change and redistribute it.
2020-02-06T20:43:48.6524261Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-02-06T20:43:48.6524475Z and "show warranty" for details.
2020-02-06T20:43:48.6524880Z This GDB was configured as "x86_64-linux-gnu".
2020-02-06T20:43:48.6524933Z Type "show configuration" for configuration details.
2020-02-06T20:43:48.6525084Z For bug reporting instructions, please see:
2020-02-06T20:43:48.6530638Z <http://www.gnu.org/software/gdb/bugs/>.
2020-02-06T20:43:48.6530752Z Find the GDB manual and other documentation resources online at:
2020-02-06T20:43:48.6530803Z <http://www.gnu.org/software/gdb/documentation/>.
2020-02-06T20:43:48.6530848Z For help, type "help".
2020-02-06T20:43:48.6530910Z Type "apropos word" to search for commands related to "word".
2020-02-06T20:43:48.6531594Z Breakpoint 1 at 0x10aa: file /checkout/src/test/debuginfo/destructured-local.rs, line 371.
2020-02-06T20:43:48.6531656Z [Thread debugging using libthread_db enabled]
2020-02-06T20:43:48.6531929Z Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
2020-02-06T20:43:48.6531965Z 
2020-02-06T20:43:48.6532241Z Breakpoint 1, destructured_local::main () at /checkout/src/test/debuginfo/destructured-local.rs:371
2020-02-06T20:43:48.6532305Z 371     zzz(); // #break
2020-02-06T20:43:48.6532345Z $1 = 1
2020-02-06T20:43:48.6532383Z $2 = false
2020-02-06T20:43:48.6532429Z $3 = 2
2020-02-06T20:43:48.6532483Z $4 = 3
2020-02-06T20:43:48.6532520Z $5 = 4
2020-02-06T20:43:48.6532556Z $6 = 5
2020-02-06T20:43:48.6532610Z $7 = (6, 7)
2020-02-06T20:43:48.6532648Z $8 = 8
2020-02-06T20:43:48.6532690Z $9 = destructured_local::Struct {a: 9, b: 10}
2020-02-06T20:43:48.6532731Z $10 = 11
2020-02-06T20:43:48.6532784Z $11 = 12
2020-02-06T20:43:48.6532821Z $12 = 13
2020-02-06T20:43:48.6532858Z $13 = 14
2020-02-06T20:43:48.6532908Z $14 = 16
2020-02-06T20:43:48.6532945Z $15 = 18
2020-02-06T20:43:48.6532981Z $16 = 19
2020-02-06T20:43:48.6533130Z $17 = 20
2020-02-06T20:43:48.6533194Z $18 = destructured_local::Struct {a: 21, b: 22}
2020-02-06T20:43:48.6533238Z $19 = 24
2020-02-06T20:43:48.6533277Z $20 = 23
2020-02-06T20:43:48.6533331Z $21 = 25
2020-02-06T20:43:48.6533378Z $22 = 26
2020-02-06T20:43:48.6533469Z $23 = 27
2020-02-06T20:43:48.6533508Z $24 = 28
2020-02-06T20:43:48.6533563Z $25 = 29
2020-02-06T20:43:48.6533601Z $26 = 30
2020-02-06T20:43:48.6533640Z $27 = 31
2020-02-06T20:43:48.6533701Z $28 = 32
2020-02-06T20:43:48.6533741Z $29 = 33
2020-02-06T20:43:48.6533782Z $30 = (34, 35)
2020-02-06T20:43:48.6533823Z $31 = (36, 37)
2020-02-06T20:43:48.6533880Z $32 = 38
2020-02-06T20:43:48.6533921Z $33 = (40, 41, 42)
2020-02-06T20:43:48.6534215Z ------------------------------------------
2020-02-06T20:43:48.6534262Z stderr:
2020-02-06T20:43:48.6534475Z ------------------------------------------
2020-02-06T20:43:48.6534475Z ------------------------------------------
2020-02-06T20:43:48.6534807Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/destructured-local.gdb/destructured-local.debugger.script:43: Error in sourced command file:
2020-02-06T20:43:48.6534911Z 
2020-02-06T20:43:48.6535128Z ------------------------------------------
2020-02-06T20:43:48.6535168Z 
2020-02-06T20:43:48.6535211Z 
2020-02-06T20:43:48.6535211Z 
2020-02-06T20:43:48.6535460Z ---- [debuginfo-gdb] debuginfo/gdb-pretty-struct-and-enums.rs stdout ----
2020-02-06T20:43:48.6535524Z NOTE: compiletest thinks it is using GDB with native rust support
2020-02-06T20:43:48.6535591Z NOTE: compiletest thinks it is using GDB version 8001000
2020-02-06T20:43:48.6535623Z 
2020-02-06T20:43:48.6535672Z error: line not found in debugger output: $2 = gdb_pretty_struct_and_enums::EmptyStruct
2020-02-06T20:43:48.6535720Z status: exit code: 0
2020-02-06T20:43:48.6536112Z command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/gdb-pretty-struct-and-enums.gdb/gdb-pretty-struct-and-enums.debugger.script"
2020-02-06T20:43:48.6536606Z ------------------------------------------
2020-02-06T20:43:48.6536606Z ------------------------------------------
2020-02-06T20:43:48.6536841Z GNU gdb (Ubuntu 8.1-0ubuntu3.2) 8.1.0.20180409-git
2020-02-06T20:43:48.6536890Z Copyright (C) 2018 Free Software Foundation, Inc.
2020-02-06T20:43:48.6537095Z License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
2020-02-06T20:43:48.6537164Z This is free software: you are free to change and redistribute it.
2020-02-06T20:43:48.6537278Z There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
2020-02-06T20:43:48.6537330Z and "show warranty" for details.
2020-02-06T20:43:48.6537602Z This GDB was configured as "x86_64-linux-gnu".
---
2020-02-06T20:43:48.6698317Z test result: FAILED. 52 passed; 26 failed; 38 ignored; 0 measured; 0 filtered out
2020-02-06T20:43:48.6698367Z 
2020-02-06T20:43:48.6698392Z 
2020-02-06T20:43:48.6698417Z 
2020-02-06T20:43:48.6699962Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-06T20:43:48.6700310Z 
2020-02-06T20:43:48.6700343Z 
2020-02-06T20:43:48.6700636Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-06T20:43:48.6700691Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-06T20:43:48.6700691Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-06T20:43:48.6700744Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-06T20:43:48.6700805Z Build completed unsuccessfully in 1:08:04
2020-02-06T20:43:48.6700847Z == clock drift check ==
2020-02-06T20:43:48.6700890Z   local time: Thu Feb  6 20:43:48 UTC 2020
2020-02-06T20:43:48.8315681Z   network time: Thu, 06 Feb 2020 20:43:48 GMT
2020-02-06T20:43:48.8315906Z == end clock drift check ==
2020-02-06T20:43:49.8034471Z 
2020-02-06T20:43:49.8176199Z ##[error]Bash exited with code '1'.
2020-02-06T20:43:49.8187985Z ##[section]Finishing: Run build
2020-02-06T20:43:49.8211523Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68902/merge to s
2020-02-06T20:43:49.8213396Z Task         : Get sources
2020-02-06T20:43:49.8213459Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-06T20:43:49.8213505Z Version      : 1.0.0
2020-02-06T20:43:49.8213546Z Author       : Microsoft
2020-02-06T20:43:49.8213546Z Author       : Microsoft
2020-02-06T20:43:49.8213609Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-06T20:43:49.8213659Z ==============================================================================
2020-02-06T20:43:50.2641370Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-06T20:43:50.2684208Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68902/merge to s
2020-02-06T20:43:50.2794097Z Cleaning up task key
2020-02-06T20:43:50.2794861Z Start cleaning up orphan processes.
2020-02-06T20:43:50.2906721Z Terminate orphan process: pid (3691) (python)
2020-02-06T20:43:50.3170136Z ##[section]Finishing: Finalize Job
