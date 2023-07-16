plain
   Compiling rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
[RUSTC-TIMING] rustc_lint_defs test:false 1.360
[RUSTC-TIMING] rustc_hir test:false 7.708
   Compiling rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error[E0658]: using `_` for array lengths is unstable
     |
     |
1167 |             const VALUES: [&Symbol; _] = [
     |
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = note: see issue #85077 <https://github.com/rust-lang/rust/issues/85077> for more information
     = help: add `#![feature(generic_arg_infer)]` to the crate attributes to enable

error: in expressions, `_` can only be used on the left-hand side of an assignment
     |
     |
1167 |             const VALUES: [&Symbol; _] = [
     |                                     ^ `_` not allowed here
error[E0658]: use of unstable library feature 'map_many_mut'
    --> compiler/rustc_session/src/config.rs:1196:18
     |
     |
1196 |                 .get_many_mut(VALUES)
     |
     = note: see issue #97601 <https://github.com/rust-lang/rust/issues/97601> for more information
     = help: add `#![feature(map_many_mut)]` to the crate attributes to enable

