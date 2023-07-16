diff
--- mir_dump/lib.bar.004-003.Inline.before.mir
+++ mir_dump/lib.bar.004-003.Inline.after.mir
@@ -1,50 +1,49 @@
 fn bar(_1: P) -> () {
     debug _baz => _1;
     let mut _0: ();
     let _2: ();
     let mut _3: &impl std::ops::Fn<()>;
     let _4: impl std::ops::Fn<()>;
     let mut _5: ();
+    scope 1 (inlined <fn() {foo} as Fn<()>>::call - shim(fn() {foo})) {
+    }


     bb1: {
         _3 = &_4;
         StorageLive(_5);
-        _2 = <impl Fn<()> as Fn<()>>::call(move _3, move _5) -> [return: bb2, unwind: bb4];
+        _2 = move (*_3)() -> [return: bb5, unwind: bb3];
     }
