diff
diff --git a/compiler/rustc_mir_transform/src/inline.rs b/compiler/rustc_mir_transform/src/inline.rs
index bf670c5c26a..02a401e7c63 100644
--- a/compiler/rustc_mir_transform/src/inline.rs
+++ b/compiler/rustc_mir_transform/src/inline.rs
@@ -343,8 +343,8 @@ fn check_codegen_attributes(
             InlineAttr::Never => return Err("never inline hint"),
             InlineAttr::Always | InlineAttr::Hint => {}
             InlineAttr::None => {
-                if self.tcx.sess.mir_opt_level() <= 2 {
-                    return Err("at mir-opt-level=2, only #[inline] is inlined");
+                if self.tcx.sess.mir_opt_level() <= 1 {
+                    return Err("at mir-opt-level=1, only #[inline] is inlined");
                 }
             }
         }
