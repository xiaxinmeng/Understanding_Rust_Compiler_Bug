plain
..............F........ii........i..................................

failures:

---- [mir-opt] tests/mir-opt/lower_intrinsics.rs stdout ----
24           _4 = &raw const (*_1);           // scope 1 at $DIR/lower_intrinsics.rs:+2:55: +2:56
25 -         _3 = option_payload_ptr::<usize>(move _4) -> [return: bb1, unwind unreachable]; // scope 1 at $DIR/lower_intrinsics.rs:+2:18: +2:57
26 -                                          // mir::Constant
- -                                          // + span: $DIR/lower_intrinsics.rs:133:18: 133:54
+ -                                          // + span: $DIR/lower_intrinsics.rs:132:18: 132:54
28 -                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(*const Option<usize>) -> *const usize {option_payload_ptr::<usize>}, val: Value(<ZST>) }
29 +         _3 = &raw const (((*_4) as Some).0: usize); // scope 1 at $DIR/lower_intrinsics.rs:+2:18: +2:57
30 +         goto -> bb1;                     // scope 1 at $DIR/lower_intrinsics.rs:+2:18: +2:57

37           _6 = &raw const (*_2);           // scope 2 at $DIR/lower_intrinsics.rs:+3:55: +3:56
38 -         _5 = option_payload_ptr::<String>(move _6) -> [return: bb2, unwind unreachable]; // scope 2 at $DIR/lower_intrinsics.rs:+3:18: +3:57
39 -                                          // mir::Constant
- -                                          // + span: $DIR/lower_intrinsics.rs:134:18: 134:54
+ -                                          // + span: $DIR/lower_intrinsics.rs:133:18: 133:54
41 -                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(*const Option<String>) -> *const String {option_payload_ptr::<String>}, val: Value(<ZST>) }
42 +         _5 = &raw const (((*_6) as Some).0: std::string::String); // scope 2 at $DIR/lower_intrinsics.rs:+3:18: +3:57
43 +         goto -> bb2;                     // scope 2 at $DIR/lower_intrinsics.rs:+3:18: +3:57

thread '[mir-opt] tests/mir-opt/lower_intrinsics.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/lower_intrinsics.option_payload.LowerIntrinsics.diff', src/tools/compiletest/src/runtest.rs:3553:21


failures:
    [mir-opt] tests/mir-opt/lower_intrinsics.rs
