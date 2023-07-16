plain
........................................................................................ 13552/13566
..............
failures:

---- [ui] src/test/ui/keyword_generics/const_trait.rs stdout ----


- error[E0277]: the trait bound `A: ~const Foo<_>` is not satisfied
-   --> $DIR/const_trait.rs:23:5
-    |
- LL |     A::bar();
-    |     ^^^^^^ the trait `~const Foo<_>` is not implemented for `A`
-    |
- note: the trait `Foo<_>` is implemented for `A`, but that implementation is not `const`
-   --> $DIR/const_trait.rs:23:5
-    |
- LL |     A::bar();
- 
- 
13 error[E0277]: the trait bound `A: ~const Foo<!const>` is not satisfied
15    |


27 LL | const fn boo<T: ~const Foo>() {
28    |                 ^^^^^^^^^^ required by this bound in `boo`
29 
- error[E0277]: the trait bound `A: ~const Foo<_>` is not satisfied
-   --> $DIR/const_trait.rs:33:5
-    |
- LL |     A::bar();
-    |     ^^^^^^ the trait `~const Foo<_>` is not implemented for `A`
-    |
- note: the trait `Foo<_>` is implemented for `A`, but that implementation is not `const`
-   --> $DIR/const_trait.rs:33:5
-    |
- LL |     A::bar();
- 
- 
42 error[E0277]: the trait bound `A: ~const Foo<!const>` is not satisfied
44    |


56 LL | const fn boo<T: ~const Foo>() {
57    |                 ^^^^^^^^^^ required by this bound in `boo`
58 
- error[E0277]: the trait bound `A: ~const Foo<_>` is not satisfied
-   --> $DIR/const_trait.rs:43:5
-    |
- LL |     A::bar();
-    |     ^^^^^^ the trait `~const Foo<_>` is not implemented for `A`
-    |
- note: the trait `Foo<_>` is implemented for `A`, but that implementation is not `const`
-   --> $DIR/const_trait.rs:43:5
-    |
- LL |     A::bar();
-    |     ^^^^^^
- help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
-    |
- LL | const fn foo() where A: ~const Foo<_> {
- 
- 
75 error[E0277]: the trait bound `A: ~const Foo<!const>` is not satisfied
77    |


93 LL | const fn foo() where A: ~const Foo<!const> {
95 
95 
- error[E0277]: the trait bound `T: ~const Foo<_>` is not satisfied
-   --> $DIR/const_trait.rs:52:5
-    |
- LL |     T::bar();
-    |     ^^^^^^ the trait `~const Foo<_>` is not implemented for `T`
-    |
- note: the trait `Foo<_>` is implemented for `T`, but that implementation is not `const`
-   --> $DIR/const_trait.rs:52:5
-    |
- LL |     T::bar();
- help: consider further restricting this bound
-    |
-    |
- LL | const fn moo<T: Foo + ~const Foo<_>>() {
- 
- 
- error[E0277]: the trait bound `A: ~const Foo<_>` is not satisfied
-   --> $DIR/const_trait.rs:69:9
-    |
- LL |         A::bar();
-    |         ^^^^^^ the trait `~const Foo<_>` is not implemented for `A`
-    |
- note: the trait `Foo<_>` is implemented for `A`, but that implementation is not `const`
-   --> $DIR/const_trait.rs:69:9
-    |
- LL |         A::bar();
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
-    |
- LL | fn main() where A: ~const Foo<_> {
- 
- 
128 error[E0277]: the trait bound `A: ~const Foo<!const>` is not satisfied
130    |


146 LL | fn main() where A: ~const Foo<!const> {
148 
148 
- error[E0277]: the trait bound `A: ~const Foo<_>` is not satisfied
-   --> $DIR/const_trait.rs:79:9
-    |
- LL |         A::bar();
-    |         ^^^^^^ the trait `~const Foo<_>` is not implemented for `A`
-    |
- note: the trait `Foo<_>` is implemented for `A`, but that implementation is not `const`
-   --> $DIR/const_trait.rs:79:9
-    |
- LL |         A::bar();
-    |         ^^^^^^
- help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
-    |
- LL | fn main() where A: ~const Foo<_> {
- 
- 
165 error[E0277]: the trait bound `A: ~const Foo<!const>` is not satisfied
167    |


183 LL | fn main() where A: ~const Foo<!const> {
185 
- error: aborting due to 11 previous errors
+ error: aborting due to 5 previous errors
187 
---
To only update this specific test, also pass `--test-args keyword_generics/const_trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/keyword_generics/const_trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/keyword_generics/const_trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/keyword_generics/const_trait/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `A: ~const Foo<!const>` is not satisfied
  --> /checkout/src/test/ui/keyword_generics/const_trait.rs:28:11
   |
LL |     boo::<A>(); //~ ERROR: the trait bound `A: ~const Foo<!const>` is not satisfied
   |           ^ the trait `~const Foo<!const>` is not implemented for `A`
   |
note: the trait `Foo<!const>` is implemented for `A`, but that implementation is not `const`
  --> /checkout/src/test/ui/keyword_generics/const_trait.rs:28:11
   |
LL |     boo::<A>(); //~ ERROR: the trait bound `A: ~const Foo<!const>` is not satisfied
note: required by a bound in `boo`
  --> /checkout/src/test/ui/keyword_generics/const_trait.rs:55:17
   |
   |
LL | const fn boo<T: ~const Foo>() {
   |                 ^^^^^^^^^^ required by this bound in `boo`

error[E0277]: the trait bound `A: ~const Foo<!const>` is not satisfied
  --> /checkout/src/test/ui/keyword_generics/const_trait.rs:38:11
   |
LL |     boo::<A>(); //~ ERROR: the trait bound `A: ~const Foo<!const>` is not satisfied
   |           ^ the trait `~const Foo<!const>` is not implemented for `A`
   |
note: the trait `Foo<!const>` is implemented for `A`, but that implementation is not `const`
  --> /checkout/src/test/ui/keyword_generics/const_trait.rs:38:11
   |
LL |     boo::<A>(); //~ ERROR: the trait bound `A: ~const Foo<!const>` is not satisfied
note: required by a bound in `boo`
  --> /checkout/src/test/ui/keyword_generics/const_trait.rs:55:17
   |
   |
LL | const fn boo<T: ~const Foo>() {
   |                 ^^^^^^^^^^ required by this bound in `boo`

error[E0277]: the trait bound `A: ~const Foo<!const>` is not satisfied
  --> /checkout/src/test/ui/keyword_generics/const_trait.rs:47:11
   |
LL |     boo::<A>(); //~ ERROR: the trait bound `A: ~const Foo<!const>` is not satisfied
   |           ^ the trait `~const Foo<!const>` is not implemented for `A`
   |
note: the trait `Foo<!const>` is implemented for `A`, but that implementation is not `const`
  --> /checkout/src/test/ui/keyword_generics/const_trait.rs:47:11
   |
LL |     boo::<A>(); //~ ERROR: the trait bound `A: ~const Foo<!const>` is not satisfied
note: required by a bound in `boo`
  --> /checkout/src/test/ui/keyword_generics/const_trait.rs:55:17
   |
   |
LL | const fn boo<T: ~const Foo>() {
   |                 ^^^^^^^^^^ required by this bound in `boo`
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | const fn foo() where A: ~const Foo<!const> {


error[E0277]: the trait bound `A: ~const Foo<!const>` is not satisfied
  --> /checkout/src/test/ui/keyword_generics/const_trait.rs:74:15
   |
LL |         boo::<A>(); //~ ERROR: the trait bound `A: ~const Foo<!const>` is not satisfied
   |               ^ the trait `~const Foo<!const>` is not implemented for `A`
   |
note: the trait `Foo<!const>` is implemented for `A`, but that implementation is not `const`
  --> /checkout/src/test/ui/keyword_generics/const_trait.rs:74:15
   |
LL |         boo::<A>(); //~ ERROR: the trait bound `A: ~const Foo<!const>` is not satisfied
note: required by a bound in `boo`
  --> /checkout/src/test/ui/keyword_generics/const_trait.rs:55:17
   |
   |
LL | const fn boo<T: ~const Foo>() {
   |                 ^^^^^^^^^^ required by this bound in `boo`
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | fn main() where A: ~const Foo<!const> {


error[E0277]: the trait bound `A: ~const Foo<!const>` is not satisfied
  --> /checkout/src/test/ui/keyword_generics/const_trait.rs:84:15
   |
LL |         boo::<A>(); //~ ERROR: the trait bound `A: ~const Foo<!const>` is not satisfied
   |               ^ the trait `~const Foo<!const>` is not implemented for `A`
   |
note: the trait `Foo<!const>` is implemented for `A`, but that implementation is not `const`
  --> /checkout/src/test/ui/keyword_generics/const_trait.rs:84:15
   |
LL |         boo::<A>(); //~ ERROR: the trait bound `A: ~const Foo<!const>` is not satisfied
note: required by a bound in `boo`
  --> /checkout/src/test/ui/keyword_generics/const_trait.rs:55:17
   |
   |
LL | const fn boo<T: ~const Foo>() {
   |                 ^^^^^^^^^^ required by this bound in `boo`
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | fn main() where A: ~const Foo<!const> {

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0277`.
