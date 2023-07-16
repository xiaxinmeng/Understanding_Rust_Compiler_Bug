
---- [ui] rust-llvm-integrate-prototype/tests/ui/never_type/fallback-closure-wrap.rs#fallback stdout ----

diff of stderr:

10	   |
11	   = note: expected unit type `()`
12	                   found type `!`
-	   = note: required for the cast from `[closure@$DIR/fallback-closure-wrap.rs:18:40: 18:47]` to the object type `dyn FnMut()`
+	   = note: required for the cast from `[closure@fallback-closure-wrap.rs:18:40]` to the object type `dyn FnMut()`
+	   = note: the full name for the casted type has been written to '$TEST_BUILD_DIR/never_type/fallback-closure-wrap.fallback/fallback-closure-wrap.long-type-6621918892693463368.txt'
14	
15	error: aborting due to previous error
16	
