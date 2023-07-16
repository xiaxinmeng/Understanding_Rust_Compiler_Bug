
failures:

---- [ui] rustdoc-ui\failed-doctest-output.rs stdout ----
\rustdoc-ui\failed-doctest-output.rs
\rustdoc-ui\failed-doctest-output.rs
\rustdoc-ui\failed-doctest-output.rs
thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:354:22
\rustdoc-ui\failed-doctest-output.rs
\rustdoc-ui\failed-doctest-output.rs
\rustdoc-ui\failed-doctest-output.rs
\rustdoc-ui\failed-doctest-output.rs
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
\rustdoc-ui\failed-doctest-output.rs
diff of stdout:

1	
2	running 2 tests
-	test $DIR/failed-doctest-output.rs - OtherStruct (line 21) ... FAILED
4	test $DIR/failed-doctest-output.rs - SomeStruct (line 11) ... FAILED
+	test $DIR/failed-doctest-output.rs - OtherStruct (line 21) ... FAILED
5	
6	failures:
7	

-	---- $DIR/failed-doctest-output.rs - OtherStruct (line 21) stdout ----
-	error[E0425]: cannot find value `no` in this scope
-	  --> $DIR/failed-doctest-output.rs:22:1
-	   |
-	LL | no
-	   | ^^ not found in this scope
-	
-	error: aborting due to previous error
-	
-	For more information about this error, try `rustc --explain E0425`.
-	Couldn't compile the test.
19	---- $DIR/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
20	Test executable failed (exit code 101).
21	

30	note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
31	
32	
+	---- $DIR/failed-doctest-output.rs - OtherStruct (line 21) stdout ----
+	error[E0425]: cannot find value `no` in this scope
+	  --> $DIR/failed-doctest-output.rs:22:1
+	   |
+	LL | no
+	   | ^^ not found in this scope
+	
+	error: aborting due to previous error
+	
+	For more information about this error, try `rustc --explain E0425`.
+	Couldn't compile the test.
33	
34	failures:
35	    $DIR/failed-doctest-output.rs - OtherStruct (line 21)
