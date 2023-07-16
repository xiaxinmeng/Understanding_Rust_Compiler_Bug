diff
diff --git a/src/bootstrap/dist.rs b/src/bootstrap/dist.rs
index 9092deeac461..73cda4b436d4 100644
--- a/src/bootstrap/dist.rs
+++ b/src/bootstrap/dist.rs
@@ -1756,6 +1740,7 @@ impl Step for HashSign {
         cmd.arg(builder.package_vers(&builder.release_num("cargo")));
         cmd.arg(builder.package_vers(&builder.release_num("rls")));
         cmd.arg(builder.package_vers(&builder.release_num("rustfmt")));
+        cmd.arg(builder.llvm_tools_vers());
         cmd.arg(addr);
 
         builder.create_dir(&distdir(builder));
