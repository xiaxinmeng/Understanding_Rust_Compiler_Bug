diff
 ---- [ui] ui\mut\mutable-class-fields.rs#mir stdout ----
 diff of stderr:
-	error[E0384]: cannot assign twice to immutable variable `nyan`
+	error[E0594]: cannot assign to `nyan.how_hungry`, as `nyan` is not declared as mutable
2	  --> $DIR/mutable-class-fields.rs:28:3
3	   |
4	LL |   let nyan : cat = cat(52, 99);
-	   |       ----
-	   |       |
-	   |       first assignment to `nyan`
-	   |       consider changing this to `mut nyan`
+	   |       ---- help: consider changing this to be mutable: `mut nyan`
9	LL |   nyan.how_hungry = 0; //[ast]~ ERROR cannot assign
-	   |   ^^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
+	   |   ^^^^^^^^^^^^^^^^^^^ cannot assign
11	
12	error: aborting due to previous error
13	
-	For more information about this error, try `rustc --explain E0384`.
+	For more information about this error, try `rustc --explain E0594`.
15	
