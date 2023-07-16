console
--- a/src/test/ui/argument-suggestions/missing_arguments.stderr
+++ b/src/test/ui/argument-suggestions/missing_arguments.stderr
@@ -210,10 +210,7 @@ error[E0061]: this function takes 3 arguments but 1 argument was supplied
   --> $DIR/missing_arguments.rs:30:3
    |
 LL |   three_diff(          1.0          );
-   |   ^^^^^^^^^^-------------------------
-   |             |          |
-   |             |          an argument of type `i32` is missing
-   |             an argument of type `&str` is missing
+   |   ^^^^^^^^^^------------------------- two arguments of type `i32` and `f32` are missing
    |
 note: function defined here
   --> $DIR/missing_arguments.rs:5:4
@@ -222,8 +219,8 @@ LL | fn three_diff(_a: i32, _b: f32, _c: &str) {}
    |    ^^^^^^^^^^ -------  -------  --------
 help: provide the arguments
    |
-LL |   three_diff(/* i32 */, 1.0, /* &str */);
-   |   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
+LL |   three_diff(/* i32 */, /* f32 */, /* &str */);
+   |   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
