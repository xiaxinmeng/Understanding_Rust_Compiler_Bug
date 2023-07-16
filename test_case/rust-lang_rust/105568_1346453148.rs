plain
failures:

---- [ui] src/test/ui/coercion/coerce-issue-49593-box-never.rs#fallback stdout ----

error in revision `fallback`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coercion/coerce-issue-49593-box-never.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "fallback" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-issue-49593-box-never.fallback" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-issue-49593-box-never.fallback/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the size for values of type `dyn std::error::Error` cannot be known at compilation time
   |
   |
LL |     /* *mut $0 is coerced to Box<dyn Error> here */ Box::<_ /* ! */>::new(x)
   |                                                     --------------------- ^ doesn't have a size known at compile-time
   |                                                     required by a bound introduced by this call
   |
   = help: the trait `Sized` is not implemented for `dyn std::error::Error`
note: required by a bound in `Box::<T>::new`
note: required by a bound in `Box::<T>::new`
  --> /checkout/library/alloc/src/boxed.rs:204:6
   |
LL | impl<T> Box<T> {
   |      ^ required by this bound in `Box::<T>::new`

error[E0277]: the size for values of type `(dyn std::error::Error + 'static)` cannot be known at compilation time
   |
   |
LL |     /* *mut $0 is coerced to *mut Error here */ raw_ptr_box::<_ /* ! */>(x)
   |                                                 ------------------------ ^ doesn't have a size known at compile-time
   |                                                 required by a bound introduced by this call
   |
   = help: the trait `Sized` is not implemented for `(dyn std::error::Error + 'static)`
note: required by a bound in `raw_ptr_box`
note: required by a bound in `raw_ptr_box`
  --> /checkout/src/test/ui/coercion/coerce-issue-49593-box-never.rs:13:16
   |
LL | fn raw_ptr_box<T>(t: T) -> *mut T {
   |                ^ required by this bound in `raw_ptr_box`
help: consider relaxing the implicit `Sized` restriction
   |
LL | fn raw_ptr_box<T: ?Sized>(t: T) -> *mut T {


error[E0277]: the size for values of type `dyn Xyz` cannot be known at compilation time
   |
   |
LL |                 = /* Box<$0> is coerced to Box<Xyz> here */ Box::new(x.unwrap());
   |                                                             |
   |                                                             required by a bound introduced by this call
   |
   |
   = help: the trait `Sized` is not implemented for `dyn Xyz`
note: required by a bound in `Box::<T>::new`
   |
   |
LL | impl<T> Box<T> {
   |      ^ required by this bound in `Box::<T>::new`

error[E0277]: the size for values of type `dyn Xyz` cannot be known at compilation time
   |
   |
LL |                 = /* Box<$0> is coerced to Box<Xyz> here */ Box::new(x.unwrap());
   |                                                                      ^ ------ required by a bound introduced by this call
   |                                                                      doesn't have a size known at compile-time
   |
   |
   = help: the trait `Sized` is not implemented for `dyn Xyz`
note: required by a bound in `Option::<T>::unwrap`
   |
   |
LL | impl<T> Option<T> {
   |      ^ required by this bound in `Option::<T>::unwrap`

error[E0277]: the size for values of type `dyn Xyz` cannot be known at compilation time
   |
   |
LL |     let mut x /* : Option<S> */ = None;
   |
   |
   = help: the trait `Sized` is not implemented for `dyn Xyz`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
note: required by a bound in `None`
   |
LL | pub enum Option<T> {
   |                 ^ required by this bound in `None`


error[E0308]: mismatched types
  --> /checkout/src/test/ui/coercion/coerce-issue-49593-box-never.rs:48:13
   |
LL |     let mut x /* : Option<S> */ = None;
...
LL |         x = Some(S);
LL |         x = Some(S);
   |             ^^^^^^^ expected trait object `dyn Xyz`, found struct `S`
   |
   = note: expected enum `Option<dyn Xyz>`
              found enum `Option<S>`

error[E0277]: the size for values of type `dyn Xyz` cannot be known at compilation time
   |
   |
LL |     mem::swap(&mut x, &mut y);
   |
   |
   = help: the trait `Sized` is not implemented for `dyn Xyz`
  --> /checkout/library/core/src/option.rs:518:17
   |
LL | pub enum Option<T> {
   |                 ^ required by this bound in `Option`
   |                 ^ required by this bound in `Option`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/coercion/coerce-issue-49593-box-never.rs:54:23
   |
LL |     mem::swap(&mut x, &mut y);
   |     ---------         ^^^^^^ expected trait object `dyn Xyz`, found struct `S`
   |     arguments to this function are incorrect
   |
   |
   = note: expected mutable reference `&mut Option<dyn Xyz>`
              found mutable reference `&mut Option<S>`
  --> /checkout/library/core/src/mem/mod.rs:719:14
   |
   |
LL | pub const fn swap<T>(x: &mut T, y: &mut T) {

error: aborting due to 8 previous errors

Some errors have detailed explanations: E0277, E0308.
Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/coercion/coerce-issue-49593-box-never.rs#nofallback stdout ----

error in revision `nofallback`: /checkout/src/test/ui/coercion/coerce-issue-49593-box-never.rs:18: unexpected error: '18:75: 18:76: the size for values of type `dyn std::error::Error` cannot be known at compilation time [E0277]'

error in revision `nofallback`: /checkout/src/test/ui/coercion/coerce-issue-49593-box-never.rs:23: unexpected error: '23:74: 23:75: the size for values of type `(dyn std::error::Error + 'static)` cannot be known at compilation time [E0277]'

error in revision `nofallback`: /checkout/src/test/ui/coercion/coerce-issue-49593-box-never.rs:45: unexpected error: '45:70: 45:80: the size for values of type `dyn Xyz` cannot be known at compilation time [E0277]'

error in revision `nofallback`: /checkout/src/test/ui/coercion/coerce-issue-49593-box-never.rs:45: unexpected error: '45:70: 45:71: the size for values of type `dyn Xyz` cannot be known at compilation time [E0277]'

error in revision `nofallback`: /checkout/src/test/ui/coercion/coerce-issue-49593-box-never.rs:40: unexpected error: '40:35: 40:39: the size for values of type `dyn Xyz` cannot be known at compilation time [E0277]'

error in revision `nofallback`: /checkout/src/test/ui/coercion/coerce-issue-49593-box-never.rs:48: unexpected error: '48:13: 48:20: mismatched types [E0308]'

error in revision `nofallback`: /checkout/src/test/ui/coercion/coerce-issue-49593-box-never.rs:54: unexpected error: '54:5: 54:14: the size for values of type `dyn Xyz` cannot be known at compilation time [E0277]'

error in revision `nofallback`: /checkout/src/test/ui/coercion/coerce-issue-49593-box-never.rs:54: unexpected error: '54:23: 54:29: mismatched types [E0308]'

error in revision `nofallback`: /checkout/src/test/ui/coercion/coerce-issue-49593-box-never.rs:18: expected error not found: trait bound `(): std::error::Error` is not satisfied

error in revision `nofallback`: /checkout/src/test/ui/coercion/coerce-issue-49593-box-never.rs:23: expected error not found: trait bound `(): std::error::Error` is not satisfied

error in revision `nofallback`: 8 unexpected errors found, 2 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coercion/coerce-issue-49593-box-never.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nofallback" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-issue-49593-box-never.nofallback" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-issue-49593-box-never.nofallback/auxiliary"
    Error {
        line_num: 18,
        kind: Some(
            Error,
---
        line_num: 23,
        kind: Some(
            Error,
        ),
        msg: "23:74: 23:75: the size for values of type `(dyn std::error::Error + 'static)` cannot be known at compilation time [E0277]",
    Error {
        line_num: 45,
        kind: Some(
            Error,
            Error,
        ),
        msg: "45:70: 45:80: the size for values of type `dyn Xyz` cannot be known at compilation time [E0277]",
    Error {
        line_num: 45,
        kind: Some(
            Error,
            Error,
        ),
        msg: "45:70: 45:71: the size for values of type `dyn Xyz` cannot be known at compilation time [E0277]",
    Error {
        line_num: 40,
        kind: Some(
            Error,
            Error,
        ),
        msg: "40:35: 40:39: the size for values of type `dyn Xyz` cannot be known at compilation time [E0277]",
    Error {
        line_num: 48,
        kind: Some(
            Error,
---
        line_num: 54,
        kind: Some(
            Error,
        ),
        msg: "54:5: 54:14: the size for values of type `dyn Xyz` cannot be known at compilation time [E0277]",
    Error {
        line_num: 54,
        kind: Some(
            Error,
---
        line_num: 18,
        kind: Some(
            Error,
        ),
        msg: "trait bound `(): std::error::Error` is not satisfied",
    Error {
        line_num: 23,
        kind: Some(
            Error,
            Error,
        ),
        msg: "trait bound `(): std::error::Error` is not satisfied",
]


thread '[ui] src/test/ui/coercion/coerce-issue-49593-box-never.rs#nofallback' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1409:13


failures:
    [ui] src/test/ui/coercion/coerce-issue-49593-box-never.rs#fallback
