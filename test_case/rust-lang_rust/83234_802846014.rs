plain
.................................................................................................... 9300/11700
.................................................................................................... 9400/11700
.................................................................................................... 9500/11700
...........................................i......i................................................. 9600/11700
.........................................................................................iiiiiii..ii 9700/11700
.................................................................................................... 9900/11700
.................................................................................................... 10000/11700
.................................................................................................... 10100/11700
.................................................................................................... 10200/11700
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.104 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.23s

 finished in 2.282 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 22 tests
iiiiiiiiiiii..........

 finished in 0.594 seconds
Build completed successfully in 0:26:54
+ python2.7 ../x.py --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
---
Warning: Skipping "src/test/mir-opt": not a regular file or directory
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 159 tests
....FF...F...F....................i.............................i..............i......F............. 100/159
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
.....FF...F...............i............................F...

---- [mir-opt] mir-opt/const_allocation.rs stdout ----
---- [mir-opt] mir-opt/const_allocation.rs stdout ----
9         StorageLive(_1);                 // scope 0 at $DIR/const_allocation.rs:8:5: 8:8
10         StorageLive(_2);                 // scope 0 at $DIR/const_allocation.rs:8:5: 8:8
11         _2 = const {alloc0: &&[(Option<i32>, &[&str])]}; // scope 0 at $DIR/const_allocation.rs:8:5: 8:8
-                                          // ty::Const
-                                          // + ty: &&[(std::option::Option<i32>, &[&str])]
-                                          // + val: Value(Scalar(alloc0))
15                                          // mir::Constant
16                                          // + span: $DIR/const_allocation.rs:8:5: 8:8
-                                          // + literal: Const { ty: &&[(std::option::Option<i32>, &[&str])], val: Value(Scalar(alloc0)) }
+                                          // + literal: Const { ty: &&[(Option<i32>, &[&str])], val: Value(Scalar(alloc0)) }
18         _1 = (*_2);                      // scope 0 at $DIR/const_allocation.rs:8:5: 8:8
19         StorageDead(_2);                 // scope 0 at $DIR/const_allocation.rs:8:8: 8:9
20         StorageDead(_1);                 // scope 0 at $DIR/const_allocation.rs:8:8: 8:9

thread '[mir-opt] mir-opt/const_allocation.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation.main.ConstProp.after.32bit.mir', src/tools/compiletest/src/runtest.rs:3517:25

---- [mir-opt] mir-opt/array-index-is-temporary.rs stdout ----
---- [mir-opt] mir-opt/array-index-is-temporary.rs stdout ----
39         _5 = foo(move _6) -> bb1;        // scope 4 at $DIR/array-index-is-temporary.rs:16:21: 16:27
40                                          // mir::Constant
41                                          // + span: $DIR/array-index-is-temporary.rs:16:21: 16:24
-                                          // + literal: Const { ty: unsafe fn(*mut usize) -> u32 {foo}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: unsafe fn(*mut usize) -> u32 {foo}, val: Value(Leaf(<ZST>)) }
44 
45     bb1: {


thread '[mir-opt] mir-opt/array-index-is-temporary.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/array_index_is_temporary.main.SimplifyCfg-elaborate-drops.after.32bit.mir', src/tools/compiletest/src/runtest.rs:3517:25
---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
9         StorageLive(_1);                 // scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
10         StorageLive(_2);                 // scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
11         _2 = const {alloc0: &&[(Option<i32>, &[&u8])]}; // scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
-                                          // ty::Const
-                                          // + ty: &&[(std::option::Option<i32>, &[&u8])]
-                                          // + val: Value(Scalar(alloc0))
15                                          // mir::Constant
16                                          // + span: $DIR/const_allocation2.rs:5:5: 5:8
-                                          // + literal: Const { ty: &&[(std::option::Option<i32>, &[&u8])], val: Value(Scalar(alloc0)) }
+                                          // + literal: Const { ty: &&[(Option<i32>, &[&u8])], val: Value(Scalar(alloc0)) }
18         _1 = (*_2);                      // scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
19         StorageDead(_2);                 // scope 0 at $DIR/const_allocation2.rs:5:8: 5:9
20         StorageDead(_1);                 // scope 0 at $DIR/const_allocation2.rs:5:8: 5:9

thread '[mir-opt] mir-opt/const_allocation2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation2.main.ConstProp.after.32bit.mir', src/tools/compiletest/src/runtest.rs:3517:25
---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
9         StorageLive(_1);                 // scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
10         StorageLive(_2);                 // scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
11         _2 = const {alloc0: &&Packed};   // scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
-                                          // ty::Const
-                                          // + ty: &&Packed
-                                          // + val: Value(Scalar(alloc0))
15                                          // mir::Constant
16                                          // + span: $DIR/const_allocation3.rs:5:5: 5:8
17                                          // + literal: Const { ty: &&Packed, val: Value(Scalar(alloc0)) }

thread '[mir-opt] mir-opt/const_allocation3.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation3.main.ConstProp.after.32bit.mir', src/tools/compiletest/src/runtest.rs:3517:25
---- [mir-opt] mir-opt/inline/inline-into-box-place.rs stdout ----
24                                            // mir::Constant
24                                            // mir::Constant
25 -                                          // + span: $DIR/inline-into-box-place.rs:8:33: 8:41
26 -                                          // + user_ty: UserType(1)
- -                                          // + literal: Const { ty: fn() -> std::vec::Vec<u32> {std::vec::Vec::<u32>::new}, val: Value(Scalar(<ZST>)) }
+ -                                          // + literal: Const { ty: fn() -> std::vec::Vec<u32> {std::vec::Vec::<u32>::new}, val: Value(Leaf(<ZST>)) }
28 -     }
30 -     bb1: {


55 -         _3 = alloc::alloc::box_free::<Vec<u32>, std::alloc::Global>(move (_2.0: std::ptr::Unique<std::vec::Vec<u32>>), move (_2.1: std::alloc::Global)) -> bb3; // scope 0 at $DIR/inline-into-box-place.rs:8:42: 8:43
56 -                                          // mir::Constant
57 -                                          // + span: $DIR/inline-into-box-place.rs:8:42: 8:43
- -                                          // + literal: Const { ty: unsafe fn(std::ptr::Unique<std::vec::Vec<u32>>, std::alloc::Global) {alloc::alloc::box_free::<std::vec::Vec<u32>, std::alloc::Global>}, val: Value(Scalar(<ZST>)) }
+ -                                          // + literal: Const { ty: unsafe fn(std::ptr::Unique<std::vec::Vec<u32>>, std::alloc::Global) {alloc::alloc::box_free::<std::vec::Vec<u32>, std::alloc::Global>}, val: Value(Leaf(<ZST>)) }
60   }
61   


thread '[mir-opt] mir-opt/inline/inline-into-box-place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_into_box_place.main.Inline.32bit.diff', src/tools/compiletest/src/runtest.rs:3517:25
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
117           StorageLive(_21);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
118           StorageLive(_22);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
119           _22 = const core::panicking::AssertKind::Eq; // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                                            // ty::Const
-                                            // + ty: core::panicking::AssertKind
-                                            // + val: Value(Scalar(0x00))
123                                            // mir::Constant
124                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
125                                            // + literal: Const { ty: core::panicking::AssertKind, val: Value(Scalar(0x00)) }

136           core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _23, move _25, move _27); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
137                                            // mir::Constant
138                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                                            // + literal: Const { ty: for<'r, 's, 't0> fn(core::panicking::AssertKind, &'r i32, &'s i32, std::option::Option<std::fmt::Arguments<'t0>>) -> ! {core::panicking::assert_failed::<i32, i32>}, val: Value(Scalar(<ZST>)) }
-                                            // ty::Const
-                                            // + ty: core::panicking::AssertKind
-                                            // + val: Value(Scalar(0x00))
+                                            // + literal: Const { ty: for<'r, 's, 't0> fn(core::panicking::AssertKind, &'r i32, &'s i32, std::option::Option<std::fmt::Arguments<'t0>>) -> ! {core::panicking::assert_failed::<i32, i32>}, val: Value(Leaf(<ZST>)) }
143                                            // mir::Constant
144                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
145                                            // + literal: Const { ty: core::panicking::AssertKind, val: Value(Scalar(0x00)) }

thread '[mir-opt] mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.32bit.diff', src/tools/compiletest/src/runtest.rs:3517:25
---- [mir-opt] mir-opt/issue-72181.rs stdout ----
---- [mir-opt] mir-opt/issue-72181.rs stdout ----
25         _1 = std::mem::size_of::<Foo>() -> [return: bb1, unwind: bb3]; // scope 0 at $DIR/issue-72181.rs:24:13: 24:34
26                                          // mir::Constant
27                                          // + span: $DIR/issue-72181.rs:24:13: 24:32
-                                          // + literal: Const { ty: fn() -> usize {std::mem::size_of::<Foo>}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: fn() -> usize {std::mem::size_of::<Foo>}, val: Value(Leaf(<ZST>)) }
30 
31     bb1: {


thread '[mir-opt] mir-opt/issue-72181.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_72181.main.mir_map.0.32bit.mir', src/tools/compiletest/src/runtest.rs:3517:25
---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
23 |
24 fn main() -> () {
24 fn main() -> () {
25     let mut _0: ();                      // return place in scope 0 at $DIR/region-subtyping-basic.rs:16:11: 16:11
-     let mut _1: [usize; Const { ty: usize, val: Value(Scalar(0x00000003)) }]; // in scope 0 at $DIR/region-subtyping-basic.rs:17:9: 17:14
+     let mut _1: [usize; Const { ty: usize, val: Value(Leaf(0x00000003)) }]; // in scope 0 at $DIR/region-subtyping-basic.rs:17:9: 17:14
27     let _3: usize;                       // in scope 0 at $DIR/region-subtyping-basic.rs:18:16: 18:17
28     let mut _4: usize;                   // in scope 0 at $DIR/region-subtyping-basic.rs:18:14: 18:18
29     let mut _5: bool;                    // in scope 0 at $DIR/region-subtyping-basic.rs:18:14: 18:18
45 
46     bb0: {
46     bb0: {
47         StorageLive(_1);                 // bb0[0]: scope 0 at $DIR/region-subtyping-basic.rs:17:9: 17:14
-         _1 = [const Const(Value(Scalar(0x00000001)): usize), const Const(Value(Scalar(0x00000002)): usize), const Const(Value(Scalar(0x00000003)): usize)]; // bb0[1]: scope 0 at $DIR/region-subtyping-basic.rs:17:17: 17:26
+         _1 = [const Const(Value(Leaf(0x00000001)): usize), const Const(Value(Leaf(0x00000002)): usize), const Const(Value(Leaf(0x00000003)): usize)]; // bb0[1]: scope 0 at $DIR/region-subtyping-basic.rs:17:17: 17:26
49         FakeRead(ForLet, _1);            // bb0[2]: scope 0 at $DIR/region-subtyping-basic.rs:17:9: 17:14
50         StorageLive(_2);                 // bb0[3]: scope 1 at $DIR/region-subtyping-basic.rs:18:9: 18:10
51         StorageLive(_3);                 // bb0[4]: scope 1 at $DIR/region-subtyping-basic.rs:18:16: 18:17

-         _3 = const Const(Value(Scalar(0x00000000)): usize); // bb0[5]: scope 1 at $DIR/region-subtyping-basic.rs:18:16: 18:17
+         _3 = const Const(Value(Leaf(0x00000000)): usize); // bb0[5]: scope 1 at $DIR/region-subtyping-basic.rs:18:16: 18:17
53         _4 = Len(_1);                    // bb0[6]: scope 1 at $DIR/region-subtyping-basic.rs:18:14: 18:18
54         _5 = Lt(_3, _4);                 // bb0[7]: scope 1 at $DIR/region-subtyping-basic.rs:18:14: 18:18
55         assert(move _5, "index out of bounds: the length is {} but the index is {}", move _4, _3) -> [success: bb1, unwind: bb7]; // bb0[8]: scope 1 at $DIR/region-subtyping-basic.rs:18:14: 18:18

62         _6 = _2;                         // bb1[3]: scope 2 at $DIR/region-subtyping-basic.rs:19:13: 19:14
63         FakeRead(ForLet, _6);            // bb1[4]: scope 2 at $DIR/region-subtyping-basic.rs:19:9: 19:10
64         StorageLive(_7);                 // bb1[5]: scope 3 at $DIR/region-subtyping-basic.rs:20:8: 20:12
-         _7 = const Const(Value(Scalar(0x01)): bool); // bb1[6]: scope 3 at $DIR/region-subtyping-basic.rs:20:8: 20:12
-         switchInt(move _7) -> [Const(Value(Scalar(0x00)): bool): bb3, otherwise: bb2]; // bb1[7]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
+         _7 = const Const(Value(Leaf(0x01)): bool); // bb1[6]: scope 3 at $DIR/region-subtyping-basic.rs:20:8: 20:12
+         switchInt(move _7) -> [Const(Value(Leaf(0x00)): bool): bb3, otherwise: bb2]; // bb1[7]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
68 
69     bb2: {


70         StorageLive(_8);                 // bb2[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
71         StorageLive(_9);                 // bb2[1]: scope 3 at $DIR/region-subtyping-basic.rs:21:15: 21:17
72         _9 = (*_6);                      // bb2[2]: scope 3 at $DIR/region-subtyping-basic.rs:21:15: 21:17
-         _8 = Const(Value(Scalar(<ZST>)): fn(usize) -> bool {use_x})(move _9) -> [return: bb4, unwind: bb7]; // bb2[3]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
+         _8 = Const(Value(Leaf(<ZST>)): fn(usize) -> bool {use_x})(move _9) -> [return: bb4, unwind: bb7]; // bb2[3]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
74                                          // mir::Constant
75                                          // + span: $DIR/region-subtyping-basic.rs:21:9: 21:14
-                                          // + literal: Const { ty: fn(usize) -> bool {use_x}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: fn(usize) -> bool {use_x}, val: Value(Leaf(<ZST>)) }
78 
79     bb3: {


80         StorageLive(_10);                // bb3[0]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
-         _10 = Const(Value(Scalar(<ZST>)): fn(usize) -> bool {use_x})(const Const(Value(Scalar(0x00000016)): usize)) -> [return: bb5, unwind: bb7]; // bb3[1]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
+         _10 = Const(Value(Leaf(<ZST>)): fn(usize) -> bool {use_x})(const Const(Value(Leaf(0x00000016)): usize)) -> [return: bb5, unwind: bb7]; // bb3[1]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
82                                          // mir::Constant
83                                          // + span: $DIR/region-subtyping-basic.rs:23:9: 23:14
-                                          // + literal: Const { ty: fn(usize) -> bool {use_x}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: fn(usize) -> bool {use_x}, val: Value(Leaf(<ZST>)) }
86 
87     bb4: {


88         StorageDead(_9);                 // bb4[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:17: 21:18
89         StorageDead(_8);                 // bb4[1]: scope 3 at $DIR/region-subtyping-basic.rs:21:18: 21:19
-         _0 = const Const(Value(Scalar(<ZST>)): ()); // bb4[2]: scope 3 at $DIR/region-subtyping-basic.rs:20:13: 22:6
+         _0 = const Const(Value(Leaf(<ZST>)): ()); // bb4[2]: scope 3 at $DIR/region-subtyping-basic.rs:20:13: 22:6
91         goto -> bb6;                     // bb4[3]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
93 

94     bb5: {
94     bb5: {
95         StorageDead(_10);                // bb5[0]: scope 3 at $DIR/region-subtyping-basic.rs:23:18: 23:19
-         _0 = const Const(Value(Scalar(<ZST>)): ()); // bb5[1]: scope 3 at $DIR/region-subtyping-basic.rs:22:12: 24:6
+         _0 = const Const(Value(Leaf(<ZST>)): ()); // bb5[1]: scope 3 at $DIR/region-subtyping-basic.rs:22:12: 24:6
97         goto -> bb6;                     // bb5[2]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
99 


thread '[mir-opt] mir-opt/nll/region-subtyping-basic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/nll/region_subtyping_basic.main.nll.0.32bit.mir', src/tools/compiletest/src/runtest.rs:3517:25
---- [mir-opt] mir-opt/unusual-item-types.rs stdout ----
---- [mir-opt] mir-opt/unusual-item-types.rs stdout ----
34         _3 = <Vec<i32> as Drop>::drop(move _2) -> [return: bb5, unwind: bb4]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
35                                          // mir::Constant
36                                          // + span: $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-                                          // + literal: Const { ty: for<'r> fn(&'r mut std::vec::Vec<i32>) {<std::vec::Vec<i32> as std::ops::Drop>::drop}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: for<'r> fn(&'r mut std::vec::Vec<i32>) {<std::vec::Vec<i32> as std::ops::Drop>::drop}, val: Value(Leaf(<ZST>)) }
39 }
40 


thread '[mir-opt] mir-opt/unusual-item-types.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/unusual_item_types.core.ptr-drop_in_place.Vec_i32_.AddMovesForPackedDrops.before.32bit.mir', src/tools/compiletest/src/runtest.rs:3517:25

failures:
    [mir-opt] mir-opt/array-index-is-temporary.rs
    [mir-opt] mir-opt/const_allocation.rs
---
test result: FAILED. 146 passed; 9 failed; 4 ignored; 0 measured; 0 filtered out; finished in 2.53s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
Build completed unsuccessfully in 0:01:28
