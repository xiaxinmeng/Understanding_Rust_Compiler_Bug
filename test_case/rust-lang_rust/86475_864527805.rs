plain
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error[E0433]: failed to resolve: use of undeclared type `ConstValue`
  --> src/vtable.rs:93:30
   |
93 |     let vtable_const_value = ConstValue::ByRef { alloc_id: vtable_allocation, offset: Size::ZERO };
   |                              ^^^^^^^^^^ use of undeclared type `ConstValue`
error[E0433]: failed to resolve: unresolved import
  --> src/vtable.rs:94:32
   |
   |
94 |     let vtable_cvalue = crate::constants::codegen_const_value(fx, vtable_const_value);
   |                                |
   |                                unresolved import
   |                                unresolved import
   |                                help: a similar path exists: `gimli::constants`

warning: unused import: `ty::VtblEntry`
 --> src/vtable.rs:7:5
  |
7 | use ty::VtblEntry;
  |
  = note: `#[warn(unused_imports)]` on by default

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> src/unsize.rs:34:74
   |
34 |         (_, &ty::Dynamic(ref data, ..)) => crate::vtable::get_vtable(fx, source, data.principal()),
   |                                                                          ^^^^^^ expected struct `rustc_target::abi::TyAndLayout`, found `&TyS<'_>`
   |
   = note: expected struct `rustc_target::abi::TyAndLayout<'_, &TyS<'_>>`
           found reference `&TyS<'_>`
error[E0308]: mismatched types
  --> src/vtable.rs:75:40
   |
   |
75 |         let data_id = build_vtable(fx, layout, trait_ref);
   |                                        ^^^^^^ expected `&TyS<'_>`, found struct `rustc_target::abi::TyAndLayout`
   |
   = note: expected reference `&TyS<'_>`
                 found struct `rustc_target::abi::TyAndLayout<'tcx, &'tcx TyS<'tcx>>`
error[E0308]: mismatched types
  --> src/vtable.rs:76:51
   |
   |
76 |         fx.vtables.insert((layout.ty, trait_ref), data_id);
   |                                                   ^^^^^^^ expected struct `cranelift_module::DataId`, found struct `cranelift_codegen::ir::Value`

error[E0308]: `if` and `else` have incompatible types
  --> src/vtable.rs:77:9
   |
72 |       let data_id = if let Some(data_id) = fx.vtables.get(&(layout.ty, trait_ref)) {
   |  ___________________-
73 | |         *data_id
74 | |     } else {
74 | |     } else {
75 | |         let data_id = build_vtable(fx, layout, trait_ref);
76 | |         fx.vtables.insert((layout.ty, trait_ref), data_id);
77 | |         data_id
   | |         ^^^^^^^ expected struct `cranelift_module::DataId`, found struct `cranelift_codegen::ir::Value`
78 | |     };
   | |_____- `if` and `else` have incompatible types
error: aborting due to 6 previous errors; 1 warning emitted

Some errors have detailed explanations: E0308, E0433.
For more information about an error, try `rustc --explain E0308`.
