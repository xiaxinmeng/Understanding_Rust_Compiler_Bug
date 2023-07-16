patch
diff --git a/build.rs b/build.rs
index b520b62..56fe410 100644
--- a/build.rs
+++ b/build.rs
@@ -51,7 +51,7 @@ fn main() {
         //   time). This can probably be removed in the future
         if !target.contains("wasm32") && !target.contains("nvptx") && !target.starts_with("riscv") {
             #[cfg(feature = "c")]
-            c::compile(&llvm_target);
+            c::compile(&llvm_target, &target);
         }
     }
 
@@ -121,13 +121,24 @@ mod c {
     }
 
     /// Compile intrinsics from the compiler-rt C source code
-    pub fn compile(llvm_target: &[&str]) {
+    pub fn compile(llvm_target: &[&str], target: &String) {
         let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
         let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();
         let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
         let target_vendor = env::var("CARGO_CFG_TARGET_VENDOR").unwrap();
+        let mut use_float = true;
         let cfg = &mut cc::Build::new();
 
+        // Evaluate if files containing floating point code should be considered.
+        if target_arch == "aarch64" {
+            let cflags_key = String::from("CFLAGS_") + &(target.to_owned().replace("-", "_"));
+            if let Ok(cflags_value) = env::var(cflags_key) {
+                if cflags_value.contains("+nofp") || cflags_value.contains("+nosimd") {
+                    use_float = false;
+                }
+            }
+        }
+
         cfg.warnings(false);
 
         if target_env == "msvc" {
@@ -166,34 +177,39 @@ mod c {
             ("__cmpdi2", "cmpdi2.c"),
             ("__ctzdi2", "ctzdi2.c"),
             ("__ctzsi2", "ctzsi2.c"),
-            ("__divdc3", "divdc3.c"),
-            ("__divsc3", "divsc3.c"),
-            ("__divxc3", "divxc3.c"),
-            ("__extendhfsf2", "extendhfsf2.c"),
             ("__int_util", "int_util.c"),
-            ("__muldc3", "muldc3.c"),
-            ("__mulsc3", "mulsc3.c"),
             ("__mulvdi3", "mulvdi3.c"),
             ("__mulvsi3", "mulvsi3.c"),
-            ("__mulxc3", "mulxc3.c"),
-            ("__negdf2", "negdf2.c"),
             ("__negdi2", "negdi2.c"),
-            ("__negsf2", "negsf2.c"),
             ("__negvdi2", "negvdi2.c"),
             ("__negvsi2", "negvsi2.c"),
             ("__paritydi2", "paritydi2.c"),
             ("__paritysi2", "paritysi2.c"),
             ("__popcountdi2", "popcountdi2.c"),
             ("__popcountsi2", "popcountsi2.c"),
-            ("__powixf2", "powixf2.c"),
             ("__subvdi3", "subvdi3.c"),
             ("__subvsi3", "subvsi3.c"),
-            ("__truncdfhf2", "truncdfhf2.c"),
-            ("__truncdfsf2", "truncdfsf2.c"),
-            ("__truncsfhf2", "truncsfhf2.c"),
             ("__ucmpdi2", "ucmpdi2.c"),
         ]);
 
+        if use_float {
+            sources.extend(&[
+                ("__divdc3", "divdc3.c"),
+                ("__divsc3", "divsc3.c"),
+                ("__divxc3", "divxc3.c"),
+                ("__extendhfsf2", "extendhfsf2.c"),
+                ("__muldc3", "muldc3.c"),
+                ("__mulsc3", "mulsc3.c"),
+                ("__mulxc3", "mulxc3.c"),
+                ("__negdf2", "negdf2.c"),
+                ("__negsf2", "negsf2.c"),
+                ("__powixf2", "powixf2.c"),
+                ("__truncdfhf2", "truncdfhf2.c"),
+                ("__truncdfsf2", "truncdfsf2.c"),
+                ("__truncsfhf2", "truncsfhf2.c"),
+            ]);
+        }
+
         // When compiling in rustbuild (the rust-lang/rust repo) this library
         // also needs to satisfy intrinsics that jemalloc or C in general may
         // need, so include a few more that aren't typically needed by
@@ -214,12 +230,15 @@ mod c {
                 ("__ffsti2", "ffsti2.c"),
                 ("__mulvti3", "mulvti3.c"),
                 ("__negti2", "negti2.c"),
-                ("__negvti2", "negvti2.c"),
                 ("__parityti2", "parityti2.c"),
                 ("__popcountti2", "popcountti2.c"),
                 ("__subvti3", "subvti3.c"),
                 ("__ucmpti2", "ucmpti2.c"),
             ]);
+
+            if use_float {
+                sources.extend(&[("__negvti2", "negvti2.c")]);
+            }
         }
 
         if target_vendor == "apple" {
@@ -372,7 +391,7 @@ mod c {
             ]);
         }
 
-        if target_arch == "aarch64" {
+        if target_arch == "aarch64" && use_float {
             sources.extend(&[
                 ("__comparetf2", "comparetf2.c"),
                 ("__extenddftf2", "extenddftf2.c"),
