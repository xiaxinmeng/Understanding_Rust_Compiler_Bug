diff
 ---- [ui] ui/imports/local-modularized-tricky-fail-3.rs stdout ----
 diff of stderr:

1	error: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
-	  --> $DIR/local-modularized-tricky-fail-3.rs:30:5
+	  --> $DIR/local-modularized-tricky-fail-3.rs:25:9
3	   |
-	LL |     ::exported!();
-	   |     ^^^^^^^^^^
+	LL |     use exported;
+	   |         ^^^^^^^^
6	   |
7	note: the macro is defined here
8	  --> $DIR/local-modularized-tricky-fail-3.rs:17:5

16	   |   ------------------- in this macro invocation
17	
18	error: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
-	  --> $DIR/local-modularized-tricky-fail-3.rs:25:9
+	  --> $DIR/local-modularized-tricky-fail-3.rs:30:5
20	   |
-	LL |     use exported;
-	   |         ^^^^^^^^
+	LL |     ::exported!();
+	   |     ^^^^^^^^^^
23	   |
24	note: the macro is defined here
25	  --> $DIR/local-modularized-tricky-fail-3.rs:17:5
