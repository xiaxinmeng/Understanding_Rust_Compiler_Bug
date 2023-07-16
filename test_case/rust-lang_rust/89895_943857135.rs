plain
.................................................................................................... 5800/12292
.................................................................................................... 5900/12292
.................................................................i.................................. 6000/12292
.................................................................................................... 6100/12292
...............................................................i.....................F.............. 6200/12292
F...................................i............................................................... 6300/12292
...........................................F.................................................F.ii.ii 6400/12292
.................................................i....i.........................................i... 6600/12292
.............i..............i...............................................i....................... 6700/12292
.............................................................................i...................... 6800/12292
.................................................................................................... 6900/12292
---
.................................................................................................... 10400/12292
.................................................................................................... 10500/12292
.................................................................................................... 10600/12292
.................................................................................................... 10700/12292
..............................................F....................F................................ 10800/12292
...................................................ii...............................i............... 11000/12292
.................................................................................................... 11100/12292
.................................................................................................... 11200/12292
.................................................................................................... 11300/12292
---

---- [ui] ui/issues/issue-61108.rs stdout ----
diff of stderr:

4 LL |     let mut bad_letters = vec!['e', 't', 'o', 'i'];
5    |         --------------- move occurs because `bad_letters` has type `Vec<char>`, which does not implement the `Copy` trait
6 LL |     for l in bad_letters {
-    |              |
-    |              |
-    |              `bad_letters` moved due to this implicit call to `.into_iter()`
-    |              help: consider borrowing to avoid moving into the for loop: `&bad_letters`
+    |              ----------- `bad_letters` moved due to this implicit call to `.into_iter()`
11 ...
12 LL |     bad_letters.push('s');
13    |     ^^^^^^^^^^^^^^^^^^^^^ value borrowed here after move

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-61108/issue-61108.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-61108.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-61108.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-61108" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-61108/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0382]: borrow of moved value: `bad_letters`
   |
   |
LL |     let mut bad_letters = vec!['e', 't', 'o', 'i'];
   |         --------------- move occurs because `bad_letters` has type `Vec<char>`, which does not implement the `Copy` trait
LL |     for l in bad_letters {
   |              ----------- `bad_letters` moved due to this implicit call to `.into_iter()`
...
LL |     bad_letters.push('s'); //~ ERROR borrow of moved value: `bad_letters`
   |     ^^^^^^^^^^^^^^^^^^^^^ value borrowed here after move
   |
note: this function takes ownership of the receiver `self`, which moves `bad_letters`
   |
LL |     fn into_iter(self) -> Self::IntoIter;
   |                  ^^^^

---

---- [ui] ui/issues/issue-64559.rs stdout ----
diff of stderr:

4 LL |     let orig = vec![true];
5    |         ---- move occurs because `orig` has type `Vec<bool>`, which does not implement the `Copy` trait
6 LL |     for _val in orig {}
-    |                 |
-    |                 |
-    |                 `orig` moved due to this implicit call to `.into_iter()`
-    |                 help: consider borrowing to avoid moving into the for loop: `&orig`
+    |                 ---- `orig` moved due to this implicit call to `.into_iter()`
11 LL |     let _closure = || orig;
12    |                    ^^ ---- use occurs due to use in closure

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-64559/issue-64559.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-64559.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-64559.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-64559" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-64559/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0382]: use of moved value: `orig`
   |
   |
LL |     let orig = vec![true];
   |         ---- move occurs because `orig` has type `Vec<bool>`, which does not implement the `Copy` trait
LL |     for _val in orig {}
   |                 ---- `orig` moved due to this implicit call to `.into_iter()`
LL |     let _closure = || orig;
   |                    ^^ ---- use occurs due to use in closure
   |                    |
   |                    value used here after move
   |
note: this function takes ownership of the receiver `self`, which moves `orig`
   |
LL |     fn into_iter(self) -> Self::IntoIter;
   |                  ^^^^

---

15    |
16 LL |     fn into_iter(self) -> Self::IntoIter;
17    |                  ^^^^
- help: consider creating a fresh reborrow of `v` here
-    |
- LL |     for n in &mut *v {
22 
23 error: aborting due to previous error
24 

---

6 
7     let mut max = 0;
8 
-     for n in &mut *v {
+     for n in v {
10         max = std::cmp::max(max, *n);
12 


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-83924/issue-83924.fixed
To only update this specific test, also pass `--test-args issues/issue-83924.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-83924.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-83924" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-83924/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0382]: use of moved value: `v`
  --> /checkout/src/test/ui/issues/issue-83924.rs:15:14
   |
LL |     let v = &mut values;
   |         - move occurs because `v` has type `&mut Vec<i32>`, which does not implement the `Copy` trait
...
LL |     for n in v {
   |              - `v` moved due to this implicit call to `.into_iter()`
...
LL |     for n in v {
   |              ^ value used here after move
   |
note: this function takes ownership of the receiver `self`, which moves `v`
   |
LL |     fn into_iter(self) -> Self::IntoIter;
   |                  ^^^^

---
---- [ui] ui/iterators/into-iter-on-arrays-2018.rs stdout ----
diff of stderr:

57    |                        ~~~~
58 help: or remove `.into_iter()` to iterate by value
59    |
- LL -     for _ in [1, 2, 3].into_iter() {}
- LL +     for _ in [1, 2, 3] {}
+ LL - // check-pass
+ LL + .into_iter() {}
63 
64 warning: 5 warnings emitted



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-2018/into-iter-on-arrays-2018.stderr
To only update this specific test, also pass `--test-args iterators/into-iter-on-arrays-2018.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/into-iter-on-arrays-2018.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-2018" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-2018/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     let _: Iter<'_, i32> = array.into_iter();
   |
   = note: `#[warn(array_into_iter)]` on by default
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
   |
LL |     let _: Iter<'_, i32> = array.iter();
   |                                  ~~~~
help: or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value
   |
LL |     let _: Iter<'_, i32> = IntoIterator::into_iter(array);
   |                            ++++++++++++++++++++++++     ~

warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     let _: Iter<'_, i32> = Box::new(array).into_iter();
   |                                            ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     let _: Iter<'_, i32> = Rc::new(array).into_iter();
   |                                           ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     let _: Iter<'_, i32> = Array(array).into_iter();
   |                                         ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     for _ in [1, 2, 3].into_iter() {}
   |
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
   |
LL |     for _ in [1, 2, 3].iter() {}
   |                        ~~~~
help: or remove `.into_iter()` to iterate by value
LL - // check-pass
LL + .into_iter() {}
   | 

---

---- [ui] ui/loops/issue-82916.rs stdout ----
diff of stderr:

4 LL | fn foo(x: Vec<S>) {
5    |        - move occurs because `x` has type `Vec<S>`, which does not implement the `Copy` trait
6 LL |     for y in x {
-    |              |
-    |              |
-    |              `x` moved due to this implicit call to `.into_iter()`
-    |              help: consider borrowing to avoid moving into the for loop: `&x`
+    |              - `x` moved due to this implicit call to `.into_iter()`
11 ...
12 LL |     let z = x;
13    |             ^ value used here after move

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/issue-82916/issue-82916.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args loops/issue-82916.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/loops/issue-82916.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/issue-82916" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/issue-82916/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0382]: use of moved value: `x`
   |
   |
LL | fn foo(x: Vec<S>) {
   |        - move occurs because `x` has type `Vec<S>`, which does not implement the `Copy` trait
LL |     for y in x {
   |              - `x` moved due to this implicit call to `.into_iter()`
...
LL |     let z = x; //~ ERROR use of moved value: `x`
   |             ^ value used here after move
   |
note: this function takes ownership of the receiver `self`, which moves `x`
   |
LL |     fn into_iter(self) -> Self::IntoIter;
   |                  ^^^^

---

---- [ui] ui/moves/move-fn-self-receiver.rs stdout ----
diff of stderr:

119 LL |     let implicit_into_iter = vec![true];
120    |         ------------------ move occurs because `implicit_into_iter` has type `Vec<bool>`, which does not implement the `Copy` trait
121 LL |     for _val in implicit_into_iter {}
-    |                 |
-    |                 |
-    |                 `implicit_into_iter` moved due to this implicit call to `.into_iter()`
-    |                 help: consider borrowing to avoid moving into the for loop: `&implicit_into_iter`
+    |                 ------------------ `implicit_into_iter` moved due to this implicit call to `.into_iter()`
126 LL |     implicit_into_iter;
127    |     ^^^^^^^^^^^^^^^^^^ value used here after move


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-fn-self-receiver/move-fn-self-receiver.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-fn-self-receiver/move-fn-self-receiver.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args moves/move-fn-self-receiver.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/moves/move-fn-self-receiver.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-fn-self-receiver" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-fn-self-receiver/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0382]: use of moved value: `val.0`
   |
   |
LL |     val.0.into_iter().next();
   |           ----------- `val.0` moved due to this method call
LL |     val.0; //~ ERROR use of moved
   |     ^^^^^ value used here after move
   |
note: this function takes ownership of the receiver `self`, which moves `val.0`
   |
LL |     fn into_iter(self) -> Self::IntoIter;
   |                  ^^^^
   |                  ^^^^
   = note: move occurs because `val.0` has type `Vec<bool>`, which does not implement the `Copy` trait
error[E0382]: use of moved value: `foo`
  --> /checkout/src/test/ui/moves/move-fn-self-receiver.rs:34:5
   |
LL |     let foo = Foo;
LL |     let foo = Foo;
   |         --- move occurs because `foo` has type `Foo`, which does not implement the `Copy` trait
LL |     foo.use_self();
   |         ---------- `foo` moved due to this method call
LL |     foo; //~ ERROR use of moved
   |     ^^^ value used here after move
   |
note: this function takes ownership of the receiver `self`, which moves `foo`
   |
   |
LL |     fn use_self(self) {}


error[E0382]: use of moved value: `second_foo`
   |
LL |     let second_foo = Foo;
LL |     let second_foo = Foo;
   |         ---------- move occurs because `second_foo` has type `Foo`, which does not implement the `Copy` trait
LL |     second_foo.use_self();
   |                ---------- `second_foo` moved due to this method call
LL |     second_foo; //~ ERROR use of moved
   |     ^^^^^^^^^^ value used here after move

error[E0382]: use of moved value: `boxed_foo`
   |
   |
LL |     let boxed_foo = Box::new(Foo);
   |         --------- move occurs because `boxed_foo` has type `Box<Foo>`, which does not implement the `Copy` trait
LL |     boxed_foo.use_box_self();
   |               -------------- `boxed_foo` moved due to this method call
LL |     boxed_foo; //~ ERROR use of moved
   |     ^^^^^^^^^ value used here after move
   |
note: this function takes ownership of the receiver `self`, which moves `boxed_foo`
   |
   |
LL |     fn use_box_self(self: Box<Self>) {}


error[E0382]: use of moved value: `pin_box_foo`
   |
   |
LL |     let pin_box_foo = Box::pin(Foo);
   |         ----------- move occurs because `pin_box_foo` has type `Pin<Box<Foo>>`, which does not implement the `Copy` trait
LL |     pin_box_foo.use_pin_box_self();
   |                 ------------------ `pin_box_foo` moved due to this method call
LL |     pin_box_foo; //~ ERROR use of moved
   |     ^^^^^^^^^^^ value used here after move
   |
note: this function takes ownership of the receiver `self`, which moves `pin_box_foo`
   |
   |
LL |     fn use_pin_box_self(self: Pin<Box<Self>>) {}


error[E0505]: cannot move out of `mut_foo` because it is borrowed
   |
   |
LL |     let ret = mut_foo.use_mut_self();
   |               ---------------------- borrow of `mut_foo` occurs here
LL |     mut_foo; //~ ERROR cannot move out
   |     ^^^^^^^ move out of `mut_foo` occurs here
LL |     ret;
   |     --- borrow later used here

error[E0382]: use of moved value: `rc_foo`
   |
   |
LL |     let rc_foo = Rc::new(Foo);
   |         ------ move occurs because `rc_foo` has type `Rc<Foo>`, which does not implement the `Copy` trait
LL |     rc_foo.use_rc_self();
   |            ------------- `rc_foo` moved due to this method call
LL |     rc_foo; //~ ERROR use of moved
   |     ^^^^^^ value used here after move
   |
note: this function takes ownership of the receiver `self`, which moves `rc_foo`
   |
   |
LL |     fn use_rc_self(self: Rc<Self>) {}


error[E0382]: use of moved value: `foo_add`
   |
LL |     let foo_add = Foo;
LL |     let foo_add = Foo;
   |         ------- move occurs because `foo_add` has type `Foo`, which does not implement the `Copy` trait
LL |     foo_add + Foo;
   |     ------------- `foo_add` moved due to usage in operator
LL |     foo_add; //~ ERROR use of moved
   |     ^^^^^^^ value used here after move
note: calling this operator moves the left-hand side
  --> /checkout/library/core/src/ops/arith.rs:89:12
   |
   |
LL |     fn add(self, rhs: Rhs) -> Self::Output;

error[E0382]: use of moved value: `implicit_into_iter`
  --> /checkout/src/test/ui/moves/move-fn-self-receiver.rs:63:5
   |
   |
LL |     let implicit_into_iter = vec![true];
   |         ------------------ move occurs because `implicit_into_iter` has type `Vec<bool>`, which does not implement the `Copy` trait
LL |     for _val in implicit_into_iter {}
   |                 ------------------ `implicit_into_iter` moved due to this implicit call to `.into_iter()`
LL |     implicit_into_iter; //~ ERROR use of moved
   |     ^^^^^^^^^^^^^^^^^^ value used here after move

error[E0382]: use of moved value: `explicit_into_iter`
   |
   |
LL |     let explicit_into_iter = vec![true];
   |         ------------------ move occurs because `explicit_into_iter` has type `Vec<bool>`, which does not implement the `Copy` trait
LL |     for _val in explicit_into_iter.into_iter() {}
   |                                    ----------- `explicit_into_iter` moved due to this method call
LL |     explicit_into_iter; //~ ERROR use of moved
   |     ^^^^^^^^^^^^^^^^^^ value used here after move
error[E0382]: use of moved value: `container`
  --> /checkout/src/test/ui/moves/move-fn-self-receiver.rs:71:5
   |
   |
LL |     let container = Container(vec![]);
   |         --------- move occurs because `container` has type `Container`, which does not implement the `Copy` trait
LL |     for _val in container.custom_into_iter() {}
   |                           ------------------ `container` moved due to this method call
LL |     container; //~ ERROR use of moved
   |     ^^^^^^^^^ value used here after move
   |
note: this function takes ownership of the receiver `self`, which moves `container`
   |
   |
LL |     fn custom_into_iter(self) -> impl Iterator<Item = bool> {

error[E0382]: use of moved value: `foo2`
  --> /checkout/src/test/ui/moves/move-fn-self-receiver.rs:75:9
   |
   |
LL |     let foo2 = Foo;
   |         ---- move occurs because `foo2` has type `Foo`, which does not implement the `Copy` trait
LL |     loop {
LL |         foo2.use_self(); //~ ERROR use of moved
   |         ^^^^ ---------- `foo2` moved due to this method call, in previous iteration of loop
error: aborting due to 12 previous errors

Some errors have detailed explanations: E0382, E0505.
For more information about an error, try `rustc --explain E0382`.
For more information about an error, try `rustc --explain E0382`.

------------------------------------------


---- [ui] ui/suggestions/borrow-for-loop-head.rs stdout ----
diff of stderr:

13    |         - move occurs because `a` has type `Vec<i32>`, which does not implement the `Copy` trait
14 LL |     for i in &a {
15 LL |         for j in a {
-    |                  |
-    |                  |
-    |                  `a` moved due to this implicit call to `.into_iter()`, in previous iteration of loop
-    |                  help: consider borrowing to avoid moving into the for loop: `&a`
+    |                  ^ `a` moved due to this implicit call to `.into_iter()`, in previous iteration of loop
20    |
21 note: this function takes ownership of the receiver `self`, which moves `a`
22   --> $SRC_DIR/core/src/iter/traits/collect.rs:LL:COL

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/borrow-for-loop-head/borrow-for-loop-head.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/borrow-for-loop-head.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/borrow-for-loop-head.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/borrow-for-loop-head" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/borrow-for-loop-head/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0505]: cannot move out of `a` because it is borrowed
   |
LL |     for i in &a {
LL |     for i in &a {
   |              -- borrow of `a` occurs here
LL |         for j in a {
   |                  ^ move out of `a` occurs here
error[E0382]: use of moved value: `a`
  --> /checkout/src/test/ui/suggestions/borrow-for-loop-head.rs:4:18
   |
   |
LL |     let a = vec![1, 2, 3];
   |         - move occurs because `a` has type `Vec<i32>`, which does not implement the `Copy` trait
LL |     for i in &a {
LL |         for j in a {
   |                  ^ `a` moved due to this implicit call to `.into_iter()`, in previous iteration of loop
   |
note: this function takes ownership of the receiver `self`, which moves `a`
   |
LL |     fn into_iter(self) -> Self::IntoIter;
   |                  ^^^^

---
---- [ui] ui/suggestions/for-i-in-vec.rs stdout ----
diff of stderr:

3    |
4 LL |         for _ in self.v {
5    |                  ^^^^^^ move occurs because `self.v` has type `Vec<u32>`, which does not implement the `Copy` trait
-    |
- help: consider iterating over a slice of the `Vec<u32>`'s content
-    |
- LL |         for _ in &self.v {
11 
11 
12 error[E0507]: cannot move out of `self.h` which is behind a shared reference

14    |
14    |
15 LL |         for _ in self.h {
16    |                  ^^^^^^ move occurs because `self.h` has type `HashMap<i32, i32>`, which does not implement the `Copy` trait
-    |
- help: consider iterating over a slice of the `HashMap<i32, i32>`'s content
-    |
- LL |         for _ in &self.h {
22 
23 error[E0507]: cannot move out of a shared reference
24   --> $DIR/for-i-in-vec.rs:21:19


25    |
26 LL |     for loader in *LOADERS {
27    |                   ^^^^^^^^ move occurs because value has type `Vec<&u8>`, which does not implement the `Copy` trait
-    |
- help: consider iterating over a slice of the `Vec<&u8>`'s content
-    |
- LL |     for loader in &*LOADERS {
33 
34 error: aborting due to 3 previous errors
35 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/for-i-in-vec/for-i-in-vec.stderr
diff of fixed:

8 
9 impl Foo {
10     fn bar(&self) {
-         for _ in &self.v { //~ ERROR cannot move out of `self.v` which is behind a shared reference
+         for _ in self.v { //~ ERROR cannot move out of `self.v` which is behind a shared reference
12         }
-         for _ in &self.h { //~ ERROR cannot move out of `self.h` which is behind a shared reference
+         for _ in self.h { //~ ERROR cannot move out of `self.h` which is behind a shared reference
15     }
16 }


18 const LOADERS: &Vec<&'static u8> = &Vec::new();
19 
20 pub fn break_code() -> Option<&'static u8> {
-     for loader in &*LOADERS { //~ ERROR cannot move out of a shared reference
+     for loader in *LOADERS { //~ ERROR cannot move out of a shared reference
22         return Some(loader);
24     None


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/for-i-in-vec/for-i-in-vec.fixed
To only update this specific test, also pass `--test-args suggestions/for-i-in-vec.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/for-i-in-vec.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/for-i-in-vec" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/for-i-in-vec/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0507]: cannot move out of `self.v` which is behind a shared reference
   |
   |
LL |         for _ in self.v { //~ ERROR cannot move out of `self.v` which is behind a shared reference
   |                  ^^^^^^ move occurs because `self.v` has type `Vec<u32>`, which does not implement the `Copy` trait

error[E0507]: cannot move out of `self.h` which is behind a shared reference
   |
   |
LL |         for _ in self.h { //~ ERROR cannot move out of `self.h` which is behind a shared reference
   |                  ^^^^^^ move occurs because `self.h` has type `HashMap<i32, i32>`, which does not implement the `Copy` trait
error[E0507]: cannot move out of a shared reference
  --> /checkout/src/test/ui/suggestions/for-i-in-vec.rs:21:19
   |
   |
LL |     for loader in *LOADERS { //~ ERROR cannot move out of a shared reference
   |                   ^^^^^^^^ move occurs because value has type `Vec<&u8>`, which does not implement the `Copy` trait
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0507`.

---
test result: FAILED. 12167 passed; 8 failed; 117 ignored; 0 measured; 0 filtered out; finished in 132.46s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:55
