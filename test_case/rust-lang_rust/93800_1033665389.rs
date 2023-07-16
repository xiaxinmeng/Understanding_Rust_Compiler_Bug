plain
Successfully built 6d0d57f96eb4
Successfully tagged rust-ci:latest
Built container sha256:6d0d57f96eb46ba5b900e29f8602cf34e8908a48709cf06febf2dc7591d9dc28
Uploading finished image to https://ci-caches.rust-lang.org/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0
upload failed: - to s3://rust-lang-ci-sccache2/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---
    Finished release [optimized] target(s) in 0.30s
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 165 tests
....F....FF........................i..............................i...............i................. 100/165
failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu

---- [mir-opt] mir-opt/const_allocation.rs stdout ----
---- [mir-opt] mir-opt/const_allocation.rs stdout ----
9         StorageLive(_1);                 // scope 0 at $DIR/const_allocation.rs:8:5: 8:8
10         StorageLive(_2);                 // scope 0 at $DIR/const_allocation.rs:8:5: 8:8
11         _2 = const {alloc1: &&[(Option<i32>, &[&str])]}; // scope 0 at $DIR/const_allocation.rs:8:5: 8:8
-                                          // ty::Const
-                                          // + ty: &&[(std::option::Option<i32>, &[&str])]
-                                          // + val: Value(Scalar(alloc1))
15                                          // mir::Constant
16                                          // + span: $DIR/const_allocation.rs:8:5: 8:8
-                                          // + literal: Const { ty: &&[(std::option::Option<i32>, &[&str])], val: Value(Scalar(alloc1)) }
+                                          // + literal: Const { ty: &&[(Option<i32>, &[&str])], val: Value(Scalar(alloc1)) }
18         _1 = (*_2);                      // scope 0 at $DIR/const_allocation.rs:8:5: 8:8
19         StorageDead(_2);                 // scope 0 at $DIR/const_allocation.rs:8:8: 8:9
20         StorageDead(_1);                 // scope 0 at $DIR/const_allocation.rs:8:8: 8:9

21         nop;                             // scope 0 at $DIR/const_allocation.rs:7:11: 9:2
22         return;                          // scope 0 at $DIR/const_allocation.rs:9:2: 9:2
- }
- 
- 
- alloc1 (static: FOO, size: 8, align: 4) {
- }
- 
- 
- alloc18 (size: 48, align: 4) {
-     0x00 │ 00 00 00 00 __ __ __ __ ╾─alloc5──╼ 00 00 00 00 │ ....░░░░╾──╼....
-     0x10 │ 00 00 00 00 __ __ __ __ ╾─alloc9──╼ 02 00 00 00 │ ....░░░░╾──╼....
-     0x20 │ 01 00 00 00 2a 00 00 00 ╾─alloc14─╼ 03 00 00 00 │ ....*...╾──╼....
- 
- 
- alloc5 (size: 0, align: 4) {}
- 
- alloc9 (size: 16, align: 4) {
- }
- 
- 
- alloc8 (size: 3, align: 1) {
-     66 6f 6f                                        │ foo
- 
- 
- alloc10 (size: 3, align: 1) {
- }
- 
- 
- alloc14 (size: 24, align: 4) {
-     0x00 │ ╾─alloc13─╼ 03 00 00 00 ╾─alloc15─╼ 03 00 00 00 │ ╾──╼....╾──╼....
-     0x10 │ ╾─alloc16─╼ 04 00 00 00                         │ ╾──╼....
- 
- 
- alloc13 (size: 3, align: 1) {
-     6d 65 68                                        │ meh
- 
- 
- alloc15 (size: 3, align: 1) {
-     6d 6f 70                                        │ mop
- 
- 
- alloc16 (size: 4, align: 1) {
-     6d c3 b6 70                                     │ m..p
66 


thread '[mir-opt] mir-opt/const_allocation.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation.main.ConstProp.after.32bit.mir', src/tools/compiletest/src/runtest.rs:3375:25

---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
9         StorageLive(_1);                 // scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
10         StorageLive(_2);                 // scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
11         _2 = const {alloc1: &&[(Option<i32>, &[&u8])]}; // scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
-                                          // ty::Const
-                                          // + ty: &&[(std::option::Option<i32>, &[&u8])]
-                                          // + val: Value(Scalar(alloc1))
15                                          // mir::Constant
16                                          // + span: $DIR/const_allocation2.rs:5:5: 5:8
-                                          // + literal: Const { ty: &&[(std::option::Option<i32>, &[&u8])], val: Value(Scalar(alloc1)) }
+                                          // + literal: Const { ty: &&[(Option<i32>, &[&u8])], val: Value(Scalar(alloc1)) }
18         _1 = (*_2);                      // scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
19         StorageDead(_2);                 // scope 0 at $DIR/const_allocation2.rs:5:8: 5:9
20         StorageDead(_1);                 // scope 0 at $DIR/const_allocation2.rs:5:8: 5:9

21         nop;                             // scope 0 at $DIR/const_allocation2.rs:4:11: 6:2
22         return;                          // scope 0 at $DIR/const_allocation2.rs:6:2: 6:2
- }
- 
- 
- alloc1 (static: FOO, size: 8, align: 4) {
- }
- 
- 
- alloc28 (size: 48, align: 4) {
-     0x00 │ 00 00 00 00 __ __ __ __ ╾─alloc13─╼ 00 00 00 00 │ ....░░░░╾──╼....
-     0x10 │ 00 00 00 00 __ __ __ __ ╾─alloc18─╼ 02 00 00 00 │ ....░░░░╾──╼....
-     0x20 │ 01 00 00 00 2a 00 00 00 ╾─alloc26─╼ 03 00 00 00 │ ....*...╾──╼....
- 
- 
- alloc13 (size: 0, align: 4) {}
- 
- alloc18 (size: 8, align: 4) {
-     ╾─alloc16─╼ ╾─alloc17─╼                         │ ╾──╼╾──╼
- 
- 
- alloc16 (size: 1, align: 1) {
- }
- 
- 
- alloc17 (size: 1, align: 1) {
- }
- 
- 
- alloc26 (size: 12, align: 4) {
-     ╾─a22+0x3─╼ ╾─alloc23─╼ ╾─a25+0x2─╼             │ ╾──╼╾──╼╾──╼
- 
- 
- alloc22 (size: 4, align: 1) {
-     2a 45 15 6f                                     │ *E.o
- 
- 
- alloc23 (size: 1, align: 1) {
-     2a                                              │ *
- 
- 
- alloc25 (size: 4, align: 1) {
-     2a 45 15 6f                                     │ *E.o
65 


thread '[mir-opt] mir-opt/const_allocation2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation2.main.ConstProp.after.32bit.mir', src/tools/compiletest/src/runtest.rs:3375:25
---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
9         StorageLive(_1);                 // scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
10         StorageLive(_2);                 // scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
11         _2 = const {alloc1: &&Packed};   // scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
-                                          // ty::Const
-                                          // + ty: &&Packed
-                                          // + val: Value(Scalar(alloc1))
15                                          // mir::Constant
16                                          // + span: $DIR/const_allocation3.rs:5:5: 5:8
17                                          // + literal: Const { ty: &&Packed, val: Value(Scalar(alloc1)) }

21         nop;                             // scope 0 at $DIR/const_allocation3.rs:4:11: 6:2
22         return;                          // scope 0 at $DIR/const_allocation3.rs:6:2: 6:2
- }
- 
- 
- alloc1 (static: FOO, size: 4, align: 4) {
- }
- 
- 
- alloc11 (size: 168, align: 1) {
-     0x00 │ ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab ab │ ................
-     0x10 │ ab ab ab ab ab ab ab ab ab ab ab ab ╾─alloc6──╼ │ ............╾──╼
-     0x20 │ 01 ef cd ab 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x30 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x40 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x50 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x60 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x70 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x80 │ 00 00 00 00 00 00 00 00 00 00 ╾─alloc8──╼ 00 00 │ ..........╾──╼..
-     0x90 │ ╾─a9+0x63─╼ 00 00 00 00 00 00 00 00 00 00 00 00 │ ╾──╼............
- }
- 
- 
- alloc6 (size: 4, align: 4) {
-     2a 00 00 00                                     │ *...
- 
- alloc8 (fn: main)
- 
- 
- alloc9 (size: 100, align: 1) {
-     0x00 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x10 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x20 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x30 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x40 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x50 │ 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 │ ................
-     0x60 │ 00 00 00 00                                     │ ....
59 


thread '[mir-opt] mir-opt/const_allocation3.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation3.main.ConstProp.after.32bit.mir', src/tools/compiletest/src/runtest.rs:3375:25

failures:
    [mir-opt] mir-opt/const_allocation.rs
    [mir-opt] mir-opt/const_allocation2.rs
