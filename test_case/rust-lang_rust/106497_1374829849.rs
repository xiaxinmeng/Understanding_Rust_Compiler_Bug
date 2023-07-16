plain
...............iii...................................................................... 14080/14117
.....................................
failures:

---- [ui] src/test/ui/suggestions/issue-106443-sugg-clone-for-bound.rs stdout ----

6    |
7 help: consider further restricting this bound
8    |
8    |
- LL | fn bar<T: X + Clone>(s: &T)  {
+ LL | fn bar<T: X + Clone>(s: &T) {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
11 help: consider using clone here
12    |

---
To only update this specific test, also pass `--test-args suggestions/issue-106443-sugg-clone-for-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-106443-sugg-clone-for-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-106443-sugg-clone-for-bound" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-106443-sugg-clone-for-bound/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `&T: X` is not satisfied
  --> /checkout/src/test/ui/suggestions/issue-106443-sugg-clone-for-bound.rs:10:9
   |
LL |     foo(s); //~ ERROR the trait bound `&T: X` is not satisfied
   |         ^ the trait `X` is not implemented for `&T`
help: consider further restricting this bound
   |
   |
LL | fn bar<T: X + Clone>(s: &T) {
help: consider using clone here
   |
   |
LL |     foo(s.clone()); //~ ERROR the trait bound `&T: X` is not satisfied


error[E0277]: the trait bound `&T: X` is not satisfied
  --> /checkout/src/test/ui/suggestions/issue-106443-sugg-clone-for-bound.rs:14:9
   |
LL |     foo(s); //~ ERROR the trait bound `&T: X` is not satisfied
   |         ^ the trait `X` is not implemented for `&T`
help: consider using clone here
   |
   |
LL |     foo(s.clone()); //~ ERROR the trait bound `&T: X` is not satisfied

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
