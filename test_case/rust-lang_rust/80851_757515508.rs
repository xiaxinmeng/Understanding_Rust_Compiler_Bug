plain
.................................................................................................... 9000/11246
.................................................................................................... 9100/11246
.................................................................................................... 9200/11246
..........................................i......i.................................................. 9300/11246
.................................................................................iiiiii..iiiiii.i... 9400/11246
.................................................................................................... 9600/11246
.................................................................................................... 9700/11246
.................................................................................................... 9800/11246
.................................................................................................... 9900/11246
---
Suite("src/test/mir-opt") not skipped for "bootstrap::test::MirOpt" -- not in ["src/tools/tidy"]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 151 tests
........................F.......i.........................................i.....................F... 100/151
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.F.......F........i................................

---- [mir-opt] mir-opt/const_prop/control-flow-simplification.rs stdout ----
---- [mir-opt] mir-opt/const_prop/control-flow-simplification.rs stdout ----
4   fn hello() -> () {
5       let mut _0: ();                      // return place in scope 0 at $DIR/control-flow-simplification.rs:11:14: 11:14
6       let mut _1: bool;                    // in scope 0 at $DIR/control-flow-simplification.rs:12:8: 12:21
-       let mut _2: !;                       // in scope 0 at $SRC_DIR/std/src/macros.rs:LL:COL
+       let mut _2: !;                       // in scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
9       bb0: {
9       bb0: {
10           StorageLive(_1);                 // scope 0 at $DIR/control-flow-simplification.rs:12:8: 12:21
21       }
22   
23       bb2: {
23       bb2: {
-           StorageLive(_2);                 // scope 0 at $SRC_DIR/std/src/macros.rs:LL:COL
-           begin_panic::<&str>(const "explicit panic"); // scope 0 at $SRC_DIR/std/src/macros.rs:LL:COL
+           StorageLive(_2);                 // scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
+           begin_panic::<&str>(const "explicit panic"); // scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
26                                            // mir::Constant
-                                            // + span: $SRC_DIR/std/src/macros.rs:LL:COL
+                                            // + span: $SRC_DIR/std/src/panic.rs:LL:COL
28                                            // + literal: Const { ty: fn(&str) -> ! {std::rt::begin_panic::<&str>}, val: Value(Scalar(<ZST>)) }
29                                            // ty::Const
30                                            // + ty: &str

31                                            // + val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 })
32                                            // mir::Constant
-                                            // + span: $SRC_DIR/std/src/macros.rs:LL:COL
+                                            // + span: $SRC_DIR/std/src/panic.rs:LL:COL
34                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
36   }


thread '[mir-opt] mir-opt/const_prop/control-flow-simplification.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/control_flow_simplification.hello.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3452:25

---- [mir-opt] mir-opt/issue_76432.rs stdout ----
---- [mir-opt] mir-opt/issue_76432.rs stdout ----
21       let mut _19: *const T;               // in scope 0 at $DIR/issue_76432.rs:9:54: 9:68
22       let mut _20: *const T;               // in scope 0 at $DIR/issue_76432.rs:9:70: 9:84
23       let mut _21: *const T;               // in scope 0 at $DIR/issue_76432.rs:9:70: 9:84
-       let mut _22: !;                      // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _22: !;                      // in scope 0 at $SRC_DIR/core/src/panic.rs:LL:COL
25       scope 1 {
26           debug v => _2;                   // in scope 1 at $DIR/issue_76432.rs:7:9: 7:10
27           let _13: &T;                     // in scope 1 at $DIR/issue_76432.rs:9:10: 9:16
64       }
65   
66       bb1: {
66       bb1: {
-           StorageLive(_22);                // scope 1 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           core::panicking::panic(const "internal error: entered unreachable code"); // scope 1 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_22);                // scope 1 at $SRC_DIR/core/src/panic.rs:LL:COL
+           core::panicking::panic(const "internal error: entered unreachable code"); // scope 1 at $SRC_DIR/core/src/panic.rs:LL:COL
69                                            // mir::Constant
-                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                                            // + span: $SRC_DIR/core/src/panic.rs:LL:COL
71                                            // + literal: Const { ty: fn(&'static str) -> ! {core::panicking::panic}, val: Value(Scalar(<ZST>)) }
72                                            // ty::Const
73                                            // + ty: &str

thread '[mir-opt] mir-opt/issue_76432.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_76432.test.SimplifyComparisonIntegral.diff', src/tools/compiletest/src/runtest.rs:3452:25
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
18       let mut _16: bool;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
19       let mut _17: i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
20       let mut _18: i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _19: !;                      // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _20: std::fmt::Arguments;    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _19: !;                      // in scope 0 at $SRC_DIR/core/src/panic.rs:LL:COL
+       let mut _20: std::fmt::Arguments;    // in scope 0 at $SRC_DIR/core/src/panic.rs:LL:COL
23       let mut _21: &[&str];                // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
24       let mut _22: &[&str; 3];             // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
25       let _23: &[&str; 3];                 // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

26       let _24: [&str; 3];                  // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _25: &[std::fmt::ArgumentV1]; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _26: &[std::fmt::ArgumentV1; 2]; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let _27: &[std::fmt::ArgumentV1; 2]; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let _28: [std::fmt::ArgumentV1; 2];  // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _29: (&&i32, &&i32);         // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _25: &[std::fmt::ArgumentV1]; // in scope 0 at $SRC_DIR/core/src/panic.rs:LL:COL
+       let mut _26: &[std::fmt::ArgumentV1; 2]; // in scope 0 at $SRC_DIR/core/src/panic.rs:LL:COL
+       let _27: &[std::fmt::ArgumentV1; 2]; // in scope 0 at $SRC_DIR/core/src/panic.rs:LL:COL
+       let _28: [std::fmt::ArgumentV1; 2];  // in scope 0 at $SRC_DIR/core/src/panic.rs:LL:COL
+       let mut _29: (&&i32, &&i32);         // in scope 0 at $SRC_DIR/core/src/panic.rs:LL:COL
32       let mut _30: &&i32;                  // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
33       let _31: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
34       let mut _32: &&i32;                  // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

35       let _33: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _36: std::fmt::ArgumentV1;   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _36: std::fmt::ArgumentV1;   // in scope 0 at $SRC_DIR/core/src/panic.rs:LL:COL
37       let mut _37: &&i32;                  // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
38       let mut _38: for<'r, 's, 't0> fn(&'r &i32, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _39: std::fmt::ArgumentV1;   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _39: std::fmt::ArgumentV1;   // in scope 0 at $SRC_DIR/core/src/panic.rs:LL:COL
40       let mut _40: &&i32;                  // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
41       let mut _41: for<'r, 's, 't0> fn(&'r &i32, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
42       scope 1 {
56                   scope 5 {
56                   scope 5 {
57                       debug arg0 => _34;   // in scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
58                       debug arg1 => _35;   // in scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                       scope 6 (inlined ArgumentV1::new::<&i32>) { // at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                           debug x => _37;  // in scope 6 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                           debug f => _38;  // in scope 6 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                           let mut _44: for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>; // in scope 6 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                           let mut _45: for<'r, 's, 't0> fn(&'r &i32, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>; // in scope 6 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                           let mut _46: &core::fmt::Opaque; // in scope 6 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                           let mut _47: &&i32; // in scope 6 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                       scope 6 (inlined ArgumentV1::new::<&i32>) { // at $SRC_DIR/core/src/panic.rs:LL:COL
+                           debug x => _37;  // in scope 6 at $SRC_DIR/core/src/panic.rs:LL:COL
+                           debug f => _38;  // in scope 6 at $SRC_DIR/core/src/panic.rs:LL:COL
+                           let mut _44: for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>; // in scope 6 at $SRC_DIR/core/src/panic.rs:LL:COL
+                           let mut _45: for<'r, 's, 't0> fn(&'r &i32, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>; // in scope 6 at $SRC_DIR/core/src/panic.rs:LL:COL
+                           let mut _46: &core::fmt::Opaque; // in scope 6 at $SRC_DIR/core/src/panic.rs:LL:COL
+                           let mut _47: &&i32; // in scope 6 at $SRC_DIR/core/src/panic.rs:LL:COL
66                           scope 7 {
68                       }


-                       scope 8 (inlined ArgumentV1::new::<&i32>) { // at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                           debug x => _40;  // in scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                           debug f => _41;  // in scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                           let mut _48: for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>; // in scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                           let mut _49: for<'r, 's, 't0> fn(&'r &i32, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>; // in scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                           let mut _50: &core::fmt::Opaque; // in scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                           let mut _51: &&i32; // in scope 8 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                       scope 8 (inlined ArgumentV1::new::<&i32>) { // at $SRC_DIR/core/src/panic.rs:LL:COL
+                           debug x => _40;  // in scope 8 at $SRC_DIR/core/src/panic.rs:LL:COL
+                           debug f => _41;  // in scope 8 at $SRC_DIR/core/src/panic.rs:LL:COL
+                           let mut _48: for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>; // in scope 8 at $SRC_DIR/core/src/panic.rs:LL:COL
+                           let mut _49: for<'r, 's, 't0> fn(&'r &i32, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>; // in scope 8 at $SRC_DIR/core/src/panic.rs:LL:COL
+                           let mut _50: &core::fmt::Opaque; // in scope 8 at $SRC_DIR/core/src/panic.rs:LL:COL
+                           let mut _51: &&i32; // in scope 8 at $SRC_DIR/core/src/panic.rs:LL:COL
76                           scope 9 {
78                       }

79                   }
79                   }
-                   scope 10 (inlined Arguments::new_v1) { // at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                       debug pieces => _21; // in scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                       debug args => _25;   // in scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                       let mut _52: &[&str]; // in scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                       let mut _53: std::option::Option<&[std::fmt::rt::v1::Argument]>; // in scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                       let mut _54: &[std::fmt::ArgumentV1]; // in scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                   scope 10 (inlined Arguments::new_v1) { // at $SRC_DIR/core/src/panic.rs:LL:COL
+                       debug pieces => _21; // in scope 10 at $SRC_DIR/core/src/panic.rs:LL:COL
+                       debug args => _25;   // in scope 10 at $SRC_DIR/core/src/panic.rs:LL:COL
+                       let mut _52: &[&str]; // in scope 10 at $SRC_DIR/core/src/panic.rs:LL:COL
+                       let mut _53: std::option::Option<&[std::fmt::rt::v1::Argument]>; // in scope 10 at $SRC_DIR/core/src/panic.rs:LL:COL
+                       let mut _54: &[std::fmt::ArgumentV1]; // in scope 10 at $SRC_DIR/core/src/panic.rs:LL:COL
87               }
88           }

168       }
168       }
169   
170       bb4: {
-           StorageLive(_19);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_20);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_19);                // scope 4 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageLive(_20);                // scope 4 at $SRC_DIR/core/src/panic.rs:LL:COL
173           StorageLive(_21);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
174           StorageLive(_22);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
175           StorageLive(_23);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

184           _22 = _23;                       // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
185           _21 = move _22 as &[&str] (Pointer(Unsize)); // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
186           StorageDead(_22);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_25);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_26);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_27);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_28);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_29);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_25);                // scope 4 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageLive(_26);                // scope 4 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageLive(_27);                // scope 4 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageLive(_28);                // scope 4 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageLive(_29);                // scope 4 at $SRC_DIR/core/src/panic.rs:LL:COL
192           StorageLive(_30);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
193           StorageLive(_31);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
194           _31 = _13;                       // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

197           StorageLive(_33);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
198           _33 = _14;                       // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
199           _32 = &_33;                      // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           (_29.0: &&i32) = move _30;       // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           (_29.1: &&i32) = move _32;       // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_32);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_30);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           (_29.0: &&i32) = move _30;       // scope 4 at $SRC_DIR/core/src/panic.rs:LL:COL
+           (_29.1: &&i32) = move _32;       // scope 4 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageDead(_32);                // scope 4 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageDead(_30);                // scope 4 at $SRC_DIR/core/src/panic.rs:LL:COL
204           StorageLive(_34);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
205           _34 = (_29.0: &&i32);            // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
206           StorageLive(_35);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

207           _35 = (_29.1: &&i32);            // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_36);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_36);                // scope 5 at $SRC_DIR/core/src/panic.rs:LL:COL
209           StorageLive(_37);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
210           _37 = _34;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
211           StorageLive(_38);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
213                                            // mir::Constant
213                                            // mir::Constant
214                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
215                                            // + literal: Const { ty: for<'r, 's, 't0> fn(&'r &i32, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error> {<&i32 as std::fmt::Debug>::fmt}, val: Value(Scalar(<ZST>)) }
-           StorageLive(_44);                // scope 7 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_45);                // scope 7 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _45 = _38;                       // scope 7 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _44 = transmute::<for<'r, 's, 't0> fn(&'r &i32, &'s mut Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>, for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>>(move _45) -> bb5; // scope 7 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_44);                // scope 7 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageLive(_45);                // scope 7 at $SRC_DIR/core/src/panic.rs:LL:COL
+           _45 = _38;                       // scope 7 at $SRC_DIR/core/src/panic.rs:LL:COL
+           _44 = transmute::<for<'r, 's, 't0> fn(&'r &i32, &'s mut Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>, for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>>(move _45) -> bb5; // scope 7 at $SRC_DIR/core/src/panic.rs:LL:COL
220                                            // mir::Constant
-                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                                            // + span: $SRC_DIR/core/src/panic.rs:LL:COL
222                                            // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(for<'r, 's, 't0> fn(&'r &i32, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>) -> for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error> {std::intrinsics::transmute::<for<'r, 's, 't0> fn(&'r &i32, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>, for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>>}, val: Value(Scalar(<ZST>)) }
224   

225       bb5: {
225       bb5: {
-           StorageDead(_45);                // scope 7 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_46);                // scope 7 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_47);                // scope 7 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _47 = _37;                       // scope 7 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _46 = transmute::<&&i32, &core::fmt::Opaque>(move _47) -> bb6; // scope 7 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_45);                // scope 7 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageLive(_46);                // scope 7 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageLive(_47);                // scope 7 at $SRC_DIR/core/src/panic.rs:LL:COL
+           _47 = _37;                       // scope 7 at $SRC_DIR/core/src/panic.rs:LL:COL
+           _46 = transmute::<&&i32, &core::fmt::Opaque>(move _47) -> bb6; // scope 7 at $SRC_DIR/core/src/panic.rs:LL:COL
231                                            // mir::Constant
-                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                                            // + span: $SRC_DIR/core/src/panic.rs:LL:COL
233                                            // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(&&i32) -> &core::fmt::Opaque {std::intrinsics::transmute::<&&i32, &core::fmt::Opaque>}, val: Value(Scalar(<ZST>)) }
235   

236       bb6: {
236       bb6: {
-           StorageDead(_47);                // scope 7 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           (_36.0: &core::fmt::Opaque) = move _46; // scope 7 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           (_36.1: for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>) = move _44; // scope 7 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_46);                // scope 7 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_44);                // scope 7 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_38);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_37);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_39);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_47);                // scope 7 at $SRC_DIR/core/src/panic.rs:LL:COL
+           (_36.0: &core::fmt::Opaque) = move _46; // scope 7 at $SRC_DIR/core/src/panic.rs:LL:COL
+           (_36.1: for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>) = move _44; // scope 7 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageDead(_46);                // scope 7 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageDead(_44);                // scope 7 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageDead(_38);                // scope 5 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageDead(_37);                // scope 5 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageLive(_39);                // scope 5 at $SRC_DIR/core/src/panic.rs:LL:COL
245           StorageLive(_40);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
246           _40 = _35;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
247           StorageLive(_41);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
249                                            // mir::Constant
249                                            // mir::Constant
250                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
251                                            // + literal: Const { ty: for<'r, 's, 't0> fn(&'r &i32, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error> {<&i32 as std::fmt::Debug>::fmt}, val: Value(Scalar(<ZST>)) }
-           StorageLive(_48);                // scope 9 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_49);                // scope 9 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _49 = _41;                       // scope 9 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _48 = transmute::<for<'r, 's, 't0> fn(&'r &i32, &'s mut Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>, for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>>(move _49) -> bb7; // scope 9 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_48);                // scope 9 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageLive(_49);                // scope 9 at $SRC_DIR/core/src/panic.rs:LL:COL
+           _49 = _41;                       // scope 9 at $SRC_DIR/core/src/panic.rs:LL:COL
+           _48 = transmute::<for<'r, 's, 't0> fn(&'r &i32, &'s mut Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>, for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>>(move _49) -> bb7; // scope 9 at $SRC_DIR/core/src/panic.rs:LL:COL
256                                            // mir::Constant
-                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                                            // + span: $SRC_DIR/core/src/panic.rs:LL:COL
258                                            // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(for<'r, 's, 't0> fn(&'r &i32, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>) -> for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error> {std::intrinsics::transmute::<for<'r, 's, 't0> fn(&'r &i32, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>, for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>>}, val: Value(Scalar(<ZST>)) }
260   

261       bb7: {
261       bb7: {
-           StorageDead(_49);                // scope 9 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_50);                // scope 9 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_51);                // scope 9 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _51 = _40;                       // scope 9 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _50 = transmute::<&&i32, &core::fmt::Opaque>(move _51) -> bb8; // scope 9 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_49);                // scope 9 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageLive(_50);                // scope 9 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageLive(_51);                // scope 9 at $SRC_DIR/core/src/panic.rs:LL:COL
+           _51 = _40;                       // scope 9 at $SRC_DIR/core/src/panic.rs:LL:COL
+           _50 = transmute::<&&i32, &core::fmt::Opaque>(move _51) -> bb8; // scope 9 at $SRC_DIR/core/src/panic.rs:LL:COL
267                                            // mir::Constant
-                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                                            // + span: $SRC_DIR/core/src/panic.rs:LL:COL
269                                            // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(&&i32) -> &core::fmt::Opaque {std::intrinsics::transmute::<&&i32, &core::fmt::Opaque>}, val: Value(Scalar(<ZST>)) }
271   

272       bb8: {
272       bb8: {
-           StorageDead(_51);                // scope 9 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           (_39.0: &core::fmt::Opaque) = move _50; // scope 9 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           (_39.1: for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>) = move _48; // scope 9 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_50);                // scope 9 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_48);                // scope 9 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_41);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_40);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _28 = [move _36, move _39];      // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_39);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_36);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_35);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_34);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _27 = &_28;                      // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _26 = _27;                       // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _25 = move _26 as &[std::fmt::ArgumentV1] (Pointer(Unsize)); // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_26);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_52);                // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _52 = _21;                       // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_53);                // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           discriminant(_53) = 0;           // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_54);                // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _54 = _25;                       // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           (_20.0: &[&str]) = move _52;     // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           (_20.1: std::option::Option<&[std::fmt::rt::v1::Argument]>) = move _53; // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           (_20.2: &[std::fmt::ArgumentV1]) = move _54; // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_54);                // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_53);                // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_52);                // scope 10 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_25);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_21);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           core::panicking::panic_fmt(move _20); // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_51);                // scope 9 at $SRC_DIR/core/src/panic.rs:LL:COL
+           (_39.0: &core::fmt::Opaque) = move _50; // scope 9 at $SRC_DIR/core/src/panic.rs:LL:COL
+           (_39.1: for<'r, 's, 't0> fn(&'r core::fmt::Opaque, &'s mut std::fmt::Formatter<'t0>) -> std::result::Result<(), std::fmt::Error>) = move _48; // scope 9 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageDead(_50);                // scope 9 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageDead(_48);                // scope 9 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageDead(_41);                // scope 5 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageDead(_40);                // scope 5 at $SRC_DIR/core/src/panic.rs:LL:COL
+           _28 = [move _36, move _39];      // scope 5 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageDead(_39);                // scope 5 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageDead(_36);                // scope 5 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageDead(_35);                // scope 4 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageDead(_34);                // scope 4 at $SRC_DIR/core/src/panic.rs:LL:COL
+           _27 = &_28;                      // scope 4 at $SRC_DIR/core/src/panic.rs:LL:COL
+           _26 = _27;                       // scope 4 at $SRC_DIR/core/src/panic.rs:LL:COL
+           _25 = move _26 as &[std::fmt::ArgumentV1] (Pointer(Unsize)); // scope 4 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageDead(_26);                // scope 4 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageLive(_52);                // scope 10 at $SRC_DIR/core/src/panic.rs:LL:COL
+           _52 = _21;                       // scope 10 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageLive(_53);                // scope 10 at $SRC_DIR/core/src/panic.rs:LL:COL
+           discriminant(_53) = 0;           // scope 10 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageLive(_54);                // scope 10 at $SRC_DIR/core/src/panic.rs:LL:COL
+           _54 = _25;                       // scope 10 at $SRC_DIR/core/src/panic.rs:LL:COL
+           (_20.0: &[&str]) = move _52;     // scope 10 at $SRC_DIR/core/src/panic.rs:LL:COL
+           (_20.1: std::option::Option<&[std::fmt::rt::v1::Argument]>) = move _53; // scope 10 at $SRC_DIR/core/src/panic.rs:LL:COL
+           (_20.2: &[std::fmt::ArgumentV1]) = move _54; // scope 10 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageDead(_54);                // scope 10 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageDead(_53);                // scope 10 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageDead(_52);                // scope 10 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageDead(_25);                // scope 4 at $SRC_DIR/core/src/panic.rs:LL:COL
+           StorageDead(_21);                // scope 4 at $SRC_DIR/core/src/panic.rs:LL:COL
+           core::panicking::panic_fmt(move _20); // scope 4 at $SRC_DIR/core/src/panic.rs:LL:COL
304                                            // mir::Constant
-                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                                            // + span: $SRC_DIR/core/src/panic.rs:LL:COL
306                                            // + literal: Const { ty: for<'r> fn(std::fmt::Arguments<'r>) -> ! {core::panicking::panic_fmt}, val: Value(Scalar(<ZST>)) }
308   }


thread '[mir-opt] mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.64bit.diff', src/tools/compiletest/src/runtest.rs:3452:25
---- [mir-opt] mir-opt/no-drop-for-inactive-variant.rs stdout ----
---- [mir-opt] mir-opt/no-drop-for-inactive-variant.rs stdout ----
5     let mut _0: T;                       // return place in scope 0 at $DIR/no-drop-for-inactive-variant.rs:7:33: 7:34
6     let mut _2: isize;                   // in scope 0 at $DIR/no-drop-for-inactive-variant.rs:9:9: 9:16
7     let _3: T;                           // in scope 0 at $DIR/no-drop-for-inactive-variant.rs:9:14: 9:15
-     let mut _4: !;                       // in scope 0 at $SRC_DIR/std/src/macros.rs:LL:COL
+     let mut _4: !;                       // in scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
9     let mut _5: isize;                   // in scope 0 at $DIR/no-drop-for-inactive-variant.rs:12:1: 12:2
10     let mut _6: isize;                   // in scope 0 at $DIR/no-drop-for-inactive-variant.rs:12:1: 12:2
11     let mut _7: isize;                   // in scope 0 at $DIR/no-drop-for-inactive-variant.rs:12:1: 12:2
19     }
20 
21     bb1: {
21     bb1: {
-         StorageLive(_4);                 // scope 0 at $SRC_DIR/std/src/macros.rs:LL:COL
-         begin_panic::<&str>(const "explicit panic") -> bb4; // scope 0 at $SRC_DIR/std/src/macros.rs:LL:COL
+         StorageLive(_4);                 // scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
+         begin_panic::<&str>(const "explicit panic") -> bb4; // scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
24                                          // mir::Constant
-                                          // + span: $SRC_DIR/std/src/macros.rs:LL:COL
+                                          // + span: $SRC_DIR/std/src/panic.rs:LL:COL
26                                          // + literal: Const { ty: fn(&str) -> ! {std::rt::begin_panic::<&str>}, val: Value(Scalar(<ZST>)) }
27                                          // ty::Const
28                                          // + ty: &str

29                                          // + val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 })
30                                          // mir::Constant
-                                          // + span: $SRC_DIR/std/src/macros.rs:LL:COL
+                                          // + span: $SRC_DIR/std/src/panic.rs:LL:COL
32                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
34 


thread '[mir-opt] mir-opt/no-drop-for-inactive-variant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/no_drop_for_inactive_variant.unwrap.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3452:25

failures:
    [mir-opt] mir-opt/const_prop/control-flow-simplification.rs
    [mir-opt] mir-opt/issue-73223.rs
    [mir-opt] mir-opt/issue-73223.rs
    [mir-opt] mir-opt/issue_76432.rs
    [mir-opt] mir-opt/no-drop-for-inactive-variant.rs

test result: FAILED. 144 passed; 4 failed; 3 ignored; 0 measured; 0 filtered out; finished in 2.38s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:10
