diff
 ---- [ui] ui\const-eval\const_panic_libcore_main.rs stdout ----
 diff of stderr:
-	error: this constant cannot be used
-	  --> $DIR/const_panic_libcore_main.rs:20:1
-	   |
-	LL | const Z: () = panic!("cheese");
-	   | ^^^^^^^^^^^^^^----------------^
-	   |               |
-	   |               the evaluated program panicked at 'cheese', $DIR/const_panic_libcore_main.rs:20:15
-	   |
-	   = note: #[deny(const_err)] on by default
-	   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
+	error: language item required, but not found: `eh_unwind_resume`
11	
-	error: this constant cannot be used
-	  --> $DIR/const_panic_libcore_main.rs:23:1
-	   |
-	LL | const Y: () = unreachable!();
-	   | ^^^^^^^^^^^^^^--------------^
-	   |               |
-	   |               the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_libcore_main.rs:23:15
-	   |
-	   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
-	
-	error: this constant cannot be used
-	  --> $DIR/const_panic_libcore_main.rs:26:1
-	   |
-	LL | const X: () = unimplemented!();
-	   | ^^^^^^^^^^^^^^----------------^
-	   |               |
-	   |               the evaluated program panicked at 'not yet implemented', $DIR/const_panic_libcore_main.rs:26:15
-	   |
-	   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
-	
-	error: aborting due to 3 previous errors
+	error: aborting due to previous error
33	
34	
