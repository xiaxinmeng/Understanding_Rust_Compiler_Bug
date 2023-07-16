diff
 failures:
 ---- [ui] ui\nll\issue-52742.rs stdout ----
 diff of stderr:
-	error: unsatisfied lifetime constraints
-	  --> $DIR/issue-52742.rs:25:9
+	error[E0106]: missing lifetime specifiers
+	  --> $DIR/issue-52742.rs:23:10
3	   |
-	LL |     fn take_bar(&mut self, b: Bar<'_>) {
-	   |                 ---------         -- let's call this `'1`
-	   |                 |
-	   |                 has type `&mut Foo<'_, '2>`
-	LL |         self.y = b.z
-	   |         ^^^^^^^^^^^^ requires that `'1` must outlive `'2`
+	LL | impl Foo<'_, '_> {
+	   |          ^^ expected 2 lifetime parameters
10	
11	error: aborting due to previous error
12	
+	For more information about this error, try `rustc --explain E0106`.
13	
