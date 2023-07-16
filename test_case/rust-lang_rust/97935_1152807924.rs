plain

---- [mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs stdout ----
23 |
24 fn main() -> () {
25     let mut _0: ();                      // return place in scope 0 at $DIR/region-subtyping-basic.rs:16:11: 16:11
-     let mut _1: [usize; Const { ty: usize, val: Value(Scalar(0x00000003)) }]; // in scope 0 at $DIR/region-subtyping-basic.rs:17:9: 17:14
+     let mut _1: [usize; Const { ty: usize, kind: Value(Scalar(0x00000003)) }]; // in scope 0 at $DIR/region-subtyping-basic.rs:17:9: 17:14
27     let _3: usize;                       // in scope 0 at $DIR/region-subtyping-basic.rs:18:16: 18:17
28     let mut _4: usize;                   // in scope 0 at $DIR/region-subtyping-basic.rs:18:14: 18:18
29     let mut _5: bool;                    // in scope 0 at $DIR/region-subtyping-basic.rs:18:14: 18:18

thread '[mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/nll/region_subtyping_basic.main.nll.0.32bit.mir', src/tools/compiletest/src/runtest.rs:3437:25


failures:
    [mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs
