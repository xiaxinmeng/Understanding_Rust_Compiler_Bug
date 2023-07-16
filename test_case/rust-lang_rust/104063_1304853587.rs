plain
[RUSTC-TIMING] rustc_symbol_mangling test:false 8.992
error[E0308]: mismatched types
   --> compiler/rustc_trait_selection/src/traits/project.rs:654:24
    |
648 |     fn fold_const(&mut self, constant: ty::Const<'tcx>) -> ty::Const<'tcx> {
    |                                                            --------------- expected `rustc_middle::ty::Const<'tcx>` because of return type
654 |                 return Ok(constant);
    |                        ^^^^^^^^^^^^ expected struct `rustc_middle::ty::Const`, found enum `Result`
    |
    = note: expected struct `rustc_middle::ty::Const<'_>`
