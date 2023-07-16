diff
diff --git a/src/bootstrap/tool.rs b/src/bootstrap/tool.rs
index 7c37621d417..0f20b74bcb4 100644
--- a/src/bootstrap/tool.rs
+++ b/src/bootstrap/tool.rs
@@ -250,6 +250,15 @@ pub fn prepare_tool_cargo(
         }
     }
 
+    if path.ends_with("rust-analyzer") {
+        // RA wants to load proc macros at runtime for one of its tests. But by default, proc macros
+        // are built with the beta proc_macro, not the in-tree proc_macro. Explicitly override this
+        // in the rustc shim.
+        // This is ok because rust-analyzer is `Mode::ToolStd`, so proc_macro is always built before.
+        cargo.rustflag("--sysroot");
+        cargo.rustflag(builder.sysroot(compiler).as_os_str().to_str().unwrap());
+    }
+
     // clippy tests need to know about the stage sysroot. Set them consistently while building to
     // avoid rebuilding when running tests.
     cargo.env("SYSROOT", builder.sysroot(compiler));
