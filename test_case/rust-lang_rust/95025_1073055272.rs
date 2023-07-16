plain

   Doc-tests rustc_lint_defs

running 109 tests
ii...i.i.....i......i.ii.i.......i......iii......................i.........Fi.Fi...F.........i...... 100/109
failures:

---- src/builtin.rs - builtin::UNSAFE_CALL_DEPRECATED_SAFE (line 3807) stdout ----
---- src/builtin.rs - builtin::UNSAFE_CALL_DEPRECATED_SAFE (line 3807) stdout ----
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
  |
8 |     foo();
  |     ^^^^^ call to unsafe function
  |
  |
  = note: consult the function's documentation for information on how to avoid undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.
Couldn't compile the test.
Couldn't compile the test.
---- src/builtin.rs - builtin::UNSAFE_IMPL_DEPRECATED_SAFE (line 3883) stdout ----
error: expected item, found `;`
 --> src/builtin.rs:3891:20
  |
9 | impl Foo for Bar {};
  |                    ^ help: remove this semicolon

error[E0200]: the trait `Foo` requires an `unsafe impl` declaration
  |
9 | impl Foo for Bar {};
  | ^^^^^^^^^^^^^^^^^^^

---
---- src/builtin.rs - builtin::UNSAFE_COERCION_DEPRECATED_SAFE (line 3844) stdout ----
error[E0308]: mismatched types
 --> src/builtin.rs:3851:25
  |
8 |     let foo_ptr: fn() = foo;
  |                  ----   ^^^ expected normal fn, found unsafe fn
  |                  expected due to this
  |
  = note: expected fn pointer `fn()`
  = note: expected fn pointer `fn()`
                found fn item `unsafe fn() {foo}`

error[E0277]: expected a `Fn<()>` closure, found `unsafe fn() {foo}`
  |
  |
9 |     let foo_fn_impl: Box<dyn Fn()> = Box::new(foo);
  |                                      ^^^^^^^^^^^^^ expected an `Fn<()>` closure, found `unsafe fn() {foo}`
  |
  = help: the trait `Fn<()>` is not implemented for `unsafe fn() {foo}`
  = note: wrap the `unsafe fn() {foo}` in a closure with no arguments: `|| { /* code */ }`
  = note: required for the cast to the object type `dyn Fn()`

error[E0277]: expected a `FnMut<()>` closure, found `unsafe fn() {foo}`
   |
   |
10 |     let foo_fnmut_impl: Box<dyn FnMut()> = Box::new(foo);
   |                                            ^^^^^^^^^^^^^ expected an `FnMut<()>` closure, found `unsafe fn() {foo}`
   |
   = help: the trait `FnMut<()>` is not implemented for `unsafe fn() {foo}`
   = note: wrap the `unsafe fn() {foo}` in a closure with no arguments: `|| { /* code */ }`
   = note: required for the cast to the object type `dyn FnMut()`

error[E0277]: expected a `FnOnce<()>` closure, found `unsafe fn() {foo}`
   |
   |
11 |     let foo_fnonce_impl: Box<dyn FnOnce()> = Box::new(foo);
   |                                              ^^^^^^^^^^^^^ expected an `FnOnce<()>` closure, found `unsafe fn() {foo}`
   |
   = help: the trait `FnOnce<()>` is not implemented for `unsafe fn() {foo}`
   = note: wrap the `unsafe fn() {foo}` in a closure with no arguments: `|| { /* code */ }`
   = note: required for the cast to the object type `dyn FnOnce()`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
