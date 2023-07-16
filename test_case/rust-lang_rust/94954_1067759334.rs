plain
    Checking core v0.0.0 (/checkout/library/core)
error[E0277]: the size for values of type `ptr::test_is_null::Extern` cannot be known at compilation time
   --> library/core/tests/ptr.rs:99:36
    |
99  |     let ec: *const Extern = null::<Extern>();
    |
    = help: the trait `Sized` is not implemented for `ptr::test_is_null::Extern`
note: required by a bound in `std::ptr::null`
   --> /checkout/library/core/src/ptr/mod.rs:213:19
   --> /checkout/library/core/src/ptr/mod.rs:213:19
    |
213 | pub const fn null<T>() -> *const T {
    |                   ^ required by this bound in `std::ptr::null`
error[E0277]: the size for values of type `ptr::test_is_null::Extern` cannot be known at compilation time
   --> library/core/tests/ptr.rs:102:38
    |
    |
102 |     let em: *mut Extern = null_mut::<Extern>();
    |
    = help: the trait `Sized` is not implemented for `ptr::test_is_null::Extern`
note: required by a bound in `std::ptr::null_mut`
   --> /checkout/library/core/src/ptr/mod.rs:256:23
   --> /checkout/library/core/src/ptr/mod.rs:256:23
    |
256 | pub const fn null_mut<T>() -> *mut T {
    |                       ^ required by this bound in `std::ptr::null_mut`
error[E0283]: type annotations needed
  --> library/core/tests/ptr.rs:84:35
   |
   |
84 |     let ci: *const dyn ToString = &3;
   |                                   ^^ cannot infer type for type `{integer}`
   |
   = note: multiple `impl`s satisfying `{integer}: ToString` found in the `alloc` crate:
           - impl ToString for i8;
           - impl ToString for u8;
   = note: required for the cast to the object type `dyn ToString`
error[E0283]: type annotations needed
  --> library/core/tests/ptr.rs:87:33
   |
   |
87 |     let mi: *mut dyn ToString = &mut 3;
   |                                 ^^^^^^ cannot infer type for type `{integer}`
   |
   = note: multiple `impl`s satisfying `{integer}: ToString` found in the `alloc` crate:
           - impl ToString for i8;
           - impl ToString for u8;
   = note: required for the cast to the object type `dyn ToString`
Some errors have detailed explanations: E0277, E0283.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `core` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
