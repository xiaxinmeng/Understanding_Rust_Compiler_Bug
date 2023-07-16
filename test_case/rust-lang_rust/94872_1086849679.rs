diff
diff --git a/compiler/rustc_target/src/spec/windows_gnu_base.rs b/compiler/rustc_target/src/spec/windows_gnu_base.rs
index 35a52896f6f..a4d7113feab 100644
--- a/compiler/rustc_target/src/spec/windows_gnu_base.rs
+++ b/compiler/rustc_target/src/spec/windows_gnu_base.rs
@@ -6,9 +6,6 @@ pub fn opts() -> TargetOptions {
     pre_link_args.insert(
         LinkerFlavor::Gcc,
         vec![
-            // Tell GCC to avoid linker plugins, because we are not bundling
-            // them with Windows installer, and Rust does its own LTO anyways.
-            "-fno-use-linker-plugin".to_string(),
             // Enable ASLR
             "-Wl,--dynamicbase".to_string(),
             // ASLR will rebase it anyway so leaving that option enabled only leads to confusion
@@ -37,7 +34,6 @@ pub fn opts() -> TargetOptions {
         "-luser32".to_string(),
         "-lkernel32".to_string(),
     ];
-    late_link_args.insert(LinkerFlavor::Gcc, mingw_libs.clone());
     late_link_args.insert(LinkerFlavor::Lld(LldFlavor::Ld), mingw_libs);
     let dynamic_unwind_libs = vec![
         // If any of our crates are dynamically linked then we need to use
@@ -45,7 +41,6 @@ pub fn opts() -> TargetOptions {
         // unwinding across DLL boundaries.
         "-lgcc_s".to_string(),
     ];
-    late_link_args_dynamic.insert(LinkerFlavor::Gcc, dynamic_unwind_libs.clone());
     late_link_args_dynamic.insert(LinkerFlavor::Lld(LldFlavor::Ld), dynamic_unwind_libs);
     let static_unwind_libs = vec![
         // If all of our crates are statically linked then we can get away
@@ -56,7 +51,6 @@ pub fn opts() -> TargetOptions {
         "-lgcc_eh".to_string(),
         "-l:libpthread.a".to_string(),
     ];
-    late_link_args_static.insert(LinkerFlavor::Gcc, static_unwind_libs.clone());
     late_link_args_static.insert(LinkerFlavor::Lld(LldFlavor::Ld), static_unwind_libs);
 
     TargetOptions {
@@ -64,8 +58,8 @@ pub fn opts() -> TargetOptions {
         env: "gnu".to_string(),
         vendor: "pc".to_string(),
         // FIXME(#13846) this should be enabled for windows
-        function_sections: false,
-        linker: Some("gcc".to_string()),
+        function_sections: true,
+        linker: Some("cc".to_string()),
         dynamic_linking: true,
         executables: true,
         dll_prefix: String::new(),
@@ -87,6 +81,8 @@ pub fn opts() -> TargetOptions {
         emit_debug_gdb_scripts: false,
         requires_uwtable: true,
         eh_frame_header: false,
+        no_default_libraries: false,
+        has_thread_local: true,
 
         ..Default::default()
     }
