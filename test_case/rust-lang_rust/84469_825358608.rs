
error[E0599]: no method named `name_str` found for enum `rustc_type_ir::IntTy` in the current scope
   --> compiler/rustc_middle/src/ty/print/pretty.rs:527:44
    |
527 |             ty::Int(t) => p!(write("{}", t.name_str())),
    |                                            ^^^^^^^^ method not found in `rustc_type_ir::IntTy`
