diff
diff --git a/compiler/rustc_driver/Cargo.toml b/compiler/rustc_driver/Cargo.toml
index b88b556d143..26f657d4c1a 100644
--- a/compiler/rustc_driver/Cargo.toml
+++ b/compiler/rustc_driver/Cargo.toml
@@ -40,4 +40,5 @@ winapi = { version = "0.3", features = ["consoleapi", "debugapi", "processenv"]
 
 [features]
 llvm = ['rustc_interface/llvm']
+cranelift = ['rustc_interface/cranelift']
 max_level_info = ['tracing/max_level_info']
diff --git a/compiler/rustc_driver/src/lib.rs b/compiler/rustc_driver/src/lib.rs
index f8ceb94916e..988dcb23ac5 100644
--- a/compiler/rustc_driver/src/lib.rs
+++ b/compiler/rustc_driver/src/lib.rs
@@ -800,6 +800,8 @@ fn unw(x: Option<&str>) -> &str {
         println!("release: {}", unw(util::release_str()));
         if cfg!(feature = "llvm") {
             get_builtin_codegen_backend("llvm")().print_version();
+        } else if cfg!(feature = "cranelift") {
+            get_builtin_codegen_backend("cranelift")().print_version();
         }
     }
 }
@@ -1089,6 +1091,8 @@ pub fn handle_options(args: &[String]) -> Option<getopts::Matches> {
     if cg_flags.iter().any(|x| *x == "passes=list") {
         if cfg!(feature = "llvm") {
             get_builtin_codegen_backend("llvm")().print_passes();
+        } else if cfg!(feature = "cranelift") {
+            get_builtin_codegen_backend("cranelift")().print_passes();
         }
         return None;
     }
diff --git a/compiler/rustc_interface/Cargo.toml b/compiler/rustc_interface/Cargo.toml
index 2481a27dee7..c9f254104e1 100644
--- a/compiler/rustc_interface/Cargo.toml
+++ b/compiler/rustc_interface/Cargo.toml
@@ -29,6 +29,7 @@ rustc_data_structures = { path = "../rustc_data_structures" }
 rustc_codegen_ssa = { path = "../rustc_codegen_ssa" }
 rustc_symbol_mangling = { path = "../rustc_symbol_mangling" }
 rustc_codegen_llvm = { path = "../rustc_codegen_llvm", optional = true }
+rustc_codegen_cranelift = { path = "../rustc_codegen_cranelift", optional = true }
 rustc_hir = { path = "../rustc_hir" }
 rustc_metadata = { path = "../rustc_metadata" }
 rustc_mir = { path = "../rustc_mir" }
@@ -52,3 +53,4 @@ rustc_target = { path = "../rustc_target" }
 
 [features]
 llvm = ['rustc_codegen_llvm']
+cranelift = ['rustc_codegen_cranelift']
diff --git a/compiler/rustc_interface/src/util.rs b/compiler/rustc_interface/src/util.rs
index f34990a1a10..a22ec92ed5b 100644
--- a/compiler/rustc_interface/src/util.rs
+++ b/compiler/rustc_interface/src/util.rs
@@ -370,6 +370,8 @@ pub fn get_builtin_codegen_backend(backend_name: &str) -> fn() -> Box<dyn Codege
     match backend_name {
         #[cfg(feature = "llvm")]
         "llvm" => rustc_codegen_llvm::LlvmCodegenBackend::new,
+        #[cfg(feature = "cranelift")]
+        "cranelift" => rustc_codegen_cranelift::CraneliftCodegenBackend::new,
         _ => get_codegen_sysroot(backend_name),
     }
 }
