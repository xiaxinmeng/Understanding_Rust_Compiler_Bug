`patch
diff --git a/compiler/rustc_macros/src/query.rs b/compiler/rustc_macros/src/query.rs
index 6d876784be6..4a20e7778e2 100644
--- a/compiler/rustc_macros/src/query.rs
+++ b/compiler/rustc_macros/src/query.rs
@@ -27,13 +27,22 @@ fn parse(input: ParseStream<'_>) -> Result<Self> {
     }
 }

+macro_rules! static_assert_size {
+    ($ty:ty, $size:expr) => {
+        const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
+    };
+}
+
+#[cfg(target_arch = "x86_64")]
+static_assert_size!(QueryModifier, 120);
+
 /// A modifier for a query
 enum QueryModifier {
     /// The description of the query.
     Desc(Option<Ident>, Punctuated<Expr, Token![,]>),

     /// Use this type for the in-memory cache.
-    Storage(Type),
+    Storage(Box<Type>),

     /// Cache the query to disk if the `Expr` returns true.
     Cache(Option<(IdentOrWild, IdentOrWild)>, Block),
@@ -213,7 +222,7 @@ struct QueryModifiers {
     desc: (Option<Ident>, Punctuated<Expr, Token![,]>),

     /// Use this type for the in-memory cache.
-    storage: Option<Type>,
+    storage: Option<Box<Type>>,

     /// Cache the query to disk if the `Block` returns true.
     cache: Option<(Option<(IdentOrWild, IdentOrWild)>, Block)>,
 