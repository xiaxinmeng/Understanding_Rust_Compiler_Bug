plain
..................................iii............................................................... 12600/12639
.......................................
failures:

---- [ui] ui/functions-closures/fn-help-with-err-generic-is-not-function.rs stdout ----

1 error[E0412]: cannot find type `T` in this scope
1 error[E0412]: cannot find type `T` in this scope
-   --> $DIR/fn-help-with-err-generic-is-not-function.rs:4:13
+   --> $DIR/fn-help-with-err-generic-is-not-function.rs:2:13
4 LL | impl Struct<T>
5    |     -       ^ not found in this scope

7    |     help: you might be missing a type parameter: `<T>`
7    |     help: you might be missing a type parameter: `<T>`
8 
9 error[E0412]: cannot find type `T` in this scope
-   --> $DIR/fn-help-with-err-generic-is-not-function.rs:9:5
+   --> $DIR/fn-help-with-err-generic-is-not-function.rs:7:5
12 LL |     T: Copy,
13    |     ^ not found in this scope

14 
14 
15 error[E0282]: type annotations needed
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   --> $DIR/fn-help-with-err-generic-is-not-function.rs:16:31
+   --> $DIR/fn-help-with-err-generic-is-not-function.rs:14:31
17    |
18 LL |     fn method(v: Vec<u8>) { v.len(); }

21    = note: type must be known at this point
22 
22 
23 error[E0599]: no method named `len` found for struct `Vec<u8>` in the current scope
-   --> $DIR/fn-help-with-err-generic-is-not-function.rs:16:31
+   --> $DIR/fn-help-with-err-generic-is-not-function.rs:14:31
25    |
26 LL |     fn method(v: Vec<u8>) { v.len(); }
27    |                               ^^^ private field, not a method

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/functions-closures/fn-help-with-err-generic-is-not-function/fn-help-with-err-generic-is-not-function.stderr
To only update this specific test, also pass `--test-args functions-closures/fn-help-with-err-generic-is-not-function.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/functions-closures/fn-help-with-err-generic-is-not-function.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/functions-closures/fn-help-with-err-generic-is-not-function" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/functions-closures/fn-help-with-err-generic-is-not-function/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0282]: type annotations needed
  --> /checkout/src/test/ui/functions-closures/fn-help-with-err-generic-is-not-function.rs:14:31
   |
LL |     fn method(v: Vec<u8>) { v.len(); }
   |
   = note: type must be known at this point


error[E0599]: no method named `len` found for struct `Vec<u8>` in the current scope
  --> /checkout/src/test/ui/functions-closures/fn-help-with-err-generic-is-not-function.rs:14:31
   |
LL |     fn method(v: Vec<u8>) { v.len(); }
   |                               ^^^ private field, not a method
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0282, E0412, E0599.
For more information about an error, try `rustc --explain E0282`.
For more information about an error, try `rustc --explain E0282`.

------------------------------------------


---- [ui] ui/functions-closures/fn-help-with-err.rs stdout ----


1 error[E0425]: cannot find value `oops` in this scope
-   --> $DIR/fn-help-with-err.rs:5:35
+   --> $DIR/fn-help-with-err.rs:3:35
3    |
4 LL |     let arc = std::sync::Arc::new(oops);

6 
6 
7 error[E0599]: no method named `blablabla` found for struct `Arc<_>` in the current scope
-   --> $DIR/fn-help-with-err.rs:9:9
+   --> $DIR/fn-help-with-err.rs:7:9
9    |
10 LL |     arc.blablabla();
11    |         ^^^^^^^^^ method not found in `Arc<_>`
12 
12 
- error[E0599]: no method named `blablabla` found for struct `Arc<[closure@$DIR/fn-help-with-err.rs:12:36: 12:40]>` in the current scope
-   --> $DIR/fn-help-with-err.rs:14:10
+ error[E0599]: no method named `blablabla` found for struct `Arc<[closure@$DIR/fn-help-with-err.rs:10:36: 10:40]>` in the current scope
+   --> $DIR/fn-help-with-err.rs:12:10
15    |
16 LL |     arc2.blablabla();
-    |          ^^^^^^^^^ method not found in `Arc<[closure@$DIR/fn-help-with-err.rs:12:36: 12:40]>`
+    |          ^^^^^^^^^ method not found in `Arc<[closure@$DIR/fn-help-with-err.rs:10:36: 10:40]>`
19    = note: `arc2` is a function, perhaps you wish to call it
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/functions-closures/fn-help-with-err/fn-help-with-err.stderr
To only update this specific test, also pass `--test-args functions-closures/fn-help-with-err.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/functions-closures/fn-help-with-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/functions-closures/fn-help-with-err" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/functions-closures/fn-help-with-err/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find value `oops` in this scope
  --> /checkout/src/test/ui/functions-closures/fn-help-with-err.rs:3:35
   |
LL |     let arc = std::sync::Arc::new(oops);


error[E0599]: no method named `blablabla` found for struct `Arc<_>` in the current scope
  --> /checkout/src/test/ui/functions-closures/fn-help-with-err.rs:7:9
   |
LL |     arc.blablabla();
   |         ^^^^^^^^^ method not found in `Arc<_>`

error[E0599]: no method named `blablabla` found for struct `Arc<[closure@/checkout/src/test/ui/functions-closures/fn-help-with-err.rs:10:36: 10:40]>` in the current scope
  --> /checkout/src/test/ui/functions-closures/fn-help-with-err.rs:12:10
   |
LL |     arc2.blablabla();
   |          ^^^^^^^^^ method not found in `Arc<[closure@/checkout/src/test/ui/functions-closures/fn-help-with-err.rs:10:36: 10:40]>`
   = note: `arc2` is a function, perhaps you wish to call it

error: aborting due to 3 previous errors

