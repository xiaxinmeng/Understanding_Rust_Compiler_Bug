
failures:

---- [ui] tests/ui/lazy-type-alias-impl-trait/branches3.rs stdout ----
diff of stderr:

1	error[E0282]: type annotations needed
-	  --> $DIR/branches3.rs:8:10
+	  --> $DIR/branches3.rs:8:13
3	   |
4	LL |         |s| s.len()
-	   |          ^  - type must be known at this point
-	   |
-	help: consider giving this closure parameter an explicit type
-	   |
-	LL |         |s: /* Type */| s.len()
-	   |           ++++++++++++
+	   |             ^ cannot infer type
11	
12	error[E0282]: type annotations needed
-	  --> $DIR/branches3.rs:15:10
+	  --> $DIR/branches3.rs:15:13
14	   |
15	LL |         |s| s.len()
-	   |          ^  - type must be known at this point
-	   |
-	help: consider giving this closure parameter an explicit type
-	   |
-	LL |         |s: /* Type */| s.len()
-	   |           ++++++++++++
+	   |             ^ cannot infer type
22	
23	error[E0282]: type annotations needed
-	  --> $DIR/branches3.rs:23:10
+	  --> $DIR/branches3.rs:23:13
25	   |
26	LL |         |s| s.len()
-	   |          ^  - type must be known at this point
-	   |
-	help: consider giving this closure parameter an explicit type
-	   |
-	LL |         |s: /* Type */| s.len()
-	   |           ++++++++++++
+	   |             ^ cannot infer type
33	
34	error[E0282]: type annotations needed
-	  --> $DIR/branches3.rs:30:10
+	  --> $DIR/branches3.rs:30:13
36	   |
37	LL |         |s| s.len()
-	   |          ^  - type must be known at this point
-	   |
-	help: consider giving this closure parameter an explicit type
-	   |
-	LL |         |s: /* Type */| s.len()
-	   |           ++++++++++++
+	   |             ^ cannot infer type
44	
45	error: aborting due to 4 previous errors
46	


The actual stderr differed from the expected stderr.
Actual stderr saved to /home/gh-spastorino/rust3/build/aarch64-unknown-linux-gnu/test/ui/lazy-type-alias-impl-trait/branches3/branches3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lazy-type-alias-impl-trait/branches3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/home/gh-spastorino/rust3/build/aarch64-unknown-linux-gnu/stage1/bin/rustc" "/home/gh-spastorino/rust3/tests/ui/lazy-type-alias-impl-trait/branches3.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/home/gh-spastorino/rust3/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/home/gh-spastorino/rust3/build/aarch64-unknown-linux-gnu/test/ui/lazy-type-alias-impl-trait/branches3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/home/gh-spastorino/rust3/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/home/gh-spastorino/rust3/build/aarch64-unknown-linux-gnu/test/ui/lazy-type-alias-impl-trait/branches3/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0282]: type annotations needed
  --> fake-test-src-base/lazy-type-alias-impl-trait/branches3.rs:8:13
   |
LL |         |s| s.len() //~ ERROR type annotations needed
   |             ^ cannot infer type

error[E0282]: type annotations needed
  --> fake-test-src-base/lazy-type-alias-impl-trait/branches3.rs:15:13
   |
LL |         |s| s.len() //~ ERROR type annotations needed
   |             ^ cannot infer type

error[E0282]: type annotations needed
  --> fake-test-src-base/lazy-type-alias-impl-trait/branches3.rs:23:13
   |
LL |         |s| s.len() //~ ERROR type annotations needed
   |             ^ cannot infer type

error[E0282]: type annotations needed
  --> fake-test-src-base/lazy-type-alias-impl-trait/branches3.rs:30:13
   |
LL |         |s| s.len() //~ ERROR type annotations needed
   |             ^ cannot infer type

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0282`.
------------------------------------------


---- [ui] tests/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-2.rs stdout ----
diff of stderr:

-	error[E0282]: type annotations needed for `Option<T>`
-	  --> $DIR/unboxed-closures-failed-recursive-fn-2.rs:8:9
+	error[E0282]: type annotations needed
+	  --> $DIR/unboxed-closures-failed-recursive-fn-2.rs:17:32
3	   |
-	LL |     let mut closure0 = None;
-	   |         ^^^^^^^^^^^^
-	...
7	LL |                         return c();
-	   |                                --- type must be known at this point
-	   |
-	help: consider giving `closure0` an explicit type, where the placeholders `_` are specified
-	   |
-	LL |     let mut closure0: Option<T> = None;
-	   |                     +++++++++++
+	   |                                ^^^ cannot infer type
14	
15	error: aborting due to previous error
16	


The actual stderr differed from the expected stderr.
Actual stderr saved to /home/gh-spastorino/rust3/build/aarch64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-2/unboxed-closures-failed-recursive-fn-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closures-failed-recursive-fn-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/home/gh-spastorino/rust3/build/aarch64-unknown-linux-gnu/stage1/bin/rustc" "/home/gh-spastorino/rust3/tests/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-2.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/home/gh-spastorino/rust3/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/home/gh-spastorino/rust3/build/aarch64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/home/gh-spastorino/rust3/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/home/gh-spastorino/rust3/build/aarch64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0282]: type annotations needed
  --> fake-test-src-base/unboxed-closures/unboxed-closures-failed-recursive-fn-2.rs:17:32
   |
LL |                         return c();
   |                                ^^^ cannot infer type

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
------------------------------------------


---- [ui] tests/ui/type-alias-impl-trait/closures_in_branches.rs stdout ----
diff of stderr:

1	error[E0282]: type annotations needed
-	  --> $DIR/closures_in_branches.rs:7:10
+	  --> $DIR/closures_in_branches.rs:7:13
3	   |
4	LL |         |x| x.len()
-	   |          ^  - type must be known at this point
-	   |
-	help: consider giving this closure parameter an explicit type
-	   |
-	LL |         |x: /* Type */| x.len()
-	   |           ++++++++++++
+	   |             ^ cannot infer type
11	
12	error[E0282]: type annotations needed
-	  --> $DIR/closures_in_branches.rs:21:10
+	  --> $DIR/closures_in_branches.rs:21:13
14	   |
15	LL |         |x| x.len()
-	   |          ^  - type must be known at this point
-	   |
-	help: consider giving this closure parameter an explicit type
-	   |
-	LL |         |x: /* Type */| x.len()
-	   |           ++++++++++++
+	   |             ^ cannot infer type
22	
23	error: aborting due to 2 previous errors
24	


The actual stderr differed from the expected stderr.
Actual stderr saved to /home/gh-spastorino/rust3/build/aarch64-unknown-linux-gnu/test/ui/type-alias-impl-trait/closures_in_branches/closures_in_branches.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/closures_in_branches.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/home/gh-spastorino/rust3/build/aarch64-unknown-linux-gnu/stage1/bin/rustc" "/home/gh-spastorino/rust3/tests/ui/type-alias-impl-trait/closures_in_branches.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/home/gh-spastorino/rust3/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/home/gh-spastorino/rust3/build/aarch64-unknown-linux-gnu/test/ui/type-alias-impl-trait/closures_in_branches" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/home/gh-spastorino/rust3/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/home/gh-spastorino/rust3/build/aarch64-unknown-linux-gnu/test/ui/type-alias-impl-trait/closures_in_branches/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0282]: type annotations needed
  --> fake-test-src-base/type-alias-impl-trait/closures_in_branches.rs:7:13
   |
LL |         |x| x.len() //~ ERROR type annotations needed
   |             ^ cannot infer type

error[E0282]: type annotations needed
  --> fake-test-src-base/type-alias-impl-trait/closures_in_branches.rs:21:13
   |
LL |         |x| x.len() //~ ERROR type annotations needed
   |             ^ cannot infer type

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0282`.
------------------------------------------



failures:
    [ui] tests/ui/lazy-type-alias-impl-trait/branches3.rs
    [ui] tests/ui/type-alias-impl-trait/closures_in_branches.rs
    [ui] tests/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-2.rs

test result: FAILED. 4 passed; 3 failed; 14577 ignored; 0 measured; 0 filtered out; finished in 0.19s
