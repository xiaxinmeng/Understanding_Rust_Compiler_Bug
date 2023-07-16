 diff
diff --git a/src/librustc_llvm/lib.rs b/src/librustc_llvm/lib.rs
index c1b909b..71f760d 100644
--- a/src/librustc_llvm/lib.rs
+++ b/src/librustc_llvm/lib.rs
@@ -965,8 +965,8 @@ extern {
                                           Name: *const c_char,
                                           Value: *const c_char);
     pub fn LLVMRemoveFunctionAttrString(Fn: ValueRef, index: c_uint, Name: *const c_char);
-    pub fn LLVMGetFunctionAttr(Fn: ValueRef) -> c_ulonglong;
-    pub fn LLVMRemoveFunctionAttr(Fn: ValueRef, val: c_ulonglong);
+    pub fn LLVMGetFunctionAttr(Fn: ValueRef) -> c_uint;
+    pub fn LLVMRemoveFunctionAttr(Fn: ValueRef, val: c_uint);

     /* Operations on parameters */
     pub fn LLVMCountParams(Fn: ValueRef) -> c_uint;
diff --git a/src/librustc_trans/trans/attributes.rs b/src/librustc_trans/trans/attributes.rs
index 4ea920c5..a043d58 100644
--- a/src/librustc_trans/trans/attributes.rs
+++ b/src/librustc_trans/trans/attributes.rs
@@ -29,7 +29,7 @@ pub fn inline(val: ValueRef, inline: InlineAttr) {
                        llvm::Attribute::AlwaysInline |
                        llvm::Attribute::NoInline;
             unsafe {
-                llvm::LLVMRemoveFunctionAttr(val, attr.bits() as c_ulonglong)
+                llvm::LLVMRemoveFunctionAttr(val, attr.bits() as c_uint)
             }
         },
     };
@@ -44,7 +44,7 @@ pub fn emit_uwtable(val: ValueRef, emit: bool) {
         unsafe {
             llvm::LLVMRemoveFunctionAttr(
                 val,
-                llvm::Attribute::UWTable.bits() as c_ulonglong,
+                llvm::Attribute::UWTable.bits() as c_uint,
             );
         }
     }
@@ -57,7 +57,7 @@ pub fn unwind(val: ValueRef, can_unwind: bool) {
         unsafe {
             llvm::LLVMRemoveFunctionAttr(
                 val,
-                llvm::Attribute::NoUnwind.bits() as c_ulonglong,
+                llvm::Attribute::NoUnwind.bits() as c_uint,
             );
         }
     } else {
@@ -75,7 +75,7 @@ pub fn set_optimize_for_size(val: ValueRef, optimize: bool) {
         unsafe {
             llvm::LLVMRemoveFunctionAttr(
                 val,
-                llvm::Attribute::OptimizeForSize.bits() as c_ulonglong,
+                llvm::Attribute::OptimizeForSize.bits() as c_uint,
             );
         }
     }
@@ -88,7 +88,7 @@ pub fn naked(val: ValueRef, is_naked: bool) {
         llvm::SetFunctionAttribute(val, llvm::Attribute::Naked);
     } else {
         unsafe {
-            llvm::LLVMRemoveFunctionAttr(val, llvm::Attribute::Naked.bits() as c_ulonglong);
+            llvm::LLVMRemoveFunctionAttr(val, llvm::Attribute::Naked.bits() as c_uint);
         }
     }
 }
