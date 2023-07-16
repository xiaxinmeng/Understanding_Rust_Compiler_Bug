diff
 ---- [ui] ui/consts/issue-79690.rs stdout ----
diff of stderr:

2	  --> $DIR/issue-79690.rs:26:1
3	   |
4	LL | const G: Fat = unsafe { Transmute { t: FOO }.u };
-	   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered read of part of a pointer at .1.<deref>.size.foo
+	   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc4 at .1.<deref>.size.foo, but expected initialized plain (non-pointer) bytes
6	   |
7	   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
8	
