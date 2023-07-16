plain
    Checking chalk-engine v0.55.0
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error[E0425]: cannot find function `set_gcx` in module `wow`
    --> compiler/rustc_middle/src/ty/context.rs:1017:14
     |
1017 |         wow::set_gcx(gcx as *const _ as usize);
     |              ^^^^^^^ not found in `wow`

error[E0425]: cannot find function `get_gcx` in module `wow`
    --> compiler/rustc_middle/src/ty/context.rs:1023:26
     |
1023 |         unsafe { &*(wow::get_gcx() as *const GlobalCtxt<'tcx>) }
     |                          ^^^^^^^ not found in `wow`
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: unused import: `std::cell::Cell`
  --> compiler/rustc_middle/src/ty/context.rs:65:5
   |
   |
65 | use std::cell::Cell;
   |     ^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `super::*`
   --> compiler/rustc_middle/src/ty/context.rs:990:9
    |
990 |     use super::*;
