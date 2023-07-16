 patch
--- blaold.rs   2015-03-07 18:48:27.695438678 +0100
+++ bla.rs  2015-03-07 18:48:14.581938331 +0100
@@ -1,51 +1,46 @@
 pub mod strided {
     use raw;

-    pub struct Slice<'a, T>(pub raw::Slice<'a, T>) where T: 'a;
+    pub struct Slice<'a, T>(pub raw::Slice<'a, T>);
     impl<'a, T> Copy for Slice<'a, T> {}
-    pub struct SliceMut<'a, T>(pub raw::Slice<'a, T>) where T: 'a;
+    pub struct SliceMut<'a, T>(pub raw::SliceMut<'a, T>);
 }

 pub mod raw {
     use std::marker::PhantomData;

-    pub struct Slice<'a, T> where T: 'a {
-        _marker: PhantomData<&'a T>,
-        data: *mut T,
+    pub struct Slice<'a, T> {
+        _marker: PhantomData<fn() -> &'a T>,
+
+        data: *const T,
         len: usize,
     }

     impl<'a, T> Copy for Slice<'a, T> {}
+
+    pub struct SliceMut<'a, T> {
+        _marker: PhantomData<fn() -> &'a mut T>,
+
+        data: *const T,
+        len: usize,
+    }
 }

-pub struct MutRow<'a, T>(strided::SliceMut<'a, T>) where T: 'a;
-pub struct Row<'a, T>(strided::Slice<'a, T>) where T: 'a;
+pub struct MutRow<'a, T>(strided::SliceMut<'a, T>);
+pub struct Row<'a, T>(strided::Slice<'a, T>);
 impl<'a, T> Copy for Row<'a, T> {}
 pub struct Scaled<T, M>(T, M);

-#[cfg(yesterday)]
 impl<'a, 'b, T> AddAssign<Scaled<T, Row<'a, T>>> for MutRow<'b, T> {
     fn add_assign(&mut self, rhs: &Scaled<T, Row<T>>) {
-        let &mut MutRow(strided::SliceMut(lhs)) = self;
-        let &Scaled(ref alpha, Row(strided::Slice(rhs))) = rhs;
-
-        axpy(lhs, alpha, rhs)
-    }
-}
-
-#[cfg(today)]
-impl<'a, 'b, T> AddAssign<Scaled<T, Row<'a, T>>> for MutRow<'b, T>
-    where T: 'a,  // <--
-{
-    fn add_assign(&mut self, rhs: &Scaled<T, Row<'a, T>>) {  // <-- `Row<['a], T>`
-        let &mut MutRow(strided::SliceMut(lhs)) = self;
-        let &Scaled(ref alpha, Row(strided::Slice(rhs))) = rhs;
+        let MutRow(strided::SliceMut(ref mut lhs)) = *self;
+        let Scaled(ref alpha, Row(strided::Slice(rhs))) = *rhs;

         axpy(lhs, alpha, rhs)
     }
 }

-fn axpy<T>(_: raw::Slice<T>, _: &T, _: raw::Slice<T>) {}
+fn axpy<T>(_: &mut raw::SliceMut<T>, _: &T, _: raw::Slice<T>) {}

 pub trait AddAssign<Rhs> {
     fn add_assign(&mut self, &Rhs);
