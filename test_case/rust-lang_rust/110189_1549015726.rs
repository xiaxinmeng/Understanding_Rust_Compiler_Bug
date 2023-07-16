plain
---- [ui] tests/ui/methods/method-not-found-generic-arg-elision.rs stdout ----
diff of stderr:

24    |
25 LL |     v.iter().map(Box::new(|x| x * x) as Box<dyn Fn(&i32) -> i32>).extend(std::iter::once(100));
26    |                                                                   ^^^^^^ method not found in `Map<Iter<'_, i32>, Box<dyn Fn(&i32) -> i32>>`
-    |
-    = note: the full type name has been written to '$TEST_BUILD_DIR/methods/method-not-found-generic-arg-elision/method-not-found-generic-arg-elision.long-type-11847890071882761621.txt'
29 
30 error[E0599]: no method named `method` found for struct `Wrapper<bool>` in the current scope


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-not-found-generic-arg-elision/method-not-found-generic-arg-elision.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-not-found-generic-arg-elision/method-not-found-generic-arg-elision.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args methods/method-not-found-generic-arg-elision.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/methods/method-not-found-generic-arg-elision.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-not-found-generic-arg-elision" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-not-found-generic-arg-elision/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `distance` found for struct `Point<i32>` in the current scope
  --> fake-test-src-base/methods/method-not-found-generic-arg-elision.rs:82:23
LL | struct Point<T> {
   | --------------- method `distance` not found for this struct
...
...
LL |     let d = point_i32.distance();
   |                       ^^^^^^^^ method not found in `Point<i32>`
   = note: the method was found for
   = note: the method was found for
           - `Point<f64>`

error[E0599]: no method named `other` found for struct `Point` in the current scope
  --> fake-test-src-base/methods/method-not-found-generic-arg-elision.rs:84:23
LL | struct Point<T> {
LL | struct Point<T> {
   | --------------- method `other` not found for this struct
...
LL |     let d = point_i32.other();
   |                       ^^^^^ method not found in `Point<i32>`
error[E0599]: no method named `extend` found for struct `Map` in the current scope
  --> fake-test-src-base/methods/method-not-found-generic-arg-elision.rs:87:67
   |
   |
LL |     v.iter().map(Box::new(|x| x * x) as Box<dyn Fn(&i32) -> i32>).extend(std::iter::once(100));
   |                                                                   ^^^^^^ method not found in `Map<Iter<'_, i32>, Box<dyn Fn(&i32) -> i32>>`

error[E0599]: no method named `method` found for struct `Wrapper<bool>` in the current scope
  --> fake-test-src-base/methods/method-not-found-generic-arg-elision.rs:90:13
   |
LL | struct Wrapper<T>(T);
   | ----------------- method `method` not found for this struct
LL |     wrapper.method();
LL |     wrapper.method();
   |             ^^^^^^ method not found in `Wrapper<bool>`
   = note: the method was found for
   = note: the method was found for
           - `Wrapper<i8>`
           - `Wrapper<i16>`
           - `Wrapper<i32>`
           - `Wrapper<i64>`
           and 2 more types

error[E0599]: no method named `other` found for struct `Wrapper` in the current scope
  --> fake-test-src-base/methods/method-not-found-generic-arg-elision.rs:92:13
   |
LL | struct Wrapper<T>(T);
   | ----------------- method `other` not found for this struct
LL |     wrapper.other();
LL |     wrapper.other();
   |             ^^^^^ method not found in `Wrapper<bool>`

error[E0599]: no method named `method` found for struct `Wrapper2<'_, bool, 3>` in the current scope
  --> fake-test-src-base/methods/method-not-found-generic-arg-elision.rs:96:13
   |
LL | struct Wrapper2<'a, T, const C: usize> {
   | -------------------------------------- method `method` not found for this struct
LL |     wrapper.method();
LL |     wrapper.method();
   |             ^^^^^^ method not found in `Wrapper2<'_, bool, 3>`
   = note: the method was found for
   = note: the method was found for
           - `Wrapper2<'a, i8, C>`
           - `Wrapper2<'a, i16, C>`
           - `Wrapper2<'a, i32, C>`

error[E0599]: no method named `other` found for struct `Wrapper2` in the current scope
  --> fake-test-src-base/methods/method-not-found-generic-arg-elision.rs:98:13
   |
LL | struct Wrapper2<'a, T, const C: usize> {
   | -------------------------------------- method `other` not found for this struct
LL |     wrapper.other();
LL |     wrapper.other();
   |             ^^^^^ method not found in `Wrapper2<'_, bool, 3>`

error[E0599]: no method named `not_found` found for struct `Vec<{integer}>` in the current scope
  --> fake-test-src-base/methods/method-not-found-generic-arg-elision.rs:101:7
LL |     a.not_found();
LL |     a.not_found();
   |       ^^^^^^^^^ method not found in `Vec<{integer}>`

error[E0599]: the method `method` exists for struct `Struct<f64>`, but its trait bounds were not satisfied
  --> fake-test-src-base/methods/method-not-found-generic-arg-elision.rs:104:7
LL | struct Struct<T> {
LL | struct Struct<T> {
   | ---------------- method `method` not found for this struct
...
LL |     s.method();
   |       ^^^^^^ method cannot be called on `Struct<f64>` due to unsatisfied trait bounds
note: the following trait bounds were not satisfied:
note: the following trait bounds were not satisfied:
      `f64: Eq`
      `f64: Ord`
  --> fake-test-src-base/methods/method-not-found-generic-arg-elision.rs:74:36
   |
LL | impl<T: Clone + Copy + PartialEq + Eq + PartialOrd + Ord> Struct<T> {
   |                                    ^^                ^^^  ---------
   |                                    |                 unsatisfied trait bound introduced here
   |                                    unsatisfied trait bound introduced here

error: aborting due to 9 previous errors
---

27    |
28    = note: doesn't satisfy `_: Iterator`
29    |
-    = note: the full type name has been written to '$TEST_BUILD_DIR/mismatched_types/issue-36053-2/issue-36053-2.long-type-2523243994590504090.txt'
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
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-83150/issue-83150.long-type-1477129275729476159.txt'
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0275`.
------------------------------------------
---

42    |
43    = note: doesn't satisfy `_: Iterator`
44    |
-    = note: the full type name has been written to '$TEST_BUILD_DIR/typeck/issue-31173/issue-31173.long-type-1298596514340768542.txt'
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
