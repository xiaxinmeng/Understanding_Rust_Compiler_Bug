plain

running 248 tests
i.............................................................................i.........  88/248
...............................................i.....i...........ii.i................... 176/248
.............ii.iiii..F.....ii........i.................................
failures:

---- [mir-opt] tests/mir-opt/nll/region_subtyping_basic.rs stdout ----
---- [mir-opt] tests/mir-opt/nll/region_subtyping_basic.rs stdout ----
32     let _10: bool;                       // in scope 0 at $DIR/region_subtyping_basic.rs:+7:9: +7:18
33     scope 1 {
34         debug v => _1;                   // in scope 1 at $DIR/region_subtyping_basic.rs:+1:9: +1:14
-         let _2: &'?3 usize;            // in scope 1 at $DIR/region_subtyping_basic.rs:+2:9: +2:10
+         let _2: &'?3 usize;              // in scope 1 at $DIR/region_subtyping_basic.rs:+2:9: +2:10
36         scope 2 {
37             debug p => _2;               // in scope 2 at $DIR/region_subtyping_basic.rs:+2:9: +2:10
-             let _6: &'?4 usize;        // in scope 2 at $DIR/region_subtyping_basic.rs:+3:9: +3:10
+             let _6: &'?4 usize;          // in scope 2 at $DIR/region_subtyping_basic.rs:+3:9: +3:10
39             scope 3 {
40                 debug q => _6;           // in scope 3 at $DIR/region_subtyping_basic.rs:+3:9: +3:10

55     }
56 
57     bb1: {
57     bb1: {
-         _2 = &'?2 _1[_3];              // bb1[0]: scope 1 at $DIR/region_subtyping_basic.rs:+2:13: +2:18
+         _2 = &'?2 _1[_3];                // bb1[0]: scope 1 at $DIR/region_subtyping_basic.rs:+2:13: +2:18
59         FakeRead(ForLet(None), _2);      // bb1[1]: scope 1 at $DIR/region_subtyping_basic.rs:+2:9: +2:10
60         StorageLive(_6);                 // bb1[2]: scope 2 at $DIR/region_subtyping_basic.rs:+3:9: +3:10
61         _6 = _2;                         // bb1[3]: scope 2 at $DIR/region_subtyping_basic.rs:+3:13: +3:14

thread '[mir-opt] tests/mir-opt/nll/region_subtyping_basic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/nll/region_subtyping_basic.main.nll.0.32bit.mir', src/tools/compiletest/src/runtest.rs:3553:21


failures:
    [mir-opt] tests/mir-opt/nll/region_subtyping_basic.rs
