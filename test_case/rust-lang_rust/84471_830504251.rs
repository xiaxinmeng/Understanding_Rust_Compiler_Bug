diff
diff --git a/src/bootstrap/tool.rs b/src/bootstrap/tool.rs
index 4f2426648fd..a7c7ff9730b 100644
--- a/src/bootstrap/tool.rs
+++ b/src/bootstrap/tool.rs
@@ -391,10 +391,18 @@ pub fn command(builder: &Builder<'_>) -> Command {
         // use new syntax, but it should work otherwise.)
         let compiler = builder.compiler(builder.top_stage.saturating_sub(1), builder.config.build);
         let mut cmd = Command::new(builder.ensure(ErrorIndex { compiler }));
+        // because rustdoc depends on rustc_driver, error_index transitively depends on libLLVM.so.
+        // by default libLLVM is only copied in the assemble stage, so copy it explicitly here.
+        // NOTE: this does *not* use `builder.sysroot(compiler)` because that gives `stage0-sysroot/` for the stage0 compiler,
+        // but we want `stage0/` to be consistent with the dynamic load path.
+        let rustc_libdir = builder.rustc_libdir(compiler);
+        let sysroot = rustc_libdir.parent().unwrap();
+        crate::dist::maybe_install_llvm_runtime(builder, compiler.host, dbg!(&sysroot));
+
         add_dylib_path(
             vec![
-                PathBuf::from(&builder.sysroot_libdir(compiler, compiler.host)),
-                PathBuf::from(builder.rustc_libdir(compiler)),
+                builder.sysroot_libdir(compiler, compiler.host).to_path_buf(),
+                rustc_libdir,
             ],
             &mut cmd,
         );
