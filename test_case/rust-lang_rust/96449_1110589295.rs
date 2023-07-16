diff
diff --git a/compiler/toc_analysis/src/const_eval/errors.rs b/compiler/toc_analysis/src/const_eval/errors.rs
index f0fa6196..5915c56d 100644
--- a/compiler/toc_analysis/src/const_eval/errors.rs
+++ b/compiler/toc_analysis/src/const_eval/errors.rs
@@ -146,6 +146,8 @@ pub(super) enum ErrorKind {
     /// Produced a char(n) that is too big
     #[error("produced a character sequence that is too large")]
     CharNTooBig,
+    #[error("")]
+    NoFields(toc_hir::symbol::Symbol),
 
     // Unsupported messages
     /// Currently unsupported const eval value
diff --git a/compiler/toc_analysis/src/const_eval/value.rs b/compiler/toc_analysis/src/const_eval/value.rs
index 60443b09..17b2b4f6 100644
--- a/compiler/toc_analysis/src/const_eval/value.rs
+++ b/compiler/toc_analysis/src/const_eval/value.rs
@@ -49,6 +49,8 @@ impl PartialEq for ConstValue {
 
 impl Eq for ConstValue {}
 
+fn use_me() { if let crate::ty::TypeKind::Enum(..) = loop {} {} }
+
 impl ConstValue {
     /// Formats the constant value for display.
     pub fn display<DB: crate::ty::db::TypeDatabase + ?Sized>(&self, _db: &DB) -> String {                                                                                      
