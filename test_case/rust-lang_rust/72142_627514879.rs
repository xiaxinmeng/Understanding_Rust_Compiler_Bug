plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 2'
Agent machine name: 'fv-az578'
Current agent version: '2.168.2'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200430.2
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200430.2/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.1)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c61d5728-296e-49dd-b514-e0dadb88e011.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72142/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72142/merge:refs/remotes/pull/72142/merge
---
 ---> cb2676f08729
Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Using cache
 ---> df25ce111862
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
 ---> 599b9ac96b27
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Using cache
 ---> 091087e35a36
---
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
.....................................................i.............................................. 1800/10161
.................................................................................................... 1900/10161
.......................................................................i..i......................... 2000/10161
.................................................................................................... 2100/10161
.............................................................iiiii.................................. 2200/10161
.................................................................................................... 2400/10161
.................................................................................................... 2500/10161
.................................................................................................... 2600/10161
.................................................................................................... 2700/10161
---
.................................................................................................... 5200/10161
.................................................................................................... 5300/10161
........................i........................................................................... 5400/10161
.................i.................................................................................. 5500/10161
........................ii.ii........i...i.......................................................... 5600/10161
.........................................................................i.......................... 5800/10161
.................................................................................................... 5900/10161
....................ii.....................................i........................................ 6000/10161
.................................................................................................... 6100/10161
.................................................................................................... 6100/10161
.................................................................................................... 6200/10161
.................................................................................ii...i..ii......... 6300/10161
.................................................................................................... 6500/10161
.................................................................................................... 6600/10161
.................................................................................................... 6700/10161
.................................................................................................... 6700/10161
..............i..ii................................................................................. 6800/10161
.................................................................................................... 7000/10161
....................................................................i............................... 7100/10161
.................................................................................................... 7200/10161
.................................................................................................... 7300/10161
---
.................................................................................................... 8100/10161
.................................................................................................... 8200/10161
...................................................................................i................ 8300/10161
.................................................................................................... 8400/10161
.....................................iiiiii.iiiii.i................................................. 8500/10161
.................................................................................................... 8700/10161
.................................................................................................... 8800/10161
.................................................................................................... 8900/10161
.................................................................................................... 9000/10161
---
Suite("src/test/mir-opt") not skipped for "bootstrap::test::MirOpt" -- not in ["src/tools/tidy"]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 100 tests
FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.FF...F.F.FF...FFFFFFF.FFFFFFFFFFFFF.FF.FFFF.FFF.FFFFFFFFFFFF 100/100
failures:

---- [mir-opt] mir-opt/array-index-is-temporary.rs stdout ----
30                                          // + ty: u32
30                                          // + ty: u32
31                                          // + val: Value(Scalar(0x0000002a))
32                                          // mir::Constant
-                                          // + span: $DIR/array-index-is-temporary.rs:13:18: 13:20
+                                          // + span: Span($DIR/array-index-is-temporary.rs:13:18: 13:20)
34                                          // + literal: Const { ty: u32, val: Value(Scalar(0x0000002a)) }
35                                          // ty::Const
36                                          // + ty: u32

37                                          // + val: Value(Scalar(0x0000002b))
38                                          // mir::Constant
-                                          // + span: $DIR/array-index-is-temporary.rs:13:22: 13:24
+                                          // + span: Span($DIR/array-index-is-temporary.rs:13:22: 13:24)
40                                          // + literal: Const { ty: u32, val: Value(Scalar(0x0000002b)) }
41                                          // ty::Const
42                                          // + ty: u32

43                                          // + val: Value(Scalar(0x0000002c))
44                                          // mir::Constant
-                                          // + span: $DIR/array-index-is-temporary.rs:13:26: 13:28
+                                          // + span: Span($DIR/array-index-is-temporary.rs:13:26: 13:28)
46                                          // + literal: Const { ty: u32, val: Value(Scalar(0x0000002c)) }
47         StorageLive(_2);                 // scope 1 at $DIR/array-index-is-temporary.rs:14:9: 14:14
48         _2 = const 1usize;               // scope 1 at $DIR/array-index-is-temporary.rs:14:17: 14:18
50                                          // + ty: usize
50                                          // + ty: usize
51                                          // + val: Value(Scalar(0x0000000000000001))
52                                          // mir::Constant
-                                          // + span: $DIR/array-index-is-temporary.rs:14:17: 14:18
+                                          // + span: Span($DIR/array-index-is-temporary.rs:14:17: 14:18)
54                                          // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000001)) }
55         StorageLive(_3);                 // scope 2 at $DIR/array-index-is-temporary.rs:15:9: 15:10
56         StorageLive(_4);                 // scope 2 at $DIR/array-index-is-temporary.rs:15:25: 15:31

65                                          // + ty: unsafe fn(*mut usize) -> u32 {foo}
66                                          // + val: Value(Scalar(<ZST>))
67                                          // mir::Constant
-                                          // + span: $DIR/array-index-is-temporary.rs:16:21: 16:24
+                                          // + span: Span($DIR/array-index-is-temporary.rs:16:21: 16:24)
69                                          // + literal: Const { ty: unsafe fn(*mut usize) -> u32 {foo}, val: Value(Scalar(<ZST>)) }
71 


87                                          // + ty: ()
88                                          // + val: Value(Scalar(<ZST>))
89                                          // mir::Constant
-                                          // + span: $DIR/array-index-is-temporary.rs:12:11: 17:2
+                                          // + span: Span($DIR/array-index-is-temporary.rs:12:11: 17:2)
91                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
92         StorageDead(_3);                 // scope 2 at $DIR/array-index-is-temporary.rs:17:1: 17:2
93         StorageDead(_2);                 // scope 1 at $DIR/array-index-is-temporary.rs:17:1: 17:2

thread '[mir-opt] mir-opt/array-index-is-temporary.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/array-index-is-temporary/64bit/rustc.main.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3166:25

---- [mir-opt] mir-opt/address-of.rs stdout ----
133                                          // + ty: i32
133                                          // + ty: i32
134                                          // + val: Value(Scalar(0x00000000))
135                                          // mir::Constant
-                                          // + span: $DIR/address-of.rs:4:15: 4:16
+                                          // + span: Span($DIR/address-of.rs:4:15: 4:16)
137                                          // + literal: Const { ty: i32, val: Value(Scalar(0x00000000)) }
138         _1 = &_2;                        // scope 0 at $DIR/address-of.rs:4:13: 4:21
139         FakeRead(ForLet, _1);            // scope 0 at $DIR/address-of.rs:4:9: 4:10
144                                          // + ty: i32
144                                          // + ty: i32
145                                          // + val: Value(Scalar(0x00000000))
146                                          // mir::Constant
-                                          // + span: $DIR/address-of.rs:5:23: 5:24
+                                          // + span: Span($DIR/address-of.rs:5:23: 5:24)
148                                          // + literal: Const { ty: i32, val: Value(Scalar(0x00000000)) }
149         _3 = &mut _4;                    // scope 1 at $DIR/address-of.rs:5:17: 5:29
150         FakeRead(ForLet, _3);            // scope 1 at $DIR/address-of.rs:5:9: 5:14

303                                          // + ty: ()
304                                          // + val: Value(Scalar(<ZST>))
305                                          // mir::Constant
-                                          // + span: $DIR/address-of.rs:3:26: 37:2
+                                          // + span: Span($DIR/address-of.rs:3:26: 37:2)
307                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
308         StorageDead(_47);                // scope 13 at $DIR/address-of.rs:37:1: 37:2
309         StorageDead(_45);                // scope 12 at $DIR/address-of.rs:37:1: 37:2

thread '[mir-opt] mir-opt/address-of.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/address-of/rustc.address_of_reborrow.SimplifyCfg-initial.after.mir', src/tools/compiletest/src/runtest.rs:3166:25
---- [mir-opt] mir-opt/basic_assignment.rs stdout ----
32                                          // + ty: bool
32                                          // + ty: bool
33                                          // + val: Value(Scalar(0x00))
34                                          // mir::Constant
-                                          // + span: $DIR/basic_assignment.rs:11:20: 11:25
+                                          // + span: Span($DIR/basic_assignment.rs:11:20: 11:25)
36                                          // + literal: Const { ty: bool, val: Value(Scalar(0x00)) }
37         FakeRead(ForLet, _1);            // scope 0 at $DIR/basic_assignment.rs:11:9: 11:17
38         StorageLive(_2);                 // scope 1 at $DIR/basic_assignment.rs:12:9: 12:17

61                                          // + ty: ()
62                                          // + val: Value(Scalar(<ZST>))
63                                          // mir::Constant
-                                          // + span: $DIR/basic_assignment.rs:10:11: 24:2
+                                          // + span: Span($DIR/basic_assignment.rs:10:11: 24:2)
65                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
66         drop(_5) -> [return: bb3, unwind: bb7]; // scope 3 at $DIR/basic_assignment.rs:24:1: 24:2


thread '[mir-opt] mir-opt/basic_assignment.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/basic_assignment/rustc.main.SimplifyCfg-initial.after.mir', src/tools/compiletest/src/runtest.rs:3166:25
---- [mir-opt] mir-opt/box_expr.rs stdout ----
---- [mir-opt] mir-opt/box_expr.rs stdout ----
19                                          // + ty: fn() -> S {S::new}
20                                          // + val: Value(Scalar(<ZST>))
21                                          // mir::Constant
-                                          // + span: $DIR/box_expr.rs:7:17: 7:23
+                                          // + span: Span($DIR/box_expr.rs:7:17: 7:23)
23                                          // + literal: Const { ty: fn() -> S {S::new}, val: Value(Scalar(<ZST>)) }
25 


38                                          // + ty: fn(std::boxed::Box<S>) {std::mem::drop::<std::boxed::Box<S>>}
39                                          // + val: Value(Scalar(<ZST>))
40                                          // mir::Constant
-                                          // + span: $DIR/box_expr.rs:8:5: 8:9
+                                          // + span: Span($DIR/box_expr.rs:8:5: 8:9)
42                                          // + literal: Const { ty: fn(std::boxed::Box<S>) {std::mem::drop::<std::boxed::Box<S>>}, val: Value(Scalar(<ZST>)) }
44 


50                                          // + ty: ()
51                                          // + val: Value(Scalar(<ZST>))
52                                          // mir::Constant
-                                          // + span: $DIR/box_expr.rs:6:11: 9:2
+                                          // + span: Span($DIR/box_expr.rs:6:11: 9:2)
54                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
55         drop(_1) -> bb4;                 // scope 0 at $DIR/box_expr.rs:9:1: 9:2


thread '[mir-opt] mir-opt/box_expr.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/box_expr/rustc.main.ElaborateDrops.before.mir', src/tools/compiletest/src/runtest.rs:3166:25
---- [mir-opt] mir-opt/byte_slice.rs stdout ----
---- [mir-opt] mir-opt/byte_slice.rs stdout ----
18                                          // + ty: &[u8; 3]
19                                          // + val: Value(Scalar(alloc0+0x0))
20                                          // mir::Constant
-                                          // + span: $DIR/byte_slice.rs:5:13: 5:19
+                                          // + span: Span($DIR/byte_slice.rs:5:13: 5:19)
22                                          // + literal: Const { ty: &[u8; 3], val: Value(Scalar(alloc0+0x0)) }
23         StorageLive(_2);                 // scope 1 at $DIR/byte_slice.rs:6:9: 6:10
24         _2 = [const 5u8, const 120u8];   // scope 1 at $DIR/byte_slice.rs:6:13: 6:24
26                                          // + ty: u8
26                                          // + ty: u8
27                                          // + val: Value(Scalar(0x05))
28                                          // mir::Constant
-                                          // + span: $DIR/byte_slice.rs:6:14: 6:17
+                                          // + span: Span($DIR/byte_slice.rs:6:14: 6:17)
30                                          // + literal: Const { ty: u8, val: Value(Scalar(0x05)) }
31                                          // ty::Const
32                                          // + ty: u8

33                                          // + val: Value(Scalar(0x78))
34                                          // mir::Constant
-                                          // + span: $DIR/byte_slice.rs:6:19: 6:23
+                                          // + span: Span($DIR/byte_slice.rs:6:19: 6:23)
36                                          // + literal: Const { ty: u8, val: Value(Scalar(0x78)) }
37         _0 = const ();                   // scope 0 at $DIR/byte_slice.rs:4:11: 7:2
38                                          // ty::Const

39                                          // + ty: ()
40                                          // + val: Value(Scalar(<ZST>))
41                                          // mir::Constant
-                                          // + span: $DIR/byte_slice.rs:4:11: 7:2
+                                          // + span: Span($DIR/byte_slice.rs:4:11: 7:2)
43                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
44         StorageDead(_2);                 // scope 1 at $DIR/byte_slice.rs:7:1: 7:2
45         StorageDead(_1);                 // scope 0 at $DIR/byte_slice.rs:7:1: 7:2

thread '[mir-opt] mir-opt/byte_slice.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/byte_slice/rustc.main.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3166:25
---- [mir-opt] mir-opt/const-promotion-extern-static.rs stdout ----
---- [mir-opt] mir-opt/const-promotion-extern-static.rs stdout ----
24 +                                          // + ty: &[&i32; 1]
25 +                                          // + val: Unevaluated(DefId(0:6 ~ const_promotion_extern_static[317d]::BAR[0]), [], Some(promoted[0]))
26                                            // mir::Constant
- -                                          // + span: $DIR/const-promotion-extern-static.rs:9:33: 9:34
+ -                                          // + span: Span($DIR/const-promotion-extern-static.rs:9:33: 9:34)
28 -                                          // + literal: Const { ty: &i32, val: Value(Scalar(alloc0+0x0)) }
29 -         _4 = &(*_5);                     // scope 0 at $DIR/const-promotion-extern-static.rs:9:32: 9:34
30 -         _3 = [move _4];                  // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35

31 -         _2 = &_3;                        // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
- +                                          // + span: $DIR/const-promotion-extern-static.rs:9:31: 9:35
+ +                                          // + span: Span($DIR/const-promotion-extern-static.rs:9:31: 9:35)
33 +                                          // + literal: Const { ty: &[&i32; 1], val: Unevaluated(DefId(0:6 ~ const_promotion_extern_static[317d]::BAR[0]), [], Some(promoted[0])) }
34 +         _2 = &(*_6);                     // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
35           _1 = move _2 as &[&i32] (Pointer(Unsize)); // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35

38                                            // + ty: for<'r> fn(&'r [&i32]) -> *const &i32 {core::slice::<impl [&i32]>::as_ptr}
39                                            // + val: Value(Scalar(<ZST>))
40                                            // mir::Constant
-                                            // + span: $DIR/const-promotion-extern-static.rs:9:36: 9:42
+                                            // + span: Span($DIR/const-promotion-extern-static.rs:9:36: 9:42)
42                                            // + literal: Const { ty: for<'r> fn(&'r [&i32]) -> *const &i32 {core::slice::<impl [&i32]>::as_ptr}, val: Value(Scalar(<ZST>)) }
44   


thread '[mir-opt] mir-opt/const-promotion-extern-static.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const-promotion-extern-static/rustc.BAR.PromoteTemps.diff', src/tools/compiletest/src/runtest.rs:3166:25
---- [mir-opt] mir-opt/combine_array_len.rs stdout ----
33                                            // + ty: usize
33                                            // + ty: usize
34                                            // + val: Value(Scalar(0x0000000000000000))
35                                            // mir::Constant
-                                            // + span: $DIR/combine_array_len.rs:5:15: 5:16
+                                            // + span: Span($DIR/combine_array_len.rs:5:15: 5:16)
37                                            // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000000)) }
38 -         _4 = Len(_1);                    // scope 0 at $DIR/combine_array_len.rs:5:13: 5:17
39 +         _4 = const 2usize;               // scope 0 at $DIR/combine_array_len.rs:5:13: 5:17
41 +                                          // + ty: usize
41 +                                          // + ty: usize
42 +                                          // + val: Value(Scalar(0x0000000000000002))
43 +                                          // mir::Constant
- +                                          // + span: $DIR/combine_array_len.rs:5:13: 5:17
+ +                                          // + span: Span($DIR/combine_array_len.rs:5:13: 5:17)
45 +                                          // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000002)) }
46           _5 = Lt(_3, _4);                 // scope 0 at $DIR/combine_array_len.rs:5:13: 5:17
47           assert(move _5, "index out of bounds: the len is {} but the index is {}", move _4, _3) -> bb1; // scope 0 at $DIR/combine_array_len.rs:5:13: 5:17
57                                            // + ty: usize
57                                            // + ty: usize
58                                            // + val: Value(Scalar(0x0000000000000001))
59                                            // mir::Constant
-                                            // + span: $DIR/combine_array_len.rs:6:15: 6:16
+                                            // + span: Span($DIR/combine_array_len.rs:6:15: 6:16)
61                                            // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000001)) }
62 -         _8 = Len(_1);                    // scope 1 at $DIR/combine_array_len.rs:6:13: 6:17
63 +         _8 = const 2usize;               // scope 1 at $DIR/combine_array_len.rs:6:13: 6:17
65 +                                          // + ty: usize
65 +                                          // + ty: usize
66 +                                          // + val: Value(Scalar(0x0000000000000002))
67 +                                          // mir::Constant
- +                                          // + span: $DIR/combine_array_len.rs:6:13: 6:17
+ +                                          // + span: Span($DIR/combine_array_len.rs:6:13: 6:17)
69 +                                          // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000002)) }
70           _9 = Lt(_7, _8);                 // scope 1 at $DIR/combine_array_len.rs:6:13: 6:17
71           assert(move _9, "index out of bounds: the len is {} but the index is {}", move _8, _7) -> bb2; // scope 1 at $DIR/combine_array_len.rs:6:13: 6:17

thread '[mir-opt] mir-opt/combine_array_len.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/combine_array_len/64bit/rustc.norm2.InstCombine.diff', src/tools/compiletest/src/runtest.rs:3166:25
---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
13                                          // + ty: &&[(std::option::Option<i32>, &[&u8])]
14                                          // + val: Value(Scalar(alloc0+0x0))
15                                          // mir::Constant
-                                          // + span: $DIR/const_allocation2.rs:5:5: 5:8
+                                          // + span: Span($DIR/const_allocation2.rs:5:5: 5:8)
17                                          // + literal: Const { ty: &&[(std::option::Option<i32>, &[&u8])], val: Value(Scalar(alloc0+0x0)) }
18         _1 = (*_2);                      // scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
19         StorageDead(_2);                 // scope 0 at $DIR/const_allocation2.rs:5:8: 5:9

23                                          // + ty: ()
24                                          // + val: Value(Scalar(<ZST>))
25                                          // mir::Constant
-                                          // + span: $DIR/const_allocation2.rs:4:11: 6:2
+                                          // + span: Span($DIR/const_allocation2.rs:4:11: 6:2)
27                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
28         return;                          // scope 0 at $DIR/const_allocation2.rs:6:2: 6:2


thread '[mir-opt] mir-opt/const_allocation2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation2/64bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3166:25
---- [mir-opt] mir-opt/const_allocation.rs stdout ----
---- [mir-opt] mir-opt/const_allocation.rs stdout ----
13                                          // + ty: &&[(std::option::Option<i32>, &[&str])]
14                                          // + val: Value(Scalar(alloc0+0x0))
15                                          // mir::Constant
-                                          // + span: $DIR/const_allocation.rs:8:5: 8:8
+                                          // + span: Span($DIR/const_allocation.rs:8:5: 8:8)
17                                          // + literal: Const { ty: &&[(std::option::Option<i32>, &[&str])], val: Value(Scalar(alloc0+0x0)) }
18         _1 = (*_2);                      // scope 0 at $DIR/const_allocation.rs:8:5: 8:8
19         StorageDead(_2);                 // scope 0 at $DIR/const_allocation.rs:8:8: 8:9

23                                          // + ty: ()
24                                          // + val: Value(Scalar(<ZST>))
25                                          // mir::Constant
-                                          // + span: $DIR/const_allocation.rs:7:11: 9:2
+                                          // + span: Span($DIR/const_allocation.rs:7:11: 9:2)
27                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
28         return;                          // scope 0 at $DIR/const_allocation.rs:9:2: 9:2


thread '[mir-opt] mir-opt/const_allocation.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation/64bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3166:25
---- [mir-opt] mir-opt/const_prop/aggregate.rs stdout ----
19                                            // + ty: i32
19                                            // + ty: i32
20                                            // + val: Value(Scalar(0x00000000))
21                                            // mir::Constant
-                                            // + span: $DIR/aggregate.rs:5:14: 5:15
+                                            // + span: Span($DIR/aggregate.rs:5:14: 5:15)
23                                            // + literal: Const { ty: i32, val: Value(Scalar(0x00000000)) }
24                                            // ty::Const
25                                            // + ty: i32

26                                            // + val: Value(Scalar(0x00000001))
27                                            // mir::Constant
-                                            // + span: $DIR/aggregate.rs:5:17: 5:18
+                                            // + span: Span($DIR/aggregate.rs:5:17: 5:18)
29                                            // + literal: Const { ty: i32, val: Value(Scalar(0x00000001)) }
30                                            // ty::Const
31                                            // + ty: i32

32                                            // + val: Value(Scalar(0x00000002))
33                                            // mir::Constant
-                                            // + span: $DIR/aggregate.rs:5:20: 5:21
+                                            // + span: Span($DIR/aggregate.rs:5:20: 5:21)
35                                            // + literal: Const { ty: i32, val: Value(Scalar(0x00000002)) }
36 -         _2 = (_3.1: i32);                // scope 0 at $DIR/aggregate.rs:5:13: 5:24
37 -         _1 = Add(move _2, const 0i32);   // scope 0 at $DIR/aggregate.rs:5:13: 5:28

41 -                                          // + val: Value(Scalar(0x00000000))
42 +                                          // + val: Value(Scalar(0x00000001))
43                                            // mir::Constant
- -                                          // + span: $DIR/aggregate.rs:5:27: 5:28
+ -                                          // + span: Span($DIR/aggregate.rs:5:27: 5:28)
45 -                                          // + literal: Const { ty: i32, val: Value(Scalar(0x00000000)) }
- +                                          // + span: $DIR/aggregate.rs:5:13: 5:24
+ +                                          // + span: Span($DIR/aggregate.rs:5:13: 5:24)
47 +                                          // + literal: Const { ty: i32, val: Value(Scalar(0x00000001)) }
48 +         _1 = const 1i32;                 // scope 0 at $DIR/aggregate.rs:5:13: 5:28
49 +                                          // ty::Const
50 +                                          // + ty: i32
50 +                                          // + ty: i32
51 +                                          // + val: Value(Scalar(0x00000001))
52 +                                          // mir::Constant
- +                                          // + span: $DIR/aggregate.rs:5:13: 5:28
+ +                                          // + span: Span($DIR/aggregate.rs:5:13: 5:28)
54 +                                          // + literal: Const { ty: i32, val: Value(Scalar(0x00000001)) }
55           StorageDead(_2);                 // scope 0 at $DIR/aggregate.rs:5:27: 5:28
56           StorageDead(_3);                 // scope 0 at $DIR/aggregate.rs:5:28: 5:29

59                                            // + ty: ()
60                                            // + val: Value(Scalar(<ZST>))
61                                            // mir::Constant
-                                            // + span: $DIR/aggregate.rs:4:11: 6:2
+                                            // + span: Span($DIR/aggregate.rs:4:11: 6:2)
63                                            // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
64           StorageDead(_1);                 // scope 0 at $DIR/aggregate.rs:6:1: 6:2
65           return;                          // scope 0 at $DIR/aggregate.rs:6:2: 6:2

thread '[mir-opt] mir-opt/const_prop/aggregate.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/aggregate/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3166:25
---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
13                                          // + ty: &&Packed
14                                          // + val: Value(Scalar(alloc0+0x0))
15                                          // mir::Constant
-                                          // + span: $DIR/const_allocation3.rs:5:5: 5:8
+                                          // + span: Span($DIR/const_allocation3.rs:5:5: 5:8)
17                                          // + literal: Const { ty: &&Packed, val: Value(Scalar(alloc0+0x0)) }
18         _1 = (*_2);                      // scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
19         StorageDead(_2);                 // scope 0 at $DIR/const_allocation3.rs:5:8: 5:9

23                                          // + ty: ()
24                                          // + val: Value(Scalar(<ZST>))
25                                          // mir::Constant
-                                          // + span: $DIR/const_allocation3.rs:4:11: 6:2
+                                          // + span: Span($DIR/const_allocation3.rs:4:11: 6:2)
27                                          // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
28         return;                          // scope 0 at $DIR/const_allocation3.rs:6:2: 6:2


thread '[mir-opt] mir-opt/const_allocation3.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation3/64bit/rustc.main.ConstProp.after.mir', src/tools/compiletest/src/runtest.rs:3166:25
---- [mir-opt] mir-opt/const_prop/array_index.rs stdout ----
20                                            // + ty: u32
20                                            // + ty: u32
21                                            // + val: Value(Scalar(0x00000000))
22                                            // mir::Constant
-                                            // + span: $DIR/array_index.rs:5:19: 5:20
+                                            // + span: Span($DIR/array_index.rs:5:19: 5:20)
24                                            // + literal: Const { ty: u32, val: Value(Scalar(0x00000000)) }
25                                            // ty::Const
26                                            // + ty: u32

27                                            // + val: Value(Scalar(0x00000001))
28                                            // mir::Constant
-                                            // + span: $DIR/array_index.rs:5:22: 5:23
+                                            // + span: Span($DIR/array_index.rs:5:22: 5:23)
30                                            // + literal: Const { ty: u32, val: Value(Scalar(0x00000001)) }
31                                            // ty::Const
32                                            // + ty: u32

33                                            // + val: Value(Scalar(0x00000002))
34                                            // mir::Constant
-                                            // + span: $DIR/array_index.rs:5:25: 5:26
+                                            // + span: Span($DIR/array_index.rs:5:25: 5:26)
36                                            // + literal: Const { ty: u32, val: Value(Scalar(0x00000002)) }
37                                            // ty::Const
38                                            // + ty: u32

39                                            // + val: Value(Scalar(0x00000003))
40                                            // mir::Constant
-                                            // + span: $DIR/array_index.rs:5:28: 5:29
+                                            // + span: Span($DIR/array_index.rs:5:28: 5:29)
42                                            // + literal: Const { ty: u32, val: Value(Scalar(0x00000003)) }
43           StorageLive(_3);                 // scope 0 at $DIR/array_index.rs:5:31: 5:32
44           _3 = const 2usize;               // scope 0 at $DIR/array_index.rs:5:31: 5:32
46                                            // + ty: usize
46                                            // + ty: usize
47                                            // + val: Value(Scalar(0x0000000000000002))
48                                            // mir::Constant
-                                            // + span: $DIR/array_index.rs:5:31: 5:32
+                                            // + span: Span($DIR/array_index.rs:5:31: 5:32)
50                                            // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000002)) }
51           _4 = const 4usize;               // scope 0 at $DIR/array_index.rs:5:18: 5:33
52                                            // ty::Const
53                                            // + ty: usize
53                                            // + ty: usize
54                                            // + val: Value(Scalar(0x0000000000000004))
55                                            // mir::Constant
-                                            // + span: $DIR/array_index.rs:5:18: 5:33
+                                            // + span: Span($DIR/array_index.rs:5:18: 5:33)
57                                            // + literal: Const { ty: usize, val: Value(Scalar(0x0000000000000004)) }
58 -         _5 = Lt(_3, _4);                 // scope 0 at $DIR/array_index.rs:5:18: 5:33
59 -         assert(move _5, "index out of bounds: the len is {} but the index is {}", move _4, _3) -> bb1; // scope 0 at $DIR/array_index.rs:5:18: 5:33
62 +                                          // + ty: bool
62 +                                          // + ty: bool
63 +                                          // + val: Value(Scalar(0x01))
64 +                                          // mir::Constant
- +                                          // + span: $DIR/array_index.rs:5:18: 5:33
+ +                                          // + span: Span($DIR/array_index.rs:5:18: 5:33)
66 +                                          // + literal: Const { ty: bool, val: Value(Scalar(0x01)) }
67 +         assert(const true, "index out of bounds: the len is {} but the index is {}", move _4, _3) -> bb1; // scope 0 at $DIR/array_index.rs:5:18: 5:33
68 +                                          // ty::Const
69 +                                          // + ty: bool
69 +                                          // + ty: bool
70 +                                          // + val: Value(Scalar(0x01))
71 +                                          // mir::Constant
- +                                          // + span: $DIR/array_index.rs:5:18: 5:33
+ +                                          // + span: Span($DIR/array_index.rs:5:18: 5:33)
73 +                                          // + literal: Const { ty: bool, val: Value(Scalar(0x01)) }
75   

80 +                                          // + ty: u32
80 +                                          // + ty: u32
81 +                                          // + val: Value(Scalar(0x00000002))
82 +                                          // mir::Constant
- +                                          // + span: $DIR/array_index.rs:5:18: 5:33
+ +                                          // + span: Span($DIR/array_index.rs:5:18: 5:33)
84 +                                          // + literal: Const { ty: u32, val: Value(Scalar(0x00000002)) }
85           StorageDead(_3);                 // scope 0 at $DIR/array_index.rs:5:33: 5:34
86           StorageDead(_2);                 // scope 0 at $DIR/array_index.rs:5:33: 5:34

89                                            // + ty: ()
90                                            // + val: Value(Scalar(<ZST>))
91                                            // mir::Constant
-                                            // + span: $DIR/array_index.rs:4:11: 6:2
+                                            // + span: Span($DIR/array_index.rs:4:11: 6:2)
93                                            // + literal: Const { ty: (), val: Value(Scalar(<ZST>)) }
94           StorageDead(_1);                 // scope 0 at $DIR/array_index.rs:6:1: 6:2
95           return;                          // scope 0 at $DIR/array_index.rs:6:2: 6:2

thread '[mir-opt] mir-opt/const_prop/array_index.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/array_index/64bit/rustc.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3166:25
---- [mir-opt] mir-opt/const_prop/bad_op_div_by_zero.rs stdout ----
24                                            // + ty: i32
24                                            // + ty: i32
25                                            // + val: Value(Scalar(0x00000000))
26                                            // mir::Constant
-                                            // + span: $DIR/bad_op_div_by_zero.rs:4:13: 4:14
+                                            // + span: Span($DIR/bad_op_div_by_zero.rs:4:13: 4:14)
28                                            // + literal: Const { ty: i32, val: Value(Scalar(0x00000000)) }
29           StorageLive(_2);                 // scope 1 at $DIR/bad_op_div_by_zero.rs:5:9: 5:11
30           StorageLive(_3);                 // scope 1 at $DIR/bad_op_div_by_zero.rs:5:18: 5:19
35                                            // + ty: i32
35                                            // + ty: i32
36                                            // + val: Value(Scalar(0x00000000))
37                                            // mir::Constant
- -                                          // + span: $DIR/bad_op_div_by_zero.rs:5:14: 5:19
- +                                          // + span: $DIR/bad_op_div_by_zero.rs:5:18: 5:19
+ -                                          // + span: Span($DIR/bad_op_div_by_zero.rs:5:14: 5:19)
+ +                                          // + span: Span($DIR/bad_op_div_by_zero.rs:5:18: 5:19)
40                                            // + literal: Const { ty: i32, val: Value(Scalar(0x00000000)) }
41 +         _4 = const true;                 // scope 1 at $DIR/bad_op_div_by_zero.rs:5:14: 5:19
42 +                                          // ty::Const
43 +                                          // + ty: bool
43 +                                          // + ty: bool
44 +                                          // + val: Value(Scalar(0x01))
45 +                                          // mir::Constant
- +                                          // + span: $DIR/bad_op_div_by_zero.rs:5:14: 5:19
+ +                                          // + span: Span($DIR/bad_op_div_by_zero.rs:5:14: 5:19)
47 +                                          // + literal: Const { ty: bool, val: Value(Scalar(0x01)) }
48           assert(!move _4, "attempt to divide by zero") -> bb1; // scope 1 at $DIR/bad_op_div_by_zero.rs:5:14: 5:19

57 +                                          // + ty: bool
57 +                                          // + ty: bool
58 +                                          // + val: Value(Scalar(0x00))
59                                            // mir::Constant
-                                            // + span: $DIR/bad_op_div_by_zero.rs:5:14: 5:19
+                                            // + span: Span($DIR/bad_op_div_by_zero.rs:5:14: 5:19)
61 -                                          // + literal: Const { ty: i32, val: Value(Scalar(0xffffffff)) }
62 -         _6 = Eq(const 1i32, const std::i32::MIN); // scope 1 at $DIR/bad_op_div_by_zero.rs:5:14: 5:19
63 +                                          // + literal: Const { ty: bool, val: Value(Scalar(0x00)) }
68 +                                          // + ty: bool
---
    [mir-opt] mir-opt/const_allocation2.rs
    [mir-opt] mir-opt/const_allocation3.rs
    [mir-opt] mir-opt/const_prop/aggregate.rs
    [mir-opt] mir-opt/const_prop/array_index.rs
    [mir-opt] mir-opt/const_prop/bad_op_div_by_zero.rs
    [mir-opt] mir-opt/const_prop/bad_op_mod_by_zero.rs
    [mir-opt] mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs
    [mir-opt] mir-opt/const_prop/cast.rs
    [mir-opt] mir-opt/const_prop/checked_add.rs
    [mir-opt] mir-opt/const_prop/const_prop_fails_gracefully.rs
    [mir-opt] mir-opt/const_prop/control-flow-simplification.rs
    [mir-opt] mir-opt/const_prop/control-flow-simplification.rs
    [mir-opt] mir-opt/const_prop/discriminant.rs
    [mir-opt] mir-opt/const_prop/indirect.rs
    [mir-opt] mir-opt/const_prop/issue-66971.rs
    [mir-opt] mir-opt/const_prop/issue-67019.rs
    [mir-opt] mir-opt/const_prop/mutable_variable.rs
    [mir-opt] mir-opt/const_prop/mutable_variable_aggregate.rs
    [mir-opt] mir-opt/const_prop/mutable_variable_aggregate_mut_ref.rs
    [mir-opt] mir-opt/const_prop/mutable_variable_no_prop.rs
    [mir-opt] mir-opt/const_prop/read_immutable_static.rs
    [mir-opt] mir-opt/const_prop/ref_deref.rs
    [mir-opt] mir-opt/const_prop/ref_deref_project.rs
    [mir-opt] mir-opt/const_prop/reify_fn_ptr.rs
---
test result: FAILED. 14 passed; 86 failed; 0 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:00:14
Build completed unsuccessfully in 1:00:14
== clock drift check ==
  local time: Tue May 12 18:27:16 UTC 2020
  network time: thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
Tue, 12 May 2020 18:27:16 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72142/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72142/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3464) (python)
##[section]Finishing: Finalize Job
