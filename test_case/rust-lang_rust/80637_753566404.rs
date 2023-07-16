plain
..........................................................................................i......... 3800/11240
.................................................................................................... 3900/11240
.................................................................................................... 4000/11240
.................................................................................................... 4100/11240
.....................................ii...........F....F............................................ 4200/11240
..............FF.................................................................................... 4400/11240
.................................................i.................................................. 4500/11240
..................................F................................................................. 4600/11240
..........................................................................................F......... 4700/11240
..........................................................................................F......... 4700/11240
.................................................................................................... 4800/11240
..............................F..................................................................... 4900/11240
.......................................F.........................................................i.. 5000/11240
.................................................................................................... 5200/11240
.................................................................................................... 5300/11240
.................................................................................................... 5400/11240
.................................................................................................... 5500/11240
.................................................................................................... 5500/11240
.................................................................................................... 5600/11240
..................................................................i...........i..................... 5700/11240
.................................................................................................... 5800/11240
.................................................................................................... 5900/11240
........i........................................................................................... 6000/11240
....i.........................................F..............i..........................F........... 6100/11240
...................................................................ii..ii......i...i................ 6200/11240
.......................i.........................i.................................................. 6400/11240
..........................................................i......................................... 6500/11240
.................................................................................................... 6600/11240
...............................ii..........................................i........................ 6700/11240
---
.................................................................................................... 9000/11240
.................................................................................................... 9100/11240
.................................................................................................... 9200/11240
....................................i......i........................................................ 9300/11240
...........................................................................iiiiii..iiiiii.i......... 9400/11240
.................................................................................................... 9600/11240
.................................................................................................... 9700/11240
.................................................................................................... 9800/11240
.................................................................................................... 9900/11240
.................................................................................................... 9900/11240
..........................................................................F......................... 10000/11240
..............................................F.FF.................................................. 10100/11240
.................................................................F.................................. 10300/11240
.................................................................................................... 10400/11240
.................................................................................................... 10500/11240
.................................................................................................i.. 10600/11240
.................................................................................................i.. 10600/11240
..........................................................................FF.F...................... 10700/11240
..................................................................F................................. 10800/11240
.................................................................................................... 11000/11240
.................................................................................................... 11100/11240
...............................i.................................................................... 11200/11240
........................................
........................................
failures:

---- [ui] ui/array-slice-vec/vector-no-ann.rs stdout ----
diff of stderr:

- error[E0282]: type annotations needed for `Vec<T>`
+ error[E0282]: type annotations needed
3    |
3    |
4 LL |     let _foo = Vec::new();

5    |         ----   ^^^^^^^^ cannot infer type for type parameter `T`
6    |         |
-    |         consider giving `_foo` the explicit type `Vec<T>`, where the type parameter `T` is specified
+    |         consider giving `_foo` a type
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/vector-no-ann/vector-no-ann.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args array-slice-vec/vector-no-ann.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/array-slice-vec/vector-no-ann.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/vector-no-ann" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/vector-no-ann/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/array-slice-vec/vector-no-ann.rs:2:16
   |
LL |     let _foo = Vec::new();
   |         ----   ^^^^^^^^ cannot infer type for type parameter `T`
   |         |
   |         consider giving `_foo` a type
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.

---

7    = note: `#[warn(incomplete_features)]` on by default
8    = note: see issue #63065 <https://github.com/rust-lang/rust/issues/63065> for more information
9 
- error[E0282]: type annotations needed for `impl Future`
+ error[E0282]: type annotations needed
12    |
12    |
13 LL |     let fut = async {

-    |         --- consider giving `fut` the explicit type `impl Future`, with the type parameters specified
+    |         --- consider giving `fut` a type
15 LL |         make_unit()?;
16    |                    ^ cannot infer type


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-async-enabled-impl-trait-bindings/cannot-infer-async-enabled-impl-trait-bindings.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-async-enabled-impl-trait-bindings/cannot-infer-async-enabled-impl-trait-bindings.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inference/cannot-infer-async-enabled-impl-trait-bindings.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/cannot-infer-async-enabled-impl-trait-bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-async-enabled-impl-trait-bindings" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-async-enabled-impl-trait-bindings/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `impl_trait_in_bindings` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(impl_trait_in_bindings)]
   |            ^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #63065 <https://github.com/rust-lang/rust/issues/63065> for more information

error[E0282]: type annotations needed
  --> /checkout/src/test/ui/inference/cannot-infer-async-enabled-impl-trait-bindings.rs:13:20
   |
LL |     let fut = async {
   |         --- consider giving `fut` a type
LL |         make_unit()?; //~ ERROR type annotations needed
   |                    ^ cannot infer type
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0282`.


------------------------------------------


---- [ui] ui/inference/cannot-infer-closure.rs stdout ----
diff of stderr:

- error[E0282]: type annotations needed for the closure `fn((), ()) -> std::result::Result<(), _>`
+ error[E0282]: type annotations needed
3    |
3    |
+ LL |     let x = |a: (), b: ()| {
+    |         - consider giving `x` a type
4 LL |         Err(a)?;
5    |               ^ cannot infer type
-    |
- help: give this closure an explicit return type without `_` placeholders
-    |
- LL |     let x = |a: (), b: ()| -> std::result::Result<(), _> {
11 
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-closure/cannot-infer-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inference/cannot-infer-closure.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/cannot-infer-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-closure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-closure/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/inference/cannot-infer-closure.rs:3:15
   |
LL |     let x = |a: (), b: ()| {
   |         - consider giving `x` a type
LL |         Err(a)?; //~ ERROR type annotations needed for the closure
   |               ^ cannot infer type
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.


------------------------------------------


---- [ui] ui/issues/issue-12187-1.rs stdout ----
diff of stderr:

- error[E0282]: type annotations needed for `&T`
+ error[E0282]: type annotations needed
3    |
3    |
4 LL |     let &v = new();
5    |         -^
6    |         ||
7    |         |cannot infer type
7    |         |cannot infer type
-    |         consider giving this pattern the explicit type `&T`, with the type parameters specified
+    |         consider giving this pattern a type
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12187-1/issue-12187-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-12187-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12187-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12187-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12187-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-12187-1.rs:6:10
   |
LL |     let &v = new();
   |         -^
   |         ||
   |         |cannot infer type
   |         consider giving this pattern a type
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.


------------------------------------------


---- [ui] ui/issues/issue-12187-2.rs stdout ----
diff of stderr:

- error[E0282]: type annotations needed for `&T`
+ error[E0282]: type annotations needed
3    |
3    |
4 LL |     let &v = new();
5    |         -^
6    |         ||
7    |         |cannot infer type
7    |         |cannot infer type
-    |         consider giving this pattern the explicit type `&T`, with the type parameters specified
+    |         consider giving this pattern a type
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12187-2/issue-12187-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-12187-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12187-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12187-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12187-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-12187-2.rs:6:10
   |
LL |     let &v = new();
   |         -^
   |         ||
   |         |cannot infer type
   |         consider giving this pattern a type
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.


------------------------------------------


---- [ui] ui/issues/issue-17551.rs stdout ----
diff of stderr:

- error[E0282]: type annotations needed for `B<T>`
+ error[E0282]: type annotations needed
3    |
3    |
4 LL |     let foo = B(marker::PhantomData);

5    |         ---   ^ cannot infer type for type parameter `T` declared on the struct `B`
6    |         |
-    |         consider giving `foo` the explicit type `B<T>`, where the type parameter `T` is specified
+    |         consider giving `foo` a type
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17551/issue-17551.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-17551.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17551.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17551" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17551/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-17551.rs:6:15
   |
LL |     let foo = B(marker::PhantomData); //~ ERROR type annotations needed
   |         ---   ^ cannot infer type for type parameter `T` declared on the struct `B`
   |         |
   |         consider giving `foo` a type
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.


------------------------------------------


---- [ui] ui/issues/issue-20261.rs stdout ----
diff of stderr:

- error[E0282]: type annotations needed for `&(_,)`
+ error[E0282]: type annotations needed
3    |
3    |
4 LL |     for (ref i,) in [].iter() {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20261/issue-20261.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20261/issue-20261.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-20261.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20261.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20261" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20261/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-20261.rs:4:11
   |
LL |     for (ref i,) in [].iter() {
   |                     --------- the element type for this iterator is not specified
LL |         i.clone();
   |           ^^^^^ cannot infer type
   |
   = note: type must be known at this point
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.


------------------------------------------


---- [ui] ui/issues/issue-23046.rs stdout ----
diff of stderr:

- error[E0282]: type annotations needed for `Expr<'_, VAR>`
+ error[E0282]: type annotations needed
3    |
3    |
4 LL |     let ex = |x| {

-    |               ^ consider giving this closure parameter the explicit type `Expr<'_, VAR>`, where the type parameter `VAR` is specified
+    |               ^ consider giving this closure parameter a type
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23046/issue-23046.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-23046.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23046.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23046" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23046/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-23046.rs:17:15
   |
LL |     let ex = |x| { //~ ERROR type annotations needed
   |               ^ consider giving this closure parameter a type
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.


------------------------------------------


---- [ui] ui/issues/issue-25368.rs stdout ----
diff of stderr:

- error[E0282]: type annotations needed for `(Sender<Foo<T>>, std::sync::mpsc::Receiver<Foo<T>>)`
+ error[E0282]: type annotations needed
3    |
3    |
4 LL |     let (tx, rx) = channel();

-    |         -------- consider giving this pattern the explicit type `(Sender<Foo<T>>, std::sync::mpsc::Receiver<Foo<T>>)`, where the type parameter `T` is specified
+    |         -------- consider giving this pattern a type
6 ...
7 LL |         tx.send(Foo{ foo: PhantomData });
8    |                 ^^^ cannot infer type for type parameter `T` declared on the struct `Foo`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25368/issue-25368.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25368/issue-25368.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-25368.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-25368.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25368" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25368/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-25368.rs:11:17
   |
LL |     let (tx, rx) = channel();
   |         -------- consider giving this pattern a type
...
LL |         tx.send(Foo{ foo: PhantomData }); //~ ERROR E0282
   |                 ^^^ cannot infer type for type parameter `T` declared on the struct `Foo`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.


------------------------------------------


---- [ui] ui/issues/issue-72690.rs stdout ----
diff of stderr:

13 LL |     |x| String::from("x".as_ref());
14    |      ^ consider giving this closure parameter a type
15 
- error[E0283]: type annotations needed for `&T`
+ error[E0283]: type annotations needed
18    |
18    |
19 LL |     let _ = "x".as_ref();

20    |         -       ^^^^^^ cannot infer type for type parameter `T` declared on the trait `AsRef`
21    |         |
-    |         consider giving this pattern the explicit type `&T`, where the type parameter `T` is specified
+    |         consider giving this pattern a type
23    |
24    = note: cannot satisfy `str: AsRef<_>`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-72690/issue-72690.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-72690/issue-72690.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-72690.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-72690.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-72690" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-72690/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-72690.rs:7:5
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
   |
   = note: cannot satisfy `String: From<&_>`
   = note: required by `from`
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-72690.rs:11:6
   |
   |
LL |     |x| String::from("x".as_ref()); //~ ERROR type annotations needed
   |      ^ consider giving this closure parameter a type
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-72690.rs:15:17
   |
   |
LL |     let _ = "x".as_ref(); //~ ERROR type annotations needed
   |         -       ^^^^^^ cannot infer type for type parameter `T` declared on the trait `AsRef`
   |         |
   |         consider giving this pattern a type
   |
   = note: cannot satisfy `str: AsRef<_>`
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-72690.rs:19:5
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
   |
   = note: cannot satisfy `String: From<&_>`
   = note: required by `from`
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-72690.rs:25:5
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
   |
   = note: cannot satisfy `String: From<&_>`
   = note: required by `from`
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-72690.rs:33:5
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
   |
   = note: cannot satisfy `String: From<&_>`
   = note: required by `from`
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-72690.rs:41:5
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
   |
   = note: cannot satisfy `String: From<&_>`
   = note: required by `from`
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-72690.rs:47:5
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
   |
   = note: cannot satisfy `String: From<&_>`
   = note: required by `from`
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-72690.rs:55:5
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
   |
   = note: cannot satisfy `String: From<&_>`
   = note: required by `from`
error: aborting due to 9 previous errors

Some errors have detailed explanations: E0282, E0283.
For more information about an error, try `rustc --explain E0282`.
For more information about an error, try `rustc --explain E0282`.

------------------------------------------
---

---- [ui] ui/suggestions/fn-needing-specified-return-type-param.rs stdout ----
diff of stderr:

- error[E0282]: type annotations needed for `fn() -> A`
+ error[E0282]: type annotations needed
2   --> $DIR/fn-needing-specified-return-type-param.rs:3:13
3    |
4 LL |     let _ = f;

5    |         -   ^ cannot infer type for type parameter `A` declared on the function `f`
6    |         |
-    |         consider giving this pattern the explicit type `fn() -> A`, where the type parameter `A` is specified
+    |         consider giving this pattern a type
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/fn-needing-specified-return-type-param/fn-needing-specified-return-type-param.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/fn-needing-specified-return-type-param.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/fn-needing-specified-return-type-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/fn-needing-specified-return-type-param" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/fn-needing-specified-return-type-param/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/suggestions/fn-needing-specified-return-type-param.rs:3:13
   |
LL |     let _ = f; //~ ERROR type annotations needed for `fn() -> A`
   |         -   ^ cannot infer type for type parameter `A` declared on the function `f`
   |         |
   |         consider giving this pattern a type
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.


------------------------------------------


---- [ui] ui/suggestions/suggest-closure-return-type-1.rs stdout ----
diff of stderr:

- error[E0282]: type annotations needed for the closure `fn() -> [_; 0]`
+ error[E0282]: type annotations needed
2   --> $DIR/suggest-closure-return-type-1.rs:2:24
3    |
4 LL |     let _v = || -> _ { [] };
-    |                        ^^ cannot infer type
-    |
-    |
- help: give this closure an explicit return type without `_` placeholders
-    |
- LL |     let _v = || -> [_; 0] { [] };
+    |         --             ^^ cannot infer type
+    |         |
+    |         |
+    |         consider giving `_v` a type
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-closure-return-type-1/suggest-closure-return-type-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/suggest-closure-return-type-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-closure-return-type-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-closure-return-type-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-closure-return-type-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/suggestions/suggest-closure-return-type-1.rs:2:24
   |
LL |     let _v = || -> _ { [] }; //~ ERROR type annotations needed for the closure
   |         --             ^^ cannot infer type
   |         |
   |         consider giving `_v` a type
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.


------------------------------------------


---- [ui] ui/suggestions/suggest-closure-return-type-2.rs stdout ----
diff of stderr:

- error[E0282]: type annotations needed for the closure `fn() -> [_; 0]`
+ error[E0282]: type annotations needed
2   --> $DIR/suggest-closure-return-type-2.rs:2:19
3    |
4 LL |     let _v = || { [] };
-    |                   ^^ cannot infer type
-    |
-    |
- help: give this closure an explicit return type without `_` placeholders
-    |
- LL |     let _v = || -> [_; 0] { [] };
+    |         --        ^^ cannot infer type
+    |         |
+    |         |
+    |         consider giving `_v` a type
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-closure-return-type-2/suggest-closure-return-type-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/suggest-closure-return-type-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-closure-return-type-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-closure-return-type-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-closure-return-type-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/suggestions/suggest-closure-return-type-2.rs:2:19
   |
LL |     let _v = || { [] }; //~ ERROR type annotations needed for the closure
   |         --        ^^ cannot infer type
   |         |
   |         consider giving `_v` a type
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.


------------------------------------------


---- [ui] ui/suggestions/suggest-closure-return-type-3.rs stdout ----
diff of stderr:

- error[E0282]: type annotations needed for the closure `fn() -> [_; 0]`
+ error[E0282]: type annotations needed
2   --> $DIR/suggest-closure-return-type-3.rs:2:17
3    |
4 LL |     let _v = || [];
-    |                 ^^ cannot infer type
-    |
-    |
- help: give this closure an explicit return type without `_` placeholders
-    |
- LL |     let _v = || -> [_; 0] { [] };
-    |                 ^^^^^^^^^^^    ^
+    |         --      ^^ cannot infer type
+    |         |
+    |         consider giving `_v` a type
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-closure-return-type-3/suggest-closure-return-type-3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/suggest-closure-return-type-3.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-closure-return-type-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-closure-return-type-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-closure-return-type-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/suggestions/suggest-closure-return-type-3.rs:2:17
   |
LL |     let _v = || []; //~ ERROR type annotations needed for the closure
   |         --      ^^ cannot infer type
   |         |
   |         consider giving `_v` a type
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.


------------------------------------------


---- [ui] ui/traits/issue-77982.rs stdout ----
diff of stderr:

19    = note: cannot satisfy `u32: From<_>`
20    = note: required by `from`
21 
- error[E0283]: type annotations needed for `Box<T>`
+ error[E0283]: type annotations needed
24    |
24    |
25 LL |     let _ = ().foo();

26    |         -      ^^^ cannot infer type for type parameter `T` declared on the trait `Foo`
27    |         |
-    |         consider giving this pattern the explicit type `Box<T>`, where the type parameter `T` is specified
+    |         consider giving this pattern a type
29    |
30    = note: cannot satisfy `(): Foo<'_, _>`


- error[E0283]: type annotations needed for `Box<T>`
+ error[E0283]: type annotations needed
34    |
34    |
35 LL |     let _ = (&()).bar();

36    |         -         ^^^ cannot infer type for type parameter `T` declared on the trait `Bar`
37    |         |
-    |         consider giving this pattern the explicit type `Box<T>`, where the type parameter `T` is specified
+    |         consider giving this pattern a type
39    |
40    = note: cannot satisfy `&(): Bar<'_, _>`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/issue-77982.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/issue-77982.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-77982.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-77982.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:8:10
   |
LL |     opts.get(opt.as_ref()); //~ ERROR type annotations needed
   |          ^^^ ------------ this method call resolves to `&T`
   |          |
   |          cannot infer type for type parameter `Q` declared on the associated function `get`
   |
   = note: cannot satisfy `String: Borrow<_>`
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:12:44
   |
   |
LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(0u32.into())).collect();
   |                                            ^^^^^^^^^ ----------- this method call resolves to `T`
   |                                            |
   |                                            cannot infer type for type parameter `T` declared on the trait `From`
   |
   = note: cannot satisfy `u32: From<_>`
   = note: required by `from`
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:35:16
   |
   |
LL |     let _ = ().foo(); //~ ERROR type annotations needed
   |         -      ^^^ cannot infer type for type parameter `T` declared on the trait `Foo`
   |         |
   |         consider giving this pattern a type
   |
   = note: cannot satisfy `(): Foo<'_, _>`
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:39:19
   |
   |
LL |     let _ = (&()).bar(); //~ ERROR type annotations needed
   |         -         ^^^ cannot infer type for type parameter `T` declared on the trait `Bar`
   |         |
   |         consider giving this pattern a type
   |
   = note: cannot satisfy `&(): Bar<'_, _>`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0283`.


------------------------------------------


---- [ui] ui/type/type-check/cannot_infer_local_or_vec_in_tuples.rs stdout ----
diff of stderr:

- error[E0282]: type annotations needed for `(Vec<T>,)`
+ error[E0282]: type annotations needed
3    |
3    |
4 LL |     let (x, ) = (vec![], );

5    |         -----    ^^^^^^ cannot infer type for type parameter `T`
6    |         |
-    |         consider giving this pattern the explicit type `(Vec<T>,)`, where the type parameter `T` is specified
+    |         consider giving this pattern a type
9    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/cannot_infer_local_or_vec_in_tuples/cannot_infer_local_or_vec_in_tuples.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type/type-check/cannot_infer_local_or_vec_in_tuples.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-check/cannot_infer_local_or_vec_in_tuples.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/cannot_infer_local_or_vec_in_tuples" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/cannot_infer_local_or_vec_in_tuples/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/type/type-check/cannot_infer_local_or_vec_in_tuples.rs:2:18
   |
LL |     let (x, ) = (vec![], );
   |         -----    ^^^^^^ cannot infer type for type parameter `T`
   |         |
   |         consider giving this pattern a type
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---

---- [ui] ui/type/type-check/cannot_infer_local_or_vec.rs stdout ----
diff of stderr:

- error[E0282]: type annotations needed for `Vec<T>`
+ error[E0282]: type annotations needed
3    |
3    |
4 LL |     let x = vec![];

5    |         -   ^^^^^^ cannot infer type for type parameter `T`
6    |         |
-    |         consider giving `x` the explicit type `Vec<T>`, where the type parameter `T` is specified
+    |         consider giving `x` a type
9    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/cannot_infer_local_or_vec/cannot_infer_local_or_vec.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type/type-check/cannot_infer_local_or_vec.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-check/cannot_infer_local_or_vec.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/cannot_infer_local_or_vec" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/cannot_infer_local_or_vec/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/type/type-check/cannot_infer_local_or_vec.rs:2:13
   |
LL |     let x = vec![];
   |         -   ^^^^^^ cannot infer type for type parameter `T`
   |         |
   |         consider giving `x` a type
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---

---- [ui] ui/type/type-check/cannot_infer_local_or_array.rs stdout ----
diff of stderr:

- error[E0282]: type annotations needed for `[_; 0]`
+ error[E0282]: type annotations needed
3    |
4 LL |     let x = [];


5    |         -   ^^ cannot infer type
6    |         |
-    |         consider giving `x` the explicit type `[_; 0]`, with the type parameters specified
+    |         consider giving `x` a type
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/cannot_infer_local_or_array/cannot_infer_local_or_array.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type/type-check/cannot_infer_local_or_array.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-check/cannot_infer_local_or_array.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/cannot_infer_local_or_array" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/cannot_infer_local_or_array/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/type/type-check/cannot_infer_local_or_array.rs:2:13
   |
LL |     let x = []; //~ ERROR type annotations needed
   |         -   ^^ cannot infer type
   |         |
   |         consider giving `x` a type
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.


------------------------------------------


---- [ui] ui/unboxed-closures/unboxed-closures-failed-recursive-fn-2.rs stdout ----
diff of stderr:

- error[E0282]: type annotations needed for `Option<T>`
+ error[E0282]: type annotations needed
3    |
4 LL |     let mut closure0 = None;


-    |         ------------ consider giving `closure0` the explicit type `Option<T>`, with the type parameters specified
+    |         ------------ consider giving `closure0` a type
7 LL |                         return c();
8    |                                ^^^ cannot infer type



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-2/unboxed-closures-failed-recursive-fn-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closures-failed-recursive-fn-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-2.rs:16:32
   |
LL |     let mut closure0 = None;
   |         ------------ consider giving `closure0` a type
LL |                         return c();
   |                                ^^^ cannot infer type
   |
   |
   = note: type must be known at this point
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.

---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:51
