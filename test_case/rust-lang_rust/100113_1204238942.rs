plain
........................................................................................ 88/13312
......................................................................iiiiiiiiiiiiii.... 176/13312
...............i.i.................ii...i............................................... 264/13312
........................................................................................ 352/13312
.....................F.................F.F....................F................F........ 440/13312
...........................F............................................................ 528/13312
......................................F................................................. 704/13312
...........................................................i............................ 792/13312
.......................................................i................................ 880/13312
........................................................................................ 968/13312
---
......................................F................................................. 11176/13312
........................................................................................ 11264/13312
........................................................................................ 11352/13312
........................................................................................ 11440/13312
..........................................................FF............................ 11528/13312
.........F................F........................F...................F................ 11616/13312
........................................F............................................... 11704/13312
......................................................................................F. 11880/13312
........................................................................................ 11968/13312
........................................................................................ 12056/13312
........................................................................................ 12144/13312
........................................................................................ 12144/13312
........................................F....................F.......................... 12232/13312
........................................................................................ 12408/13312
....................................................i................................... 12496/13312
.............F.......................................................................... 12584/13312
........................................................................................ 12672/13312
---

6    |
7 help: consider further restricting `Self`
8    |
- LL |     fn uhoh<U:Get>(&self, foo: U, bar: <Self as Get>::Value) where Self: Get {}
-    |                                                              +++++++++++++++
+ LL |     fn uhoh<U:Get>(&self, foo: U, bar: <Self as Get>::Value), Self: Get {}
11 
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-for-unimpl-trait/associated-types-for-unimpl-trait.stderr
diff of fixed:

7 }
8 
9 trait Other {
-     fn uhoh<U:Get>(&self, foo: U, bar: <Self as Get>::Value) where Self: Get {}
+     fn uhoh<U:Get>(&self, foo: U, bar: <Self as Get>::Value), Self: Get {}
11     //~^ ERROR the trait bound `Self: Get` is not satisfied
13 


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-for-unimpl-trait/associated-types-for-unimpl-trait.fixed
To only update this specific test, also pass `--test-args associated-types/associated-types-for-unimpl-trait.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-for-unimpl-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-for-unimpl-trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-for-unimpl-trait/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `Self: Get` is not satisfied
   |
   |
LL |     fn uhoh<U:Get>(&self, foo: U, bar: <Self as Get>::Value) {}
   |                                        ^^^^^^^^^^^^^^^^^^^^ the trait `Get` is not implemented for `Self`
help: consider further restricting `Self`
   |
   |
LL |     fn uhoh<U:Get>(&self, foo: U, bar: <Self as Get>::Value), Self: Get {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
---

6    |
7 help: consider further restricting `Self`
8    |
- LL |     fn uhoh<U:Get>(&self, foo: U, bar: <Self as Get>::Value) where Self: Get {}
-    |                                                              +++++++++++++++
+ LL |     fn uhoh<U:Get>(&self, foo: U, bar: <Self as Get>::Value), Self: Get {}
11 
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-no-suitable-supertrait-2/associated-types-no-suitable-supertrait-2.stderr
To only update this specific test, also pass `--test-args associated-types/associated-types-no-suitable-supertrait-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-no-suitable-supertrait-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-no-suitable-supertrait-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-no-suitable-supertrait-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `Self: Get` is not satisfied
   |
   |
LL |     fn uhoh<U:Get>(&self, foo: U, bar: <Self as Get>::Value) {}
   |                                        ^^^^^^^^^^^^^^^^^^^^ the trait `Get` is not implemented for `Self`
help: consider further restricting `Self`
   |
   |
LL |     fn uhoh<U:Get>(&self, foo: U, bar: <Self as Get>::Value), Self: Get {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
---

12    |
13 help: consider further restricting `Self`
14    |
- LL |     fn uhoh<U:Get>(&self, foo: U, bar: <Self as Get>::Value) where Self: Get {}
-    |                                                              +++++++++++++++
+ LL |     fn uhoh<U:Get>(&self, foo: U, bar: <Self as Get>::Value), Self: Get {}
17 
18 error: aborting due to 2 previous errors
19 

---
To only update this specific test, also pass `--test-args associated-types/associated-types-no-suitable-supertrait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-no-suitable-supertrait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-no-suitable-supertrait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-no-suitable-supertrait/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `(T, U): Get` is not satisfied
   |
   |
LL |     fn uhoh<U:Get>(&self, foo: U, bar: <(T, U) as Get>::Value) {}
   |                                        ^^^^^^^^^^^^^^^^^^^^^^ the trait `Get` is not implemented for `(T, U)`

error[E0277]: the trait bound `Self: Get` is not satisfied
   |
   |
LL |     fn uhoh<U:Get>(&self, foo: U, bar: <Self as Get>::Value) {}
   |                                        ^^^^^^^^^^^^^^^^^^^^ the trait `Get` is not implemented for `Self`
help: consider further restricting `Self`
   |
   |
LL |     fn uhoh<U:Get>(&self, foo: U, bar: <Self as Get>::Value), Self: Get {}

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
---

6    |
7 help: consider further restricting `Self`
8    |
- LL |     fn okay<U:Get>(&self, foo: U, bar: <Self as Get>::Value) where Self: Get;
-    |                                                              +++++++++++++++
+ LL |     fn okay<U:Get>(&self, foo: U, bar: <Self as Get>::Value), Self: Get;
11 
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-projection-to-unrelated-trait-in-method-without-default/associated-types-projection-to-unrelated-trait-in-method-without-default.stderr
diff of fixed:

7 }
8 
9 trait Other {
-     fn okay<U:Get>(&self, foo: U, bar: <Self as Get>::Value) where Self: Get;
+     fn okay<U:Get>(&self, foo: U, bar: <Self as Get>::Value), Self: Get;
11     //~^ ERROR E0277
13 


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-projection-to-unrelated-trait-in-method-without-default/associated-types-projection-to-unrelated-trait-in-method-without-default.fixed
To only update this specific test, also pass `--test-args associated-types/associated-types-projection-to-unrelated-trait-in-method-without-default.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-projection-to-unrelated-trait-in-method-without-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-projection-to-unrelated-trait-in-method-without-default" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-projection-to-unrelated-trait-in-method-without-default/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `Self: Get` is not satisfied
   |
   |
LL |     fn okay<U:Get>(&self, foo: U, bar: <Self as Get>::Value);
   |                                        ^^^^^^^^^^^^^^^^^^^^ the trait `Get` is not implemented for `Self`
help: consider further restricting `Self`
   |
   |
LL |     fn okay<U:Get>(&self, foo: U, bar: <Self as Get>::Value), Self: Get;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
---

9    = help: unsized locals are gated as an unstable feature
10 help: consider further restricting the associated type
11    |
- LL | fn foo<T:Get>(t: T) where <T as Get>::Value: Sized {
-    |                     ++++++++++++++++++++++++++++++
+ LL | fn foo<T:Get>(t: T), <T as Get>::Value: Sized {
14 
15 error: aborting due to previous error
16 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-unsized/associated-types-unsized.stderr
diff of fixed:

6     fn get(&self) -> <Self as Get>::Value;
8 
8 
- fn foo<T:Get>(t: T) where <T as Get>::Value: Sized {
+ fn foo<T:Get>(t: T), <T as Get>::Value: Sized {
10     let x = t.get(); //~ ERROR the size for values of type
12 


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-unsized/associated-types-unsized.fixed
To only update this specific test, also pass `--test-args associated-types/associated-types-unsized.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-unsized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-unsized" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-unsized/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the size for values of type `<T as Get>::Value` cannot be known at compilation time
   |
   |
LL |     let x = t.get(); //~ ERROR the size for values of type
   |
   |
   = help: the trait `Sized` is not implemented for `<T as Get>::Value`
   = help: unsized locals are gated as an unstable feature
help: consider further restricting the associated type
   |
   |
LL | fn foo<T:Get>(t: T), <T as Get>::Value: Sized {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
---

108    |               ^^^^^ required by this bound in `Foo25::Bar`
109 help: consider further restricting the associated type
110    |
- LL | trait Foo25<T: Clone> where <Self as Foo25<T>>::Baz: Clone {
-    |                       ++++++++++++++++++++++++++++++++++++
+ LL | trait Foo25<T: Clone>, <Self as Foo25<T>>::Baz: Clone {
113 
113 
114 error[E0277]: the trait bound `T: Clone` is not satisfied


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-suitability/defaults-suitability.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-suitability/defaults-suitability.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/defaults-suitability.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-suitability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-suitability" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-suitability/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `NotClone: Clone` is not satisfied
   |
   |
LL |     type Ty: Clone = NotClone;
   |                      ^^^^^^^^ the trait `Clone` is not implemented for `NotClone`
note: required by a bound in `Tr::Ty`
  --> /checkout/src/test/ui/associated-types/defaults-suitability.rs:13:14
   |
   |
LL |     type Ty: Clone = NotClone;
   |              ^^^^^ required by this bound in `Tr::Ty`
help: consider annotating `NotClone` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |


error[E0277]: the trait bound `NotClone: Clone` is not satisfied
   |
   |
LL |     type Ty = NotClone;
   |               ^^^^^^^^ the trait `Clone` is not implemented for `NotClone`
note: required by a bound in `Tr2::Ty`
  --> /checkout/src/test/ui/associated-types/defaults-suitability.rs:20:15
   |
LL |     Self::Ty: Clone,
LL |     Self::Ty: Clone,
   |               ^^^^^ required by this bound in `Tr2::Ty`
LL | {
LL |     type Ty = NotClone;
   |          -- required by a bound in this
help: consider annotating `NotClone` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |


error[E0277]: the trait bound `T: Clone` is not satisfied
   |
   |
LL |     type Bar: Clone = Vec<T>;
   |                       ^^^^^^ the trait `Clone` is not implemented for `T`
   = note: required because of the requirements on the impl of `Clone` for `Vec<T>`
note: required by a bound in `Foo::Bar`
  --> /checkout/src/test/ui/associated-types/defaults-suitability.rs:28:15
   |
   |
LL |     type Bar: Clone = Vec<T>;
   |               ^^^^^ required by this bound in `Foo::Bar`
help: consider restricting type parameter `T`
   |
LL | trait Foo<T: std::clone::Clone> {


error[E0277]: the trait bound `(): Foo<Self>` is not satisfied
   |
   |
LL |     type Assoc: Foo<Self> = ();
   |                             ^^ the trait `Foo<Self>` is not implemented for `()`
   |
note: required by a bound in `Bar::Assoc`
   |
   |
LL |     type Assoc: Foo<Self> = ();
   |                 ^^^^^^^^^ required by this bound in `Bar::Assoc`

error[E0277]: the trait bound `NotClone: IsU8<NotClone>` is not satisfied
   |
   |
LL |     type Assoc = NotClone;
   |                  ^^^^^^^^ the trait `IsU8<NotClone>` is not implemented for `NotClone`
note: required by a bound in `D::Assoc`
  --> /checkout/src/test/ui/associated-types/defaults-suitability.rs:53:18
   |
   |
LL |     Self::Assoc: IsU8<Self::Assoc>,
   |                  ^^^^^^^^^^^^^^^^^ required by this bound in `D::Assoc`
...
LL |     type Assoc = NotClone;
   |          ----- required by a bound in this

error[E0277]: the trait bound `<Self as Foo2<T>>::Baz: Clone` is not satisfied
   |
   |
LL |     type Bar: Clone = Vec<Self::Baz>;
   |                       ^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `<Self as Foo2<T>>::Baz`
   |
   = note: required because of the requirements on the impl of `Clone` for `Vec<<Self as Foo2<T>>::Baz>`
note: required by a bound in `Foo2::Bar`
   |
   |
LL |     type Bar: Clone = Vec<Self::Baz>;
   |               ^^^^^ required by this bound in `Foo2::Bar`
   |
   |
LL | trait Foo2<T> where <Self as Foo2<T>>::Baz: Clone {


error[E0277]: the trait bound `<Self as Foo25<T>>::Baz: Clone` is not satisfied
   |
   |
LL |     type Bar: Clone = Vec<Self::Baz>;
   |                       ^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `<Self as Foo25<T>>::Baz`
   |
   = note: required because of the requirements on the impl of `Clone` for `Vec<<Self as Foo25<T>>::Baz>`
note: required by a bound in `Foo25::Bar`
   |
   |
LL |     type Bar: Clone = Vec<Self::Baz>;
   |               ^^^^^ required by this bound in `Foo25::Bar`
   |
   |
LL | trait Foo25<T: Clone>, <Self as Foo25<T>>::Baz: Clone {


error[E0277]: the trait bound `T: Clone` is not satisfied
   |
LL |     type Baz = T;
   |                ^ the trait `Clone` is not implemented for `T`
   |
   |
note: required by a bound in `Foo3::Baz`
  --> /checkout/src/test/ui/associated-types/defaults-suitability.rs:84:16
   |
LL |     Self::Baz: Clone,
   |                ^^^^^ required by this bound in `Foo3::Baz`
LL |     type Baz = T;
   |          --- required by a bound in this
help: consider further restricting type parameter `T`
   |
   |
LL |     Self::Baz: Clone, T: std::clone::Clone

error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/async-await/issue-70818.rs stdout ----
diff of stderr:

9    |
10 LL |     async { (ty, ty1) }
11    |                  ^^^ has type `U` which is not `Send`
- help: consider restricting type parameter `U`
+ help: consider further restricting type parameter `U`
13    |
- LL | fn foo<T: Send, U: std::marker::Send>(ty: T, ty1: U) -> impl Future<Output = (T, U)> + Send {
-    |                  +++++++++++++++++++
+ LL | fn foo<T: Send, U>(ty: T, ty1: U) -> impl Future<Output = (T, U)> + Send, U: std::marker::Send {
16 
17 error: aborting due to previous error
18 

---
To only update this specific test, also pass `--test-args async-await/issue-70818.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-70818.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-70818" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-70818/auxiliary"
stdout: none
--- stderr -------------------------------
error: future cannot be sent between threads safely
   |
   |
LL | fn foo<T: Send, U>(ty: T, ty1: U) -> impl Future<Output = (T, U)> + Send {
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ future created by async block is not `Send`
   |
note: captured value is not `Send`
   |
   |
LL |     async { (ty, ty1) }
   |                  ^^^ has type `U` which is not `Send`
help: consider further restricting type parameter `U`
   |
LL | fn foo<T: Send, U>(ty: T, ty1: U) -> impl Future<Output = (T, U)> + Send, U: std::marker::Send {

error: aborting due to previous error
------------------------------------------

---
31    |
- help: consider restricting type parameter `B`
+ help: consider further restricting type parameter `B`
33    |
- LL | fn add<A: Add<B, Output=()>, B: Copy>(lhs: A, rhs: B) {
-    |                               ++++++
+ LL | fn add<A: Add<B, Output=()>, B>(lhs: A, rhs: B), B: Copy {
36 
37 error[E0382]: use of moved value: `lhs`
38   --> $DIR/binop-consume-args.rs:13:10


65 LL |     drop(rhs);
66    |          ^^^ value used here after move
67    |
- help: consider restricting type parameter `B`
+ help: consider further restricting type parameter `B`
69    |
- LL | fn sub<A: Sub<B, Output=()>, B: Copy>(lhs: A, rhs: B) {
-    |                               ++++++
+ LL | fn sub<A: Sub<B, Output=()>, B>(lhs: A, rhs: B), B: Copy {
72 
73 error[E0382]: use of moved value: `lhs`
74   --> $DIR/binop-consume-args.rs:19:10


101 LL |     drop(rhs);
102    |          ^^^ value used here after move
103    |
- help: consider restricting type parameter `B`
+ help: consider further restricting type parameter `B`
105    |
- LL | fn mul<A: Mul<B, Output=()>, B: Copy>(lhs: A, rhs: B) {
-    |                               ++++++
+ LL | fn mul<A: Mul<B, Output=()>, B>(lhs: A, rhs: B), B: Copy {
108 
109 error[E0382]: use of moved value: `lhs`
110   --> $DIR/binop-consume-args.rs:25:10


137 LL |     drop(rhs);
138    |          ^^^ value used here after move
139    |
- help: consider restricting type parameter `B`
+ help: consider further restricting type parameter `B`
141    |
- LL | fn div<A: Div<B, Output=()>, B: Copy>(lhs: A, rhs: B) {
-    |                               ++++++
+ LL | fn div<A: Div<B, Output=()>, B>(lhs: A, rhs: B), B: Copy {
144 
145 error[E0382]: use of moved value: `lhs`
146   --> $DIR/binop-consume-args.rs:31:10


173 LL |     drop(rhs);
174    |          ^^^ value used here after move
175    |
- help: consider restricting type parameter `B`
+ help: consider further restricting type parameter `B`
177    |
- LL | fn rem<A: Rem<B, Output=()>, B: Copy>(lhs: A, rhs: B) {
-    |                               ++++++
+ LL | fn rem<A: Rem<B, Output=()>, B>(lhs: A, rhs: B), B: Copy {
180 
181 error[E0382]: use of moved value: `lhs`
182   --> $DIR/binop-consume-args.rs:37:10


209 LL |     drop(rhs);
210    |          ^^^ value used here after move
211    |
- help: consider restricting type parameter `B`
+ help: consider further restricting type parameter `B`
213    |
- LL | fn bitand<A: BitAnd<B, Output=()>, B: Copy>(lhs: A, rhs: B) {
-    |                                     ++++++
+ LL | fn bitand<A: BitAnd<B, Output=()>, B>(lhs: A, rhs: B), B: Copy {
216 
217 error[E0382]: use of moved value: `lhs`
218   --> $DIR/binop-consume-args.rs:43:10


245 LL |     drop(rhs);
246    |          ^^^ value used here after move
247    |
- help: consider restricting type parameter `B`
+ help: consider further restricting type parameter `B`
249    |
- LL | fn bitor<A: BitOr<B, Output=()>, B: Copy>(lhs: A, rhs: B) {
-    |                                   ++++++
+ LL | fn bitor<A: BitOr<B, Output=()>, B>(lhs: A, rhs: B), B: Copy {
252 
253 error[E0382]: use of moved value: `lhs`
254   --> $DIR/binop-consume-args.rs:49:10


281 LL |     drop(rhs);
282    |          ^^^ value used here after move
283    |
- help: consider restricting type parameter `B`
+ help: consider further restricting type parameter `B`
285    |
- LL | fn bitxor<A: BitXor<B, Output=()>, B: Copy>(lhs: A, rhs: B) {
-    |                                     ++++++
+ LL | fn bitxor<A: BitXor<B, Output=()>, B>(lhs: A, rhs: B), B: Copy {
288 
289 error[E0382]: use of moved value: `lhs`
290   --> $DIR/binop-consume-args.rs:55:10


317 LL |     drop(rhs);
318    |          ^^^ value used here after move
319    |
- help: consider restricting type parameter `B`
+ help: consider further restricting type parameter `B`
321    |
- LL | fn shl<A: Shl<B, Output=()>, B: Copy>(lhs: A, rhs: B) {
-    |                               ++++++
+ LL | fn shl<A: Shl<B, Output=()>, B>(lhs: A, rhs: B), B: Copy {
324 
325 error[E0382]: use of moved value: `lhs`
326   --> $DIR/binop-consume-args.rs:61:10


353 LL |     drop(rhs);
354    |          ^^^ value used here after move
355    |
- help: consider restricting type parameter `B`
+ help: consider further restricting type parameter `B`
357    |
- LL | fn shr<A: Shr<B, Output=()>, B: Copy>(lhs: A, rhs: B) {
-    |                               ++++++
+ LL | fn shr<A: Shr<B, Output=()>, B>(lhs: A, rhs: B), B: Copy {
360 
361 error: aborting due to 20 previous errors
362 

---
To only update this specific test, also pass `--test-args binop/binop-consume-args.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/binop-consume-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-consume-args" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-consume-args/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `lhs`
   |
   |
LL | fn add<A: Add<B, Output=()>, B>(lhs: A, rhs: B) {
   |                                 --- move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
LL |     lhs + rhs;
   |     --------- `lhs` moved due to usage in operator
LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
   |          ^^^ value used here after move
note: calling this operator moves the left-hand side
  --> /checkout/library/core/src/ops/arith.rs:114:12
   |
   |
LL |     fn add(self, rhs: Rhs) -> Self::Output;
help: consider further restricting this bound
   |
   |
LL | fn add<A: Add<B, Output=()> + Copy, B>(lhs: A, rhs: B) {

error[E0382]: use of moved value: `rhs`
  --> /checkout/src/test/ui/binop/binop-consume-args.rs:8:10
   |
   |
LL | fn add<A: Add<B, Output=()>, B>(lhs: A, rhs: B) {
   |                                         --- move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
LL |     lhs + rhs;
   |           --- value moved here
LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
   |          ^^^ value used here after move
help: consider further restricting type parameter `B`
   |
   |
LL | fn add<A: Add<B, Output=()>, B>(lhs: A, rhs: B), B: Copy {

error[E0382]: use of moved value: `lhs`
  --> /checkout/src/test/ui/binop/binop-consume-args.rs:13:10
   |
   |
LL | fn sub<A: Sub<B, Output=()>, B>(lhs: A, rhs: B) {
   |                                 --- move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
LL |     lhs - rhs;
   |     --------- `lhs` moved due to usage in operator
LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
   |          ^^^ value used here after move
note: calling this operator moves the left-hand side
  --> /checkout/library/core/src/ops/arith.rs:221:12
   |
   |
LL |     fn sub(self, rhs: Rhs) -> Self::Output;
help: consider further restricting this bound
   |
   |
LL | fn sub<A: Sub<B, Output=()> + Copy, B>(lhs: A, rhs: B) {

error[E0382]: use of moved value: `rhs`
  --> /checkout/src/test/ui/binop/binop-consume-args.rs:14:10
   |
   |
LL | fn sub<A: Sub<B, Output=()>, B>(lhs: A, rhs: B) {
   |                                         --- move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
LL |     lhs - rhs;
   |           --- value moved here
LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
   |          ^^^ value used here after move
help: consider further restricting type parameter `B`
   |
   |
LL | fn sub<A: Sub<B, Output=()>, B>(lhs: A, rhs: B), B: Copy {

error[E0382]: use of moved value: `lhs`
  --> /checkout/src/test/ui/binop/binop-consume-args.rs:19:10
   |
   |
LL | fn mul<A: Mul<B, Output=()>, B>(lhs: A, rhs: B) {
   |                                 --- move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
LL |     lhs * rhs;
   |     --------- `lhs` moved due to usage in operator
LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
   |          ^^^ value used here after move
note: calling this operator moves the left-hand side
  --> /checkout/library/core/src/ops/arith.rs:350:12
   |
   |
LL |     fn mul(self, rhs: Rhs) -> Self::Output;
help: consider further restricting this bound
   |
   |
LL | fn mul<A: Mul<B, Output=()> + Copy, B>(lhs: A, rhs: B) {

error[E0382]: use of moved value: `rhs`
  --> /checkout/src/test/ui/binop/binop-consume-args.rs:20:10
   |
   |
LL | fn mul<A: Mul<B, Output=()>, B>(lhs: A, rhs: B) {
   |                                         --- move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
LL |     lhs * rhs;
   |           --- value moved here
LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
   |          ^^^ value used here after move
help: consider further restricting type parameter `B`
   |
   |
LL | fn mul<A: Mul<B, Output=()>, B>(lhs: A, rhs: B), B: Copy {

error[E0382]: use of moved value: `lhs`
  --> /checkout/src/test/ui/binop/binop-consume-args.rs:25:10
   |
   |
LL | fn div<A: Div<B, Output=()>, B>(lhs: A, rhs: B) {
   |                                 --- move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
LL |     lhs / rhs;
   |     --------- `lhs` moved due to usage in operator
LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
   |          ^^^ value used here after move
note: calling this operator moves the left-hand side
  --> /checkout/library/core/src/ops/arith.rs:483:12
   |
   |
LL |     fn div(self, rhs: Rhs) -> Self::Output;
help: consider further restricting this bound
   |
   |
LL | fn div<A: Div<B, Output=()> + Copy, B>(lhs: A, rhs: B) {

error[E0382]: use of moved value: `rhs`
  --> /checkout/src/test/ui/binop/binop-consume-args.rs:26:10
   |
   |
LL | fn div<A: Div<B, Output=()>, B>(lhs: A, rhs: B) {
   |                                         --- move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
LL |     lhs / rhs;
   |           --- value moved here
LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
   |          ^^^ value used here after move
help: consider further restricting type parameter `B`
   |
   |
LL | fn div<A: Div<B, Output=()>, B>(lhs: A, rhs: B), B: Copy {

error[E0382]: use of moved value: `lhs`
  --> /checkout/src/test/ui/binop/binop-consume-args.rs:31:10
   |
   |
LL | fn rem<A: Rem<B, Output=()>, B>(lhs: A, rhs: B) {
   |                                 --- move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
LL |     lhs % rhs;
   |     --------- `lhs` moved due to usage in operator
LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
   |          ^^^ value used here after move
note: calling this operator moves the left-hand side
  --> /checkout/library/core/src/ops/arith.rs:585:12
   |
   |
LL |     fn rem(self, rhs: Rhs) -> Self::Output;
help: consider further restricting this bound
   |
   |
LL | fn rem<A: Rem<B, Output=()> + Copy, B>(lhs: A, rhs: B) {

error[E0382]: use of moved value: `rhs`
  --> /checkout/src/test/ui/binop/binop-consume-args.rs:32:10
   |
   |
LL | fn rem<A: Rem<B, Output=()>, B>(lhs: A, rhs: B) {
   |                                         --- move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
LL |     lhs % rhs;
   |           --- value moved here
LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
   |          ^^^ value used here after move
help: consider further restricting type parameter `B`
   |
   |
LL | fn rem<A: Rem<B, Output=()>, B>(lhs: A, rhs: B), B: Copy {

error[E0382]: use of moved value: `lhs`
  --> /checkout/src/test/ui/binop/binop-consume-args.rs:37:10
   |
   |
LL | fn bitand<A: BitAnd<B, Output=()>, B>(lhs: A, rhs: B) {
   |                                       --- move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
LL |     lhs & rhs;
   |     --------- `lhs` moved due to usage in operator
LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
   |          ^^^ value used here after move
note: calling this operator moves the left-hand side
  --> /checkout/library/core/src/ops/bit.rs:163:15
   |
   |
LL |     fn bitand(self, rhs: Rhs) -> Self::Output;
help: consider further restricting this bound
   |
   |
LL | fn bitand<A: BitAnd<B, Output=()> + Copy, B>(lhs: A, rhs: B) {

error[E0382]: use of moved value: `rhs`
  --> /checkout/src/test/ui/binop/binop-consume-args.rs:38:10
   |
   |
LL | fn bitand<A: BitAnd<B, Output=()>, B>(lhs: A, rhs: B) {
   |                                               --- move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
LL |     lhs & rhs;
   |           --- value moved here
LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
   |          ^^^ value used here after move
help: consider further restricting type parameter `B`
   |
   |
LL | fn bitand<A: BitAnd<B, Output=()>, B>(lhs: A, rhs: B), B: Copy {

error[E0382]: use of moved value: `lhs`
  --> /checkout/src/test/ui/binop/binop-consume-args.rs:43:10
   |
   |
LL | fn bitor<A: BitOr<B, Output=()>, B>(lhs: A, rhs: B) {
   |                                     --- move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
LL |     lhs | rhs;
   |     --------- `lhs` moved due to usage in operator
LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
   |          ^^^ value used here after move
note: calling this operator moves the left-hand side
  --> /checkout/library/core/src/ops/bit.rs:264:14
   |
   |
LL |     fn bitor(self, rhs: Rhs) -> Self::Output;
help: consider further restricting this bound
   |
   |
LL | fn bitor<A: BitOr<B, Output=()> + Copy, B>(lhs: A, rhs: B) {

error[E0382]: use of moved value: `rhs`
  --> /checkout/src/test/ui/binop/binop-consume-args.rs:44:10
   |
   |
LL | fn bitor<A: BitOr<B, Output=()>, B>(lhs: A, rhs: B) {
   |                                             --- move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
LL |     lhs | rhs;
   |           --- value moved here
LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
   |          ^^^ value used here after move
help: consider further restricting type parameter `B`
   |
   |
LL | fn bitor<A: BitOr<B, Output=()>, B>(lhs: A, rhs: B), B: Copy {

error[E0382]: use of moved value: `lhs`
  --> /checkout/src/test/ui/binop/binop-consume-args.rs:49:10
   |
   |
LL | fn bitxor<A: BitXor<B, Output=()>, B>(lhs: A, rhs: B) {
   |                                       --- move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
LL |     lhs ^ rhs;
   |     --------- `lhs` moved due to usage in operator
LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
   |          ^^^ value used here after move
note: calling this operator moves the left-hand side
  --> /checkout/library/core/src/ops/bit.rs:365:15
   |
   |
LL |     fn bitxor(self, rhs: Rhs) -> Self::Output;
help: consider further restricting this bound
   |
   |
LL | fn bitxor<A: BitXor<B, Output=()> + Copy, B>(lhs: A, rhs: B) {

error[E0382]: use of moved value: `rhs`
  --> /checkout/src/test/ui/binop/binop-consume-args.rs:50:10
   |
   |
LL | fn bitxor<A: BitXor<B, Output=()>, B>(lhs: A, rhs: B) {
   |                                               --- move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
LL |     lhs ^ rhs;
   |           --- value moved here
LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
   |          ^^^ value used here after move
help: consider further restricting type parameter `B`
   |
   |
LL | fn bitxor<A: BitXor<B, Output=()>, B>(lhs: A, rhs: B), B: Copy {

error[E0382]: use of moved value: `lhs`
  --> /checkout/src/test/ui/binop/binop-consume-args.rs:55:10
   |
   |
LL | fn shl<A: Shl<B, Output=()>, B>(lhs: A, rhs: B) {
   |                                 --- move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
LL |     lhs << rhs;
   |     ---------- `lhs` moved due to usage in operator
LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
   |          ^^^ value used here after move
note: calling this operator moves the left-hand side
  --> /checkout/library/core/src/ops/bit.rs:463:12
   |
   |
LL |     fn shl(self, rhs: Rhs) -> Self::Output;
help: consider further restricting this bound
   |
   |
LL | fn shl<A: Shl<B, Output=()> + Copy, B>(lhs: A, rhs: B) {

error[E0382]: use of moved value: `rhs`
  --> /checkout/src/test/ui/binop/binop-consume-args.rs:56:10
   |
   |
LL | fn shl<A: Shl<B, Output=()>, B>(lhs: A, rhs: B) {
   |                                         --- move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
LL |     lhs << rhs;
   |            --- value moved here
LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
   |          ^^^ value used here after move
help: consider further restricting type parameter `B`
   |
   |
LL | fn shl<A: Shl<B, Output=()>, B>(lhs: A, rhs: B), B: Copy {

error[E0382]: use of moved value: `lhs`
  --> /checkout/src/test/ui/binop/binop-consume-args.rs:61:10
   |
   |
LL | fn shr<A: Shr<B, Output=()>, B>(lhs: A, rhs: B) {
   |                                 --- move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
LL |     lhs >> rhs;
   |     ---------- `lhs` moved due to usage in operator
LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
   |          ^^^ value used here after move
note: calling this operator moves the left-hand side
  --> /checkout/library/core/src/ops/bit.rs:582:12
   |
   |
LL |     fn shr(self, rhs: Rhs) -> Self::Output;
help: consider further restricting this bound
   |
   |
LL | fn shr<A: Shr<B, Output=()> + Copy, B>(lhs: A, rhs: B) {

error[E0382]: use of moved value: `rhs`
  --> /checkout/src/test/ui/binop/binop-consume-args.rs:62:10
   |
   |
LL | fn shr<A: Shr<B, Output=()>, B>(lhs: A, rhs: B) {
   |                                         --- move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
LL |     lhs >> rhs;
   |            --- value moved here
LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
   |          ^^^ value used here after move
help: consider further restricting type parameter `B`
   |
   |
LL | fn shr<A: Shr<B, Output=()>, B>(lhs: A, rhs: B), B: Copy {

error: aborting due to 20 previous errors

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] src/test/ui/partialeq_help.rs stdout ----
diff of stderr:

7    = help: the trait `PartialEq<T>` is not implemented for `&T`
8 help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
9    |
- LL | fn foo<T: PartialEq>(a: &T, b: T) where &T: PartialEq<T> {
-    |                                   ++++++++++++++++++++++
+ LL | fn foo<T: PartialEq>(a: &T, b: T), &T: PartialEq<T> {
12 
12 
13 error[E0277]: can't compare `&T` with `T`


19    = help: the trait `PartialEq<T>` is not implemented for `&T`
20 help: consider extending the `where` clause, but there might be an alternative better way to express this requirement
21    |
- LL | fn foo2<T: PartialEq>(a: &T, b: T) where &T: PartialEq<T> {
-    |                                          ++++++++++++++++
+ LL | fn foo2<T: PartialEq>(a: &T, b: T) where, &T: PartialEq<T> {
24 
25 error: aborting due to 2 previous errors
26 

---
To only update this specific test, also pass `--test-args partialeq_help.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/partialeq_help.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/partialeq_help" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/partialeq_help/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: can't compare `&T` with `T`
   |
   |
LL |     a == b; //~ ERROR E0277
   |       ^^ no implementation for `&T == T`
   |
   = help: the trait `PartialEq<T>` is not implemented for `&T`
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | fn foo<T: PartialEq>(a: &T, b: T), &T: PartialEq<T> {


error[E0277]: can't compare `&T` with `T`
   |
   |
LL |     a == b; //~ ERROR E0277
   |       ^^ no implementation for `&T == T`
   |
   = help: the trait `PartialEq<T>` is not implemented for `&T`
help: consider extending the `where` clause, but there might be an alternative better way to express this requirement
   |
LL | fn foo2<T: PartialEq>(a: &T, b: T) where, &T: PartialEq<T> {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
---
diff of stderr:

139   --> $DIR/edition-lint-infer-outlives-multispan.rs:77:38
140    |
141 LL |     struct BeeOutlivesAyTeeBee<'a, 'b: 'a, T: 'b> {
-    |                                      ^^^^   ^^^^
+    |                                      ^^^^   ^^^^ ^
144 help: remove these bounds
145    |

151   --> $DIR/edition-lint-infer-outlives-multispan.rs:82:40
151   --> $DIR/edition-lint-infer-outlives-multispan.rs:82:40
152    |
153 LL |     struct BeeOutlivesAyTeeAyBee<'a, 'b: 'a, T: 'a + 'b> {
-    |                                        ^^^^   ^^^^^^^^^
+    |                                        ^^^^   ^^^^^^^^^ ^
156 help: remove these bounds
157    |

343   --> $DIR/edition-lint-infer-outlives-multispan.rs:148:38
343   --> $DIR/edition-lint-infer-outlives-multispan.rs:148:38
344    |
345 LL |     struct BeeOutlivesAyTeeBee<'a, 'b: 'a, T: 'b>(&'a &'b T);
-    |                                      ^^^^   ^^^^
+    |                                      ^^^^   ^^^^            ^
348 help: remove these bounds
349    |

355   --> $DIR/edition-lint-infer-outlives-multispan.rs:151:40
355   --> $DIR/edition-lint-infer-outlives-multispan.rs:151:40
356    |
357 LL |     struct BeeOutlivesAyTeeAyBee<'a, 'b: 'a, T: 'a + 'b>(&'a &'b T);
-    |                                        ^^^^   ^^^^^^^^^
+    |                                        ^^^^   ^^^^^^^^^            ^
360 help: remove these bounds
361    |

547   --> $DIR/edition-lint-infer-outlives-multispan.rs:235:36
547   --> $DIR/edition-lint-infer-outlives-multispan.rs:235:36
548    |
549 LL |     enum BeeOutlivesAyTeeBee<'a, 'b: 'a, T: 'b> {
-    |                                    ^^^^   ^^^^
+    |                                    ^^^^   ^^^^ ^
552 help: remove these bounds
553    |

559   --> $DIR/edition-lint-infer-outlives-multispan.rs:240:38
559   --> $DIR/edition-lint-infer-outlives-multispan.rs:240:38
560    |
561 LL |     enum BeeOutlivesAyTeeAyBee<'a, 'b: 'a, T: 'a + 'b> {
-    |                                      ^^^^   ^^^^^^^^^
+    |                                      ^^^^   ^^^^^^^^^ ^
564 help: remove these bounds
565    |

751   --> $DIR/edition-lint-infer-outlives-multispan.rs:335:37
751   --> $DIR/edition-lint-infer-outlives-multispan.rs:335:37
752    |
753 LL |     union BeeOutlivesAyTeeBee<'a, 'b: 'a, T: 'b> {
-    |                                     ^^^^   ^^^^
+    |                                     ^^^^   ^^^^ ^
756 help: remove these bounds
757    |

763   --> $DIR/edition-lint-infer-outlives-multispan.rs:340:39
763   --> $DIR/edition-lint-infer-outlives-multispan.rs:340:39
764    |
765 LL |     union BeeOutlivesAyTeeAyBee<'a, 'b: 'a, T: 'a + 'b> {
-    |                                       ^^^^   ^^^^^^^^^
+    |                                       ^^^^   ^^^^^^^^^ ^
768 help: remove these bounds
769    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-infer-outlives-multispan/edition-lint-infer-outlives-multispan.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rust-2018/edition-lint-infer-outlives-multispan.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-infer-outlives-multispan" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-infer-outlives-multispan/auxiliary"
stdout: none
--- stderr -------------------------------
error: outlives requirements can be inferred
   |
   |
LL |     struct TeeOutlivesAyIsDebugBee<'a, 'b, T: 'a + Debug + 'b> {
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:2:9
   |
   |
LL | #![deny(explicit_outlives_requirements)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: remove these bounds
   |
LL -     struct TeeOutlivesAyIsDebugBee<'a, 'b, T: 'a + Debug + 'b> {
LL +     struct TeeOutlivesAyIsDebugBee<'a, 'b, T: Debug> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:18:61
   |
   |
LL |     struct TeeWhereOutlivesAyIsDebugBee<'a, 'b, T> where T: 'a + Debug + 'b {
   |
help: remove these bounds
   |
   |
LL -     struct TeeWhereOutlivesAyIsDebugBee<'a, 'b, T> where T: 'a + Debug + 'b {
LL +     struct TeeWhereOutlivesAyIsDebugBee<'a, 'b, T> where T: Debug {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:23:53
   |
   |
LL |     struct TeeYooOutlivesAyIsDebugBee<'a, 'b, T, U: 'a + Debug + 'b> {
   |
help: remove these bounds
   |
   |
LL -     struct TeeYooOutlivesAyIsDebugBee<'a, 'b, T, U: 'a + Debug + 'b> {
LL +     struct TeeYooOutlivesAyIsDebugBee<'a, 'b, T, U: Debug> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:29:48
   |
   |
LL |     struct TeeOutlivesAyYooBeeIsDebug<'a, 'b, T: 'a, U: 'b + Debug> {
   |
help: remove these bounds
   |
   |
LL -     struct TeeOutlivesAyYooBeeIsDebug<'a, 'b, T: 'a, U: 'b + Debug> {
LL +     struct TeeOutlivesAyYooBeeIsDebug<'a, 'b, T, U: Debug> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:35:48
   |
   |
LL |     struct TeeOutlivesAyYooIsDebugBee<'a, 'b, T: 'a, U: Debug + 'b> {
   |
help: remove these bounds
   |
   |
LL -     struct TeeOutlivesAyYooIsDebugBee<'a, 'b, T: 'a, U: Debug + 'b> {
LL +     struct TeeOutlivesAyYooIsDebugBee<'a, 'b, T, U: Debug> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:41:46
   |
   |
LL |     struct TeeOutlivesAyYooWhereBee<'a, 'b, T: 'a, U> where U: 'b {
   |
help: remove these bounds
   |
   |
LL -     struct TeeOutlivesAyYooWhereBee<'a, 'b, T: 'a, U> where U: 'b {
LL +     struct TeeOutlivesAyYooWhereBee<'a, 'b, T, U> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:47:67
   |
   |
LL |     struct TeeYooWhereOutlivesAyIsDebugBee<'a, 'b, T, U> where U: 'a + Debug + 'b {
   |
help: remove these bounds
   |
   |
LL -     struct TeeYooWhereOutlivesAyIsDebugBee<'a, 'b, T, U> where U: 'a + Debug + 'b {
LL +     struct TeeYooWhereOutlivesAyIsDebugBee<'a, 'b, T, U> where U: Debug {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:53:53
   |
   |
LL |     struct TeeOutlivesAyYooWhereBeeIsDebug<'a, 'b, T: 'a, U> where U: 'b + Debug {
   |
help: remove these bounds
   |
   |
LL -     struct TeeOutlivesAyYooWhereBeeIsDebug<'a, 'b, T: 'a, U> where U: 'b + Debug {
LL +     struct TeeOutlivesAyYooWhereBeeIsDebug<'a, 'b, T, U> where U: Debug {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:59:53
   |
   |
LL |     struct TeeOutlivesAyYooWhereIsDebugBee<'a, 'b, T: 'a, U> where U: Debug + 'b {
   |
help: remove these bounds
   |
   |
LL -     struct TeeOutlivesAyYooWhereIsDebugBee<'a, 'b, T: 'a, U> where U: Debug + 'b {
LL +     struct TeeOutlivesAyYooWhereIsDebugBee<'a, 'b, T, U> where U: Debug {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:65:69
   |
   |
LL |     struct TeeWhereOutlivesAyYooWhereBeeIsDebug<'a, 'b, T, U> where T: 'a, U: 'b + Debug {
   |                                                                     ^^^^^^^   ^^^^^
help: remove these bounds
   |
   |
LL -     struct TeeWhereOutlivesAyYooWhereBeeIsDebug<'a, 'b, T, U> where T: 'a, U: 'b + Debug {
LL +     struct TeeWhereOutlivesAyYooWhereBeeIsDebug<'a, 'b, T, U> where U: Debug {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:71:69
   |
   |
LL |     struct TeeWhereOutlivesAyYooWhereIsDebugBee<'a, 'b, T, U> where T: 'a, U: Debug + 'b {
   |
help: remove these bounds
   |
   |
LL -     struct TeeWhereOutlivesAyYooWhereIsDebugBee<'a, 'b, T, U> where T: 'a, U: Debug + 'b {
LL +     struct TeeWhereOutlivesAyYooWhereIsDebugBee<'a, 'b, T, U> where U: Debug {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:77:38
   |
   |
LL |     struct BeeOutlivesAyTeeBee<'a, 'b: 'a, T: 'b> {
   |                                      ^^^^   ^^^^ ^
help: remove these bounds
   |
   |
LL -     struct BeeOutlivesAyTeeBee<'a, 'b: 'a, T: 'b> {
LL +     struct BeeOutlivesAyTeeBee<'a, 'b, T> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:82:40
   |
   |
LL |     struct BeeOutlivesAyTeeAyBee<'a, 'b: 'a, T: 'a + 'b> {
   |                                        ^^^^   ^^^^^^^^^ ^
help: remove these bounds
   |
   |
LL -     struct BeeOutlivesAyTeeAyBee<'a, 'b: 'a, T: 'a + 'b> {
LL +     struct BeeOutlivesAyTeeAyBee<'a, 'b, T> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:87:55
   |
   |
LL |     struct BeeOutlivesAyTeeOutlivesAyIsDebugBee<'a, 'b: 'a, T: 'a + Debug + 'b> {
   |
help: remove these bounds
   |
   |
LL -     struct BeeOutlivesAyTeeOutlivesAyIsDebugBee<'a, 'b: 'a, T: 'a + Debug + 'b> {
LL +     struct BeeOutlivesAyTeeOutlivesAyIsDebugBee<'a, 'b, T: Debug> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:92:68
   |
   |
LL |     struct BeeWhereAyTeeWhereOutlivesAyIsDebugBee<'a, 'b, T> where 'b: 'a, T: 'a + Debug + 'b {
   |                                                                    ^^^^^^^^   ^^^^^     ^^^^^
help: remove these bounds
   |
   |
LL -     struct BeeWhereAyTeeWhereOutlivesAyIsDebugBee<'a, 'b, T> where 'b: 'a, T: 'a + Debug + 'b {
LL +     struct BeeWhereAyTeeWhereOutlivesAyIsDebugBee<'a, 'b, T> where T: Debug {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:97:58
   |
   |
LL |     struct BeeOutlivesAyTeeYooOutlivesAyIsDebugBee<'a, 'b: 'a, T, U: 'a + Debug + 'b> {
   |
help: remove these bounds
   |
   |
LL -     struct BeeOutlivesAyTeeYooOutlivesAyIsDebugBee<'a, 'b: 'a, T, U: 'a + Debug + 'b> {
LL +     struct BeeOutlivesAyTeeYooOutlivesAyIsDebugBee<'a, 'b, T, U: Debug> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:104:18
   |
   |
LL |         where U: 'a + Debug + 'b, 'b: 'a
   |
help: remove these bounds
   |
   |
LL -         where U: 'a + Debug + 'b, 'b: 'a
LL +         where U: Debug, 

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:115:47
   |
   |
LL |     struct TeeOutlivesAyIsDebugBee<'a, 'b, T: 'a + Debug + 'b>(&'a &'b T);
   |
help: remove these bounds
   |
   |
LL -     struct TeeOutlivesAyIsDebugBee<'a, 'b, T: 'a + Debug + 'b>(&'a &'b T);
LL +     struct TeeOutlivesAyIsDebugBee<'a, 'b, T: Debug>(&'a &'b T);

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:118:72
   |
   |
LL |     struct TeeWhereOutlivesAyIsDebugBee<'a, 'b, T>(&'a &'b T) where T: 'a + Debug + 'b;
   |
help: remove these bounds
   |
   |
LL -     struct TeeWhereOutlivesAyIsDebugBee<'a, 'b, T>(&'a &'b T) where T: 'a + Debug + 'b;
LL +     struct TeeWhereOutlivesAyIsDebugBee<'a, 'b, T>(&'a &'b T) where T: Debug;

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:121:53
   |
   |
LL |     struct TeeYooOutlivesAyIsDebugBee<'a, 'b, T, U: 'a + Debug + 'b>(T, &'a &'b U);
   |
help: remove these bounds
   |
   |
LL -     struct TeeYooOutlivesAyIsDebugBee<'a, 'b, T, U: 'a + Debug + 'b>(T, &'a &'b U);
LL +     struct TeeYooOutlivesAyIsDebugBee<'a, 'b, T, U: Debug>(T, &'a &'b U);

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:124:48
   |
   |
LL |     struct TeeOutlivesAyYooBeeIsDebug<'a, 'b, T: 'a, U: 'b + Debug>(&'a T, &'b U);
   |
help: remove these bounds
   |
   |
LL -     struct TeeOutlivesAyYooBeeIsDebug<'a, 'b, T: 'a, U: 'b + Debug>(&'a T, &'b U);
LL +     struct TeeOutlivesAyYooBeeIsDebug<'a, 'b, T, U: Debug>(&'a T, &'b U);

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:127:48
   |
   |
LL |     struct TeeOutlivesAyYooIsDebugBee<'a, 'b, T: 'a, U: Debug + 'b>(&'a T, &'b U);
   |
help: remove these bounds
   |
   |
LL -     struct TeeOutlivesAyYooIsDebugBee<'a, 'b, T: 'a, U: Debug + 'b>(&'a T, &'b U);
LL +     struct TeeOutlivesAyYooIsDebugBee<'a, 'b, T, U: Debug>(&'a T, &'b U);

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:130:46
   |
   |
LL |     struct TeeOutlivesAyYooWhereBee<'a, 'b, T: 'a, U>(&'a T, &'b U) where U: 'b;
   |
help: remove these bounds
   |
   |
LL -     struct TeeOutlivesAyYooWhereBee<'a, 'b, T: 'a, U>(&'a T, &'b U) where U: 'b;
LL +     struct TeeOutlivesAyYooWhereBee<'a, 'b, T, U>(&'a T, &'b U) ;

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:133:81
   |
   |
LL |     struct TeeYooWhereOutlivesAyIsDebugBee<'a, 'b, T, U>(T, &'a &'b U) where U: 'a + Debug + 'b;
   |
help: remove these bounds
   |
   |
LL -     struct TeeYooWhereOutlivesAyIsDebugBee<'a, 'b, T, U>(T, &'a &'b U) where U: 'a + Debug + 'b;
LL +     struct TeeYooWhereOutlivesAyIsDebugBee<'a, 'b, T, U>(T, &'a &'b U) where U: Debug;

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:136:53
   |
   |
LL |     struct TeeOutlivesAyYooWhereBeeIsDebug<'a, 'b, T: 'a, U>(&'a T, &'b U) where U: 'b + Debug;
   |
help: remove these bounds
   |
   |
LL -     struct TeeOutlivesAyYooWhereBeeIsDebug<'a, 'b, T: 'a, U>(&'a T, &'b U) where U: 'b + Debug;
LL +     struct TeeOutlivesAyYooWhereBeeIsDebug<'a, 'b, T, U>(&'a T, &'b U) where U: Debug;

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:139:53
   |
   |
LL |     struct TeeOutlivesAyYooWhereIsDebugBee<'a, 'b, T: 'a, U>(&'a T, &'b U) where U: Debug + 'b;
   |
help: remove these bounds
   |
   |
LL -     struct TeeOutlivesAyYooWhereIsDebugBee<'a, 'b, T: 'a, U>(&'a T, &'b U) where U: Debug + 'b;
LL +     struct TeeOutlivesAyYooWhereIsDebugBee<'a, 'b, T, U>(&'a T, &'b U) where U: Debug;

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:142:75
   |
   |
LL |     struct TeeWhereAyYooWhereBeeIsDebug<'a, 'b, T, U>(&'a T, &'b U) where T: 'a, U: 'b + Debug;
   |                                                                           ^^^^^^^   ^^^^^
help: remove these bounds
   |
   |
LL -     struct TeeWhereAyYooWhereBeeIsDebug<'a, 'b, T, U>(&'a T, &'b U) where T: 'a, U: 'b + Debug;
LL +     struct TeeWhereAyYooWhereBeeIsDebug<'a, 'b, T, U>(&'a T, &'b U) where U: Debug;

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:145:75
   |
   |
LL |     struct TeeWhereAyYooWhereIsDebugBee<'a, 'b, T, U>(&'a T, &'b U) where T: 'a, U: Debug + 'b;
   |
help: remove these bounds
   |
   |
LL -     struct TeeWhereAyYooWhereIsDebugBee<'a, 'b, T, U>(&'a T, &'b U) where T: 'a, U: Debug + 'b;
LL +     struct TeeWhereAyYooWhereIsDebugBee<'a, 'b, T, U>(&'a T, &'b U) where U: Debug;

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:148:38
   |
   |
LL |     struct BeeOutlivesAyTeeBee<'a, 'b: 'a, T: 'b>(&'a &'b T);
   |                                      ^^^^   ^^^^            ^
help: remove these bounds
   |
   |
LL -     struct BeeOutlivesAyTeeBee<'a, 'b: 'a, T: 'b>(&'a &'b T);
LL +     struct BeeOutlivesAyTeeBee<'a, 'b, T>(&'a &'b T);

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:151:40
   |
   |
LL |     struct BeeOutlivesAyTeeAyBee<'a, 'b: 'a, T: 'a + 'b>(&'a &'b T);
   |                                        ^^^^   ^^^^^^^^^            ^
help: remove these bounds
   |
   |
LL -     struct BeeOutlivesAyTeeAyBee<'a, 'b: 'a, T: 'a + 'b>(&'a &'b T);
LL +     struct BeeOutlivesAyTeeAyBee<'a, 'b, T>(&'a &'b T);

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:154:55
   |
   |
LL |     struct BeeOutlivesAyTeeOutlivesAyIsDebugBee<'a, 'b: 'a, T: 'a + Debug + 'b>(&'a &'b T);
   |
help: remove these bounds
   |
   |
LL -     struct BeeOutlivesAyTeeOutlivesAyIsDebugBee<'a, 'b: 'a, T: 'a + Debug + 'b>(&'a &'b T);
LL +     struct BeeOutlivesAyTeeOutlivesAyIsDebugBee<'a, 'b, T: Debug>(&'a &'b T);

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:157:71
   |
   |
LL |     struct BeeWhereAyTeeWhereAyIsDebugBee<'a, 'b, T>(&'a &'b T) where 'b: 'a, T: 'a + Debug + 'b;
   |                                                                       ^^^^^^^^   ^^^^^     ^^^^^
help: remove these bounds
   |
   |
LL -     struct BeeWhereAyTeeWhereAyIsDebugBee<'a, 'b, T>(&'a &'b T) where 'b: 'a, T: 'a + Debug + 'b;
LL +     struct BeeWhereAyTeeWhereAyIsDebugBee<'a, 'b, T>(&'a &'b T) where T: Debug;

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:160:58
   |
   |
LL |     struct BeeOutlivesAyTeeYooOutlivesAyIsDebugBee<'a, 'b: 'a, T, U: 'a + Debug + 'b>(T, &'a &'b U);
   |
help: remove these bounds
   |
   |
LL -     struct BeeOutlivesAyTeeYooOutlivesAyIsDebugBee<'a, 'b: 'a, T, U: 'a + Debug + 'b>(T, &'a &'b U);
LL +     struct BeeOutlivesAyTeeYooOutlivesAyIsDebugBee<'a, 'b, T, U: Debug>(T, &'a &'b U);

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:164:18
   |
   |
LL |         where U: 'a + Debug + 'b, 'b: 'a;
   |
help: remove these bounds
   |
   |
LL -         where U: 'a + Debug + 'b, 'b: 'a;
LL +         where U: Debug, ;

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-multispan.rs:171:45
   |
---
To only update this specific test, also pass `--test-args rust-2018/edition-lint-infer-outlives.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-infer-outlives" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-infer-outlives/auxiliary"
stdout: none
--- stderr -------------------------------
error: outlives requirements can be inferred
   |
   |
LL |     struct TeeOutlivesAy<'a, T: 'a> {
   |                               ^^^^ ^
note: the lint level is defined here
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:4:9
   |
LL | #![deny(explicit_outlives_requirements)]
LL | #![deny(explicit_outlives_requirements)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: remove this bound
   |
LL -     struct TeeOutlivesAy<'a, T: 'a> {
LL +     struct TeeOutlivesAy<'a, T> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:31:40
   |
   |
LL |     struct TeeOutlivesAyIsDebug<'a, T: 'a + Debug> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:36:45
   |
   |
LL |     struct TeeIsDebugOutlivesAy<'a, T: Debug + 'a> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:41:38
   |
   |
LL |     struct TeeOutlivesAyBee<'a, 'b, T: 'a + 'b> {
   |                                      ^^^^^^^^^ ^
help: remove these bounds
   |
   |
LL -     struct TeeOutlivesAyBee<'a, 'b, T: 'a + 'b> {
LL +     struct TeeOutlivesAyBee<'a, 'b, T> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:46:47
   |
   |
LL |     struct TeeOutlivesAyBeeIsDebug<'a, 'b, T: 'a + 'b + Debug> {
   |                                               ^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:51:52
   |
   |
LL |     struct TeeIsDebugOutlivesAyBee<'a, 'b, T: Debug + 'a + 'b> {
   |                                                    ^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:56:37
   |
   |
LL |     struct TeeWhereOutlivesAy<'a, T> where T: 'a {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:61:54
   |
   |
LL |     struct TeeWhereOutlivesAyIsDebug<'a, T> where T: 'a + Debug {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:66:59
   |
   |
LL |     struct TeeWhereIsDebugOutlivesAy<'a, T> where T: Debug + 'a {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:71:44
   |
   |
LL |     struct TeeWhereOutlivesAyBee<'a, 'b, T> where T: 'a + 'b {
   |                                            ^^^^^^^^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:76:61
   |
   |
LL |     struct TeeWhereOutlivesAyBeeIsDebug<'a, 'b, T> where T: 'a + 'b + Debug {
   |                                                             ^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:81:66
   |
   |
LL |     struct TeeWhereIsDebugOutlivesAyBee<'a, 'b, T> where T: Debug + 'a + 'b {
   |                                                                  ^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:86:37
   |
   |
LL |     struct TeeYooOutlivesAy<'a, T, U: 'a> {
   |                                     ^^^^ ^
help: remove this bound
   |
   |
LL -     struct TeeYooOutlivesAy<'a, T, U: 'a> {
LL +     struct TeeYooOutlivesAy<'a, T, U> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:92:46
   |
   |
LL |     struct TeeYooOutlivesAyIsDebug<'a, T, U: 'a + Debug> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:98:51
   |
   |
LL |     struct TeeYooIsDebugOutlivesAy<'a, T, U: Debug + 'a> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:104:41
   |
   |
LL |     struct TeeOutlivesAyYooIsDebug<'a, T: 'a, U: Debug> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:110:44
   |
   |
LL |     struct TeeYooOutlivesAyBee<'a, 'b, T, U: 'a + 'b> {
   |                                            ^^^^^^^^^ ^
help: remove these bounds
   |
   |
LL -     struct TeeYooOutlivesAyBee<'a, 'b, T, U: 'a + 'b> {
LL +     struct TeeYooOutlivesAyBee<'a, 'b, T, U> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:116:53
   |
   |
LL |     struct TeeYooOutlivesAyBeeIsDebug<'a, 'b, T, U: 'a + 'b + Debug> {
   |                                                     ^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:122:58
   |
   |
LL |     struct TeeYooIsDebugOutlivesAyBee<'a, 'b, T, U: Debug + 'a + 'b> {
   |                                                          ^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:128:48
   |
   |
LL |     struct TeeOutlivesAyBeeYooIsDebug<'a, 'b, T: 'a + 'b, U: Debug> {
   |                                                ^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:134:43
   |
   |
LL |     struct TeeYooWhereOutlivesAy<'a, T, U> where U: 'a {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:140:60
   |
   |
LL |     struct TeeYooWhereOutlivesAyIsDebug<'a, T, U> where U: 'a + Debug {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:146:65
   |
   |
LL |     struct TeeYooWhereIsDebugOutlivesAy<'a, T, U> where U: Debug + 'a {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:152:46
   |
   |
LL |     struct TeeOutlivesAyYooWhereIsDebug<'a, T: 'a, U> where U: Debug {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:158:50
   |
   |
LL |     struct TeeYooWhereOutlivesAyBee<'a, 'b, T, U> where U: 'a + 'b {
   |                                                  ^^^^^^^^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:164:67
   |
   |
LL |     struct TeeYooWhereOutlivesAyBeeIsDebug<'a, 'b, T, U> where U: 'a + 'b + Debug {
   |                                                                   ^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:170:72
   |
   |
LL |     struct TeeYooWhereIsDebugOutlivesAyBee<'a, 'b, T, U> where U: Debug + 'a + 'b {
   |                                                                        ^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:176:53
   |
   |
LL |     struct TeeOutlivesAyBeeYooWhereIsDebug<'a, 'b, T: 'a + 'b, U> where U: Debug {
   |                                                     ^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:182:62
   |
   |
LL |     struct TeeWhereOutlivesAyYooWhereIsDebug<'a, T, U> where T: 'a, U: Debug {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:188:69
   |
   |
LL |     struct TeeWhereOutlivesAyBeeYooWhereIsDebug<'a, 'b, T, U> where T: 'a + 'b, U: Debug {
   |                                                                     ^^^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:194:32
   |
   |
LL |     struct BeeOutlivesAy<'a, 'b: 'a> {
   |                                ^^^^ ^
help: remove this bound
   |
   |
LL -     struct BeeOutlivesAy<'a, 'b: 'a> {
LL +     struct BeeOutlivesAy<'a, 'b> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:199:38
   |
   |
LL |     struct BeeWhereOutlivesAy<'a, 'b> where 'b: 'a {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:204:35
   |
   |
LL |     struct BeeOutlivesAyTee<'a, 'b: 'a, T> {
   |
help: remove this bound
   |
   |
LL -     struct BeeOutlivesAyTee<'a, 'b: 'a, T> {
LL +     struct BeeOutlivesAyTee<'a, 'b, T> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:209:44
   |
   |
LL |     struct BeeWhereOutlivesAyTee<'a, 'b, T> where 'b: 'a {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:214:52
   |
   |
LL |     struct BeeWhereOutlivesAyTeeWhereBee<'a, 'b, T> where 'b: 'a, T: 'b {
   |                                                    ^^^^^^^^^^^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:219:54
   |
   |
LL |     struct BeeWhereOutlivesAyTeeWhereAyBee<'a, 'b, T> where 'b: 'a, T: 'a + 'b {
   |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:224:40
   |
   |
LL |     struct BeeOutlivesAyTeeDebug<'a, 'b: 'a, T: Debug> {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:229:61
   |
   |
LL |     struct BeeWhereOutlivesAyTeeWhereDebug<'a, 'b, T> where 'b: 'a, T: Debug {

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:238:31
   |
   |
LL |     struct TeeOutlivesAy<'a, T: 'a>(&'a T);
   |
help: remove this bound
   |
   |
LL -     struct TeeOutlivesAy<'a, T: 'a>(&'a T);
LL +     struct TeeOutlivesAy<'a, T>(&'a T);

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:241:40
   |
   |
LL |     struct TeeOutlivesAyIsDebug<'a, T: 'a + Debug>(&'a T);

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:244:45
   |
   |
LL |     struct TeeIsDebugOutlivesAy<'a, T: Debug + 'a>(&'a T);

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:247:38
   |
   |
LL |     struct TeeOutlivesAyBee<'a, 'b, T: 'a + 'b>(&'a &'b T);
   |
help: remove these bounds
   |
   |
LL -     struct TeeOutlivesAyBee<'a, 'b, T: 'a + 'b>(&'a &'b T);
LL +     struct TeeOutlivesAyBee<'a, 'b, T>(&'a &'b T);

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:250:47
   |
   |
LL |     struct TeeOutlivesAyBeeIsDebug<'a, 'b, T: 'a + 'b + Debug>(&'a &'b T);
   |                                               ^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:253:52
   |
   |
LL |     struct TeeIsDebugOutlivesAyBee<'a, 'b, T: Debug + 'a + 'b>(&'a &'b T);
   |                                                    ^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:256:45
   |
   |
LL |     struct TeeWhereOutlivesAy<'a, T>(&'a T) where T: 'a;

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:259:61
   |
   |
LL |     struct TeeWhereOutlivesAyIsDebug<'a, T>(&'a T) where T: 'a + Debug;

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:262:66
   |
   |
LL |     struct TeeWhereIsDebugOutlivesAy<'a, T>(&'a T) where T: Debug + 'a;

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:265:56
   |
   |
LL |     struct TeeWhereOutlivesAyBee<'a, 'b, T>(&'a &'b T) where T: 'a + 'b;
   |                                                        ^^^^^^^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:268:72
   |
   |
LL |     struct TeeWhereOutlivesAyBeeIsDebug<'a, 'b, T>(&'a &'b T) where T: 'a + 'b + Debug;
   |                                                                        ^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:271:77
   |
   |
LL |     struct TeeWhereIsDebugOutlivesAyBee<'a, 'b, T>(&'a &'b T) where T: Debug + 'a + 'b;
   |                                                                             ^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:274:37
   |
   |
LL |     struct TeeYooOutlivesAy<'a, T, U: 'a>(T, &'a U);
   |
help: remove this bound
   |
   |
LL -     struct TeeYooOutlivesAy<'a, T, U: 'a>(T, &'a U);
LL +     struct TeeYooOutlivesAy<'a, T, U>(T, &'a U);

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:277:46
   |
   |
LL |     struct TeeYooOutlivesAyIsDebug<'a, T, U: 'a + Debug>(T, &'a U);

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:280:51
   |
   |
LL |     struct TeeYooIsDebugOutlivesAy<'a, T, U: Debug + 'a>(T, &'a U);

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:283:41
   |
   |
LL |     struct TeeOutlivesAyYooIsDebug<'a, T: 'a, U: Debug>(&'a T, U);

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:286:44
   |
   |
LL |     struct TeeYooOutlivesAyBee<'a, 'b, T, U: 'a + 'b>(T, &'a &'b U);
   |
help: remove these bounds
   |
   |
LL -     struct TeeYooOutlivesAyBee<'a, 'b, T, U: 'a + 'b>(T, &'a &'b U);
LL +     struct TeeYooOutlivesAyBee<'a, 'b, T, U>(T, &'a &'b U);

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:289:53
   |
   |
LL |     struct TeeYooOutlivesAyBeeIsDebug<'a, 'b, T, U: 'a + 'b + Debug>(T, &'a &'b U);
   |                                                     ^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:292:58
   |
   |
LL |     struct TeeYooIsDebugOutlivesAyBee<'a, 'b, T, U: Debug + 'a + 'b>(T, &'a &'b U);
   |                                                          ^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:295:48
   |
   |
LL |     struct TeeOutlivesAyBeeYooIsDebug<'a, 'b, T: 'a + 'b, U: Debug>(&'a &'b T, U);
   |                                                ^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:298:54
   |
   |
LL |     struct TeeYooWhereOutlivesAy<'a, T, U>(T, &'a U) where U: 'a;

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:301:70
   |
   |
LL |     struct TeeYooWhereOutlivesAyIsDebug<'a, T, U>(T, &'a U) where U: 'a + Debug;

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:304:75
   |
   |
LL |     struct TeeYooWhereIsDebugOutlivesAy<'a, T, U>(T, &'a U) where U: Debug + 'a;

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:307:46
   |
   |
LL |     struct TeeOutlivesAyYooWhereIsDebug<'a, T: 'a, U>(&'a T, U) where U: Debug;

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:310:65
   |
   |
LL |     struct TeeYooWhereOutlivesAyBee<'a, 'b, T, U>(T, &'a &'b U) where U: 'a + 'b;
   |                                                                 ^^^^^^^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:313:81
   |
   |
LL |     struct TeeYooWhereOutlivesAyBeeIsDebug<'a, 'b, T, U>(T, &'a &'b U) where U: 'a + 'b + Debug;
   |                                                                                 ^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:316:86
   |
   |
LL |     struct TeeYooWhereIsDebugOutlivesAyBee<'a, 'b, T, U>(T, &'a &'b U) where U: Debug + 'a + 'b;
   |                                                                                      ^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:319:53
   |
   |
LL |     struct TeeOutlivesAyBeeYooWhereIsDebug<'a, 'b, T: 'a + 'b, U>(&'a &'b T, U) where U: Debug;
   |                                                     ^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:322:72
   |
   |
LL |     struct TeeWhereOutlivesAyYooWhereIsDebug<'a, T, U>(&'a T, U) where T: 'a, U: Debug;

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:325:75
   |
   |
LL |     struct TeeWhereAyBeeYooWhereIsDebug<'a, 'b, T, U>(&'a &'b T, U) where T: 'a + 'b, U: Debug;
   |                                                                           ^^^^^^^^^^^^ help: remove these bounds
error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:328:32
   |
   |
LL |     struct BeeOutlivesAy<'a, 'b: 'a>(&'a &'b ());
   |
help: remove this bound
   |
   |
LL -     struct BeeOutlivesAy<'a, 'b: 'a>(&'a &'b ());
LL +     struct BeeOutlivesAy<'a, 'b>(&'a &'b ());

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:331:51
   |
   |
LL |     struct BeeWhereOutlivesAy<'a, 'b>(&'a &'b ()) where 'b: 'a;

error: outlives requirements can be inferred
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives.rs:334:35
   |
---
To only update this specific test, also pass `--test-args rust-2018/edition-lint-infer-outlives-macro.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-infer-outlives-macro" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-infer-outlives-macro/auxiliary"
stdout: none
--- stderr -------------------------------
error: outlives requirements can be inferred
   |
   |
LL | struct Bar<'a, 'b: 'a> {
   |                  ^^^^ ^
note: the lint level is defined here
  --> /checkout/src/test/ui/rust-2018/edition-lint-infer-outlives-macro.rs:8:9
   |
LL | #![deny(explicit_outlives_requirements)]
LL | #![deny(explicit_outlives_requirements)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: remove this bound
   |
LL - struct Bar<'a, 'b: 'a> {
LL + struct Bar<'a, 'b> {

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/specialization/default-associated-type-bound-2.rs stdout ----
diff of stderr:

22    |             ^^^^^^^^^^^^ required by this bound in `X::U`
23 help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
24    |
- LL | impl<B: 'static, T> X<B> for T where &'static B: PartialEq<B> {
-    |                                ++++++++++++++++++++++++++++++
+ LL | impl<B: 'static, T> X<B> for T, &'static B: PartialEq<B> {
27 
28 error: aborting due to previous error; 1 warning emitted
29 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/default-associated-type-bound-2/default-associated-type-bound-2.stderr
To only update this specific test, also pass `--test-args specialization/default-associated-type-bound-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/default-associated-type-bound-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/default-associated-type-bound-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/default-associated-type-bound-2/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete

error[E0277]: can't compare `&'static B` with `B`
   |
LL |     default type U = &'static B;
LL |     default type U = &'static B;
   |                      ^^^^^^^^^^ no implementation for `&'static B == B`
   |
   = help: the trait `PartialEq<B>` is not implemented for `&'static B`
note: required by a bound in `X::U`
   |
   |
LL |     type U: PartialEq<T>;
   |             ^^^^^^^^^^^^ required by this bound in `X::U`
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | impl<B: 'static, T> X<B> for T, &'static B: PartialEq<B> {

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/suggestions/derive-clone-for-eq.rs stdout ----
diff of stderr:

15 LL | pub trait Eq: PartialEq<Self> {
16    |               ^^^^^^^^^^^^^^^ required by this bound in `Eq`
17    = note: this error originates in the derive macro `Eq` (in Nightly builds, run with -Z macro-backtrace for more info)
- help: consider restricting type parameter `T`
+ help: consider further restricting type parameter `T`
19    |
- LL | pub struct Struct<T: std::clone::Clone>(T);
-    |                    +++++++++++++++++++
+ LL | pub struct Struct<T>(T), T: std::clone::Clone;
22 
23 error: aborting due to previous error
24 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/derive-clone-for-eq/derive-clone-for-eq.stderr

4 use std::cmp::PartialEq;
5 
5 
6 #[derive(Clone, Eq)] //~ ERROR [E0277]
- pub struct Struct<T: std::clone::Clone>(T);
+ pub struct Struct<T>(T), T: std::clone::Clone;
8 
9 impl<T: Clone, U> PartialEq<U> for Struct<T>


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/derive-clone-for-eq/derive-clone-for-eq.fixed
To only update this specific test, also pass `--test-args suggestions/derive-clone-for-eq.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/derive-clone-for-eq.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/derive-clone-for-eq" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/derive-clone-for-eq/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `T: Clone` is not satisfied
  --> /checkout/src/test/ui/suggestions/derive-clone-for-eq.rs:6:17
   |
LL | #[derive(Clone, Eq)] //~ ERROR [E0277]
   |                 ^^ the trait `Clone` is not implemented for `T`
   |
note: required because of the requirements on the impl of `PartialEq` for `Struct<T>`
  --> /checkout/src/test/ui/suggestions/derive-clone-for-eq.rs:9:19
   |
LL | impl<T: Clone, U> PartialEq<U> for Struct<T>
note: required by a bound in `Eq`
  --> /checkout/library/core/src/cmp.rs:288:15
   |
   |
LL | pub trait Eq: PartialEq<Self> {
   |               ^^^^^^^^^^^^^^^ required by this bound in `Eq`
   = note: this error originates in the derive macro `Eq` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider further restricting type parameter `T`
   |
LL | pub struct Struct<T>(T), T: std::clone::Clone;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/suggestions/derive-macro-missing-bounds.rs stdout ----
diff of stderr:

15    |
16 help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
17    |
- LL |     struct Outer<T>(Inner<T>) where a::Inner<T>: Debug;
-    |                               ++++++++++++++++++++++++
+ LL |     struct Outer<T>(Inner<T>), a::Inner<T>: Debug;
20 
20 
21 error[E0277]: the trait bound `T: c::Trait` is not satisfied


35    = note: required because of the requirements on the impl of `Debug` for `&c::Inner<T>`
36    = note: required for the cast from `&c::Inner<T>` to the object type `dyn Debug`
- help: consider restricting type parameter `T`
+ help: consider further restricting type parameter `T`
39    |
39    |
- LL |     struct Outer<T: c::Trait>(Inner<T>);
-    |                   ++++++++++
+ LL |     struct Outer<T>(Inner<T>), T: c::Trait;
42 
42 
43 error[E0277]: the trait bound `T: d::Trait` is not satisfied


57    = note: required because of the requirements on the impl of `Debug` for `&d::Inner<T>`
58    = note: required for the cast from `&d::Inner<T>` to the object type `dyn Debug`
- help: consider restricting type parameter `T`
+ help: consider further restricting type parameter `T`
61    |
61    |
- LL |     struct Outer<T: d::Trait>(Inner<T>);
-    |                   ++++++++++
+ LL |     struct Outer<T>(Inner<T>), T: d::Trait;
64 
64 
65 error[E0277]: the trait bound `T: e::Trait` is not satisfied


79    = note: required because of the requirements on the impl of `Debug` for `&e::Inner<T>`
80    = note: required for the cast from `&e::Inner<T>` to the object type `dyn Debug`
- help: consider restricting type parameter `T`
+ help: consider further restricting type parameter `T`
83    |
83    |
- LL |     struct Outer<T: e::Trait>(Inner<T>);
-    |                   ++++++++++
+ LL |     struct Outer<T>(Inner<T>), T: e::Trait;
86 
86 
87 error[E0277]: the trait bound `T: f::Trait` is not satisfied


101    = note: required because of the requirements on the impl of `Debug` for `&f::Inner<T>`
102    = note: required for the cast from `&f::Inner<T>` to the object type `dyn Debug`
- help: consider restricting type parameter `T`
+ help: consider further restricting type parameter `T`
105    |
105    |
- LL |     struct Outer<T: f::Trait>(Inner<T>);
-    |                   ++++++++++
+ LL |     struct Outer<T>(Inner<T>), T: f::Trait;
108 
109 error: aborting due to 5 previous errors
110 

---
To only update this specific test, also pass `--test-args suggestions/derive-macro-missing-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/derive-macro-missing-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/derive-macro-missing-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/derive-macro-missing-bounds/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `a::Inner<T>` doesn't implement `Debug`
   |
LL |     #[derive(Debug)]
   |              ----- in this derive macro expansion
   |              ----- in this derive macro expansion
LL |     struct Outer<T>(Inner<T>); //~ ERROR `a::Inner<T>` doesn't implement `Debug`
   |                     ^^^^^^^^ `a::Inner<T>` cannot be formatted using `{:?}`
   = help: the trait `Debug` is not implemented for `a::Inner<T>`
   = help: the trait `Debug` is not implemented for `a::Inner<T>`
   = note: add `#[derive(Debug)]` to `a::Inner<T>` or manually `impl Debug for a::Inner<T>`
   = note: this error originates in the derive macro `Debug` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `a::Inner<T>` with `#[derive(Debug)]`
LL |     #[derive(Debug)]
   |
   |
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL |     struct Outer<T>(Inner<T>), a::Inner<T>: Debug; //~ ERROR `a::Inner<T>` doesn't implement `Debug`


error[E0277]: the trait bound `T: c::Trait` is not satisfied
   |
LL |     #[derive(Debug)]
   |              ----- in this derive macro expansion
   |              ----- in this derive macro expansion
LL |     struct Outer<T>(Inner<T>); //~ ERROR the trait bound `T: c::Trait` is not satisfied
   |                     ^^^^^^^^ the trait `c::Trait` is not implemented for `T`
   |
note: required because of the requirements on the impl of `Debug` for `c::Inner<T>`
   |
   |
LL |     impl<T: Debug + Trait> Debug for Inner<T> {
   = note: 1 redundant requirement hidden
   = note: 1 redundant requirement hidden
   = note: required because of the requirements on the impl of `Debug` for `&c::Inner<T>`
   = note: required for the cast from `&c::Inner<T>` to the object type `dyn Debug`
help: consider further restricting type parameter `T`
   |
   |
LL |     struct Outer<T>(Inner<T>), T: c::Trait; //~ ERROR the trait bound `T: c::Trait` is not satisfied


error[E0277]: the trait bound `T: d::Trait` is not satisfied
   |
LL |     #[derive(Debug)]
   |              ----- in this derive macro expansion
   |              ----- in this derive macro expansion
LL |     struct Outer<T>(Inner<T>); //~ ERROR the trait bound `T: d::Trait` is not satisfied
   |                     ^^^^^^^^ the trait `d::Trait` is not implemented for `T`
   |
note: required because of the requirements on the impl of `Debug` for `d::Inner<T>`
   |
   |
LL |     impl<T> Debug for Inner<T> where T: Debug, T: Trait {
   = note: 1 redundant requirement hidden
   = note: 1 redundant requirement hidden
   = note: required because of the requirements on the impl of `Debug` for `&d::Inner<T>`
   = note: required for the cast from `&d::Inner<T>` to the object type `dyn Debug`
help: consider further restricting type parameter `T`
   |
   |
LL |     struct Outer<T>(Inner<T>), T: d::Trait; //~ ERROR the trait bound `T: d::Trait` is not satisfied


error[E0277]: the trait bound `T: e::Trait` is not satisfied
   |
LL |     #[derive(Debug)]
   |              ----- in this derive macro expansion
   |              ----- in this derive macro expansion
LL |     struct Outer<T>(Inner<T>); //~ ERROR the trait bound `T: e::Trait` is not satisfied
   |                     ^^^^^^^^ the trait `e::Trait` is not implemented for `T`
   |
note: required because of the requirements on the impl of `Debug` for `e::Inner<T>`
   |
   |
LL |     impl<T> Debug for Inner<T> where T: Debug + Trait {
   = note: 1 redundant requirement hidden
   = note: 1 redundant requirement hidden
   = note: required because of the requirements on the impl of `Debug` for `&e::Inner<T>`
   = note: required for the cast from `&e::Inner<T>` to the object type `dyn Debug`
help: consider further restricting type parameter `T`
   |
   |
LL |     struct Outer<T>(Inner<T>), T: e::Trait; //~ ERROR the trait bound `T: e::Trait` is not satisfied


error[E0277]: the trait bound `T: f::Trait` is not satisfied
   |
LL |     #[derive(Debug)]
   |              ----- in this derive macro expansion
   |              ----- in this derive macro expansion
LL |     struct Outer<T>(Inner<T>); //~ ERROR the trait bound `T: f::Trait` is not satisfied
   |                     ^^^^^^^^ the trait `f::Trait` is not implemented for `T`
   |
note: required because of the requirements on the impl of `Debug` for `f::Inner<T>`
   |
   |
LL |     impl<T: Debug> Debug for Inner<T> where T: Trait {
   = note: 1 redundant requirement hidden
   = note: 1 redundant requirement hidden
   = note: required because of the requirements on the impl of `Debug` for `&f::Inner<T>`
   = note: required for the cast from `&f::Inner<T>` to the object type `dyn Debug`
help: consider further restricting type parameter `T`
   |
   |
LL |     struct Outer<T>(Inner<T>), T: f::Trait; //~ ERROR the trait bound `T: f::Trait` is not satisfied

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/suggestions/impl-trait-with-missing-bounds.rs stdout ----
diff of stderr:

14    |                ^^^^^^^^^^^^^^^ required by this bound in `qux`
15 help: introduce a type parameter with a trait bound instead of using `impl Trait`
16    |
- LL | fn foo<I: Iterator>(constraints: I) where <I as Iterator>::Item: Debug {
-    |       +++++++++++++              ~  ++++++++++++++++++++++++++++++++++
+ LL | fn foo<I: Iterator>(constraints: I), <I as Iterator>::Item: Debug {
+    |       +++++++++++++              ~ ++++++++++++++++++++++++++++++
20 error[E0277]: `<impl Iterator as Iterator>::Item` doesn't implement `Debug`
21   --> $DIR/impl-trait-with-missing-bounds.rs:14:13


52    |                ^^^^^^^^^^^^^^^ required by this bound in `qux`
53 help: introduce a type parameter with a trait bound instead of using `impl Trait`
54    |
- LL | fn baz<I: Iterator>(t: impl std::fmt::Debug, constraints: I) where <I as Iterator>::Item: Debug {
-    |       +++++++++++++                                       ~  ++++++++++++++++++++++++++++++++++
+ LL | fn baz<I: Iterator>(t: impl std::fmt::Debug, constraints: I), <I as Iterator>::Item: Debug {
+    |       +++++++++++++                                       ~ ++++++++++++++++++++++++++++++
58 error[E0277]: `<impl Iterator as Iterator>::Item` doesn't implement `Debug`
59   --> $DIR/impl-trait-with-missing-bounds.rs:30:13


71    |                ^^^^^^^^^^^^^^^ required by this bound in `qux`
72 help: introduce a type parameter with a trait bound instead of using `impl Trait`
73    |
- LL | fn bat<I, T: std::fmt::Debug, U: Iterator>(t: T, constraints: U, _: I) where <U as Iterator>::Item: Debug {
-    |                             +++++++++++++                     ~        ++++++++++++++++++++++++++++++++++
+ LL | fn bat<I, T: std::fmt::Debug, U: Iterator>(t: T, constraints: U, _: I), <U as Iterator>::Item: Debug {
76 
76 
77 error[E0277]: `<impl Iterator + std::fmt::Debug as Iterator>::Item` doesn't implement `Debug`


90    |                ^^^^^^^^^^^^^^^ required by this bound in `qux`
91 help: introduce a type parameter with a trait bound instead of using `impl Trait`
92    |
- LL | fn bak<I: Iterator + std::fmt::Debug>(constraints: I) where <I as Iterator>::Item: Debug {
-    |       +++++++++++++++++++++++++++++++              ~  ++++++++++++++++++++++++++++++++++
+ LL | fn bak<I: Iterator + std::fmt::Debug>(constraints: I), <I as Iterator>::Item: Debug {
+    |       +++++++++++++++++++++++++++++++              ~ ++++++++++++++++++++++++++++++
96 error[E0277]: `<impl Iterator as Iterator>::Item` doesn't implement `Debug`
97   --> $DIR/impl-trait-with-missing-bounds.rs:45:13


109    |                ^^^^^^^^^^^^^^^ required by this bound in `qux`
110 help: introduce a type parameter with a trait bound instead of using `impl Trait`
111    |
- LL | fn baw<I: Iterator>(constraints: I) where <I as Iterator>::Item: Debug {
-    |       ~~~~~~~~~~~~~              ~  ++++++++++++++++++++++++++++++++++
+ LL | fn baw<I: Iterator>(constraints: I), <I as Iterator>::Item: Debug {
+    |       ~~~~~~~~~~~~~              ~ ++++++++++++++++++++++++++++++
115 error: aborting due to 6 previous errors
116 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-trait-with-missing-bounds/impl-trait-with-missing-bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/impl-trait-with-missing-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-trait-with-missing-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-trait-with-missing-bounds/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `<impl Iterator as Iterator>::Item` doesn't implement `Debug`
   |
   |
LL |         qux(constraint);
   |         --- ^^^^^^^^^^ `<impl Iterator as Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |         required by a bound introduced by this call
   |
   = help: the trait `Debug` is not implemented for `<impl Iterator as Iterator>::Item`
note: required by a bound in `qux`
note: required by a bound in `qux`
  --> /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:50:16
   |
LL | fn qux(_: impl std::fmt::Debug) {}
   |                ^^^^^^^^^^^^^^^ required by this bound in `qux`
help: introduce a type parameter with a trait bound instead of using `impl Trait`
   |
LL | fn foo<I: Iterator>(constraints: I), <I as Iterator>::Item: Debug {
   |       +++++++++++++              ~ ++++++++++++++++++++++++++++++
error[E0277]: `<impl Iterator as Iterator>::Item` doesn't implement `Debug`
  --> /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:14:13
   |
   |
LL |         qux(constraint);
   |         --- ^^^^^^^^^^ `<impl Iterator as Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |         required by a bound introduced by this call
   |
   = help: the trait `Debug` is not implemented for `<impl Iterator as Iterator>::Item`
note: required by a bound in `qux`
note: required by a bound in `qux`
  --> /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:50:16
   |
LL | fn qux(_: impl std::fmt::Debug) {}
   |                ^^^^^^^^^^^^^^^ required by this bound in `qux`
help: introduce a type parameter with a trait bound instead of using `impl Trait`
   |
LL | fn bar<T, I: Iterator>(t: T, constraints: I) where T: std::fmt::Debug, <I as Iterator>::Item: Debug {

error[E0277]: `<impl Iterator as Iterator>::Item` doesn't implement `Debug`
  --> /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:22:13
   |
   |
LL |         qux(constraint);
   |         --- ^^^^^^^^^^ `<impl Iterator as Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |         required by a bound introduced by this call
   |
   = help: the trait `Debug` is not implemented for `<impl Iterator as Iterator>::Item`
note: required by a bound in `qux`
note: required by a bound in `qux`
  --> /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:50:16
   |
LL | fn qux(_: impl std::fmt::Debug) {}
   |                ^^^^^^^^^^^^^^^ required by this bound in `qux`
help: introduce a type parameter with a trait bound instead of using `impl Trait`
   |
LL | fn baz<I: Iterator>(t: impl std::fmt::Debug, constraints: I), <I as Iterator>::Item: Debug {
   |       +++++++++++++                                       ~ ++++++++++++++++++++++++++++++
error[E0277]: `<impl Iterator as Iterator>::Item` doesn't implement `Debug`
  --> /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:30:13
   |
   |
LL |         qux(constraint);
   |         --- ^^^^^^^^^^ `<impl Iterator as Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |         required by a bound introduced by this call
   |
   = help: the trait `Debug` is not implemented for `<impl Iterator as Iterator>::Item`
note: required by a bound in `qux`
note: required by a bound in `qux`
  --> /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:50:16
   |
LL | fn qux(_: impl std::fmt::Debug) {}
   |                ^^^^^^^^^^^^^^^ required by this bound in `qux`
help: introduce a type parameter with a trait bound instead of using `impl Trait`
   |
LL | fn bat<I, T: std::fmt::Debug, U: Iterator>(t: T, constraints: U, _: I), <U as Iterator>::Item: Debug {


error[E0277]: `<impl Iterator + std::fmt::Debug as Iterator>::Item` doesn't implement `Debug`
   |
   |
LL |         qux(constraint);
   |         --- ^^^^^^^^^^ `<impl Iterator + std::fmt::Debug as Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |         required by a bound introduced by this call
   |
   |
   = help: the trait `Debug` is not implemented for `<impl Iterator + std::fmt::Debug as Iterator>::Item`
note: required by a bound in `qux`
   |
   |
LL | fn qux(_: impl std::fmt::Debug) {}
   |                ^^^^^^^^^^^^^^^ required by this bound in `qux`
help: introduce a type parameter with a trait bound instead of using `impl Trait`
   |
LL | fn bak<I: Iterator + std::fmt::Debug>(constraints: I), <I as Iterator>::Item: Debug {
   |       +++++++++++++++++++++++++++++++              ~ ++++++++++++++++++++++++++++++
error[E0277]: `<impl Iterator as Iterator>::Item` doesn't implement `Debug`
  --> /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:45:13
   |
   |
LL |         qux(constraint);
   |         --- ^^^^^^^^^^ `<impl Iterator as Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |         required by a bound introduced by this call
   |
   = help: the trait `Debug` is not implemented for `<impl Iterator as Iterator>::Item`
note: required by a bound in `qux`
note: required by a bound in `qux`
  --> /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:50:16
   |
LL | fn qux(_: impl std::fmt::Debug) {}
   |                ^^^^^^^^^^^^^^^ required by this bound in `qux`
help: introduce a type parameter with a trait bound instead of using `impl Trait`
   |
LL | fn baw<I: Iterator>(constraints: I), <I as Iterator>::Item: Debug {
   |       ~~~~~~~~~~~~~              ~ ++++++++++++++++++++++++++++++
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
---

14    |                      ^^^^ required by this bound in `assert_is_send`
15 help: introduce a type parameter with a trait bound instead of using `impl Trait`
16    |
- LL | async fn run<F: Foo>(_: &(), foo: F) -> std::io::Result<()> where <F as Foo>::Bar: Send {
-    |             ++++++++              ~                         +++++++++++++++++++++++++++
+ LL | async fn run<F: Foo>(_: &(), foo: F) -> std::io::Result<()>, <F as Foo>::Bar: Send {
19 
19 
20 error[E0277]: `<impl Foo as Foo>::Bar` cannot be sent between threads safely

33    |                      ^^^^ required by this bound in `assert_is_send`
34 help: introduce a type parameter with a trait bound instead of using `impl Trait`
35    |
35    |
- LL | async fn run2<F: Foo>(_: &(), foo: F) -> std::io::Result<()> where <F as Foo>::Bar: Send {
-    |              ~~~~~~~~              ~                         +++++++++++++++++++++++++++
+ LL | async fn run2<F: Foo>(_: &(), foo: F) -> std::io::Result<()>, <F as Foo>::Bar: Send {
38 
39 error: aborting due to 2 previous errors
40 

---
To only update this specific test, also pass `--test-args suggestions/issue-79843-impl-trait-with-missing-bounds-on-async-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-79843-impl-trait-with-missing-bounds-on-async-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-79843-impl-trait-with-missing-bounds-on-async-fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-79843-impl-trait-with-missing-bounds-on-async-fn/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `<impl Foo as Foo>::Bar` cannot be sent between threads safely
   |
   |
LL |     assert_is_send(&bar);
   |     -------------- ^^^^ `<impl Foo as Foo>::Bar` cannot be sent between threads safely
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `Send` is not implemented for `<impl Foo as Foo>::Bar`
note: required by a bound in `assert_is_send`
   |
   |
LL | fn assert_is_send<T: Send>(_: &T) {}
   |                      ^^^^ required by this bound in `assert_is_send`
help: introduce a type parameter with a trait bound instead of using `impl Trait`
   |
LL | async fn run<F: Foo>(_: &(), foo: F) -> std::io::Result<()>, <F as Foo>::Bar: Send {


error[E0277]: `<impl Foo as Foo>::Bar` cannot be sent between threads safely
   |
   |
LL |     assert_is_send(&bar);
   |     -------------- ^^^^ `<impl Foo as Foo>::Bar` cannot be sent between threads safely
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `Send` is not implemented for `<impl Foo as Foo>::Bar`
note: required by a bound in `assert_is_send`
   |
   |
LL | fn assert_is_send<T: Send>(_: &T) {}
   |                      ^^^^ required by this bound in `assert_is_send`
help: introduce a type parameter with a trait bound instead of using `impl Trait`
   |
LL | async fn run2<F: Foo>(_: &(), foo: F) -> std::io::Result<()>, <F as Foo>::Bar: Send {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
---
diff of stderr:

10 help: introduce a type parameter with a trait bound instead of using `impl Trait`
11    |
12 LL ~ pub fn print_values<I: IntoIterator>(values: &I)
- LL ~ where <I as IntoIterator>::Item: std::fmt::Display {
+ LL ~ where, <I as IntoIterator>::Item: std::fmt::Display {
15 
16 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-97760/issue-97760.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/issue-97760.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-97760.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-97760" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-97760/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `<impl IntoIterator as IntoIterator>::Item` doesn't implement `std::fmt::Display`
   |
   |
LL |         println!("{x}");
   |                    ^ `<impl IntoIterator as IntoIterator>::Item` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `<impl IntoIterator as IntoIterator>::Item`
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: introduce a type parameter with a trait bound instead of using `impl Trait`
   |
   |
LL ~ pub fn print_values<I: IntoIterator>(values: &I)
LL ~ where, <I as IntoIterator>::Item: std::fmt::Display {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
---
19    = note: this error originates in the derive macro `Copy` (in Nightly builds, run with -Z macro-backtrace for more info)
- help: consider restricting type parameter `K`
+ help: consider further restricting type parameter `K`
21    |
- LL | pub struct AABB<K: Debug>{
-    |                  +++++++
+ LL | pub struct AABB<K>, K: Debug, K: Debug{
24 
25 error: aborting due to previous error
26 

---
To only update this specific test, also pass `--test-args suggestions/missing-bound-in-derive-copy-impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/missing-bound-in-derive-copy-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-bound-in-derive-copy-impl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-bound-in-derive-copy-impl/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0204]: the trait `Copy` may not be implemented for this type
   |
   |
LL | #[derive(Debug, Copy, Clone)] //~ ERROR the trait `Copy` may not be implemented for this type
   |                 ^^^^
LL | pub struct AABB<K>{
LL |     pub loc: Vector2<K>,
   |     ------------------- this field does not implement `Copy`
LL |     pub size: Vector2<K>
   |     -------------------- this field does not implement `Copy`
   |
note: the `Copy` impl for `Vector2<K>` requires that `K: Debug`
   |
   |
LL |     pub loc: Vector2<K>,
LL |     pub size: Vector2<K>
   |               ^^^^^^^^^^
   = note: this error originates in the derive macro `Copy` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider further restricting type parameter `K`
help: consider further restricting type parameter `K`
   |
LL | pub struct AABB<K>, K: Debug, K: Debug{

error: aborting due to previous error

For more information about this error, try `rustc --explain E0204`.
For more information about this error, try `rustc --explain E0204`.
------------------------------------------


---- [ui] src/test/ui/suggestions/suggest-change-mut.rs stdout ----
diff of stderr:

18    |
19 help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
20    |
- LL | fn issue_81421<T: Read + Write>(mut stream: T) where &T: std::io::Read {
-    |                                                +++++++++++++++++++++++
+ LL | fn issue_81421<T: Read + Write>(mut stream: T), &T: std::io::Read {
+    |                                               +++++++++++++++++++
23 help: consider changing this borrow's mutability
24    |
25 LL |         let mut stream_reader = BufReader::new(&mut stream);

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
LL | fn issue_81421<T: Read + Write>(mut stream: T), &T: std::io::Read { //~ HELP consider introducing a `where` clause
   |                                               +++++++++++++++++++
help: consider changing this borrow's mutability
   |
LL |         let mut stream_reader = BufReader::new(&mut stream);


error[E0599]: the method `read_until` exists for struct `BufReader<&T>`, but its trait bounds were not satisfied
   |
   |
LL |         stream_reader.read_until(b'\n', &mut buffer).expect("Reading into buffer failed");
   |                       ^^^^^^^^^^ method cannot be called on `BufReader<&T>` due to unsatisfied trait bounds
  ::: /checkout/library/std/src/io/buffered/bufreader.rs:50:1
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

23    |             ++++++++
24 help: consider restricting the type parameter to satisfy the trait bound
25    |
- LL | struct Victim<'a, T: Perpetrator + ?Sized> where Self: Sized {
-    |                                            +++++++++++++++++
+ LL | struct Victim<'a, T: Perpetrator + ?Sized>, Self: Sized {
28 
29 error: aborting due to previous error
30 



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
   | |
   | method `get` not found for this struct
   | method `get` not found for this struct
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
LL | struct Victim<'a, T: Perpetrator + ?Sized>, Self: Sized {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.
------------------------------------------


---- [ui] src/test/ui/traits/resolution-in-overloaded-op.rs stdout ----
diff of stderr:

8    |
9 help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
10    |
- LL | fn foo<T: MyMul<f64, f64>>(a: &T, b: f64) -> f64 where &T: Mul<f64> {
-    |                                                  ++++++++++++++++++
+ LL | fn foo<T: MyMul<f64, f64>>(a: &T, b: f64) -> f64, &T: Mul<f64> {
13 
14 error: aborting due to previous error
15 

---
To only update this specific test, also pass `--test-args traits/resolution-in-overloaded-op.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/resolution-in-overloaded-op.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/resolution-in-overloaded-op" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/resolution-in-overloaded-op/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: cannot multiply `&T` by `f64`
   |
   |
LL |     a * b //~ ERROR cannot multiply `&T` by `f64`
   |     - ^ - f64
   |     &T
   |
   |
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | fn foo<T: MyMul<f64, f64>>(a: &T, b: f64) -> f64, &T: Mul<f64> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0369`.
For more information about this error, try `rustc --explain E0369`.
------------------------------------------


---- [ui] src/test/ui/traits/suggest-where-clause.rs stdout ----
diff of stderr:

51    |
52 help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
53    |
- LL | fn check<T: Iterator, U: ?Sized>() where u64: From<T> {
-    |                                    ++++++++++++++++++
+ LL | fn check<T: Iterator, U: ?Sized>(), u64: From<T> {
56 
56 
57 error[E0277]: the trait bound `u64: From<<T as Iterator>::Item>` is not satisfied

62    |
62    |
63 help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
64    |
- LL | fn check<T: Iterator, U: ?Sized>() where u64: From<<T as Iterator>::Item> {
-    |                                    ++++++++++++++++++++++++++++++++++++++
+ LL | fn check<T: Iterator, U: ?Sized>(), u64: From<<T as Iterator>::Item> {
67 
67 
68 error[E0277]: the trait bound `Misc<_>: From<T>` is not satisfied


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-where-clause/suggest-where-clause.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-where-clause/suggest-where-clause.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/suggest-where-clause.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/suggest-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-where-clause" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-where-clause/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the size for values of type `U` cannot be known at compilation time
   |
   |
LL | fn check<T: Iterator, U: ?Sized>() {
   |                       - this type parameter needs to be `std::marker::Sized`
LL |     // suggest a where-clause, if needed
LL |     mem::size_of::<U>();
   |
note: required by a bound in `std::mem::size_of`
  --> /checkout/library/core/src/mem/mod.rs:314:22
   |
   |
LL | pub const fn size_of<T>() -> usize {
   |                      ^ required by this bound in `std::mem::size_of`
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL - fn check<T: Iterator, U: ?Sized>() {
LL + fn check<T: Iterator, U>() {

error[E0277]: the size for values of type `U` cannot be known at compilation time
  --> /checkout/src/test/ui/traits/suggest-where-clause.rs:10:5
   |
   |
LL | fn check<T: Iterator, U: ?Sized>() {
   |                       - this type parameter needs to be `std::marker::Sized`
...
LL |     mem::size_of::<Misc<U>>();
   |
   |
note: required because it appears within the type `Misc<U>`
   |
   |
LL | struct Misc<T:?Sized>(T);
note: required by a bound in `std::mem::size_of`
  --> /checkout/library/core/src/mem/mod.rs:314:22
   |
   |
LL | pub const fn size_of<T>() -> usize {
   |                      ^ required by this bound in `std::mem::size_of`
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL - fn check<T: Iterator, U: ?Sized>() {
LL + fn check<T: Iterator, U>() {


error[E0277]: the trait bound `u64: From<T>` is not satisfied
   |
   |
LL |     <u64 as From<T>>::from;
   |     ^^^^^^^^^^^^^^^^^^^^^^ the trait `From<T>` is not implemented for `u64`
   |
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | fn check<T: Iterator, U: ?Sized>(), u64: From<T> {


error[E0277]: the trait bound `u64: From<<T as Iterator>::Item>` is not satisfied
   |
   |
LL |     <u64 as From<<T as Iterator>::Item>>::from;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `From<<T as Iterator>::Item>` is not implemented for `u64`
   |
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | fn check<T: Iterator, U: ?Sized>(), u64: From<<T as Iterator>::Item> {


error[E0277]: the trait bound `Misc<_>: From<T>` is not satisfied
   |
   |
LL |     <Misc<_> as From<T>>::from;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `From<T>` is not implemented for `Misc<_>`

error[E0277]: the size for values of type `[T]` cannot be known at compilation time
   |
   |
LL |     mem::size_of::<[T]>();
   |
   = help: the trait `Sized` is not implemented for `[T]`
note: required by a bound in `std::mem::size_of`
  --> /checkout/library/core/src/mem/mod.rs:314:22
  --> /checkout/library/core/src/mem/mod.rs:314:22
   |
LL | pub const fn size_of<T>() -> usize {
   |                      ^ required by this bound in `std::mem::size_of`

error[E0277]: the size for values of type `[&U]` cannot be known at compilation time
   |
   |
LL |     mem::size_of::<[&U]>();
   |
   = help: the trait `Sized` is not implemented for `[&U]`
note: required by a bound in `std::mem::size_of`
  --> /checkout/library/core/src/mem/mod.rs:314:22
  --> /checkout/library/core/src/mem/mod.rs:314:22
   |
LL | pub const fn size_of<T>() -> usize {
   |                      ^ required by this bound in `std::mem::size_of`
error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/type-alias-impl-trait/multiple-def-uses-in-one-fn.rs stdout ----
diff of stderr:

7    = note: required because of the requirements on the impl of `Into<&'static B>` for `&A`
8 help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
9    |
- LL | fn f<A, B: 'static>(a: &'static A, b: B) -> (X<A, B>, X<B, A>) where &'static B: From<&A> {
-    |                                                                ++++++++++++++++++++++++++
+ LL | fn f<A, B: 'static>(a: &'static A, b: B) -> (X<A, B>, X<B, A>), &'static B: From<&A> {
12 
13 error: aborting due to previous error
14 

---
To only update this specific test, also pass `--test-args type-alias-impl-trait/multiple-def-uses-in-one-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/multiple-def-uses-in-one-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/multiple-def-uses-in-one-fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/multiple-def-uses-in-one-fn/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `&'static B: From<&A>` is not satisfied
   |
   |
LL | fn f<A, B: 'static>(a: &'static A, b: B) -> (X<A, B>, X<B, A>) {
   |                                             ^^^^^^^^^^^^^^^^^^ the trait `From<&A>` is not implemented for `&'static B`
   |
   = note: required because of the requirements on the impl of `Into<&'static B>` for `&A`
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | fn f<A, B: 'static>(a: &'static A, b: B) -> (X<A, B>, X<B, A>), &'static B: From<&A> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
---

12    |              ^^^^ required by this bound in `is_send`
13 help: consider further restricting the associated type
14    |
- LL | fn bar<T:Trait+Send>() where <T as Trait>::AssocType: Send {
-    |                        +++++++++++++++++++++++++++++++++++
+ LL | fn bar<T:Trait+Send>(), <T as Trait>::AssocType: Send {
17 
18 error: aborting due to previous error
19 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck-default-trait-impl-assoc-type/typeck-default-trait-impl-assoc-type.stderr
diff of fixed:

7     type AssocType;
8     fn dummy(&self) { }
9 }
- fn bar<T:Trait+Send>() where <T as Trait>::AssocType: Send {
+ fn bar<T:Trait+Send>(), <T as Trait>::AssocType: Send {
11     is_send::<T::AssocType>(); //~ ERROR E0277
13 


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck-default-trait-impl-assoc-type/typeck-default-trait-impl-assoc-type.fixed
To only update this specific test, also pass `--test-args typeck/typeck-default-trait-impl-assoc-type.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/typeck-default-trait-impl-assoc-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck-default-trait-impl-assoc-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck-default-trait-impl-assoc-type/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `<T as Trait>::AssocType` cannot be sent between threads safely
   |
   |
LL |     is_send::<T::AssocType>(); //~ ERROR E0277
   |               ^^^^^^^^^^^^ `<T as Trait>::AssocType` cannot be sent between threads safely
   |
   = help: the trait `Send` is not implemented for `<T as Trait>::AssocType`
note: required by a bound in `is_send`
   |
   |
LL | fn is_send<T:Send>() {
   |              ^^^^ required by this bound in `is_send`
   |
   |
LL | fn bar<T:Trait+Send>(), <T as Trait>::AssocType: Send {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
