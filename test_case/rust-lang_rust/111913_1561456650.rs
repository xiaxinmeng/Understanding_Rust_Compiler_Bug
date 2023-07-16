plain
...................iii.iiiii.........ii........i................................

failures:

---- [mir-opt] tests/mir-opt/deref-patterns/string.rs stdout ----
43                                          // + literal: Const { ty: for<'a, 'b> fn(&'a str, &'b str) -> bool {<str as PartialEq>::eq}, val: Value(<ZST>) }
44                                          // mir::Constant
45                                          // + span: $DIR/string.rs:9:14: 9:17
-                                          // + literal: Const { ty: &str, val: Value(Slice(..)) }
+                                          // + literal: Const { ty: &str, val: Value(ValTree::Branch(..)) }
48 
49     bb4: {


thread '[mir-opt] tests/mir-opt/deref-patterns/string.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/deref-patterns/string.foo.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3639:21


failures:
    [mir-opt] tests/mir-opt/deref-patterns/string.rs
