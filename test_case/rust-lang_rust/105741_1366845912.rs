patch
diff --git a/src/bootstrap/compile.rs b/src/bootstrap/compile.rs
index ae5dfdea19f..c8aa8c99163 100644
--- a/src/bootstrap/compile.rs
+++ b/src/bootstrap/compile.rs
@@ -362,12 +362,7 @@ pub fn std_cargo(builder: &Builder<'_>, target: TargetSelection, stage: u32, car
     // built with bitcode so that the produced rlibs can be used for both LTO
     // builds (which use bitcode) and non-LTO builds (which use object code).
     // So we override the override here!
-    //
-    // But we don't bother for the stage 0 compiler because it's never used
-    // with LTO.
-    if stage >= 1 {
-        cargo.rustflag("-Cembed-bitcode=yes");
-    }
+    cargo.rustflag("-Cembed-bitcode=yes");
 
     // By default, rustc does not include unwind tables unless they are required
     // for a particular target. They are not required by RISC-V targets, but
