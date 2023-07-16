diff
---- [ui] ui/extern/extern-const.rs stdout ----
diff of stderr:

4	LL |     const rust_dbg_static_mut: libc::c_int; //~ ERROR extern items cannot be `const`
5	   |     ^^^^^ help: try using a static value: `static`
6	
-	error: aborting due to previous error
+	error[E0412]: cannot find type `c_int` in module `libc`
+	  --> $DIR/extern-const.rs:15:38
+	   |
+	LL |     const rust_dbg_static_mut: libc::c_int; //~ ERROR extern items cannot be `const`
+	   |                                      ^^^^^ not found in `libc`
+	help: possible candidate is found in another module, you can import it into scope
+	   |
+	LL | use std::os::raw::c_int;
+	   |
8	
+	error: aborting due to 2 previous errors
+	
+	For more information about this error, try `rustc --explain E0412`.
9	


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/extern-const.stderr
diff of fixed:

10	#![feature(libc)]
11	extern crate libc;
12	
+	use std::os::raw::c_int;
+	
13	#[link(name = "rust_test_helpers", kind = "static")]
14	extern "C" {
15	    static rust_dbg_static_mut: libc::c_int; //~ ERROR extern items cannot be `const`


The actual fixed differed from the expected fixed.
