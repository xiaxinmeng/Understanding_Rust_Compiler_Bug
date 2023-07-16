plain
2020-03-06T21:14:47.1030287Z ========================== Starting Command Output ===========================
2020-03-06T21:14:47.1034007Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/dc2ec7c1-b68e-4399-82d4-f09b2f6e0a5b.sh
2020-03-06T21:14:47.1034273Z 
2020-03-06T21:14:47.1038636Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-06T21:14:47.1061085Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69718/merge to s
2020-03-06T21:14:47.1064476Z Task         : Get sources
2020-03-06T21:14:47.1064766Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-06T21:14:47.1065367Z Version      : 1.0.0
2020-03-06T21:14:47.1065570Z Author       : Microsoft
---
2020-03-06T21:14:48.0933119Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-06T21:14:48.0947737Z ##[command]git config gc.auto 0
2020-03-06T21:14:48.0951785Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-06T21:14:48.0954810Z ##[command]git config --get-all http.proxy
2020-03-06T21:14:48.0960535Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69718/merge:refs/remotes/pull/69718/merge
---
2020-03-06T22:18:25.9429527Z .................................................................................................... 1700/9732
2020-03-06T22:18:30.8496777Z .................................................................................................... 1800/9732
2020-03-06T22:18:43.3949048Z ..........................................................i......................................... 1900/9732
2020-03-06T22:18:51.5703253Z .................................................................................................... 2000/9732
2020-03-06T22:19:06.3415268Z ................................................iiiii............................................... 2100/9732
2020-03-06T22:19:17.7064341Z .................................................................................................... 2300/9732
2020-03-06T22:19:20.1971058Z .................................................................................................... 2400/9732
2020-03-06T22:19:24.0302280Z .................................................................................................... 2500/9732
2020-03-06T22:19:47.1616475Z .................................................................................................... 2600/9732
---
2020-03-06T22:22:35.7960338Z .........i...............i.......................................................................... 5000/9732
2020-03-06T22:22:46.4733205Z .................................................................................................... 5100/9732
2020-03-06T22:22:51.7418544Z ....................................................i............................................... 5200/9732
2020-03-06T22:23:00.7763271Z .................................................................................................... 5300/9732
2020-03-06T22:23:08.5528067Z ................................ii.ii........i...i.................................................. 5400/9732
2020-03-06T22:23:17.2660602Z .................................................................................................... 5600/9732
2020-03-06T22:23:27.0832610Z .................................................................................................... 5700/9732
2020-03-06T22:23:34.3629596Z .......................i............................................................................ 5800/9732
2020-03-06T22:23:40.1302046Z .................................................................................................... 5900/9732
2020-03-06T22:23:40.1302046Z .................................................................................................... 5900/9732
2020-03-06T22:23:51.7220746Z .................................................................................................... 6000/9732
2020-03-06T22:24:02.6481290Z ...............ii...i..ii...........i............................................................... 6100/9732
2020-03-06T22:24:20.2438390Z .................................................................................................... 6300/9732
2020-03-06T22:24:27.7276347Z .................................................................................................... 6400/9732
2020-03-06T22:24:27.7276347Z .................................................................................................... 6400/9732
2020-03-06T22:24:44.5661670Z ..............................................i..ii................................................. 6500/9732
2020-03-06T22:25:09.3348966Z .................................................................................................... 6700/9732
2020-03-06T22:25:11.7735259Z ......................................i............................................................. 6800/9732
2020-03-06T22:25:14.2214723Z .................................................................................................... 6900/9732
2020-03-06T22:25:16.6301920Z ....................................................................i............................... 7000/9732
---
2020-03-06T22:27:06.9359323Z .................................................................................................... 7700/9732
2020-03-06T22:27:12.4812415Z .................................................................................................... 7800/9732
2020-03-06T22:27:17.7469088Z .................................................................................................... 7900/9732
2020-03-06T22:27:26.6477406Z ..............i..................................................................................... 8000/9732
2020-03-06T22:27:35.6658315Z ...............................................................iiiiiiiii.i.......................... 8100/9732
2020-03-06T22:27:51.7596424Z ......i......i...................................................................................... 8300/9732
2020-03-06T22:27:57.4638965Z .................................................................................................... 8400/9732
2020-03-06T22:28:12.2422918Z .................................................................................................... 8500/9732
2020-03-06T22:28:22.2725397Z .................................................................................................... 8600/9732
---
2020-03-06T22:30:52.4890452Z  finished in 8.022
2020-03-06T22:30:52.5075597Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-06T22:30:52.7191433Z 
2020-03-06T22:30:52.7192107Z running 180 tests
2020-03-06T22:30:55.9654026Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/180
2020-03-06T22:30:58.4000574Z ...i.i.i.FFiii..iiiiiiiiiiiiiiii.......................iii..............ii......
2020-03-06T22:30:58.4003034Z 
2020-03-06T22:30:58.4003587Z ---- [codegen] codegen/remap_path_prefix/main.rs stdout ----
2020-03-06T22:30:58.4003787Z 
2020-03-06T22:30:58.4003787Z 
2020-03-06T22:30:58.4004367Z error: verification with 'FileCheck' failed
2020-03-06T22:30:58.4004738Z status: exit code: 1
2020-03-06T22:30:58.4005642Z command: "/usr/lib/llvm-7/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/main/main.ll" "/checkout/src/test/codegen/remap_path_prefix/main.rs"
2020-03-06T22:30:58.4006465Z ------------------------------------------
2020-03-06T22:30:58.4006783Z 
2020-03-06T22:30:58.4007195Z ------------------------------------------
2020-03-06T22:30:58.4007389Z stderr:
2020-03-06T22:30:58.4007389Z stderr:
2020-03-06T22:30:58.4007918Z ------------------------------------------
2020-03-06T22:30:58.4008425Z /checkout/src/test/codegen/remap_path_prefix/main.rs:28:11: error: CHECK: expected string not found in input
2020-03-06T22:30:58.4009460Z // CHECK: !DIFile(filename: "/the/aux-src/remap_path_prefix_aux.rs", directory: "")
2020-03-06T22:30:58.4010119Z           ^
2020-03-06T22:30:58.4010754Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/main/main.ll:292:1: note: scanning from here
2020-03-06T22:30:58.4011244Z !31 = !{!0, !14}
2020-03-06T22:30:58.4011736Z ^
2020-03-06T22:30:58.4012572Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/main/main.ll:294:7: note: possible intended match here
2020-03-06T22:30:58.4013748Z !33 = !DIFile(filename: "/the/aux-src/remap_path_prefix_aux.rs", directory: "", checksumkind: CSK_MD5, checksum: "dc542ec503a403f5f76478f17ed0322a")
2020-03-06T22:30:58.4015082Z 
2020-03-06T22:30:58.4015550Z ------------------------------------------
2020-03-06T22:30:58.4015715Z 
2020-03-06T22:30:58.4016294Z 
2020-03-06T22:30:58.4016294Z 
2020-03-06T22:30:58.4017301Z ---- [codegen] codegen/remap_path_prefix/xcrate-generic.rs stdout ----
2020-03-06T22:30:58.4017548Z 
2020-03-06T22:30:58.4018159Z error: verification with 'FileCheck' failed
2020-03-06T22:30:58.4018461Z status: exit code: 1
2020-03-06T22:30:58.4019485Z command: "/usr/lib/llvm-7/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/xcrate-generic/xcrate-generic.ll" "/checkout/src/test/codegen/remap_path_prefix/xcrate-generic.rs"
2020-03-06T22:30:58.4020916Z ------------------------------------------
2020-03-06T22:30:58.4021080Z 
2020-03-06T22:30:58.4021613Z ------------------------------------------
2020-03-06T22:30:58.4021961Z stderr:
2020-03-06T22:30:58.4021961Z stderr:
2020-03-06T22:30:58.4022377Z ------------------------------------------
2020-03-06T22:30:58.4023154Z /checkout/src/test/codegen/remap_path_prefix/xcrate-generic.rs:14:11: error: CHECK: expected string not found in input
2020-03-06T22:30:58.4024045Z // CHECK: !DIFile(filename: "/the/aux-src/xcrate-generic.rs", directory: "")
2020-03-06T22:30:58.4024668Z           ^
2020-03-06T22:30:58.4029969Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/xcrate-generic/xcrate-generic.ll:1:1: note: scanning from here
2020-03-06T22:30:58.4030797Z ; ModuleID = 'xcrate_generic.515safou-cgu.0'
2020-03-06T22:30:58.4031124Z ^
2020-03-06T22:30:58.4031841Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/xcrate-generic/xcrate-generic.ll:39:6: note: possible intended match here
2020-03-06T22:30:58.4032931Z !7 = !DIFile(filename: "/the/aux-src/xcrate-generic.rs", directory: "", checksumkind: CSK_MD5, checksum: "27cb50785fd61df201391d2b01e4c43c")
2020-03-06T22:30:58.4033592Z 
2020-03-06T22:30:58.4034207Z ------------------------------------------
2020-03-06T22:30:58.4034397Z 
2020-03-06T22:30:58.4034620Z 
---
2020-03-06T22:30:58.4038221Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-06T22:30:58.4038787Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-06T22:30:58.4039119Z 
2020-03-06T22:30:58.4039277Z 
2020-03-06T22:30:58.4043096Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-06T22:30:58.4052058Z 
2020-03-06T22:30:58.4052358Z 
2020-03-06T22:30:58.4057958Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-06T22:30:58.4058331Z Build completed unsuccessfully in 1:10:27
2020-03-06T22:30:58.4058331Z Build completed unsuccessfully in 1:10:27
2020-03-06T22:30:58.4115972Z == clock drift check ==
2020-03-06T22:30:59.0883181Z   local time: Fri Mar  6 22:30:58 UTC 2020
2020-03-06T22:30:59.0883577Z   network time: Fri, 06 Mar 2020 22:30:58 GMT
2020-03-06T22:30:59.0884009Z == end clock drift check ==
2020-03-06T22:31:00.7121117Z 
2020-03-06T22:31:00.7208473Z ##[error]Bash exited with code '1'.
2020-03-06T22:31:00.7223706Z ##[section]Finishing: Run build
2020-03-06T22:31:00.7273178Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69718/merge to s
2020-03-06T22:31:00.7278689Z Task         : Get sources
2020-03-06T22:31:00.7279036Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-06T22:31:00.7279350Z Version      : 1.0.0
2020-03-06T22:31:00.7279583Z Author       : Microsoft
2020-03-06T22:31:00.7279583Z Author       : Microsoft
2020-03-06T22:31:00.7279922Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-06T22:31:00.7280315Z ==============================================================================
2020-03-06T22:31:01.0850802Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-06T22:31:01.0937681Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69718/merge to s
2020-03-06T22:31:01.1052990Z Cleaning up task key
2020-03-06T22:31:01.1054468Z Start cleaning up orphan processes.
2020-03-06T22:31:01.1243855Z Terminate orphan process: pid (3772) (python)
2020-03-06T22:31:01.1485460Z ##[section]Finishing: Finalize Job
