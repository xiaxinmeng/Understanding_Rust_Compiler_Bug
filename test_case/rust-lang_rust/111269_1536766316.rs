plain
   Compiling rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
error: variable does not need to be mutable
  --> compiler/rustc_macros/src/diagnostics/diagnostic.rs:80:13
   |
80 |         let mut imp = structure.gen_impl(quote! {
   |             |
   |             help: remove this `mut`
   |
   |
   = note: `-D unused-mut` implied by `-D warnings`
   Compiling zerofrom-derive v0.1.1
   Compiling yoke-derive v0.7.0
   Compiling zerovec-derive v0.9.3
   Compiling displaydoc v0.2.3
