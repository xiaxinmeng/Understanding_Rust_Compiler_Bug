plain
failures:

---- [mir-opt] src/test/mir-opt/issue-73223.rs stdout ----
71   
72       bb3: {
73           StorageLive(_4);                 // scope 0 at $DIR/issue-73223.rs:+2:14: +2:15
-           _4 = const 1_i32;                // scope 0 at $DIR/issue-73223.rs:+2:14: +2:15
-           _1 = const 1_i32;                // scope 2 at $DIR/issue-73223.rs:+2:20: +2:21
+           _4 = ((_2 as Some).0: i32);      // scope 0 at $DIR/issue-73223.rs:+2:14: +2:15
+           _1 = _4;                         // scope 2 at $DIR/issue-73223.rs:+2:20: +2:21
76           StorageDead(_4);                 // scope 0 at $DIR/issue-73223.rs:+2:20: +2:21
77           StorageDead(_2);                 // scope 0 at $DIR/issue-73223.rs:+4:6: +4:7
78           StorageLive(_6);                 // scope 1 at $DIR/issue-73223.rs:+6:9: +6:14

79           StorageLive(_7);                 // scope 1 at $DIR/issue-73223.rs:+6:22: +6:27
-           _7 = const 1_i32;                // scope 1 at $DIR/issue-73223.rs:+6:22: +6:27
+           _7 = _1;                         // scope 1 at $DIR/issue-73223.rs:+6:22: +6:27
81           Deinit(_6);                      // scope 1 at $DIR/issue-73223.rs:+6:17: +6:28
-           ((_6 as Some).0: i32) = const 1_i32; // scope 1 at $DIR/issue-73223.rs:+6:17: +6:28
+           ((_6 as Some).0: i32) = move _7; // scope 1 at $DIR/issue-73223.rs:+6:17: +6:28
83           discriminant(_6) = 1;            // scope 1 at $DIR/issue-73223.rs:+6:17: +6:28
84           StorageDead(_7);                 // scope 1 at $DIR/issue-73223.rs:+6:27: +6:28
85           StorageLive(_8);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

104           StorageLive(_15);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
105           StorageLive(_16);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
106           StorageLive(_17);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _17 = const 1_i32;               // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _17 = (*_13);                    // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
108           StorageLive(_18);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
109           _18 = const 1_i32;               // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _16 = const true;                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _16 = Eq(move _17, const 1_i32); // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
111           StorageDead(_18);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
112           StorageDead(_17);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _15 = const false;               // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _15 = Not(move _16);             // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
114           StorageDead(_16);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           goto -> bb5;                     // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           switchInt(move _15) -> [false: bb5, otherwise: bb4]; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
117   
118       bb4: {


thread '[mir-opt] src/test/mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.64bit.diff', src/tools/compiletest/src/runtest.rs:3516:25


failures:
    [mir-opt] src/test/mir-opt/issue-73223.rs
