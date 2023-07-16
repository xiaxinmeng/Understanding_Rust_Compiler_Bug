diff
diff --git a/src/bootstrap/test.rs b/src/bootstrap/test.rs
index 86d940cd733..96a94d43428 100644
--- a/src/bootstrap/test.rs
+++ b/src/bootstrap/test.rs
@@ -393,6 +392,12 @@ fn run(self, builder: &Builder<'_>) {
             );
             cargo.arg("--").arg("miri").arg("setup");
 
+            // Make sure we can access all the dynamic libraries.
+            let mut dylib_path = dylib_path();
+            let ci_llvm_lib = builder.out.join(&*host.triple).join("ci-llvm/lib");
+            dylib_path.insert(0, ci_llvm_lib);
+            cargo.env(dylib_path_var(), env::join_paths(&dylib_path).unwrap());
+            eprintln!("{} = {:?}", dylib_path_var(), env::join_paths(&dylib_path).unwrap());
             // Tell `cargo miri setup` where to find the sources.
             cargo.env("XARGO_RUST_SRC", builder.src.join("library"));
             // Tell it where to find Miri.
@@ -442,6 +447,7 @@ fn run(self, builder: &Builder<'_>) {
                 SourceType::Submodule,
                 &[],
             );
+            cargo.env(dylib_path_var(), env::join_paths(&dylib_path).unwrap());
 
             // miri tests need to know about the stage sysroot
             cargo.env("MIRI_SYSROOT", miri_sysroot);
@@ -450,7 +456,7 @@ fn run(self, builder: &Builder<'_>) {
 
             cargo.arg("--").args(builder.config.cmd.test_args());
 
-            cargo.add_rustc_lib_path(builder, compiler);
+            //cargo.add_rustc_lib_path(builder, compiler);
 
             if !try_run(builder, &mut cargo.into()) {
                 return;
