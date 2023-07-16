diff
--- a/compiler/rustc_codegen_ssa/src/back/metadata.rs
+++ b/compiler/rustc_codegen_ssa/src/back/metadata.rs
@@ -140,6 +140,14 @@ pub(crate) fn create_object_file(sess: &Session) -> Option<write::Object<'static
     };
 
     let mut file = write::Object::new(binary_format, architecture, endianness);
+    if sess.target.llvm_target.ends_with("-macabi") {
+        let version = write::MachOBuildVersion {
+            platform: object::macho::PLATFORM_MACCATALYST,
+            minos: 0x000E0000,
+            sdk: 0x00100200,
+        };
+        file.set_macho_build_version(version);
+    }
     let e_flags = match architecture {
         Architecture::Mips => {
             let arch = match sess.target.options.cpu.as_ref() {
