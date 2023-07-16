
// for `{impl#0}::next`
convert_impl_item 
   ->  rustc_middle::fn_sig 
   -> execute_job 
   -> rustc_hir_analysis::collect::fn_sig 
   -> lookup_inherent_assoc_ty 
   -> sup 
   -> variances_of 
   -> rustc_middle::fn_sig
