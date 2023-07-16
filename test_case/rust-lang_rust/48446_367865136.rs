
failures:

---- [ui] ui/explain.rs stdout ----
	
error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/nobackup/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/nobackup/rust/src/test/ui/explain.rs" "-L" "/nobackup/rust/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/nobackup/rust/build/x86_64-unknown-linux-gnu/test/ui/explain.stage1-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/nobackup/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--explain" "E0591" "-L" "/nobackup/rust/build/x86_64-unknown-linux-gnu/test/ui/explain.stage1-x86_64-unknown-linux-gnu.aux" "-A" "unused"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
{"message":"no extended information for E0591","code":null,"level":"error","spans":[],"children":[],"rendered":"error: no extended information for E0591\n\n"}

------------------------------------------

thread '[ui] ui/explain.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2892:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- [ui] ui/lint/use_suggestion_json.rs stdout ----
	diff of stderr:

2	  "message": "cannot find type `Iter` in this scope",
3	  "code": {
4	    "code": "E0412",
-	    "explanation": "
-	The type name used is not in scope.
-	
-	Erroneous code examples:
-	
-	