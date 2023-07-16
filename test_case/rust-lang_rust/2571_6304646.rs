
 fn T_unique_ptr(t: TypeRef) -> TypeRef {
-    const unique_addrspace: uint = 0u;
+    const unique_addrspace: uint = 1u;
     ret llvm::LLVMPointerType(t, unique_addrspace as c_uint);
 }
