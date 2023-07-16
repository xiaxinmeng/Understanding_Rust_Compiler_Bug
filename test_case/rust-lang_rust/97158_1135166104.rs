plain
.....i..................................................i............................... 176/177
.
failures:

---- [mir-opt] src/test/mir-opt/const_debuginfo.rs stdout ----
11       let mut _14: u32;                    // in scope 0 at $DIR/const_debuginfo.rs:21:13: 21:16
12       let mut _15: u32;                    // in scope 0 at $DIR/const_debuginfo.rs:21:19: 21:22
13       scope 1 {
-           debug x => _1;                   // in scope 1 at $DIR/const_debuginfo.rs:9:9: 9:10
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ -         debug x => _1;                   // in scope 1 at $DIR/const_debuginfo.rs:9:9: 9:10
+ +         debug x => const 1_u8;           // in scope 1 at $DIR/const_debuginfo.rs:9:9: 9:10
15           let _2: u8;                      // in scope 1 at $DIR/const_debuginfo.rs:10:9: 10:10
16           scope 2 {
-               debug y => _2;               // in scope 2 at $DIR/const_debuginfo.rs:10:9: 10:10
+ -             debug y => _2;               // in scope 2 at $DIR/const_debuginfo.rs:10:9: 10:10
+ +             debug y => const 2_u8;       // in scope 2 at $DIR/const_debuginfo.rs:10:9: 10:10
18               let _3: u8;                  // in scope 2 at $DIR/const_debuginfo.rs:11:9: 11:10
19               scope 3 {
-                   debug z => _3;           // in scope 3 at $DIR/const_debuginfo.rs:11:9: 11:10
+ -                 debug z => _3;           // in scope 3 at $DIR/const_debuginfo.rs:11:9: 11:10
+ +                 debug z => const 3_u8;   // in scope 3 at $DIR/const_debuginfo.rs:11:9: 11:10
21                   let _4: u8;              // in scope 3 at $DIR/const_debuginfo.rs:12:9: 12:12
22                   scope 4 {
-                       debug sum => _4;     // in scope 4 at $DIR/const_debuginfo.rs:12:9: 12:12
+ -                     debug sum => _4;     // in scope 4 at $DIR/const_debuginfo.rs:12:9: 12:12
+ +                     debug sum => const 6_u8; // in scope 4 at $DIR/const_debuginfo.rs:12:9: 12:12
24                       let _9: &str;        // in scope 4 at $DIR/const_debuginfo.rs:14:9: 14:10
25                       scope 5 {
-                           debug s => _9;   // in scope 5 at $DIR/const_debuginfo.rs:14:9: 14:10
+ -                         debug s => _9;   // in scope 5 at $DIR/const_debuginfo.rs:14:9: 14:10
+ +                         debug s => const "hello, world!"; // in scope 5 at $DIR/const_debuginfo.rs:14:9: 14:10
27                           let _10: (bool, bool, u32); // in scope 5 at $DIR/const_debuginfo.rs:16:9: 16:10
28                           scope 6 {
29                               debug f => _10; // in scope 6 at $DIR/const_debuginfo.rs:16:9: 16:10

35                                       debug p => _12; // in scope 8 at $DIR/const_debuginfo.rs:20:9: 20:10
36                                       let _13: u32; // in scope 8 at $DIR/const_debuginfo.rs:21:9: 21:10
37                                       scope 9 {
-                                           debug a => _13; // in scope 9 at $DIR/const_debuginfo.rs:21:9: 21:10
+ -                                         debug a => _13; // in scope 9 at $DIR/const_debuginfo.rs:21:9: 21:10
+ +                                         debug a => const 64_u32; // in scope 9 at $DIR/const_debuginfo.rs:21:9: 21:10
40                                   }
41                               }

48   
48   
49       bb0: {
50           StorageLive(_1);                 // scope 0 at $DIR/const_debuginfo.rs:9:9: 9:10
+           _1 = const 1_u8;                 // scope 0 at $DIR/const_debuginfo.rs:9:13: 9:16
51           StorageLive(_2);                 // scope 1 at $DIR/const_debuginfo.rs:10:9: 10:10
+           _2 = const 2_u8;                 // scope 1 at $DIR/const_debuginfo.rs:10:13: 10:16
52           StorageLive(_3);                 // scope 2 at $DIR/const_debuginfo.rs:11:9: 11:10
+           _3 = const 3_u8;                 // scope 2 at $DIR/const_debuginfo.rs:11:13: 11:16
53           StorageLive(_4);                 // scope 3 at $DIR/const_debuginfo.rs:12:9: 12:12
54           StorageLive(_5);                 // scope 3 at $DIR/const_debuginfo.rs:12:15: 12:20
55           StorageLive(_6);                 // scope 3 at $DIR/const_debuginfo.rs:12:15: 12:16

+           _6 = const 1_u8;                 // scope 3 at $DIR/const_debuginfo.rs:12:15: 12:16
56           StorageLive(_7);                 // scope 3 at $DIR/const_debuginfo.rs:12:19: 12:20
+           _7 = const 2_u8;                 // scope 3 at $DIR/const_debuginfo.rs:12:19: 12:20
+           _5 = const 3_u8;                 // scope 3 at $DIR/const_debuginfo.rs:12:15: 12:20
57           StorageDead(_7);                 // scope 3 at $DIR/const_debuginfo.rs:12:19: 12:20
58           StorageDead(_6);                 // scope 3 at $DIR/const_debuginfo.rs:12:19: 12:20
59           StorageLive(_8);                 // scope 3 at $DIR/const_debuginfo.rs:12:23: 12:24

+           _8 = const 3_u8;                 // scope 3 at $DIR/const_debuginfo.rs:12:23: 12:24
+           _4 = const 6_u8;                 // scope 3 at $DIR/const_debuginfo.rs:12:15: 12:24
60           StorageDead(_8);                 // scope 3 at $DIR/const_debuginfo.rs:12:23: 12:24
61           StorageDead(_5);                 // scope 3 at $DIR/const_debuginfo.rs:12:23: 12:24
62           StorageLive(_9);                 // scope 4 at $DIR/const_debuginfo.rs:14:9: 14:10

+           _9 = const "hello, world!";      // scope 4 at $DIR/const_debuginfo.rs:14:13: 14:28
+                                            // mir::Constant
+                                            // + span: $DIR/const_debuginfo.rs:14:13: 14:28
+                                            // + literal: Const { ty: &str, val: Value(Slice(..)) }
63           StorageLive(_10);                // scope 5 at $DIR/const_debuginfo.rs:16:9: 16:10
+           Deinit(_10);                     // scope 5 at $DIR/const_debuginfo.rs:16:13: 16:34
+           (_10.0: bool) = const true;      // scope 5 at $DIR/const_debuginfo.rs:16:13: 16:34
+           (_10.1: bool) = const false;     // scope 5 at $DIR/const_debuginfo.rs:16:13: 16:34
+           (_10.2: u32) = const 123_u32;    // scope 5 at $DIR/const_debuginfo.rs:16:13: 16:34
64           StorageLive(_11);                // scope 6 at $DIR/const_debuginfo.rs:18:9: 18:10
+           Deinit(_11);                     // scope 6 at $DIR/const_debuginfo.rs:18:13: 18:24
+           ((_11 as Some).0: u16) = const 99_u16; // scope 6 at $DIR/const_debuginfo.rs:18:13: 18:24
+           discriminant(_11) = 1;           // scope 6 at $DIR/const_debuginfo.rs:18:13: 18:24
65           StorageLive(_12);                // scope 7 at $DIR/const_debuginfo.rs:20:9: 20:10
+           Deinit(_12);                     // scope 7 at $DIR/const_debuginfo.rs:20:13: 20:35
+           (_12.0: u32) = const 32_u32;     // scope 7 at $DIR/const_debuginfo.rs:20:13: 20:35
+           (_12.1: u32) = const 32_u32;     // scope 7 at $DIR/const_debuginfo.rs:20:13: 20:35
66           StorageLive(_13);                // scope 8 at $DIR/const_debuginfo.rs:21:9: 21:10
67           StorageLive(_14);                // scope 8 at $DIR/const_debuginfo.rs:21:13: 21:16
+           _14 = const 32_u32;              // scope 8 at $DIR/const_debuginfo.rs:21:13: 21:16
68           StorageLive(_15);                // scope 8 at $DIR/const_debuginfo.rs:21:19: 21:22
+           _15 = const 32_u32;              // scope 8 at $DIR/const_debuginfo.rs:21:19: 21:22
+           _13 = const 64_u32;              // scope 8 at $DIR/const_debuginfo.rs:21:13: 21:22
69           StorageDead(_15);                // scope 8 at $DIR/const_debuginfo.rs:21:21: 21:22
70           StorageDead(_14);                // scope 8 at $DIR/const_debuginfo.rs:21:21: 21:22
71           nop;                             // scope 0 at $DIR/const_debuginfo.rs:8:11: 22:2

thread '[mir-opt] src/test/mir-opt/const_debuginfo.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_debuginfo.main.ConstDebugInfo.diff', src/tools/compiletest/src/runtest.rs:3410:25


failures:
    [mir-opt] src/test/mir-opt/const_debuginfo.rs
