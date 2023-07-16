plain

---- [ui] ui/issues/issue-31173.rs stdout ----
diff of stderr:

13    |                                ^^^^^^^^^^^^ required by this bound in `cloned`
14 
15 error[E0599]: the method `collect` exists for struct `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@$DIR/issue-31173.rs:6:39: 9:6]>>`, but its trait bounds were not satisfied
-   --> $DIR/issue-31173.rs:12:10
17    |
- LL |         .collect();
-    |          ^^^^^^^ method cannot be called on `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@$DIR/issue-31173.rs:6:39: 9:6]>>` due to unsatisfied trait bounds
+   ::: $SRC_DIR/core/src/iter/adapters/cloned.rs:LL:COL
20    |
+ LL | pub struct Cloned<I> {
+    | -------------------- doesn't satisfy `_: Iterator`
21   ::: $SRC_DIR/core/src/iter/adapters/take_while.rs:LL:COL
22    |
22    |
23 LL | pub struct TakeWhile<I, P> {

24    | -------------------------- doesn't satisfy `<_ as Iterator>::Item = &_`
25    |
-   ::: $SRC_DIR/core/src/iter/adapters/cloned.rs:LL:COL
-    |
-    |
- LL | pub struct Cloned<I> {
-    | -------------------- doesn't satisfy `_: Iterator`
+ LL |         .collect();
+    |          ^^^^^^^ method cannot be called on `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@$DIR/issue-31173.rs:6:39: 9:6]>>` due to unsatisfied trait bounds
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i586-unknown-linux-gnu
31    = note: the following trait bounds were not satisfied:
31    = note: the following trait bounds were not satisfied:
32            `<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@$DIR/issue-31173.rs:6:39: 9:6]> as Iterator>::Item = &_`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31173/issue-31173.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-31173.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-31173.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31173" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31173/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0271]: type mismatch resolving `<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6]> as Iterator>::Item == &_`
   |
LL |         .cloned()
   |          ^^^^^^ expected reference, found `u8`
   |
   |
   = note: expected reference `&_`
                   found type `u8`
note: required by a bound in `cloned`
  --> /checkout/library/core/src/iter/traits/iterator.rs:3140:32
   |
LL |         Self: Sized + Iterator<Item = &'a T>,
   |                                ^^^^^^^^^^^^ required by this bound in `cloned`

error[E0599]: the method `collect` exists for struct `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6]>>`, but its trait bounds were not satisfied
  ::: /checkout/library/core/src/iter/adapters/cloned.rs:17:1
   |
   |
LL | pub struct Cloned<I> {
   | -------------------- doesn't satisfy `_: Iterator`
  ::: /checkout/library/core/src/iter/adapters/take_while.rs:15:1
   |
   |
LL | pub struct TakeWhile<I, P> {
   | -------------------------- doesn't satisfy `<_ as Iterator>::Item = &_`
   |
   |
LL |         .collect(); //~ ERROR the method
   |          ^^^^^^^ method cannot be called on `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6]>>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6]> as Iterator>::Item = &_`
           which is required by `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6]>>: Iterator`
           `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6]>>: Iterator`
           which is required by `&mut Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6]>>: Iterator`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0271, E0599.
For more information about an error, try `rustc --explain E0271`.
