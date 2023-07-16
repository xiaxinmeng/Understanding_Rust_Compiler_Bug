plain
................................F.................i................................
failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu

---- [mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs stdout ----
18 | '_#3r live at {bb1[0]}
19 | '_#4r live at {bb1[1..=3]}
20 | '_#5r live at {bb1[4..=7], bb2[0..=2]}
- | '_#3r: '_#4r due to Assignment at Single(bb1[0])
- | '_#4r: '_#5r due to Assignment at Single(bb1[3])
+ | '_#3r: '_#4r due to Assignment at Single(bb1[0]) ($DIR/region-subtyping-basic.rs:18:13: 18:18 (#0)
+ | '_#4r: '_#5r due to Assignment at Single(bb1[3]) ($DIR/region-subtyping-basic.rs:19:13: 19:14 (#0)
24 fn main() -> () {
24 fn main() -> () {
25     let mut _0: ();                      // return place in scope 0 at $DIR/region-subtyping-basic.rs:16:11: 16:11

thread '[mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/nll/region_subtyping_basic.main.nll.0.32bit.mir', src/tools/compiletest/src/runtest.rs:3406:25


failures:
    [mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs
