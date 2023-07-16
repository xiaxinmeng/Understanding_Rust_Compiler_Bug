plain
diff of stderr:

2   --> $DIR/closure-in-projection-issue-97405.rs:24:5
3    |
4 LL |     assert_static(opaque(async move { t; }).next());
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
6    |
7    = help: consider adding an explicit lifetime bound `<impl Iterator as Iterator>::Item: 'static`...
-    = note: ...so that the type `Option<<impl Iterator as Iterator>::Item>` will meet its required lifetime bounds...
- note: ...that is required by this bound
-   --> $DIR/closure-in-projection-issue-97405.rs:11:21
-    |
- LL | fn assert_static<T: 'static>(_: T) {}
-    |                     ^^^^^^^
+    = note: ...so that the type `<impl Iterator as Iterator>::Item` will meet its required lifetime bounds
14 
15 error[E0310]: the associated type `<impl Iterator as Iterator>::Item` may not live long enough

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
17    |
17    |
18 LL |     assert_static(opaque(move || { t; }).next());
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
20    |
20    |
21    = help: consider adding an explicit lifetime bound `<impl Iterator as Iterator>::Item: 'static`...
-    = note: ...so that the type `Option<<impl Iterator as Iterator>::Item>` will meet its required lifetime bounds...
- note: ...that is required by this bound
-   --> $DIR/closure-in-projection-issue-97405.rs:11:21
-    |
- LL | fn assert_static<T: 'static>(_: T) {}
-    |                     ^^^^^^^
+    = note: ...so that the type `<impl Iterator as Iterator>::Item` will meet its required lifetime bounds
28 
29 error[E0310]: the associated type `<impl Iterator as Iterator>::Item` may not live long enough

31    |
31    |
32 LL |     assert_static(opaque(opaque(async move { t; }).next()).next());
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
34    |
34    |
35    = help: consider adding an explicit lifetime bound `<impl Iterator as Iterator>::Item: 'static`...
-    = note: ...so that the type `Option<<impl Iterator as Iterator>::Item>` will meet its required lifetime bounds...
- note: ...that is required by this bound
-   --> $DIR/closure-in-projection-issue-97405.rs:11:21
-    |
- LL | fn assert_static<T: 'static>(_: T) {}
-    |                     ^^^^^^^
+    = note: ...so that the type `<impl Iterator as Iterator>::Item` will meet its required lifetime bounds
43 error: aborting due to 3 previous errors
44 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/closure-in-projection-issue-97405/closure-in-projection-issue-97405.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/closure-in-projection-issue-97405.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/closure-in-projection-issue-97405.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/closure-in-projection-issue-97405" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/closure-in-projection-issue-97405/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0310]: the associated type `<impl Iterator as Iterator>::Item` may not live long enough
   |
   |
LL |     assert_static(opaque(async move { t; }).next());
   |
   |
   = help: consider adding an explicit lifetime bound `<impl Iterator as Iterator>::Item: 'static`...
   = note: ...so that the type `<impl Iterator as Iterator>::Item` will meet its required lifetime bounds

error[E0310]: the associated type `<impl Iterator as Iterator>::Item` may not live long enough
   |
   |
LL |     assert_static(opaque(move || { t; }).next());
   |
   |
   = help: consider adding an explicit lifetime bound `<impl Iterator as Iterator>::Item: 'static`...
   = note: ...so that the type `<impl Iterator as Iterator>::Item` will meet its required lifetime bounds

error[E0310]: the associated type `<impl Iterator as Iterator>::Item` may not live long enough
   |
   |
LL |     assert_static(opaque(opaque(async move { t; }).next()).next());
   |
   |
   = help: consider adding an explicit lifetime bound `<impl Iterator as Iterator>::Item: 'static`...
   = note: ...so that the type `<impl Iterator as Iterator>::Item` will meet its required lifetime bounds
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0310`.
------------------------------------------
