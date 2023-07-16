plain

failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] src/test/mir-opt/instrument_coverage.rs stdout ----
2 + // MIR for `main` after InstrumentCoverage
4   fn main() -> () {
4   fn main() -> () {
-       let mut _0: ();                      // return place in scope 0 at /the/src/instrument_coverage.rs:10:11: 10:11
-       let mut _1: ();                      // in scope 0 at /the/src/instrument_coverage.rs:10:1: 16:2
-       let mut _2: bool;                    // in scope 0 at /the/src/instrument_coverage.rs:12:12: 12:17
-       let mut _3: !;                       // in scope 0 at /the/src/instrument_coverage.rs:12:18: 14:10
+       let mut _0: ();                      // return place in scope 0 at /the/src/instrument_coverage.rs:+0:11: +0:11
+       let mut _1: ();                      // in scope 0 at /the/src/instrument_coverage.rs:+0:1: +6:2
+       let mut _2: bool;                    // in scope 0 at /the/src/instrument_coverage.rs:+2:12: +2:17
+       let mut _3: !;                       // in scope 0 at /the/src/instrument_coverage.rs:+2:18: +4:10
10       bb0: {
10       bb0: {
- +         Coverage::Counter(1) for /the/src/instrument_coverage.rs:10:1 - 10:11; // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
-           goto -> bb1;                     // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
+ +         Coverage::Counter(1) for /the/src/instrument_coverage.rs:10:1 - 10:11; // scope 0 at /the/src/instrument_coverage.rs:+1:5: +5:6
+           goto -> bb1;                     // scope 0 at /the/src/instrument_coverage.rs:+1:5: +5:6
14   
15       bb1: {


- +         Coverage::Expression(4294967295) = 1 + 2 for /the/src/instrument_coverage.rs:11:5 - 12:17; // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
-           falseUnwind -> [real: bb2, cleanup: bb6]; // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
+ +         Coverage::Expression(4294967295) = 1 + 2 for /the/src/instrument_coverage.rs:11:5 - 12:17; // scope 0 at /the/src/instrument_coverage.rs:+1:5: +5:6
+           falseUnwind -> [real: bb2, cleanup: bb6]; // scope 0 at /the/src/instrument_coverage.rs:+1:5: +5:6
19   
20       bb2: {


-           StorageLive(_2);                 // scope 0 at /the/src/instrument_coverage.rs:12:12: 12:17
-           _2 = bar() -> [return: bb3, unwind: bb6]; // scope 0 at /the/src/instrument_coverage.rs:12:12: 12:17
+           StorageLive(_2);                 // scope 0 at /the/src/instrument_coverage.rs:+2:12: +2:17
+           _2 = bar() -> [return: bb3, unwind: bb6]; // scope 0 at /the/src/instrument_coverage.rs:+2:12: +2:17
23                                            // mir::Constant
24                                            // + span: /the/src/instrument_coverage.rs:12:12: 12:15
25                                            // + literal: Const { ty: fn() -> bool {bar}, val: Value(<ZST>) }
26       }
27   
28       bb3: {
28       bb3: {
-           switchInt(move _2) -> [false: bb5, otherwise: bb4]; // scope 0 at /the/src/instrument_coverage.rs:12:12: 12:17
+           switchInt(move _2) -> [false: bb5, otherwise: bb4]; // scope 0 at /the/src/instrument_coverage.rs:+2:12: +2:17
31   
32       bb4: {


- +         Coverage::Expression(4294967293) = 4294967294 + 0 for /the/src/instrument_coverage.rs:16:1 - 16:2; // scope 0 at /the/src/instrument_coverage.rs:16:2: 16:2
- +         Coverage::Expression(4294967294) = 4294967295 - 2 for /the/src/instrument_coverage.rs:13:13 - 13:18; // scope 0 at /the/src/instrument_coverage.rs:16:2: 16:2
-           _0 = const ();                   // scope 0 at /the/src/instrument_coverage.rs:13:13: 13:18
-           StorageDead(_2);                 // scope 0 at /the/src/instrument_coverage.rs:14:9: 14:10
-           return;                          // scope 0 at /the/src/instrument_coverage.rs:16:2: 16:2
+ +         Coverage::Expression(4294967293) = 4294967294 + 0 for /the/src/instrument_coverage.rs:16:1 - 16:2; // scope 0 at /the/src/instrument_coverage.rs:+6:2: +6:2
+ +         Coverage::Expression(4294967294) = 4294967295 - 2 for /the/src/instrument_coverage.rs:13:13 - 13:18; // scope 0 at /the/src/instrument_coverage.rs:+6:2: +6:2
+           _0 = const ();                   // scope 0 at /the/src/instrument_coverage.rs:+3:13: +3:18
+           StorageDead(_2);                 // scope 0 at /the/src/instrument_coverage.rs:+4:9: +4:10
+           return;                          // scope 0 at /the/src/instrument_coverage.rs:+6:2: +6:2
39   
40       bb5: {


- +         Coverage::Counter(2) for /the/src/instrument_coverage.rs:14:10 - 14:11; // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
-           _1 = const ();                   // scope 0 at /the/src/instrument_coverage.rs:14:10: 14:10
-           StorageDead(_2);                 // scope 0 at /the/src/instrument_coverage.rs:14:9: 14:10
-           goto -> bb1;                     // scope 0 at /the/src/instrument_coverage.rs:11:5: 15:6
+ +         Coverage::Counter(2) for /the/src/instrument_coverage.rs:14:10 - 14:11; // scope 0 at /the/src/instrument_coverage.rs:+1:5: +5:6
+           _1 = const ();                   // scope 0 at /the/src/instrument_coverage.rs:+4:10: +4:10
+           StorageDead(_2);                 // scope 0 at /the/src/instrument_coverage.rs:+4:9: +4:10
+           goto -> bb1;                     // scope 0 at /the/src/instrument_coverage.rs:+1:5: +5:6
46   
46   
47       bb6 (cleanup): {

-           resume;                          // scope 0 at /the/src/instrument_coverage.rs:10:1: 16:2
+           resume;                          // scope 0 at /the/src/instrument_coverage.rs:+0:1: +6:2
50   }
51   


thread '[mir-opt] src/test/mir-opt/instrument_coverage.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/instrument_coverage.main.InstrumentCoverage.diff', src/tools/compiletest/src/runtest.rs:3513:25


failures:
    [mir-opt] src/test/mir-opt/instrument_coverage.rs
