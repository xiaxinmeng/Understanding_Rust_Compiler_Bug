plain

---- [ui (nll)] ui/generic-associated-types/extended/lending_iterator.rs#base stdout ----
diff of stderr:

7 LL |     fn from_iter<I: for<'x> LendingIterator<Item<'x> = A>>(mut iter: I) -> Self {
8    |                                             ^^^^^^^^^^^^ impl has extra requirement `I: 'x`
- error[E0311]: the parameter type `Self` may not live long enough
-   --> $DIR/lending_iterator.rs:35:9
-    |
-    |
- LL |         <B as FromLendingIterator<A>>::from_iter(self)
-    |
-    |
-    = help: consider adding an explicit lifetime bound `Self: 'a`...
-    = note: ...so that the type `Self` will meet its required lifetime bounds...
- note: ...that is required by this bound
-   --> $DIR/lending_iterator.rs:10:45
-    |
- LL |     fn from_iter<T: for<'x> LendingIterator<Item<'x> = A>>(iter: T) -> Self;
- 
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
25 
25 
26 For more information about this error, try `rustc --explain E0276`.
27 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/extended/lending_iterator.base.nll/lending_iterator.base.nll.stderr
To only update this specific test, also pass `--test-args generic-associated-types/extended/lending_iterator.rs`


error in revision `base`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/extended/lending_iterator.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/extended/lending_iterator.base.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/extended/lending_iterator.base.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0276]: impl has stricter requirements than trait
  --> /checkout/src/test/ui/generic-associated-types/extended/lending_iterator.rs:14:45
   |
LL |     fn from_iter<T: for<'x> LendingIterator<Item<'x> = A>>(iter: T) -> Self;
   |     ------------------------------------------------------------------------ definition of `from_iter` from trait
...
LL |     fn from_iter<I: for<'x> LendingIterator<Item<'x> = A>>(mut iter: I) -> Self {
   |                                             ^^^^^^^^^^^^ impl has extra requirement `I: 'x`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0276`.
------------------------------------------
---
    [ui (nll)] ui/generic-associated-types/extended/lending_iterator.rs#base

test result: FAILED. 12687 passed; 1 failed; 150 ignored; 0 measured; 0 filtered out; finished in 108.53s

Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
