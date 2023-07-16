plain
test [ui] src/test/ui/type/type-unsatisfiable.rs#usage ... ok
test [ui] src/test/ui/type/type-unsatisfiable.rs#lib ... ok
test [ui] src/test/ui/typeck/call-block.rs ... ok
test [ui] src/test/ui/typeck/autoderef-with-param-env-error.rs ... ok
test [ui] src/test/ui/typeck/do-not-suggest-adding-missing-zero-to-floating-point-number.rs ... ok
test [ui] src/test/ui/typeck/issue-10401.rs ... ok
test [ui] src/test/ui/typeck/deref-multi.rs ... ok
test [ui] src/test/ui/typeck/explain_clone_autoref.rs ... ok
test [ui] src/test/ui/typeck/issue-13853-2.rs ... ok
---
test [ui] src/test/ui/typeck/typeck-default-trait-impl-negation-send.rs ... ok
test [ui] src/test/ui/typeck/remove-extra-argument.rs ... ok
test [ui] src/test/ui/typeck/typeck-default-trait-impl-send-param.rs ... ok
test [ui] src/test/ui/typeck/return_type_containing_closure.rs ... ok
test [ui] src/test/ui/typeck/suggest-adding-missing-zero-to-floating-point-number.rs ... ok
test [ui] src/test/ui/typeck/typeck-unsafe-always-share.rs ... ok
test [ui] src/test/ui/typeck/typeck-default-trait-impl-assoc-type.rs ... ok
test [ui] src/test/ui/typeck/typeck-default-trait-impl-cross-crate-coherence.rs ... ok
test [ui] src/test/ui/typeck/typeck_type_placeholder_item_help.rs ... ok
---

failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown

---- [mir-opt] src/test/mir-opt/derefer_complex_case.rs stdout ----
33                                            // + span: $DIR/derefer_complex_case.rs:4:17: 4:26
34                                            // + literal: Const { ty: &[i32; 2], val: Unevaluated(main, [], Some(promoted[0])) }
35           _2 = &(*_14);                    // scope 0 at $DIR/derefer_complex_case.rs:4:17: 4:26
-           _1 = <&[i32; 2] as IntoIterator>::into_iter(move _2) -> bb1; // scope 0 at $DIR/derefer_complex_case.rs:4:17: 4:26
+           _1 = <&[i32; 2] as IntoIterator>::into_iter(move _2) -> [return: bb1, unwind: bb8]; // scope 0 at $DIR/derefer_complex_case.rs:4:17: 4:26
37                                            // mir::Constant
38                                            // + span: $DIR/derefer_complex_case.rs:4:17: 4:26
39                                            // + literal: Const { ty: fn(&[i32; 2]) -> <&[i32; 2] as IntoIterator>::IntoIter {<&[i32; 2] as IntoIterator>::into_iter}, val: Value(<ZST>) }

53           StorageLive(_9);                 // scope 1 at $DIR/derefer_complex_case.rs:4:17: 4:26
54           _9 = &mut _4;                    // scope 1 at $DIR/derefer_complex_case.rs:4:17: 4:26
55           _8 = &mut (*_9);                 // scope 1 at $DIR/derefer_complex_case.rs:4:17: 4:26
-           _7 = <std::slice::Iter<i32> as Iterator>::next(move _8) -> bb3; // scope 1 at $DIR/derefer_complex_case.rs:4:17: 4:26
+           _7 = <std::slice::Iter<i32> as Iterator>::next(move _8) -> [return: bb3, unwind: bb8]; // scope 1 at $DIR/derefer_complex_case.rs:4:17: 4:26
57                                            // mir::Constant
58                                            // + span: $DIR/derefer_complex_case.rs:4:17: 4:26
59                                            // + literal: Const { ty: for<'r> fn(&'r mut std::slice::Iter<i32>) -> Option<<std::slice::Iter<i32> as Iterator>::Item> {<std::slice::Iter<i32> as Iterator>::next}, val: Value(<ZST>) }

74 +         StorageDead(_15);                // scope 2 at $DIR/derefer_complex_case.rs:4:34: 4:37
75           StorageLive(_13);                // scope 2 at $DIR/derefer_complex_case.rs:4:34: 4:37
76           _13 = _12;                       // scope 2 at $DIR/derefer_complex_case.rs:4:34: 4:37
-           _6 = std::mem::drop::<i32>(move _13) -> bb7; // scope 2 at $DIR/derefer_complex_case.rs:4:29: 4:38
+           _6 = std::mem::drop::<i32>(move _13) -> [return: bb7, unwind: bb8]; // scope 2 at $DIR/derefer_complex_case.rs:4:29: 4:38
78                                            // mir::Constant
79                                            // + span: $DIR/derefer_complex_case.rs:4:29: 4:33
80                                            // + literal: Const { ty: fn(i32) {std::mem::drop::<i32>}, val: Value(<ZST>) }

102           StorageDead(_6);                 // scope 1 at $DIR/derefer_complex_case.rs:4:39: 4:40
103           _5 = const ();                   // scope 1 at $DIR/derefer_complex_case.rs:4:5: 4:40
104           goto -> bb2;                     // scope 1 at $DIR/derefer_complex_case.rs:4:5: 4:40
- +     }
- + 
- +     bb8 (cleanup): {
- +         resume;                          // scope 0 at $DIR/derefer_complex_case.rs:3:1: 5:2
+   
+   
+       bb8 (cleanup): {
+           resume;                          // scope 0 at $DIR/derefer_complex_case.rs:3:1: 5:2
110   }
111   


thread '[mir-opt] src/test/mir-opt/derefer_complex_case.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/derefer_complex_case.main.Derefer.diff', src/tools/compiletest/src/runtest.rs:3466:25

---- [mir-opt] src/test/mir-opt/derefer_terminator_test.rs stdout ----
30   
31       bb0: {
31       bb0: {
32           StorageLive(_1);                 // scope 0 at $DIR/derefer_terminator_test.rs:3:9: 3:10
-           _1 = foo() -> bb1;               // scope 0 at $DIR/derefer_terminator_test.rs:3:13: 3:18
+           _1 = foo() -> [return: bb1, unwind: bb6]; // scope 0 at $DIR/derefer_terminator_test.rs:3:13: 3:18
34                                            // mir::Constant
35                                            // + span: $DIR/derefer_terminator_test.rs:3:13: 3:16
36                                            // + literal: Const { ty: fn() -> bool {foo}, val: Value(<ZST>) }
38   
39       bb1: {
39       bb1: {
40           StorageLive(_2);                 // scope 1 at $DIR/derefer_terminator_test.rs:4:9: 4:10
-           _2 = foo() -> bb2;               // scope 1 at $DIR/derefer_terminator_test.rs:4:13: 4:18
+           _2 = foo() -> [return: bb2, unwind: bb6]; // scope 1 at $DIR/derefer_terminator_test.rs:4:13: 4:18
42                                            // mir::Constant
43                                            // + span: $DIR/derefer_terminator_test.rs:4:13: 4:16
44                                            // + literal: Const { ty: fn() -> bool {foo}, val: Value(<ZST>) }

94           StorageDead(_2);                 // scope 1 at $DIR/derefer_terminator_test.rs:10:1: 10:2
95           StorageDead(_1);                 // scope 0 at $DIR/derefer_terminator_test.rs:10:1: 10:2
96           return;                          // scope 0 at $DIR/derefer_terminator_test.rs:10:2: 10:2
- +     }
- + 
- +     bb6 (cleanup): {
- +         resume;                          // scope 0 at $DIR/derefer_terminator_test.rs:2:1: 10:2
+   
+   
+       bb6 (cleanup): {
+           resume;                          // scope 0 at $DIR/derefer_terminator_test.rs:2:1: 10:2
102   }
103   


thread '[mir-opt] src/test/mir-opt/derefer_terminator_test.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/derefer_terminator_test.main.Derefer.diff', src/tools/compiletest/src/runtest.rs:3466:25

failures:
    [mir-opt] src/test/mir-opt/derefer_complex_case.rs
    [mir-opt] src/test/mir-opt/derefer_terminator_test.rs
