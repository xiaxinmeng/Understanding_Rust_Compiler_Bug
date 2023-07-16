plain
test [ui (nll)] src/test/ui/wrong-hashset-issue-42918.rs ... ok

failures:

---- [ui (nll)] src/test/ui/hr-subtype/placeholder-pattern-fail.rs stdout ----

1 error[E0308]: mismatched types
-   --> $DIR/placeholder-pattern-fail.rs:9:47
+   --> $DIR/placeholder-pattern-fail.rs:9:12
+   --> $DIR/placeholder-pattern-fail.rs:9:12
3    |
4 LL |     let _: for<'a, 'b> fn(Inv<'a>, Inv<'b>) = sub;
-    |                                               ^^^ one type is more general than the other
+    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
6    |
7    = note: expected fn pointer `for<'a, 'b> fn(Inv<'a>, Inv<'b>)`
8               found fn pointer `for<'a> fn(Inv<'a>, Inv<'a>)`
9 
10 error[E0308]: mismatched types
-   --> $DIR/placeholder-pattern-fail.rs:14:31
+   --> $DIR/placeholder-pattern-fail.rs:9:12
+   --> $DIR/placeholder-pattern-fail.rs:9:12
12    |
- LL |     let _x: (&'static i32,) = x;
-    |                               ^ lifetime mismatch
+ LL |     let _: for<'a, 'b> fn(Inv<'a>, Inv<'b>) = sub;
+    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
15    |
-    = note: expected tuple `(&'static i32,)`
-               found tuple `(&'c i32,)`
- note: the lifetime `'c` as defined here...
-   --> $DIR/placeholder-pattern-fail.rs:13:12
+    = note: expected fn pointer `for<'a, 'b> fn(Inv<'a>, Inv<'b>)`
+               found fn pointer `for<'a> fn(Inv<'a>, Inv<'a>)`
+ error: lifetime may not live long enough
+   --> $DIR/placeholder-pattern-fail.rs:14:13
20    |
20    |
21 LL | fn simple1<'c>(x: (&'c i32,)) {
-    |            ^^
-    = note: ...does not necessarily outlive the static lifetime
+    |            -- lifetime `'c` defined here
+ LL |     let _x: (&'static i32,) = x;
+    |             ^^^^^^^^^^^^^^^ type annotation requires that `'c` must outlive `'static`
- error[E0308]: mismatched types
-   --> $DIR/placeholder-pattern-fail.rs:19:30
+ error: lifetime may not live long enough
+   --> $DIR/placeholder-pattern-fail.rs:19:12
+   --> $DIR/placeholder-pattern-fail.rs:19:12
27    |
- LL |     let _: (&'static i32,) = x;
-    |                              ^ lifetime mismatch
-    |
-    = note: expected tuple `(&'static i32,)`
-               found tuple `(&'c i32,)`
- note: the lifetime `'c` as defined here...
-   --> $DIR/placeholder-pattern-fail.rs:18:12
-    |
36 LL | fn simple2<'c>(x: (&'c i32,)) {
-    |            ^^
-    = note: ...does not necessarily outlive the static lifetime
+    |            -- lifetime `'c` defined here
+ LL |     let _: (&'static i32,) = x;
+    |            ^^^^^^^^^^^^^^^ type annotation requires that `'c` must outlive `'static`
- error: aborting due to 3 previous errors
+ error: aborting due to 4 previous errors
41 
42 For more information about this error, try `rustc --explain E0308`.
42 For more information about this error, try `rustc --explain E0308`.
43 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/placeholder-pattern-fail.nll/placeholder-pattern-fail.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hr-subtype/placeholder-pattern-fail.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hr-subtype/placeholder-pattern-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/placeholder-pattern-fail.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/placeholder-pattern-fail.nll/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/hr-subtype/placeholder-pattern-fail.rs:9:12
   |
   |
LL |     let _: for<'a, 'b> fn(Inv<'a>, Inv<'b>) = sub;
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected fn pointer `for<'a, 'b> fn(Inv<'a>, Inv<'b>)`
              found fn pointer `for<'a> fn(Inv<'a>, Inv<'a>)`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/hr-subtype/placeholder-pattern-fail.rs:9:12
   |
   |
LL |     let _: for<'a, 'b> fn(Inv<'a>, Inv<'b>) = sub;
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected fn pointer `for<'a, 'b> fn(Inv<'a>, Inv<'b>)`
              found fn pointer `for<'a> fn(Inv<'a>, Inv<'a>)`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/hr-subtype/placeholder-pattern-fail.rs:14:13
   |
   |
LL | fn simple1<'c>(x: (&'c i32,)) {
   |            -- lifetime `'c` defined here
LL |     let _x: (&'static i32,) = x;
   |             ^^^^^^^^^^^^^^^ type annotation requires that `'c` must outlive `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/hr-subtype/placeholder-pattern-fail.rs:19:12
   |
   |
LL | fn simple2<'c>(x: (&'c i32,)) {
   |            -- lifetime `'c` defined here
LL |     let _: (&'static i32,) = x;
   |            ^^^^^^^^^^^^^^^ type annotation requires that `'c` must outlive `'static`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui (nll)] src/test/ui/lifetimes/re-empty-in-error.rs stdout ----


- error[E0477]: the type `&'b ()` does not fulfill the required lifetime
+ error: higher-ranked lifetime error
2   --> $DIR/re-empty-in-error.rs:8:5
4 LL |     foo(&10);

-    |     ^^^
+    |     ^^^^^^^^
+    |     ^^^^^^^^
6    |
- note: type must outlive the empty lifetime as required by this binding
-   --> $DIR/re-empty-in-error.rs:3:47
-    |
- LL | fn foo<'a>(_a: &'a u32) where for<'b> &'b (): 'a {
-    |                                               ^^
+    = note: could not prove for<'b, 'r> &'b (): 'r
13 error: aborting due to previous error
14 

- For more information about this error, try `rustc --explain E0477`.
- For more information about this error, try `rustc --explain E0477`.
16 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/re-empty-in-error.nll/re-empty-in-error.nll.stderr
To only update this specific test, also pass `--test-args lifetimes/re-empty-in-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/re-empty-in-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/re-empty-in-error.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/re-empty-in-error.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error: higher-ranked lifetime error
  --> /checkout/src/test/ui/lifetimes/re-empty-in-error.rs:8:5
LL |     foo(&10);
   |     ^^^^^^^^
   |
   |
   = note: could not prove for<'b, 'r> &'b (): 'r
error: aborting due to previous error
------------------------------------------




failures:
    [ui (nll)] src/test/ui/hr-subtype/placeholder-pattern-fail.rs
    [ui (nll)] src/test/ui/lifetimes/re-empty-in-error.rs

test result: FAILED. 12759 passed; 2 failed; 358 ignored; 0 measured; 0 filtered out; finished in 85.23s

Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
