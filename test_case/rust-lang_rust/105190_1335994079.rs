plain
    Finished release [optimized] target(s) in 0.21s
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 210 tests
i...........................F....F.......F...F.....i...F....F...F....................... 88/210
.i................................
failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu


---- [mir-opt] src/test/mir-opt/const_prop/array_index.rs stdout ----
20           _3 = const 2_usize;              // scope 0 at $DIR/array_index.rs:+1:31: +1:32
21 -         _4 = Len(_2);                    // scope 0 at $DIR/array_index.rs:+1:18: +1:33
22 -         _5 = Lt(_3, _4);                 // scope 0 at $DIR/array_index.rs:+1:18: +1:33
- -         assert(move _5, "index out of bounds: the length is {} but the index is {}", move _4, _3) -> bb1; // scope 0 at $DIR/array_index.rs:+1:18: +1:33
+ -         assert(_5, "index out of bounds: the length is {} but the index is {}", _4, _3) -> bb1; // scope 0 at $DIR/array_index.rs:+1:18: +1:33
24 +         _4 = const 4_usize;              // scope 0 at $DIR/array_index.rs:+1:18: +1:33
25 +         _5 = const true;                 // scope 0 at $DIR/array_index.rs:+1:18: +1:33
- +         assert(const true, "index out of bounds: the length is {} but the index is {}", move _4, _3) -> bb1; // scope 0 at $DIR/array_index.rs:+1:18: +1:33
+ +         assert(const true, "index out of bounds: the length is {} but the index is {}", _4, _3) -> bb1; // scope 0 at $DIR/array_index.rs:+1:18: +1:33
28   
29       bb1: {


thread '[mir-opt] src/test/mir-opt/const_prop/array_index.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/array_index.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3451:21

---- [mir-opt] src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs stdout ----
39           _6 = const 3_usize;              // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:23: +3:24
40           _7 = Len((*_1));                 // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
41 -         _8 = Lt(_6, _7);                 // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
- -         assert(move _8, "index out of bounds: the length is {} but the index is {}", move _7, _6) -> bb1; // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
+ -         assert(_8, "index out of bounds: the length is {} but the index is {}", _7, _6) -> bb1; // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
43 +         _8 = Lt(const 3_usize, _7);      // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
- +         assert(move _8, "index out of bounds: the length is {} but the index is {}", move _7, const 3_usize) -> bb1; // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
+ +         assert(_8, "index out of bounds: the length is {} but the index is {}", _7, const 3_usize) -> bb1; // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:18: +3:25
46   
47       bb1: {


thread '[mir-opt] src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/const_prop/discriminant.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/discriminant.rs stdout ----
21           ((_3 as Some).0: bool) = const true; // scope 2 at $DIR/discriminant.rs:+1:34: +1:44
22           discriminant(_3) = 1;            // scope 2 at $DIR/discriminant.rs:+1:34: +1:44
23 -         _4 = discriminant(_3);           // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
- -         switchInt(move _4) -> [1_isize: bb1, otherwise: bb3]; // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
+ -         switchInt(_4) -> [1_isize: bb1, otherwise: bb3]; // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
25 +         _4 = const 1_isize;              // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
26 +         switchInt(const 1_isize) -> [1_isize: bb1, otherwise: bb3]; // scope 2 at $DIR/discriminant.rs:+1:21: +1:31

41       }
42   
43       bb4: {
43       bb4: {
-           _1 = Add(move _2, const 0_i32);  // scope 0 at $DIR/discriminant.rs:+1:13: +1:68
+           _1 = Add(_2, const 0_i32);       // scope 0 at $DIR/discriminant.rs:+1:13: +1:68
45           StorageDead(_2);                 // scope 0 at $DIR/discriminant.rs:+1:67: +1:68
46           StorageDead(_3);                 // scope 0 at $DIR/discriminant.rs:+1:68: +1:69
47           _0 = const ();                   // scope 0 at $DIR/discriminant.rs:+0:11: +2:2

thread '[mir-opt] src/test/mir-opt/const_prop/discriminant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/discriminant.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/const_prop/large_array_index.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/large_array_index.rs stdout ----
20           _3 = const 2_usize;              // scope 0 at $DIR/large_array_index.rs:+2:30: +2:31
21           _4 = const 5000_usize;           // scope 0 at $DIR/large_array_index.rs:+2:17: +2:32
22 -         _5 = Lt(_3, _4);                 // scope 0 at $DIR/large_array_index.rs:+2:17: +2:32
- -         assert(move _5, "index out of bounds: the length is {} but the index is {}", move _4, _3) -> bb1; // scope 0 at $DIR/large_array_index.rs:+2:17: +2:32
+ -         assert(_5, "index out of bounds: the length is {} but the index is {}", _4, _3) -> bb1; // scope 0 at $DIR/large_array_index.rs:+2:17: +2:32
24 +         _5 = const true;                 // scope 0 at $DIR/large_array_index.rs:+2:17: +2:32
25 +         assert(const true, "index out of bounds: the length is {} but the index is {}", const 5000_usize, const 2_usize) -> bb1; // scope 0 at $DIR/large_array_index.rs:+2:17: +2:32


thread '[mir-opt] src/test/mir-opt/const_prop/large_array_index.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/large_array_index.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/const_prop/optimizes_into_variable.rs stdout ----
27       bb0: {
27       bb0: {
28           StorageLive(_1);                 // scope 0 at $DIR/optimizes_into_variable.rs:+1:9: +1:10
29           _2 = CheckedAdd(const 2_i32, const 2_i32); // scope 0 at $DIR/optimizes_into_variable.rs:+1:13: +1:18
-           assert(!move (_2.1: bool), "attempt to compute `{} + {}`, which would overflow", const 2_i32, const 2_i32) -> bb1; // scope 0 at $DIR/optimizes_into_variable.rs:+1:13: +1:18
+           assert(!(_2.1: bool), "attempt to compute `{} + {}`, which would overflow", const 2_i32, const 2_i32) -> bb1; // scope 0 at $DIR/optimizes_into_variable.rs:+1:13: +1:18
32   
33       bb1: {


-           _1 = move (_2.0: i32);           // scope 0 at $DIR/optimizes_into_variable.rs:+1:13: +1:18
+           _1 = (_2.0: i32);                // scope 0 at $DIR/optimizes_into_variable.rs:+1:13: +1:18
35           StorageLive(_3);                 // scope 1 at $DIR/optimizes_into_variable.rs:+2:9: +2:10
36           StorageLive(_4);                 // scope 1 at $DIR/optimizes_into_variable.rs:+2:13: +2:31
37           _4 = [const 0_i32, const 1_i32, const 2_i32, const 3_i32, const 4_i32, const 5_i32]; // scope 1 at $DIR/optimizes_into_variable.rs:+2:13: +2:31

39           _5 = const 3_usize;              // scope 1 at $DIR/optimizes_into_variable.rs:+2:32: +2:33
40           _6 = Len(_4);                    // scope 1 at $DIR/optimizes_into_variable.rs:+2:13: +2:34
41           _7 = Lt(_5, _6);                 // scope 1 at $DIR/optimizes_into_variable.rs:+2:13: +2:34
-           assert(move _7, "index out of bounds: the length is {} but the index is {}", move _6, _5) -> bb2; // scope 1 at $DIR/optimizes_into_variable.rs:+2:13: +2:34
+           assert(_7, "index out of bounds: the length is {} but the index is {}", _6, _5) -> bb2; // scope 1 at $DIR/optimizes_into_variable.rs:+2:13: +2:34
44   
45       bb2: {


thread '[mir-opt] src/test/mir-opt/const_prop/optimizes_into_variable.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/optimizes_into_variable.main.ScalarReplacementOfAggregates.32bit.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/const_prop/repeat.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/repeat.rs stdout ----
22           _4 = const 2_usize;              // scope 0 at $DIR/repeat.rs:+1:26: +1:27
23           _5 = const 8_usize;              // scope 0 at $DIR/repeat.rs:+1:18: +1:28
24 -         _6 = Lt(_4, _5);                 // scope 0 at $DIR/repeat.rs:+1:18: +1:28
- -         assert(move _6, "index out of bounds: the length is {} but the index is {}", move _5, _4) -> bb1; // scope 0 at $DIR/repeat.rs:+1:18: +1:28
+ -         assert(_6, "index out of bounds: the length is {} but the index is {}", _5, _4) -> bb1; // scope 0 at $DIR/repeat.rs:+1:18: +1:28
26 +         _6 = const true;                 // scope 0 at $DIR/repeat.rs:+1:18: +1:28
27 +         assert(const true, "index out of bounds: the length is {} but the index is {}", const 8_usize, const 2_usize) -> bb1; // scope 0 at $DIR/repeat.rs:+1:18: +1:28

29   
30       bb1: {
30       bb1: {
31 -         _2 = _3[_4];                     // scope 0 at $DIR/repeat.rs:+1:18: +1:28
- -         _1 = Add(move _2, const 0_u32);  // scope 0 at $DIR/repeat.rs:+1:18: +1:32
+ -         _1 = Add(_2, const 0_u32);       // scope 0 at $DIR/repeat.rs:+1:18: +1:32
33 +         _2 = const 42_u32;               // scope 0 at $DIR/repeat.rs:+1:18: +1:28
34 +         _1 = const 42_u32;               // scope 0 at $DIR/repeat.rs:+1:18: +1:32
35           StorageDead(_2);                 // scope 0 at $DIR/repeat.rs:+1:31: +1:32

thread '[mir-opt] src/test/mir-opt/const_prop/repeat.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/repeat.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/const_prop/slice_len.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/slice_len.rs stdout ----
34           _7 = const 3_usize;              // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
35           StorageDead(_10);                // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
36 -         _8 = Lt(_6, _7);                 // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
- -         assert(move _8, "index out of bounds: the length is {} but the index is {}", move _7, _6) -> bb1; // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
+ -         assert(_8, "index out of bounds: the length is {} but the index is {}", _7, _6) -> bb1; // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
38 +         _8 = const true;                 // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
39 +         assert(const true, "index out of bounds: the length is {} but the index is {}", const 3_usize, const 1_usize) -> bb1; // scope 0 at $DIR/slice_len.rs:+1:5: +1:33


thread '[mir-opt] src/test/mir-opt/const_prop/slice_len.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/slice_len.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3451:21

failures:
    [mir-opt] src/test/mir-opt/const_prop/array_index.rs
    [mir-opt] src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs
