plain
........................................................................................  5016/15026
........................................................................................  5104/15026
........................................................................................  5192/15026
..............................................................i.........................  5280/15026
......................................................................F.........F.......  5368/15026
........................................................................................  5544/15026
........................................................................................  5632/15026
........................................................................................  5720/15026
........................................................................................  5808/15026
---

---- [ui] tests/ui/higher-rank-trait-bounds/issue-30786.rs stdout ----
diff of stderr:

10 LL |     let filter = map.filterx(|x: &_| true);
11    |                      ^^^^^^^ method cannot be called on `Map<Repeat, [closure@issue-30786.rs:119:27]>` due to unsatisfied trait bounds
12    |
-    = note: the full type name has been written to '$TEST_BUILD_DIR/higher-rank-trait-bounds/issue-30786/issue-30786.long-type-hash.txt'
14 note: the following trait bounds were not satisfied:
15       `&'a mut &Map<Repeat, [closure@$DIR/issue-30786.rs:119:27: 119:34]>: Stream`
16       `&'a mut &mut Map<Repeat, [closure@$DIR/issue-30786.rs:119:27: 119:34]>: Stream`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-30786/issue-30786.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args higher-rank-trait-bounds/issue-30786.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/higher-rank-trait-bounds/issue-30786.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-30786" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-30786/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `filterx` exists for struct `Map<Repeat, [closure@issue-30786.rs:119:27]>`, but its trait bounds were not satisfied
  --> fake-test-src-base/higher-rank-trait-bounds/issue-30786.rs:120:22
   |
LL | pub struct Map<S, F> {
   | |
   | |
   | method `filterx` not found for this struct
   | doesn't satisfy `_: StreamExt`
...
LL |     let filter = map.filterx(|x: &_| true);
   |                      ^^^^^^^ method cannot be called on `Map<Repeat, [closure@issue-30786.rs:119:27]>` due to unsatisfied trait bounds
note: the following trait bounds were not satisfied:
note: the following trait bounds were not satisfied:
      `&'a mut &Map<Repeat, [closure@fake-test-src-base/higher-rank-trait-bounds/issue-30786.rs:119:27: 119:34]>: Stream`
      `&'a mut &mut Map<Repeat, [closure@fake-test-src-base/higher-rank-trait-bounds/issue-30786.rs:119:27: 119:34]>: Stream`
      `&'a mut Map<Repeat, [closure@fake-test-src-base/higher-rank-trait-bounds/issue-30786.rs:119:27: 119:34]>: Stream`
  --> fake-test-src-base/higher-rank-trait-bounds/issue-30786.rs:98:50
   |
LL | impl<T> StreamExt for T where for<'a> &'a mut T: Stream {}
   |         ---------     -                          ^^^^^^ unsatisfied trait bound introduced here

error[E0599]: the method `countx` exists for struct `Filter<Map<Repeat, fn(&u64) -> &u64 {identity::<u64>}>, [closure@issue-30786.rs:131:30]>`, but its trait bounds were not satisfied
  --> fake-test-src-base/higher-rank-trait-bounds/issue-30786.rs:132:24
   |
LL | pub struct Filter<S, F> {
   | |
   | |
   | method `countx` not found for this struct
   | doesn't satisfy `_: StreamExt`
...
LL |     let count = filter.countx();
   |
   |
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-30786/issue-30786.long-type-2907370422938660774.txt'
note: the following trait bounds were not satisfied:
      `&'a mut &Filter<Map<Repeat, for<'a> fn(&'a u64) -> &'a u64 {identity::<u64>}>, [closure@fake-test-src-base/higher-rank-trait-bounds/issue-30786.rs:131:30: 131:37]>: Stream`
      `&'a mut &mut Filter<Map<Repeat, for<'a> fn(&'a u64) -> &'a u64 {identity::<u64>}>, [closure@fake-test-src-base/higher-rank-trait-bounds/issue-30786.rs:131:30: 131:37]>: Stream`
      `&'a mut Filter<Map<Repeat, for<'a> fn(&'a u64) -> &'a u64 {identity::<u64>}>, [closure@fake-test-src-base/higher-rank-trait-bounds/issue-30786.rs:131:30: 131:37]>: Stream`
  --> fake-test-src-base/higher-rank-trait-bounds/issue-30786.rs:98:50
   |
LL | impl<T> StreamExt for T where for<'a> &'a mut T: Stream {}
   |         ---------     -                          ^^^^^^ unsatisfied trait bound introduced here
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
---

44 LL | |         },
45    | |_________^ expected `Unit3`, found `Unit4`
46    |
- note: required for `L<[closure@issue-62203-hrtb-ice.rs:42:16]>` to implement `for<'r> T0<'r, (&'r u8,)>`
+ note: required for `L<[closure@$DIR/issue-62203-hrtb-ice.rs:42:16: 42:19]>` to implement `for<'r> T0<'r, (&'r u8,)>`
49    |
49    |
50 LL | impl<'a, A, T> T0<'a, A> for L<T>
52 LL | where
52 LL | where
53 LL |     T: FnMut(A) -> Unit3,
54    |                    ----- unsatisfied trait bound introduced here
-    = note: the full type name has been written to '$TEST_BUILD_DIR/higher-rank-trait-bounds/issue-62203-hrtb-ice/issue-62203-hrtb-ice.long-type-4781964035802402551.txt'
56 note: required by a bound in `T1::m`
58    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-62203-hrtb-ice/issue-62203-hrtb-ice.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args higher-rank-trait-bounds/issue-62203-hrtb-ice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/higher-rank-trait-bounds/issue-62203-hrtb-ice.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-62203-hrtb-ice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-62203-hrtb-ice/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0271]: type mismatch resolving `<L<[closure@issue-62203-hrtb-ice.rs:42:16]> as T0<'r, (&u8,)>>::O == <_ as Ty<'r>>::V`
  --> fake-test-src-base/higher-rank-trait-bounds/issue-62203-hrtb-ice.rs:39:9
   |
LL |       let v = Unit2.m(
LL | /         L {
LL | /         L {
LL | |             //~^ ERROR to be a closure that returns `Unit3`, but it returns `Unit4`
LL | |             //~| ERROR type mismatch
LL | |             f: |x| {
LL | |             },
LL | |         },
LL | |         },
   | |_________^ type mismatch resolving `<L<[closure@issue-62203-hrtb-ice.rs:42:16]> as T0<'r, (&u8,)>>::O == <_ as Ty<'r>>::V`
   |
note: expected this to be `<_ as Ty<'_>>::V`
  --> fake-test-src-base/higher-rank-trait-bounds/issue-62203-hrtb-ice.rs:21:14
   |
LL |     type O = T::Output;
   |              ^^^^^^^^^
   = note: expected associated type `<_ as Ty<'_>>::V`
                       found struct `Unit4`
   = help: consider constraining the associated type `<_ as Ty<'_>>::V` to `Unit4` or calling a method that returns `<_ as Ty<'_>>::V`
note: required by a bound in `T1::m`
  --> fake-test-src-base/higher-rank-trait-bounds/issue-62203-hrtb-ice.rs:27:51
   |
   |
LL |     fn m<'a, B: Ty<'a>, F>(&self, f: F) -> Unit1
LL |     where
LL |     where
LL |         F: for<'r> T0<'r, (<Self as Ty<'r>>::V,), O = <B as Ty<'r>>::V>,
   |                                                   ^^^^^^^^^^^^^^^^^^^^ required by this bound in `T1::m`

error[E0271]: expected `[closure@issue-62203-hrtb-ice.rs:42:16]` to be a closure that returns `Unit3`, but it returns `Unit4`
  --> fake-test-src-base/higher-rank-trait-bounds/issue-62203-hrtb-ice.rs:39:9
   |
LL |       let v = Unit2.m(
LL | /         L {
LL | /         L {
LL | |             //~^ ERROR to be a closure that returns `Unit3`, but it returns `Unit4`
LL | |             //~| ERROR type mismatch
LL | |             f: |x| {
LL | |             },
LL | |         },
   | |_________^ expected `Unit3`, found `Unit4`
   |
   |
note: required for `L<[closure@fake-test-src-base/higher-rank-trait-bounds/issue-62203-hrtb-ice.rs:42:16: 42:19]>` to implement `for<'r> T0<'r, (&'r u8,)>`
  --> fake-test-src-base/higher-rank-trait-bounds/issue-62203-hrtb-ice.rs:17:16
   |
LL | impl<'a, A, T> T0<'a, A> for L<T>
LL | where
LL | where
LL |     T: FnMut(A) -> Unit3,
   |                    ----- unsatisfied trait bound introduced here
note: required by a bound in `T1::m`
  --> fake-test-src-base/higher-rank-trait-bounds/issue-62203-hrtb-ice.rs:27:12
   |
LL |     fn m<'a, B: Ty<'a>, F>(&self, f: F) -> Unit1
LL |     where
LL |     where
LL |         F: for<'r> T0<'r, (<Self as Ty<'r>>::V,), O = <B as Ty<'r>>::V>,
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `T1::m`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0271`.
------------------------------------------
---

27    |
28    = note: doesn't satisfy `_: Iterator`
29    |
-    = note: the full type name has been written to '$TEST_BUILD_DIR/mismatched_types/issue-36053-2/issue-36053-2.long-type-5257602210078210066.txt'
31    = note: the following trait bounds were not satisfied:
32            `<[closure@$DIR/issue-36053-2.rs:7:39: 7:48] as FnOnce<(&&str,)>>::Output = bool`
33            which is required by `Filter<Fuse<std::iter::Once<&str>>, [closure@$DIR/issue-36053-2.rs:7:39: 7:48]>: Iterator`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2/issue-36053-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/issue-36053-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/mismatched_types/issue-36053-2.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2/auxiliary"
stdout: none
error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/mismatched_types/issue-36053-2.rs:7:32
   |
   |
LL |     once::<&str>("str").fuse().filter(|a: &str| true).count();
   |                                |
   |                                expected due to this
   |
   |
   = note: expected closure signature `for<'a> fn(&'a &str) -> _`
              found closure signature `for<'a> fn(&'a str) -> _`
note: required by a bound in `filter`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/traits/iterator.rs:922:5
   |
   |
LL |     once::<&str>("str").fuse().filter(|a: &&str| true).count();


error[E0599]: the method `count` exists for struct `Filter<Fuse<Once<&str>>, [closure@issue-36053-2.rs:7:39]>`, but its trait bounds were not satisfied
  --> fake-test-src-base/mismatched_types/issue-36053-2.rs:7:55
   |
LL |     once::<&str>("str").fuse().filter(|a: &str| true).count();
   |                                       |
   |                                       |
   |                                       doesn't satisfy `<_ as FnOnce<(&&str,)>>::Output = bool`
   |                                       doesn't satisfy `_: FnMut<(&&str,)>`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/adapters/filter.rs:15:1
   = note: doesn't satisfy `_: Iterator`
   |
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `<[closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48] as FnOnce<(&&str,)>>::Output = bool`
           which is required by `Filter<Fuse<std::iter::Once<&str>>, [closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48]>: Iterator`
           `[closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48]: FnMut<(&&str,)>`
           which is required by `Filter<Fuse<std::iter::Once<&str>>, [closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48]>: Iterator`
           `Filter<Fuse<std::iter::Once<&str>>, [closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48]>: Iterator`
           which is required by `&mut Filter<Fuse<std::iter::Once<&str>>, [closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48]>: Iterator`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0599, E0631.
For more information about an error, try `rustc --explain E0599`.
For more information about an error, try `rustc --explain E0599`.
------------------------------------------


---- [ui] tests/ui/recursion/issue-83150.rs stdout ----
diff of stderr:

12 error[E0275]: overflow evaluating the requirement `Map<&mut std::ops::Range<u8>, [closure@$DIR/issue-83150.rs:12:24: 12:27]>: Iterator`
13    |
14    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_83150`)
-    = note: required for `&mut Map<&mut Range<u8>, [closure@issue-83150.rs:12:24]>` to implement `Iterator`
-    = note: the full type name has been written to '$TEST_BUILD_DIR/recursion/issue-83150/issue-83150.long-type-hash.txt'
+    = note: required for `&mut Map<&mut std::ops::Range<u8>, [closure@$DIR/issue-83150.rs:12:24: 12:27]>` to implement `Iterator`
17    = note: 65 redundant requirements hidden
18    = note: required for `&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<..., ...>, ...>, ...>, ...>, ...>, ...>, ...>` to implement `Iterator`
19    = note: the full type name has been written to '$TEST_BUILD_DIR/recursion/issue-83150/issue-83150.long-type-hash.txt'

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-83150/issue-83150.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args recursion/issue-83150.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/recursion/issue-83150.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-83150" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-83150/auxiliary" "-Copt-level=0"
stdout: none
warning: function cannot return without recursing
  --> fake-test-src-base/recursion/issue-83150.rs:11:1
   |
   |
LL | fn func<T: Iterator<Item = u8>>(iter: &mut T) { //~ WARN function cannot return without recursing
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
LL |     func(&mut iter.map(|x| x + 1))
   |     ------------------------------ recursive call site
   |
   = help: a `loop` may express intention better if this is on purpose
   = note: `#[warn(unconditional_recursion)]` on by default

error[E0275]: overflow evaluating the requirement `Map<&mut std::ops::Range<u8>, [closure@fake-test-src-base/recursion/issue-83150.rs:12:24: 12:27]>: Iterator`
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_83150`)
   = note: required for `&mut Map<&mut std::ops::Range<u8>, [closure@fake-test-src-base/recursion/issue-83150.rs:12:24: 12:27]>` to implement `Iterator`
   = note: 65 redundant requirements hidden
   = note: required for `&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<&mut Map<..., ...>, ...>, ...>, ...>, ...>, ...>, ...>` to implement `Iterator`
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-83150/issue-83150.long-type-8093535002315500471.txt'
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0275`.
------------------------------------------
---

42    |
43    = note: doesn't satisfy `_: Iterator`
44    |
-    = note: the full type name has been written to '$TEST_BUILD_DIR/typeck/issue-31173/issue-31173.long-type-16022488263681348557.txt'
46    = note: the following trait bounds were not satisfied:
47            `<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@$DIR/issue-31173.rs:7:21: 7:25]> as Iterator>::Item = &_`
48            which is required by `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@$DIR/issue-31173.rs:7:21: 7:25]>>: Iterator`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-31173/issue-31173.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/issue-31173.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/typeck/issue-31173.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-31173" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-31173/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0271]: expected `TakeWhile<&mut IntoIter<u8>, [closure@issue-31173.rs:7:21]>` to be an iterator that yields `&_`, but it yields `u8`
  --> fake-test-src-base/typeck/issue-31173.rs:11:10
   |
LL |         .cloned() //~ ERROR to be an iterator that yields `&_`, but it yields `u8`
   |          ^^^^^^ expected `&_`, found `u8`
   = note: expected reference `&_`
                   found type `u8`
note: the method call chain might not have had the expected associated types
  --> fake-test-src-base/typeck/issue-31173.rs:3:20
  --> fake-test-src-base/typeck/issue-31173.rs:3:20
   |
LL |   pub fn get_tok(it: &mut IntoIter<u8>) {
   |                      ^^^^^^^^^^^^^^^^^ `Iterator::Item` is `u8` here
...
LL |           .take_while(|&x| {
   |  __________-
LL | |             found_e = true;
LL | |             false
LL | |         })
   | |__________- `Iterator::Item` remains `u8` here
note: required by a bound in `cloned`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/traits/iterator.rs:3358:5

error[E0599]: the method `collect` exists for struct `Cloned<TakeWhile<&mut IntoIter<u8>, [closure@issue-31173.rs:7:21]>>`, but its trait bounds were not satisfied
  --> fake-test-src-base/typeck/issue-31173.rs:12:10
   |
LL |       let temp: Vec<u8> = it
   |  _________________________-
LL | |         .take_while(|&x| {
LL | |             found_e = true;
LL | |             false
LL | |         })
LL | |         .cloned() //~ ERROR to be an iterator that yields `&_`, but it yields `u8`
LL | |         .collect(); //~ ERROR the method
   | |         -^^^^^^^ method cannot be called due to unsatisfied trait bounds
   | 
  --> /rustc/FAKE_PREFIX/library/core/src/iter/adapters/take_while.rs:15:1
   |
   |
   = note: doesn't satisfy `<_ as Iterator>::Item = &_`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/adapters/cloned.rs:17:1
   = note: doesn't satisfy `_: Iterator`
   |
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@fake-test-src-base/typeck/issue-31173.rs:7:21: 7:25]> as Iterator>::Item = &_`
           which is required by `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@fake-test-src-base/typeck/issue-31173.rs:7:21: 7:25]>>: Iterator`
           `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@fake-test-src-base/typeck/issue-31173.rs:7:21: 7:25]>>: Iterator`
           which is required by `&mut Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@fake-test-src-base/typeck/issue-31173.rs:7:21: 7:25]>>: Iterator`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0271, E0599.
For more information about an error, try `rustc --explain E0271`.
For more information about an error, try `rustc --explain E0271`.
------------------------------------------


---- [ui] tests/ui/unboxed-closures/unboxed-closures-static-call-wrong-trait.rs stdout ----
diff of stderr:

- error[E0599]: no method named `call` found for closure `[closure@unboxed-closures-static-call-wrong-trait.rs:6:26]` in the current scope
+ error[E0599]: no method named `call` found for closure `[closure@$DIR/unboxed-closures-static-call-wrong-trait.rs:6:26: 6:29]` in the current scope
3    |
3    |
4 LL |     mut_.call((0, ));

5    |          ^^^^ method not found in `[closure@unboxed-closures-static-call-wrong-trait.rs:6:26]`
-    |
-    = note: the full type name has been written to '$TEST_BUILD_DIR/unboxed-closures/unboxed-closures-static-call-wrong-trait/unboxed-closures-static-call-wrong-trait.long-type-1884555690480894559.txt'
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-static-call-wrong-trait/unboxed-closures-static-call-wrong-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closures-static-call-wrong-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/unboxed-closures/unboxed-closures-static-call-wrong-trait.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-static-call-wrong-trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-static-call-wrong-trait/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `call` found for closure `[closure@fake-test-src-base/unboxed-closures/unboxed-closures-static-call-wrong-trait.rs:6:26: 6:29]` in the current scope
  --> fake-test-src-base/unboxed-closures/unboxed-closures-static-call-wrong-trait.rs:7:10
   |
LL |     mut_.call((0, )); //~ ERROR no method named `call` found
   |          ^^^^ method not found in `[closure@unboxed-closures-static-call-wrong-trait.rs:6:26]`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
