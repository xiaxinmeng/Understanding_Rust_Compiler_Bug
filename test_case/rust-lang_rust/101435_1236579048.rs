plain

---- [mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs stdout ----
23 |
24 fn main() -> () {
25     let mut _0: ();                      // return place in scope 0 at $DIR/region-subtyping-basic.rs:+0:11: +0:11
-     let mut _1: [usize; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]; // in scope 0 at $DIR/region-subtyping-basic.rs:+1:9: +1:14
+     let mut _1: [usize; Const { ty: usize, kind: Value(Leaf(0x00000003)) }]; // in scope 0 at $DIR/region-subtyping-basic.rs:+1:9: +1:14
27     let _3: usize;                       // in scope 0 at $DIR/region-subtyping-basic.rs:+2:16: +2:17
28     let mut _4: usize;                   // in scope 0 at $DIR/region-subtyping-basic.rs:+2:14: +2:18
29     let mut _5: bool;                    // in scope 0 at $DIR/region-subtyping-basic.rs:+2:14: +2:18
45 
46     bb0: {
46     bb0: {
47         StorageLive(_1);                 // bb0[0]: scope 0 at $DIR/region-subtyping-basic.rs:+1:9: +1:14
-         _1 = [const ConstValue(Scalar(0x0000000000000001): usize), const ConstValue(Scalar(0x0000000000000002): usize), const ConstValue(Scalar(0x0000000000000003): usize)]; // bb0[1]: scope 0 at $DIR/region-subtyping-basic.rs:+1:17: +1:26
+         _1 = [const ConstValue(Scalar(0x00000001): usize), const ConstValue(Scalar(0x00000002): usize), const ConstValue(Scalar(0x00000003): usize)]; // bb0[1]: scope 0 at $DIR/region-subtyping-basic.rs:+1:17: +1:26
49         FakeRead(ForLet(None), _1);      // bb0[2]: scope 0 at $DIR/region-subtyping-basic.rs:+1:9: +1:14
50         StorageLive(_2);                 // bb0[3]: scope 1 at $DIR/region-subtyping-basic.rs:+2:9: +2:10
51         StorageLive(_3);                 // bb0[4]: scope 1 at $DIR/region-subtyping-basic.rs:+2:16: +2:17

-         _3 = const ConstValue(Scalar(0x0000000000000000): usize); // bb0[5]: scope 1 at $DIR/region-subtyping-basic.rs:+2:16: +2:17
+         _3 = const ConstValue(Scalar(0x00000000): usize); // bb0[5]: scope 1 at $DIR/region-subtyping-basic.rs:+2:16: +2:17
53         _4 = Len(_1);                    // bb0[6]: scope 1 at $DIR/region-subtyping-basic.rs:+2:14: +2:18
54         _5 = Lt(_3, _4);                 // bb0[7]: scope 1 at $DIR/region-subtyping-basic.rs:+2:14: +2:18
55         assert(move _5, "index out of bounds: the length is {} but the index is {}", move _4, _3) -> [success: bb1, unwind: bb7]; // bb0[8]: scope 1 at $DIR/region-subtyping-basic.rs:+2:14: +2:18
85 
86     bb4: {
86     bb4: {
87         StorageLive(_10);                // bb4[0]: scope 3 at $DIR/region-subtyping-basic.rs:+7:9: +7:18
-         _10 = ConstValue(ZeroSized: fn(usize) -> bool {use_x})(const ConstValue(Scalar(0x0000000000000016): usize)) -> [return: bb5, unwind: bb7]; // bb4[1]: scope 3 at $DIR/region-subtyping-basic.rs:+7:9: +7:18
+         _10 = ConstValue(ZeroSized: fn(usize) -> bool {use_x})(const ConstValue(Scalar(0x00000016): usize)) -> [return: bb5, unwind: bb7]; // bb4[1]: scope 3 at $DIR/region-subtyping-basic.rs:+7:9: +7:18
89                                          // mir::Constant
90                                          // + span: $DIR/region-subtyping-basic.rs:23:9: 23:14
91                                          // + literal: Const { ty: fn(usize) -> bool {use_x}, val: Value(<ZST>) }

thread '[mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/nll/region_subtyping_basic.main.nll.0.mir', src/tools/compiletest/src/runtest.rs:3516:25


failures:
    [mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs
