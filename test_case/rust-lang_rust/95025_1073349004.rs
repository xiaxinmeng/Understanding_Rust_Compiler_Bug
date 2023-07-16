plain

---- [ui] ui/regions/regions-close-object-into-object-5.rs stdout ----
diff of stderr:

20    |          - help: consider adding an explicit lifetime bound...: `T: 'static`
21 LL |     // oh dear!
22 LL |     Box::new(B(&*v)) as Box<dyn X>
+    |     ^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds...
+    |
+ note: ...that is required by this bound
+    |
+    |
+ LL | struct B<'a, T: 'a>(&'a (A<T> + 'a));
+ 
+ error[E0310]: the parameter type `T` may not live long enough
+   --> $DIR/regions-close-object-into-object-5.rs:17:5
+    |
+    |
+ LL | fn f<'a, T, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |          - help: consider adding an explicit lifetime bound...: `T: 'static`
+ LL |     // oh dear!
+ LL |     Box::new(B(&*v)) as Box<dyn X>
23    |     ^^^^^^^^^^^^^^^^ ...so that the type `B<'_, T>` will meet its required lifetime bounds
25 error[E0310]: the parameter type `T` may not live long enough


79 LL |     Box::new(B(&*v)) as Box<dyn X>
80    |                ^^^ ...so that the type `(dyn A<T> + 'static)` is not borrowed for too long
- error: aborting due to 7 previous errors
+ error: aborting due to 8 previous errors
83 
84 For more information about this error, try `rustc --explain E0310`.
84 For more information about this error, try `rustc --explain E0310`.
85 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-5/regions-close-object-into-object-5.stderr
To only update this specific test, also pass `--test-args regions/regions-close-object-into-object-5.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-close-object-into-object-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-5" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-5/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0310]: the parameter type `T` may not live long enough
   |
   |
LL | fn f<'a, T, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
   |          - help: consider adding an explicit lifetime bound...: `T: 'static`
LL |     // oh dear!
LL |     Box::new(B(&*v)) as Box<dyn X>
   |     ^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds...
   |
note: ...that is required by this bound
   |
   |
LL | struct B<'a, T: 'a>(&'a (A<T> + 'a));

error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:17:5
   |
   |
LL | fn f<'a, T, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
   |          - help: consider adding an explicit lifetime bound...: `T: 'static`
LL |     // oh dear!
LL |     Box::new(B(&*v)) as Box<dyn X>
   |     ^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds...
   |
note: ...that is required by this bound
   |
   |
LL | struct B<'a, T: 'a>(&'a (A<T> + 'a));

error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:17:5
   |
   |
LL | fn f<'a, T, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
   |          - help: consider adding an explicit lifetime bound...: `T: 'static`
LL |     // oh dear!
LL |     Box::new(B(&*v)) as Box<dyn X>
   |     ^^^^^^^^^^^^^^^^ ...so that the type `B<'_, T>` will meet its required lifetime bounds
error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:17:14
   |
   |
LL | fn f<'a, T, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
   |          - help: consider adding an explicit lifetime bound...: `T: 'static`
LL |     // oh dear!
LL |     Box::new(B(&*v)) as Box<dyn X>
   |              ^ ...so that the type `T` will meet its required lifetime bounds...
   |
note: ...that is required by this bound
   |
   |
LL | struct B<'a, T: 'a>(&'a (A<T> + 'a));

error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:17:14
   |
   |
LL | fn f<'a, T, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
   |          - help: consider adding an explicit lifetime bound...: `T: 'static`
LL |     // oh dear!
LL |     Box::new(B(&*v)) as Box<dyn X>
   |              ^^^^^^ ...so that the type `T` will meet its required lifetime bounds...
   |
note: ...that is required by this bound
   |
   |
LL | struct B<'a, T: 'a>(&'a (A<T> + 'a));

error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:17:16
   |
   |
LL | fn f<'a, T, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
   |          - help: consider adding an explicit lifetime bound...: `T: 'static`
LL |     // oh dear!
LL |     Box::new(B(&*v)) as Box<dyn X>
   |                ^^^ ...so that the reference type `&dyn A<T>` does not outlive the data it points at
error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:17:16
   |
   |
LL | fn f<'a, T, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
   |          - help: consider adding an explicit lifetime bound...: `T: 'static`
LL |     // oh dear!
LL |     Box::new(B(&*v)) as Box<dyn X>
   |                ^^^ ...so that the type `(dyn A<T> + 'static)` is not borrowed for too long
error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:17:16
   |
   |
LL | fn f<'a, T, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
   |          - help: consider adding an explicit lifetime bound...: `T: 'static`
LL |     // oh dear!
LL |     Box::new(B(&*v)) as Box<dyn X>
   |                ^^^ ...so that the type `(dyn A<T> + 'static)` is not borrowed for too long
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0310`.
------------------------------------------
