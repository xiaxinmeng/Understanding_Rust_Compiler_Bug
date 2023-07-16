plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
error[E0308]: mismatched types
    --> compiler/rustc_lint/src/builtin.rs:2150:80
     |
2150 | ...                   hir_generics.span_for_bound_removal(i, index),
     |                                    ----------------------    ^^^^^ expected `usize`, found `u32`
     |                                    arguments to this function are incorrect
     |
note: associated function defined here
    --> /checkout/compiler/rustc_hir/src/hir.rs:682:12
    --> /checkout/compiler/rustc_hir/src/hir.rs:682:12
     |
682  |     pub fn span_for_bound_removal(&self, predicate_pos: usize, bound_pos: usize) -> Span {
     |            ^^^^^^^^^^^^^^^^^^^^^^
help: you can convert a `u32` to a `usize` and panic if the converted value doesn't fit
     |
2150 |                                         hir_generics.span_for_bound_removal(i, index.try_into().unwrap()),

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_lint` due to previous error
warning: build failed, waiting for other jobs to finish...
