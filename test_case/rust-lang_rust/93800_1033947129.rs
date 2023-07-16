plain
Successfully built 348c3ac3e7e0
Successfully tagged rust-ci:latest
Built container sha256:348c3ac3e7e0b288ff1c28b7b222098d01ddc8b8cea2b5996b18a958c818425b
Uploading finished image to https://ci-caches.rust-lang.org/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0
upload failed: - to s3://rust-lang-ci-sccache2/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---
    Finished release [optimized] target(s) in 0.28s
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 165 tests
.......F.F...F.....................i..............................i...............i................. 100/165
failures:

---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
9         StorageLive(_1);                 // scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
10         StorageLive(_2);                 // scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
11         _2 = const {alloc1: &&Packed};   // scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
-                                          // ty::Const
-                                          // + ty: &&Packed
-                                          // + val: Value(Scalar(alloc1))
15                                          // mir::Constant
16                                          // + span: $DIR/const_allocation3.rs:5:5: 5:8
17                                          // + literal: Const { ty: &&Packed, val: Value(Scalar(alloc1)) }

thread '[mir-opt] mir-opt/const_allocation3.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation3.main.ConstProp.after.32bit.mir', src/tools/compiletest/src/runtest.rs:3375:25

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

thread '[mir-opt] mir-opt/const_allocation2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation2.main.ConstProp.after.32bit.mir', src/tools/compiletest/src/runtest.rs:3375:25
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

thread '[mir-opt] mir-opt/const_allocation.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation.main.ConstProp.after.32bit.mir', src/tools/compiletest/src/runtest.rs:3375:25

failures:
    [mir-opt] mir-opt/const_allocation.rs
    [mir-opt] mir-opt/const_allocation2.rs
