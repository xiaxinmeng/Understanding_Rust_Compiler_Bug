
--- rustc-1.26.2-src/src/librustc_trans/cabi_sparc64.rs
+++ rustc-1.26.2-src/src/librustc_trans/cabi_sparc64.rs
@@ -51,16 +51,7 @@
     let size = ret.layout.size;
     let bits = size.bits();
     if bits <= 256 {
-        let unit = if bits <= 8 {
-            Reg::i8()
-        } else if bits <= 16 {
-            Reg::i16()
-        } else if bits <= 32 {
-            Reg::i32()
-        } else {
-            Reg::i64()
-        };
-
+        let unit = Reg::i64();
         ret.cast_to(Uniform {
             unit,
             total: size		 
