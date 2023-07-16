plain
2020-03-17T14:16:28.6477002Z ========================== Starting Command Output ===========================
2020-03-17T14:16:28.6481630Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/05f13333-6029-4983-ae4f-c845e3830c22.sh
2020-03-17T14:16:28.6481931Z 
2020-03-17T14:16:28.6486893Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-17T14:16:28.6507247Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69916/merge to s
2020-03-17T14:16:28.6511034Z Task         : Get sources
2020-03-17T14:16:28.6511367Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T14:16:28.6511681Z Version      : 1.0.0
2020-03-17T14:16:28.6511914Z Author       : Microsoft
---
2020-03-17T14:16:29.6374541Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-17T14:16:29.6380277Z ##[command]git config gc.auto 0
2020-03-17T14:16:29.6384356Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-17T14:16:29.6388114Z ##[command]git config --get-all http.proxy
2020-03-17T14:16:29.6395114Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69916/merge:refs/remotes/pull/69916/merge
---
2020-03-17T15:19:47.7630108Z .................................................................................................... 1700/9795
2020-03-17T15:19:52.3567048Z .................................................................................................... 1800/9795
2020-03-17T15:20:04.2659206Z ..........................................................................i......................... 1900/9795
2020-03-17T15:20:11.0915772Z .................................................................................................... 2000/9795
2020-03-17T15:20:19.5172486Z ................................................................iiiii............................... 2100/9795
2020-03-17T15:20:38.1336180Z .................................................................................................... 2300/9795
2020-03-17T15:20:40.4064215Z .................................................................................................... 2400/9795
2020-03-17T15:20:43.4713279Z .................................................................................................... 2500/9795
2020-03-17T15:21:04.4014280Z .................................................................................................... 2600/9795
---
2020-03-17T15:23:52.5375426Z ....................................i...............i............................................... 5000/9795
2020-03-17T15:24:01.8632189Z .................................................................................................... 5100/9795
2020-03-17T15:24:08.5531785Z ...............................................................................i.................... 5200/9795
2020-03-17T15:24:14.3146818Z .................................................................................................... 5300/9795
2020-03-17T15:24:24.8978269Z ............................................................ii.ii........i...i...................... 5400/9795
2020-03-17T15:24:29.3807692Z ...................................................................................................i 5500/9795
2020-03-17T15:24:43.3478201Z .................................................................................................... 5700/9795
2020-03-17T15:24:49.7565469Z .....................................................i.............................................. 5800/9795
2020-03-17T15:24:56.2907185Z .................................................................................................... 5900/9795
2020-03-17T15:25:06.2142318Z .................................................................................................... 6000/9795
2020-03-17T15:25:06.2142318Z .................................................................................................... 6000/9795
2020-03-17T15:25:12.9443143Z ...............................................ii...i..ii...........i............................... 6100/9795
2020-03-17T15:25:33.5752418Z .................................................................................................... 6300/9795
2020-03-17T15:25:40.7435591Z .................................................................................................... 6400/9795
2020-03-17T15:25:40.7435591Z .................................................................................................... 6400/9795
2020-03-17T15:25:49.5869729Z .............................................................................i..ii.................. 6500/9795
2020-03-17T15:26:16.3291937Z .................................................................................................... 6700/9795
2020-03-17T15:26:25.8381789Z ...........................................................................i........................ 6800/9795
2020-03-17T15:26:27.9921481Z .................................................................................................... 6900/9795
2020-03-17T15:26:30.2010801Z .................................................................................................... 7000/9795
---
2020-03-17T15:28:15.8010488Z .................................................................................................... 7800/9795
2020-03-17T15:28:21.3245443Z .................................................................................................... 7900/9795
2020-03-17T15:28:27.6058508Z ............................................................i....................................... 8000/9795
2020-03-17T15:28:37.7307456Z .................................................................................................... 8100/9795
2020-03-17T15:28:43.7061989Z .........iiiiiiiiii..i.............................................................................. 8200/9795
2020-03-17T15:28:57.9889710Z .................................................................................................... 8400/9795
2020-03-17T15:29:06.2409556Z .................................................................................................... 8500/9795
2020-03-17T15:29:20.7045277Z .................................................................................................... 8600/9795
2020-03-17T15:29:27.6323377Z .................................................................................................... 8700/9795
---
2020-03-17T15:31:45.1020710Z  finished in 3.323
2020-03-17T15:31:45.1207981Z Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-17T15:31:45.2761715Z 
2020-03-17T15:31:45.2763365Z running 86 tests
2020-03-17T15:31:46.0053366Z .....19 -         StorageLive(_3);                 // bb0[2]: scope 0 at $DIR/const-promotion-extern-static.rs:9:39: 9:43
2020-03-17T15:31:46.0054471Z 20 -         StorageLive(_4);                 // bb0[3]: scope 0 at $DIR/const-promotion-extern-static.rs:9:40: 9:42
2020-03-17T15:31:46.0055441Z 21 -         StorageLive(_5);                 // bb0[4]: scope 0 at $DIR/const-promotion-extern-static.rs:9:41: 9:42
2020-03-17T15:31:46.0056359Z - -         _5 = const {alloc0+0: &i32};     // bb0[5]: scope 0 at $DIR/const-promotion-extern-static.rs:9:41: 9:42
2020-03-17T15:31:46.0061801Z + -         _5 = const Scalar(alloc0+0): &i32; // bb0[5]: scope 0 at $DIR/const-promotion-extern-static.rs:9:41: 9:42
2020-03-17T15:31:46.0067775Z 23 +         _6 = const BAR::promoted[0];     // bb0[2]: scope 0 at $DIR/const-promotion-extern-static.rs:9:39: 9:43
2020-03-17T15:31:46.0068318Z 24                                            // ty::Const
2020-03-17T15:31:46.0068911Z 25 -                                          // + ty: &i32
2020-03-17T15:31:46.0069123Z 
2020-03-17T15:31:46.2553876Z F.11     bb0: {
2020-03-17T15:31:46.2555732Z 12         StorageLive(_1);                 // bb0[0]: scope 0 at $DIR/const_allocation.rs:8:5: 8:8
2020-03-17T15:31:46.2558345Z 13         StorageLive(_2);                 // bb0[1]: scope 0 at $DIR/const_allocation.rs:8:5: 8:8
2020-03-17T15:31:46.2564428Z -         _2 = const {alloc0+0: &&[(std::option::Option<i32>, &[&str])]}; // bb0[2]: scope 0 at $DIR/const_allocation.rs:8:5: 8:8
2020-03-17T15:31:46.2567712Z +         _2 = const Scalar(alloc0+0): &&[(std::option::Option<i32>, &[&str])]; // bb0[2]: scope 0 at $DIR/const_allocation.rs:8:5: 8:8
2020-03-17T15:31:46.2569316Z 15                                          // ty::Const
2020-03-17T15:31:46.2569813Z 16                                          // + ty: &&[(std::option::Option<i32>, &[&str])]
2020-03-17T15:31:46.2571374Z 17                                          // + val: Value(Scalar(alloc0+0))
2020-03-17T15:31:46.2573578Z 11     bb0: {
2020-03-17T15:31:46.2573578Z 11     bb0: {
2020-03-17T15:31:46.2575812Z 12         StorageLive(_1);                 // bb0[0]: scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
2020-03-17T15:31:46.2577565Z 13         StorageLive(_2);                 // bb0[1]: scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
2020-03-17T15:31:46.2583201Z -         _2 = const {alloc0+0: &&[(std::option::Option<i32>, &[&u8])]}; // bb0[2]: scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
2020-03-17T15:31:46.2584359Z +         _2 = const Scalar(alloc0+0): &&[(std::option::Option<i32>, &[&u8])]; // bb0[2]: scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
2020-03-17T15:31:46.2589005Z F15                                          // ty::Const
2020-03-17T15:31:46.2590922Z 16                                          // + ty: &&[(std::option::Option<i32>, &[&u8])]
2020-03-17T15:31:46.2591493Z 17                                          // + val: Value(Scalar(alloc0+0))
2020-03-17T15:31:46.2592851Z 
2020-03-17T15:31:46.4575073Z F.11     bb0: {
2020-03-17T15:31:46.4575658Z 12         StorageLive(_1);                 // bb0[0]: scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
2020-03-17T15:31:46.4576294Z 13         StorageLive(_2);                 // bb0[1]: scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
2020-03-17T15:31:46.4577316Z -         _2 = const {alloc4+0: &&Packed}; // bb0[2]: scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
2020-03-17T15:31:46.4577994Z +         _2 = const Scalar(alloc4+0): &&Packed; // bb0[2]: scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
2020-03-17T15:31:46.4578554Z 15                                          // ty::Const
2020-03-17T15:31:46.4578928Z 16                                          // + ty: &&Packed
2020-03-17T15:31:46.4579350Z 17                                          // + val: Value(Scalar(alloc4+0))
2020-03-17T15:31:53.1318768Z F...........................................................................
2020-03-17T15:31:53.1323657Z failures:
2020-03-17T15:31:53.1326060Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-17T15:31:53.1326771Z 
2020-03-17T15:31:53.1326771Z 
2020-03-17T15:31:53.1327775Z ---- [mir-opt] mir-opt/const-promotion-extern-static.rs stdout ----
2020-03-17T15:31:53.1328983Z thread '[mir-opt] mir-opt/const-promotion-extern-static.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const-promotion-extern-static/rustc.BAR.PromoteTemps.diff', src/tools/compiletest/src/runtest.rs:3145:25
2020-03-17T15:31:53.1329977Z 
2020-03-17T15:31:53.1333242Z ---- [mir-opt] mir-opt/const_allocation.rs stdout ----
2020-03-17T15:31:53.1333242Z ---- [mir-opt] mir-opt/const_allocation.rs stdout ----
2020-03-17T15:31:53.1334862Z thread '[mir-opt] mir-opt/const_allocation.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation/64bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3145:25
2020-03-17T15:31:53.1335836Z ---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
2020-03-17T15:31:53.1335836Z ---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
2020-03-17T15:31:53.1336977Z thread '[mir-opt] mir-opt/const_allocation2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation2/64bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3145:25
2020-03-17T15:31:53.1337979Z ---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
2020-03-17T15:31:53.1337979Z ---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
2020-03-17T15:31:53.1338985Z thread '[mir-opt] mir-opt/const_allocation3.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation3/64bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3145:25
2020-03-17T15:31:53.1339616Z 
2020-03-17T15:31:53.1339770Z failures:
2020-03-17T15:31:53.1340196Z     [mir-opt] mir-opt/const-promotion-extern-static.rs
2020-03-17T15:31:53.1340664Z     [mir-opt] mir-opt/const_allocation.rs
2020-03-17T15:31:53.1340664Z     [mir-opt] mir-opt/const_allocation.rs
2020-03-17T15:31:53.1341128Z     [mir-opt] mir-opt/const_allocation2.rs
2020-03-17T15:31:53.1341575Z     [mir-opt] mir-opt/const_allocation3.rs
2020-03-17T15:31:53.1341750Z 
2020-03-17T15:31:53.1342261Z test result: FAILED. 82 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out
2020-03-17T15:31:53.1342558Z 
2020-03-17T15:31:53.1343769Z 
2020-03-17T15:31:53.1343996Z 
2020-03-17T15:31:53.1348184Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-17T15:31:53.1351282Z 
2020-03-17T15:31:53.1351380Z 
2020-03-17T15:31:53.1358207Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-17T15:31:53.1358556Z Build completed unsuccessfully in 1:08:27
2020-03-17T15:31:53.1358556Z Build completed unsuccessfully in 1:08:27
2020-03-17T15:31:53.1415584Z == clock drift check ==
2020-03-17T15:31:53.1431359Z   local time: Tue Mar 17 15:31:53 UTC 2020
2020-03-17T15:31:53.3118454Z   network time: Tue, 17 Mar 2020 15:31:53 GMT
2020-03-17T15:31:53.3122066Z == end clock drift check ==
2020-03-17T15:31:55.5257035Z 
2020-03-17T15:31:55.5349748Z ##[error]Bash exited with code '1'.
2020-03-17T15:31:55.5364155Z ##[section]Finishing: Run build
2020-03-17T15:31:55.5412534Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69916/merge to s
2020-03-17T15:31:55.5417753Z Task         : Get sources
2020-03-17T15:31:55.5418142Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T15:31:55.5418650Z Version      : 1.0.0
2020-03-17T15:31:55.5418882Z Author       : Microsoft
2020-03-17T15:31:55.5418882Z Author       : Microsoft
2020-03-17T15:31:55.5419274Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-17T15:31:55.5419700Z ==============================================================================
2020-03-17T15:31:55.9225983Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-17T15:31:55.9275209Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69916/merge to s
2020-03-17T15:31:55.9380053Z Cleaning up task key
2020-03-17T15:31:55.9381432Z Start cleaning up orphan processes.
2020-03-17T15:31:55.9588954Z Terminate orphan process: pid (5295) (python)
2020-03-17T15:31:55.9772243Z ##[section]Finishing: Finalize Job
