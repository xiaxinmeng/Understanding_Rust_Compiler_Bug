rust
error[E0391]: cycle detected when computing the supertraits of `traits::HasCodegen`tc_codegen_ssa                                                                                            
  --> src/librustc_codegen_ssa/traits/mod.rs:88:1
   |
88 | pub trait HasCodegen<'tcx>: Backend<'tcx> + ::std::ops::Deref<Target = Self::CodegenCx> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: ...which again requires computing the supertraits of `traits::HasCodegen`, completing the cycle
note: cycle used when computing the supertraits of `traits::type_::ArgTypeMethods`
  --> src/librustc_codegen_ssa/traits/type_.rs:186:33
   |
186| pub trait ArgTypeMethods<'tcx>: HasCodegen<'tcx> {
   |                                 ^^^^^^^^^^^^^^^^

error: aborting due to previous error
