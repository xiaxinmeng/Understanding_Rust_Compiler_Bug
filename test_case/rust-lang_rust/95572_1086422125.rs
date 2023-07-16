plain

---- [ui (nll)] ui/generic-associated-types/trait-objects.rs#extended stdout ----
diff of stderr:

- error[E0621]: explicit lifetime required in the type of `x`
-   --> $DIR/trait-objects.rs:16:7
+ error[E0521]: borrowed data escapes outside of function
3    |
3    |
4 LL | fn min_size(x: &mut dyn for<'a> StreamingIterator<Item<'a> = &'a i32>) -> usize {
-    |                ------------------------------------------------------ help: add explicit lifetime `'a` to the type of `x`: `&'a mut (dyn StreamingIterator<for<'a> Item = &'a i32> + 'a)`
+    |             -  - let's call the lifetime of this reference `'1`
+    |             |
+    |             `x` is a reference that is only valid in the function body
7 LL |     x.size_hint().0
7 LL |     x.size_hint().0
-    |       ^^^^^^^^^ lifetime `'a` required
+    |     |
+    |     |
+    |     `x` escapes the function body here
+    |     argument requires that `'1` must outlive `'static`
10 error: aborting due to previous error
11 

- For more information about this error, try `rustc --explain E0621`.
- For more information about this error, try `rustc --explain E0621`.
+ For more information about this error, try `rustc --explain E0521`.
13 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/trait-objects.extended.nll/trait-objects.extended.nll.stderr
To only update this specific test, also pass `--test-args generic-associated-types/trait-objects.rs`


Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error in revision `extended`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/trait-objects.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "extended" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/trait-objects.extended.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/trait-objects.extended.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0521]: borrowed data escapes outside of function
   |
   |
LL | fn min_size(x: &mut dyn for<'a> StreamingIterator<Item<'a> = &'a i32>) -> usize {
   |             -  - let's call the lifetime of this reference `'1`
   |             |
   |             `x` is a reference that is only valid in the function body
LL |     //[base]~^ the trait `StreamingIterator` cannot be made into an object
LL |     x.size_hint().0
   |     |
   |     |
   |     `x` escapes the function body here
   |     argument requires that `'1` must outlive `'static`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0521`.
------------------------------------------
