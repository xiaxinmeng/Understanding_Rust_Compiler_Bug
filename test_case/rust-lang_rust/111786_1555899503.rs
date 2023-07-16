plain

running 255 tests
................................................................................i.......  88/255
.........................................................i...........iii................ 176/255
................F.iii.iiiiiF........ii........i................................
failures:

---- [mir-opt] tests/mir-opt/lower_slice_len.rs stdout ----
---- [mir-opt] tests/mir-opt/lower_slice_len.rs stdout ----
thread '[mir-opt] tests/mir-opt/lower_slice_len.rs' panicked at 'the mir dump file for lower_slice_len.bound.LowerSliceLenCalls.before.mir does not exist (requested in /checkout/tests/mir-opt/lower_slice_len.rs)', src/tools/compiletest/src/runtest.rs:3652:17

---- [mir-opt] tests/mir-opt/lower_array_len.rs stdout ----
---- [mir-opt] tests/mir-opt/lower_array_len.rs stdout ----
24           _7 = &(*_2);                     // scope 0 at $DIR/lower_array_len.rs:+1:16: +1:27
25           _6 = move _7 as &[u8] (Pointer(Unsize)); // scope 0 at $DIR/lower_array_len.rs:+1:16: +1:27
26           StorageDead(_7);                 // scope 0 at $DIR/lower_array_len.rs:+1:20: +1:21
- -         _5 = Len((*_6));                 // scope 0 at $DIR/lower_array_len.rs:+1:16: +1:27
- +         _5 = const N;                    // scope 0 at $DIR/lower_array_len.rs:+1:16: +1:27
-           goto -> bb1;                     // scope 0 at $DIR/lower_array_len.rs:+1:16: +1:27
+           _5 = core::slice::<impl [u8]>::len(move _6) -> bb1; // scope 0 at $DIR/lower_array_len.rs:+1:16: +1:27
+                                            // mir::Constant
+                                            // + span: $DIR/lower_array_len.rs:7:22: 7:25
+                                            // + literal: Const { ty: for<'a> fn(&'a [u8]) -> usize {core::slice::<impl [u8]>::len}, val: Value(<ZST>) }
31   
Build completed unsuccessfully in 0:13:06
32       bb1: {


thread '[mir-opt] tests/mir-opt/lower_array_len.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/lower_array_len.array_bound.NormalizeArrayLen.diff', src/tools/compiletest/src/runtest.rs:3639:21

failures:
    [mir-opt] tests/mir-opt/lower_slice_len.rs
    [mir-opt] tests/mir-opt/lower_array_len.rs
