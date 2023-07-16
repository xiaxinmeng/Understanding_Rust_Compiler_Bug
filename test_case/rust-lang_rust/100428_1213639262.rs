plain

140 note: the following traits must be implemented
141   --> $SRC_DIR/core/src/num/nonzero.rs:LL:COL
142    |
- LL | pub trait ZeroablePrimitive: private::Sealed {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | pub trait ZeroablePrimitive: Sized + Copy + private::Sealed {
145    |
146   ::: $SRC_DIR/core/src/ops/bit.rs:LL:COL
147    |

---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/issue-28837.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-28837" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-28837/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: cannot add `A` to `A`
   |
   |
LL |     a + a; //~ ERROR cannot add `A` to `A`
   |     - ^ - A
   |     A
   |
   |
note: an implementation of `Add<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `Add<_>`
  --> /checkout/library/core/src/ops/arith.rs:100:1
   |
   |
LL | pub trait Add<Rhs = Self> {


error[E0369]: cannot subtract `A` from `A`
   |
   |
LL |     a - a; //~ ERROR cannot subtract `A` from `A`
   |     - ^ - A
   |     A
   |
   |
note: an implementation of `Sub<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `Sub<_>`
  --> /checkout/library/core/src/ops/arith.rs:207:1
   |
   |
LL | pub trait Sub<Rhs = Self> {


error[E0369]: cannot multiply `A` by `A`
   |
   |
LL |     a * a; //~ ERROR cannot multiply `A` by `A`
   |     - ^ - A
   |     A
   |
   |
note: an implementation of `Mul<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `Mul<_>`
  --> /checkout/library/core/src/ops/arith.rs:336:1
   |
   |
LL | pub trait Mul<Rhs = Self> {

error[E0369]: cannot divide `A` by `A`
  --> /checkout/src/test/ui/binop/issue-28837.rs:12:7
   |
   |
LL |     a / a; //~ ERROR cannot divide `A` by `A`
   |     - ^ - A
   |     A
   |
   |
note: an implementation of `Div<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `Div<_>`
  --> /checkout/library/core/src/ops/arith.rs:469:1
   |
   |
LL | pub trait Div<Rhs = Self> {


error[E0369]: cannot mod `A` by `A`
   |
   |
LL |     a % a; //~ ERROR cannot mod `A` by `A`
   |     - ^ - A
   |     A
   |
   |
note: an implementation of `Rem<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `Rem<_>`
  --> /checkout/library/core/src/ops/arith.rs:571:1
   |
   |
LL | pub trait Rem<Rhs = Self> {


error[E0369]: no implementation for `A & A`
   |
   |
LL |     a & a; //~ ERROR no implementation for `A & A`
   |     - ^ - A
   |     A
   |
   |
note: an implementation of `BitAnd<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `BitAnd<_>`
  --> /checkout/library/core/src/ops/bit.rs:146:1
   |
   |
LL | pub trait BitAnd<Rhs = Self> {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/binop/issue-28837.rs:18:9
   |
   |
LL |     a | a;
   |         ^ expected struct `NonZero`, found struct `A`
   |
   = note: expected struct `NonZero<A>`
              found struct `A`

error[E0369]: no implementation for `A | NonZero<A>`
   |
   |
LL |     a | a;
   |     - ^ - NonZero<A>
   |     A
   |
   |
note: the following type would have to `impl` its required traits for this operation to be valid
   |
LL | struct A;
   | ^^^^^^^^
   | |
   | |
   | must implement `core::num::nonzero::ZeroablePrimitive`
   | must implement `BitOr`
  --> /checkout/library/core/src/num/nonzero.rs:31:1
   |
   |
LL | pub trait ZeroablePrimitive: Sized + Copy + private::Sealed {
   |
  ::: /checkout/library/core/src/ops/bit.rs:247:1
   |
   |
LL | pub trait BitOr<Rhs = Self> {


error[E0369]: no implementation for `A << A`
   |
   |
LL |     a << a; //~ ERROR no implementation for `A << A`
   |     - ^^ - A
   |     A
   |
   |
note: an implementation of `Shl<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `Shl<_>`
  --> /checkout/library/core/src/ops/bit.rs:448:1
   |
   |
LL | pub trait Shl<Rhs = Self> {


error[E0369]: no implementation for `A >> A`
   |
   |
LL |     a >> a; //~ ERROR no implementation for `A >> A`
   |     - ^^ - A
   |     A
   |
   |
note: an implementation of `Shr<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `Shr<_>`
  --> /checkout/library/core/src/ops/bit.rs:567:1
   |
   |
LL | pub trait Shr<Rhs = Self> {


error[E0369]: binary operation `==` cannot be applied to type `A`
   |
   |
LL |     a == a; //~ ERROR binary operation `==` cannot be applied to type `A`
   |     - ^^ - A
   |     A
   |
   |
note: an implementation of `PartialEq<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `PartialEq<_>`
help: consider annotating `A` with `#[derive(PartialEq)]`
   |
LL | #[derive(PartialEq)]


error[E0369]: binary operation `!=` cannot be applied to type `A`
   |
   |
LL |     a != a; //~ ERROR binary operation `!=` cannot be applied to type `A`
   |     - ^^ - A
   |     A
   |
   |
note: an implementation of `PartialEq<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `PartialEq<_>`
help: consider annotating `A` with `#[derive(PartialEq)]`
   |
LL | #[derive(PartialEq)]


error[E0369]: binary operation `<` cannot be applied to type `A`
   |
   |
LL |     a < a; //~ ERROR binary operation `<` cannot be applied to type `A`
   |     - ^ - A
   |     A
   |
   |
note: an implementation of `PartialOrd<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `PartialOrd<_>`
help: consider annotating `A` with `#[derive(PartialEq, PartialOrd)]`
   |
LL | #[derive(PartialEq, PartialOrd)]


error[E0369]: binary operation `<=` cannot be applied to type `A`
   |
   |
LL |     a <= a; //~ ERROR binary operation `<=` cannot be applied to type `A`
   |     - ^^ - A
   |     A
   |
   |
note: an implementation of `PartialOrd<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `PartialOrd<_>`
help: consider annotating `A` with `#[derive(PartialEq, PartialOrd)]`
   |
LL | #[derive(PartialEq, PartialOrd)]


error[E0369]: binary operation `>` cannot be applied to type `A`
   |
   |
LL |     a > a; //~ ERROR binary operation `>` cannot be applied to type `A`
   |     - ^ - A
   |     A
   |
   |
note: an implementation of `PartialOrd<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `PartialOrd<_>`
help: consider annotating `A` with `#[derive(PartialEq, PartialOrd)]`
   |
LL | #[derive(PartialEq, PartialOrd)]


error[E0369]: binary operation `>=` cannot be applied to type `A`
   |
   |
LL |     a >= a; //~ ERROR binary operation `>=` cannot be applied to type `A`
   |     - ^^ - A
   |     A
   |
   |
note: an implementation of `PartialOrd<_>` might be missing for `A`
   |
LL | struct A;
LL | struct A;
   | ^^^^^^^^ must implement `PartialOrd<_>`
help: consider annotating `A` with `#[derive(PartialEq, PartialOrd)]`
   |
LL | #[derive(PartialEq, PartialOrd)]

error: aborting due to 16 previous errors

Some errors have detailed explanations: E0308, E0369.
---

50 note: the following traits must be implemented
51   --> $SRC_DIR/core/src/num/nonzero.rs:LL:COL
52    |
- LL | pub trait ZeroablePrimitive: private::Sealed {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | pub trait ZeroablePrimitive: Sized + Copy + private::Sealed {
55    |
56   ::: $SRC_DIR/core/src/ops/bit.rs:LL:COL
57    |

---
To only update this specific test, also pass `--test-args or-patterns/or-patterns-syntactic-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail/auxiliary"
stdout: none
--- stderr -------------------------------
error: top-level or-patterns are not allowed in function parameters
   |
   |
LL |     fn fun1(A | B: E) {}
   |             ^^^^^ help: wrap the pattern in parentheses: `(A | B)`
error: top-level or-patterns are not allowed in function parameters
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:20:13
   |
   |
LL |     fn fun2(| A | B: E) {}
   |             ^^^^^^^ help: wrap the pattern in parentheses: `(A | B)`

error: top-level or-patterns are not allowed in `let` bindings
   |
   |
LL |     let A | B: E = A;
   |         ^^^^^ help: wrap the pattern in parentheses: `(A | B)`

error: top-level or-patterns are not allowed in `let` bindings
   |
   |
LL |     let | A | B: E = A;
   |         ^^^^^^^ help: wrap the pattern in parentheses: `(A | B)`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:11:24
   |
   |
LL |     let _ = |A | B: E| ();
   |                        ^^ expected struct `NonZero`, found `()`
   |
   = note: expected struct `NonZero<E>`


error[E0369]: no implementation for `E | NonZero<E>`
   |
   |
LL |     let _ = |A | B: E| ();
   |                  ----^ -- NonZero<E>
   |                  E
   |
   |
note: the following type would have to `impl` its required traits for this operation to be valid
   |
   |
LL | enum E { A, B }
   | |
   | |
   | must implement `core::num::nonzero::ZeroablePrimitive`
   | must implement `BitOr`
  --> /checkout/library/core/src/num/nonzero.rs:31:1
   |
   |
LL | pub trait ZeroablePrimitive: Sized + Copy + private::Sealed {
   |
  ::: /checkout/library/core/src/ops/bit.rs:247:1
   |
   |
LL | pub trait BitOr<Rhs = Self> {

error: aborting due to 6 previous errors

Some errors have detailed explanations: E0308, E0369.
