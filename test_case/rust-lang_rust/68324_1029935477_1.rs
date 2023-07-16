diff
diff --git a/build-utilities/src/lib.rs b/build-utilities/src/lib.rs
index 9bc5f7f4..27b6ba19 100644
--- a/build-utilities/src/lib.rs
+++ b/build-utilities/src/lib.rs
@@ -1,4 +1,4 @@
-#![feature(option_unwrap_none)]
+// #![feature(option_unwrap_none)]
 
 use std::{fs, path};
 use std::io::ErrorKind;
@@ -31,6 +31,6 @@ impl<Str:AsRef<str>> GithubRelease<Str> {
         let result      = fs::remove_file(&file);
         let error       = result.err();
         let fatal_error = error.filter(|err| err.kind() != ErrorKind::NotFound);
-        fatal_error.unwrap_none();
+        assert!(fatal_error.is_none());
     }
 }
diff --git a/lib/core/Cargo.toml b/lib/core/Cargo.toml
index 8aeda162..25fad557 100644
--- a/lib/core/Cargo.toml
+++ b/lib/core/Cargo.toml
@@ -26,7 +26,7 @@ shapely                    = { version = "0.1.0"  , path = "../shapely/impl"  }
 
 bit_field                  = { version = "0.10.0" }
 console_error_panic_hook   = { version = "0.1.6"  }
-enum_dispatch              = { version = "0.2.0"  }
+enum_dispatch              = { version = "0.3.0"  }
 failure                    = { version = "0.1.5"  }
 Inflector                  = { version = "0.11.4" }
 itertools                  = { version = "0.8"    }
diff --git a/lib/core/msdf-sys/Cargo.toml b/lib/core/msdf-sys/Cargo.toml
index 59f13131..5d77db8e 100644
--- a/lib/core/msdf-sys/Cargo.toml
+++ b/lib/core/msdf-sys/Cargo.toml
@@ -15,8 +15,8 @@ basegl-prelude = { version = "0.1.0", path="../../prelude" }
 
 [dev-dependencies]
 wasm-bindgen-test          = "0.3.3"
-futures                    = ""
+futures                    = "*"
 basegl-core-embedded-fonts = { version = "0.1.0", path="../embedded-fonts" }
 
 [build-dependencies]
-basegl-build-utilities = { version = "0.1.0", path="../../../build-utilities" }
\ No newline at end of file
+basegl-build-utilities = { version = "0.1.0", path="../../../build-utilities" }
diff --git a/lib/logger/src/lib.rs b/lib/logger/src/lib.rs
index 420dd368..dd9900c4 100644
--- a/lib/logger/src/lib.rs
+++ b/lib/logger/src/lib.rs
@@ -1,5 +1,5 @@
 #![feature(trait_alias)]
-#![feature(set_stdio)]
+// #![feature(set_stdio)]
 
 use std::fmt::Debug;
 use wasm_bindgen::JsValue;
diff --git a/lib/system/web/src/lib.rs b/lib/system/web/src/lib.rs
index e4a39c39..ce4a4f5c 100644
--- a/lib/system/web/src/lib.rs
+++ b/lib/system/web/src/lib.rs
@@ -1,5 +1,5 @@
 #![feature(trait_alias)]
-#![feature(set_stdio)]
+// #![feature(set_stdio)]
 #![feature(arbitrary_self_types)]
 
 pub mod resize_observer;
@@ -305,12 +305,12 @@ fn _print(msg: &str) -> std::io::Result<()> {
 
 pub fn set_stdout() {
     let printer = Printer::new(_print, true);
-    std::io::set_print(Some(Box::new(printer)));
+    // std::io::set_print(Some(Box::new(printer)));
 }
 
 pub fn set_stdout_unbuffered() {
     let printer = Printer::new(_print, false);
-    std::io::set_print(Some(Box::new(printer)));
+    // std::io::set_print(Some(Box::new(printer)));
 }
 
 #[wasm_bindgen(inline_js = "
