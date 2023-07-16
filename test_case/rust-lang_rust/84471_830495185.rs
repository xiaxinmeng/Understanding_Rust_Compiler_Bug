diff
diff --git a/src/bootstrap/tool.rs b/src/bootstrap/tool.rs
index 4f2426648fd..42085a1bdae 100644
--- a/src/bootstrap/tool.rs
+++ b/src/bootstrap/tool.rs
@@ -391,6 +391,10 @@ pub fn command(builder: &Builder<'_>) -> Command {
         // use new syntax, but it should work otherwise.)
         let compiler = builder.compiler(builder.top_stage.saturating_sub(1), builder.config.build);
         let mut cmd = Command::new(builder.ensure(ErrorIndex { compiler }));
+        // because rustdoc depends on rustc_driver, error_index transitively depends on libLLVM.so.
+        // by default libLLVM is only copied in the assemble stage, so copy it explicitly here.
+        let sysroot = builder.sysroot(compiler);
+        crate::dist::maybe_install_llvm_runtime(builder, compiler.host, dbg!(&sysroot));
         add_dylib_path(
             vec![
                 PathBuf::from(&builder.sysroot_libdir(compiler, compiler.host)),
