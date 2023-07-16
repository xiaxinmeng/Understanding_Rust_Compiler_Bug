diff
--- a/src/test/ui/span/issue-39018.stderr
+++ b/src/test/ui/span/issue-39018.stderr
@@ -2,9 +2,10 @@ error[E0369]: binary operation `+` cannot be applied to type `&'static str`
   --> $DIR/issue-39018.rs:12:13
    |
 12 |     let x = "Hello " + "World!";
-   |             ^^^^^^^^ `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left: `"Hello ".to_owned()`
-   |
-   = note: `+` can't be used to concatenate two `&str` strings
+   |             ^^^^^^^^-----------
+   |             |
+   |             `+` can't be used to concatenate two `&str` strings
+   |             `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left. `"Hello ".to_owned()`
 
 error[E0369]: binary operation `+` cannot be applied to type `World`
   --> $DIR/issue-39018.rs:17:13
