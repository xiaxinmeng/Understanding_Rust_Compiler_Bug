diff
-	error[E0597]: borrowed value does not live long enough
+	error[E0716]: temporary value dropped while borrowed
2	  --> $DIR/const-ptr-nonnull.rs:4:37
3	   |
4	LL |     let x: &'static NonNull<u32> = &(NonNull::dangling());

-	   |                                     ^^^^^^^^^^^^^^^^^^^^^ temporary value does not live long enough
+	   |            ---------------------    ^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+	   |            |
+	   |            type annotation requires that borrow lasts for `'static`
6	...
7	LL | }
-	   | - temporary value only lives until here
-	   |
-	   = note: borrowed value must be valid for the static lifetime...
+	   | - temporary value is freed at the end of this statement
11	
-	error[E0597]: borrowed value does not live long enough
+	error[E0716]: temporary value dropped while borrowed
13	  --> $DIR/const-ptr-nonnull.rs:9:37
14	   |
15	LL |     let x: &'static NonNull<u32> = &(non_null.cast());

-	   |                                     ^^^^^^^^^^^^^^^^^ temporary value does not live long enough
+	   |            ---------------------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+	   |            |
+	   |            type annotation requires that borrow lasts for `'static`
17	LL |     //~^ ERROR borrowed value does not live long enough
18	LL | }
-	   | - temporary value only lives until here
-	   |
-	   = note: borrowed value must be valid for the static lifetime...
+	   | - temporary value is freed at the end of this statement
22	
23	error: aborting due to 2 previous errors
24	

-	For more information about this error, try `rustc --explain E0597`.
+	For more information about this error, try `rustc --explain E0716`.
26	
