plain
......................................................................iii........................... 12500/12575
...........................................................................
failures:

---- [ui] ui/associated-consts/assoc-const-ty-mismatch.rs stdout ----


- error: type/const mismatch in equality bind of associated field
-   --> $DIR/assoc-const-ty-mismatch.rs:23:15
+ error: mismatch in bind of [unknown], got type
+   --> $DIR/assoc-const-ty-mismatch.rs:5:3
+ LL |   const N: usize;
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |   ^^^^^^^^^^^^^^^
+ ...
+ ...
4 LL | fn foo<F: Foo<N=usize>>() {}
-    |               ^^^^^^^ type/const Mismatch
+    |               ------- [unknown]/type mismatch
6 
- error: type/const mismatch in equality bind of associated field
-   --> $DIR/assoc-const-ty-mismatch.rs:25:18
+ error: mismatch in bind of associated const, got const
+   --> $DIR/assoc-const-ty-mismatch.rs:9:3
+ LL |   type T;
+    |   ^^^^^^^
+ ...
+ ...
10 LL | fn foo2<F: FooTy<T=3usize>>() {}
-    |                  ^^^^^^^^ type/const Mismatch
+    |                  -------- associated const/const mismatch
13 error: aborting due to 2 previous errors
14 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/assoc-const-ty-mismatch/assoc-const-ty-mismatch.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-consts/assoc-const-ty-mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/assoc-const-ty-mismatch" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/assoc-const-ty-mismatch/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: mismatch in bind of [unknown], got type
  --> /checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs:5:3
LL |   const N: usize;
   |   ^^^^^^^^^^^^^^^
...
...
LL | fn foo<F: Foo<N=usize>>() {}
   |               ------- [unknown]/type mismatch
error: mismatch in bind of associated const, got const
  --> /checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs:9:3
   |
LL |   type T;
LL |   type T;
   |   ^^^^^^^
...
LL | fn foo2<F: FooTy<T=3usize>>() {}
   |                  -------- associated const/const mismatch
error: aborting due to 2 previous errors


------------------------------------------
---

4 LL |     foo(());
5    |     ^^^ lifetime mismatch
6    |
-    = note: expected reference `&'a ()`
-                    found type `&()`
+    = note: expected type `&'a ()`
+               found type `&()`
9 note: the lifetime requirement is introduced here
10   --> $DIR/higher-ranked-projection.rs:15:33


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/higher-ranked-projection.bad/higher-ranked-projection.bad.stderr
To only update this specific test, also pass `--test-args associated-types/higher-ranked-projection.rs`


error in revision `bad`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/higher-ranked-projection.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "bad" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/higher-ranked-projection.bad" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/higher-ranked-projection.bad/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/higher-ranked-projection.rs:25:5
   |
LL |     foo(());
   |     ^^^ lifetime mismatch
   |
   = note: expected type `&'a ()`
              found type `&()`
note: the lifetime requirement is introduced here
   |
   |
LL |     where for<'a> &'a T: Mirror<Image=U>

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.

------------------------------------------


---- [ui] ui/lifetimes/issue-79187-2.rs stdout ----
diff of stderr:

23 LL |     take_foo(|a: &i32| a);
24    |     ^^^^^^^^ lifetime mismatch
-    = note: expected reference `&i32`
-               found reference `&i32`
+    = note: expected type `&i32`
+               found type `&i32`
+               found type `&i32`
28 note: the anonymous lifetime #1 defined here doesn't meet the lifetime requirements
30    |


42 LL |     take_foo(|a: &i32| -> &i32 { a });
43    |     ^^^^^^^^ lifetime mismatch
-    = note: expected reference `&i32`
-               found reference `&i32`
+    = note: expected type `&i32`
+               found type `&i32`
+               found type `&i32`
47 note: the anonymous lifetime #1 defined here doesn't meet the lifetime requirements
49    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-79187-2/issue-79187-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/issue-79187-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/issue-79187-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-79187-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-79187-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:8:5
   |
LL |     take_foo(|a| a); //~ ERROR mismatched types
   |     ^^^^^^^^ lifetime mismatch
   |
   = note: expected type `for<'r> Fn<(&'r i32,)>`
              found type `Fn<(&i32,)>`
note: this closure does not fulfill the lifetime requirements
   |
   |
LL |     take_foo(|a| a); //~ ERROR mismatched types
note: the lifetime requirement is introduced here
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:5:21
   |
   |
LL | fn take_foo(_: impl Foo) {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:9:5
   |
   |
LL |     take_foo(|a: &i32| a); //~ ERROR mismatched types
   |     ^^^^^^^^ lifetime mismatch
   = note: expected type `&i32`
              found type `&i32`
              found type `&i32`
note: the anonymous lifetime #1 defined here doesn't meet the lifetime requirements
   |
   |
LL |     take_foo(|a: &i32| a); //~ ERROR mismatched types
note: the lifetime requirement is introduced here
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:5:21
   |
   |
LL | fn take_foo(_: impl Foo) {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:10:5
   |
   |
LL |     take_foo(|a: &i32| -> &i32 { a }); //~ ERROR mismatched types
   |     ^^^^^^^^ lifetime mismatch
   = note: expected type `&i32`
              found type `&i32`
              found type `&i32`
note: the anonymous lifetime #1 defined here doesn't meet the lifetime requirements
   |
   |
LL |     take_foo(|a: &i32| -> &i32 { a }); //~ ERROR mismatched types
note: the lifetime requirement is introduced here
  --> /checkout/src/test/ui/lifetimes/issue-79187-2.rs:5:21
   |
   |
LL | fn take_foo(_: impl Foo) {}

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.
