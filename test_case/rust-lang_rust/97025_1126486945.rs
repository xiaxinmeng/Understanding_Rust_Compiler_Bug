plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 174 tests
......................................i................................................. 88/174
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..i...F..............................................i................................

---- [mir-opt] src/test/mir-opt/inline/inline-generator.rs stdout ----
---- [mir-opt] src/test/mir-opt/inline/inline-generator.rs stdout ----
30 +         let mut _10: bool;               // in scope 6 at $DIR/inline-generator.rs:15:9: 15:9
31 +         let _11: bool;                   // in scope 6 at $DIR/inline-generator.rs:15:6: 15:7
32 +         let mut _12: u32;                // in scope 6 at $DIR/inline-generator.rs:15:5: 15:41
+ +         let mut _13: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]; // in scope 6 at $DIR/inline-generator.rs:15:5: 15:41
+ +         let mut _14: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]; // in scope 6 at $DIR/inline-generator.rs:15:5: 15:41
+ +         let mut _15: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]; // in scope 6 at $DIR/inline-generator.rs:15:5: 15:41
34   
35       bb0: {


73 +         StorageLive(_10);                // scope 0 at $DIR/inline-generator.rs:9:14: 9:46
74 +         StorageLive(_11);                // scope 0 at $DIR/inline-generator.rs:9:14: 9:46
75 +         StorageLive(_12);                // scope 0 at $DIR/inline-generator.rs:9:14: 9:46
- +         _12 = discriminant((*(_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]))); // scope 6 at $DIR/inline-generator.rs:15:5: 15:41
+ +         StorageLive(_13);                // scope 6 at $DIR/inline-generator.rs:15:5: 15:41
+ +         _13 = move (_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]); // scope 6 at $DIR/inline-generator.rs:15:5: 15:41
+ +         _12 = discriminant((*_13));      // scope 6 at $DIR/inline-generator.rs:15:5: 15:41
+ +         StorageDead(_13);                // scope 6 at $DIR/inline-generator.rs:15:5: 15:41
77 +         switchInt(move _12) -> [0_u32: bb3, 1_u32: bb8, 3_u32: bb7, otherwise: bb9]; // scope 6 at $DIR/inline-generator.rs:15:5: 15:41
79   


118 +         Deinit(_1);                      // scope 6 at $DIR/inline-generator.rs:15:11: 15:39
119 +         ((_1 as Yielded).0: i32) = move _8; // scope 6 at $DIR/inline-generator.rs:15:11: 15:39
120 +         discriminant(_1) = 0;            // scope 6 at $DIR/inline-generator.rs:15:11: 15:39
- +         discriminant((*(_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]))) = 3; // scope 6 at $DIR/inline-generator.rs:15:11: 15:39
+ +         StorageLive(_14);                // scope 6 at $DIR/inline-generator.rs:15:11: 15:39
+ +         _14 = move (_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]); // scope 6 at $DIR/inline-generator.rs:15:11: 15:39
+ +         discriminant((*_14)) = 3;        // scope 6 at $DIR/inline-generator.rs:15:11: 15:39
+ +         StorageDead(_14);                // scope 6 at $DIR/inline-generator.rs:15:11: 15:39
122 +         goto -> bb1;                     // scope 0 at $DIR/inline-generator.rs:15:11: 15:39
124 + 


129 +         Deinit(_1);                      // scope 6 at $DIR/inline-generator.rs:15:41: 15:41
130 +         ((_1 as Complete).0: bool) = move _10; // scope 6 at $DIR/inline-generator.rs:15:41: 15:41
131 +         discriminant(_1) = 1;            // scope 6 at $DIR/inline-generator.rs:15:41: 15:41
- +         discriminant((*(_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]))) = 1; // scope 6 at $DIR/inline-generator.rs:15:41: 15:41
+ +         StorageLive(_15);                // scope 6 at $DIR/inline-generator.rs:15:41: 15:41
+ +         _15 = move (_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]); // scope 6 at $DIR/inline-generator.rs:15:41: 15:41
+ +         discriminant((*_15)) = 1;        // scope 6 at $DIR/inline-generator.rs:15:41: 15:41
+ +         StorageDead(_15);                // scope 6 at $DIR/inline-generator.rs:15:41: 15:41
133 +         goto -> bb1;                     // scope 0 at $DIR/inline-generator.rs:15:41: 15:41
135 + 


thread '[mir-opt] src/test/mir-opt/inline/inline-generator.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_generator.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3410:25


failures:
    [mir-opt] src/test/mir-opt/inline/inline-generator.rs
