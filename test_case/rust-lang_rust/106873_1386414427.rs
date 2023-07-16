plain

---- [mir-opt] checkout/tests/mir-opt/nll/region_subtyping_basic.rs stdout ----
22 |
23 fn main() -> () {
24     let mut _0: ();                      // return place in scope 0 at $DIR/region_subtyping_basic.rs:+0:11: +0:11
-     let mut _1: [usize; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]; // in scope 0 at $DIR/region_subtyping_basic.rs:+1:9: +1:14
+     let mut _1: [usize; Const(Value(Leaf(0x0000000000000003)): usize)]; // in scope 0 at $DIR/region_subtyping_basic.rs:+1:9: +1:14
26     let _3: usize;                       // in scope 0 at $DIR/region_subtyping_basic.rs:+2:16: +2:17
27     let mut _4: usize;                   // in scope 0 at $DIR/region_subtyping_basic.rs:+2:14: +2:18
28     let mut _5: bool;                    // in scope 0 at $DIR/region_subtyping_basic.rs:+2:14: +2:18

thread '[mir-opt] checkout/tests/mir-opt/nll/region_subtyping_basic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/nll/region_subtyping_basic.main.nll.0.64bit.mir', src/tools/compiletest/src/runtest.rs:3463:21


failures:
    [mir-opt] checkout/tests/mir-opt/nll/region_subtyping_basic.rs
