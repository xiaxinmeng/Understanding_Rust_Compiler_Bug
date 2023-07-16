plain
   Compiling tracing-serde v0.1.2
   Compiling rls-span v0.5.3
   Compiling gsgdt v0.1.2
   Compiling rls-data v0.19.1
error[E0658]: unsizing casts to types besides slices are not allowed in const fn
   --> compiler/rustc_ast/src/ptr.rs:136:37
    |
136 |                 std::mem::transmute(NonNull::<[T; 0]>::dangling() as NonNull<[T]>)
    |
    = note: see issue #64992 <https://github.com/rust-lang/rust/issues/64992> for more information
    = help: add `#![feature(const_fn_unsize)]` to the crate attributes to enable

