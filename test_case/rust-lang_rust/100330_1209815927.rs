plain
test [mir-opt] src/test/mir-opt/storage_live_dead_in_statics.rs ... ok

failures:

---- [mir-opt] src/test/mir-opt/instrument_coverage.rs stdout ----
8       let mut _3: !;                       // in scope 0 at /the/src/instrument_coverage.rs:+2:18: +4:10
10       bb0: {
10       bb0: {
- +         Coverage::Counter(1) for /the/src/instrument_coverage.rs:10:1 - 10:11; // scope 0 at /the/src/instrument_coverage.rs:+1:5: +5:6
+ +         Coverage::Counter(1) for /the/src/instrument_coverage.rs:11:1 - 11:11; // scope 0 at /the/src/instrument_coverage.rs:+1:5: +5:6
12           goto -> bb1;                     // scope 0 at /the/src/instrument_coverage.rs:+1:5: +5:6
14   

15       bb1: {
15       bb1: {
- +         Coverage::Expression(4294967295) = 1 + 2 for /the/src/instrument_coverage.rs:11:5 - 12:17; // scope 0 at /the/src/instrument_coverage.rs:+1:5: +5:6
+ +         Coverage::Expression(4294967295) = 1 + 2 for /the/src/instrument_coverage.rs:12:5 - 13:17; // scope 0 at /the/src/instrument_coverage.rs:+1:5: +5:6
17           falseUnwind -> [real: bb2, cleanup: bb6]; // scope 0 at /the/src/instrument_coverage.rs:+1:5: +5:6
19   


21           StorageLive(_2);                 // scope 0 at /the/src/instrument_coverage.rs:+2:12: +2:17
22           _2 = bar() -> [return: bb3, unwind: bb6]; // scope 0 at /the/src/instrument_coverage.rs:+2:12: +2:17
23                                            // mir::Constant
-                                            // + span: /the/src/instrument_coverage.rs:12:12: 12:15
+                                            // + span: /the/src/instrument_coverage.rs:13:12: 13:15
25                                            // + literal: Const { ty: fn() -> bool {bar}, val: Value(<ZST>) }
27   

30       }
31   
31   
32       bb4: {
- +         Coverage::Expression(4294967293) = 4294967294 + 0 for /the/src/instrument_coverage.rs:16:1 - 16:2; // scope 0 at /the/src/instrument_coverage.rs:+6:2: +6:2
- +         Coverage::Expression(4294967294) = 4294967295 - 2 for /the/src/instrument_coverage.rs:13:13 - 13:18; // scope 0 at /the/src/instrument_coverage.rs:+6:2: +6:2
+ +         Coverage::Expression(4294967293) = 4294967294 + 0 for /the/src/instrument_coverage.rs:17:1 - 17:2; // scope 0 at /the/src/instrument_coverage.rs:+6:2: +6:2
+ +         Coverage::Expression(4294967294) = 4294967295 - 2 for /the/src/instrument_coverage.rs:14:13 - 14:18; // scope 0 at /the/src/instrument_coverage.rs:+6:2: +6:2
35           _0 = const ();                   // scope 0 at /the/src/instrument_coverage.rs:+3:13: +3:18
36           StorageDead(_2);                 // scope 0 at /the/src/instrument_coverage.rs:+4:9: +4:10
37           return;                          // scope 0 at /the/src/instrument_coverage.rs:+6:2: +6:2
38       }
39   
40       bb5: {
40       bb5: {
- +         Coverage::Counter(2) for /the/src/instrument_coverage.rs:14:10 - 14:11; // scope 0 at /the/src/instrument_coverage.rs:+1:5: +5:6
+ +         Coverage::Counter(2) for /the/src/instrument_coverage.rs:15:10 - 15:11; // scope 0 at /the/src/instrument_coverage.rs:+1:5: +5:6
42           _1 = const ();                   // scope 0 at /the/src/instrument_coverage.rs:+4:10: +4:10
43           StorageDead(_2);                 // scope 0 at /the/src/instrument_coverage.rs:+4:9: +4:10
44           goto -> bb1;                     // scope 0 at /the/src/instrument_coverage.rs:+1:5: +5:6

thread '[mir-opt] src/test/mir-opt/instrument_coverage.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/instrument_coverage.main.InstrumentCoverage.diff', src/tools/compiletest/src/runtest.rs:3513:25


failures:
    [mir-opt] src/test/mir-opt/instrument_coverage.rs
