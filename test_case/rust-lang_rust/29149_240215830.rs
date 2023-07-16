 diff
diff --git a/src/libcollections/btree/node.rs b/src/libcollections/btree/node.rs
index e9bc291..255bbdd 100644
--- a/src/libcollections/btree/node.rs
+++ b/src/libcollections/btree/node.rs
@@ -288,8 +288,8 @@ pub struct NodeRef<BorrowType, K, V, Type> {
     _marker: PhantomData<(BorrowType, Type)>
 }

-impl<'a, K: 'a, V: 'a, Type> Copy for NodeRef<marker::Immut<'a>, K, V, Type> { }
-impl<'a, K: 'a, V: 'a, Type> Clone for NodeRef<marker::Immut<'a>, K, V, Type> {
+impl<BorrowType: Copy, K, V, Type> Copy for NodeRef<BorrowType, K, V, Type> { }
+impl<BorrowType: Copy, K, V, Type> Clone for NodeRef<BorrowType, K, V, Type> {
     fn clone(&self) -> Self {
         *self
     }
@@ -1529,6 +1529,7 @@ pub mod marker {
     pub enum LeafOrInternal { }

     pub enum Owned { }
+    #[derive(Copy, Clone)]
     pub struct Immut<'a>(PhantomData<&'a ()>);
     pub struct Mut<'a>(PhantomData<&'a mut ()>);

    Modified   src/librustc_typeck/Cargo.toml
