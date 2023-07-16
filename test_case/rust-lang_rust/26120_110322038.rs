 diff
diff --git a/src/doc/trpl/functions.md b/src/doc/trpl/functions.md
--- a/src/doc/trpl/functions.md
+++ b/src/doc/trpl/functions.md
@@ -146,3 +146,5 @@
  expression, although its value is not particularly useful. Unlike other
  languages where an assignment evaluates to the assigned value (e.g. `5` in the
- previous example), in Rust the value of an assignment is an empty tuple `()`:
+ previous example), in Rust the value of an assignment is an empty tuple `()`
+ because the value can have (just one owner)[ownership.html], and any other
+ value would be too surprising:
