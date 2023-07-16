plain

---- [ui] src/test/ui/binop/issue-28837.rs stdout ----
diff of stderr:

138    | must implement `core::num::nonzero::ZeroablePrimitive`
139    | must implement `BitOr`
-   --> $SRC_DIR/core/src/num/nonzero.rs:LL:COL
+   --> $SRC_DIR/core/src/ops/bit.rs:LL:COL
142    |
142    |
- LL | pub trait ZeroablePrimitive: Sized + Copy + private::Sealed {
-    |
-   ::: $SRC_DIR/core/src/ops/bit.rs:LL:COL
-    |
-    |
148 LL | pub trait BitOr<Rhs = Self> {
+    |
+   ::: $SRC_DIR/core/src/num/nonzero.rs:LL:COL
+    |
+    |
+ LL | pub trait ZeroablePrimitive: Sized + Copy + private::Sealed {
150 
150 
151 error[E0369]: no implementation for `A << A`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-28837/issue-28837.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-28837/issue-28837.stderr
To update references, rerun the tests and pass the `--bless` flag
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To only update this specific test, also pass `--test-args binop/issue-28837.rs`

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
  --> /checkout/library/core/src/ops/bit.rs:247:1
   |
   |
LL | pub trait BitOr<Rhs = Self> {
   |
  ::: /checkout/library/core/src/num/nonzero.rs:31:1
   |
   |
LL | pub trait ZeroablePrimitive: Sized + Copy + private::Sealed {


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
diff of stderr:

481    = note: references must be non-null
482 
483 error: the type `NonZero<u32>` does not permit zero-initialization
-   --> $DIR/uninitialized-zeroed.rs:118:32
485    |
485    |
486 LL |         let _val: NonZero<u32> = mem::transmute(0);
-    |                                |
-    |                                this code causes undefined behavior when executed
-    |                                help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
+    |                                  ^^^^^^^^^^^^^^^^^
+    |                                  ^^^^^^^^^^^^^^^^^
+    |                                  |
+    |                                  this code causes undefined behavior when executed
+    |                                  help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
491    |
492    = note: `std::num::NonZero<u32>` must be non-null


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/uninitialized-zeroed/uninitialized-zeroed.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/uninitialized-zeroed/uninitialized-zeroed.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/uninitialized-zeroed.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/uninitialized-zeroed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/uninitialized-zeroed" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/uninitialized-zeroed/auxiliary"
stdout: none
--- stderr -------------------------------
error: the type `&T` does not permit zero-initialization
   |
   |
LL |         let _val: &'static T = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                |
   |                                this code causes undefined behavior when executed
   |                                help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:6:9
   |
LL | #![deny(invalid_value)]
   |         ^^^^^^^^^^^^^
   = note: references must be non-null

error: the type `&T` does not permit being left uninitialized
   |
   |
LL |         let _val: &'static T = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                |
   |                                this code causes undefined behavior when executed
   |                                help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: references must be non-null

error: the type `Wrap<&T>` does not permit zero-initialization
   |
   |
LL |         let _val: Wrap<&'static T> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                      |
   |                                      this code causes undefined behavior when executed
   |                                      help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: references must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:17:18
   |
LL | struct Wrap<T> { wrapped: T }


error: the type `Wrap<&T>` does not permit being left uninitialized
   |
   |
LL |         let _val: Wrap<&'static T> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                      |
   |                                      this code causes undefined behavior when executed
   |                                      help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: references must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:17:18
   |
LL | struct Wrap<T> { wrapped: T }


error: the type `!` does not permit zero-initialization
   |
   |
LL |         let _val: ! = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                       |
   |                       this code causes undefined behavior when executed
   |                       help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: the `!` type has no valid value

error: the type `!` does not permit being left uninitialized
   |
   |
LL |         let _val: ! = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                       |
   |                       this code causes undefined behavior when executed
   |                       help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: the `!` type has no valid value

error: the type `(i32, !)` does not permit zero-initialization
   |
   |
LL |         let _val: (i32, !) = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                              |
   |                              this code causes undefined behavior when executed
   |                              help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: the `!` type has no valid value

error: the type `(i32, !)` does not permit being left uninitialized
   |
   |
LL |         let _val: (i32, !) = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                              |
   |                              this code causes undefined behavior when executed
   |                              help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: integers must not be uninitialized

error: the type `Void` does not permit zero-initialization
   |
   |
LL |         let _val: Void = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                          |
   |                          this code causes undefined behavior when executed
   |                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: enums with no variants have no valid value

error: the type `Void` does not permit being left uninitialized
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:58:26
   |
LL |         let _val: Void = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                          |
   |                          this code causes undefined behavior when executed
   |                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: enums with no variants have no valid value

error: the type `&i32` does not permit zero-initialization
   |
   |
LL |         let _val: &'static i32 = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                  |
   |                                  this code causes undefined behavior when executed
   |                                  help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: references must be non-null

error: the type `&i32` does not permit being left uninitialized
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:61:34
   |
LL |         let _val: &'static i32 = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                  |
   |                                  this code causes undefined behavior when executed
   |                                  help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: references must be non-null

error: the type `Ref` does not permit zero-initialization
   |
   |
LL |         let _val: Ref = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                         |
   |                         this code causes undefined behavior when executed
   |                         help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: references must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:14:12
   |
LL | struct Ref(&'static i32);


error: the type `Ref` does not permit being left uninitialized
   |
   |
LL |         let _val: Ref = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                         |
   |                         this code causes undefined behavior when executed
   |                         help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: references must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:14:12
   |
LL | struct Ref(&'static i32);


error: the type `fn()` does not permit zero-initialization
   |
   |
LL |         let _val: fn() = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                          |
   |                          this code causes undefined behavior when executed
   |                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: function pointers must be non-null

error: the type `fn()` does not permit being left uninitialized
   |
   |
LL |         let _val: fn() = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                          |
   |                          this code causes undefined behavior when executed
   |                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: function pointers must be non-null

error: the type `Wrap<fn()>` does not permit zero-initialization
   |
   |
LL |         let _val: Wrap<fn()> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                |
   |                                this code causes undefined behavior when executed
   |                                help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: function pointers must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:17:18
   |
LL | struct Wrap<T> { wrapped: T }


error: the type `Wrap<fn()>` does not permit being left uninitialized
   |
   |
LL |         let _val: Wrap<fn()> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                |
   |                                this code causes undefined behavior when executed
   |                                help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: function pointers must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:17:18
   |
LL | struct Wrap<T> { wrapped: T }


error: the type `WrapEnum<fn()>` does not permit zero-initialization
   |
   |
LL |         let _val: WrapEnum<fn()> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                    |
   |                                    this code causes undefined behavior when executed
   |                                    help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: function pointers must be non-null (in this enum field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:18:28
   |
LL | enum WrapEnum<T> { Wrapped(T) }


error: the type `WrapEnum<fn()>` does not permit being left uninitialized
   |
   |
LL |         let _val: WrapEnum<fn()> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                    |
   |                                    this code causes undefined behavior when executed
   |                                    help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: function pointers must be non-null (in this enum field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:18:28
   |
LL | enum WrapEnum<T> { Wrapped(T) }


error: the type `Wrap<(RefPair, i32)>` does not permit zero-initialization
   |
   |
LL |         let _val: Wrap<(RefPair, i32)> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                          |
   |                                          this code causes undefined behavior when executed
   |                                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: references must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:15:16
   |
LL | struct RefPair((&'static i32, i32));


error: the type `Wrap<(RefPair, i32)>` does not permit being left uninitialized
   |
   |
LL |         let _val: Wrap<(RefPair, i32)> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                          |
   |                                          this code causes undefined behavior when executed
   |                                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: references must be non-null (in this struct field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:15:16
   |
LL | struct RefPair((&'static i32, i32));


error: the type `NonNull<i32>` does not permit zero-initialization
   |
   |
LL |         let _val: NonNull<i32> = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                  |
   |                                  this code causes undefined behavior when executed
   |                                  help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: `std::ptr::NonNull<i32>` must be non-null

error: the type `NonNull<i32>` does not permit being left uninitialized
   |
   |
LL |         let _val: NonNull<i32> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                  |
   |                                  this code causes undefined behavior when executed
   |                                  help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: `std::ptr::NonNull<i32>` must be non-null

error: the type `*const dyn Send` does not permit zero-initialization
   |
   |
LL |         let _val: *const dyn Send = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                                     |
   |                                     this code causes undefined behavior when executed
   |                                     help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: the vtable of a wide raw pointer must be non-null

error: the type `*const dyn Send` does not permit being left uninitialized
   |
   |
LL |         let _val: *const dyn Send = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                     |
   |                                     this code causes undefined behavior when executed
   |                                     help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: the vtable of a wide raw pointer must be non-null

error: the type `[fn(); 2]` does not permit zero-initialization
   |
   |
LL |         let _val: [fn(); 2] = mem::zeroed(); //~ ERROR: does not permit zero-initialization
   |                               |
   |                               this code causes undefined behavior when executed
   |                               help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: function pointers must be non-null

error: the type `[fn(); 2]` does not permit being left uninitialized
   |
   |
LL |         let _val: [fn(); 2] = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                               |
   |                               this code causes undefined behavior when executed
   |                               help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: function pointers must be non-null

error: the type `bool` does not permit being left uninitialized
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:89:26
   |
LL |         let _val: bool = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                          |
   |                          this code causes undefined behavior when executed
   |                          help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: booleans must be either `true` or `false`

error: the type `Wrap<char>` does not permit being left uninitialized
   |
   |
LL |         let _val: Wrap<char> = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                |
   |                                this code causes undefined behavior when executed
   |                                help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: characters must be a valid Unicode codepoint (in this struct field)
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:17:18
   |
LL | struct Wrap<T> { wrapped: T }


error: the type `NonBig` does not permit being left uninitialized
   |
   |
LL |         let _val: NonBig = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                            |
   |                            this code causes undefined behavior when executed
   |                            help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: `NonBig` must be initialized inside its custom valid range

error: the type `Fruit` does not permit being left uninitialized
   |
   |
LL |         let _val: Fruit = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                           |
   |                           this code causes undefined behavior when executed
   |                           help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
note: enums have to be initialized to a variant
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:26:1
   |
LL | enum Fruit {


error: the type `[bool; 2]` does not permit being left uninitialized
   |
   |
LL |         let _val: [bool; 2] = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                               |
   |                               this code causes undefined behavior when executed
   |                               help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: booleans must be either `true` or `false`
error: the type `i32` does not permit being left uninitialized
  --> /checkout/src/test/ui/lint/uninitialized-zeroed.rs:104:25
   |
   |
LL |         let _val: i32 = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                         |
   |                         this code causes undefined behavior when executed
   |                         help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: integers must not be uninitialized

error: the type `f32` does not permit being left uninitialized
   |
   |
LL |         let _val: f32 = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                         |
   |                         this code causes undefined behavior when executed
   |                         help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: floats must not be uninitialized

error: the type `*const ()` does not permit being left uninitialized
   |
   |
LL |         let _val: *const () = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                               |
   |                               this code causes undefined behavior when executed
   |                               help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: raw pointers must not be uninitialized

error: the type `*const [()]` does not permit being left uninitialized
   |
   |
LL |         let _val: *const [()] = mem::uninitialized(); //~ ERROR: does not permit being left uninitialized
   |                                 |
   |                                 this code causes undefined behavior when executed
   |                                 help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   |
   = note: raw pointers must not be uninitialized

error: the type `&i32` does not permit zero-initialization
   |
   |
LL |         let _val: &'static i32 = mem::transmute(0usize); //~ ERROR: does not permit zero-initialization
   |                                  |
   |                                  this code causes undefined behavior when executed
   |                                  help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
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
  --> /checkout/library/core/src/ops/bit.rs:247:1
   |
   |
LL | pub trait BitOr<Rhs = Self> {
   |
  ::: /checkout/library/core/src/num/nonzero.rs:31:1
   |
   |
LL | pub trait ZeroablePrimitive: Sized + Copy + private::Sealed {

error: aborting due to 6 previous errors

Some errors have detailed explanations: E0308, E0369.
