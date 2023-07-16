
--- rust-1.62.1/compiler/rustc_llvm/build.rs.orig       2022-07-22 10:08:33.937632588 +0800
+++ rust-1.62.1/compiler/rustc_llvm/build.rs    2022-07-22 09:44:45.726315852 +0800
@@ -355,7 +355,7 @@
             if target.contains("windows") {
                 println!("cargo:rustc-link-lib=static:-bundle={}", stdcppname);
             } else {
-                println!("cargo:rustc-link-lib=static={}", stdcppname);
+                println!("cargo:rustc-link-lib=static:-bundle,+whole-archive={}", stdcppname);
             }
         } else if cxxflags.contains("stdlib=libc++") {
             println!("cargo:rustc-link-lib=c++");

