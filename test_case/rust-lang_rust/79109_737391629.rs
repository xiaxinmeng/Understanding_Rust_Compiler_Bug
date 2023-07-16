
diff -u --strip-trailing-cr expected_show_coverage.uses_crate.txt "/d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-reports/coverage-reports"/actual_show_coverage.uses_crate.txt || \
	( grep -q '^\/\/ ignore-llvm-cov-show-diffs' ../coverage/uses_crate.rs && \
		>&2 echo 'diff failed, but suppressed with `// ignore-llvm-cov-show-diffs` in ../coverage/uses_crate.rs' \
	) || \
	( >&2 echo 'diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/uses_crate.rs'; \
		false \
	)
--- expected_show_coverage.uses_crate.txt	2020-12-02 09:56:05.285910400 +0000
+++ /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-reports/coverage-reports/actual_show_coverage.uses_crate.txt	2020-12-02 10:56:42.639202900 +0000
@@ -1,4 +1,16 @@
-../coverage/used_crate/mod.rs:
+..\coverage\uses_crate.rs:
+    1|       |#![allow(unused_assignments, unused_variables)]
+    2|       |
+    3|       |mod used_crate;
+    4|       |
+    5|      1|fn main() {
+    6|      1|    used_crate::used_function();
+    7|      1|    let some_vec = vec![1, 2, 3, 4];
+    8|      1|    used_crate::used_generic_function(&some_vec);
+    9|      1|    used_crate::used_twice_generic_function(some_vec);
+   10|      1|}
+
+..\coverage\used_crate\mod.rs:
     1|       |#![allow(unused_assignments, unused_variables)]
     2|       |
     3|       |use std::fmt::Debug;
@@ -55,15 +67,3 @@
    42|      0|    }
    43|      0|}
 
-../coverage/uses_crate.rs:
-    1|       |#![allow(unused_assignments, unused_variables)]
-    2|       |
-    3|       |mod used_crate;
-    4|       |
-    5|      1|fn main() {
-    6|      1|    used_crate::used_function();
-    7|      1|    let some_vec = vec![1, 2, 3, 4];
-    8|      1|    used_crate::used_generic_function(&some_vec);
-    9|      1|    used_crate::used_twice_generic_function(some_vec);
-   10|      1|}
-
make[1]: Leaving directory '/d/a/rust/rust/src/test/run-make-fulldeps/coverage-reports'
