plain

running 11545 tests
.................................................................................................... 100/11545
........................................i.........ii................................................ 200/11545
...................................F................................................................ 300/11545
........................................................................................F...F....... 400/11545
.................................................................F.................................. 500/11545
.................i............................................................................ii.... 700/11545
.................................................................................................... 800/11545
.................................................................................................... 900/11545
.................................................................................................... 1000/11545
---
.................................................................................................... 9300/11545
.................................................................................................... 9400/11545
..........................................................................i......i.................. 9500/11545
.................................................................................................... 9600/11545
....................iiiiiii..i.iiiiii............................................................... 9700/11545
.................................................................................................... 9900/11545
.................................................................................................... 10000/11545
.................................................................................................... 10100/11545
.................................................................................................... 10200/11545
---

---- [ui] ui/associated-type-bounds/bad-bounds-on-assoc-in-trait.rs stdout ----
diff of stderr:

10 LL | trait Case1 where <<Self as Case1>::C as Iterator>::Item: Iterator {
12 
12 
- error[E0277]: `<<Self as Case1>::C as Iterator>::Item` cannot be shared between threads safely
-   --> $DIR/bad-bounds-on-assoc-in-trait.rs:27:93
+ error[E0277]: `<<Self as Case1>::C as Iterator>::Item` cannot be sent between threads safely
15    |
15    |
16 LL |     type C: Clone + Iterator<Item: Send + Iterator<Item: for<'a> Lam<&'a u8, App: Debug>> + Sync>;
-    |                                                                                             ^^^^ `<<Self as Case1>::C as Iterator>::Item` cannot be shared between threads safely
+    |                                    ^^^^ `<<Self as Case1>::C as Iterator>::Item` cannot be sent between threads safely
18    | 
19   ::: $SRC_DIR/core/src/marker.rs:LL:COL

- LL | pub unsafe auto trait Sync {
- LL | pub unsafe auto trait Sync {
-    | -------------------------- required by this bound in `Sync`
+ LL | pub unsafe auto trait Send {
+    | -------------------------- required by this bound in `Send`
23    |
-    = help: the trait `Sync` is not implemented for `<<Self as Case1>::C as Iterator>::Item`
+    = help: the trait `Send` is not implemented for `<<Self as Case1>::C as Iterator>::Item`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
26    |
26    |
- LL | trait Case1 where <<Self as Case1>::C as Iterator>::Item: Sync {
+ LL | trait Case1 where <<Self as Case1>::C as Iterator>::Item: Send {
29 
29 
- error[E0277]: `<<Self as Case1>::C as Iterator>::Item` cannot be sent between threads safely
-   --> $DIR/bad-bounds-on-assoc-in-trait.rs:27:36
+ error[E0277]: `<<Self as Case1>::C as Iterator>::Item` cannot be shared between threads safely
32    |
32    |
33 LL |     type C: Clone + Iterator<Item: Send + Iterator<Item: for<'a> Lam<&'a u8, App: Debug>> + Sync>;
-    |                                    ^^^^ `<<Self as Case1>::C as Iterator>::Item` cannot be sent between threads safely
+    |                                                                                             ^^^^ `<<Self as Case1>::C as Iterator>::Item` cannot be shared between threads safely
35    | 
36   ::: $SRC_DIR/core/src/marker.rs:LL:COL

- LL | pub unsafe auto trait Send {
- LL | pub unsafe auto trait Send {
-    | -------------------------- required by this bound in `Send`
+ LL | pub unsafe auto trait Sync {
+    | -------------------------- required by this bound in `Sync`
40    |
-    = help: the trait `Send` is not implemented for `<<Self as Case1>::C as Iterator>::Item`
+    = help: the trait `Sync` is not implemented for `<<Self as Case1>::C as Iterator>::Item`
43    |
43    |
- LL | trait Case1 where <<Self as Case1>::C as Iterator>::Item: Send {
+ LL | trait Case1 where <<Self as Case1>::C as Iterator>::Item: Sync {
46 
47 error: aborting due to 3 previous errors



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/bad-bounds-on-assoc-in-trait/bad-bounds-on-assoc-in-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-type-bounds/bad-bounds-on-assoc-in-trait.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/bad-bounds-on-assoc-in-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/bad-bounds-on-assoc-in-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/bad-bounds-on-assoc-in-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `<<Self as Case1>::C as Iterator>::Item` is not an iterator
   |
   |
LL |     type C: Clone + Iterator<Item: Send + Iterator<Item: for<'a> Lam<&'a u8, App: Debug>> + Sync>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `<<Self as Case1>::C as Iterator>::Item` is not an iterator
   |
   = help: the trait `Iterator` is not implemented for `<<Self as Case1>::C as Iterator>::Item`
   |
   |
LL | trait Case1 where <<Self as Case1>::C as Iterator>::Item: Iterator {


error[E0277]: `<<Self as Case1>::C as Iterator>::Item` cannot be sent between threads safely
   |
   |
LL |     type C: Clone + Iterator<Item: Send + Iterator<Item: for<'a> Lam<&'a u8, App: Debug>> + Sync>;
   |                                    ^^^^ `<<Self as Case1>::C as Iterator>::Item` cannot be sent between threads safely
  ::: /checkout/library/core/src/marker.rs:38:1
   |
LL | pub unsafe auto trait Send {
LL | pub unsafe auto trait Send {
   | -------------------------- required by this bound in `Send`
   |
   = help: the trait `Send` is not implemented for `<<Self as Case1>::C as Iterator>::Item`
   |
   |
LL | trait Case1 where <<Self as Case1>::C as Iterator>::Item: Send {


error[E0277]: `<<Self as Case1>::C as Iterator>::Item` cannot be shared between threads safely
   |
   |
LL |     type C: Clone + Iterator<Item: Send + Iterator<Item: for<'a> Lam<&'a u8, App: Debug>> + Sync>;
   |                                                                                             ^^^^ `<<Self as Case1>::C as Iterator>::Item` cannot be shared between threads safely
  ::: /checkout/library/core/src/marker.rs:469:1
   |
LL | pub unsafe auto trait Sync {
LL | pub unsafe auto trait Sync {
   | -------------------------- required by this bound in `Sync`
   |
   = help: the trait `Sync` is not implemented for `<<Self as Case1>::C as Iterator>::Item`
   |
   |
LL | trait Case1 where <<Self as Case1>::C as Iterator>::Item: Sync {

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/associated-types/defaults-unsound-62211-1.rs stdout ----
diff of stderr:

- error[E0277]: the trait bound `Self: Copy` is not satisfied
-   --> $DIR/defaults-unsound-62211-1.rs:20:5
-    |
- LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
-    |     |            |
-    |     |            |
-    |     |            required by this bound in `UncheckedCopy::Output`
-    |     the trait `Copy` is not implemented for `Self`
- help: consider further restricting `Self`
-    |
-    |
- LL | trait UncheckedCopy: Sized + Copy {
- 
15 error[E0277]: the trait bound `Self: Deref` is not satisfied
16   --> $DIR/defaults-unsound-62211-1.rs:20:5
17    |
17    |

39    |
40 LL | trait UncheckedCopy: Sized + AddAssign<&'static str> {
+ 
+ 
+ error[E0277]: the trait bound `Self: Copy` is not satisfied
+    |
+    |
+ LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
+    |     |            |
+    |     |            |
+    |     |            required by this bound in `UncheckedCopy::Output`
+    |     the trait `Copy` is not implemented for `Self`
+ help: consider further restricting `Self`
+    |
+    |
+ LL | trait UncheckedCopy: Sized + Copy {
42 
42 
43 error[E0277]: `Self` doesn't implement `std::fmt::Display`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-1/defaults-unsound-62211-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/defaults-unsound-62211-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-unsound-62211-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `Self: Deref` is not satisfied
  --> /checkout/src/test/ui/associated-types/defaults-unsound-62211-1.rs:20:5
   |
LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
   |     |                   |
   |     |                   |
   |     |                   required by this bound in `UncheckedCopy::Output`
   |     the trait `Deref` is not implemented for `Self`
help: consider further restricting `Self`
   |
   |
LL | trait UncheckedCopy: Sized + Deref {


error[E0277]: cannot add-assign `&'static str` to `Self`
   |
   |
LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
   |     |                                         |
   |     |                                         |
   |     |                                         required by this bound in `UncheckedCopy::Output`
   |     no implementation for `Self += &'static str`
help: consider further restricting `Self`
   |
   |
LL | trait UncheckedCopy: Sized + AddAssign<&'static str> {


error[E0277]: the trait bound `Self: Copy` is not satisfied
   |
   |
LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
   |     |            |
   |     |            |
   |     |            required by this bound in `UncheckedCopy::Output`
   |     the trait `Copy` is not implemented for `Self`
help: consider further restricting `Self`
   |
   |
LL | trait UncheckedCopy: Sized + Copy {


error[E0277]: `Self` doesn't implement `std::fmt::Display`
   |
   |
LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
   |     |                                                                                |
   |     |                                                                                |
   |     |                                                                                required by this bound in `UncheckedCopy::Output`
   |     `Self` cannot be formatted with the default formatter
   |
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
help: consider further restricting `Self`
   |
LL | trait UncheckedCopy: Sized + std::fmt::Display {

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/associated-types/defaults-unsound-62211-2.rs stdout ----
diff of stderr:

- error[E0277]: the trait bound `Self: Copy` is not satisfied
-   --> $DIR/defaults-unsound-62211-2.rs:20:5
-    |
- LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
-    |     |            |
-    |     |            |
-    |     |            required by this bound in `UncheckedCopy::Output`
-    |     the trait `Copy` is not implemented for `Self`
- help: consider further restricting `Self`
-    |
-    |
- LL | trait UncheckedCopy: Sized + Copy {
- 
15 error[E0277]: the trait bound `Self: Deref` is not satisfied
16   --> $DIR/defaults-unsound-62211-2.rs:20:5
17    |
17    |

39    |
40 LL | trait UncheckedCopy: Sized + AddAssign<&'static str> {
+ 
+ 
+ error[E0277]: the trait bound `Self: Copy` is not satisfied
+    |
+    |
+ LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
+    |     |            |
+    |     |            |
+    |     |            required by this bound in `UncheckedCopy::Output`
+    |     the trait `Copy` is not implemented for `Self`
+ help: consider further restricting `Self`
+    |
+    |
+ LL | trait UncheckedCopy: Sized + Copy {
42 
42 
43 error[E0277]: `Self` doesn't implement `std::fmt::Display`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-2/defaults-unsound-62211-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/defaults-unsound-62211-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-unsound-62211-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `Self: Deref` is not satisfied
  --> /checkout/src/test/ui/associated-types/defaults-unsound-62211-2.rs:20:5
   |
LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
   |     |                   |
   |     |                   |
   |     |                   required by this bound in `UncheckedCopy::Output`
   |     the trait `Deref` is not implemented for `Self`
help: consider further restricting `Self`
   |
   |
LL | trait UncheckedCopy: Sized + Deref {


error[E0277]: cannot add-assign `&'static str` to `Self`
   |
   |
LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
   |     |                                         |
   |     |                                         |
   |     |                                         required by this bound in `UncheckedCopy::Output`
   |     no implementation for `Self += &'static str`
help: consider further restricting `Self`
   |
   |
LL | trait UncheckedCopy: Sized + AddAssign<&'static str> {


error[E0277]: the trait bound `Self: Copy` is not satisfied
   |
   |
LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
   |     |            |
   |     |            |
   |     |            required by this bound in `UncheckedCopy::Output`
   |     the trait `Copy` is not implemented for `Self`
help: consider further restricting `Self`
   |
   |
LL | trait UncheckedCopy: Sized + Copy {


error[E0277]: `Self` doesn't implement `std::fmt::Display`
   |
   |
LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
   |     |                                                                                |
   |     |                                                                                |
   |     |                                                                                required by this bound in `UncheckedCopy::Output`
   |     `Self` cannot be formatted with the default formatter
   |
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
help: consider further restricting `Self`
   |
LL | trait UncheckedCopy: Sized + std::fmt::Display {

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/associated-types/trait-with-supertraits-needing-sized-self.rs stdout ----
diff of stderr:

1 error[E0277]: the size for values of type `Self` cannot be known at compilation time
-   --> $DIR/trait-with-supertraits-needing-sized-self.rs:3:22
3    |
3    |
4 LL | trait ArithmeticOps: Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> {}
-    |                      ^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
+    |                                         ^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
6    | 
7   ::: $SRC_DIR/core/src/ops/arith.rs:LL:COL


- LL | pub trait Add<Rhs = Self> {
-    |               --- required by this bound in `Add`
+ LL | pub trait Sub<Rhs = Self> {
+    |               --- required by this bound in `Sub`
12 help: consider further restricting `Self`
13    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/trait-with-supertraits-needing-sized-self/trait-with-supertraits-needing-sized-self.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/trait-with-supertraits-needing-sized-self.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/trait-with-supertraits-needing-sized-self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/trait-with-supertraits-needing-sized-self" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/trait-with-supertraits-needing-sized-self/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `Self` cannot be known at compilation time
   |
   |
LL | trait ArithmeticOps: Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> {}
   |                                         ^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  ::: /checkout/library/core/src/ops/arith.rs:181:15
   |
   |
LL | pub trait Sub<Rhs = Self> {
   |               --- required by this bound in `Sub`
help: consider further restricting `Self`
   |
   |
LL | trait ArithmeticOps: Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Sized {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
---
diff of stderr:

13   --> $DIR/E0225.rs:8:20
14    |
15 LL | trait Foo = std::io::Read + std::io::Write;
-    |             -------------   -------------- first non-auto trait
+    |             -------------   -------------- additional non-auto trait
-    |             additional non-auto trait
+    |             first non-auto trait
19 ...
19 ...
20 LL |     let _: Box<dyn Foo>;


23    |                    trait alias used in trait object type (additional use)
24    |                    trait alias used in trait object type (first use)
25    |
-    = help: consider creating a new trait with all of these as super-traits and using that trait here instead: `trait NewTrait: std::io::Write + std::io::Read {}`
+    = help: consider creating a new trait with all of these as super-traits and using that trait here instead: `trait NewTrait: std::io::Read + std::io::Write {}`
27    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
29 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0225/E0225.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0225.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0225.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0225" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0225/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0225]: only auto traits can be used as additional traits in a trait object
   |
   |
LL |     let _: Box<dyn std::io::Read + std::io::Write>;
   |                    -------------   ^^^^^^^^^^^^^^ additional non-auto trait
   |                    first non-auto trait
   |
   |
   = help: consider creating a new trait with all of these as super-traits and using that trait here instead: `trait NewTrait: std::io::Read + std::io::Write {}`
   = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>

error[E0225]: only auto traits can be used as additional traits in a trait object
   |
   |
LL | trait Foo = std::io::Read + std::io::Write;
   |             -------------   -------------- additional non-auto trait
   |             first non-auto trait
...
...
LL |     let _: Box<dyn Foo>;
   |                    |
   |                    |
   |                    trait alias used in trait object type (additional use)
   |                    trait alias used in trait object type (first use)
   |
   = help: consider creating a new trait with all of these as super-traits and using that trait here instead: `trait NewTrait: std::io::Read + std::io::Write {}`
   = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0225`.


------------------------------------------


---- [ui] ui/issues/issue-40827.rs stdout ----
diff of stderr:

- error[E0277]: `Rc<Foo>` cannot be shared between threads safely
+ error[E0277]: `Rc<Foo>` cannot be sent between threads safely
3    |
3    |
4 LL | fn f<T: Send>(_: T) {}

5    |         ---- required by this bound in `f`
6 ...
7 LL |     f(Foo(Arc::new(Bar::B(None))));
-    |     ^ `Rc<Foo>` cannot be shared between threads safely
+    |     ^ `Rc<Foo>` cannot be sent between threads safely
9    |
-    = help: within `Bar`, the trait `Sync` is not implemented for `Rc<Foo>`
+    = help: within `Bar`, the trait `Send` is not implemented for `Rc<Foo>`
11    = note: required because it appears within the type `Bar`
12    = note: required because of the requirements on the impl of `Send` for `Arc<Bar>`
13    = note: required because it appears within the type `Foo`
14 
14 
- error[E0277]: `Rc<Foo>` cannot be sent between threads safely
+ error[E0277]: `Rc<Foo>` cannot be shared between threads safely
17    |
17    |
18 LL | fn f<T: Send>(_: T) {}

19    |         ---- required by this bound in `f`
20 ...
21 LL |     f(Foo(Arc::new(Bar::B(None))));
-    |     ^ `Rc<Foo>` cannot be sent between threads safely
+    |     ^ `Rc<Foo>` cannot be shared between threads safely
23    |
-    = help: within `Bar`, the trait `Send` is not implemented for `Rc<Foo>`
+    = help: within `Bar`, the trait `Sync` is not implemented for `Rc<Foo>`
25    = note: required because it appears within the type `Bar`
26    = note: required because of the requirements on the impl of `Send` for `Arc<Bar>`
27    = note: required because it appears within the type `Foo`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40827/issue-40827.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40827/issue-40827.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-40827.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-40827.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40827" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40827/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `Rc<Foo>` cannot be sent between threads safely
   |
   |
LL | fn f<T: Send>(_: T) {}
   |         ---- required by this bound in `f`
...
LL |     f(Foo(Arc::new(Bar::B(None))));
   |     ^ `Rc<Foo>` cannot be sent between threads safely
   |
   = help: within `Bar`, the trait `Send` is not implemented for `Rc<Foo>`
   = note: required because it appears within the type `Bar`
   = note: required because of the requirements on the impl of `Send` for `Arc<Bar>`
   = note: required because it appears within the type `Foo`

error[E0277]: `Rc<Foo>` cannot be shared between threads safely
   |
   |
LL | fn f<T: Send>(_: T) {}
   |         ---- required by this bound in `f`
...
LL |     f(Foo(Arc::new(Bar::B(None))));
   |     ^ `Rc<Foo>` cannot be shared between threads safely
   |
   = help: within `Bar`, the trait `Sync` is not implemented for `Rc<Foo>`
   = note: required because it appears within the type `Bar`
   = note: required because of the requirements on the impl of `Send` for `Arc<Bar>`
   = note: required because it appears within the type `Foo`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/traits/alias/cross-crate.rs stdout ----
diff of stderr:

- error[E0277]: `Rc<u32>` cannot be shared between threads safely
+ error[E0277]: `Rc<u32>` cannot be sent between threads safely
3    |
3    |
4 LL | fn use_alias<T: SendSync>() {}

5    |                 -------- required by this bound in `use_alias`
6 ...
7 LL |     use_alias::<Rc<u32>>();
-    |                 ^^^^^^^ `Rc<u32>` cannot be shared between threads safely
+    |                 ^^^^^^^ `Rc<u32>` cannot be sent between threads safely
9    |
-    = help: the trait `Sync` is not implemented for `Rc<u32>`
+    = help: the trait `Send` is not implemented for `Rc<u32>`
11 
- error[E0277]: `Rc<u32>` cannot be sent between threads safely
+ error[E0277]: `Rc<u32>` cannot be shared between threads safely
14    |
14    |
15 LL | fn use_alias<T: SendSync>() {}

16    |                 -------- required by this bound in `use_alias`
17 ...
18 LL |     use_alias::<Rc<u32>>();
-    |                 ^^^^^^^ `Rc<u32>` cannot be sent between threads safely
+    |                 ^^^^^^^ `Rc<u32>` cannot be shared between threads safely
20    |
-    = help: the trait `Send` is not implemented for `Rc<u32>`
+    = help: the trait `Sync` is not implemented for `Rc<u32>`
23 error: aborting due to 2 previous errors
24 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/cross-crate/cross-crate.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/alias/cross-crate.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/alias/cross-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/cross-crate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/cross-crate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `Rc<u32>` cannot be sent between threads safely
   |
   |
LL | fn use_alias<T: SendSync>() {}
   |                 -------- required by this bound in `use_alias`
...
LL |     use_alias::<Rc<u32>>();
   |                 ^^^^^^^ `Rc<u32>` cannot be sent between threads safely
   |
   = help: the trait `Send` is not implemented for `Rc<u32>`

error[E0277]: `Rc<u32>` cannot be shared between threads safely
   |
   |
LL | fn use_alias<T: SendSync>() {}
   |                 -------- required by this bound in `use_alias`
...
LL |     use_alias::<Rc<u32>>();
   |                 ^^^^^^^ `Rc<u32>` cannot be shared between threads safely
   |
   = help: the trait `Sync` is not implemented for `Rc<u32>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.

---
test result: FAILED. 11445 passed; 7 failed; 93 ignored; 0 measured; 0 filtered out; finished in 139.16s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:05
