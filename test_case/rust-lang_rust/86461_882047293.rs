plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: the trait bound `ScalarMaybeUninit: From<pointer::Pointer>` is not satisfied
   --> compiler/rustc_middle/src/ty/vtable.rs:106:26
106 |                     vptr.into()
106 |                     vptr.into()
    |                          ^^^^ the trait `From<pointer::Pointer>` is not implemented for `ScalarMaybeUninit`
    = help: the following implementations were found:
    = help: the following implementations were found:
              <ScalarMaybeUninit<Tag> as From<value::Scalar<Tag>>>
    = note: required because of the requirements on the impl of `Into<ScalarMaybeUninit>` for `pointer::Pointer`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle`
