plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
.................
failures:

---- [mir-opt] src/test/mir-opt/issue-73223.rs stdout ----
1 - // MIR for `main` before SimplifyArmIdentity
2 + // MIR for `main` after SimplifyArmIdentity
+   
4   fn main() -> () {
4   fn main() -> () {
5       let mut _0: ();                      // return place in scope 0 at $DIR/issue-73223.rs:+0:11: +0:11
6       let _1: i32;                         // in scope 0 at $DIR/issue-73223.rs:+1:9: +1:14
47       scope 2 {
47       scope 2 {
48           debug v => _4;                   // in scope 2 at $DIR/issue-73223.rs:+2:14: +2:15
- 
+   
51       bb0: {
51       bb0: {
52           StorageLive(_1);                 // scope 0 at $DIR/issue-73223.rs:+1:9: +1:14
53           StorageLive(_2);                 // scope 0 at $DIR/issue-73223.rs:+1:23: +1:30

57           _3 = const 1_isize;              // scope 0 at $DIR/issue-73223.rs:+1:23: +1:30
58           goto -> bb3;                     // scope 0 at $DIR/issue-73223.rs:+1:17: +1:30
- 
+   
61       bb1: {
61       bb1: {
62           nop;                             // scope 0 at $DIR/issue-73223.rs:+3:17: +3:23
63           StorageDead(_2);                 // scope 0 at $DIR/issue-73223.rs:+4:6: +4:7

64           StorageDead(_1);                 // scope 0 at $DIR/issue-73223.rs:+8:1: +8:2
65           return;                          // scope 0 at $DIR/issue-73223.rs:+8:2: +8:2
- 
+   
68       bb2: {
68       bb2: {
69           unreachable;                     // scope 0 at $DIR/issue-73223.rs:+1:23: +1:30

- 
+   
72       bb3: {
72       bb3: {
73           StorageLive(_4);                 // scope 0 at $DIR/issue-73223.rs:+2:14: +2:15
- -       _4 = ((_2 as Some).0: i32);      // scope 0 at $DIR/issue-73223.rs:+2:14: +2:15
- -       _1 = _4;                         // scope 2 at $DIR/issue-73223.rs:+2:20: +2:21
- +       _4 = const 1_i32;                // scope 0 at $DIR/issue-73223.rs:+2:14: +2:15
- +       _1 = const 1_i32;                // scope 2 at $DIR/issue-73223.rs:+2:20: +2:21
+           _4 = const 1_i32;                // scope 0 at $DIR/issue-73223.rs:+2:14: +2:15
+           _1 = const 1_i32;                // scope 2 at $DIR/issue-73223.rs:+2:20: +2:21
78           StorageDead(_4);                 // scope 0 at $DIR/issue-73223.rs:+2:20: +2:21
79           StorageDead(_2);                 // scope 0 at $DIR/issue-73223.rs:+4:6: +4:7
80           StorageLive(_6);                 // scope 1 at $DIR/issue-73223.rs:+6:9: +6:14

81           StorageLive(_7);                 // scope 1 at $DIR/issue-73223.rs:+6:22: +6:27
- -       _7 = _1;                         // scope 1 at $DIR/issue-73223.rs:+6:22: +6:27
- +       _7 = const 1_i32;                // scope 1 at $DIR/issue-73223.rs:+6:22: +6:27
+           _7 = const 1_i32;                // scope 1 at $DIR/issue-73223.rs:+6:22: +6:27
84           Deinit(_6);                      // scope 1 at $DIR/issue-73223.rs:+6:17: +6:28
- -       ((_6 as Some).0: i32) = move _7; // scope 1 at $DIR/issue-73223.rs:+6:17: +6:28
- +       ((_6 as Some).0: i32) = const 1_i32; // scope 1 at $DIR/issue-73223.rs:+6:17: +6:28
+           ((_6 as Some).0: i32) = const 1_i32; // scope 1 at $DIR/issue-73223.rs:+6:17: +6:28
87           discriminant(_6) = 1;            // scope 1 at $DIR/issue-73223.rs:+6:17: +6:28
88           StorageDead(_7);                 // scope 1 at $DIR/issue-73223.rs:+6:27: +6:28
89           StorageLive(_8);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

108           StorageLive(_15);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
109           StorageLive(_16);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
110           StorageLive(_17);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
- -       _17 = (*_13);                    // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
- +       _17 = const 1_i32;               // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _17 = const 1_i32;               // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
113           StorageLive(_18);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
114           _18 = const 1_i32;               // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
- -       _16 = Eq(move _17, const 1_i32); // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
- +       _16 = const true;                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _16 = const true;                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
117           StorageDead(_18);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
118           StorageDead(_17);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
- -       _15 = Not(move _16);             // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
- +       _15 = const false;               // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _15 = const false;               // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
121           StorageDead(_16);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
- -       switchInt(move _15) -> [false: bb5, otherwise: bb4]; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
- +       goto -> bb5;                     // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           goto -> bb5;                     // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
- 
+   
126       bb4: {
126       bb4: {
127           StorageLive(_20);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
128           Deinit(_20);                     // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

152                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
153                                            // + literal: Const { ty: core::panicking::AssertKind, val: Value(Scalar(0x00)) }
- 
+   
156       bb5: {
156       bb5: {
157           nop;                             // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
158           StorageDead(_15);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

166           return;                          // scope 0 at $DIR/issue-73223.rs:+8:2: +8:2
168   }
- 
+   
170 
170 

thread '[mir-opt] src/test/mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.32bit.diff', src/tools/compiletest/src/runtest.rs:3516:25


failures:
    [mir-opt] src/test/mir-opt/issue-73223.rs
