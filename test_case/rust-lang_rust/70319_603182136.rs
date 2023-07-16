plain
2020-03-24T10:25:37.6535405Z ========================== Starting Command Output ===========================
2020-03-24T10:25:37.6539777Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/24671191-5955-464f-bc0b-f07b3878fd90.sh
2020-03-24T10:25:37.6540265Z 
2020-03-24T10:25:37.6546379Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-24T10:25:37.6576231Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70319/merge to s
2020-03-24T10:25:37.6579277Z Task         : Get sources
2020-03-24T10:25:37.6579562Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T10:25:37.6579835Z Version      : 1.0.0
2020-03-24T10:25:37.6580039Z Author       : Microsoft
---
2020-03-24T10:25:38.6536321Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-24T10:25:38.6557499Z ##[command]git config gc.auto 0
2020-03-24T10:25:38.6564787Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-24T10:25:38.6571287Z ##[command]git config --get-all http.proxy
2020-03-24T10:25:38.6583927Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70319/merge:refs/remotes/pull/70319/merge
---
2020-03-24T10:32:22.1246509Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-24T10:32:30.6434467Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-24T10:32:36.2854205Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-24T10:32:46.4850037Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-24T10:32:49.2516246Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-24T10:32:50.4333595Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-24T10:33:18.7351627Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-24T10:33:25.8557855Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-24T10:34:05.0561109Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-24T10:50:35.9795087Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-24T10:50:37.0487724Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-24T10:50:45.9883400Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-24T10:50:55.0185096Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-24T10:50:59.4343131Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-24T10:51:00.6801931Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-24T10:51:36.0631305Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-24T10:51:44.1114428Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-24T10:52:35.3333336Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-24T11:11:31.7256004Z .................................................................................................... 1700/9832
2020-03-24T11:11:35.2838992Z .................................................................................................... 1800/9832
2020-03-24T11:11:43.7775887Z .......................................................................................i............ 1900/9832
2020-03-24T11:11:49.6144347Z .................................................................................................... 2000/9832
2020-03-24T11:11:54.7449627Z .............................................................................iiiii.................. 2100/9832
2020-03-24T11:12:12.0927100Z .................................................................................................... 2300/9832
2020-03-24T11:12:13.9987627Z .................................................................................................... 2400/9832
2020-03-24T11:12:16.1902755Z .................................................................................................... 2500/9832
2020-03-24T11:12:24.3868298Z .................................................................................................... 2600/9832
---
2020-03-24T11:14:50.7566506Z ....................................................i...............i............................... 5000/9832
2020-03-24T11:14:57.7828250Z .................................................................................................... 5100/9832
2020-03-24T11:15:04.0081073Z .................................................................................................i.. 5200/9832
2020-03-24T11:15:08.3737808Z .................................................................................................... 5300/9832
2020-03-24T11:15:17.6228679Z ................................................................................ii.ii........i...i.. 5400/9832
2020-03-24T11:15:24.0436784Z ....................i............................................................................... 5600/9832
2020-03-24T11:15:30.6499291Z .........................i.......................................................................... 5700/9832
2020-03-24T11:15:37.5597654Z ..........................................ii....................................i................... 5800/9832
2020-03-24T11:15:44.2149035Z .................................................................................................... 5900/9832
2020-03-24T11:15:44.2149035Z .................................................................................................... 5900/9832
2020-03-24T11:15:49.0815948Z .................................................................................................... 6000/9832
2020-03-24T11:15:57.3363194Z ..........................................................................ii...i..ii...........i.... 6100/9832
2020-03-24T11:16:15.0079611Z .................................................................................................... 6300/9832
2020-03-24T11:16:18.1354687Z .................................................................................................... 6400/9832
2020-03-24T11:16:21.2786117Z .................................................................................................... 6500/9832
2020-03-24T11:16:21.2786117Z .................................................................................................... 6500/9832
2020-03-24T11:16:32.1971186Z ....i..ii........................................................................................... 6600/9832
2020-03-24T11:16:49.9407414Z .................................................................................................... 6800/9832
2020-03-24T11:16:51.8730752Z ...i................................................................................................ 6900/9832
2020-03-24T11:16:53.8276786Z .................................................................................................... 7000/9832
2020-03-24T11:16:56.0579378Z ......................................i............................................................. 7100/9832
---
2020-03-24T11:18:26.7612417Z .................................................................................................... 7800/9832
2020-03-24T11:18:30.6865126Z .................................................................................................... 7900/9832
2020-03-24T11:18:36.2473822Z ..............................................................................................i..... 8000/9832
2020-03-24T11:18:43.0411050Z .................................................................................................... 8100/9832
2020-03-24T11:18:49.2613007Z ...........................................iiiiiiiiii.i............................................. 8200/9832
2020-03-24T11:19:00.9742227Z .................................................................................................... 8400/9832
2020-03-24T11:19:05.3488926Z .................................................................................................... 8500/9832
2020-03-24T11:19:16.7358717Z .................................................................................................... 8600/9832
2020-03-24T11:19:24.3785903Z .................................................................................................... 8700/9832
---
2020-03-24T11:21:21.2230842Z running 85 tests
2020-03-24T11:21:27.6631347Z ..........................................F..........................................
2020-03-24T11:21:27.6634486Z failures:
2020-03-24T11:21:27.6634727Z 
2020-03-24T11:21:27.6637992Z ---- [mir-opt] mir-opt/inline/inline-into-box-place.rs stdout ----
2020-03-24T11:21:27.6638452Z [ERROR compiletest::runtest] Some("bb0: {")
2020-03-24T11:21:27.6639299Z thread '[mir-opt] mir-opt/inline/inline-into-box-place.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2020-03-24T11:21:27.6639703Z Current block: bb0: {
2020-03-24T11:21:27.6640724Z Actual Line: "        ((*_4).0: alloc::raw_vec::RawVec<u32>) = const ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [65535], len: Size { raw: 16 } }, size: Size { raw: 16 }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }: alloc::raw_vec::RawVec::<u32>;"
2020-03-24T11:21:27.6642715Z Expected Line: "  ((*_4).0: alloc::raw_vec::RawVec<u32>) = const alloc::raw_vec::RawVec::<u32>::NEW;"
2020-03-24T11:21:27.6643729Z Test Name: rustc.main.Inline.after.mir
2020-03-24T11:21:27.6644318Z ... (elided)
2020-03-24T11:21:27.6644628Z let mut _0: ();
2020-03-24T11:21:27.6644628Z let mut _0: ();
2020-03-24T11:21:27.6645213Z let _1: std::boxed::Box<std::vec::Vec<u32>> as UserTypeProjection { base: UserType(0), projs: [] };
2020-03-24T11:21:27.6645785Z let mut _2: std::boxed::Box<std::vec::Vec<u32>>;
2020-03-24T11:21:27.6646025Z let mut _3: ();
2020-03-24T11:21:27.6646256Z let mut _4: &mut std::vec::Vec<u32>;
2020-03-24T11:21:27.6646612Z   debug _x => _1;
2020-03-24T11:21:27.6646770Z }
2020-03-24T11:21:27.6646897Z scope 2 {
2020-03-24T11:21:27.6647020Z }
2020-03-24T11:21:27.6647020Z }
2020-03-24T11:21:27.6647142Z bb0: {
2020-03-24T11:21:27.6656269Z   StorageLive(_1);
2020-03-24T11:21:27.6656692Z   StorageLive(_2);
2020-03-24T11:21:27.6657012Z   _2 = Box(std::vec::Vec<u32>);
2020-03-24T11:21:27.6657323Z   _4 = &mut (*_2);
2020-03-24T11:21:27.6657798Z   ((*_4).0: alloc::raw_vec::RawVec<u32>) = const alloc::raw_vec::RawVec::<u32>::NEW;
2020-03-24T11:21:27.6658261Z   ((*_4).1: usize) = const 0usize;
2020-03-24T11:21:27.6658566Z   _1 = move _2;
2020-03-24T11:21:27.6658858Z   StorageDead(_2);
2020-03-24T11:21:27.6659118Z   _0 = ();
2020-03-24T11:21:27.6659851Z   drop(_1) -> [return: bb2, unwind: bb1];
2020-03-24T11:21:27.6660175Z }
2020-03-24T11:21:27.6660429Z bb1 (cleanup): {
2020-03-24T11:21:27.6660709Z   resume;
2020-03-24T11:21:27.6661172Z bb2: {
2020-03-24T11:21:27.6661446Z   StorageDead(_1);
2020-03-24T11:21:27.6661708Z   return;
2020-03-24T11:21:27.6661935Z }
2020-03-24T11:21:27.6661935Z }
2020-03-24T11:21:27.6662186Z Actual:
2020-03-24T11:21:27.6662619Z fn main() -> () {
2020-03-24T11:21:27.6662922Z     let mut _0: ();
2020-03-24T11:21:27.6663419Z     let _1: std::boxed::Box<std::vec::Vec<u32>> as UserTypeProjection { base: UserType(0), projs: [] };
2020-03-24T11:21:27.6663965Z     let mut _2: std::boxed::Box<std::vec::Vec<u32>>;
2020-03-24T11:21:27.6664335Z     let mut _3: ();
2020-03-24T11:21:27.6664693Z     let mut _4: &mut std::vec::Vec<u32>;
2020-03-24T11:21:27.6665312Z         debug _x => _1;
2020-03-24T11:21:27.6665577Z     }
2020-03-24T11:21:27.6665824Z     scope 2 {
2020-03-24T11:21:27.6666083Z     }
2020-03-24T11:21:27.6666083Z     }
2020-03-24T11:21:27.6666332Z     bb0: {
2020-03-24T11:21:27.6666613Z         StorageLive(_1);
2020-03-24T11:21:27.6666926Z         StorageLive(_2);
2020-03-24T11:21:27.6667261Z         _2 = Box(std::vec::Vec<u32>);
2020-03-24T11:21:27.6667595Z         _4 = &mut (*_2);
2020-03-24T11:21:27.6668704Z         ((*_4).0: alloc::raw_vec::RawVec<u32>) = const ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [65535], len: Size { raw: 16 } }, size: Size { raw: 16 }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }: alloc::raw_vec::RawVec::<u32>;
2020-03-24T11:21:27.6669837Z         ((*_4).1: usize) = const 0usize;
2020-03-24T11:21:27.6670187Z         _1 = move _2;
2020-03-24T11:21:27.6670484Z         StorageDead(_2);
2020-03-24T11:21:27.6670782Z         _0 = ();
2020-03-24T11:21:27.6671323Z         drop(_1) -> [return: bb2, unwind: bb1];
2020-03-24T11:21:27.6671659Z     }
2020-03-24T11:21:27.6671944Z     bb1 (cleanup): {
2020-03-24T11:21:27.6672233Z         resume;
2020-03-24T11:21:27.6672742Z     bb2: {
2020-03-24T11:21:27.6673022Z         StorageDead(_1);
2020-03-24T11:21:27.6673420Z         return;
2020-03-24T11:21:27.6673656Z     }
---
2020-03-24T11:21:27.6677385Z 
2020-03-24T11:21:27.6678030Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-24T11:21:27.6678391Z 
2020-03-24T11:21:27.6678587Z 
2020-03-24T11:21:27.6682300Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-24T11:21:27.6685143Z 
2020-03-24T11:21:27.6685344Z 
2020-03-24T11:21:27.6685938Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-24T11:21:27.6686842Z Build completed unsuccessfully in 0:51:50
2020-03-24T11:21:27.6686842Z Build completed unsuccessfully in 0:51:50
2020-03-24T11:21:27.6710572Z == clock drift check ==
2020-03-24T11:21:27.6727415Z   local time: Tue Mar 24 11:21:27 UTC 2020
2020-03-24T11:21:27.9666151Z   network time: Tue, 24 Mar 2020 11:21:27 GMT
2020-03-24T11:21:27.9666701Z == end clock drift check ==
2020-03-24T11:21:30.0747956Z 
2020-03-24T11:21:30.0787820Z ##[error]Bash exited with code '1'.
2020-03-24T11:21:30.0809394Z ##[section]Finishing: Run build
2020-03-24T11:21:30.0861172Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70319/merge to s
2020-03-24T11:21:30.0866043Z Task         : Get sources
2020-03-24T11:21:30.0866350Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T11:21:30.0866791Z Version      : 1.0.0
2020-03-24T11:21:30.0867008Z Author       : Microsoft
2020-03-24T11:21:30.0867008Z Author       : Microsoft
2020-03-24T11:21:30.0867361Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-24T11:21:30.0867750Z ==============================================================================
2020-03-24T11:21:30.3985227Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-24T11:21:30.4028096Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70319/merge to s
2020-03-24T11:21:30.4115218Z Cleaning up task key
2020-03-24T11:21:30.4116512Z Start cleaning up orphan processes.
2020-03-24T11:21:30.4287544Z Terminate orphan process: pid (3862) (python)
2020-03-24T11:21:30.4452494Z ##[section]Finishing: Finalize Job
