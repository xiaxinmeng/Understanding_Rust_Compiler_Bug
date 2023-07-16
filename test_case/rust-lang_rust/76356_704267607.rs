
--- expected_show_coverage.generics.txt	2020-10-06 03:36:29.936045671 +0000
+++ /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports-deadcode/coverage-reports-deadcode/actual_show_coverage.generics.txt	2020-10-06 05:21:51.420743607 +0000
@@ -29,12 +29,12 @@
    18|      2|        println!("BOOM times {}!!!", self.strength);
    19|      2|    }
   ------------------
-  | <generics::Firework<i32> as core::ops::drop::Drop>::drop:
+  | <generics::Firework<i32> as core[a7a74cee373f048]::ops::drop::Drop>::drop:
   |   17|      1|    fn drop(&mut self) {
   |   18|      1|        println!("BOOM times {}!!!", self.strength);
   |   19|      1|    }
   ------------------
-  | <generics::Firework<f64> as core::ops::drop::Drop>::drop:
+  | <generics::Firework<f64> as core[a7a74cee373f048]::ops::drop::Drop>::drop:
   |   17|      1|    fn drop(&mut self) {
   |   18|      1|        println!("BOOM times {}!!!", self.strength);
   |   19|      1|    }

------------------------------------------
stderr:
------------------------------------------
Error: 1
Error: 1
diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/generics.rs
make: *** [../coverage-reports-base/Makefile:45: generics] Error 1

------------------------------------------

failures:
    [run-make] run-make-fulldeps/coverage-reports-base
    [run-make] run-make-fulldeps/coverage-reports-deadcode
