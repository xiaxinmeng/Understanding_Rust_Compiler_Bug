plain

running 255 tests
.........................................................................F......i.......  88/255
.........................................................i...........iii................ 176/255
........F.......F.iii.iiiii.........ii........i................................
failures:

---- [mir-opt] tests/mir-opt/const_prop/transmute.rs stdout ----
3   
3   
4   fn unreachable_direct() -> ! {
5       let mut _0: !;                       // return place in scope 0 at $DIR/transmute.rs:+0:39: +0:40
-       let mut _1: !;                       // in scope 0 at $DIR/transmute.rs:+0:41: +3:2
-       let _2: Never;                       // in scope 0 at $DIR/transmute.rs:+1:9: +1:10
-       let mut _3: ();                      // in scope 0 at $DIR/transmute.rs:+1:39: +1:41
-       let mut _4: !;                       // in scope 0 at $DIR/transmute.rs:+2:5: +2:15
+       let _1: Never;                       // in scope 0 at $DIR/transmute.rs:+1:9: +1:10
+       let mut _2: ();                      // in scope 0 at $DIR/transmute.rs:+1:39: +1:41
10       scope 1 {
-           debug x => _2;                   // in scope 1 at $DIR/transmute.rs:+1:9: +1:10
+           debug x => _1;                   // in scope 1 at $DIR/transmute.rs:+1:9: +1:10
13       scope 2 {
14       }

15   
15   
16       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/transmute.rs:+0:41: +3:2
-           StorageLive(_2);                 // scope 0 at $DIR/transmute.rs:+1:9: +1:10
-           StorageLive(_3);                 // scope 2 at $DIR/transmute.rs:+1:39: +1:41
-           _3 = ();                         // scope 2 at $DIR/transmute.rs:+1:39: +1:41
-           _2 = move _3 as Never (Transmute); // scope 2 at $DIR/transmute.rs:+1:29: +1:42
+           StorageLive(_1);                 // scope 0 at $DIR/transmute.rs:+1:9: +1:10
+           StorageLive(_2);                 // scope 2 at $DIR/transmute.rs:+1:39: +1:41
+           _2 = ();                         // scope 2 at $DIR/transmute.rs:+1:39: +1:41
+           _1 = move _2 as Never (Transmute); // scope 2 at $DIR/transmute.rs:+1:29: +1:42
22           unreachable;                     // scope 2 at $DIR/transmute.rs:+1:29: +1:42
24   }


thread '[mir-opt] tests/mir-opt/const_prop/transmute.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/transmute.unreachable_direct.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3634:21

---- [mir-opt] tests/mir-opt/issue_72181_1.rs stdout ----
---- [mir-opt] tests/mir-opt/issue_72181_1.rs stdout ----
3 fn f(_1: Void) -> ! {
4     debug v => _1;                       // in scope 0 at $DIR/issue_72181_1.rs:+0:6: +0:7
5     let mut _0: !;                       // return place in scope 0 at $DIR/issue_72181_1.rs:+0:18: +0:19
-     let mut _2: !;                       // in scope 0 at $DIR/issue_72181_1.rs:+0:20: +2:2
-     let mut _3: !;                       // in scope 0 at $DIR/issue_72181_1.rs:+1:5: +1:15
9     bb0: {
9     bb0: {
-         StorageLive(_2);                 // scope 0 at $DIR/issue_72181_1.rs:+0:20: +2:2
-         StorageLive(_3);                 // scope 0 at $DIR/issue_72181_1.rs:+1:5: +1:15
12         FakeRead(ForMatchedPlace(None), _1); // scope 0 at $DIR/issue_72181_1.rs:+1:11: +1:12
13         unreachable;                     // scope 0 at $DIR/issue_72181_1.rs:+1:11: +1:12

15 
16     bb1: {
Build completed unsuccessfully in 0:11:19
Build completed unsuccessfully in 0:11:19
-         unreachable;                     // scope 0 at $DIR/issue_72181_1.rs:+1:5: +1:15
- 
-     bb2: {
-     bb2: {
-         StorageDead(_3);                 // scope 0 at $DIR/issue_72181_1.rs:+1:14: +1:15
-         unreachable;                     // scope 0 at $DIR/issue_72181_1.rs:+0:20: +2:2
- 
-     bb3: {
-     bb3: {
-         StorageDead(_2);                 // scope 0 at $DIR/issue_72181_1.rs:+2:1: +2:2
27         return;                          // scope 0 at $DIR/issue_72181_1.rs:+2:2: +2:2
29 }


thread '[mir-opt] tests/mir-opt/issue_72181_1.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/issue_72181_1.f.built.after.mir', src/tools/compiletest/src/runtest.rs:3634:21
---- [mir-opt] tests/mir-opt/lower_intrinsics.rs stdout ----
3   
4   fn unreachable() -> ! {
4   fn unreachable() -> ! {
5       let mut _0: !;                       // return place in scope 0 at $DIR/lower_intrinsics.rs:+0:25: +0:26
-       let mut _1: !;                       // in scope 0 at $DIR/lower_intrinsics.rs:+0:27: +2:2
-       let _2: ();                          // in scope 0 at $DIR/lower_intrinsics.rs:+1:14: +1:45
-       let mut _3: !;                       // in scope 0 at $DIR/lower_intrinsics.rs:+1:14: +1:45
+       let _1: ();                          // in scope 0 at $DIR/lower_intrinsics.rs:+1:14: +1:45
+       let mut _2: !;                       // in scope 0 at $DIR/lower_intrinsics.rs:+1:14: +1:45
9       scope 1 {
11   

12       bb0: {
12       bb0: {
-           StorageLive(_2);                 // scope 0 at $DIR/lower_intrinsics.rs:+1:5: +1:47
-           StorageLive(_3);                 // scope 1 at $DIR/lower_intrinsics.rs:+1:14: +1:45
- -         _3 = std::intrinsics::unreachable() -> unwind unreachable; // scope 1 at $DIR/lower_intrinsics.rs:+1:14: +1:45
+           StorageLive(_1);                 // scope 0 at $DIR/lower_intrinsics.rs:+1:5: +1:47
+           StorageLive(_2);                 // scope 1 at $DIR/lower_intrinsics.rs:+1:14: +1:45
+ -         _2 = std::intrinsics::unreachable() -> unwind unreachable; // scope 1 at $DIR/lower_intrinsics.rs:+1:14: +1:45
16 -                                          // mir::Constant
17 -                                          // + span: $DIR/lower_intrinsics.rs:31:14: 31:43
18 -                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn() -> ! {std::intrinsics::unreachable}, val: Value(<ZST>) }

thread '[mir-opt] tests/mir-opt/lower_intrinsics.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/lower_intrinsics.unreachable.LowerIntrinsics.diff', src/tools/compiletest/src/runtest.rs:3634:21

failures:
    [mir-opt] tests/mir-opt/const_prop/transmute.rs
    [mir-opt] tests/mir-opt/issue_72181_1.rs
