
failures:
---- [ui (nll)] ui\issue-52126-assign-op-invariance.rs stdout ----
diff of stderr:
8	   |                                                        ------- borrow later used here
9	...
10	LL |     }
-	   |     - borrowed value only lives until here
+	   |     - `line` dropped here while still borrowed
12	
13	error: aborting due to previous error
14	
The actual stderr differed from the expected stderr.
Actual stderr saved to C:\projects\rust\build\x86_64-pc-windows-msvc\test\ui\issue-52126-assign-op-invariance.nll\issue-52126-assign-op-invariance.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issue-52126-assign-op-invariance.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
