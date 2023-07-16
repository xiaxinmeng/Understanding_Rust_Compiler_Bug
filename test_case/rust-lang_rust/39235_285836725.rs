
diff -ruN rustc-1.15.1-src/src/bootstrap/install.rs rustc-1.15.1-src_new/src/bootstrap/install.rs
--- rustc-1.15.1-src/src/bootstrap/install.rs	2017-02-09 01:37:48.000000000 +0000
+++ rustc-1.15.1-src_new/src/bootstrap/install.rs	2017-03-11 02:06:21.676817872 +0000
@@ -37,6 +37,12 @@
         install_sh(&build, "docs", "rust-docs", stage, host, prefix,
                    &docdir, &libdir, &mandir, &empty_dir);
     }
+    
+    for target in build.config.target.iter().filter(|target| *target != host) {
+        install_sh(&build, "std", "rust-std", stage, target, prefix,
+                   &docdir, &libdir, &mandir, &empty_dir);
+    }
+
     install_sh(&build, "std", "rust-std", stage, host, prefix,
                &docdir, &libdir, &mandir, &empty_dir);
     install_sh(&build, "rustc", "rustc", stage, host, prefix,
