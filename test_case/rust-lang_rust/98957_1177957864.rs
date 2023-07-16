plain
....
failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu

---- [mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs stdout ----
70         StorageLive(_8);                 // bb2[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
71         StorageLive(_9);                 // bb2[1]: scope 3 at $DIR/region-subtyping-basic.rs:21:15: 21:17
72         _9 = (*_6);                      // bb2[2]: scope 3 at $DIR/region-subtyping-basic.rs:21:15: 21:17
-         _8 = ConstValue(Zst: fn(usize) -> bool {use_x})(move _9) -> [return: bb3, unwind: bb7]; // bb2[3]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
+         _8 = ConstValue(ZeroSized: fn(usize) -> bool {use_x})(move _9) -> [return: bb3, unwind: bb7]; // bb2[3]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
74                                          // mir::Constant
75                                          // + span: $DIR/region-subtyping-basic.rs:21:9: 21:14
76                                          // + literal: Const { ty: fn(usize) -> bool {use_x}, val: Value(<ZST>) }
79     bb3: {
79     bb3: {
80         StorageDead(_9);                 // bb3[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:17: 21:18
81         StorageDead(_8);                 // bb3[1]: scope 3 at $DIR/region-subtyping-basic.rs:21:18: 21:19
-         _0 = const ConstValue(Zst: ());  // bb3[2]: scope 3 at $DIR/region-subtyping-basic.rs:20:13: 22:6
+         _0 = const ConstValue(ZeroSized: ()); // bb3[2]: scope 3 at $DIR/region-subtyping-basic.rs:20:13: 22:6
83         goto -> bb6;                     // bb3[3]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
85 

86     bb4: {
86     bb4: {
87         StorageLive(_10);                // bb4[0]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
-         _10 = ConstValue(Zst: fn(usize) -> bool {use_x})(const ConstValue(Scalar(0x00000016): usize)) -> [return: bb5, unwind: bb7]; // bb4[1]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
+         _10 = ConstValue(ZeroSized: fn(usize) -> bool {use_x})(const ConstValue(Scalar(0x00000016): usize)) -> [return: bb5, unwind: bb7]; // bb4[1]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
89                                          // mir::Constant
90                                          // + span: $DIR/region-subtyping-basic.rs:23:9: 23:14
91                                          // + literal: Const { ty: fn(usize) -> bool {use_x}, val: Value(<ZST>) }
93 
94     bb5: {
94     bb5: {
95         StorageDead(_10);                // bb5[0]: scope 3 at $DIR/region-subtyping-basic.rs:23:18: 23:19
-         _0 = const ConstValue(Zst: ());  // bb5[1]: scope 3 at $DIR/region-subtyping-basic.rs:22:12: 24:6
+         _0 = const ConstValue(ZeroSized: ()); // bb5[1]: scope 3 at $DIR/region-subtyping-basic.rs:22:12: 24:6
97         goto -> bb6;                     // bb5[2]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
99 


thread '[mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/nll/region_subtyping_basic.main.nll.0.32bit.mir', src/tools/compiletest/src/runtest.rs:3466:25


failures:
    [mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs
