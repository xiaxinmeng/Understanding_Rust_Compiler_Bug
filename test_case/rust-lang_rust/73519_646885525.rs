
$ git diff src/bootstrap/
diff --git a/src/bootstrap/compile.rs b/src/bootstrap/compile.rs
index c09b73b0420..630f9ecbe25 100644
--- a/src/bootstrap/compile.rs
+++ b/src/bootstrap/compile.rs
@@ -589,6 +589,7 @@ impl Step for RustcLink {
             &builder.sysroot_libdir(target_compiler, compiler.host),
             &librustc_stamp(builder, compiler, target),
         );
+        builder.ensure(Assemble { target_compiler });
     }
 }
 