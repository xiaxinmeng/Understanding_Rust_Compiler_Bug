plain
 finished in 0.618 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 168 tests
.......................F.............i...........................................F..i............... 100/168
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
................F..................i................................

---- [mir-opt] mir-opt/const_prop/boxes.rs stdout ----
34       bb1: {
34       bb1: {
35           StorageLive(_7);                 // scope 0 at $DIR/boxes.rs:12:14: 12:22
36           _7 = ShallowInitBox(move _6, i32); // scope 0 at $DIR/boxes.rs:12:14: 12:22
-           (*_7) = const 42_i32;            // scope 0 at $DIR/boxes.rs:12:19: 12:21
+           (*((_7.0: std::ptr::Unique<i32>).0: *const i32)) = const 42_i32; // scope 0 at $DIR/boxes.rs:12:19: 12:21
38           _3 = move _7;                    // scope 0 at $DIR/boxes.rs:12:14: 12:22
39           StorageDead(_7);                 // scope 0 at $DIR/boxes.rs:12:21: 12:22
-           _2 = (*_3);                      // scope 0 at $DIR/boxes.rs:12:13: 12:22
+           _2 = (*((_3.0: std::ptr::Unique<i32>).0: *const i32)); // scope 0 at $DIR/boxes.rs:12:13: 12:22
41           _1 = Add(move _2, const 0_i32);  // scope 0 at $DIR/boxes.rs:12:13: 12:26
42           StorageDead(_2);                 // scope 0 at $DIR/boxes.rs:12:25: 12:26
43           drop(_3) -> [return: bb2, unwind: bb3]; // scope 0 at $DIR/boxes.rs:12:26: 12:27

thread '[mir-opt] mir-opt/const_prop/boxes.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/boxes.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3406:25

---- [mir-opt] mir-opt/inline/inline-into-box-place.rs stdout ----
31       bb1: {
31       bb1: {
32           StorageLive(_5);                 // scope 0 at $DIR/inline-into-box-place.rs:8:29: 8:43
33           _5 = ShallowInitBox(move _4, std::vec::Vec<u32>); // scope 0 at $DIR/inline-into-box-place.rs:8:29: 8:43
- -         (*_5) = Vec::<u32>::new() -> [return: bb2, unwind: bb5]; // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
+ -         (*((_5.0: std::ptr::Unique<std::vec::Vec<u32>>).0: *const std::vec::Vec<u32>)) = Vec::<u32>::new() -> [return: bb2, unwind: bb5]; // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
35 +         StorageLive(_7);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
- +         _7 = &mut (*_5);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
+ +         _7 = &mut (*((_5.0: std::ptr::Unique<std::vec::Vec<u32>>).0: *const std::vec::Vec<u32>)); // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
37 +         ((*_7).0: alloc::raw_vec::RawVec<u32>) = const alloc::raw_vec::RawVec::<u32> { ptr: Unique::<u32> { pointer: {0x4 as *const u32}, _marker: PhantomData::<u32> }, cap: 0_usize, alloc: std::alloc::Global }; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
38                                            // mir::Constant
39 -                                          // + span: $DIR/inline-into-box-place.rs:8:33: 8:41

thread '[mir-opt] mir-opt/inline/inline-into-box-place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_into_box_place.main.Inline.64bit.diff', src/tools/compiletest/src/runtest.rs:3406:25
---- [mir-opt] mir-opt/inline/issue-58867-inline-as-ref-as-mut.rs stdout ----
---- [mir-opt] mir-opt/inline/issue-58867-inline-as-ref-as-mut.rs stdout ----
19         _4 = &mut (*_1);                 // scope 0 at $DIR/issue-58867-inline-as-ref-as-mut.rs:8:5: 8:15
20         StorageLive(_5);                 // scope 1 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
21         StorageLive(_6);                 // scope 1 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
-         _6 = &mut (*(*_4));              // scope 1 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
+         _6 = &mut (*(((*_4).0: std::ptr::Unique<T>).0: *const T)); // scope 1 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
23         _5 = &mut (*_6);                 // scope 1 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
24         _3 = &mut (*_5);                 // scope 1 at $SRC_DIR/alloc/src/boxed.rs:LL:COL
25         StorageDead(_6);                 // scope 1 at $SRC_DIR/alloc/src/boxed.rs:LL:COL

thread '[mir-opt] mir-opt/inline/issue-58867-inline-as-ref-as-mut.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/issue_58867_inline_as_ref_as_mut.b.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3406:25

failures:
    [mir-opt] mir-opt/const_prop/boxes.rs
    [mir-opt] mir-opt/inline/inline-into-box-place.rs
