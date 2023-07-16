diff
--- a/src/test/ui/macros/macros-nonfatal-errors.stderr
+++ b/src/test/ui/macros/macros-nonfatal-errors.stderr
@@ -1,4 +1,4 @@
-error[E0665]: `Default` cannot be derived for enums, only structs
+error: `Default` cannot be derived for enums, only structs
   --> $DIR/macros-nonfatal-errors.rs:20:10
    |
 LL | #[derive(Default)] //~ ERROR
@@ -96,5 +96,4 @@ LL |     trace_macros!(invalid); //~ ERROR
 
 error: aborting due to 15 previous errors
 
-Some errors occurred: E0658, E0665.
-For more information about an error, try `rustc --explain E0658`.
+For more information about this error, try `rustc --explain E0658`.
