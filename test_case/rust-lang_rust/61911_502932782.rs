diff
--- a/src/bootstrap/compile.rs
+++ b/src/bootstrap/compile.rs
@@ -179,8 +179,9 @@ pub fn std_cargo(builder: &Builder<'_>,
     // `compiler-rt` is located.
     let compiler_builtins_root = builder.src.join("src/llvm-project/compiler-rt");
     let compiler_builtins_c_feature = if compiler_builtins_root.exists() {
-        cargo.env("RUST_COMPILER_RT_ROOT", &compiler_builtins_root);
-        " compiler-builtins-c".to_string()
+        //cargo.env("RUST_COMPILER_RT_ROOT", &compiler_builtins_root);
+        //" compiler-builtins-c".to_string()
+        String::new() // HACK! the c builtins don't build for me
     } else {
         String::new()
     };
