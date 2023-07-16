plain
---- [ui] tests/ui/methods/method-not-found-generic-arg-elision.rs stdout ----
diff of stderr:

24    |
25 LL |     v.iter().map(|x| x * x).extend(std::iter::once(100));
26    |                             ^^^^^^ method not found in `Map<Iter<'_, i32>, [closure@method-not-found-generic-arg-elision.rs:87:18]>`
-    |
-    = note: the full type name has been written to '$TEST_BUILD_DIR/methods/method-not-found-generic-arg-elision/method-not-found-generic-arg-elision.long-type-9222998231749624688.txt'
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
  --> fake-test-src-base/methods/method-not-found-generic-arg-elision.rs:87:29
   |
   |
LL |     v.iter().map(|x| x * x).extend(std::iter::once(100));
   |                             ^^^^^^ method not found in `Map<Iter<'_, i32>, [closure@method-not-found-generic-arg-elision.rs:87:18]>`

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
