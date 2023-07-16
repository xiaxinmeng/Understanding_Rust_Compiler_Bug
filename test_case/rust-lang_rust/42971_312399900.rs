
diff --git a/src/librustc_trans/back/write.rs b/src/librustc_trans/back/write.rs
index cfb5045afe..562d717115 100644
--- a/src/librustc_trans/back/write.rs
+++ b/src/librustc_trans/back/write.rs
@@ -537,7 +537,7 @@ unsafe fn optimize_and_codegen(cgcx: &CodegenContext,
                     Err(_) => return 0,
                 };

-                if let Err(_) = write!(cursor, "{}", demangled) {
+                if let Err(_) = write!(cursor, "{:#}", demangled) {
                     // Possible only if provided buffer is not big enough
                     return 0;
                 }
