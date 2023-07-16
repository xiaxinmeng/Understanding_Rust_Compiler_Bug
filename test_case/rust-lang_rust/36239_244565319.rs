
diff --git a/src/librustc_trans/abi.rs b/src/librustc_trans/abi.rs
index 3a7fde6..fa26ce2 100644
--- a/src/librustc_trans/abi.rs
+++ b/src/librustc_trans/abi.rs
@@ -315,6 +317,12 @@ impl FnType {
                 if ty.is_integral() {
                     arg.signedness = Some(ty.is_signed());
                 }
+                if let ty::TyEnum(..) = ty.sty {
+                    if arg.ty.kind() == llvm::Integer {
+                        let repr = adt::represent_type(ccx, ty);
+                        arg.signedness = Some(adt::is_discr_signed(&repr));
+                    }
+                }
                 if llsize_of_real(ccx, arg.ty) == 0 {
                     // For some forsaken reason, x86_64-pc-windows-gnu
                     // doesn't ignore zero-sized struct arguments.
