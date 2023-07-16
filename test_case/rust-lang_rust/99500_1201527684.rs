diff
--- a/compiler/rustc_codegen_ssa/src/back/link.rs
+++ b/compiler/rustc_codegen_ssa/src/back/link.rs
@@ -2086,7 +2086,9 @@ fn add_order_independent_options(
     // Make the binary compatible with data execution prevention schemes.
     cmd.add_no_exec();

-    if crt_objects_fallback {
+    // FIXME: we are currently missing some infra here (per-linker-flavor CRT objects),
+    // so Fuchsia has to be special-cased.
+    if crt_objects_fallback && !(sess.target.os == "fuchsia" && flavor == LinkerFlavor::Gcc) {
         cmd.no_crt_objects();
     }
