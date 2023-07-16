plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
......F..........................i................................
failures:

---- [mir-opt] mir-opt/issue-73223.rs stdout ----
27       let _26: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
28       let mut _27: std::option::Option<std::fmt::Arguments>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
29       scope 1 {
-           debug split => _1;               // in scope 1 at $DIR/issue-73223.rs:2:9: 2:14
+ -         debug split => _1;               // in scope 1 at $DIR/issue-73223.rs:2:9: 2:14
+ +         debug split => ((_6 as Some).0: i32); // in scope 1 at $DIR/issue-73223.rs:2:9: 2:14
31           let _6: std::option::Option<i32>; // in scope 1 at $DIR/issue-73223.rs:7:9: 7:14
32           scope 3 {
33               debug _prev => _6;           // in scope 3 at $DIR/issue-73223.rs:7:9: 7:14
45           }
46       }
47       scope 2 {
47       scope 2 {
-           debug v => _4;                   // in scope 2 at $DIR/issue-73223.rs:3:14: 3:15
+ -         debug v => _4;                   // in scope 2 at $DIR/issue-73223.rs:3:14: 3:15
+ +         debug v => ((_6 as Some).0: i32); // in scope 2 at $DIR/issue-73223.rs:3:14: 3:15
50   
51       bb0: {

66   
66   
67       bb2: {
68           StorageLive(_4);                 // scope 0 at $DIR/issue-73223.rs:3:14: 3:15
+ +         StorageLive(_6);                 // scope 1 at $DIR/issue-73223.rs:7:9: 7:14
+ +         StorageLive(_7);                 // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
69           _4 = ((_2 as Some).0: i32);      // scope 0 at $DIR/issue-73223.rs:3:14: 3:15
70           _1 = _4;                         // scope 2 at $DIR/issue-73223.rs:3:20: 3:21
+ +         _7 = _1;                         // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
+ +         _6 = _2;                         // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
71           StorageDead(_4);                 // scope 0 at $DIR/issue-73223.rs:3:20: 3:21
72           StorageDead(_2);                 // scope 0 at $DIR/issue-73223.rs:5:6: 5:7
-           StorageLive(_6);                 // scope 1 at $DIR/issue-73223.rs:7:9: 7:14
-           StorageLive(_7);                 // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
-           _7 = _1;                         // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
-           ((_6 as Some).0: i32) = move _7; // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
-           discriminant(_6) = 1;            // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
+ -         StorageLive(_6);                 // scope 1 at $DIR/issue-73223.rs:7:9: 7:14
+ -         StorageLive(_7);                 // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
+ -         _7 = _1;                         // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
+ -         ((_6 as Some).0: i32) = move _7; // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
+ -         discriminant(_6) = 1;            // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
78           StorageDead(_7);                 // scope 1 at $DIR/issue-73223.rs:7:27: 7:28
79           StorageLive(_8);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
80           StorageLive(_9);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

thread '[mir-opt] mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.32bit.diff', src/tools/compiletest/src/runtest.rs:3386:25


failures:
    [mir-opt] mir-opt/issue-73223.rs
