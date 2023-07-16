plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
.........
failures:

---- [mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs stdout ----
7 | Inferred Region Values
8 | '_#0r | U0 | {bb0[0..=8], bb1[0..=7], bb2[0..=3], bb3[0..=3], bb4[0..=1], bb5[0..=2], bb6[0..=5], bb7[0], '_#0r, '_#1r}
9 | '_#1r | U0 | {bb0[0..=8], bb1[0..=7], bb2[0..=3], bb3[0..=3], bb4[0..=1], bb5[0..=2], bb6[0..=5], bb7[0], '_#1r}
- | '_#2r | U0 | {}
- | '_#3r | U0 | {bb1[0..=7], bb2[0..=2]}
- | '_#4r | U0 | {bb1[1..=7], bb2[0..=2]}
- | '_#5r | U0 | {bb1[4..=7], bb2[0..=2]}
+ | '_#2r | U0 | {bb1[0..=7], bb2[0..=2]}
+ | '_#3r | U0 | {bb1[1..=7], bb2[0..=2]}
+ | '_#4r | U0 | {bb1[4..=7], bb2[0..=2]}
15 | Inference Constraints
15 | Inference Constraints
16 | '_#0r live at {bb0[0..=8], bb1[0..=7], bb2[0..=3], bb3[0..=3], bb4[0..=1], bb5[0..=2], bb6[0..=5], bb7[0]}

17 | '_#1r live at {bb0[0..=8], bb1[0..=7], bb2[0..=3], bb3[0..=3], bb4[0..=1], bb5[0..=2], bb6[0..=5], bb7[0]}
- | '_#3r live at {bb1[0]}
- | '_#4r live at {bb1[1..=3]}
- | '_#5r live at {bb1[4..=7], bb2[0..=2]}
- | '_#3r: '_#4r due to Assignment at Single(bb1[0]) ($DIR/region-subtyping-basic.rs:18:13: 18:18 (#0)
- | '_#4r: '_#5r due to Assignment at Single(bb1[3]) ($DIR/region-subtyping-basic.rs:19:13: 19:14 (#0)
+ | '_#2r live at {bb1[0]}
+ | '_#3r live at {bb1[1..=3]}
+ | '_#4r live at {bb1[4..=7], bb2[0..=2]}
+ | '_#2r: '_#3r due to Assignment at Single(bb1[0]) ($DIR/region-subtyping-basic.rs:18:13: 18:18 (#0)
+ | '_#3r: '_#4r due to Assignment at Single(bb1[3]) ($DIR/region-subtyping-basic.rs:19:13: 19:14 (#0)
24 fn main() -> () {
24 fn main() -> () {
25     let mut _0: ();                      // return place in scope 0 at $DIR/region-subtyping-basic.rs:+0:11: +0:11

33     let _10: bool;                       // in scope 0 at $DIR/region-subtyping-basic.rs:+7:9: +7:18
34     scope 1 {
35         debug v => _1;                   // in scope 1 at $DIR/region-subtyping-basic.rs:+1:9: +1:14
-         let _2: &'_#4r usize;            // in scope 1 at $DIR/region-subtyping-basic.rs:+2:9: +2:10
+         let _2: &'_#3r usize;            // in scope 1 at $DIR/region-subtyping-basic.rs:+2:9: +2:10
37         scope 2 {
38             debug p => _2;               // in scope 2 at $DIR/region-subtyping-basic.rs:+2:9: +2:10
-             let _6: &'_#5r usize;        // in scope 2 at $DIR/region-subtyping-basic.rs:+3:9: +3:10
+             let _6: &'_#4r usize;        // in scope 2 at $DIR/region-subtyping-basic.rs:+3:9: +3:10
40             scope 3 {
41                 debug q => _6;           // in scope 3 at $DIR/region-subtyping-basic.rs:+3:9: +3:10

56     }
57 
58     bb1: {
58     bb1: {
-         _2 = &'_#3r _1[_3];              // bb1[0]: scope 1 at $DIR/region-subtyping-basic.rs:+2:13: +2:18
+         _2 = &'_#2r _1[_3];              // bb1[0]: scope 1 at $DIR/region-subtyping-basic.rs:+2:13: +2:18
60         FakeRead(ForLet(None), _2);      // bb1[1]: scope 1 at $DIR/region-subtyping-basic.rs:+2:9: +2:10
61         StorageLive(_6);                 // bb1[2]: scope 2 at $DIR/region-subtyping-basic.rs:+3:9: +3:10
62         _6 = _2;                         // bb1[3]: scope 2 at $DIR/region-subtyping-basic.rs:+3:13: +3:14

thread '[mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/nll/region_subtyping_basic.main.nll.0.32bit.mir', src/tools/compiletest/src/runtest.rs:3516:25


failures:
    [mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs
