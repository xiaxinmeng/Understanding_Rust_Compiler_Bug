patch
diff --git a/src/librustc_trans/base.rs b/src/librustc_trans/base.rs
index 4cdde24..6c657ef 100644
--- a/src/librustc_trans/base.rs
+++ b/src/librustc_trans/base.rs
@@ -455,6 +455,15 @@ pub fn load_fat_ptr<'a, 'tcx>(
         b.load(ptr)
     }; 
 
+    let ty = type_of::fat_ptr_base_ty(b.ccx, t).element_type();
+    if ty.kind() == llvm::TypeKind::Struct {
+        if ty.is_packed() {
+            unsafe {
+                llvm::LLVMSetAlignment(ptr, 1);
+            }
+        }
+    }
+
     // FIXME: emit metadata on `meta`.
     let meta = b.load(get_meta(b, src));
 
diff --git a/src/librustc_trans/builder.rs b/src/librustc_trans/builder.rs
index cf7f3e9..f7f89b8 100644
--- a/src/librustc_trans/builder.rs
+++ b/src/librustc_trans/builder.rs
@@ -514,7 +514,20 @@ impl<'a, 'tcx> Builder<'a, 'tcx> {
     pub fn load(&self, ptr: ValueRef) -> ValueRef {
         self.count_insn("load");
         unsafe {
-            llvm::LLVMBuildLoad(self.llbuilder, ptr, noname())
+            let load = llvm::LLVMBuildLoad(self.llbuilder, ptr, noname());
+
+            let mut ty = val_ty(ptr);
+            // Strip off pointers
+            while ty.kind() == llvm::TypeKind::Pointer {
+                ty = ty.element_type();
+            }
+
+            if ty.kind() == llvm::TypeKind::Struct {
+                if ty.is_packed() {
+                    llvm::LLVMSetAlignment(load, 1);
+                }
+            }
+            load
         }
     }  
 
@@ -577,6 +590,18 @@ impl<'a, 'tcx> Builder<'a, 'tcx> {
             let store = llvm::LLVMBuildStore(self.llbuilder, val, ptr);
             if let Some(align) = align {
                 llvm::LLVMSetAlignment(store, align as c_uint);
+            } else {
+                let mut ty = val_ty(ptr);
+                // Strip off pointers
+                while ty.kind() == llvm::TypeKind::Pointer {
+                    ty = ty.element_type();
+                }
+
+                if ty.kind() == llvm::TypeKind::Struct {
+                    if ty.is_packed() {
+                        llvm::LLVMSetAlignment(store, 1);
+                    }
+                }
             }
             store
         }
