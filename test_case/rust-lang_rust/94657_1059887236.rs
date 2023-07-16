plain
.................................................................................................... 10600/12700
.................................................................................................... 10700/12700
.................................................................................................... 10800/12700
.................................................................................................... 10900/12700
...........F.F...................................................................................... 11000/12700
.................................................................................................... 11200/12700
.................................................................................................... 11300/12700
..........i.....i............................i...................................................... 11400/12700
.................................................................................................... 11500/12700
---

23 note: required by a bound in `core::str::<impl str>::get`
24   --> $SRC_DIR/core/src/str/mod.rs:LL:COL
25    |
- LL |     pub fn get<I: SliceIndex<str>>(&self, i: I) -> Option<&I::Output> {
-    |                   ^^^^^^^^^^^^^^^ required by this bound in `core::str::<impl str>::get`
+ LL |     pub const fn get<I: ~const SliceIndex<str>>(&self, i: I) -> Option<&I::Output> {
+    |                         ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `core::str::<impl str>::get`
29 error[E0277]: the type `str` cannot be indexed by `{integer}`
30   --> $DIR/str-idx.rs:5:29
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


40 note: required by a bound in `core::str::<impl str>::get_unchecked`
41   --> $SRC_DIR/core/src/str/mod.rs:LL:COL
42    |
- LL |     pub unsafe fn get_unchecked<I: SliceIndex<str>>(&self, i: I) -> &I::Output {
-    |                                    ^^^^^^^^^^^^^^^ required by this bound in `core::str::<impl str>::get_unchecked`
+ LL |     pub const unsafe fn get_unchecked<I: ~const SliceIndex<str>>(&self, i: I) -> &I::Output {
+    |                                          ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `core::str::<impl str>::get_unchecked`
45 
46 error[E0277]: the type `str` cannot be indexed by `char`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-idx/str-idx.stderr
To only update this specific test, also pass `--test-args str/str-idx.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/str/str-idx.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-idx" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-idx/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the type `str` cannot be indexed by `{integer}`
   |
   |
LL |     let _: u8 = s[4]; //~ ERROR the type `str` cannot be indexed by `{integer}`
   |                 ^^^^ string indices are ranges of `usize`
   |
   = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
   = note: you can use `.chars().nth()` or `.bytes().nth()`
           for more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
   = note: required because of the requirements on the impl of `Index<{integer}>` for `str`
error[E0277]: the type `str` cannot be indexed by `{integer}`
  --> /checkout/src/test/ui/str/str-idx.rs:4:19
   |
   |
LL |     let _ = s.get(4); //~ ERROR the type `str` cannot be indexed by `{integer}`
   |               --- ^ string indices are ranges of `usize`
   |               required by a bound introduced by this call
   |
   |
   = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
   = note: you can use `.chars().nth()` or `.bytes().nth()`
           for more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
note: required by a bound in `core::str::<impl str>::get`
   |
   |
LL |     pub const fn get<I: ~const SliceIndex<str>>(&self, i: I) -> Option<&I::Output> {
   |                         ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `core::str::<impl str>::get`
error[E0277]: the type `str` cannot be indexed by `{integer}`
  --> /checkout/src/test/ui/str/str-idx.rs:5:29
   |
   |
LL |     let _ = s.get_unchecked(4); //~ ERROR the type `str` cannot be indexed by `{integer}`
   |               ------------- ^ string indices are ranges of `usize`
   |               required by a bound introduced by this call
   |
   |
   = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
   = note: you can use `.chars().nth()` or `.bytes().nth()`
           for more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
note: required by a bound in `core::str::<impl str>::get_unchecked`
   |
   |
LL |     pub const unsafe fn get_unchecked<I: ~const SliceIndex<str>>(&self, i: I) -> &I::Output {
   |                                          ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `core::str::<impl str>::get_unchecked`

error[E0277]: the type `str` cannot be indexed by `char`
   |
   |
LL |     let _: u8 = s['c']; //~ ERROR the type `str` cannot be indexed by `char`
   |                 ^^^^^^ string indices are ranges of `usize`
   = help: the trait `SliceIndex<str>` is not implemented for `char`
   = help: the trait `SliceIndex<str>` is not implemented for `char`
   = note: required because of the requirements on the impl of `Index<char>` for `str`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
---

47 note: required by a bound in `core::str::<impl str>::get_mut`
48   --> $SRC_DIR/core/src/str/mod.rs:LL:COL
49    |
- LL |     pub fn get_mut<I: SliceIndex<str>>(&mut self, i: I) -> Option<&mut I::Output> {
-    |                       ^^^^^^^^^^^^^^^ required by this bound in `core::str::<impl str>::get_mut`
+ LL |     pub const fn get_mut<I: ~const SliceIndex<str>>(&mut self, i: I) -> Option<&mut I::Output> {
+    |                             ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `core::str::<impl str>::get_mut`
53 error[E0277]: the type `str` cannot be indexed by `{integer}`
54   --> $DIR/str-mut-idx.rs:11:25

64 note: required by a bound in `core::str::<impl str>::get_unchecked_mut`
64 note: required by a bound in `core::str::<impl str>::get_unchecked_mut`
65   --> $SRC_DIR/core/src/str/mod.rs:LL:COL
66    |
- LL |     pub unsafe fn get_unchecked_mut<I: SliceIndex<str>>(&mut self, i: I) -> &mut I::Output {
-    |                                        ^^^^^^^^^^^^^^^ required by this bound in `core::str::<impl str>::get_unchecked_mut`
+ LL |     pub const unsafe fn get_unchecked_mut<I: ~const SliceIndex<str>>(
+    |                                              ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `core::str::<impl str>::get_unchecked_mut`
69 
70 error[E0277]: the type `str` cannot be indexed by `char`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-mut-idx/str-mut-idx.stderr
To only update this specific test, also pass `--test-args str/str-mut-idx.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/str/str-mut-idx.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-mut-idx" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-mut-idx/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the size for values of type `str` cannot be known at compilation time
   |
   |
LL |     s[1..2] = bot();
   |
   = help: the trait `Sized` is not implemented for `str`
note: required by a bound in `bot`
  --> /checkout/src/test/ui/str/str-mut-idx.rs:1:8
  --> /checkout/src/test/ui/str/str-mut-idx.rs:1:8
   |
LL | fn bot<T>() -> T { loop {} }
   |        ^ required by this bound in `bot`
help: consider relaxing the implicit `Sized` restriction
   |
LL | fn bot<T: ?Sized>() -> T { loop {} }

error[E0277]: the size for values of type `str` cannot be known at compilation time
  --> /checkout/src/test/ui/str/str-mut-idx.rs:4:5
   |
   |
LL |     s[1..2] = bot();
   |
   = help: the trait `Sized` is not implemented for `str`
   = note: the left-hand-side of an assignment must have a statically known size


error[E0277]: the type `str` cannot be indexed by `usize`
   |
   |
LL |     s[1usize] = bot();
   |     ^^^^^^^^^ string indices are ranges of `usize`
   = help: the trait `SliceIndex<str>` is not implemented for `usize`
   = help: the trait `SliceIndex<str>` is not implemented for `usize`
   = note: required because of the requirements on the impl of `Index<usize>` for `str`
error[E0277]: the type `str` cannot be indexed by `{integer}`
  --> /checkout/src/test/ui/str/str-mut-idx.rs:9:15
   |
LL |     s.get_mut(1);
LL |     s.get_mut(1);
   |       ------- ^ string indices are ranges of `usize`
   |       required by a bound introduced by this call
   |
   |
   = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
   = note: you can use `.chars().nth()` or `.bytes().nth()`
           for more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
note: required by a bound in `core::str::<impl str>::get_mut`
   |
   |
LL |     pub const fn get_mut<I: ~const SliceIndex<str>>(&mut self, i: I) -> Option<&mut I::Output> {
   |                             ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `core::str::<impl str>::get_mut`
error[E0277]: the type `str` cannot be indexed by `{integer}`
  --> /checkout/src/test/ui/str/str-mut-idx.rs:11:25
   |
LL |     s.get_unchecked_mut(1);
LL |     s.get_unchecked_mut(1);
   |       ----------------- ^ string indices are ranges of `usize`
   |       required by a bound introduced by this call
   |
   |
   = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
   = note: you can use `.chars().nth()` or `.bytes().nth()`
           for more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
note: required by a bound in `core::str::<impl str>::get_unchecked_mut`
   |
   |
LL |     pub const unsafe fn get_unchecked_mut<I: ~const SliceIndex<str>>(
   |                                              ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `core::str::<impl str>::get_unchecked_mut`

error[E0277]: the type `str` cannot be indexed by `char`
   |
   |
LL |     s['c'];
   |     ^^^^^^ string indices are ranges of `usize`
   = help: the trait `SliceIndex<str>` is not implemented for `char`
   = help: the trait `SliceIndex<str>` is not implemented for `char`
   = note: required because of the requirements on the impl of `Index<char>` for `str`
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
