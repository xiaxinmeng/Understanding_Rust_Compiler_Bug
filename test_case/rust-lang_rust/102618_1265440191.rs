plain
diff of stderr:

+ error[E0310]: the parameter type `T` may not live long enough
+    |
+ help: consider adding an explicit lifetime bound...
+    |
+ LL | fn foo<T: 'static>() {
+ 
+ 
1 error: `T` does not live long enough
3    |


22 LL |     let _: for<'a> fn(<() as Foo<T>>::Type<'a>, &'a T) = |_, _| ();
24 
- error[E0310]: the parameter type `T` may not live long enough
-   --> $DIR/issue-91139.rs:14:58
-    |
-    |
- LL |     let _: for<'a> fn(<() as Foo<T>>::Type<'a>, &'a T) = |_, _| ();
-    |                                                          ^^^^^^ ...so that the type `T` will meet its required lifetime bounds
- help: consider adding an explicit lifetime bound...
-    |
-    |
- LL | fn foo<T: 'static>() {
- 
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
36 error: `T` does not live long enough
38    |


45 LL |     let _: for<'a> fn(<() as Foo<T>>::Type<'a>, &'a T) = |_, _| ();
47 
- error[E0310]: the parameter type `T` may not live long enough
-   --> $DIR/issue-91139.rs:14:58
-    |
-    |
- LL |     let _: for<'a> fn(<() as Foo<T>>::Type<'a>, &'a T) = |_, _| ();
-    |                                                          ^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
- help: consider adding an explicit lifetime bound...
-    |
-    |
- LL | fn foo<T: 'static>() {
- 
- 
59 error: `T` does not live long enough
61    |


68 LL |     let _: for<'a> fn(<() as Foo<T>>::Type<'a>, &'a T) = |_, _| ();
70 
- error: aborting due to 10 previous errors
+ error: aborting due to 9 previous errors
72 
---
To only update this specific test, also pass `--test-args generic-associated-types/issue-91139.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-91139.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-91139" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-91139/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0310]: the parameter type `T` may not live long enough
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn foo<T: 'static>() {


error: `T` does not live long enough
   |
   |
LL |     let _: for<'a> fn(<() as Foo<T>>::Type<'a>, &'a T) = |_, _| ();


error: `T` does not live long enough
   |
   |
LL |     let _: for<'a> fn(<() as Foo<T>>::Type<'a>, &'a T) = |_, _| ();


error: `T` does not live long enough
   |
   |
LL |     let _: for<'a> fn(<() as Foo<T>>::Type<'a>, &'a T) = |_, _| ();


error: `T` does not live long enough
   |
   |
LL |     let _: for<'a> fn(<() as Foo<T>>::Type<'a>, &'a T) = |_, _| ();


error: `T` does not live long enough
   |
   |
LL |     let _: for<'a> fn(<() as Foo<T>>::Type<'a>, &'a T) = |_, _| ();


error: `T` does not live long enough
   |
   |
LL |     let _: for<'a> fn(<() as Foo<T>>::Type<'a>, &'a T) = |_, _| ();


error: `T` does not live long enough
   |
   |
LL |     let _: for<'a> fn(<() as Foo<T>>::Type<'a>, &'a T) = |_, _| ();


error: `T` does not live long enough
   |
   |
LL |     let _: for<'a> fn(<() as Foo<T>>::Type<'a>, &'a T) = |_, _| ();

error: aborting due to 9 previous errors

For more information about this error, try `rustc --explain E0310`.
