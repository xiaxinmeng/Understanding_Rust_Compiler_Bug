plain
test [mir-opt] mir-opt/storage_live_dead_in_statics.rs ... ok

failures:

---- [mir-opt] mir-opt/combine_clone_of_primitives.rs stdout ----
49           StorageLive(_10);                // scope 1 at $DIR/combine_clone_of_primitives.rs:8:5: 8:11
50 -         _10 = &(*_3);                    // scope 1 at $DIR/combine_clone_of_primitives.rs:8:5: 8:11
51 -         _9 = &(*_10);                    // scope 1 at $DIR/combine_clone_of_primitives.rs:8:5: 8:11
- -         _8 = <u64 as Clone>::clone(move _9) -> [return: bb2, unwind: bb4]; // scope 1 at $DIR/combine_clone_of_primitives.rs:8:5: 8:11
+ -         _8 = <u64 as Clone>::clone(move _9) -> bb2; // scope 1 at $DIR/combine_clone_of_primitives.rs:8:5: 8:11
53 -                                          // mir::Constant
54 -                                          // + span: $DIR/combine_clone_of_primitives.rs:8:5: 8:11
55 -                                          // + literal: Const { ty: for<'r> fn(&'r u64) -> u64 {<u64 as Clone>::clone}, val: Value(Scalar(<ZST>)) }

66           StorageLive(_13);                // scope 1 at $DIR/combine_clone_of_primitives.rs:9:5: 9:16
67 -         _13 = &(*_4);                    // scope 1 at $DIR/combine_clone_of_primitives.rs:9:5: 9:16
68 -         _12 = &(*_13);                   // scope 1 at $DIR/combine_clone_of_primitives.rs:9:5: 9:16
- -         _11 = <[f32; 3] as Clone>::clone(move _12) -> [return: bb3, unwind: bb4]; // scope 1 at $DIR/combine_clone_of_primitives.rs:9:5: 9:16
+ -         _11 = <[f32; 3] as Clone>::clone(move _12) -> bb3; // scope 1 at $DIR/combine_clone_of_primitives.rs:9:5: 9:16
70 -                                          // mir::Constant
71 -                                          // + span: $DIR/combine_clone_of_primitives.rs:9:5: 9:16
72 -                                          // + literal: Const { ty: for<'r> fn(&'r [f32; 3]) -> [f32; 3] {<[f32; 3] as Clone>::clone}, val: Value(Scalar(<ZST>)) }

91           StorageDead(_3);                 // scope 0 at $DIR/combine_clone_of_primitives.rs:5:14: 5:15
92           StorageDead(_2);                 // scope 0 at $DIR/combine_clone_of_primitives.rs:5:14: 5:15
93           return;                          // scope 0 at $DIR/combine_clone_of_primitives.rs:5:15: 5:15
-   
-   
-       bb4 (cleanup): {
-           drop(_5) -> bb5;                 // scope 1 at $DIR/combine_clone_of_primitives.rs:5:14: 5:15
-   
-   
-       bb5 (cleanup): {
-           resume;                          // scope 0 at $DIR/combine_clone_of_primitives.rs:5:10: 5:15
103   }
104   


thread '[mir-opt] mir-opt/combine_clone_of_primitives.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/combine_clone_of_primitives.{impl#0}-clone.InstCombine.diff', src/tools/compiletest/src/runtest.rs:3393:25


failures:
    [mir-opt] mir-opt/combine_clone_of_primitives.rs
