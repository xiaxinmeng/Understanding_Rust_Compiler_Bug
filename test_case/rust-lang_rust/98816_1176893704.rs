plain
........................................................................................ 2816/13155
........................................................................................ 2904/13155
.................................................i...................................... 2992/13155
..............i......................................................................... 3080/13155
...F.....................................................FF............................. 3168/13155
........................................................................................ 3344/13155
........................................................................................ 3432/13155
........................................................................................ 3520/13155
........................................................................................ 3608/13155
---
........................................................................................ 7568/13155
........................................................................................ 7656/13155
..ii.....................................................................F.............. 7744/13155
........................................................................................ 7832/13155
..................................................................F..................... 7920/13155
.F....ii...........F....i....i..ii...................................................... 8008/13155
........................................................................................ 8184/13155
........................................................................................ 8272/13155
........................................................................................ 8360/13155
.............................................i..ii...................................... 8448/13155
---
..............................................F......................................... 11088/13155
........................................................................................ 11176/13155
........................................................................................ 11264/13155
........................................................................................ 11352/13155
................................................F.................................F..... 11440/13155
.............................................................F.......................... 11528/13155
...............F........................................................................ 11616/13155
...i........i........i.....i..............................i............................. 11704/13155
.....................................................................F.F................ 11792/13155
........................................................................................ 11968/13155
........................................................................................ 12056/13155
........................................................................................ 12144/13155
........................................................................................ 12232/13155
........................................................................................ 12232/13155
............................................................................i........... 12320/13155
........................................................................................ 12408/13155
........................................................................................ 12496/13155
........................................................................................ 12584/13155
........................................................................................ 12672/13155
..........................................F.....F....................................... 12760/13155
......F.F............................................................................... 12848/13155
........................................................................................ 13024/13155
........................iii............................................................. 13112/13155
...........................................
failures:
failures:

---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-80742.rs stdout ----
diff of stderr:

24   ::: $SRC_DIR/core/src/fmt/mod.rs:LL:COL
25    |
26 LL | pub trait Debug {
-    |           ----- doesn't satisfy `dyn Debug: Sized`
+    | --------------- doesn't satisfy `dyn Debug: Sized`
29    = note: the following trait bounds were not satisfied:
29    = note: the following trait bounds were not satisfied:
30            `dyn Debug: Sized`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-80742/issue-80742.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/issue-80742.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-80742" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-80742/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of `Inline::<dyn std::fmt::Debug>::{constant#0}` failed
   |
LL |     intrinsics::size_of::<T>()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |
   |     |
   |     size_of called on unsized type `dyn Debug`
   |     inside `std::mem::size_of::<dyn Debug>` at /checkout/library/core/src/mem/mod.rs:311:5
  ::: /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs:22:10
   |
   |
LL |     [u8; size_of::<T>() + 1]: ,
   |          -------------- inside `Inline::<dyn Debug>::{constant#0}` at /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs:22:10

error[E0599]: the function or associated item `new` exists for struct `Inline<dyn Debug>`, but its trait bounds were not satisfied
   |
LL | struct Inline<T>
   |        ------ function or associated item `new` not found for this struct
...
...
LL |     let dst = Inline::<dyn Debug>::new(0); //~ ERROR
   |                                    ^^^ function or associated item cannot be called on `Inline<dyn Debug>` due to unsatisfied trait bounds
  ::: /checkout/library/core/src/fmt/mod.rs:658:1
   |
LL | pub trait Debug {
LL | pub trait Debug {
   | --------------- doesn't satisfy `dyn Debug: Sized`
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `dyn Debug: Sized`

error[E0080]: evaluation of `Inline::<dyn std::fmt::Debug>::{constant#0}` failed
   |
LL |     intrinsics::size_of::<T>()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |
   |     |
   |     size_of called on unsized type `dyn Debug`
   |     inside `std::mem::size_of::<dyn Debug>` at /checkout/library/core/src/mem/mod.rs:311:5
  ::: /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs:14:10
   |
   |
LL |     [u8; size_of::<T>() + 1]: ,
   |          -------------- inside `Inline::<dyn Debug>::{constant#0}` at /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs:14:10
error[E0277]: the size for values of type `dyn Debug` cannot be known at compilation time
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs:30:15
   |
   |
LL |     let dst = Inline::<dyn Debug>::new(0); //~ ERROR
   |
   = help: the trait `Sized` is not implemented for `dyn Debug`
note: required by a bound in `Inline`
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs:12:15
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs:12:15
   |
LL | struct Inline<T>
   |               ^ required by this bound in `Inline`
help: consider relaxing the implicit `Sized` restriction
   |
LL | struct Inline<T: ?Sized>

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0080, E0277, E0599.
---
diff of stderr:

2   --> $DIR/derive-assoc-type-not-impl.rs:18:30
3    |
4 LL | struct Bar<T: Foo> {
-    |        |
-    |        method `clone` not found for this struct
-    |        method `clone` not found for this struct
-    |        doesn't satisfy `Bar<NotClone>: Clone`
+    | |      |
+    | |      method `clone` not found for this struct
+    | |      method `clone` not found for this struct
+    | doesn't satisfy `Bar<NotClone>: Clone`
9 ...
10 LL | struct NotClone;
-    |        -------- doesn't satisfy `NotClone: Clone`
+    | --------------- doesn't satisfy `NotClone: Clone`
12 ...
13 LL |     Bar::<NotClone> { x: 1 }.clone();
14    |                              ^^^^^ method cannot be called on `Bar<NotClone>` due to unsatisfied trait bounds

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derive-assoc-type-not-impl/derive-assoc-type-not-impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args derives/derive-assoc-type-not-impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derive-assoc-type-not-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derive-assoc-type-not-impl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derive-assoc-type-not-impl/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `clone` exists for struct `Bar<NotClone>`, but its trait bounds were not satisfied
   |
   |
LL | struct Bar<T: Foo> {
   | |      |
   | |      method `clone` not found for this struct
   | |      method `clone` not found for this struct
   | doesn't satisfy `Bar<NotClone>: Clone`
...
LL | struct NotClone;
   | --------------- doesn't satisfy `NotClone: Clone`
...
LL |     Bar::<NotClone> { x: 1 }.clone(); //~ ERROR
   |                              ^^^^^ method cannot be called on `Bar<NotClone>` due to unsatisfied trait bounds
   |
note: trait bound `NotClone: Clone` was not satisfied
   |
LL | #[derive(Clone)]
   |          ^^^^^ unsatisfied trait bound introduced in this `derive` macro
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `NotClone: Clone`
           which is required by `Bar<NotClone>: Clone`
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `clone`, perhaps you need to implement it:
           candidate #1: `Clone`
help: consider annotating `NotClone` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |

error: aborting due to previous error
---
diff of stderr:

2   --> $DIR/issue-91492.rs:4:9
3    |
4 LL | pub struct NoDerives;
-    |            --------- doesn't satisfy `NoDerives: Clone`
+    | -------------------- doesn't satisfy `NoDerives: Clone`
6 LL | fn fun1(foo: &mut Vec<NoDerives>, bar: &[NoDerives]) {
7 LL |     foo.extend_from_slice(bar);

18   --> $DIR/issue-91492.rs:12:9
19    |
19    |
20 LL | pub struct SomeDerives;
-    |            ----------- doesn't satisfy `SomeDerives: Clone`
+    | ---------------------- doesn't satisfy `SomeDerives: Clone`
22 LL | fn fun2(foo: &mut Vec<SomeDerives>, bar: &[SomeDerives]) {
23 LL |     foo.extend_from_slice(bar);

34   --> $DIR/issue-91492.rs:22:9
35    |
35    |
36 LL | pub struct NoDerives;
-    |            --------- doesn't satisfy `NoDerives: Clone`
+    | -------------------- doesn't satisfy `NoDerives: Clone`
38 ...
39 LL | struct Object<T, A>(T, A);
40    |        ------ method `use_clone` not found for this struct

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/issue-91492/issue-91492.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args derives/issue-91492.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/issue-91492.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/issue-91492" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/issue-91492/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `extend_from_slice` exists for mutable reference `&mut Vec<NoDerives>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct NoDerives;
   | -------------------- doesn't satisfy `NoDerives: Clone`
LL | fn fun1(foo: &mut Vec<NoDerives>, bar: &[NoDerives]) {
LL |     foo.extend_from_slice(bar); //~ ERROR
   |
   = note: the following trait bounds were not satisfied:
           `NoDerives: Clone`
           `NoDerives: Clone`
help: consider annotating `NoDerives` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |


error[E0599]: the method `extend_from_slice` exists for mutable reference `&mut Vec<SomeDerives>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct SomeDerives;
   | ---------------------- doesn't satisfy `SomeDerives: Clone`
LL | fn fun2(foo: &mut Vec<SomeDerives>, bar: &[SomeDerives]) {
LL |     foo.extend_from_slice(bar); //~ ERROR
   |
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `SomeDerives: Clone`
help: consider annotating `SomeDerives` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |


error[E0599]: the method `use_clone` exists for struct `Object<NoDerives, SomeDerives>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct NoDerives;
   | -------------------- doesn't satisfy `NoDerives: Clone`
...
LL | struct Object<T, A>(T, A);
   |        ------ method `use_clone` not found for this struct
...
LL |     foo.use_clone(); //~ ERROR
   |         ^^^^^^^^^ method cannot be called on `Object<NoDerives, SomeDerives>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
           `NoDerives: Clone`
           `NoDerives: Clone`
help: consider annotating `NoDerives` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |

error: aborting due to 3 previous errors
---
3    |
4 LL | struct Value(u32);
-    |        -----
-    |        |
-    |        doesn't satisfy `Value: Eq`
-    |        doesn't satisfy `Value: Hash`
+    | |
+    | |
+    | doesn't satisfy `Value: Eq`
+    | doesn't satisfy `Value: Hash`
9 ...
10 LL |     hs.insert(Value(0));

22   --> $DIR/issue-91550.rs:26:9
23    |
23    |
24 LL | pub struct NoDerives;
-    |            --------- doesn't satisfy `NoDerives: Eq`
+    | -------------------- doesn't satisfy `NoDerives: Eq`
26 LL |
27 LL | struct Object<T>(T);
28    |        ------ method `use_eq` not found for this struct
41   --> $DIR/issue-91550.rs:27:9
42    |
42    |
43 LL | pub struct NoDerives;
-    |            --------- doesn't satisfy `NoDerives: Ord`
+    | -------------------- doesn't satisfy `NoDerives: Ord`
45 LL |
46 LL | struct Object<T>(T);
47    |        ------ method `use_ord` not found for this struct
60   --> $DIR/issue-91550.rs:28:9
61    |
61    |
62 LL | pub struct NoDerives;
-    |            |
-    |            |
-    |            doesn't satisfy `NoDerives: Ord`
-    |            doesn't satisfy `NoDerives: PartialOrd`
+    | |
+    | |
+    | doesn't satisfy `NoDerives: Ord`
+    | doesn't satisfy `NoDerives: PartialOrd`
67 LL |
68 LL | struct Object<T>(T);
69    |        ------ method `use_ord_and_partial_ord` not found for this struct

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/issue-91550/issue-91550.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args derives/issue-91550.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/issue-91550.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/issue-91550" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/issue-91550/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `insert` exists for struct `HashSet<Value>`, but its trait bounds were not satisfied
   |
LL | struct Value(u32);
   | ------------
   | |
   | |
   | doesn't satisfy `Value: Eq`
   | doesn't satisfy `Value: Hash`
...
LL |     hs.insert(Value(0)); //~ ERROR
   |
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `Value: Eq`
           `Value: Hash`
help: consider annotating `Value` with `#[derive(Eq, Hash, PartialEq)]`
   |
LL | #[derive(Eq, Hash, PartialEq)]


error[E0599]: the method `use_eq` exists for struct `Object<NoDerives>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct NoDerives;
   | -------------------- doesn't satisfy `NoDerives: Eq`
LL |
LL | struct Object<T>(T);
   |        ------ method `use_eq` not found for this struct
...
LL |     foo.use_eq(); //~ ERROR
   |         ^^^^^^ method cannot be called on `Object<NoDerives>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `NoDerives: Eq`
help: consider annotating `NoDerives` with `#[derive(Eq, PartialEq)]`
   |
LL | #[derive(Eq, PartialEq)]


error[E0599]: the method `use_ord` exists for struct `Object<NoDerives>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct NoDerives;
   | -------------------- doesn't satisfy `NoDerives: Ord`
LL |
LL | struct Object<T>(T);
   |        ------ method `use_ord` not found for this struct
...
LL |     foo.use_ord(); //~ ERROR
   |         ^^^^^^^ method cannot be called on `Object<NoDerives>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `NoDerives: Ord`
help: consider annotating `NoDerives` with `#[derive(Eq, Ord, PartialEq, PartialOrd)]`
   |
LL | #[derive(Eq, Ord, PartialEq, PartialOrd)]


error[E0599]: the method `use_ord_and_partial_ord` exists for struct `Object<NoDerives>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct NoDerives;
   | |
   | |
   | doesn't satisfy `NoDerives: Ord`
   | doesn't satisfy `NoDerives: PartialOrd`
LL |
LL | struct Object<T>(T);
   |        ------ method `use_ord_and_partial_ord` not found for this struct
...
LL |     foo.use_ord_and_partial_ord(); //~ ERROR
   |         ^^^^^^^^^^^^^^^^^^^^^^^ method cannot be called on `Object<NoDerives>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `NoDerives: Ord`
           `NoDerives: PartialOrd`
help: consider annotating `NoDerives` with `#[derive(Eq, Ord, PartialEq, PartialOrd)]`
   |
LL | #[derive(Eq, Ord, PartialEq, PartialOrd)]

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0599`.
---
3    |
4 LL | struct S;
-    |        -
-    |        |
-    |        method `f` not found for this struct
-    |        doesn't satisfy `<S as X>::Y<i32> = i32`
-    |        doesn't satisfy `S: M`
+    | |      |
+    | |      method `f` not found for this struct
+    | |      method `f` not found for this struct
+    | doesn't satisfy `<S as X>::Y<i32> = i32`
+    | doesn't satisfy `S: M`
11 LL |     a.f();
12    |       ^ method cannot be called on `S` due to unsatisfied trait bounds



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate/method-unsatified-assoc-type-predicate.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/method-unsatified-assoc-type-predicate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `f` exists for struct `S`, but its trait bounds were not satisfied
   |
LL | struct S;
   | --------
   | |      |
   | |      |
   | |      method `f` not found for this struct
   | doesn't satisfy `<S as X>::Y<i32> = i32`
   | doesn't satisfy `S: M`
LL |     a.f();
   |       ^ method cannot be called on `S` due to unsatisfied trait bounds
   |
   |
note: trait bound `<S as X>::Y<i32> = i32` was not satisfied
   |
   |
LL | impl<T: X<Y<i32> = i32>> M for T {}
   |           ^^^^^^^^^^^^   -     -
   |           unsatisfied trait bound introduced here

error: aborting due to previous error

---
diff of stderr:

2   --> $DIR/issue-30786.rs:118:22
3    |
4 LL | pub struct Map<S, F> {
-    |            |
-    |            |
-    |            method `filterx` not found for this struct
-    |            doesn't satisfy `_: StreamExt`
+    | |          |
+    | |          |
+    | |          method `filterx` not found for this struct
+    | doesn't satisfy `_: StreamExt`
9 ...
10 LL |     let filter = map.filterx(|x: &_| true);
11    |                      ^^^^^^^ method cannot be called on `Map<Repeat, [closure@$DIR/issue-30786.rs:117:27: 117:36]>` due to unsatisfied trait bounds
27   --> $DIR/issue-30786.rs:130:24
28    |
28    |
29 LL | pub struct Filter<S, F> {
-    |            |
-    |            |
-    |            method `countx` not found for this struct
-    |            doesn't satisfy `_: StreamExt`
+    | |          |
+    | |          |
+    | |          method `countx` not found for this struct
+    | doesn't satisfy `_: StreamExt`
34 ...
35 LL |     let count = filter.countx();
36    |                        ^^^^^^ method cannot be called on `Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@$DIR/issue-30786.rs:129:30: 129:42]>` due to unsatisfied trait bounds

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-30786/issue-30786.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hrtb/issue-30786.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hrtb/issue-30786.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-30786" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-30786/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `filterx` exists for struct `Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:117:27: 117:36]>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct Map<S, F> {
   | |          |
   | |          |
   | |          method `filterx` not found for this struct
   | doesn't satisfy `_: StreamExt`
...
LL |     let filter = map.filterx(|x: &_| true);
   |                      ^^^^^^^ method cannot be called on `Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:117:27: 117:36]>` due to unsatisfied trait bounds
note: the following trait bounds were not satisfied:
note: the following trait bounds were not satisfied:
      `&'a mut &Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:117:27: 117:36]>: Stream`
      `&'a mut &mut Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:117:27: 117:36]>: Stream`
      `&'a mut Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:117:27: 117:36]>: Stream`
   |
   |
LL | impl<T> StreamExt for T where for<'a> &'a mut T: Stream {}
   |         ---------     -                          ^^^^^^ unsatisfied trait bound introduced here
help: one of the expressions' fields has a method of the same name
   |
LL |     let filter = map.stream.filterx(|x: &_| true);


error[E0599]: the method `countx` exists for struct `Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:129:30: 129:42]>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct Filter<S, F> {
   | |          |
   | |          |
   | |          method `countx` not found for this struct
   | doesn't satisfy `_: StreamExt`
...
LL |     let count = filter.countx();
   |                        ^^^^^^ method cannot be called on `Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:129:30: 129:42]>` due to unsatisfied trait bounds
note: the following trait bounds were not satisfied:
note: the following trait bounds were not satisfied:
      `&'a mut &Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:129:30: 129:42]>: Stream`
      `&'a mut &mut Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:129:30: 129:42]>: Stream`
      `&'a mut Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:129:30: 129:42]>: Stream`
   |
   |
LL | impl<T> StreamExt for T where for<'a> &'a mut T: Stream {}
   |         ---------     -                          ^^^^^^ unsatisfied trait bound introduced here
help: one of the expressions' fields has a method of the same name
   |
LL |     let count = filter.stream.countx();

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.
------------------------------------------


---- [ui] src/test/ui/impl-trait/issues/issue-62742.rs stdout ----
diff of stderr:

18    |                      ^^^ function or associated item cannot be called on `SafeImpl<(), RawImpl<()>>` due to unsatisfied trait bounds
19 ...
20 LL | pub struct RawImpl<T>(PhantomData<T>);
-    |            ------- doesn't satisfy `RawImpl<()>: Raw<()>`
+    | --------------------- doesn't satisfy `RawImpl<()>: Raw<()>`
22 ...
23 LL | pub struct SafeImpl<T: ?Sized, A: Raw<T>>(PhantomData<(A, T)>);
24    |            -------- function or associated item `foo` not found for this struct

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-62742/issue-62742.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/issues/issue-62742.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-62742.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-62742" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-62742/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `RawImpl<_>: Raw<_>` is not satisfied
   |
   |
LL |     WrongImpl::foo(0i32);
   |     ^^^^^^^^^ the trait `Raw<_>` is not implemented for `RawImpl<_>`
   |
   = help: the trait `Raw<[T]>` is implemented for `RawImpl<T>`
note: required by a bound in `SafeImpl`
   |
   |
LL | pub struct SafeImpl<T: ?Sized, A: Raw<T>>(PhantomData<(A, T)>);
   |                                   ^^^^^^ required by this bound in `SafeImpl`

error[E0599]: the function or associated item `foo` exists for struct `SafeImpl<(), RawImpl<()>>`, but its trait bounds were not satisfied
   |
   |
LL |     WrongImpl::<()>::foo(0i32);
   |                      ^^^ function or associated item cannot be called on `SafeImpl<(), RawImpl<()>>` due to unsatisfied trait bounds
...
LL | pub struct RawImpl<T>(PhantomData<T>);
   | --------------------- doesn't satisfy `RawImpl<()>: Raw<()>`
...
LL | pub struct SafeImpl<T: ?Sized, A: Raw<T>>(PhantomData<(A, T)>);
   |            -------- function or associated item `foo` not found for this struct
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `RawImpl<()>: Raw<()>`
  --> /checkout/src/test/ui/impl-trait/issues/issue-62742.rs:12:1
   |
   |
LL | pub trait Raw<T: ?Sized> {


error[E0277]: the trait bound `RawImpl<()>: Raw<()>` is not satisfied
   |
   |
LL |     WrongImpl::<()>::foo(0i32);
   |     ^^^^^^^^^^^^^^^ the trait `Raw<()>` is not implemented for `RawImpl<()>`
   |
   = help: the trait `Raw<[T]>` is implemented for `RawImpl<T>`
note: required by a bound in `SafeImpl`
   |
   |
LL | pub struct SafeImpl<T: ?Sized, A: Raw<T>>(PhantomData<(A, T)>);
   |                                   ^^^^^^ required by this bound in `SafeImpl`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.
---
diff of stderr:

21   ::: $SRC_DIR/core/src/iter/adapters/cloned.rs:LL:COL
22    |
23 LL | pub struct Cloned<I> {
+    | -------------------- doesn't satisfy `_: Iterator`
25    |
26   ::: $SRC_DIR/core/src/iter/adapters/take_while.rs:LL:COL
27    |
27    |

28 LL | pub struct TakeWhile<I, P> {
-    |            --------- doesn't satisfy `<_ as Iterator>::Item = &_`
+    | -------------------------- doesn't satisfy `<_ as Iterator>::Item = &_`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-31173.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31173" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31173/auxiliary"
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
  --> /checkout/library/core/src/iter/traits/iterator.rs:3280:32
   |
LL |         Self: Sized + Iterator<Item = &'a T>,
   |                                ^^^^^^^^^^^^ required by this bound in `cloned`

error[E0599]: the method `collect` exists for struct `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6]>>`, but its trait bounds were not satisfied
   |
   |
LL |         .collect(); //~ ERROR the method
   |          ^^^^^^^ method cannot be called on `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6]>>` due to unsatisfied trait bounds
  ::: /checkout/library/core/src/iter/adapters/cloned.rs:17:1
   |
   |
LL | pub struct Cloned<I> {
   |
  ::: /checkout/library/core/src/iter/adapters/take_while.rs:15:1
   |
   |
LL | pub struct TakeWhile<I, P> {
   | -------------------------- doesn't satisfy `<_ as Iterator>::Item = &_`
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6]> as Iterator>::Item = &_`
           which is required by `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6]>>: Iterator`
           `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6]>>: Iterator`
           which is required by `&mut Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6]>>: Iterator`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0271, E0599.
For more information about an error, try `rustc --explain E0271`.
---
diff of stderr:

7   ::: $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
8    |
9 LL | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
-    |            --- doesn't satisfy `Vec<bool>: Iterator`
+    | ------------------------------------------------------------------------------------------------ doesn't satisfy `Vec<bool>: Iterator`
12    = note: the following trait bounds were not satisfied:
12    = note: the following trait bounds were not satisfied:
13            `Vec<bool>: Iterator`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/vec-on-unimplemented/vec-on-unimplemented.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args iterators/vec-on-unimplemented.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/vec-on-unimplemented.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/vec-on-unimplemented" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/vec-on-unimplemented/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: `Vec<bool>` is not an iterator
   |
   |
LL |     vec![true, false].map(|v| !v).collect::<Vec<_>>();
   |                       ^^^ `Vec<bool>` is not an iterator; try calling `.into_iter()` or `.iter()`
  ::: /checkout/library/alloc/src/vec/mod.rs:400:1
   |
   |
LL | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
   | ------------------------------------------------------------------------------------------------ doesn't satisfy `Vec<bool>: Iterator`
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `Vec<bool>: Iterator`
           which is required by `&mut Vec<bool>: Iterator`
           `[bool]: Iterator`
           which is required by `&mut [bool]: Iterator`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
---
51    |
52 LL | pub struct Foo;
-    |            ---
-    |            |
-    |            method `take` not found for this struct
-    |            doesn't satisfy `Foo: Iterator`
+    | |          |
+    | |          |
+    | |          method `take` not found for this struct
+    | doesn't satisfy `Foo: Iterator`
58 LL |      .take()
59    |       ^^^^ `Foo` is not an iterator



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-err-msg/method-call-err-msg.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args methods/method-call-err-msg.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-call-err-msg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-err-msg" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-err-msg/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:13:7
   |
   |
LL |     x.zero(0)   //~ ERROR this function takes 0 arguments but 1 argument was supplied
   |       ^^^^ - argument of type `{integer}` unexpected
note: associated function defined here
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:5:8
   |
   |
LL |     fn zero(self) -> Foo { self }
help: remove the extra argument
   |
   |
LL |     x.zero()   //~ ERROR this function takes 0 arguments but 1 argument was supplied

error[E0061]: this function takes 1 argument but 0 arguments were supplied
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:14:7
   |
   |
LL |      .one()     //~ ERROR this function takes 1 argument but 0 arguments were supplied
   |       ^^^-- an argument of type `isize` is missing
note: associated function defined here
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:6:8
   |
   |
LL |     fn one(self, _: isize) -> Foo { self }
help: provide the argument
   |
   |
LL |      .one(/* isize */)     //~ ERROR this function takes 1 argument but 0 arguments were supplied

error[E0061]: this function takes 2 arguments but 1 argument was supplied
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:15:7
   |
   |
LL |      .two(0);   //~ ERROR this function takes 2 arguments but 1 argument was supplied
   |       ^^^--- an argument of type `isize` is missing
note: associated function defined here
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:7:8
   |
   |
LL |     fn two(self, _: isize, _: isize) -> Foo { self }
help: provide the argument
   |
   |
LL |      .two(0, /* isize */);   //~ ERROR this function takes 2 arguments but 1 argument was supplied

error[E0599]: `Foo` is not an iterator
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:19:7
   |
   |
LL | pub struct Foo;
   | --------------
   | |          |
   | |          method `take` not found for this struct
   | doesn't satisfy `Foo: Iterator`
...
LL |      .take()    //~ ERROR not an iterator
   |       ^^^^ `Foo` is not an iterator
   = note: the following trait bounds were not satisfied:
           `Foo: Iterator`
           which is required by `&mut Foo: Iterator`
note: the following trait must be implemented
note: the following trait must be implemented
  --> /checkout/library/core/src/iter/traits/iterator.rs:67:1
   |
LL | pub trait Iterator {
   | ^^^^^^^^^^^^^^^^^^
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `take`, perhaps you need to implement it:
           candidate #1: `Iterator`
error[E0061]: this function takes 3 arguments but 0 arguments were supplied
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:21:7
   |
   |
LL |     y.three::<usize>(); //~ ERROR this function takes 3 arguments but 0 arguments were supplied
   |       ^^^^^^^^^^^^^^-- three arguments of type `usize`, `usize`, and `usize` are missing
note: associated function defined here
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:8:8
   |
   |
LL |     fn three<T>(self, _: T, _: T, _: T) -> Foo { self }
help: provide the arguments
   |
   |
LL |     y.three::<usize>(/* usize */, /* usize */, /* usize */); //~ ERROR this function takes 3 arguments but 0 arguments were supplied

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0061, E0599.
---
diff of stderr:

24   ::: $SRC_DIR/core/src/iter/adapters/filter.rs:LL:COL
25    |
26 LL | pub struct Filter<I, P> {
+    | ----------------------- doesn't satisfy `_: Iterator`
28    |
29    = note: the following trait bounds were not satisfied:
29    = note: the following trait bounds were not satisfied:
30            `<[closure@$DIR/issue-36053-2.rs:7:39: 7:53] as FnOnce<(&&str,)>>::Output = bool`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2/issue-36053-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/issue-36053-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/issue-36053-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:32
   |
   |
LL |     once::<&str>("str").fuse().filter(|a: &str| true).count();
   |                                ^^^^^^ -------------- found signature of `for<'r> fn(&'r str) -> _`
   |                                |
   |                                expected signature of `for<'r> fn(&'r &str) -> _`
note: required by a bound in `filter`
  --> /checkout/library/core/src/iter/traits/iterator.rs:899:12
   |
   |
LL |         P: FnMut(&Self::Item) -> bool,
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `filter`

error[E0599]: the method `count` exists for struct `Filter<Fuse<std::iter::Once<&str>>, [closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:39: 7:53]>`, but its trait bounds were not satisfied
   |
   |
LL |     once::<&str>("str").fuse().filter(|a: &str| true).count();
   |                                       --------------  ^^^^^ method cannot be called on `Filter<Fuse<std::iter::Once<&str>>, [closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:39: 7:53]>` due to unsatisfied trait bounds
   |                                       |
   |                                       doesn't satisfy `<_ as FnOnce<(&&str,)>>::Output = bool`
   |                                       doesn't satisfy `_: FnMut<(&&str,)>`
  ::: /checkout/library/core/src/iter/adapters/filter.rs:15:1
   |
   |
LL | pub struct Filter<I, P> {
   |
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `<[closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:39: 7:53] as FnOnce<(&&str,)>>::Output = bool`
           which is required by `Filter<Fuse<std::iter::Once<&str>>, [closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:39: 7:53]>: Iterator`
           `[closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:39: 7:53]: FnMut<(&&str,)>`
           which is required by `Filter<Fuse<std::iter::Once<&str>>, [closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:39: 7:53]>: Iterator`
           `Filter<Fuse<std::iter::Once<&str>>, [closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:39: 7:53]>: Iterator`
           which is required by `&mut Filter<Fuse<std::iter::Once<&str>>, [closure@/checkout/src/test/ui/mismatched_types/issue-36053-2.rs:7:39: 7:53]>: Iterator`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0599, E0631.
For more information about an error, try `rustc --explain E0599`.
---
diff of stderr:

2   --> $DIR/missing-trait-bounds-for-method-call.rs:14:14
3    |
4 LL | struct Foo<T> {
-    |        --- doesn't satisfy `Foo<T>: Bar`
+    | ------------- doesn't satisfy `Foo<T>: Bar`
6 ...
7 LL |         self.foo();
8    |              ^^^ method cannot be called on `&Foo<T>` due to unsatisfied trait bounds
30   --> $DIR/missing-trait-bounds-for-method-call.rs:27:14
31    |
31    |
32 LL | struct Fin<T> where T: Bar {
-    |        --- doesn't satisfy `Fin<T>: Bar`
+    | ------------- doesn't satisfy `Fin<T>: Bar`
34 ...
35 LL |         self.foo();
36    |              ^^^ method cannot be called on `&Fin<T>` due to unsatisfied trait bounds

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing-trait-bounds/missing-trait-bounds-for-method-call/missing-trait-bounds-for-method-call.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args missing-trait-bounds/missing-trait-bounds-for-method-call.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/missing-trait-bounds/missing-trait-bounds-for-method-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing-trait-bounds/missing-trait-bounds-for-method-call" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing-trait-bounds/missing-trait-bounds-for-method-call/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `foo` exists for reference `&Foo<T>`, but its trait bounds were not satisfied
   |
LL | struct Foo<T> {
LL | struct Foo<T> {
   | ------------- doesn't satisfy `Foo<T>: Bar`
...
LL |         self.foo();
   |              ^^^ method cannot be called on `&Foo<T>` due to unsatisfied trait bounds
   |
note: trait bound `T: Default` was not satisfied
   |
   |
LL | impl<T: Default + Bar> Bar for Foo<T> {}
   |         |
   |         unsatisfied trait bound introduced here
   |         unsatisfied trait bound introduced here
note: trait bound `T: Bar` was not satisfied
   |
   |
LL | impl<T: Default + Bar> Bar for Foo<T> {}
   |                   ^^^  ---     ------
   |                   unsatisfied trait bound introduced here
help: consider restricting the type parameters to satisfy the trait bounds
   |
   |
LL | struct Foo<T> where T: Bar, T: Default {


error[E0599]: the method `foo` exists for reference `&Fin<T>`, but its trait bounds were not satisfied
   |
   |
LL | struct Fin<T> where T: Bar {
   | ------------- doesn't satisfy `Fin<T>: Bar`
...
LL |         self.foo();
   |              ^^^ method cannot be called on `&Fin<T>` due to unsatisfied trait bounds
   |
note: trait bound `T: Default` was not satisfied
   |
   |
LL | impl<T: Default + Bar> Bar for Fin<T> {}
   |         |
   |         unsatisfied trait bound introduced here
help: consider restricting the type parameter to satisfy the trait bound
   |
   |
LL | struct Fin<T> where T: Bar, T: Default {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.
------------------------------------------


---- [ui] src/test/ui/missing-trait-bounds/issue-69725.rs stdout ----
diff of stderr:

4 LL |     let _ = Struct::<A>::new().clone();
5    |                                ^^^^^ method cannot be called on `Struct<A>` due to unsatisfied trait bounds
-   ::: $DIR/auxiliary/issue-69725.rs:2:12
+   ::: $DIR/auxiliary/issue-69725.rs:2:1
8    |
8    |
9 LL | pub struct Struct<A>(A);
-    |            ------ doesn't satisfy `Struct<A>: Clone`
+    | -------------------- doesn't satisfy `Struct<A>: Clone`
12    = note: the following trait bounds were not satisfied:
12    = note: the following trait bounds were not satisfied:
13            `A: Clone`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing-trait-bounds/issue-69725/issue-69725.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args missing-trait-bounds/issue-69725.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/missing-trait-bounds/issue-69725.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing-trait-bounds/issue-69725" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing-trait-bounds/issue-69725/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `clone` exists for struct `Struct<A>`, but its trait bounds were not satisfied
   |
   |
LL |     let _ = Struct::<A>::new().clone();
   |                                ^^^^^ method cannot be called on `Struct<A>` due to unsatisfied trait bounds
  ::: /checkout/src/test/ui/missing-trait-bounds/auxiliary/issue-69725.rs:2:1
   |
   |
LL | pub struct Struct<A>(A);
   | -------------------- doesn't satisfy `Struct<A>: Clone`
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `A: Clone`
           which is required by `Struct<A>: Clone`
help: consider restricting the type parameter to satisfy the trait bound
   |
LL | fn crash<A>() where A: Clone {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
---
diff of stderr:

12   --> $DIR/specialization-trait-not-implemented.rs:22:29
13    |
14 LL | struct MyStruct;
-    |        |
-    |        |
-    |        method `foo_one` not found for this struct
-    |        doesn't satisfy `MyStruct: Foo`
+    | |      |
+    | |      |
+    | |      method `foo_one` not found for this struct
+    | doesn't satisfy `MyStruct: Foo`
19 ...
20 LL |     println!("{}", MyStruct.foo_one());
21    |                             ^^^^^^^ method cannot be called on `MyStruct` due to unsatisfied trait bounds

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-trait-not-implemented/specialization-trait-not-implemented.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args specialization/defaultimpl/specialization-trait-not-implemented.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/defaultimpl/specialization-trait-not-implemented.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-trait-not-implemented" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-trait-not-implemented/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(specialization)] //~ WARN the feature `specialization` is incomplete
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete
   = help: consider using `min_specialization` instead, which is more stable and complete

error[E0599]: the method `foo_one` exists for struct `MyStruct`, but its trait bounds were not satisfied
   |
   |
LL | struct MyStruct;
   | |      |
   | |      |
   | |      method `foo_one` not found for this struct
   | doesn't satisfy `MyStruct: Foo`
...
LL |     println!("{}", MyStruct.foo_one());
   |                             ^^^^^^^ method cannot be called on `MyStruct` due to unsatisfied trait bounds
   |
note: trait bound `MyStruct: Foo` was not satisfied
   |
   |
LL | default impl<T> Foo for T {
   | ^^^^^^^^^^^^^^^^---^^^^^-
   | unsatisfied trait bound introduced here
note: the following trait must be implemented
  --> /checkout/src/test/ui/specialization/defaultimpl/specialization-trait-not-implemented.rs:7:1
   |
   |
LL | trait Foo {
   | ^^^^^^^^^
   = help: items from traits can only be used if the trait is implemented and in scope
note: `Foo` defines an item `foo_one`, perhaps you need to implement it
   |
LL | trait Foo {
   | ^^^^^^^^^

---
+    | |
+    | doesn't satisfy `Enum: Clone`
+    | doesn't satisfy `Enum: Default`
9 ...
10 LL | enum CloneEnum {
-    |      --------- doesn't satisfy `CloneEnum: Default`
+    | -------------- doesn't satisfy `CloneEnum: Default`
12 ...
13 LL | struct Foo<X, Y> (X, Y);
14    |        --- method `test` not found for this struct
34   --> $DIR/derive-trait-for-method-call.rs:34:15
35    |
36 LL | struct Struct {
-    |        ------
---
+    | |
+    | doesn't satisfy `Struct: Clone`
+    | doesn't satisfy `Struct: Default`
41 ...
42 LL | struct CloneStruct {
-    |        ----------- doesn't satisfy `CloneStruct: Default`
+    | ------------------ doesn't satisfy `CloneStruct: Default`
44 ...
45 LL | struct Foo<X, Y> (X, Y);
46    |        --- method `test` not found for this struct
73   ::: $SRC_DIR/std/src/time.rs:LL:COL
74    |
75 LL | pub struct Instant(time::Instant);
-    |            ------- doesn't satisfy `Instant: Default`
-    |            ------- doesn't satisfy `Instant: Default`
+    | ------------------ doesn't satisfy `Instant: Default`
77    |
78   ::: $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
79    |

80 LL | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
-    |            --- doesn't satisfy `Vec<Enum>: Clone`
+    | ------------------------------------------------------------------------------------------------ doesn't satisfy `Vec<Enum>: Clone`
83    = note: the following trait bounds were not satisfied:
83    = note: the following trait bounds were not satisfied:
84            `Vec<Enum>: Clone`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/derive-trait-for-method-call/derive-trait-for-method-call.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/derive-trait-for-method-call.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/derive-trait-for-method-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/derive-trait-for-method-call" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/derive-trait-for-method-call/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `test` exists for struct `Foo<Enum, CloneEnum>`, but its trait bounds were not satisfied
   |
LL | enum Enum {
   | ---------
   | |
   | |
   | doesn't satisfy `Enum: Clone`
   | doesn't satisfy `Enum: Default`
...
LL | enum CloneEnum {
   | -------------- doesn't satisfy `CloneEnum: Default`
...
LL | struct Foo<X, Y> (X, Y);
   |        --- method `test` not found for this struct
...
LL |     let y = x.test();
   |               ^^^^ method cannot be called on `Foo<Enum, CloneEnum>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
           `Enum: Clone`
           `Enum: Default`
           `CloneEnum: Default`
           `CloneEnum: Default`
note: the following trait must be implemented
  --> /checkout/library/core/src/default.rs:102:1
   |
LL | pub trait Default: Sized {
   | ^^^^^^^^^^^^^^^^^^^^^^^^
help: consider annotating `Enum` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |


error[E0599]: the method `test` exists for struct `Foo<Struct, CloneStruct>`, but its trait bounds were not satisfied
   |
LL | struct Struct {
   | -------------
   | |
   | |
   | doesn't satisfy `Struct: Clone`
   | doesn't satisfy `Struct: Default`
...
LL | struct CloneStruct {
   | ------------------ doesn't satisfy `CloneStruct: Default`
...
LL | struct Foo<X, Y> (X, Y);
   |        --- method `test` not found for this struct
...
LL |     let y = x.test();
   |               ^^^^ method cannot be called on `Foo<Struct, CloneStruct>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
           `Struct: Clone`
           `Struct: Default`
           `Struct: Default`
           `CloneStruct: Default`
help: consider annotating `CloneStruct` with `#[derive(Default)]`
LL | #[derive(Default)]
   |
   |
help: consider annotating `Struct` with `#[derive(Clone, Default)]`
LL | #[derive(Clone, Default)]
   |


error[E0599]: the method `test` exists for struct `Foo<Vec<Enum>, Instant>`, but its trait bounds were not satisfied
   |
   |
LL | struct Foo<X, Y> (X, Y);
   |        --- method `test` not found for this struct
...
LL |     let y = x.test();
   |               ^^^^ method cannot be called on `Foo<Vec<Enum>, Instant>` due to unsatisfied trait bounds
  ::: /checkout/library/std/src/time.rs:154:1
   |
LL | pub struct Instant(time::Instant);
   | ------------------ doesn't satisfy `Instant: Default`
   | ------------------ doesn't satisfy `Instant: Default`
   |
  ::: /checkout/library/alloc/src/vec/mod.rs:400:1
   |
LL | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
   | ------------------------------------------------------------------------------------------------ doesn't satisfy `Vec<Enum>: Clone`
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `Vec<Enum>: Clone`
           `Instant: Default`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/suggestions/import-trait-for-method-call.rs stdout ----
diff of stderr:

19   --> $DIR/import-trait-for-method-call.rs:15:7
20    |
21 LL | trait Bar {}
-    |       --- doesn't satisfy `dyn Bar: AsRef<_>`
+    | --------- doesn't satisfy `dyn Bar: AsRef<_>`
24 LL |     x.as_ref();
24 LL |     x.as_ref();
25    |       ^^^^^^ method cannot be called on `&dyn Bar` due to unsatisfied trait bounds

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/import-trait-for-method-call/import-trait-for-method-call.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/import-trait-for-method-call.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/import-trait-for-method-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/import-trait-for-method-call" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/import-trait-for-method-call/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `finish` found for struct `DefaultHasher` in the current scope
   |
   |
LL |     h.finish() //~ ERROR no method named `finish` found for struct `DefaultHasher`
   |       ^^^^^^ method not found in `DefaultHasher`
  ::: /checkout/library/core/src/hash/mod.rs:338:8
   |
LL |     fn finish(&self) -> u64;
   |        ------ the method is available for `DefaultHasher` here
   |        ------ the method is available for `DefaultHasher` here
   |
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use std::hash::Hasher;
   |

error[E0599]: the method `as_ref` exists for reference `&dyn Bar`, but its trait bounds were not satisfied
   |
LL | trait Bar {}
LL | trait Bar {}
   | --------- doesn't satisfy `dyn Bar: AsRef<_>`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |     x.as_ref(); //~ ERROR the method `as_ref` exists for reference `&dyn Bar`, but its trait bounds
   |       ^^^^^^ method cannot be called on `&dyn Bar` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `dyn Bar: AsRef<_>`
           which is required by `&dyn Bar: AsRef<_>`
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `as_ref`, perhaps you need to implement it:
           candidate #1: `AsRef`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/suggestions/mut-borrow-needed-by-trait.rs stdout ----
diff of stderr:

35   ::: $SRC_DIR/std/src/io/buffered/bufwriter.rs:LL:COL
36    |
37 LL | pub struct BufWriter<W: Write> {
-    |            --------- doesn't satisfy `BufWriter<&dyn std::io::Write>: std::io::Write`
+    | ------------------------------ doesn't satisfy `BufWriter<&dyn std::io::Write>: std::io::Write`
40    = note: the following trait bounds were not satisfied:
41            `&dyn std::io::Write: std::io::Write`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/mut-borrow-needed-by-trait/mut-borrow-needed-by-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/mut-borrow-needed-by-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/mut-borrow-needed-by-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/mut-borrow-needed-by-trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/mut-borrow-needed-by-trait/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `&dyn std::io::Write: std::io::Write` is not satisfied
   |
   |
LL |     let fp = BufWriter::new(fp);
   |              -------------- ^^ the trait `std::io::Write` is not implemented for `&dyn std::io::Write`
   |              required by a bound introduced by this call
   |
   |
   = note: `std::io::Write` is implemented for `&mut dyn std::io::Write`, but not for `&dyn std::io::Write`
note: required by a bound in `BufWriter::<W>::new`
   |
   |
LL | impl<W: Write> BufWriter<W> {
   |         ^^^^^ required by this bound in `BufWriter::<W>::new`
error[E0277]: the trait bound `&dyn std::io::Write: std::io::Write` is not satisfied
  --> /checkout/src/test/ui/suggestions/mut-borrow-needed-by-trait.rs:17:14
   |
   |
LL |     let fp = BufWriter::new(fp);
   |              ^^^^^^^^^^^^^^^^^^ the trait `std::io::Write` is not implemented for `&dyn std::io::Write`
   |
   = note: `std::io::Write` is implemented for `&mut dyn std::io::Write`, but not for `&dyn std::io::Write`
note: required by a bound in `BufWriter`
   |
   |
LL | pub struct BufWriter<W: Write> {
   |                         ^^^^^ required by this bound in `BufWriter`

error[E0599]: the method `write_fmt` exists for struct `BufWriter<&dyn std::io::Write>`, but its trait bounds were not satisfied
   |
   |
LL |     writeln!(fp, "hello world").unwrap(); //~ ERROR the method
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ method cannot be called on `BufWriter<&dyn std::io::Write>` due to unsatisfied trait bounds
  ::: /checkout/library/std/src/io/buffered/bufwriter.rs:70:1
   |
   |
LL | pub struct BufWriter<W: Write> {
   | ------------------------------ doesn't satisfy `BufWriter<&dyn std::io::Write>: std::io::Write`
   = note: the following trait bounds were not satisfied:
           `&dyn std::io::Write: std::io::Write`
           `&dyn std::io::Write: std::io::Write`
           which is required by `BufWriter<&dyn std::io::Write>: std::io::Write`
   = note: this error originates in the macro `writeln` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.
---
diff of stderr:

34   ::: $SRC_DIR/std/src/io/buffered/bufreader.rs:LL:COL
35    |
36 LL | pub struct BufReader<R> {
-    |            --------- doesn't satisfy `BufReader<&T>: BufRead`
+    | ----------------------- doesn't satisfy `BufReader<&T>: BufRead`
39    = note: the following trait bounds were not satisfied:
39    = note: the following trait bounds were not satisfied:
40            `&T: std::io::Read`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-change-mut/suggest-change-mut.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/suggest-change-mut.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-change-mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-change-mut" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-change-mut/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `&T: std::io::Read` is not satisfied
   |
   |
LL |         let mut stream_reader = BufReader::new(&stream);
   |                                 -------------- ^^^^^^^ the trait `std::io::Read` is not implemented for `&T`
   |                                 required by a bound introduced by this call
   |
   |
note: required by a bound in `BufReader::<R>::new`
   |
   |
LL | impl<R: Read> BufReader<R> {
   |         ^^^^ required by this bound in `BufReader::<R>::new`
help: consider removing the leading `&`-reference
   |
LL -         let mut stream_reader = BufReader::new(&stream);
LL +         let mut stream_reader = BufReader::new(stream);
   |
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | fn issue_81421<T: Read + Write>(mut stream: T) where &T: std::io::Read { //~ HELP consider introducing a `where` clause
   |                                                +++++++++++++++++++++++
help: consider changing this borrow's mutability
   |
LL |         let mut stream_reader = BufReader::new(&mut stream);


error[E0599]: the method `read_until` exists for struct `BufReader<&T>`, but its trait bounds were not satisfied
   |
   |
LL |         stream_reader.read_until(b'\n', &mut buffer).expect("Reading into buffer failed");
   |                       ^^^^^^^^^^ method cannot be called on `BufReader<&T>` due to unsatisfied trait bounds
  ::: /checkout/library/std/src/io/buffered/bufreader.rs:49:1
   |
   |
LL | pub struct BufReader<R> {
   | ----------------------- doesn't satisfy `BufReader<&T>: BufRead`
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `&T: std::io::Read`
           which is required by `BufReader<&T>: BufRead`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.
---
diff of stderr:

2   --> $DIR/impl-derived-implicit-sized-bound-2.rs:28:19
3    |
4 LL | struct Victim<'a, T: Perpetrator + ?Sized> {
-    |        |
-    |        method `get` not found for this struct
-    |        method `get` not found for this struct
-    |        doesn't satisfy `Victim<'_, Self>: VictimTrait`
+    | |      |
+    | |      method `get` not found for this struct
+    | |      method `get` not found for this struct
+    | doesn't satisfy `Victim<'_, Self>: VictimTrait`
10 LL |     self.getter().get();
10 LL |     self.getter().get();
11    |                   ^^^ method cannot be called on `Victim<'_, Self>` due to unsatisfied trait bounds

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/impl-derived-implicit-sized-bound-2/impl-derived-implicit-sized-bound-2.stderr
To only update this specific test, also pass `--test-args trait-bounds/impl-derived-implicit-sized-bound-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trait-bounds/impl-derived-implicit-sized-bound-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/impl-derived-implicit-sized-bound-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/impl-derived-implicit-sized-bound-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `get` exists for struct `Victim<'_, Self>`, but its trait bounds were not satisfied
   |
   |
LL | struct Victim<'a, T: Perpetrator + ?Sized> {
   | |      |
   | |      method `get` not found for this struct
   | |      method `get` not found for this struct
   | doesn't satisfy `Victim<'_, Self>: VictimTrait`
LL |     self.getter().get();
LL |     self.getter().get();
   |                   ^^^ method cannot be called on `Victim<'_, Self>` due to unsatisfied trait bounds
   |
note: trait bound `Self: Sized` was not satisfied
   |
   |
LL | impl<'a, T: Perpetrator /*+ ?Sized*/> VictimTrait for Victim<'a, T> {
   |          ^                            -----------     -------------
   |          unsatisfied trait bound introduced here
   |          unsatisfied trait bound introduced here
help: consider relaxing the type parameter's implicit `Sized` bound
   |
LL | impl<'a, T: ?Sized + Perpetrator /*+ ?Sized*/> VictimTrait for Victim<'a, T> {
help: consider restricting the type parameter to satisfy the trait bound
   |
   |
LL | struct Victim<'a, T: Perpetrator + ?Sized> where Self: Sized {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
---
diff of stderr:

2   --> $DIR/impl-derived-implicit-sized-bound.rs:31:19
3    |
4 LL | struct Victim<'a, T: Perpetrator + ?Sized>
-    |        |
-    |        method `get` not found for this struct
-    |        method `get` not found for this struct
-    |        doesn't satisfy `Victim<'_, Self>: VictimTrait`
+    | |      |
+    | |      method `get` not found for this struct
+    | |      method `get` not found for this struct
+    | doesn't satisfy `Victim<'_, Self>: VictimTrait`
10 LL |     self.getter().get();
10 LL |     self.getter().get();
11    |                   ^^^ method cannot be called on `Victim<'_, Self>` due to unsatisfied trait bounds

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/impl-derived-implicit-sized-bound/impl-derived-implicit-sized-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args trait-bounds/impl-derived-implicit-sized-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trait-bounds/impl-derived-implicit-sized-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/impl-derived-implicit-sized-bound" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/impl-derived-implicit-sized-bound/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `get` exists for struct `Victim<'_, Self>`, but its trait bounds were not satisfied
   |
   |
LL | struct Victim<'a, T: Perpetrator + ?Sized>
   | |      |
   | |      method `get` not found for this struct
   | |      method `get` not found for this struct
   | doesn't satisfy `Victim<'_, Self>: VictimTrait`
LL |     self.getter().get();
LL |     self.getter().get();
   |                   ^^^ method cannot be called on `Victim<'_, Self>` due to unsatisfied trait bounds
   |
note: trait bound `Self: Sized` was not satisfied
   |
   |
LL | impl<'a, T: Perpetrator /*+ ?Sized*/> VictimTrait for Victim<'a, T> {
   |          ^                            -----------     -------------
   |          unsatisfied trait bound introduced here
   |          unsatisfied trait bound introduced here
help: consider relaxing the type parameter's implicit `Sized` bound
   |
LL | impl<'a, T: ?Sized + Perpetrator /*+ ?Sized*/> VictimTrait for Victim<'a, T> {
help: consider restricting the type parameter to satisfy the trait bound
   |
   |
LL |   Self: Sized, Self: Sized

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
---
diff of stderr:

2   --> $DIR/union-derive-clone.rs:38:15
3    |
4 LL | union U5<T> {
-    |       |
-    |       method `clone` not found for this union
-    |       method `clone` not found for this union
-    |       doesn't satisfy `U5<CloneNoCopy>: Clone`
+    | |     |
+    | |     method `clone` not found for this union
+    | |     method `clone` not found for this union
+    | doesn't satisfy `U5<CloneNoCopy>: Clone`
9 ...
10 LL | struct CloneNoCopy;
-    |        ----------- doesn't satisfy `CloneNoCopy: Copy`
+    | ------------------ doesn't satisfy `CloneNoCopy: Copy`
12 ...
13 LL |     let w = u.clone();
14    |               ^^^^^ method cannot be called on `U5<CloneNoCopy>` due to unsatisfied trait bounds

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone.mirunsafeck/union-derive-clone.mirunsafeck.stderr
To only update this specific test, also pass `--test-args union/union-derive-clone.rs`


error in revision `mirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-derive-clone.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mirunsafeck" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone.mirunsafeck" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone.mirunsafeck/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `clone` exists for union `U5<CloneNoCopy>`, but its trait bounds were not satisfied
   |
   |
LL | union U5<T> {
   | |     |
   | |     method `clone` not found for this union
   | |     method `clone` not found for this union
   | doesn't satisfy `U5<CloneNoCopy>: Clone`
...
LL | struct CloneNoCopy;
   | ------------------ doesn't satisfy `CloneNoCopy: Copy`
...
LL |     let w = u.clone(); //~ ERROR the method
   |               ^^^^^ method cannot be called on `U5<CloneNoCopy>` due to unsatisfied trait bounds
   |
note: trait bound `CloneNoCopy: Copy` was not satisfied
   |
LL | #[derive(Clone, Copy)]
   |          ^^^^^ unsatisfied trait bound introduced in this `derive` macro
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `CloneNoCopy: Copy`
           which is required by `U5<CloneNoCopy>: Clone`
help: consider annotating `CloneNoCopy` with `#[derive(Clone, Copy)]`
LL | #[derive(Clone, Copy)]
   |


error[E0277]: the trait bound `U1: Copy` is not satisfied
   |
   |
LL | #[derive(Clone)] //~ ERROR the trait bound `U1: Copy` is not satisfied
   |          ^^^^^ the trait `Copy` is not implemented for `U1`
   |
note: required by a bound in `AssertParamIsCopy`
   |
   |
LL | pub struct AssertParamIsCopy<T: Copy + ?Sized> {
   |                                 ^^^^ required by this bound in `AssertParamIsCopy`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `U1` with `#[derive(Copy)]`
   |
LL | #[derive(Copy)]

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0599.
---
diff of stderr:

2   --> $DIR/union-derive-clone.rs:38:15
3    |
4 LL | union U5<T> {
-    |       |
-    |       method `clone` not found for this union
-    |       method `clone` not found for this union
-    |       doesn't satisfy `U5<CloneNoCopy>: Clone`
+    | |     |
+    | |     method `clone` not found for this union
+    | |     method `clone` not found for this union
+    | doesn't satisfy `U5<CloneNoCopy>: Clone`
9 ...
10 LL | struct CloneNoCopy;
-    |        ----------- doesn't satisfy `CloneNoCopy: Copy`
+    | ------------------ doesn't satisfy `CloneNoCopy: Copy`
12 ...
13 LL |     let w = u.clone();
14    |               ^^^^^ method cannot be called on `U5<CloneNoCopy>` due to unsatisfied trait bounds

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone.thirunsafeck/union-derive-clone.thirunsafeck.stderr
To only update this specific test, also pass `--test-args union/union-derive-clone.rs`


error in revision `thirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-derive-clone.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thirunsafeck" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone.thirunsafeck" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "thir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone.thirunsafeck/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `clone` exists for union `U5<CloneNoCopy>`, but its trait bounds were not satisfied
   |
   |
LL | union U5<T> {
   | |     |
   | |     method `clone` not found for this union
   | |     method `clone` not found for this union
   | doesn't satisfy `U5<CloneNoCopy>: Clone`
...
LL | struct CloneNoCopy;
   | ------------------ doesn't satisfy `CloneNoCopy: Copy`
...
LL |     let w = u.clone(); //~ ERROR the method
   |               ^^^^^ method cannot be called on `U5<CloneNoCopy>` due to unsatisfied trait bounds
   |
note: trait bound `CloneNoCopy: Copy` was not satisfied
   |
LL | #[derive(Clone, Copy)]
   |          ^^^^^ unsatisfied trait bound introduced in this `derive` macro
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `CloneNoCopy: Copy`
           which is required by `U5<CloneNoCopy>: Clone`
help: consider annotating `CloneNoCopy` with `#[derive(Clone, Copy)]`
LL | #[derive(Clone, Copy)]
   |


error[E0277]: the trait bound `U1: Copy` is not satisfied
   |
   |
LL | #[derive(Clone)] //~ ERROR the trait bound `U1: Copy` is not satisfied
   |          ^^^^^ the trait `Copy` is not implemented for `U1`
   |
note: required by a bound in `AssertParamIsCopy`
   |
   |
LL | pub struct AssertParamIsCopy<T: Copy + ?Sized> {
   |                                 ^^^^ required by this bound in `AssertParamIsCopy`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `U1` with `#[derive(Copy)]`
   |
LL | #[derive(Copy)]

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0599.
Some errors have detailed explanations: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/unique-object-noncopyable.rs stdout ----
diff of stderr:

1 error[E0599]: the method `clone` exists for struct `Box<dyn Foo>`, but its trait bounds were not satisfied
3    |
- LL | trait Foo {
-    |       ---
-    |       |
-    |       |
-    |       doesn't satisfy `dyn Foo: Clone`
-    |       doesn't satisfy `dyn Foo: Sized`
+ LL |   trait Foo {
+    |   |
+    |   |
+    |   doesn't satisfy `dyn Foo: Clone`
+    |   doesn't satisfy `dyn Foo: Sized`
9 ...
- LL |     let _z = y.clone();
-    |                ^^^^^ method cannot be called on `Box<dyn Foo>` due to unsatisfied trait bounds
+ LL |       let _z = y.clone();
+    |                  ^^^^^ method cannot be called on `Box<dyn Foo>` due to unsatisfied trait bounds
13   ::: $SRC_DIR/alloc/src/boxed.rs:LL:COL
14    |

- LL | pub struct Box<
- LL | pub struct Box<
-    |            --- doesn't satisfy `Box<dyn Foo>: Clone`
+ LL | / pub struct Box<
+ LL | |     T: ?Sized,
+ LL | |     #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
+ LL | | >(Unique<T>, A);
+    | |_- doesn't satisfy `Box<dyn Foo>: Clone`
18    = note: the following trait bounds were not satisfied:
18    = note: the following trait bounds were not satisfied:
19            `dyn Foo: Sized`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unique-object-noncopyable/unique-object-noncopyable.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unique-object-noncopyable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unique-object-noncopyable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unique-object-noncopyable" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unique-object-noncopyable/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `clone` exists for struct `Box<dyn Foo>`, but its trait bounds were not satisfied
   |
LL |   trait Foo {
   |   ---------
   |   |
   |   |
   |   doesn't satisfy `dyn Foo: Clone`
   |   doesn't satisfy `dyn Foo: Sized`
...
LL |       let _z = y.clone(); //~ ERROR the method
   |                  ^^^^^ method cannot be called on `Box<dyn Foo>` due to unsatisfied trait bounds
  ::: /checkout/library/alloc/src/boxed.rs:194:1
   |
LL | / pub struct Box<
LL | / pub struct Box<
LL | |     T: ?Sized,
LL | |     #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
LL | | >(Unique<T>, A);
   | |_- doesn't satisfy `Box<dyn Foo>: Clone`
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `dyn Foo: Sized`
           which is required by `Box<dyn Foo>: Clone`
           `dyn Foo: Clone`
           which is required by `Box<dyn Foo>: Clone`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/unique-pinned-nocopy.rs stdout ----
diff of stderr:

1 error[E0599]: the method `clone` exists for struct `Box<R>`, but its trait bounds were not satisfied
3    |
- LL | struct R {
- LL | struct R {
-    |        - doesn't satisfy `R: Clone`
+ LL |   struct R {
+    |   -------- doesn't satisfy `R: Clone`
6 ...
- LL |     let _j = i.clone();
-    |                ^^^^^ method cannot be called on `Box<R>` due to unsatisfied trait bounds
+ LL |       let _j = i.clone();
+    |                  ^^^^^ method cannot be called on `Box<R>` due to unsatisfied trait bounds
10   ::: $SRC_DIR/alloc/src/boxed.rs:LL:COL
11    |

- LL | pub struct Box<
- LL | pub struct Box<
-    |            --- doesn't satisfy `Box<R>: Clone`
+ LL | / pub struct Box<
+ LL | |     T: ?Sized,
+ LL | |     #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
+ LL | | >(Unique<T>, A);
+    | |_- doesn't satisfy `Box<R>: Clone`
15    = note: the following trait bounds were not satisfied:
16            `R: Clone`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unique-pinned-nocopy/unique-pinned-nocopy.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unique-pinned-nocopy.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unique-pinned-nocopy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unique-pinned-nocopy" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unique-pinned-nocopy/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `clone` exists for struct `Box<R>`, but its trait bounds were not satisfied
   |
LL |   struct R {
   |   -------- doesn't satisfy `R: Clone`
...
...
LL |       let _j = i.clone(); //~ ERROR the method
   |                  ^^^^^ method cannot be called on `Box<R>` due to unsatisfied trait bounds
  ::: /checkout/library/alloc/src/boxed.rs:194:1
   |
LL | / pub struct Box<
LL | / pub struct Box<
LL | |     T: ?Sized,
LL | |     #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
LL | | >(Unique<T>, A);
   | |_- doesn't satisfy `Box<R>: Clone`
   = note: the following trait bounds were not satisfied:
           `R: Clone`
           `R: Clone`
           which is required by `Box<R>: Clone`
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `clone`, perhaps you need to implement it:
           candidate #1: `Clone`
help: consider annotating `R` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |

error: aborting due to previous error
