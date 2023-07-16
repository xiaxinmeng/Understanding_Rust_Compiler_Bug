plain
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
   Compiling rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
   Compiling rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
error: trait-associated function `from_iter` will become ambiguous in Rust 2021
  --> compiler/rustc_traits/src/chalk/mod.rs:65:22
   |
65 |             binders: chalk_ir::CanonicalVarKinds::from_iter(
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: disambiguate the associated function: `<chalk_ir::CanonicalVarKinds as CanonicalVarKinds<I><_>>::from_iter`
   |
   = note: `-D future-prelude-collision` implied by `-D warnings`

error: trait-associated function `from_iter` will become ambiguous in Rust 2021
   --> compiler/rustc_traits/src/chalk/db.rs:631:21
    |
631 |                     chalk_ir::VariableKinds::from_iter(
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: disambiguate the associated function: `<chalk_ir::VariableKinds as VariableKinds<I><_>>::from_iter`

error: trait-associated function `from_iter` will become ambiguous in Rust 2021
   --> compiler/rustc_traits/src/chalk/db.rs:658:9
    |
658 |         chalk_ir::Substitution::from_iter(&self.interner, substitution)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: disambiguate the associated function: `<chalk_ir::Substitution as Substitution<I><_>>::from_iter`

error: trait-associated function `from_iter` will become ambiguous in Rust 2021
   --> compiler/rustc_traits/src/chalk/db.rs:693:9
693 |         chalk_ir::Variances::from_iter(
693 |         chalk_ir::Variances::from_iter(
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: disambiguate the associated function: `<chalk_ir::Variances as Variances<I><_>>::from_iter`

error: trait-associated function `from_iter` will become ambiguous in Rust 2021
   --> compiler/rustc_traits/src/chalk/db.rs:709:9
709 |         chalk_ir::Variances::from_iter(
709 |         chalk_ir::Variances::from_iter(
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: disambiguate the associated function: `<chalk_ir::Variances as Variances<I><_>>::from_iter`

error: trait-associated function `from_iter` will become ambiguous in Rust 2021
   --> compiler/rustc_traits/src/chalk/db.rs:758:5
    |
758 |     chalk_ir::VariableKinds::from_iter(
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: disambiguate the associated function: `<chalk_ir::VariableKinds as VariableKinds<I><_>>::from_iter`

error: trait-associated function `from_iter` will become ambiguous in Rust 2021
  --> compiler/rustc_traits/src/chalk/lowering.rs:57:9
   |
57 |         chalk_ir::Substitution::from_iter(interner, self.iter().map(|s| s.lower_into(interner)))
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: disambiguate the associated function: `<chalk_ir::Substitution as Substitution<I><_>>::from_iter`

error: trait-associated function `from_iter` will become ambiguous in Rust 2021
   --> compiler/rustc_traits/src/chalk/lowering.rs:127:26
    |
127 |                 clauses: chalk_ir::ProgramClauses::from_iter(&interner, clauses),
    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: disambiguate the associated function: `<chalk_ir::ProgramClauses as ProgramClauses<I><_>>::from_iter`

error: trait-associated function `from_iter` will become ambiguous in Rust 2021
   --> compiler/rustc_traits/src/chalk/lowering.rs:290:53
    |
290 |                     substitution: chalk_ir::FnSubst(chalk_ir::Substitution::from_iter(
    |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: disambiguate the associated function: `<chalk_ir::Substitution as Substitution<I><_>>::from_iter`

error: trait-associated function `from_iter` will become ambiguous in Rust 2021
   --> compiler/rustc_traits/src/chalk/lowering.rs:672:21
    |
672 |         let value = chalk_ir::QuantifiedWhereClauses::from_iter(interner, where_clauses);
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: disambiguate the associated function: `<chalk_ir::QuantifiedWhereClauses as QuantifiedWhereClauses<I><_>>::from_iter`

error: trait-associated function `from_iter` will become ambiguous in Rust 2021
   --> compiler/rustc_traits/src/chalk/lowering.rs:829:9
    |
829 |         chalk_ir::VariableKinds::from_iter(interner, parameters.into_iter().map(|(_, v)| v));
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: disambiguate the associated function: `<chalk_ir::VariableKinds as VariableKinds<I><_>>::from_iter`
error: aborting due to 11 previous errors

error: could not compile `rustc_traits`

