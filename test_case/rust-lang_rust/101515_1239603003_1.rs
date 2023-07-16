diff
--- a/src/test/ui/parser/issue-101477-let.stderr
+++ b/src/test/ui/parser/issue-101477-let.stderr
@@ -2,7 +2,13 @@ error: unexpected `==`
   --> $DIR/issue-101477-let.rs:4:11
    |
 LL |     let x == 2;
-   |           ^^ help: try using `=` instead `==`
+   |           ^^ help: replace `==` with a equal symbol: `=`

-error: aborting due to previous error
+error: expected expression, found `==`
+  --> $DIR/issue-101477-let.rs:4:11
+   |
+LL |     let x == 2;
+   |           ^^ expected expression
+
+error: aborting due to 2 previous errors
