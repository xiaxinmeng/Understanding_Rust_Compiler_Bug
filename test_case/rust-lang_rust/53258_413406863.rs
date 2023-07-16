diff
 ---- [ui] ui\immut-function-arguments.rs#mir stdout ----
 diff of stderr:
-	error[E0384]: cannot assign to immutable argument `y`
+	error[E0594]: cannot assign to `*y`, as `y` is not declared as mutable
2	  --> $DIR/immut-function-arguments.rs:15:5
3	   |
4	LL | fn f(y: Box<isize>) {
-	   |      - consider changing this to `mut y`
+	   |      - help: consider changing this to be mutable: `mut y`
6	LL |     *y = 5; //[ast]~ ERROR cannot assign
-	   |     ^^^^^^ cannot assign to immutable argument
+	   |     ^^^^^^ cannot assign
8	
-	error[E0384]: cannot assign to immutable argument `q`
+	error[E0594]: cannot assign to `*q`, as `q` is not declared as mutable
10	  --> $DIR/immut-function-arguments.rs:20:35
11	   |
12	LL |     let _frob = |q: Box<isize>| { *q = 2; }; //[ast]~ ERROR cannot assign
-	   |                  -                ^^^^^^ cannot assign to immutable argument
+	   |                  -                ^^^^^^ cannot assign
14	   |                  |
-	   |                  consider changing this to `mut q`
+	   |                  help: consider changing this to be mutable: `mut q`
16	
17	error: aborting due to 2 previous errors
18	
-	For more information about this error, try `rustc --explain E0384`.
+	For more information about this error, try `rustc --explain E0594`.
20	
