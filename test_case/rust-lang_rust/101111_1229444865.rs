plain
 finished in 0.516 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 183 tests
........................................i.........F..................................... 88/183
.........i.........F..........................................i...........F............. 176/183
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [mir-opt] src/test/mir-opt/dead-store-elimination/provenance_soundness.rs stdout ----
---- [mir-opt] src/test/mir-opt/dead-store-elimination/provenance_soundness.rs stdout ----
6       let mut _0: ();                      // return place in scope 0 at $DIR/provenance_soundness.rs:+0:25: +0:25
8       bb0: {
8       bb0: {
-           Retag([fn entry] _1);            // scope 0 at $DIR/provenance_soundness.rs:+0:1: +0:27
+           Retag([fn entry] _1);            // scope 0 at $DIR/provenance_soundness.rs:+0:11: +0:13
10           _0 = const ();                   // scope 0 at $DIR/provenance_soundness.rs:+0:25: +0:27
11           return;                          // scope 0 at $DIR/provenance_soundness.rs:+0:27: +0:27


thread '[mir-opt] src/test/mir-opt/dead-store-elimination/provenance_soundness.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/dead-store-elimination/provenance_soundness.retags.DeadStoreElimination.diff', src/tools/compiletest/src/runtest.rs:3515:25

---- [mir-opt] src/test/mir-opt/inline/inline-retag.rs stdout ----
---- [mir-opt] src/test/mir-opt/inline/inline-retag.rs stdout ----
52         Retag(_7);                       // scope 1 at $DIR/inline-retag.rs:+2:11: +2:14
53         _6 = &(*_7);                     // scope 1 at $DIR/inline-retag.rs:+2:11: +2:14
54         Retag(_6);                       // scope 1 at $DIR/inline-retag.rs:+2:11: +2:14
-         Retag(_3);                       // scope 2 at $DIR/inline-retag.rs:16:1: 18:2
-         Retag(_6);                       // scope 2 at $DIR/inline-retag.rs:16:1: 18:2
+         Retag(_3);                       // scope 2 at $DIR/inline-retag.rs:16:8: 16:9
+         Retag(_6);                       // scope 2 at $DIR/inline-retag.rs:16:17: 16:18
57         StorageLive(_11);                // scope 2 at $DIR/inline-retag.rs:17:5: 17:7
58         _11 = (*_3);                     // scope 2 at $DIR/inline-retag.rs:17:5: 17:7
59         StorageLive(_12);                // scope 2 at $DIR/inline-retag.rs:17:11: 17:13

thread '[mir-opt] src/test/mir-opt/inline/inline-retag.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_retag.bar.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3515:25
---- [mir-opt] src/test/mir-opt/retag.rs stdout ----
---- [mir-opt] src/test/mir-opt/retag.rs stdout ----
7     let mut _3: &mut i32;                // in scope 0 at $DIR/retag.rs:+1:9: +1:10
9     bb0: {
9     bb0: {
-         Retag([fn entry] _1);            // scope 0 at $DIR/retag.rs:+0:5: +2:6
-         Retag([fn entry] _2);            // scope 0 at $DIR/retag.rs:+0:5: +2:6
+         Retag([fn entry] _1);            // scope 0 at $DIR/retag.rs:+0:16: +0:21
+         Retag([fn entry] _2);            // scope 0 at $DIR/retag.rs:+0:23: +0:24
12         StorageLive(_3);                 // scope 0 at $DIR/retag.rs:+1:9: +1:10
13         _3 = &mut (*_2);                 // scope 0 at $DIR/retag.rs:+1:9: +1:10
14         Retag(_3);                       // scope 0 at $DIR/retag.rs:+1:9: +1:10

thread '[mir-opt] src/test/mir-opt/retag.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/retag.{impl#0}-foo.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3515:25

failures:
    [mir-opt] src/test/mir-opt/dead-store-elimination/provenance_soundness.rs
    [mir-opt] src/test/mir-opt/inline/inline-retag.rs
