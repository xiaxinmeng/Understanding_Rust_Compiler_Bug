plain
2020-04-22T07:24:29.5463976Z ========================== Starting Command Output ===========================
2020-04-22T07:24:29.5468393Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a9ee79a4-0ac3-4a11-9e74-7d740681a92a.sh
2020-04-22T07:24:29.5468801Z 
2020-04-22T07:24:29.5472954Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-22T07:24:29.5492768Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71418/merge to s
2020-04-22T07:24:29.5496071Z Task         : Get sources
2020-04-22T07:24:29.5496321Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T07:24:29.5496583Z Version      : 1.0.0
2020-04-22T07:24:29.5496751Z Author       : Microsoft
---
2020-04-22T07:24:30.7278530Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-22T07:24:30.7285666Z ##[command]git config gc.auto 0
2020-04-22T07:24:30.7289665Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-22T07:24:30.7293311Z ##[command]git config --get-all http.proxy
2020-04-22T07:24:30.7305388Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71418/merge:refs/remotes/pull/71418/merge
---
2020-04-22T07:26:56.3049605Z  ---> 318032b5f0e2
2020-04-22T07:26:56.3050421Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-22T07:26:56.3051066Z  ---> Using cache
2020-04-22T07:26:56.3051414Z  ---> d44a858fd1ce
2020-04-22T07:26:56.3052469Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-22T07:26:56.3053627Z  ---> 58b910f50f5a
2020-04-22T07:26:56.3053852Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-22T07:26:56.3054579Z  ---> Using cache
2020-04-22T07:26:56.3054946Z  ---> ee7702aadba1
---
2020-04-22T07:26:56.3418445Z Looks like docker image is the same as before, not uploading
2020-04-22T07:27:03.1344534Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-22T07:27:03.1683468Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-22T07:27:03.1725430Z == clock drift check ==
2020-04-22T07:27:03.1736284Z   local time: Wed Apr 22 07:27:03 UTC 2020
2020-04-22T07:27:03.4678594Z   network time: Wed, 22 Apr 2020 07:27:03 GMT
2020-04-22T07:27:03.4711148Z Starting sccache server...
2020-04-22T07:27:03.5688808Z configure: processing command line
2020-04-22T07:27:03.5689076Z configure: 
2020-04-22T07:27:03.5690010Z configure: rust.dist-src        := False
---
2020-04-22T07:32:51.6466451Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T07:32:53.2674675Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T07:32:54.9781479Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T07:32:56.6869413Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T07:33:06.1133346Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T07:33:09.1268998Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T07:33:13.9101948Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T07:33:18.3695675Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T07:33:29.4075635Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T07:58:27.4446328Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T07:58:29.2827495Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T07:58:31.3164460Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T07:58:33.4265839Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T07:58:42.7443553Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T07:58:47.4477232Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T07:58:52.6490586Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T07:58:57.4297709Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T07:59:07.2585098Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T08:24:29.4654017Z .................................................................................................... 1600/9912
2020-04-22T08:24:36.0167529Z .................................................................................................... 1700/9912
2020-04-22T08:24:40.3425883Z .................................................................................................... 1800/9912
2020-04-22T08:24:49.2009377Z .................................................................................................... 1900/9912
2020-04-22T08:24:57.5152558Z ..i................................................................................................. 2000/9912
2020-04-22T08:25:04.0396223Z ............................................................................................iiiii... 2100/9912
2020-04-22T08:25:24.6898478Z .................................................................................................... 2300/9912
2020-04-22T08:25:26.9821300Z .................................................................................................... 2400/9912
2020-04-22T08:25:29.2850652Z .................................................................................................... 2500/9912
2020-04-22T08:25:35.1841444Z .................................................................................................... 2600/9912
---
2020-04-22T08:28:46.5758808Z .................................................................................................... 5100/9912
2020-04-22T08:28:54.0614507Z .................................................................................................... 5200/9912
2020-04-22T08:28:59.2365534Z ...............i.................................................................................... 5300/9912
2020-04-22T08:29:09.2863053Z .....i.............................................................................................. 5400/9912
2020-04-22T08:29:14.5423575Z .....ii.ii........i...i............................................................................. 5500/9912
2020-04-22T08:29:22.4459505Z ....................................................i............................................... 5700/9912
2020-04-22T08:29:31.7983060Z ....................................................................................ii.............. 5800/9912
2020-04-22T08:29:38.8503452Z .......................i............................................................................ 5900/9912
2020-04-22T08:29:44.4199881Z .................................................................................................... 6000/9912
2020-04-22T08:29:44.4199881Z .................................................................................................... 6000/9912
2020-04-22T08:29:55.3712789Z .................................................................................................... 6100/9912
2020-04-22T08:30:05.5953567Z .................ii...i..ii...........i............................................................. 6200/9912
2020-04-22T08:30:21.9961698Z .................................................................................................... 6400/9912
2020-04-22T08:30:29.0817674Z .................................................................................................... 6500/9912
2020-04-22T08:30:29.0817674Z .................................................................................................... 6500/9912
2020-04-22T08:30:44.6513531Z ...............................................i..ii................................................ 6600/9912
2020-04-22T08:31:08.1758653Z .................................................................................................... 6800/9912
2020-04-22T08:31:10.5555823Z ................................................i................................................... 6900/9912
2020-04-22T08:31:12.7147012Z .................................................................................................... 7000/9912
2020-04-22T08:31:14.8472400Z ........................................................................................i........... 7100/9912
---
2020-04-22T08:32:56.7442380Z .................................................................................................... 7900/9912
2020-04-22T08:33:03.4030636Z .................................................................................................... 8000/9912
2020-04-22T08:33:09.4621256Z ......................................................i............................................. 8100/9912
2020-04-22T08:33:19.6363129Z .................................................................................................... 8200/9912
2020-04-22T08:33:25.1593028Z ...iiiiii.iiiii.i................................................................................... 8300/9912
2020-04-22T08:33:39.0084884Z .................................................................................................... 8500/9912
2020-04-22T08:33:47.0098207Z .................................................................................................... 8600/9912
2020-04-22T08:34:00.9810673Z .................................................................................................... 8700/9912
2020-04-22T08:34:07.7358285Z .................................................................................................... 8800/9912
---
2020-04-22T08:36:27.8999864Z ................F............................F................FF................F..F......
2020-04-22T08:36:27.9011802Z failures:
2020-04-22T08:36:27.9016474Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-22T08:36:27.9016806Z 
2020-04-22T08:36:27.9017367Z ---- [mir-opt] mir-opt/const_prop/control-flow-simplification.rs stdout ----
2020-04-22T08:36:27.9018288Z 51                                            // + literal: Const { ty: fn(&str) -> ! {std::rt::begin_panic::<&str>}, val: Value(Scalar(<ZST>)) }
2020-04-22T08:36:27.9018906Z 52                                            // ty::Const
2020-04-22T08:36:27.9020625Z 53                                            // + ty: &str
2020-04-22T08:36:27.9022651Z -                                            // + val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 })
2020-04-22T08:36:27.9025185Z +                                            // + val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 })
2020-04-22T08:36:27.9027843Z 55                                            // mir::Constant
2020-04-22T08:36:27.9028256Z 56                                            // + span: $SRC_DIR/libstd/macros.rs:LL:COL
2020-04-22T08:36:27.9031784Z -                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
2020-04-22T08:36:27.9034252Z +                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
2020-04-22T08:36:27.9035936Z 59   }
2020-04-22T08:36:27.9036098Z 60   
2020-04-22T08:36:27.9036213Z 
2020-04-22T08:36:27.9036213Z 
2020-04-22T08:36:27.9037282Z thread '[mir-opt] mir-opt/const_prop/control-flow-simplification.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/control-flow-simplification/rustc.hello.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-22T08:36:27.9038271Z 
2020-04-22T08:36:27.9038730Z ---- [mir-opt] mir-opt/inline/inline-into-box-place.rs stdout ----
2020-04-22T08:36:27.9038730Z ---- [mir-opt] mir-opt/inline/inline-into-box-place.rs stdout ----
2020-04-22T08:36:27.9039525Z 24 -                                          // + ty: fn() -> std::vec::Vec<u32> {std::vec::Vec::<u32>::new}
2020-04-22T08:36:27.9040284Z 25 -                                          // + val: Value(Scalar(<ZST>))
2020-04-22T08:36:27.9040731Z 26 +                                          // + ty: alloc::raw_vec::RawVec<u32>
2020-04-22T08:36:27.9044003Z - +                                          // + val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [65535], len: Size { raw: 16 } }, size: Size { raw: 16 }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } })
2020-04-22T08:36:27.9046416Z + +                                          // + val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [65535], len: Size { raw: 16 } }, size: Size { raw: 16 }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } })
2020-04-22T08:36:27.9047779Z 28                                            // mir::Constant
2020-04-22T08:36:27.9048621Z 29 -                                          // + span: $DIR/inline-into-box-place.rs:8:33: 8:41
2020-04-22T08:36:27.9049338Z 30 -                                          // + user_ty: UserType(1)
2020-04-22T08:36:27.9049577Z 
2020-04-22T08:36:27.9050195Z 41 -         _0 = const ();                   // bb2[2]: scope 0 at $DIR/inline-into-box-place.rs:7:11: 9:2
2020-04-22T08:36:27.9050764Z 42 +                                          // + span: $SRC_DIR/liballoc/vec.rs:LL:COL
2020-04-22T08:36:27.9051193Z 43 +                                          // + user_ty: UserType(0)
2020-04-22T08:36:27.9053265Z - +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [65535], len: Size { raw: 16 } }, size: Size { raw: 16 }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
2020-04-22T08:36:27.9056104Z + +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [65535], len: Size { raw: 16 } }, size: Size { raw: 16 }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
2020-04-22T08:36:27.9057700Z 45 +         ((*_4).1: usize) = const 0usize; // bb0[5]: scope 2 at $SRC_DIR/liballoc/vec.rs:LL:COL
2020-04-22T08:36:27.9058146Z 46                                            // ty::Const
2020-04-22T08:36:27.9058499Z 47 +                                          // + ty: usize
2020-04-22T08:36:27.9058845Z 
2020-04-22T08:36:27.9059903Z thread '[mir-opt] mir-opt/inline/inline-into-box-place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline-into-box-place/64bit/rustc.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-22T08:36:27.9060922Z ---- [mir-opt] mir-opt/no-drop-for-inactive-variant.rs stdout ----
2020-04-22T08:36:27.9060922Z ---- [mir-opt] mir-opt/no-drop-for-inactive-variant.rs stdout ----
2020-04-22T08:36:27.9061808Z 32                                          // + literal: Const { ty: fn(&str) -> ! {std::rt::begin_panic::<&str>}, val: Value(Scalar(<ZST>)) }
2020-04-22T08:36:27.9062404Z 33                                          // ty::Const
2020-04-22T08:36:27.9062724Z 34                                          // + ty: &str
2020-04-22T08:36:27.9064587Z -                                          // + val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 })
2020-04-22T08:36:27.9066967Z +                                          // + val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 })
2020-04-22T08:36:27.9068328Z 36                                          // mir::Constant
2020-04-22T08:36:27.9068741Z 37                                          // + span: $SRC_DIR/libstd/macros.rs:LL:COL
2020-04-22T08:36:27.9070927Z -                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
2020-04-22T08:36:27.9073450Z +                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
2020-04-22T08:36:27.9074934Z 40 
2020-04-22T08:36:27.9075111Z 41     bb3: {
2020-04-22T08:36:27.9075247Z 
2020-04-22T08:36:27.9075247Z 
2020-04-22T08:36:27.9076319Z thread '[mir-opt] mir-opt/no-drop-for-inactive-variant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/no-drop-for-inactive-variant/rustc.unwrap.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-22T08:36:27.9077371Z ---- [mir-opt] mir-opt/no-spurious-drop-after-call.rs stdout ----
2020-04-22T08:36:27.9077371Z ---- [mir-opt] mir-opt/no-spurious-drop-after-call.rs stdout ----
2020-04-22T08:36:27.9078142Z 15         _4 = const "";                   // bb0[4]: scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:22
2020-04-22T08:36:27.9078649Z 16                                          // ty::Const
2020-04-22T08:36:27.9078964Z 17                                          // + ty: &str
2020-04-22T08:36:27.9080504Z -                                          // + val: Value(Slice { data: Allocation { bytes: [], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [], len: Size { raw: 0 } }, size: Size { raw: 0 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 0 })
2020-04-22T08:36:27.9082362Z +                                          // + val: Value(Slice { data: Allocation { bytes: [], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [], len: Size { raw: 0 } }, size: Size { raw: 0 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 0 })
2020-04-22T08:36:27.9083567Z 19                                          // mir::Constant
2020-04-22T08:36:27.9084320Z 20                                          // + span: $DIR/no-spurious-drop-after-call.rs:9:20: 9:22
2020-04-22T08:36:27.9086114Z -                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [], len: Size { raw: 0 } }, size: Size { raw: 0 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 0 }) }
2020-04-22T08:36:27.9088190Z +                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [], len: Size { raw: 0 } }, size: Size { raw: 0 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 0 }) }
2020-04-22T08:36:27.9089833Z 22         _3 = &(*_4);                     // bb0[5]: scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:22
2020-04-22T08:36:27.9090882Z 23         _2 = const <str as std::string::ToString>::to_string(move _3) -> bb2; // bb0[6]: scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:34
2020-04-22T08:36:27.9091466Z 24                                          // ty::Const
2020-04-22T08:36:27.9091679Z 
2020-04-22T08:36:27.9092656Z thread '[mir-opt] mir-opt/no-spurious-drop-after-call.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/no-spurious-drop-after-call/rustc.main.ElaborateDrops.before.mir', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-22T08:36:27.9093677Z ---- [mir-opt] mir-opt/storage_live_dead_in_statics.rs stdout ----
2020-04-22T08:36:27.9093677Z ---- [mir-opt] mir-opt/storage_live_dead_in_statics.rs stdout ----
2020-04-22T08:36:27.9094834Z 653         _2 = Foo { tup: const "hi", data: move _3 }; // bb0[94]: scope 0 at $DIR/storage_live_dead_in_statics.rs:5:29: 23:2
2020-04-22T08:36:27.9095422Z 654                                          // ty::Const
2020-04-22T08:36:27.9095745Z 655                                          // + ty: &str
2020-04-22T08:36:27.9097442Z -                                          // + val: Value(Slice { data: Allocation { bytes: [104, 105], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [3], len: Size { raw: 2 } }, size: Size { raw: 2 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 2 })
2020-04-22T08:36:27.9101304Z +                                          // + val: Value(Slice { data: Allocation { bytes: [104, 105], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [3], len: Size { raw: 2 } }, size: Size { raw: 2 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 2 })
2020-04-22T08:36:27.9102474Z 657                                          // mir::Constant
2020-04-22T08:36:27.9105163Z 658                                          // + span: $DIR/storage_live_dead_in_statics.rs:6:10: 6:14
2020-04-22T08:36:27.9107424Z -                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [104, 105], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [3], len: Size { raw: 2 } }, size: Size { raw: 2 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 2 }) }
2020-04-22T08:36:27.9110105Z +                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [104, 105], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [3], len: Size { raw: 2 } }, size: Size { raw: 2 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 2 }) }
2020-04-22T08:36:27.9111524Z 660         _1 = &_2;                        // bb0[95]: scope 0 at $DIR/storage_live_dead_in_statics.rs:5:28: 23:2
2020-04-22T08:36:27.9112354Z 661         _0 = &(*_1);                     // bb0[96]: scope 0 at $DIR/storage_live_dead_in_statics.rs:5:28: 23:2
2020-04-22T08:36:27.9113574Z 662         StorageDead(_5);                 // bb0[97]: scope 0 at $DIR/storage_live_dead_in_statics.rs:23:1: 23:2
2020-04-22T08:36:27.9114596Z 
2020-04-22T08:36:27.9115724Z thread '[mir-opt] mir-opt/storage_live_dead_in_statics.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/storage_live_dead_in_statics/rustc.XXX.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-22T08:36:27.9116744Z ---- [mir-opt] mir-opt/uninhabited_enum_branching.rs stdout ----
2020-04-22T08:36:27.9116744Z ---- [mir-opt] mir-opt/uninhabited_enum_branching.rs stdout ----
2020-04-22T08:36:27.9118509Z 27           _5 = const "C";                  // bb1[1]: scope 0 at $DIR/uninhabited_enum_branching.rs:23:21: 23:24
2020-04-22T08:36:27.9119025Z 28                                            // ty::Const
2020-04-22T08:36:27.9119384Z 29                                            // + ty: &str
2020-04-22T08:36:27.9121128Z -                                            // + val: Value(Slice { data: Allocation { bytes: [67], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 })
2020-04-22T08:36:27.9123273Z +                                            // + val: Value(Slice { data: Allocation { bytes: [67], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 })
2020-04-22T08:36:27.9124403Z 31                                            // mir::Constant
2020-04-22T08:36:27.9124883Z 32                                            // + span: $DIR/uninhabited_enum_branching.rs:23:21: 23:24
2020-04-22T08:36:27.9126904Z -                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [67], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
2020-04-22T08:36:27.9130222Z +                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [67], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
2020-04-22T08:36:27.9132086Z 34           _1 = &(*_5);                     // bb1[2]: scope 0 at $DIR/uninhabited_enum_branching.rs:23:21: 23:24
2020-04-22T08:36:27.9132768Z 35           StorageDead(_5);                 // bb1[3]: scope 0 at $DIR/uninhabited_enum_branching.rs:23:23: 23:24
2020-04-22T08:36:27.9133830Z 36           goto -> bb4;                     // bb1[4]: scope 0 at $DIR/uninhabited_enum_branching.rs:20:5: 24:6
2020-04-22T08:36:27.9134198Z 
2020-04-22T08:36:27.9134594Z 40           _1 = const "A(Empty)";           // bb2[0]: scope 0 at $DIR/uninhabited_enum_branching.rs:21:24: 21:34
2020-04-22T08:36:27.9141279Z 41                                            // ty::Const
2020-04-22T08:36:27.9141610Z 42                                            // + ty: &str
2020-04-22T08:36:27.9148872Z -                                            // + val: Value(Slice { data: Allocation { bytes: [65, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 })
2020-04-22T08:36:27.9151054Z +                                            // + val: Value(Slice { data: Allocation { bytes: [65, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 })
2020-04-22T08:36:27.9152474Z 44                                            // mir::Constant
2020-04-22T08:36:27.9152943Z 45                                            // + span: $DIR/uninhabited_enum_branching.rs:21:24: 21:34
2020-04-22T08:36:27.9155170Z -                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [65, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 }) }
2020-04-22T08:36:27.9158067Z +                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [65, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 }) }
2020-04-22T08:36:27.9159927Z 47           goto -> bb4;                     // bb2[1]: scope 0 at $DIR/uninhabited_enum_branching.rs:20:5: 24:6
2020-04-22T08:36:27.9160490Z 49   
2020-04-22T08:36:27.9160605Z 
2020-04-22T08:36:27.9160605Z 
2020-04-22T08:36:27.9160994Z 52           _4 = const "B(Empty)";           // bb3[1]: scope 0 at $DIR/uninhabited_enum_branching.rs:22:24: 22:34
2020-04-22T08:36:27.9161551Z 53                                            // ty::Const
2020-04-22T08:36:27.9161881Z 54                                            // + ty: &str
2020-04-22T08:36:27.9163793Z -                                            // + val: Value(Slice { data: Allocation { bytes: [66, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 })
2020-04-22T08:36:27.9166307Z +                                            // + val: Value(Slice { data: Allocation { bytes: [66, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 })
2020-04-22T08:36:27.9167785Z 56                                            // mir::Constant
2020-04-22T08:36:27.9168248Z 57                                            // + span: $DIR/uninhabited_enum_branching.rs:22:24: 22:34
2020-04-22T08:36:27.9170358Z -                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [66, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 }) }
2020-04-22T08:36:27.9172652Z +                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [66, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 }) }
2020-04-22T08:36:27.9174789Z 59           _1 = &(*_4);                     // bb3[2]: scope 0 at $DIR/uninhabited_enum_branching.rs:22:24: 22:34
2020-04-22T08:36:27.9175448Z 60           StorageDead(_4);                 // bb3[3]: scope 0 at $DIR/uninhabited_enum_branching.rs:22:33: 22:34
2020-04-22T08:36:27.9176485Z 61           goto -> bb4;                     // bb3[4]: scope 0 at $DIR/uninhabited_enum_branching.rs:20:5: 24:6
2020-04-22T08:36:27.9176852Z 
2020-04-22T08:36:27.9177247Z 76           _9 = const "E";                  // bb5[1]: scope 0 at $DIR/uninhabited_enum_branching.rs:28:21: 28:24
2020-04-22T08:36:27.9177905Z 77                                            // ty::Const
2020-04-22T08:36:27.9178227Z 78                                            // + ty: &str
2020-04-22T08:36:27.9179874Z -                                            // + val: Value(Slice { data: Allocation { bytes: [69], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 })
2020-04-22T08:36:27.9181776Z +                                            // + val: Value(Slice { data: Allocation { bytes: [69], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 })
2020-04-22T08:36:27.9182939Z 80                                            // mir::Constant
2020-04-22T08:36:27.9183403Z 81                                            // + span: $DIR/uninhabited_enum_branching.rs:28:21: 28:24
2020-04-22T08:36:27.9186577Z -                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [69], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
2020-04-22T08:36:27.9188648Z +                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [69], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
2020-04-22T08:36:27.9190175Z 83           _6 = &(*_9);                     // bb5[2]: scope 0 at $DIR/uninhabited_enum_branching.rs:28:21: 28:24
2020-04-22T08:36:27.9190844Z 84           StorageDead(_9);                 // bb5[3]: scope 0 at $DIR/uninhabited_enum_branching.rs:28:23: 28:24
2020-04-22T08:36:27.9191834Z 85           goto -> bb7;                     // bb5[4]: scope 0 at $DIR/uninhabited_enum_branching.rs:26:5: 29:6
2020-04-22T08:36:27.9192198Z 
2020-04-22T08:36:27.9192576Z 89           _6 = const "D";                  // bb6[0]: scope 0 at $DIR/uninhabited_enum_branching.rs:27:21: 27:24
2020-04-22T08:36:27.9193080Z 90                                            // ty::Const
2020-04-22T08:36:27.9193403Z 91                                            // + ty: &str
2020-04-22T08:36:27.9194979Z -                                            // + val: Value(Slice { data: Allocation { bytes: [68], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 })
2020-04-22T08:36:27.9196898Z +                                            // + val: Value(Slice { data: Allocation { bytes: [68], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 })
2020-04-22T08:36:27.9198390Z 93                                            // mir::Constant
2020-04-22T08:36:27.9198854Z 94                                            // + span: $DIR/uninhabited_enum_branching.rs:27:21: 27:24
2020-04-22T08:36:27.9201019Z -                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [68], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
2020-04-22T08:36:27.9203401Z +                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [68], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
2020-04-22T08:36:27.9205572Z 96           goto -> bb7;                     // bb6[1]: scope 0 at $DIR/uninhabited_enum_branching.rs:26:5: 29:6
2020-04-22T08:36:27.9206113Z 98   
2020-04-22T08:36:27.9206247Z 
2020-04-22T08:36:27.9206247Z 
2020-04-22T08:36:27.9207247Z thread '[mir-opt] mir-opt/uninhabited_enum_branching.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/uninhabited_enum_branching/rustc.main.UninhabitedEnumBranching.diff', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-22T08:36:27.9207932Z 
2020-04-22T08:36:27.9208076Z failures:
2020-04-22T08:36:27.9208538Z     [mir-opt] mir-opt/const_prop/control-flow-simplification.rs
2020-04-22T08:36:27.9209064Z     [mir-opt] mir-opt/inline/inline-into-box-place.rs
---
2020-04-22T08:36:27.9211804Z test result: FAILED. 84 passed; 6 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-22T08:36:27.9212083Z 
2020-04-22T08:36:27.9212258Z 
2020-04-22T08:36:27.9212361Z 
2020-04-22T08:36:27.9217112Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-22T08:36:27.9219888Z 
2020-04-22T08:36:27.9219996Z 
2020-04-22T08:36:27.9220667Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-22T08:36:27.9221056Z Build completed unsuccessfully in 1:07:37
2020-04-22T08:36:27.9221056Z Build completed unsuccessfully in 1:07:37
2020-04-22T08:36:27.9221317Z == clock drift check ==
2020-04-22T08:36:27.9221591Z   local time: Wed Apr 22 08:36:27 UTC 2020
2020-04-22T08:36:28.2105174Z   network time: Wed, 22 Apr 2020 08:36:28 GMT
2020-04-22T08:36:30.4355933Z 
2020-04-22T08:36:30.4355933Z 
2020-04-22T08:36:30.4431076Z ##[error]Bash exited with code '1'.
2020-04-22T08:36:30.4443946Z ##[section]Finishing: Run build
2020-04-22T08:36:30.4517706Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71418/merge to s
2020-04-22T08:36:30.4524506Z Task         : Get sources
2020-04-22T08:36:30.4524836Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T08:36:30.4525123Z Version      : 1.0.0
2020-04-22T08:36:30.4525515Z Author       : Microsoft
2020-04-22T08:36:30.4525515Z Author       : Microsoft
2020-04-22T08:36:30.4525859Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-22T08:36:30.4526227Z ==============================================================================
2020-04-22T08:36:30.8419346Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-22T08:36:30.8462350Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71418/merge to s
2020-04-22T08:36:30.8571980Z Cleaning up task key
2020-04-22T08:36:30.8573223Z Start cleaning up orphan processes.
2020-04-22T08:36:30.8809043Z Terminate orphan process: pid (4791) (python)
2020-04-22T08:36:30.9088851Z ##[section]Finishing: Finalize Job
