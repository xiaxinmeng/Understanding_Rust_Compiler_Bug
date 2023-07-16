
error: internal compiler error: broken MIR in DefId(0:38 ~ bar[290d]::slice::{closure#0}) (bb0[0]): equate_normalized_input_or_output: `&'_#3r [slice::{closure#0} closure_kind_ty=i8 closure_sig_as_fn_ptr_ty=for<'r> extern "rust-call" fn((&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) str, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) ())) -> &ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) [u8] upvar_tys=()]==&'_#11r [slice::{closure#0} closure_kind_ty=i8 closure_sig_as_fn_ptr_ty=for<'r> extern "rust-call" fn((&'_#12r &ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) str, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) ())) -> <&'_#13r [u8] as Yokeable<ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) })>>::Output upvar_tys=()]` failed with `NoSolution`
  --> bar.rs:31:15
   |
31 |     y.project(move |yk, _| yk.as_bytes())
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:253:27
