
---- [ui] ui/consts/min_const_fn/min_const_fn_unsafe_bad.rs stdout ----
1551
diff of stderr:
1552

1553
34	   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
1554
35	   = help: add `#![feature(const_fn)]` to the crate attributes to enable
1555
36	
1556
-	error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
1557
-	  --> $DIR/min_const_fn_unsafe_bad.rs:1:77
1558
-	   |
1559
-	LL | const fn bad_const_fn_deref_raw(x: *mut usize) -> &'static usize { unsafe { &*x } }
1560
-	   |                                                                             ^^^ dereference of raw pointer
1561
-	   |
1562
-	   = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
1563
+	error: aborting due to 4 previous errors
1564
44	
1565
-	error: aborting due to 5 previous errors
1566
-	
1567
-	Some errors have detailed explanations: E0133, E0658, E0723.
1568
-	For more information about an error, try `rustc --explain E0133`.
1569
+	Some errors have detailed explanations: E0658, E0723.
1570
+	For more information about an error, try `rustc --explain E0658`.
1571
49	
