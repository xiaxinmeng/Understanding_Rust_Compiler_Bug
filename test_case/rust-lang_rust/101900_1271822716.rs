plain
.........................................ii............................................. 88/13651
..............................................................................iiiiiiiiii 176/13651
iiii.....................i..................i........................................... 264/13651
........................................................................................ 352/13651
.......F.F......................................................F....................... 440/13651
........................................................................................ 616/13651
........................................................................................ 704/13651
...................................................................................i.... 792/13651
........................................................................................ 880/13651
---
........................................................................................ 6600/13651
....................................................i................................... 6688/13651
........................................................................................ 6776/13651
.............................i........................................................ii 6864/13651
.ii........i....i.........................F...........F.............................i... 6952/13651
...............................................................i....i................... 7128/13651
......................i..................i.............i................................ 7216/13651
............................i........................................................... 7304/13651
..................................................i..................................... 7392/13651
---

---- [ui] src/test/ui/associated-types/associated-type-projection-ambig-between-bound-and-where-clause.rs stdout ----
diff of stderr:

- error[E0221]: ambiguous associated type `Color` in bounds of `C`
-   --> $DIR/associated-type-projection-ambig-between-bound-and-where-clause.rs:16:24
+ error[E0392]: parameter `X` is never used
3    |
- LL |     type Color;
- LL |     type Color;
-    |     ---------- ambiguous `Color` from `Vehicle`
- LL |     type Color;
- LL |     type Color;
-    |     ---------- ambiguous `Color` from `Box`
- ...
- LL | fn a<C:Vehicle+Box>(_: C::Color) {
-    |                        ^^^^^^^^ ambiguous associated type `Color`
+ LL | struct D<X>;
+    |          ^ unused parameter
- help: use fully qualified syntax to disambiguate
-    |
-    |
- LL | fn a<C:Vehicle+Box>(_: <C as Box>::Color) {
- help: use fully qualified syntax to disambiguate
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |
-    |
- LL | fn a<C:Vehicle+Box>(_: <C as Vehicle>::Color) {
-    |                        ~~~~~~~~~~~~~~~~
+    = help: consider removing `X`, referring to it in a field, or using a marker such as `PhantomData`
+    = help: if you intended `X` to be a const parameter, use `const X: usize` instead
21 
- error[E0221]: ambiguous associated type `Color` in bounds of `C`
-   --> $DIR/associated-type-projection-ambig-between-bound-and-where-clause.rs:20:12
- LL |     type Color;
- LL |     type Color;
-    |     ---------- ambiguous `Color` from `Vehicle`
- LL |     type Color;
- LL |     type Color;
-    |     ---------- ambiguous `Color` from `Box`
- ...
- LL | fn b<C>(_: C::Color) where C : Vehicle+Box {
-    |            ^^^^^^^^ ambiguous associated type `Color`
- help: use fully qualified syntax to disambiguate
-    |
-    |
- LL | fn b<C>(_: <C as Box>::Color) where C : Vehicle+Box {
- help: use fully qualified syntax to disambiguate
-    |
-    |
- LL | fn b<C>(_: <C as Vehicle>::Color) where C : Vehicle+Box {
+ error: aborting due to previous error
42 
42 
- error[E0221]: ambiguous associated type `Color` in bounds of `C`
-   --> $DIR/associated-type-projection-ambig-between-bound-and-where-clause.rs:24:12
- LL |     type Color;
- LL |     type Color;
-    |     ---------- ambiguous `Color` from `Vehicle`
- LL |     type Color;
- LL |     type Color;
-    |     ---------- ambiguous `Color` from `Box`
- ...
- LL | fn c<C>(_: C::Color) where C : Vehicle, C : Box {
-    |            ^^^^^^^^ ambiguous associated type `Color`
- help: use fully qualified syntax to disambiguate
-    |
-    |
- LL | fn c<C>(_: <C as Box>::Color) where C : Vehicle, C : Box {
- help: use fully qualified syntax to disambiguate
-    |
-    |
- LL | fn c<C>(_: <C as Vehicle>::Color) where C : Vehicle, C : Box {
- 
- 
- error[E0221]: ambiguous associated type `Color` in bounds of `X`
-   --> $DIR/associated-type-projection-ambig-between-bound-and-where-clause.rs:35:20
- LL |     type Color;
- LL |     type Color;
-    |     ---------- ambiguous `Color` from `Vehicle`
- LL |     type Color;
- LL |     type Color;
-    |     ---------- ambiguous `Color` from `Box`
- ...
- LL |     fn e(&self, _: X::Color) where X : Box;
-    |                    ^^^^^^^^ ambiguous associated type `Color`
- help: use fully qualified syntax to disambiguate
-    |
-    |
- LL |     fn e(&self, _: <X as Box>::Color) where X : Box;
- help: use fully qualified syntax to disambiguate
-    |
-    |
- LL |     fn e(&self, _: <X as Vehicle>::Color) where X : Box;
- 
- 
- error[E0221]: ambiguous associated type `Color` in bounds of `X`
-   --> $DIR/associated-type-projection-ambig-between-bound-and-where-clause.rs:38:20
- LL |     type Color;
- LL |     type Color;
-    |     ---------- ambiguous `Color` from `Vehicle`
- LL |     type Color;
- LL |     type Color;
-    |     ---------- ambiguous `Color` from `Box`
- ...
- LL |     fn f(&self, _: X::Color) where X : Box { }
-    |                    ^^^^^^^^ ambiguous associated type `Color`
- help: use fully qualified syntax to disambiguate
-    |
-    |
- LL |     fn f(&self, _: <X as Box>::Color) where X : Box { }
- help: use fully qualified syntax to disambiguate
-    |
-    |
- LL |     fn f(&self, _: <X as Vehicle>::Color) where X : Box { }
- 
- 
- error[E0221]: ambiguous associated type `Color` in bounds of `X`
-   --> $DIR/associated-type-projection-ambig-between-bound-and-where-clause.rs:30:20
- LL |     type Color;
- LL |     type Color;
-    |     ---------- ambiguous `Color` from `Vehicle`
- LL |     type Color;
- LL |     type Color;
-    |     ---------- ambiguous `Color` from `Box`
- ...
- LL |     fn d(&self, _: X::Color) where X : Box { }
-    |                    ^^^^^^^^ ambiguous associated type `Color`
- help: use fully qualified syntax to disambiguate
-    |
-    |
- LL |     fn d(&self, _: <X as Box>::Color) where X : Box { }
- help: use fully qualified syntax to disambiguate
-    |
-    |
- LL |     fn d(&self, _: <X as Vehicle>::Color) where X : Box { }
- 
- error: aborting due to 6 previous errors
- 
- For more information about this error, try `rustc --explain E0221`.
---
To only update this specific test, also pass `--test-args associated-types/associated-type-projection-ambig-between-bound-and-where-clause.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-type-projection-ambig-between-bound-and-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-type-projection-ambig-between-bound-and-where-clause" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-type-projection-ambig-between-bound-and-where-clause/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0392]: parameter `X` is never used
   |
   |
LL | struct D<X>;
   |          ^ unused parameter
   |
   = help: consider removing `X`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `X` to be a const parameter, use `const X: usize` instead
error: aborting due to previous error

For more information about this error, try `rustc --explain E0392`.
------------------------------------------
---

6    |
7    = note: see issue #20041 <https://github.com/rust-lang/rust/issues/20041> for more information
8 
- error[E0221]: ambiguous associated type `Color` in bounds of `C`
-   --> $DIR/associated-type-projection-from-multiple-supertraits.rs:19:32
- LL |     type Color;
- LL |     type Color;
-    |     ---------- ambiguous `Color` from `Vehicle`
- LL |     type Color;
- LL |     type Color;
-    |     ---------- ambiguous `Color` from `Box`
- ...
- LL | fn dent<C:BoxCar>(c: C, color: C::Color) {
-    |                                ^^^^^^^^ ambiguous associated type `Color`
- help: use fully qualified syntax to disambiguate
-    |
-    |
- LL | fn dent<C:BoxCar>(c: C, color: <C as Vehicle>::Color) {
- help: use fully qualified syntax to disambiguate
-    |
-    |
- LL | fn dent<C:BoxCar>(c: C, color: <C as Box>::Color) {
- 
- 
- error[E0222]: ambiguous associated type `Color` in bounds of `BoxCar`
-   --> $DIR/associated-type-projection-from-multiple-supertraits.rs:23:37
- LL |     type Color;
- LL |     type Color;
-    |     ---------- ambiguous `Color` from `Vehicle`
- LL |     type Color;
- LL |     type Color;
-    |     ---------- ambiguous `Color` from `Box`
- ...
- LL | fn dent_object<COLOR>(c: dyn BoxCar<Color=COLOR>) {
-    |                                     ^^^^^^^^^^^ ambiguous associated type `Color`
-    |
-    = help: consider introducing a new type parameter `T` and adding `where` constraints:
-                where
-                    T: BoxCar,
-                    T: Vehicle::Color = COLOR,
-                    T: Box::Color = COLOR
- 
- error[E0191]: the value of the associated types `Color` (from trait `Box`), `Color` (from trait `Vehicle`) must be specified
+ error[E0191]: the value of the associated type `Color` (from trait `Box`) must be specified
50    |
51 LL |     type Color;


-    |     ---------- `Vehicle::Color` defined here
+    |     ---------- `Color` defined here
- LL |     type Color;
- LL |     type Color;
-    |     ---------- `Box::Color` defined here
- ...
57 LL | fn dent_object<COLOR>(c: dyn BoxCar<Color=COLOR>) {
-    |                              ^^^^^^^^^^^^^^^^^^^ associated types `Color` (from trait `Vehicle`), `Color` (from trait `Box`) must be specified
-    |
-    = help: consider introducing a new type parameter, adding `where` constraints using the fully-qualified path to the associated types
+    |                              ^^^^^^^^^^^^^^^^^^^ help: specify the associated type: `BoxCar<Color=COLOR, Color = Type>`
61 
- error[E0221]: ambiguous associated type `Color` in bounds of `C`
-   --> $DIR/associated-type-projection-from-multiple-supertraits.rs:28:29
- LL |     type Color;
- LL |     type Color;
-    |     ---------- ambiguous `Color` from `Vehicle`
- LL |     type Color;
- LL |     type Color;
-    |     ---------- ambiguous `Color` from `Box`
- ...
- LL | fn paint<C:BoxCar>(c: C, d: C::Color) {
-    |                             ^^^^^^^^ ambiguous associated type `Color`
- help: use fully qualified syntax to disambiguate
-    |
-    |
- LL | fn paint<C:BoxCar>(c: C, d: <C as Vehicle>::Color) {
- help: use fully qualified syntax to disambiguate
-    |
-    |
- LL | fn paint<C:BoxCar>(c: C, d: <C as Box>::Color) {
- 
- 
83 error[E0191]: the value of the associated types `Color` (from trait `Box`), `Color` (from trait `Vehicle`) must be specified
85    |

94    |
94    |
95    = help: consider introducing a new type parameter, adding `where` constraints using the fully-qualified path to the associated types
- error: aborting due to 6 previous errors
+ error: aborting due to 3 previous errors
98 
- Some errors have detailed explanations: E0191, E0221, E0222.
---
To only update this specific test, also pass `--test-args associated-types/associated-type-projection-from-multiple-supertraits.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-type-projection-from-multiple-supertraits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-type-projection-from-multiple-supertraits" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-type-projection-from-multiple-supertraits/auxiliary"
stdout: none
--- stderr -------------------------------
error: equality constraints are not yet supported in `where` clauses
   |
   |
LL | fn dent_object_2<COLOR>(c: dyn BoxCar) where <dyn BoxCar as Vehicle>::Color = COLOR {
   |
   = note: see issue #20041 <https://github.com/rust-lang/rust/issues/20041> for more information


error[E0191]: the value of the associated type `Color` (from trait `Box`) must be specified
   |
LL |     type Color;
   |     ---------- `Color` defined here
...
...
LL | fn dent_object<COLOR>(c: dyn BoxCar<Color=COLOR>) {
   |                              ^^^^^^^^^^^^^^^^^^^ help: specify the associated type: `BoxCar<Color=COLOR, Color = Type>`

error[E0191]: the value of the associated types `Color` (from trait `Box`), `Color` (from trait `Vehicle`) must be specified
   |
LL |     type Color;
LL |     type Color;
   |     ---------- `Vehicle::Color` defined here
LL |     type Color;
LL |     type Color;
   |     ---------- `Box::Color` defined here
...
LL | fn dent_object_2<COLOR>(c: dyn BoxCar) where <dyn BoxCar as Vehicle>::Color = COLOR {
   |                                ^^^^^^ associated types `Color` (from trait `Vehicle`), `Color` (from trait `Box`) must be specified
   |
   = help: consider introducing a new type parameter, adding `where` constraints using the fully-qualified path to the associated types
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0191`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/associated-types/associated-types-path-1.rs stdout ----
diff of stderr:

4 LL | pub fn f1<T>(a: T, x: T::A) {}
5    |                          ^ associated type `A` not found
6 
- error[E0221]: ambiguous associated type `A` in bounds of `T`
-   --> $DIR/associated-types-path-1.rs:11:34
- LL |     type A;
- LL |     type A;
-    |     ------ ambiguous `A` from `Foo`
- LL |     type A;
- LL |     type A;
-    |     ------ ambiguous `A` from `Bar`
- ...
- LL | pub fn f2<T: Foo + Bar>(a: T, x: T::A) {}
-    |                                  ^^^^ ambiguous associated type `A`
- help: use fully qualified syntax to disambiguate
-    |
-    |
- LL | pub fn f2<T: Foo + Bar>(a: T, x: <T as Bar>::A) {}
- help: use fully qualified syntax to disambiguate
-    |
-    |
- LL | pub fn f2<T: Foo + Bar>(a: T, x: <T as Foo>::A) {}
+ error: aborting due to previous error
27 
- error: aborting due to 2 previous errors
- 
- 
- Some errors have detailed explanations: E0220, E0221.
- For more information about an error, try `rustc --explain E0220`.
+ For more information about this error, try `rustc --explain E0220`.
32 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-path-1/associated-types-path-1.stderr
To only update this specific test, also pass `--test-args associated-types/associated-types-path-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-path-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-path-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-path-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0220]: associated type `A` not found for `T`
   |
   |
LL | pub fn f1<T>(a: T, x: T::A) {} //~ERROR associated type `A` not found
   |                          ^ associated type `A` not found
error: aborting due to previous error

For more information about this error, try `rustc --explain E0220`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/operator_traits.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/operator_traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/operator_traits" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/operator_traits/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `Function: ~const Fn<(&mut &'a mut A, _), _>` is not satisfied
   |
   |
LL |                 (self.func)(($($var),*), args)
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `~const Fn<(&mut &'a mut A, _), _>` is not implemented for `Function`
...
LL | impl_fn_mut_tuple!(A);
   |
   = note: this error originates in the macro `impl_fn_mut_tuple` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
---
---- [ui] src/test/ui/error-codes/E0221.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0221.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0221" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0221/auxiliary"
stdout: none
stderr: none


---- [ui] src/test/ui/keyword_generics/effects_are_not_user_visible_generic_params.rs stdout ----
normalized stderr:
error[E0107]: this trait takes 0 generic arguments but 1 generic argument was supplied
  --> $DIR/effects_are_not_user_visible_generic_params.rs:10:11
   |
LL | fn foo<T: Foo<()>>() {}
   |           ^^^---- help: remove these generics
   |           expected 0 generic arguments
   |
   |
note: trait defined here, with 0 generic parameters
  --> $DIR/effects_are_not_user_visible_generic_params.rs:6:7
LL | trait Foo {
   |       ^^^

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0107`.



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/keyword_generics/effects_are_not_user_visible_generic_params/effects_are_not_user_visible_generic_params.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args keyword_generics/effects_are_not_user_visible_generic_params.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/keyword_generics/effects_are_not_user_visible_generic_params.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/keyword_generics/effects_are_not_user_visible_generic_params" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/keyword_generics/effects_are_not_user_visible_generic_params/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0107]: this trait takes 0 generic arguments but 1 generic argument was supplied
  --> /checkout/src/test/ui/keyword_generics/effects_are_not_user_visible_generic_params.rs:10:11
   |
LL | fn foo<T: Foo<()>>() {}
   |           ^^^---- help: remove these generics
   |           expected 0 generic arguments
   |
   |
note: trait defined here, with 0 generic parameters
  --> /checkout/src/test/ui/keyword_generics/effects_are_not_user_visible_generic_params.rs:6:7
LL | trait Foo {
   |       ^^^

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0107`.
------------------------------------------


---- [ui] src/test/ui/keyword_generics/feature-gate-keyword_generics.rs stdout ----

1 error[E0308]: mismatched types
1 error[E0308]: mismatched types
-   --> $DIR/feature-gate-keyword_generics.rs:5:5
+   --> $DIR/feature-gate-keyword_generics.rs:6:5
4 LL | fn main() {
4 LL | fn main() {
5    |           - expected `()` because of default return type

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/keyword_generics/feature-gate-keyword_generics/feature-gate-keyword_generics.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args keyword_generics/feature-gate-keyword_generics.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/keyword_generics/feature-gate-keyword_generics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/keyword_generics/feature-gate-keyword_generics" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/keyword_generics/feature-gate-keyword_generics/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/keyword_generics/feature-gate-keyword_generics.rs:6:5
LL | fn main() {
LL | fn main() {
   |           - expected `()` because of default return type
LL |     "just fail this test" //~ ERROR mismatched types
   |     ^^^^^^^^^^^^^^^^^^^^^ expected `()`, found `&str`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
---
    [ui] src/test/ui/associated-types/associated-type-projection-from-multiple-supertraits.rs
    [ui] src/test/ui/associated-types/associated-types-path-1.rs
    [ui] src/test/ui/consts/operator_traits.rs
    [ui] src/test/ui/error-codes/E0221.rs
    [ui] src/test/ui/keyword_generics/effects_are_not_user_visible_generic_params.rs
    [ui] src/test/ui/keyword_generics/feature-gate-keyword_generics.rs
test result: FAILED. 13519 passed; 7 failed; 125 ignored; 0 measured; 0 filtered out; finished in 109.55s

Build completed unsuccessfully in 0:10:28
