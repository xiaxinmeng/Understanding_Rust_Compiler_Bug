plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..i................................
failures:

---- [mir-opt] src/test/mir-opt/issue_73223.rs stdout ----
111           _17 = (*_13);                    // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
112           StorageLive(_18);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
113           _18 = const 1_i32;               // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _16 = Eq(move _17, const 1_i32); // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           nop;                             // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
115           StorageDead(_18);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_17);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _15 = Not(move _16);             // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           nop;                             // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
118           StorageDead(_16);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           switchInt(move _15) -> [false: bb5, otherwise: bb4]; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           switchInt(move _17) -> [1_i32: bb5, otherwise: bb4]; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
121   
122       bb4: {


+           StorageDead(_17);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
123           StorageLive(_20);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
124           Deinit(_20);                     // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
125           discriminant(_20) = 0;           // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
150       }
151   
152       bb5: {
152       bb5: {
+           StorageDead(_17);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
153           nop;                             // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
154           StorageDead(_15);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
155           StorageDead(_14);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

thread '[mir-opt] src/test/mir-opt/issue_73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.diff', src/tools/compiletest/src/runtest.rs:3451:21


failures:
    [mir-opt] src/test/mir-opt/issue_73223.rs
