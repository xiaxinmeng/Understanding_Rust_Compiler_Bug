
--- expected_show_coverage.partial_eq.txt	2021-02-11 03:54:17.271247325 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-reports/coverage-reports/actual_show_coverage.partial_eq.txt	2021-02-11 04:39:04.423096295 +0000
@@ -2,11 +2,11 @@
     2|       |// structure of this test.
     3|       |
     4|      2|#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
-                       ^0            ^0      ^0 ^0  ^1       ^0 ^0^0
+                       ^0            ^0      ^0 ^0  ^1       ^1 ^0^0
     5|       |pub struct Version {
     6|       |    major: usize,
-    7|      1|    minor: usize, // Count: 1 - `PartialOrd` compared `minor` values in 3.2.1 vs. 3.3.0
-    8|      0|    patch: usize, // Count: 0 - `PartialOrd` was determined by `minor` (2 < 3)
+    7|       |    minor: usize, // Count: 1 - `PartialOrd` compared `minor` values in 3.2.1 vs. 3.3.0
+    8|       |    patch: usize, // Count: 0 - `PartialOrd` was determined by `minor` (2 < 3)
     9|       |}
    10|       |
    11|       |impl Version {
