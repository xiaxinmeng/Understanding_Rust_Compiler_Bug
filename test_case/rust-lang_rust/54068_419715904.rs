
---- [ui (nll)] ui\borrowck\borrowck-multiple-captures.rs stdout ----
diff of stderr:
26	LL |     borrow(&*p2);
27	   |            ---- borrow later used here
28	
-	error[E0382]: use of moved value: `x2`
+	error[E0382]: use of moved value: `x1`
30	  --> $DIR/borrowck-multiple-captures.rs:35:19
31	   |
-	LL |     drop(x2);
+	LL |     drop(x1);
33	   |          -- value moved here
+	...
34	LL |     thread::spawn(move|| {
35	   |                   ^^^^^^ value used here after move
36	LL |         drop(x1); //~ ERROR capture of moved value: `x1`
-	LL |         drop(x2); //~ ERROR capture of moved value: `x2`
38	   |              -- use occurs due to use in closure
39	   |
-	   = note: move occurs because `x2` has type `std::boxed::Box<i32>`, which does not implement the `Copy` trait
+	   = note: move occurs because `x1` has type `std::boxed::Box<i32>`, which does not implement the `Copy` trait
41	
-	error[E0382]: use of moved value: `x1`
+	error[E0382]: use of moved value: `x2`
43	  --> $DIR/borrowck-multiple-captures.rs:35:19
44	   |
-	LL |     drop(x1);
+	LL |     drop(x2);
46	   |          -- value moved here
-	...
48	LL |     thread::spawn(move|| {
49	   |                   ^^^^^^ value used here after move
50	LL |         drop(x1); //~ ERROR capture of moved value: `x1`
+	LL |         drop(x2); //~ ERROR capture of moved value: `x2`
51	   |              -- use occurs due to use in closure
52	   |
-	   = note: move occurs because `x1` has type `std::boxed::Box<i32>`, which does not implement the `Copy` trait
+	   = note: move occurs because `x2` has type `std::boxed::Box<i32>`, which does not implement the `Copy` trait
54	
55	error[E0382]: use of moved value: `x`
56	  --> $DIR/borrowck-multiple-captures.rs:46:14
