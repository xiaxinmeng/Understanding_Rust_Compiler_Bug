plain
2020-01-30T10:47:47.9328428Z ========================== Starting Command Output ===========================
2020-01-30T10:47:47.9330160Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3bb738cd-e496-4bd5-80f8-c493c0411ed4.sh
2020-01-30T10:47:47.9330200Z 
2020-01-30T10:47:47.9333245Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-30T10:47:47.9340855Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68664/merge to s
2020-01-30T10:47:47.9342862Z Task         : Get sources
2020-01-30T10:47:47.9342952Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-30T10:47:47.9342991Z Version      : 1.0.0
2020-01-30T10:47:47.9343027Z Author       : Microsoft
---
2020-01-30T10:47:48.9907837Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-30T10:47:48.9968714Z ##[command]git config gc.auto 0
2020-01-30T10:47:48.9971397Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-30T10:47:48.9973670Z ##[command]git config --get-all http.proxy
2020-01-30T10:47:48.9984306Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68664/merge:refs/remotes/pull/68664/merge
---
2020-01-30T11:45:56.3187257Z .................................................................................................... 1700/9558
2020-01-30T11:46:01.3179809Z .................................................................................................... 1800/9558
2020-01-30T11:46:14.4225888Z .........................i.......................................................................... 1900/9558
2020-01-30T11:46:21.5463168Z .................................................................................................... 2000/9558
2020-01-30T11:46:36.5430342Z ...............iiiii................................................................................ 2100/9558
2020-01-30T11:46:46.4623843Z .................................................................................................... 2300/9558
2020-01-30T11:46:49.3343858Z .................................................................................................... 2400/9558
2020-01-30T11:46:54.8355525Z .................................................................................................... 2500/9558
2020-01-30T11:47:15.3176843Z .................................................................................................... 2600/9558
---
2020-01-30T11:49:51.1469668Z .................................................................................................... 4800/9558
2020-01-30T11:49:56.1969265Z ...........................................................i...............i........................ 4900/9558
2020-01-30T11:50:04.1274710Z .................................................................................................... 5000/9558
2020-01-30T11:50:12.4547831Z .................................................................................................... 5100/9558
2020-01-30T11:50:17.2003332Z ..i................................................................................................. 5200/9558
2020-01-30T11:50:28.5686797Z ...........................................................................ii.ii........i...i....... 5300/9558
2020-01-30T11:50:37.3855472Z .............i...................................................................................... 5500/9558
2020-01-30T11:50:47.1166302Z .................................................................................................... 5600/9558
2020-01-30T11:50:53.4777101Z ..............................................................i..................................... 5700/9558
2020-01-30T11:51:00.6318254Z .................................................................................................... 5800/9558
2020-01-30T11:51:00.6318254Z .................................................................................................... 5800/9558
2020-01-30T11:51:08.4264974Z .................................................................................................... 5900/9558
2020-01-30T11:51:17.8945356Z .....................................................ii...i..ii...........i......................... 6000/9558
2020-01-30T11:51:38.9500279Z .................................................................................................... 6200/9558
2020-01-30T11:51:46.5187216Z .................................................................................................... 6300/9558
2020-01-30T11:51:46.5187216Z .................................................................................................... 6300/9558
2020-01-30T11:51:55.0799533Z .................................................................................i..ii.............. 6400/9558
2020-01-30T11:52:27.6391015Z .................................................................................................... 6600/9558
2020-01-30T11:52:33.2082354Z .........................................................i.......................................... 6700/9558
2020-01-30T11:52:35.3887262Z .................................................................................................... 6800/9558
2020-01-30T11:52:37.6526604Z .........................................................i.......................................... 6900/9558
---
2020-01-30T11:54:21.9343838Z .................................................................................................... 7600/9558
2020-01-30T11:54:27.3349509Z .................................................................................................... 7700/9558
2020-01-30T11:54:34.1655808Z .................................................................................................... 7800/9558
2020-01-30T11:54:45.0416749Z .................................................................................................... 7900/9558
2020-01-30T11:54:51.1505523Z ............iiiiiii.i............................................................................... 8000/9558
2020-01-30T11:55:05.6938871Z .................................................................................................... 8200/9558
2020-01-30T11:55:16.4931800Z .................................................................................................... 8300/9558
2020-01-30T11:55:29.8509775Z .................................................................................................... 8400/9558
2020-01-30T11:55:36.6211932Z .................................................................................................... 8500/9558
---
2020-01-30T11:58:00.9249528Z  finished in 7.504
2020-01-30T11:58:00.9447279Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-30T11:58:01.1127838Z 
2020-01-30T11:58:01.1128106Z running 169 tests
2020-01-30T11:58:04.1676784Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/169
2020-01-30T11:58:06.5124517Z i.i.i...iii..iiiiiiiiii.......................iii............ii......
2020-01-30T11:58:06.5125802Z 
2020-01-30T11:58:06.5129986Z  finished in 5.568
2020-01-30T11:58:06.5337151Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-30T11:58:06.6981725Z 
---
2020-01-30T11:58:08.6625940Z  finished in 2.129
2020-01-30T11:58:08.6845091Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-30T11:58:08.8449738Z 
2020-01-30T11:58:08.8451689Z running 9 tests
2020-01-30T11:58:08.8452963Z iiiiiiiii
2020-01-30T11:58:08.8453725Z 
2020-01-30T11:58:08.8454867Z  finished in 0.160
2020-01-30T11:58:08.8652506Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-30T11:58:09.0328361Z 
---
2020-01-30T11:58:29.5398357Z  finished in 20.674
2020-01-30T11:58:29.5597512Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-30T11:58:29.7233310Z 
2020-01-30T11:58:29.7233547Z running 116 tests
2020-01-30T11:58:43.5598830Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-01-30T11:58:45.4744849Z ....iiii.....ii.
2020-01-30T11:58:46.0030468Z 
2020-01-30T11:58:46.0030799Z  finished in 15.915
2020-01-30T11:58:46.0031299Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-30T11:58:46.0031833Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-30T12:12:55.8319676Z 
2020-01-30T12:12:55.8320610Z    Doc-tests core
2020-01-30T12:13:00.8509315Z 
2020-01-30T12:13:00.8509686Z running 2467 tests
2020-01-30T12:13:10.3225616Z ......iiiii......................................................................................... 100/2467
2020-01-30T12:13:19.4916854Z ..................................................................................ii................ 200/2467
2020-01-30T12:13:41.1499550Z .................i.................................................................................. 400/2467
2020-01-30T12:13:41.1499550Z .................i.................................................................................. 400/2467
2020-01-30T12:13:51.0139247Z ..................................................................i..i..................iiii........ 500/2467
2020-01-30T12:14:08.5028276Z .................................................................................................... 700/2467
2020-01-30T12:14:17.3695818Z .................................................................................................... 800/2467
2020-01-30T12:14:26.1219707Z .................................................................................................... 900/2467
2020-01-30T12:14:34.8314038Z .................................................................................................... 1000/2467
---
2020-01-30T12:18:20.5146831Z 
2020-01-30T12:18:20.5148372Z running 1007 tests
2020-01-30T12:18:39.3127239Z i................................................................................................... 100/1007
2020-01-30T12:18:49.6198159Z .................................................................................................... 200/1007
2020-01-30T12:18:58.9738528Z ..................iii......i......i...i......i...................................................... 300/1007
2020-01-30T12:19:10.7103800Z .................................................................................................... 400/1007
2020-01-30T12:19:20.1187712Z ..........................................i..i.....................................ii............... 500/1007
2020-01-30T12:19:41.4023822Z .................................................................................................... 700/1007
2020-01-30T12:19:41.4023822Z .................................................................................................... 700/1007
2020-01-30T12:19:52.7070432Z .................................iiii............................................................... 800/1007
2020-01-30T12:20:10.5249459Z .................................................................................................... 900/1007
2020-01-30T12:20:21.1302388Z .......................................................iiii......................................... 1000/1007
2020-01-30T12:20:21.7846079Z test result: ok. 987 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-01-30T12:20:21.7849506Z 
2020-01-30T12:20:21.7982536Z  finished in 207.662
2020-01-30T12:20:21.7994209Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-01-30T12:39:55.9596860Z 
2020-01-30T12:39:55.9597194Z ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0454 (line 8008) stdout ----
2020-01-30T12:39:55.9597271Z error: linking with `cc` failed: exit code: 1
2020-01-30T12:39:55.9597316Z   |
2020-01-30T12:39:55.9600997Z   = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/tmp/rustdoctestoWq26P/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "-o" "/tmp/rustdoctestoWq26P/rust_out" "/tmp/rustdoctestoWq26P/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lsome_lib" "-Wl,--start-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-8528e5f4833852aa.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-a0d0566ac62804f5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-6583bdb038f0769e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-68e6ba8672e810f3.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace-e8e99be55e599058.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-a16f20798c1b905e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-1aa22b9d384ca38f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-43fd232abb4b83a2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-33644c87047eb298.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-473fe78237295435.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-955c2db7d9774431.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-7215cce5c10397e6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-c21aa0a453978729.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-9915cc4411371776.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil"
2020-01-30T12:39:55.9601826Z   = note: /usr/bin/ld: cannot find -lsome_lib
2020-01-30T12:39:55.9601886Z           collect2: error: ld returned 1 exit status
2020-01-30T12:39:55.9601992Z 
2020-01-30T12:39:55.9602036Z error: aborting due to previous error
2020-01-30T12:39:55.9602082Z 
2020-01-30T12:39:55.9602318Z Couldn't compile the test.
2020-01-30T12:39:55.9602318Z Couldn't compile the test.
2020-01-30T12:39:55.9602640Z ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0459 (line 8058) stdout ----
2020-01-30T12:39:55.9602699Z error: linking with `cc` failed: exit code: 1
2020-01-30T12:39:55.9602764Z   |
2020-01-30T12:39:55.9605840Z   = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/tmp/rustdoctestooqPEN/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "-o" "/tmp/rustdoctestooqPEN/rust_out" "/tmp/rustdoctestooqPEN/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lsome_lib" "-Wl,--start-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-8528e5f4833852aa.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-a0d0566ac62804f5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-6583bdb038f0769e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-68e6ba8672e810f3.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace-e8e99be55e599058.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-a16f20798c1b905e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-1aa22b9d384ca38f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-43fd232abb4b83a2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-33644c87047eb298.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-473fe78237295435.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-955c2db7d9774431.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-7215cce5c10397e6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-c21aa0a453978729.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-9915cc4411371776.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil"
2020-01-30T12:39:55.9606589Z   = note: /usr/bin/ld: cannot find -lsome_lib
2020-01-30T12:39:55.9606644Z           collect2: error: ld returned 1 exit status
2020-01-30T12:39:55.9606755Z 
2020-01-30T12:39:55.9606800Z error: aborting due to previous error
2020-01-30T12:39:55.9606829Z 
2020-01-30T12:39:55.9607045Z Couldn't compile the test.
---
2020-01-30T12:39:55.9608555Z   local time: Thu Jan 30 12:39:55 UTC 2020
2020-01-30T12:39:56.2177971Z   network time: Thu, 30 Jan 2020 12:39:56 GMT
2020-01-30T12:39:56.2178216Z == end clock drift check ==
2020-01-30T12:39:56.5601748Z 
2020-01-30T12:39:56.5687295Z ##[error]Bash exited with code '1'.
2020-01-30T12:39:56.5702774Z ##[section]Finishing: Run build
2020-01-30T12:39:56.5728120Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68664/merge to s
2020-01-30T12:39:56.5730478Z Task         : Get sources
2020-01-30T12:39:56.5730529Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-30T12:39:56.5730591Z Version      : 1.0.0
2020-01-30T12:39:56.5730634Z Author       : Microsoft
2020-01-30T12:39:56.5730634Z Author       : Microsoft
2020-01-30T12:39:56.5730680Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-30T12:39:56.5730748Z ==============================================================================
2020-01-30T12:39:57.0074618Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-30T12:39:57.0123745Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68664/merge to s
2020-01-30T12:39:57.0246918Z Cleaning up task key
2020-01-30T12:39:57.0247860Z Start cleaning up orphan processes.
2020-01-30T12:39:57.0371648Z Terminate orphan process: pid (3566) (python)
2020-01-30T12:39:57.0635165Z ##[section]Finishing: Finalize Job
